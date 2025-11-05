// cargo run -p step_11 --release --example demo_05
//
//! GPU Benchmark - Uses shaders
// Version 05
// Uses the GPU with shaders.
// Compare with demo_04 to see the difference:
//      - demo_04: CPU calculates pixels → ~   100 FPS
//      - demo_05: GPU calculates pixels → ~ 3_000 FPS

// Iris     Vulkan  Fifo        Average FPS =  238
// Iris     Vulkan  Immediate   Average FPS =  238
// Iris     DX12    Immediate   Average FPS = 1019
// NVIDIA   Vulkan  Fifo        Average FPS = 1891
// NVIDIA   Vulkan  Immediate   Average FPS = 1899
// NVIDIA   DX12    Immediate   Average FPS = 3044

use pixels::{Pixels, PixelsBuilder, SurfaceTexture, wgpu};
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BENCHMARK_DURATION: Duration = Duration::from_secs(15);

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    uniform_buffer: Option<wgpu::Buffer>,
    frame_count: u64,
    start_time: Option<Instant>,
    last_print: Instant,
    render_pipeline: Option<wgpu::RenderPipeline>,
    bind_group: Option<wgpu::BindGroup>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            uniform_buffer: None,
            frame_count: 0,
            start_time: None,
            last_print: Instant::now(),
            render_pipeline: None,
            bind_group: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Step 11: GPU bench 02, shaders").with_resizable(false))
            .unwrap();
        // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None))),
        let window_ref: &'static Window = Box::leak(Box::new(window));
        let size = window_ref.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window_ref);

        let mut builder = PixelsBuilder::new(WIDTH, HEIGHT, surface);

        builder = builder.request_adapter_options(wgpu::RequestAdapterOptions {
            // 1 - GPU: Pick one or the other
            // power_preference: wgpu::PowerPreference::LowPower,
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        });

        // 2 - Backend: Pick one or the other
        builder = builder.wgpu_backend(wgpu::Backends::DX12);
        // builder = builder.wgpu_backend(wgpu::Backends::VULKAN);

        let mut pixels = builder.build().expect("create pixels");

        // 3 - PresentationMode: Pick one or the other
        // pixels.set_present_mode(wgpu::PresentMode::Fifo);
        pixels.set_present_mode(wgpu::PresentMode::Immediate);
        println!("Present mode: {:?}", pixels.present_mode());

        // Get wgpu device and queue from pixels
        let device = pixels.device();

        // Shader source code inline
        let shader_source = r#"
struct Uniforms {
    time: f32,
    width: f32,
    height: f32,
    _padding: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    let x = f32((vertex_index << 1u) & 2u);
    let y = f32(vertex_index & 2u);
    output.position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    output.uv = vec2<f32>(x, y);
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let x = input.uv.x * uniforms.width;
    let y = input.uv.y * uniforms.height;
    let time = uniforms.time;
    
    let r = sin(x / 10.0 + time) * 0.5 + 0.5;
    let g = cos(y / 10.0 + time) * 0.5 + 0.5;
    let b = sin((x / 5.0 + y / 5.0) + time) * 0.5 + 0.5;
    
    return vec4<f32>(r, g, b, 1.0);
}
"#;

        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Animation Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: pixels.render_texture_format(),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        self.window = Some(window_ref);
        self.pixels = Some(pixels);
        self.render_pipeline = Some(render_pipeline);
        self.bind_group = Some(bind_group);
        self.uniform_buffer = Some(uniform_buffer);

        // Init print timer
        self.start_time = Some(Instant::now());

        println!("\nBENCHMARK STARTED");
        println!("Duration: {} seconds", BENCHMARK_DURATION.as_secs());
        println!("Resolution: {}x{}", WIDTH, HEIGHT);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.print_final_results();
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                // Check if benchmark is complete
                if let Some(start) = self.start_time
                    && start.elapsed() >= BENCHMARK_DURATION
                {
                    self.print_final_results();
                    event_loop.exit();
                    return;
                }

                // Render with GPU shader
                if let (Some(pixels), Some(pipeline), Some(bind_group), Some(uniform_buffer)) = (&mut self.pixels, &self.render_pipeline, &self.bind_group, &self.uniform_buffer) {
                    let time = self.frame_count as f32 * 0.05;

                    // Update uniform buffer - convert floats to bytes
                    let uniforms = [time, WIDTH as f32, HEIGHT as f32, 0.0_f32];
                    let uniform_bytes: &[u8] = unsafe { std::slice::from_raw_parts(uniforms.as_ptr() as *const u8, std::mem::size_of_val(&uniforms)) };
                    pixels.queue().write_buffer(uniform_buffer, 0, uniform_bytes);

                    // Render with GPU shader
                    let result = pixels.render_with(|encoder, render_target, _context| {
                        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: render_target,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });

                        render_pass.set_pipeline(pipeline);
                        render_pass.set_bind_group(0, bind_group, &[]);
                        render_pass.draw(0..6, 0..1);

                        Ok(())
                    });

                    if result.is_err() {
                        eprintln!("Render error: {:?}", result);
                    }
                }

                self.frame_count += 1;

                // Print progress every second
                if self.last_print.elapsed() >= Duration::from_secs(1) {
                    if let Some(start) = self.start_time {
                        let elapsed = start.elapsed().as_secs();
                        let fps = self.frame_count as f32 / start.elapsed().as_secs_f32();
                        println!("[{:2}s] Frames: {} | FPS: {:.0}", elapsed, self.frame_count, fps);
                    }
                    self.last_print = Instant::now();
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        // if let Some(window) = &self.window {
        //     window.request_redraw();
        // }
        self.window.expect("Bug - Window should exist").request_redraw();
    }
}

impl App {
    fn print_final_results(&self) {
        println!("\n{}", "=".repeat(50));
        println!("BENCHMARK COMPLETE");
        println!("{}", "=".repeat(50));

        if let Some(start) = self.start_time {
            let elapsed = start.elapsed().as_secs_f32();
            let fps = self.frame_count as f32 / elapsed;

            println!("Total Frames:  {}", self.frame_count);
            println!("Total Time:    {:.2}s", elapsed);
            println!("Average FPS:   {:.0}", fps);
            println!("Frame Time:    {:.2}ms", 1000.0 / fps);
        }
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("wgpu_core=info")).init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}

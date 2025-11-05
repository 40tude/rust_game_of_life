// src/app/events.rs

// use crate::prelude::*; // see lib.rs
use crate::{
    app::{render, state::App},
    config,
    gol::life,
};

use rfd::FileDialog;
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseScrollDelta, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{self, Fullscreen, Icon, Window},
};

// Load window icon from PNG file
fn load_icon() -> Option<Icon> {
    // Try multiple paths and formats
    // let image = image::open("assets/40tude.ico").or_else(|_| image::open("assets/40tude.webp")).ok()?.to_rgba8();
    // Icon data embedded at compile time (no external file needed!)
    const ICON_DATA: &[u8] = include_bytes!("../../../assets/40tude.ico");

    let image = image::load_from_memory(ICON_DATA).ok()?.to_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    Icon::from_rgba(rgba, width, height).ok()
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window with specified dimensions
        let mut window_attributes = Window::default_attributes()
            .with_title(config::TITLE)
            .with_inner_size(winit::dpi::PhysicalSize::new(config::WINDOW_WIDTH, config::WINDOW_HEIGHT));

        // Set window icon if available
        if let Some(icon) = load_icon() {
            window_attributes = window_attributes.with_window_icon(Some(icon));
        }

        let window = event_loop.create_window(window_attributes).unwrap();

        // Leak the window to obtain a &'static Window for the app lifetime
        // TODO:
        let window_ref: &'static Window = Box::leak(Box::new(window));
        self.window = Some(window_ref);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let zoom_delta = match delta {
                    MouseScrollDelta::LineDelta(_x, y) => y, // y > 0 = scroll up (zoom in)
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
                };

                if zoom_delta.abs() > f32::EPSILON {
                    self.handle_zoom(zoom_delta);
                }
            }

            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key,
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                },
                ..
            } => {
                // Zoom in with +
                if matches!(logical_key.as_ref(), Key::Character("+") | Key::Character("=")) {
                    // Pavé numérique + ou touche +
                    self.handle_zoom(1.0); // delta positif = zoom avant
                    return;
                }

                // Zoom out with -
                if matches!(logical_key.as_ref(), Key::Character("-") | Key::Character("_")) {
                    // Pavé numérique - ou touche -
                    self.handle_zoom(-1.0); // delta négatif = zoom arrière
                    return;
                }
                // `F11` or `F` to toggle full screen
                let is_fullscreen_key = matches!(logical_key, Key::Named(NamedKey::F11)) || matches!(logical_key.as_ref(), Key::Character(s) if s.eq_ignore_ascii_case("f"));

                if is_fullscreen_key {
                    self.full_screen = !self.full_screen;

                    if let Some(window) = &self.window {
                        if self.full_screen {
                            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        } else {
                            window.set_fullscreen(None);
                        }
                    }
                    return;
                }

                // `o` : to open .rle file
                if matches!(logical_key.as_ref(), Key::Character(s) if s.eq_ignore_ascii_case("o"))
                    && let Some(path) = FileDialog::new().add_filter("RLE files", &["rle"]).set_directory("rle/").pick_file()
                    && let Err(e) = self.load_pattern(&path)
                {
                    let error_msg = format!("Failed to load pattern: {}", e);
                    log::error!("{}", error_msg);
                    self.set_error(error_msg, 5); // Display error for 5 seconds

                    // Should I clear previous error on success? Something like
                    // match self.load_pattern(&path) {
                    //     Ok(_) => {
                    //         info!("Pattern loaded successfully from {:?}", path);
                    //         self.last_error = None; // Clear any previous error
                    //         self.error_display_until = None;
                    //     }
                    //     Err(e) => {
                    //         let error_msg = format!("Failed to load pattern: {}", e);
                    //         error!("{}", error_msg);
                    //         self.set_error(error_msg, 5); // Display error for 5 seconds
                    //     }
                    // }
                }
            }

            WindowEvent::Resized(size) => {
                self.pending_resize = Some((size.width, size.height));
                log::debug!("WindowEvent::Resized(): pending_resize = {:?}", self.pending_resize);
            }

            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(w) = self.window {
                    let s = w.inner_size();
                    self.pending_resize = Some((s.width, s.height));
                    log::debug!("WindowEvent::ScaleFactorChanged(): pending_resize = {:?}", self.pending_resize);
                }
            }

            WindowEvent::RedrawRequested => {
                // Update the board & measure
                let step_start = Instant::now();
                life::step_life(&self.board_current, &mut self.board_next, self.board_width, self.board_height);
                std::mem::swap(&mut self.board_current, &mut self.board_next);
                let step_duration = step_start.elapsed();
                self.perf_metrics.record_step(step_duration);

                // Draw the current board with camera and zoom & measure
                if let Some(pixels) = &mut self.pixels {
                    let render_start = Instant::now();
                    render::draw_board_with_camera(
                        pixels,
                        &self.board_current,
                        self.board_width,
                        self.board_height,
                        self.camera_x,
                        self.camera_y,
                        self.zoom_level,
                        self.surface_w,
                        self.surface_h,
                    );
                    let render_duration = render_start.elapsed();
                    self.perf_metrics.record_render(render_duration);

                    // TODO: Draw error overlay if there's an error. DO NOT MEASURE ???
                    if let Some(error_msg) = &self.last_error {
                        render::draw_error_overlay(pixels, error_msg, self.board_width, self.board_height);
                    }
                }

                // Display every second
                if self.perf_metrics.should_log(Duration::from_secs(config::PERF_LOG_INTERVAL_SECS))
                    && let (Some(avg_step), Some(avg_render), Some(p95_step)) = (self.perf_metrics.avg_step_time(), self.perf_metrics.avg_render_time(), self.perf_metrics.percentile_95_step())
                {
                    let total = avg_step + avg_render;
                    let fps_theoretical = if total.as_micros() > 0 { 1_000_000 / total.as_micros() } else { 0 };

                    log::info!(
                        "Perf: step={:>6.2}ms (p95={:>6.2}ms) | render={:>6.2}ms | total={:>6.2}ms | theo_fps={:>4} | board={}x{} | zoom={:.2}",
                        avg_step.as_secs_f64() * 1000.0,
                        p95_step.as_secs_f64() * 1000.0,
                        avg_render.as_secs_f64() * 1000.0,
                        total.as_secs_f64() * 1000.0,
                        fps_theoretical,
                        self.board_width,
                        self.board_height,
                        self.zoom_level
                    );
                }
            }

            _ => {}
        }
    }

    // https://docs.rs/winit/latest/winit/application/trait.ApplicationHandler.html#method.about_to_wait
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let now = Instant::now();
        // Limit to 60 FPS
        if now - self.last_frame >= config::FRAME_DURATION {
            self.last_frame = now;
            // Handle pending resize
            if let Some((w, h)) = self.pending_resize.take() {
                self.handle_resize(w, h)
            }

            // Update error display timer
            self.update_error_display();

            // .window is guaranteed to be Some at this point (created in Event::Resumed)
            self.window.expect("Bug - Window should exist").request_redraw();
        }
    }
}

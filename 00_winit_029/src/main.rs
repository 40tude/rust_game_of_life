// cargo run -p step_00_winit_029
//! Kind of "Hello world!"

use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{/*ControlFlow,*/ EventLoop},
    window::{Window, WindowBuilder},
};

const WIDTH: u32 = 200;
const HEIGHT: u32 = 150;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut window: Option<&'static Window> = None;
    let mut pixels: Option<Pixels> = None;

    // elwt: EventLoopWindowTarget
    // elwt can create windows, see .build(elwt) below. Factory
    // elwt can control the event loop, see elwt.exit() below
    // window is a product that elwt creates
    // elwt abstracts platform-specific details (Win64, Wayland, Browser DOM...)

    // The closure is moved into event_loop.run(), which owns it
    // The event loop could run forever (until elwt.exit())
    // Any references captured must live as long as the closure
    // They must be 'static => See window above and the Box::leak() below

    // This starts the event loop
    event_loop.run(move |event, elwt| {
        match event {
            Event::Resumed => {
                // elwt.set_control_flow(ControlFlow::Poll); // Never sleep and call the closure ASAP
                if window.is_none() {
                    // Use elwt to create a window
                    let built_window = WindowBuilder::new().with_title("Step_00_winit_029: First try").build(elwt).unwrap();

                    let size = built_window.inner_size();

                    // Box::leak() converts a Box<T> into a &'static T reference intentionally creating a memory leak.
                    // It transfers ownership of the heap-allocated value (Box::new(built_window)) to "nobody," making it live forever (until the program ends).
                    let window_ref: &'static Window = Box::leak(Box::new(built_window));

                    let surface = SurfaceTexture::new(size.width, size.height, window_ref);
                    let built_pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

                    window = Some(window_ref);
                    pixels = Some(built_pixels);

                    // built_window was moved into Box, then leaked
                    // window_ref points to heap memory that will never be freed

                    let scale_factor = window_ref.scale_factor();
                    println!("Scale factor : {}", scale_factor);
                }
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested, .. // `..` ignores any other fields of WindowEvent (window_id). Only the presence of a RedrawRequested matters.
            } => {
                // Render the frame
                if let Some(pixels) = &mut pixels {
                    let frame = pixels.frame_mut();

                    // Fill all the frame in blue
                    for spot in frame.chunks_exact_mut(4) {
                        spot[0] = 0x20; // R
                        spot[1] = 0x40; // G
                        spot[2] = 0xFF; // B
                        spot[3] = 0xFF; // A
                    }

                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                        elwt.exit();
                    }
                }
            }

            // Event emitted by winit's event loop just before it goes idle waiting for new events.
            // It says: "I've finished processing all pending events, and I'm about to sleep until something else happens. Is there anything I can do for you before?"
            // Calling window.request_redraw() inside Event::AboutToWait creates a continuous rendering loop.
            Event::AboutToWait => {
                // Unnecessarily defensive
                // if let Some(window) = &window_opt {
                //     window.request_redraw();
                // }
                // window_opt is guaranteed to be Some at this point (created in Event::Resumed)
                window.expect("Bug - Window should exist").request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..
            } => {
                elwt.exit(); // Use elwt to exit the loop
            }

            _ => {}
        }
    })?;

    Ok(())
}

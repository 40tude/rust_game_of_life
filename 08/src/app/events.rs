// src/app/events.rs

use crate::app::render;
use crate::app::state::App;
use crate::config;
use crate::gol::life;

use rfd::FileDialog;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Fullscreen, Window},
};

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window
        let window = event_loop.create_window(Window::default_attributes().with_title(config::TITLE)).unwrap();

        // Leak the window to obtain a &'static Window for the app lifetime
        // TODO:
        let window_ref: &'static Window = Box::leak(Box::new(window));
        self.window = Some(window_ref);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: winit::window::WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
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
                // F11 or F to toggle full screen
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

                // O : to open .rle file
                if matches!(logical_key.as_ref(), Key::Character(s) if s.eq_ignore_ascii_case("o"))
                    && let Some(path) = FileDialog::new().add_filter("RLE files", &["rle"]).set_directory("rle/").pick_file()
                {
                    println!("File selected: {:?}", path);
                    // TODO call read_rle(&path) ...
                    let _ = self.load_pattern(&path);
                }
            }

            WindowEvent::Resized(size) => {
                self.pending_resize = Some((size.width, size.height));
            }

            WindowEvent::ScaleFactorChanged { .. } => {
                if let Some(w) = self.window {
                    let s = w.inner_size();
                    self.pending_resize = Some((s.width, s.height));
                }
            }

            WindowEvent::RedrawRequested => {
                // Update the board
                life::step_life(&self.board_current, &mut self.board_next, self.board_width, self.board_height);
                std::mem::swap(&mut self.board_current, &mut self.board_next);

                // Draw the current board
                if let Some(pixels) = &mut self.pixels {
                    render::draw_board(pixels, &self.board_current, self.board_width, self.board_height);
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
            if let Some((w, h)) = self.pending_resize.take() {
                let _ = self.handle_resize(w, h); // crée/resize pixels + (ré)initialise les cells
            }

            // .window is guaranteed to be Some at this point (created in Event::Resumed)
            self.window.expect("Bug - Window should exist").request_redraw();
        }
    }
}

// src/app/events.rs

use crate::{
    app::{render, state::App},
    config,
    gol::life,
};

use rfd::FileDialog;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{self, Fullscreen, Window},
};

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window with specified dimensions
        let window_attributes = Window::default_attributes()
            .with_title(config::TITLE)
            .with_inner_size(winit::dpi::PhysicalSize::new(config::WINDOW_WIDTH, config::WINDOW_HEIGHT));

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

            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    logical_key,
                    state: ElementState::Pressed,
                    repeat: false,
                    ..
                },
                ..
            } => {
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
                    eprintln!("{}", error_msg);
                    self.set_error(error_msg, 5); // Display error for 5 seconds

                    // Should I clear previous error on success? Something like
                    // match self.load_pattern(&path) {
                    //     Ok(_) => {
                    //         println!("Pattern loaded successfully from {:?}", path);
                    //         self.last_error = None; // Clear any previous error
                    //         self.error_display_until = None;
                    //     }
                    //     Err(e) => {
                    //         let error_msg = format!("Failed to load pattern: {}", e);
                    //         eprintln!("{}", error_msg);
                    //         self.set_error(error_msg, 5); // Display error for 5 seconds
                    //     }
                    // }
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

                    // Draw error overlay if there's an error
                    if let Some(error_msg) = &self.last_error {
                        render::draw_error_overlay(pixels, error_msg, self.board_width, self.board_height);
                    }
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
            if let Some((w, h)) = self.pending_resize.take()
                && let Err(e) = self.handle_resize(w, h)
            {
                let error_msg = format!("Error during resize: {}", e);
                eprintln!("{}", error_msg);
                self.set_error(error_msg, 3);
            }

            // Update error display timer
            self.update_error_display();

            // .window is guaranteed to be Some at this point (created in Event::Resumed)
            self.window.expect("Bug - Window should exist").request_redraw();
        }
    }
}

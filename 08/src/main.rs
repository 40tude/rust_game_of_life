// src/main.rs
// cargo run -p step_08
// cargo test -p step_08
//! Much more modularization + load pattern with 'o'

// cargo add rfd -p step_08 # File Dialog

use step_08::{Result, app::state::App};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::try_new()?;
    event_loop.run_app(&mut app)?;

    Ok(())
}

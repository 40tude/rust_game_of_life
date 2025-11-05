// src/main.rs
// cargo run -p step_09
//! Smarter error management?

use step_09::{Result, app::state::App};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::try_new()?;
    event_loop.run_app(&mut app)?;

    Ok(())
}

// src/lib.rs

pub mod app;
pub mod config;
pub mod error;
pub mod gol;

// re-export lib from crate root
pub use self::error::{Error, Result};

// help to have them everywhere
// pub mod prelude {
//     pub use log::{debug, error, info, trace, warn};
// }

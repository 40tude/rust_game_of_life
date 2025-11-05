// src/lib.rs

pub mod error;
pub mod gol;

// re-export lib from crate root
pub use self::error::{Error, Result};

// src/main.rs

use clap::{Arg, Command};
use std::fs::File;
use std::path::{Path, PathBuf};
use step_13::{Result, app::state::App};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<()> {
    // Activate logs before any window or GPU surface
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("step_13=info, wgpu_core=info, wgpu_hal=warn, wgpu=warn")).init();
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("wgpu_core=info,wgpu_hal=warn,wgpu=warn")).init();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("step_13=info, wgpu_core=info, wgpu_hal=warn, wgpu=off, naga=off")).init();

    log::info!("Logger initialized.");

    // Handle parameters and exit gracefully on error
    let pattern_path = match handle_parameters() {
        Ok(p) => {
            log::info!("Using pattern file: {}", p.display());
            p
        }
        Err(e) => {
            log::error!("Failed to handle parameters: {:?}", e);
            std::process::exit(1);
        }
    };

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::try_new(&pattern_path)?;
    log::info!("App initialized successfully, starting event loop...");

    if let Err(e) = event_loop.run_app(&mut app) {
        log::error!("Application error: {:?}", e);
    }

    log::info!("Application terminated.");
    Ok(())
}

// Handle CLI parameters and return parsed values if valid
fn handle_parameters() -> Result<PathBuf> {
    let cli = Command::new("step_13")
        .version("0.1.0")
        .author("Philippe <philippe@gmail.com>")
        .about("Simple Game of Life")
        .arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .value_name("PATTERN")
                .value_parser(clap::value_parser!(PathBuf)) // specify the PathBuf type
                .help("Path to the pattern file without .rle extension (e.g. \"rle/gosperglidergun\")")
                .required(false),
        )
        .after_help("Example: step_11 --pattern rle/canadagoose");

    let matches = cli.clone().get_matches();

    // Try to get and parse the path to .rle
    let path_to_pattern = match matches.get_one::<PathBuf>("pattern") {
        Some(p) => {
            let mut path = p.clone(); // Clone to get an owned PathBuf
            path.set_extension("rle");
            if !is_valid_file_path(&path) {
                let err_msg = format!("Invalid path to pattern file: {:?}", path);
                log::error!("{err_msg}");
                return Err(err_msg.into());
            }
            path
        }
        None => {
            // Use default pattern from config if no argument provided
            let path = PathBuf::from(step_13::config::DEFAULT_PATTERN_PATH);
            if !is_valid_file_path(&path) {
                let err_msg = format!("Default pattern file not found: {:?}", path);
                log::error!("{err_msg}");
                return Err(err_msg.into());
            }
            log::info!("No pattern specified, using default: {:?}", path);
            path
        }
    };

    Ok(path_to_pattern)
}

// Check if the path points to a valid file
fn is_valid_file_path(path: &Path) -> bool {
    // Check if path exists and is a file
    if !path.exists() || !path.is_file() {
        return false;
    }

    // Try opening the file to ensure it's accessible (permissions OK)
    File::open(path).is_ok()
}

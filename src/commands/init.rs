#[cfg(debug_assertions)]
use crate::debug;

use crossterm::style::Stylize;

use std::{fs::File, io::ErrorKind, process};

use crate::{error, info, toml};

pub fn cli() {
    let proj_dir = std::env::current_dir().unwrap_or_default();
    match File::open("Cargo.toml") {
        Ok(_) => (),
        Err(err) => {
            error!(&format!(
                "Could not find 'Cargo.toml' in current directory. \
                    Make sure your working directory is set to your Rust's project root! \
                    Error: {}",
                err
            ));
            process::exit(0)
        }
    };
    #[cfg(debug_assertions)]
    debug!(&format!("Trying initialization at {}", proj_dir.display()));
    info!("Initializing project...");
    match File::create_new("Cargorush.toml") {
        Ok(_) => {
            info!(&format!("Initialized cargo-rush in {}", proj_dir.display()));
        }
        Err(err) => {
            if err.kind() == ErrorKind::AlreadyExists {
                error!("cargo-rush has already been initialized in this directory!");
                // process::exit(0)
            }
        }
    };
    toml::init(&proj_dir);
}

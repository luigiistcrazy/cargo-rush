#[cfg(debug_assertions)]
use crate::debug;

use crate::info;

pub fn cli() {
    let current_dir = std::env::current_dir().unwrap_or_default();
    #[cfg(debug_assertions)]
    debug!(&format!("Trying initialization at {:?}", current_dir));
    info!("Initializing project...");
}

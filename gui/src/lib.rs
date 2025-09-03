//! # ReactOS GUI en Rust

pub mod window_manager;
pub mod desktop;
pub mod controls;

use anyhow::Result;

pub fn initialize() -> Result<()> {
    log::info!("GUI inicializado");
    Ok(())
}

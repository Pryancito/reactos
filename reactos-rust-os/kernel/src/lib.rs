//! # ReactOS Kernel en Rust

pub mod memory;
pub mod process;
pub mod thread;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelError {
    MemoryError,
    ProcessError,
    ThreadError,
    Unknown,
}

pub type KernelResult<T> = Result<T, KernelError>;

pub const KERNEL_VERSION: &str = "0.1.0";

pub fn initialize() -> Result<()> {
    log::info!("Kernel inicializado");
    Ok(())
}

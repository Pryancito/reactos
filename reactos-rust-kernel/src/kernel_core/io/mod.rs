//! # I/O System
//! 
//! Sistema de entrada/salida del kernel en Rust

pub mod device;
pub mod filesystem;
pub mod network;
pub mod driver;

use core::sync::atomic::{AtomicU64, Ordering};
use crate::kernel_core::memory::{MemoryResult, MemoryError};

// Las definiciones de Device, DeviceManager, etc. están ahora en device.rs

// Re-exportar tipos importantes
pub use device::*;
pub use filesystem::{FileSystemManager, FileSystem, FileSystemType, FileSystemState, SpaceStats};
pub use network::{NetworkManager, NetworkInterface, ProtocolType, NetworkInterfaceState, NetworkStats};
pub use driver::{DriverManager, Driver, DriverType, DriverState, DriverStats};

/// Inicializar el sistema I/O
pub fn init() -> MemoryResult<()> {
    // Inicializar subsistemas I/O
    device::init()?;
    filesystem::init()?;
    network::init()?;
    driver::init()?;
    
    Ok(())
}

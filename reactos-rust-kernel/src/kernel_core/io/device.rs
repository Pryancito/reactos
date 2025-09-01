//! # Device Manager
//! 
//! Gestión de dispositivos del kernel en Rust

use crate::kernel_core::memory::{MemoryResult, MemoryError};
use core::sync::atomic::{AtomicU64, Ordering};

/// Tipo de dispositivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Block,      // Dispositivos de bloque (discos, SSDs)
    Character,  // Dispositivos de carácter (teclado, mouse, serial)
    Network,    // Dispositivos de red (NIC, WiFi)
    Graphics,   // Dispositivos gráficos (GPU, framebuffer)
    Audio,      // Dispositivos de audio
    Storage,    // Dispositivos de almacenamiento
    Unknown,    // Tipo desconocido
}

/// Estado del dispositivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Uninitialized,
    Initializing,
    Ready,
    Busy,
    Error,
    Disconnected,
}

/// Información de un dispositivo
#[derive(Debug)]
pub struct Device {
    pub id: u32,
    pub name: &'static str,
    pub device_type: DeviceType,
    pub state: DeviceState,
    pub major_number: u32,
    pub minor_number: u32,
    pub driver_name: Option<&'static str>,
    pub capabilities: DeviceCapabilities,
    pub last_operation_time: u64,
    pub operation_count: u64,
}

/// Capacidades del dispositivo
#[derive(Debug, Clone, Copy)]
pub struct DeviceCapabilities {
    pub readable: bool,
    pub writable: bool,
    pub seekable: bool,
    pub async_io: bool,
    pub dma_capable: bool,
    pub interrupt_driven: bool,
}

impl DeviceCapabilities {
    pub const fn new() -> Self {
        Self {
            readable: false,
            writable: false,
            seekable: false,
            async_io: false,
            dma_capable: false,
            interrupt_driven: false,
        }
    }

    pub const fn block_device() -> Self {
        Self {
            readable: true,
            writable: true,
            seekable: true,
            async_io: true,
            dma_capable: true,
            interrupt_driven: true,
        }
    }

    pub const fn character_device() -> Self {
        Self {
            readable: true,
            writable: true,
            seekable: false,
            async_io: false,
            dma_capable: false,
            interrupt_driven: true,
        }
    }
}

/// Manager de dispositivos del kernel
pub struct DeviceManager {
    devices: [Option<Device>; 256], // Array fijo para evitar Vec
    next_device_id: AtomicU64,
    device_count: AtomicU64,
    total_operations: AtomicU64,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: [(); 256].map(|_| None),
            next_device_id: AtomicU64::new(1),
            device_count: AtomicU64::new(0),
            total_operations: AtomicU64::new(0),
        }
    }

    /// Registrar un nuevo dispositivo
    pub fn register_device(&mut self, name: &'static str, device_type: DeviceType, major: u32, minor: u32, capabilities: DeviceCapabilities) -> MemoryResult<u32> {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if id >= 256 {
            return Err(MemoryError::OutOfMemory);
        }

        let device = Device {
            id,
            name,
            device_type,
            state: DeviceState::Uninitialized,
            major_number: major,
            minor_number: minor,
            driver_name: None,
            capabilities,
            last_operation_time: 0,
            operation_count: 0,
        };

        self.devices[id as usize] = Some(device);
        self.device_count.fetch_add(1, Ordering::SeqCst);

        Ok(id)
    }

    /// Obtener información de un dispositivo
    pub fn get_device(&self, device_id: u32) -> Option<&Device> {
        if device_id >= 256 {
            return None;
        }
        self.devices[device_id as usize].as_ref()
    }

    /// Obtener dispositivo mutable
    pub fn get_device_mut(&mut self, device_id: u32) -> Option<&mut Device> {
        if device_id >= 256 {
            return None;
        }
        self.devices[device_id as usize].as_mut()
    }

    /// Obtener estadísticas del sistema I/O
    pub fn get_io_stats(&self) -> IoStats {
        IoStats {
            total_devices: self.device_count.load(Ordering::SeqCst),
            total_operations: self.total_operations.load(Ordering::SeqCst),
            active_devices: self.count_active_devices(),
        }
    }

    /// Contar dispositivos activos
    fn count_active_devices(&self) -> u64 {
        let mut count = 0;
        for device in &self.devices {
            if let Some(dev) = device {
                if dev.state == DeviceState::Ready || dev.state == DeviceState::Busy {
                    count += 1;
                }
            }
        }
        count
    }
}

/// Estadísticas del sistema I/O
#[derive(Debug, Clone, Copy)]
pub struct IoStats {
    pub total_devices: u64,
    pub total_operations: u64,
    pub active_devices: u64,
}

/// Inicializar el device manager
pub fn init() -> MemoryResult<()> {
    // En una implementación completa, esto inicializaría:
    // - Device manager
    // - Dispositivos básicos del sistema
    // - Drivers de dispositivos
    
    Ok(())
}

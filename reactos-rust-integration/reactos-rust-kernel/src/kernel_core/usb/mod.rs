//! # USB Support
//! 
//! Soporte USB del kernel en Rust

// pub mod controller; // Comentado para simplificar
// pub mod device;     // Comentado para simplificar
// pub mod driver;     // Comentado para simplificar

use crate::kernel_core::memory::{MemoryResult, MemoryError};
use core::sync::atomic::{AtomicU64, Ordering};

/// Tipo de dispositivo USB
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbDeviceType {
    HID,        // Human Interface Device
    MassStorage, // Almacenamiento masivo
    Audio,      // Dispositivo de audio
    Video,      // Dispositivo de video
    Network,    // Dispositivo de red
    Printer,    // Impresora
    Camera,     // Cámara
    Unknown,    // Tipo desconocido
}

/// Velocidad USB
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UsbSpeed {
    Low,        // 1.5 Mbps
    Full,       // 12 Mbps
    High,       // 480 Mbps
    Super,      // 5 Gbps
    SuperPlus,  // 10 Gbps
}

/// Estado del dispositivo USB
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbDeviceState {
    Disconnected,
    Connected,
    Configured,
    Suspended,
    Error,
}

/// Información del dispositivo USB
#[derive(Debug, Clone, Copy)]
pub struct UsbDeviceInfo {
    pub device_id: u32,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_type: UsbDeviceType,
    pub speed: UsbSpeed,
    pub state: UsbDeviceState,
    pub port_number: u8,
    pub max_packet_size: u16,
    pub power_consumption: u16, // en mA
}

/// Manager USB
pub struct UsbManager {
    devices: [Option<UsbDeviceInfo>; 64], // Array fijo para evitar Vec
    controllers: [Option<UsbControllerInfo>; 8], // Array fijo para evitar Vec
    next_device_id: AtomicU64,
    next_controller_id: AtomicU64,
    device_count: AtomicU64,
    controller_count: AtomicU64,
    total_transfers: AtomicU64,
    successful_transfers: AtomicU64,
    failed_transfers: AtomicU64,
    power_consumption: AtomicU64, // Consumo total en mA
}

/// Información del controlador USB
#[derive(Debug, Clone, Copy)]
pub struct UsbControllerInfo {
    pub controller_id: u32,
    pub controller_type: UsbControllerType,
    pub port_count: u8,
    pub speed_support: [UsbSpeed; 5], // Array fijo para evitar Vec
    pub power_available: u16, // en mA
    pub power_used: u16,      // en mA
}

/// Tipo de controlador USB
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbControllerType {
    OHCI,       // Open Host Controller Interface
    UHCI,       // Universal Host Controller Interface
    EHCI,       // Enhanced Host Controller Interface
    XHCI,       // eXtensible Host Controller Interface
    Unknown,    // Tipo desconocido
}

impl UsbManager {
    pub fn new() -> Self {
        Self {
            devices: [(); 64].map(|_| None),
            controllers: [(); 8].map(|_| None),
            next_device_id: AtomicU64::new(1),
            next_controller_id: AtomicU64::new(1),
            device_count: AtomicU64::new(0),
            controller_count: AtomicU64::new(0),
            total_transfers: AtomicU64::new(0),
            successful_transfers: AtomicU64::new(0),
            failed_transfers: AtomicU64::new(0),
            power_consumption: AtomicU64::new(0),
        }
    }

    /// Registrar controlador USB
    pub fn register_controller(&mut self, controller_type: UsbControllerType, port_count: u8, power_available: u16) -> MemoryResult<u32> {
        let controller_id = self.next_controller_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if controller_id >= 8 {
            return Err(MemoryError::OutOfMemory);
        }

        let speed_support = match controller_type {
            UsbControllerType::OHCI => [UsbSpeed::Low, UsbSpeed::Full, UsbSpeed::Low, UsbSpeed::Low, UsbSpeed::Low],
            UsbControllerType::UHCI => [UsbSpeed::Low, UsbSpeed::Full, UsbSpeed::Low, UsbSpeed::Low, UsbSpeed::Low],
            UsbControllerType::EHCI => [UsbSpeed::Low, UsbSpeed::Full, UsbSpeed::High, UsbSpeed::Low, UsbSpeed::Low],
            UsbControllerType::XHCI => [UsbSpeed::Low, UsbSpeed::Full, UsbSpeed::High, UsbSpeed::Super, UsbSpeed::SuperPlus],
            UsbControllerType::Unknown => [UsbSpeed::Low, UsbSpeed::Low, UsbSpeed::Low, UsbSpeed::Low, UsbSpeed::Low],
        };

        let controller_info = UsbControllerInfo {
            controller_id,
            controller_type,
            port_count,
            speed_support,
            power_available,
            power_used: 0,
        };

        self.controllers[controller_id as usize] = Some(controller_info);
        self.controller_count.fetch_add(1, Ordering::SeqCst);

        Ok(controller_id)
    }

    /// Conectar dispositivo USB
    pub fn connect_device(&mut self, vendor_id: u16, product_id: u16, device_type: UsbDeviceType, speed: UsbSpeed, port_number: u8, power_consumption: u16) -> MemoryResult<u32> {
        let device_id = self.next_device_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if device_id >= 64 {
            return Err(MemoryError::OutOfMemory);
        }

        // Verificar que el puerto esté disponible
        if self.is_port_occupied(port_number) {
            return Err(MemoryError::AlreadyMapped);
        }

        // Verificar disponibilidad de energía
        if !self.check_power_availability(power_consumption) {
            return Err(MemoryError::OutOfMemory);
        }

        let device_info = UsbDeviceInfo {
            device_id,
            vendor_id,
            product_id,
            device_type,
            speed,
            state: UsbDeviceState::Connected,
            port_number,
            max_packet_size: self.get_max_packet_size(speed),
            power_consumption,
        };

        self.devices[device_id as usize] = Some(device_info);
        self.device_count.fetch_add(1, Ordering::SeqCst);
        self.power_consumption.fetch_add(power_consumption as u64, Ordering::SeqCst);

        Ok(device_id)
    }

    /// Desconectar dispositivo USB
    pub fn disconnect_device(&mut self, device_id: u32) -> MemoryResult<()> {
        if device_id >= 64 {
            return Err(MemoryError::InvalidAddress);
        }

        if let Some(device) = &self.devices[device_id as usize] {
            self.power_consumption.fetch_sub(device.power_consumption as u64, Ordering::SeqCst);
            self.devices[device_id as usize] = None;
            self.device_count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener información de dispositivo
    pub fn get_device_info(&self, device_id: u32) -> Option<&UsbDeviceInfo> {
        if device_id >= 64 {
            return None;
        }
        self.devices[device_id as usize].as_ref()
    }

    /// Obtener información de controlador
    pub fn get_controller_info(&self, controller_id: u32) -> Option<&UsbControllerInfo> {
        if controller_id >= 8 {
            return None;
        }
        self.controllers[controller_id as usize].as_ref()
    }

    /// Verificar si un puerto está ocupado
    fn is_port_occupied(&self, port_number: u8) -> bool {
        for device in &self.devices {
            if let Some(dev) = device {
                if dev.port_number == port_number {
                    return true;
                }
            }
        }
        false
    }

    /// Verificar disponibilidad de energía
    fn check_power_availability(&self, power_needed: u16) -> bool {
        let current_power = self.power_consumption.load(Ordering::SeqCst) as u16;
        let max_power = 5000; // 5A máximo por controlador USB
        
        current_power + power_needed <= max_power
    }

    /// Obtener tamaño máximo de paquete según la velocidad
    fn get_max_packet_size(&self, speed: UsbSpeed) -> u16 {
        match speed {
            UsbSpeed::Low => 8,
            UsbSpeed::Full => 64,
            UsbSpeed::High => 512,
            UsbSpeed::Super => 1024,
            UsbSpeed::SuperPlus => 1024,
        }
    }

    /// Realizar transferencia USB
    pub fn transfer_data(&mut self, device_id: u32, endpoint: u8, data: &[u8]) -> MemoryResult<usize> {
        self.total_transfers.fetch_add(1, Ordering::SeqCst);

        // Verificar que el dispositivo existe y está configurado
        if let Some(device) = &self.devices[device_id as usize] {
            if device.state != UsbDeviceState::Configured {
                self.failed_transfers.fetch_add(1, Ordering::SeqCst);
                return Err(MemoryError::PermissionDenied);
            }

            // Simular transferencia
            let bytes_transferred = data.len();
            self.successful_transfers.fetch_add(1, Ordering::SeqCst);
            Ok(bytes_transferred)
        } else {
            self.failed_transfers.fetch_add(1, Ordering::SeqCst);
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Configurar dispositivo USB
    pub fn configure_device(&mut self, device_id: u32) -> MemoryResult<()> {
        if let Some(device) = &mut self.devices[device_id as usize] {
            device.state = UsbDeviceState::Configured;
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Suspender dispositivo USB
    pub fn suspend_device(&mut self, device_id: u32) -> MemoryResult<()> {
        if let Some(device) = &mut self.devices[device_id as usize] {
            device.state = UsbDeviceState::Suspended;
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Reanudar dispositivo USB
    pub fn resume_device(&mut self, device_id: u32) -> MemoryResult<()> {
        if let Some(device) = &mut self.devices[device_id as usize] {
            device.state = UsbDeviceState::Configured;
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener estadísticas USB
    pub fn get_usb_stats(&self) -> UsbStats {
        UsbStats {
            device_count: self.device_count.load(Ordering::SeqCst),
            controller_count: self.controller_count.load(Ordering::SeqCst),
            total_transfers: self.total_transfers.load(Ordering::SeqCst),
            successful_transfers: self.successful_transfers.load(Ordering::SeqCst),
            failed_transfers: self.failed_transfers.load(Ordering::SeqCst),
            power_consumption: self.power_consumption.load(Ordering::SeqCst),
        }
    }
}

/// Estadísticas USB
#[derive(Debug, Clone, Copy)]
pub struct UsbStats {
    pub device_count: u64,
    pub controller_count: u64,
    pub total_transfers: u64,
    pub successful_transfers: u64,
    pub failed_transfers: u64,
    pub power_consumption: u64,
}

/// Inicializar el USB manager
pub fn init() -> MemoryResult<()> {
    // En una implementación completa, esto inicializaría:
    // - USB manager
    // - Controladores USB
    // - Drivers USB
    // - Dispositivos USB
    // - Power management USB
    
    Ok(())
}

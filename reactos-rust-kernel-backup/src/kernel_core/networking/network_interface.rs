//! Network Interface
//! 
//! Implementa la gestión de interfaces de red

use core::sync::atomic::{AtomicU64, Ordering};

/// Network Interface Manager
pub struct NetworkInterface {
    pub interface_count: AtomicU64,
    pub packet_count: AtomicU64,
    pub byte_count: AtomicU64,
    pub error_count: AtomicU64,
    pub interface_state: NetworkInterfaceState,
    pub interfaces: [Option<InterfaceInfo>; 16],
}

/// Estado del Network Interface
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkInterfaceState {
    Initialized,
    Active,
    Paused,
    Error,
}

/// Tipo de interfaz de red
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterfaceType {
    Ethernet,
    Wireless,
    Loopback,
    Virtual,
    Bridge,
    Tunnel,
}

/// Estado de la interfaz
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterfaceStatus {
    Up,
    Down,
    Unknown,
    Testing,
    Dormant,
    NotPresent,
    LowerLayerDown,
}

/// Información de interfaz
#[derive(Debug, Clone, Copy)]
pub struct InterfaceInfo {
    pub interface_id: u32,
    pub interface_type: InterfaceType,
    pub name: &'static str,
    pub mac_address: [u8; 6],
    pub ip_address: u32,
    pub subnet_mask: u32,
    pub gateway: u32,
    pub status: InterfaceStatus,
    pub mtu: u16,
    pub speed: u32,
    pub is_enabled: bool,
}

/// Estadísticas de interfaz
#[derive(Debug, Clone, Copy)]
pub struct InterfaceStats {
    pub interface_id: u32,
    pub packet_count: u64,
    pub byte_count: u64,
    pub error_count: u64,
    pub dropped_packets: u64,
    pub interface_status: InterfaceStatus,
}

/// Estadísticas del Network Interface
#[derive(Debug, Clone, Copy)]
pub struct NetworkInterfaceStats {
    pub interface_count: u64,
    pub packet_count: u64,
    pub byte_count: u64,
    pub error_count: u64,
    pub interface_state: NetworkInterfaceState,
}

impl NetworkInterface {
    /// Crear nuevo Network Interface Manager
    pub fn new() -> Self {
        Self {
            interface_count: AtomicU64::new(0),
            packet_count: AtomicU64::new(0),
            byte_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            interface_state: NetworkInterfaceState::Initialized,
            interfaces: [None; 16],
        }
    }

    /// Registrar una interfaz
    pub fn register_interface(&mut self, interface_info: InterfaceInfo) -> bool {
        if interface_info.interface_id >= 16 {
            return false; // ID fuera de rango
        }

        if self.interfaces[interface_info.interface_id as usize].is_some() {
            return false; // Interfaz ya registrada
        }

        self.interfaces[interface_info.interface_id as usize] = Some(interface_info);
        self.interface_count.fetch_add(1, Ordering::SeqCst);
        true
    }

    /// Desregistrar una interfaz
    pub fn unregister_interface(&mut self, interface_id: u32) -> bool {
        if interface_id >= 16 {
            return false;
        }

        if self.interfaces[interface_id as usize].is_some() {
            self.interfaces[interface_id as usize] = None;
            self.interface_count.fetch_sub(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    /// Obtener información de una interfaz
    pub fn get_interface_info(&self, interface_id: u32) -> Option<InterfaceInfo> {
        if interface_id >= 16 {
            return None;
        }

        self.interfaces[interface_id as usize]
    }

    /// Cambiar estado de una interfaz
    pub fn set_interface_status(&mut self, interface_id: u32, status: InterfaceStatus) -> bool {
        if interface_id >= 16 {
            return false;
        }

        if let Some(interface) = &mut self.interfaces[interface_id as usize] {
            interface.status = status;
            true
        } else {
            false
        }
    }

    /// Habilitar/deshabilitar una interfaz
    pub fn set_interface_enabled(&mut self, interface_id: u32, enabled: bool) -> bool {
        if interface_id >= 16 {
            return false;
        }

        if let Some(interface) = &mut self.interfaces[interface_id as usize] {
            interface.is_enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Procesar paquete en una interfaz
    pub fn process_packet(&self, interface_id: u32, packet_data: &[u8]) -> bool {
        self.packet_count.fetch_add(1, Ordering::SeqCst);
        self.byte_count.fetch_add(packet_data.len() as u64, Ordering::SeqCst);

        if self.interface_state != NetworkInterfaceState::Active {
            self.error_count.fetch_add(1, Ordering::SeqCst);
            return false;
        }

        if interface_id >= 16 {
            self.error_count.fetch_add(1, Ordering::SeqCst);
            return false;
        }

        if let Some(interface) = &self.interfaces[interface_id as usize] {
            if !interface.is_enabled || interface.status != InterfaceStatus::Up {
                self.error_count.fetch_add(1, Ordering::SeqCst);
                return false;
            }

            // Procesar paquete
            self.process_packet_internal(interface, packet_data)
        } else {
            self.error_count.fetch_add(1, Ordering::SeqCst);
            false
        }
    }

    /// Procesamiento interno del paquete
    fn process_packet_internal(&self, interface: &InterfaceInfo, packet_data: &[u8]) -> bool {
        // Implementación simplificada
        // En una implementación real, se procesaría el paquete según el tipo de interfaz
        match interface.interface_type {
            InterfaceType::Ethernet => true,
            InterfaceType::Wireless => true,
            InterfaceType::Loopback => true,
            InterfaceType::Virtual => true,
            InterfaceType::Bridge => true,
            InterfaceType::Tunnel => true,
        }
    }

    /// Buscar interfaces por tipo
    pub fn find_interfaces_by_type(&self, interface_type: InterfaceType) -> u32 {
        let mut count = 0;
        for i in 0..16 {
            if let Some(interface) = &self.interfaces[i] {
                if interface.interface_type == interface_type {
                    count += 1;
                }
            }
        }
        count
    }

    /// Buscar interfaces por estado
    pub fn find_interfaces_by_status(&self, status: InterfaceStatus) -> u32 {
        let mut count = 0;
        for i in 0..16 {
            if let Some(interface) = &self.interfaces[i] {
                if interface.status == status {
                    count += 1;
                }
            }
        }
        count
    }

    /// Obtener estadísticas del manager
    pub fn get_stats(&self) -> NetworkInterfaceStats {
        NetworkInterfaceStats {
            interface_count: self.interface_count.load(Ordering::SeqCst),
            packet_count: self.packet_count.load(Ordering::SeqCst),
            byte_count: self.byte_count.load(Ordering::SeqCst),
            error_count: self.error_count.load(Ordering::SeqCst),
            interface_state: self.interface_state,
        }
    }

    /// Cambiar estado del manager
    pub fn set_state(&mut self, new_state: NetworkInterfaceState) {
        self.interface_state = new_state;
    }

    /// Reiniciar estadísticas
    pub fn reset_stats(&self) {
        self.interface_count.store(0, Ordering::SeqCst);
        self.packet_count.store(0, Ordering::SeqCst);
        self.byte_count.store(0, Ordering::SeqCst);
        self.error_count.store(0, Ordering::SeqCst);
    }

    /// Verificar si el manager está activo
    pub fn is_active(&self) -> bool {
        self.interface_state == NetworkInterfaceState::Active
    }
}

/// Instancia global del Network Interface Manager
static mut NETWORK_INTERFACE: Option<NetworkInterface> = None;

/// Inicializar el Network Interface Manager
pub fn init() {
    unsafe {
        NETWORK_INTERFACE = Some(NetworkInterface::new());
        
        // Registrar interfaces básicas
        let mut interface = NETWORK_INTERFACE.as_mut().unwrap();
        
        // Loopback
        interface.register_interface(InterfaceInfo {
            interface_id: 0,
            interface_type: InterfaceType::Loopback,
            name: "lo",
            mac_address: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            ip_address: 0x7F000001, // 127.0.0.1
            subnet_mask: 0xFF000000, // 255.0.0.0
            gateway: 0x00000000,
            status: InterfaceStatus::Up,
            mtu: 65535,
            speed: 1000000000, // 1 Gbps
            is_enabled: true,
        });

        // Ethernet (ejemplo)
        interface.register_interface(InterfaceInfo {
            interface_id: 1,
            interface_type: InterfaceType::Ethernet,
            name: "eth0",
            mac_address: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            ip_address: 0xC0A80101, // 192.168.1.1
            subnet_mask: 0xFFFFFF00, // 255.255.255.0
            gateway: 0xC0A80101, // 192.168.1.1
            status: InterfaceStatus::Up,
            mtu: 1500,
            speed: 100000000, // 100 Mbps
            is_enabled: true,
        });

        interface.set_state(NetworkInterfaceState::Active);
    }
}

/// Obtener instancia del Network Interface Manager
pub fn get_interface() -> &'static mut NetworkInterface {
    unsafe {
        NETWORK_INTERFACE.as_mut().unwrap()
    }
}

/// Procesar paquete en interfaz (función pública)
pub fn process_packet(interface_id: u32, packet_data: &[u8]) -> bool {
    get_interface().process_packet(interface_id, packet_data)
}

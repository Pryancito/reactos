//! # Network Stack
//! 
//! Stack de red del kernel en Rust

use crate::kernel_core::memory::{MemoryResult, MemoryError};
use core::sync::atomic::{AtomicU64, Ordering};

/// Tipo de protocolo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolType {
    TCP,
    UDP,
    ICMP,
    ARP,
    IPv4,
    IPv6,
    Unknown,
}

/// Estado de la interfaz de red
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkInterfaceState {
    Down,
    Up,
    Initializing,
    Error,
    Disconnected,
}

/// Información de interfaz de red
#[derive(Debug)]
pub struct NetworkInterface {
    pub id: u32,
    pub name: &'static str,
    pub state: NetworkInterfaceState,
    pub device_id: u32,
    pub mac_address: [u8; 6],
    pub ip_address: u32,
    pub netmask: u32,
    pub gateway: u32,
    pub mtu: u16,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub last_activity_time: u64,
}

/// Manager de red
pub struct NetworkManager {
    interfaces: [Option<NetworkInterface>; 16], // Array fijo para evitar Vec
    next_interface_id: AtomicU64,
    interface_count: AtomicU64,
    total_rx_packets: AtomicU64,
    total_tx_packets: AtomicU64,
    total_rx_bytes: AtomicU64,
    total_tx_bytes: AtomicU64,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            interfaces: [(); 16].map(|_| None),
            next_interface_id: AtomicU64::new(1),
            interface_count: AtomicU64::new(0),
            total_rx_packets: AtomicU64::new(0),
            total_tx_packets: AtomicU64::new(0),
            total_rx_bytes: AtomicU64::new(0),
            total_tx_bytes: AtomicU64::new(0),
        }
    }

    /// Registrar una interfaz de red
    pub fn register_interface(&mut self, name: &'static str, device_id: u32, mac_address: [u8; 6], mtu: u16) -> MemoryResult<u32> {
        let id = self.next_interface_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if id >= 16 {
            return Err(MemoryError::OutOfMemory);
        }

        let interface = NetworkInterface {
            id,
            name,
            state: NetworkInterfaceState::Down,
            device_id,
            mac_address,
            ip_address: 0,
            netmask: 0,
            gateway: 0,
            mtu,
            rx_packets: 0,
            tx_packets: 0,
            rx_bytes: 0,
            tx_bytes: 0,
            rx_errors: 0,
            tx_errors: 0,
            last_activity_time: self.get_system_time(),
        };

        self.interfaces[id as usize] = Some(interface);
        self.interface_count.fetch_add(1, Ordering::SeqCst);

        Ok(id)
    }

    /// Configurar IP de interfaz
    pub fn configure_interface(&mut self, interface_id: u32, ip_address: u32, netmask: u32, gateway: u32) -> MemoryResult<()> {
        if let Some(interface) = self.interfaces[interface_id as usize].as_mut() {
            interface.ip_address = ip_address;
            interface.netmask = netmask;
            interface.gateway = gateway;
            interface.state = NetworkInterfaceState::Up;
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener interfaz por ID
    pub fn get_interface(&self, interface_id: u32) -> Option<&NetworkInterface> {
        if interface_id >= 16 {
            return None;
        }
        self.interfaces[interface_id as usize].as_ref()
    }

    /// Obtener interfaz por nombre
    pub fn get_interface_by_name(&self, name: &str) -> Option<&NetworkInterface> {
        for interface in &self.interfaces {
            if let Some(iface) = interface {
                if iface.name == name {
                    return Some(iface);
                }
            }
        }
        None
    }

    /// Obtener todas las interfaces activas (simplificado)
    pub fn get_active_interfaces(&self) -> u32 {
        let mut count = 0;
        for interface in &self.interfaces {
            if let Some(iface) = interface {
                if iface.state == NetworkInterfaceState::Up {
                    count += 1;
                }
            }
        }
        count
    }

    /// Registrar recepción de paquete
    pub fn record_rx_packet(&mut self, interface_id: u32, packet_size: u64) -> MemoryResult<()> {
        let current_time = self.get_system_time();
        if let Some(interface) = self.interfaces[interface_id as usize].as_mut() {
            interface.rx_packets += 1;
            interface.rx_bytes += packet_size;
            interface.last_activity_time = current_time;
            
            self.total_rx_packets.fetch_add(1, Ordering::SeqCst);
            self.total_rx_bytes.fetch_add(packet_size, Ordering::SeqCst);
            
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Registrar envío de paquete
    pub fn record_tx_packet(&mut self, interface_id: u32, packet_size: u64) -> MemoryResult<()> {
        let current_time = self.get_system_time();
        if let Some(interface) = self.interfaces[interface_id as usize].as_mut() {
            interface.tx_packets += 1;
            interface.tx_bytes += packet_size;
            interface.last_activity_time = current_time;
            
            self.total_tx_packets.fetch_add(1, Ordering::SeqCst);
            self.total_tx_bytes.fetch_add(packet_size, Ordering::SeqCst);
            
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Registrar error de recepción
    pub fn record_rx_error(&mut self, interface_id: u32) -> MemoryResult<()> {
        if let Some(interface) = self.interfaces[interface_id as usize].as_mut() {
            interface.rx_errors += 1;
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Registrar error de envío
    pub fn record_tx_error(&mut self, interface_id: u32) -> MemoryResult<()> {
        if let Some(interface) = self.interfaces[interface_id as usize].as_mut() {
            interface.tx_errors += 1;
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener estadísticas de red
    pub fn get_network_stats(&self) -> NetworkStats {
        NetworkStats {
            total_interfaces: self.interface_count.load(Ordering::SeqCst),
            active_interfaces: self.count_active_interfaces(),
            total_rx_packets: self.total_rx_packets.load(Ordering::SeqCst),
            total_tx_packets: self.total_tx_packets.load(Ordering::SeqCst),
            total_rx_bytes: self.total_rx_bytes.load(Ordering::SeqCst),
            total_tx_bytes: self.total_tx_bytes.load(Ordering::SeqCst),
        }
    }

    /// Contar interfaces activas
    fn count_active_interfaces(&self) -> u64 {
        let mut count = 0;
        for interface in &self.interfaces {
            if let Some(iface) = interface {
                if iface.state == NetworkInterfaceState::Up {
                    count += 1;
                }
            }
        }
        count
    }

    /// Obtener tiempo del sistema (simulado)
    fn get_system_time(&self) -> u64 {
        // En una implementación completa, esto obtendría el tiempo real del sistema
        0
    }
}

/// Estadísticas de red
#[derive(Debug, Clone, Copy)]
pub struct NetworkStats {
    pub total_interfaces: u64,
    pub active_interfaces: u64,
    pub total_rx_packets: u64,
    pub total_tx_packets: u64,
    pub total_rx_bytes: u64,
    pub total_tx_bytes: u64,
}

/// Inicializar el network manager
pub fn init() -> MemoryResult<()> {
    // En una implementación completa, esto inicializaría:
    // - Network manager
    // - Stack TCP/IP
    // - Interfaces de red
    // - Drivers de red
    // - Protocolos de red
    
    Ok(())
}

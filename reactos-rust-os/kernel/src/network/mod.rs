//! Módulo de red TCP/IP
//! 
//! Implementa el stack de red básico con soporte para TCP/IP

pub mod ethernet;
pub mod ip;
pub mod tcp;
pub mod udp;
pub mod arp;
pub mod icmp;
pub mod network_manager;

/// Inicializar el stack de red
pub fn init() {
    network_manager::init_network_stack();
}

/// Procesar eventos de red
pub fn process_network_events() {
    network_manager::process_network_events();
}

/// Obtener estadísticas de red
pub fn get_network_statistics() -> network_manager::NetworkStatistics {
    network_manager::get_network_statistics()
}

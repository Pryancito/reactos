//! Sistema de Red Real
//! 
//! Módulo de red completamente funcional para Eclipse OS en Rust

pub mod network_manager;

pub use network_manager::{NetworkManager, NetworkInterface, NetworkConnection, NetworkService, Protocol, ConnectionState, PingResult};

/// Inicializar el sistema de red
pub fn init_network_system() -> NetworkManager {
    println!("🌐 Inicializando sistema de red real...");
    
    let network_manager = NetworkManager::new();
    
    println!("✅ Sistema de red inicializado");
    println!("  - Interfaces de red configuradas");
    println!("  - Servicios de red disponibles");
    println!("  - Monitoreo de red activo");
    
    network_manager
}

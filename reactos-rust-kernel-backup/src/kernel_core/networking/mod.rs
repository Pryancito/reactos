//! Networking Module
//! 
//! Este módulo implementa el sistema de networking que permite al kernel Rust
//! manejar comunicaciones de red de manera segura y eficiente.

pub mod network_stack;
pub mod protocol_stack;
pub mod network_interface;
pub mod network_driver;
pub mod network_security;
pub mod network_monitoring;

// Re-exportar tipos principales
pub use network_stack::{NetworkStack, NetworkStackType, NetworkStackState, NetworkStackConfig, NetworkPacket, NetworkStackStats};
pub use protocol_stack::{ProtocolStack, ProtocolType, ProtocolInfo, ProtocolStats, ProtocolStackStats};
pub use network_interface::{NetworkInterface, InterfaceType, InterfaceStatus, InterfaceInfo, InterfaceStats, NetworkInterfaceStats};
pub use network_driver::{NetworkDriver, DriverType, DriverStatus, DriverInfo, DriverStats, NetworkDriverStats};
pub use network_security::{NetworkSecurity, SecurityRuleType, SecurityAction, SecurityPriority, SecurityRule, SecurityStats};
pub use network_monitoring::{NetworkMonitoring, MonitorType, MonitorStatus, MonitorInfo, MonitorStats, NetworkMonitoringStats};

/// Inicializar el sistema de Networking
pub fn init() {
    network_stack::init();
    protocol_stack::init();
    network_interface::init();
    network_driver::init();
    network_security::init();
    network_monitoring::init();
}

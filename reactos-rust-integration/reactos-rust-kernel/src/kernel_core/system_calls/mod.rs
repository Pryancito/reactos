//! System Calls Module
//! 
//! Este módulo implementa el sistema de llamadas al sistema (system calls)
//! que permite a las aplicaciones x86_64 interactuar con el kernel Rust
//! de manera segura y eficiente.

pub mod syscall_handler;
pub mod syscall_table;
pub mod syscall_interface;
pub mod syscall_validation;
pub mod syscall_performance;

// Re-exportar tipos principales
pub use syscall_handler::*;
pub use syscall_table::*;
pub use syscall_interface::*;
pub use syscall_validation::*;
pub use syscall_performance::*;

/// Inicializar el sistema de System Calls
pub fn init() {
    syscall_handler::init();
    syscall_table::init();
    syscall_interface::init();
    syscall_validation::init();
    syscall_performance::init();
}

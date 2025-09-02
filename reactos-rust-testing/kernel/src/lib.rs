//! ReactOS Rust Kernel Library
//! 
//! Biblioteca del kernel de ReactOS Rust OS
//! Proporciona las funciones y estructuras principales del kernel

#![no_std]

extern crate alloc;

// Módulos del kernel
pub mod memory;
pub mod process;
pub mod scheduler;

// Re-exportar funciones principales
pub use memory::{initialize_memory, get_memory_info, MemoryInfo};
pub use process::{initialize_process_manager, get_process_list, ProcessInfo, ProcessState, ProcessPriority};
pub use scheduler::{initialize_scheduler, get_scheduler_stats, SchedulerStats, SchedulingAlgorithm};

/// Inicializar todos los componentes del kernel
pub fn initialize_kernel() {
    // Inicializar componentes básicos del kernel
    initialize_memory();
    initialize_process_manager();
    initialize_scheduler(SchedulingAlgorithm::RoundRobin);
}

/// Obtener información del sistema
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        memory_info: get_memory_info(),
        process_count: get_process_list().len(),
        scheduler_stats: get_scheduler_stats(),
    }
}

/// Información del sistema
#[derive(Debug)]
pub struct SystemInfo {
    pub memory_info: MemoryInfo,
    pub process_count: usize,
    pub scheduler_stats: Option<SchedulerStats>,
}

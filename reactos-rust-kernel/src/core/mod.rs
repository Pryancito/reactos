//! # Core Kernel Components
//! 
//! Componentes principales del kernel en Rust

pub mod memory;
pub mod process;
pub mod interrupt;

// Re-exportar tipos importantes
pub use memory::{MemoryManager, MemoryResult, MemoryError, MemoryInfo, MemoryFlags};
pub use process::{ProcessManager, Process, Thread, ProcessState, ThreadState, Priority, ProcessId, ThreadId, SystemInfo};
pub use process::scheduler::{Scheduler, SchedulingAlgorithm, SchedulingInfo};
pub use interrupt::{InterruptManager, InterruptResult, InterruptError, InterruptInfo, InterruptStats, InterruptNumber, InterruptType, InterruptPriority};

/// Inicializar todos los componentes del core
pub fn init() -> Result<(), &'static str> {
    // Inicializar memory manager
    memory::init().map_err(|_| "Failed to initialize memory manager")?;
    
    // Inicializar process manager
    process::init().map_err(|_| "Failed to initialize process manager")?;
    
    // Inicializar interrupt manager
    interrupt::init().map_err(|_| "Failed to initialize interrupt manager")?;
    
    Ok(())
}

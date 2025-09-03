//! Implementación real de las funciones del planificador
//! 
//! Este archivo contiene las implementaciones reales de las funciones
//! del planificador que se conectan con las interfaces C

use crate::kernel_core::process::{
    ProcessScheduler, ProcessInfo, ProcessPriority, ProcessState,
    SchedulerStatistics, ProcessManager
};
use core::sync::atomic::{AtomicU64, Ordering};
use core::ptr;

/// Manager global del planificador
static mut SCHEDULER_MANAGER: Option<ProcessScheduler> = None;
static SCHEDULER_INITIALIZED: AtomicU64 = AtomicU64::new(0);

/// Inicializar el planificador
pub unsafe extern "C" fn scheduler_initialize() -> i32 {
    if SCHEDULER_INITIALIZED.load(Ordering::SeqCst) == 1 {
        return 0; // Ya inicializado
    }
    
    // Crear el manager del planificador
    SCHEDULER_MANAGER = Some(ProcessScheduler::new());
    
    if let Some(ref mut manager) = SCHEDULER_MANAGER {
        match manager.initialize() {
            Ok(_) => {
                SCHEDULER_INITIALIZED.store(1, Ordering::SeqCst);
                0 // Éxito
            }
            Err(_) => -1 // Error
        }
    } else {
        -1 // Error
    }
}

/// Cerrar el planificador
pub unsafe extern "C" fn scheduler_shutdown() {
    if SCHEDULER_INITIALIZED.load(Ordering::SeqCst) == 1 {
        if let Some(ref mut manager) = SCHEDULER_MANAGER {
            let _ = manager.shutdown();
        }
        SCHEDULER_MANAGER = None;
        SCHEDULER_INITIALIZED.store(0, Ordering::SeqCst);
    }
}

/// Crear proceso
pub unsafe extern "C" fn scheduler_create_process(process_info: *const ProcessInfo) -> i32 {
    if SCHEDULER_INITIALIZED.load(Ordering::SeqCst) != 1 {
        return -1;
    }
    
    if process_info.is_null() {
        return -1;
    }
    
    if let Some(ref mut manager) = SCHEDULER_MANAGER {
        let rust_process_info = &*process_info;
        match manager.create_process(rust_process_info) {
            Ok(process_id) => process_id as i32,
            Err(_) => -1
        }
    } else {
        -1
    }
}

/// Terminar proceso
pub unsafe extern "C" fn scheduler_terminate_process(process_id: u32) -> i32 {
    if SCHEDULER_INITIALIZED.load(Ordering::SeqCst) != 1 {
        return -1;
    }
    
    if let Some(ref mut manager) = SCHEDULER_MANAGER {
        match manager.terminate_process(process_id) {
            Ok(_) => 0,
            Err(_) => -1
        }
    } else {
        -1
    }
}

/// Establecer prioridad
pub unsafe extern "C" fn scheduler_set_priority(process_id: u32, priority: u32) -> i32 {
    if SCHEDULER_INITIALIZED.load(Ordering::SeqCst) != 1 {
        return -1;
    }
    
    if let Some(ref mut manager) = SCHEDULER_MANAGER {
        let rust_priority = match priority {
            0 => ProcessPriority::Idle,
            1 => ProcessPriority::Low,
            2 => ProcessPriority::Normal,
            3 => ProcessPriority::High,
            4 => ProcessPriority::Realtime,
            _ => return -1
        };
        
        match manager.set_priority(process_id, rust_priority) {
            Ok(_) => 0,
            Err(_) => -1
        }
    } else {
        -1
    }
}

/// Establecer afinidad de CPU
pub unsafe extern "C" fn scheduler_set_affinity(process_id: u32, cpu_affinity: u32) -> i32 {
    if SCHEDULER_INITIALIZED.load(Ordering::SeqCst) != 1 {
        return -1;
    }
    
    if let Some(ref mut manager) = SCHEDULER_MANAGER {
        match manager.set_affinity(process_id, cpu_affinity) {
            Ok(_) => 0,
            Err(_) => -1
        }
    } else {
        -1
    }
}

/// Obtener información del proceso
pub unsafe extern "C" fn scheduler_get_process_info(
    process_id: u32,
    process_info: *mut ProcessInfo
) -> i32 {
    if SCHEDULER_INITIALIZED.load(Ordering::SeqCst) != 1 {
        return -1;
    }
    
    if process_info.is_null() {
        return -1;
    }
    
    if let Some(ref mut manager) = SCHEDULER_MANAGER {
        match manager.get_process_info(process_id) {
            Ok(rust_info) => {
                ptr::copy_nonoverlapping(&rust_info, process_info, 1);
                0
            }
            Err(_) => -1
        }
    } else {
        -1
    }
}

/// Obtener estadísticas del planificador
pub unsafe extern "C" fn scheduler_get_statistics(
    statistics: *mut SchedulerStatistics
) -> i32 {
    if SCHEDULER_INITIALIZED.load(Ordering::SeqCst) != 1 {
        return -1;
    }
    
    if statistics.is_null() {
        return -1;
    }
    
    if let Some(ref mut manager) = SCHEDULER_MANAGER {
        match manager.get_statistics() {
            Ok(rust_stats) => {
                ptr::copy_nonoverlapping(&rust_stats, statistics, 1);
                0
            }
            Err(_) => -1
        }
    } else {
        -1
    }
}

/// Ceder control del procesador
pub unsafe extern "C" fn scheduler_yield() -> i32 {
    if SCHEDULER_INITIALIZED.load(Ordering::SeqCst) != 1 {
        return -1;
    }
    
    if let Some(ref mut manager) = SCHEDULER_MANAGER {
        match manager.yield_cpu() {
            Ok(_) => 0,
            Err(_) => -1
        }
    } else {
        -1
    }
}

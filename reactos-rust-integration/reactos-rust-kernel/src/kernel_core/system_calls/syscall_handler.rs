//! System Call Handler
//! 
//! Maneja la ejecución de system calls desde el espacio de usuario

use core::sync::atomic::{AtomicU64, Ordering};
use crate::kernel_core::system_calls::syscall_table::SyscallTable;
use crate::kernel_core::system_calls::syscall_validation::SyscallValidator;
use crate::kernel_core::system_calls::syscall_performance::SyscallProfiler;

/// Información de un System Call
#[derive(Debug, Clone, Copy)]
pub struct SyscallInfo {
    pub id: u32,
    pub name: &'static str,
    pub handler: fn(u64, u64, u64, u64, u64, u64) -> u64,
    pub flags: SyscallFlags,
    pub category: SyscallCategory,
}

/// Flags de System Call
#[derive(Debug, Clone, Copy)]
pub struct SyscallFlags {
    pub requires_privilege: bool,
    pub can_block: bool,
    pub can_fail: bool,
    pub is_deprecated: bool,
}

/// Categorías de System Call
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyscallCategory {
    Process,
    Memory,
    FileSystem,
    Network,
    Security,
    Hardware,
    System,
    Debug,
}

/// Estado del System Call Handler
#[derive(Debug)]
pub struct SyscallHandler {
    pub syscall_count: AtomicU64,
    pub error_count: AtomicU64,
    pub total_execution_time: AtomicU64,
    pub last_syscall_id: AtomicU64,
    pub handler_state: SyscallHandlerState,
}

/// Estado del handler
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyscallHandlerState {
    Initialized,
    Running,
    Paused,
    Error,
}

impl SyscallHandler {
    /// Crear nuevo System Call Handler
    pub fn new() -> Self {
        Self {
            syscall_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            total_execution_time: AtomicU64::new(0),
            last_syscall_id: AtomicU64::new(0),
            handler_state: SyscallHandlerState::Initialized,
        }
    }

    /// Ejecutar un system call
    pub fn execute_syscall(
        &self,
        syscall_id: u32,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> u64 {
        // Incrementar contador
        self.syscall_count.fetch_add(1, Ordering::SeqCst);
        self.last_syscall_id.store(syscall_id as u64, Ordering::SeqCst);

        // Validar system call
        if !crate::kernel_core::system_calls::syscall_validation::validate_syscall(syscall_id) {
            self.error_count.fetch_add(1, Ordering::SeqCst);
            return 0xFFFFFFFFFFFFFFFF; // Error
        }

        // Obtener información del system call
        if let Some(syscall_info) = crate::kernel_core::system_calls::syscall_table::get_syscall_info(syscall_id) {
            // Verificar privilegios si es necesario
            if syscall_info.flags.requires_privilege {
                if !self.check_privileges() {
                    self.error_count.fetch_add(1, Ordering::SeqCst);
                    return 0xFFFFFFFFFFFFFFFE; // Privilege error
                }
            }

            // Ejecutar system call
            let start_time = self.get_system_time();
            let result = (syscall_info.handler)(arg1, arg2, arg3, arg4, arg5, arg6);
            let end_time = self.get_system_time();

            // Actualizar estadísticas
            let execution_time = end_time - start_time;
            self.total_execution_time.fetch_add(execution_time, Ordering::SeqCst);

            // Registrar en profiler
            crate::kernel_core::system_calls::syscall_performance::record_syscall(syscall_id, execution_time, result);

            result
        } else {
            self.error_count.fetch_add(1, Ordering::SeqCst);
            0xFFFFFFFFFFFFFFFF // System call no encontrado
        }
    }

    /// Verificar privilegios del proceso actual
    fn check_privileges(&self) -> bool {
        // Implementación simplificada - siempre retorna true
        // En una implementación real, verificaría los privilegios del proceso
        true
    }

    /// Obtener tiempo del sistema
    fn get_system_time(&self) -> u64 {
        // Implementación simplificada
        1000000
    }

    /// Obtener estadísticas del handler
    pub fn get_stats(&self) -> SyscallHandlerStats {
        SyscallHandlerStats {
            syscall_count: self.syscall_count.load(Ordering::SeqCst),
            error_count: self.error_count.load(Ordering::SeqCst),
            total_execution_time: self.total_execution_time.load(Ordering::SeqCst),
            last_syscall_id: self.last_syscall_id.load(Ordering::SeqCst),
            handler_state: self.handler_state,
        }
    }

    /// Reiniciar estadísticas
    pub fn reset_stats(&self) {
        self.syscall_count.store(0, Ordering::SeqCst);
        self.error_count.store(0, Ordering::SeqCst);
        self.total_execution_time.store(0, Ordering::SeqCst);
        self.last_syscall_id.store(0, Ordering::SeqCst);
    }

    /// Cambiar estado del handler
    pub fn set_state(&mut self, new_state: SyscallHandlerState) {
        self.handler_state = new_state;
    }
}

/// Estadísticas del System Call Handler
#[derive(Debug, Clone, Copy)]
pub struct SyscallHandlerStats {
    pub syscall_count: u64,
    pub error_count: u64,
    pub total_execution_time: u64,
    pub last_syscall_id: u64,
    pub handler_state: SyscallHandlerState,
}

/// Instancia global del System Call Handler
static mut SYSCALL_HANDLER: Option<SyscallHandler> = None;

/// Inicializar el System Call Handler
pub fn init() {
    unsafe {
        SYSCALL_HANDLER = Some(SyscallHandler::new());
    }
}

/// Obtener instancia del System Call Handler
pub fn get_handler() -> &'static mut SyscallHandler {
    unsafe {
        SYSCALL_HANDLER.as_mut().unwrap()
    }
}

/// Ejecutar system call (función pública)
pub fn execute_syscall(
    syscall_id: u32,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
) -> u64 {
    get_handler().execute_syscall(syscall_id, arg1, arg2, arg3, arg4, arg5, arg6)
}

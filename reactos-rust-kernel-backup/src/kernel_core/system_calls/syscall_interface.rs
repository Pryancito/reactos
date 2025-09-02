//! System Call Interface
//! 
//! Interfaz entre el espacio de usuario y el kernel para system calls

use core::sync::atomic::{AtomicU64, Ordering};
use crate::kernel_core::system_calls::syscall_handler::SyscallHandler;

/// Interfaz de System Calls
pub struct SyscallInterface {
    pub interface_id: u32,
    pub version: u32,
    pub max_syscalls: u32,
    pub interface_state: SyscallInterfaceState,
    pub call_count: AtomicU64,
    pub error_count: AtomicU64,
    pub total_latency: AtomicU64,
}

/// Estado de la interfaz
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyscallInterfaceState {
    Initialized,
    Active,
    Paused,
    Error,
}

/// Tipo de interfaz
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterfaceType {
    Standard,
    Fast,
    Secure,
    Debug,
}

/// Configuración de la interfaz
#[derive(Debug, Clone, Copy)]
pub struct InterfaceConfig {
    pub interface_type: InterfaceType,
    pub max_concurrent_calls: u32,
    pub timeout_ms: u32,
    pub enable_validation: bool,
    pub enable_profiling: bool,
}

impl SyscallInterface {
    /// Crear nueva interfaz de system calls
    pub fn new(interface_id: u32, version: u32) -> Self {
        Self {
            interface_id,
            version,
            max_syscalls: 512,
            interface_state: SyscallInterfaceState::Initialized,
            call_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            total_latency: AtomicU64::new(0),
        }
    }

    /// Configurar la interfaz
    pub fn configure(&mut self, config: InterfaceConfig) -> bool {
        match config.interface_type {
            InterfaceType::Standard => {
                self.max_syscalls = 512;
            }
            InterfaceType::Fast => {
                self.max_syscalls = 256;
            }
            InterfaceType::Secure => {
                self.max_syscalls = 128;
            }
            InterfaceType::Debug => {
                self.max_syscalls = 1024;
            }
        }
        true
    }

    /// Procesar una llamada al sistema
    pub fn process_syscall(
        &self,
        syscall_id: u32,
        arg1: u64,
        arg2: u64,
        arg3: u64,
        arg4: u64,
        arg5: u64,
        arg6: u64,
    ) -> u64 {
        // Incrementar contador de llamadas
        self.call_count.fetch_add(1, Ordering::SeqCst);

        // Verificar estado de la interfaz
        if self.interface_state != SyscallInterfaceState::Active {
            self.error_count.fetch_add(1, Ordering::SeqCst);
            return 0xFFFFFFFFFFFFFFFD; // Interface not active
        }

        // Verificar límites
        if syscall_id >= self.max_syscalls {
            self.error_count.fetch_add(1, Ordering::SeqCst);
            return 0xFFFFFFFFFFFFFFFC; // Syscall ID out of range
        }

        // Medir latencia
        let start_time = self.get_system_time();

        // Procesar system call
        let result = crate::kernel_core::system_calls::syscall_handler::execute_syscall(
            syscall_id, arg1, arg2, arg3, arg4, arg5, arg6
        );

        let end_time = self.get_system_time();
        let latency = end_time - start_time;
        self.total_latency.fetch_add(latency, Ordering::SeqCst);

        // Verificar si hubo error
        if result >= 0xFFFFFFFFFFFFFF00 {
            self.error_count.fetch_add(1, Ordering::SeqCst);
        }

        result
    }

    /// Obtener estadísticas de la interfaz
    pub fn get_stats(&self) -> SyscallInterfaceStats {
        let call_count = self.call_count.load(Ordering::SeqCst);
        let error_count = self.error_count.load(Ordering::SeqCst);
        let total_latency = self.total_latency.load(Ordering::SeqCst);

        let success_count = if call_count > error_count {
            call_count - error_count
        } else {
            0
        };

        let average_latency = if call_count > 0 {
            total_latency / call_count
        } else {
            0
        };

        SyscallInterfaceStats {
            interface_id: self.interface_id,
            version: self.version,
            call_count,
            success_count,
            error_count,
            total_latency,
            average_latency,
            interface_state: self.interface_state,
        }
    }

    /// Cambiar estado de la interfaz
    pub fn set_state(&mut self, new_state: SyscallInterfaceState) {
        self.interface_state = new_state;
    }

    /// Reiniciar estadísticas
    pub fn reset_stats(&self) {
        self.call_count.store(0, Ordering::SeqCst);
        self.error_count.store(0, Ordering::SeqCst);
        self.total_latency.store(0, Ordering::SeqCst);
    }

    /// Obtener tiempo del sistema
    fn get_system_time(&self) -> u64 {
        // Implementación simplificada
        1000000
    }

    /// Verificar si la interfaz está activa
    pub fn is_active(&self) -> bool {
        self.interface_state == SyscallInterfaceState::Active
    }

    /// Obtener información de la interfaz
    pub fn get_info(&self) -> SyscallInterfaceInfo {
        SyscallInterfaceInfo {
            interface_id: self.interface_id,
            version: self.version,
            max_syscalls: self.max_syscalls,
            interface_state: self.interface_state,
        }
    }
}

/// Estadísticas de la interfaz de system calls
#[derive(Debug, Clone, Copy)]
pub struct SyscallInterfaceStats {
    pub interface_id: u32,
    pub version: u32,
    pub call_count: u64,
    pub success_count: u64,
    pub error_count: u64,
    pub total_latency: u64,
    pub average_latency: u64,
    pub interface_state: SyscallInterfaceState,
}

/// Información de la interfaz de system calls
#[derive(Debug, Clone, Copy)]
pub struct SyscallInterfaceInfo {
    pub interface_id: u32,
    pub version: u32,
    pub max_syscalls: u32,
    pub interface_state: SyscallInterfaceState,
}

/// Instancia global de la interfaz de system calls
static mut SYSCALL_INTERFACE: Option<SyscallInterface> = None;

/// Inicializar la interfaz de system calls
pub fn init() {
    unsafe {
        SYSCALL_INTERFACE = Some(SyscallInterface::new(1, 1));
        
        // Configurar interfaz estándar
        let mut interface = SYSCALL_INTERFACE.as_mut().unwrap();
        let config = InterfaceConfig {
            interface_type: InterfaceType::Standard,
            max_concurrent_calls: 1000,
            timeout_ms: 5000,
            enable_validation: true,
            enable_profiling: true,
        };
        interface.configure(config);
        interface.set_state(SyscallInterfaceState::Active);
    }
}

/// Obtener instancia de la interfaz de system calls
pub fn get_interface() -> &'static mut SyscallInterface {
    unsafe {
        SYSCALL_INTERFACE.as_mut().unwrap()
    }
}

/// Procesar system call (función pública)
pub fn process_syscall(
    syscall_id: u32,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
    arg6: u64,
) -> u64 {
    get_interface().process_syscall(syscall_id, arg1, arg2, arg3, arg4, arg5, arg6)
}

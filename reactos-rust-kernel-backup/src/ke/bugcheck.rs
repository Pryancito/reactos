//! # Bug Check System
//! 
//! Sistema de bug check para el kernel en Rust

use core::fmt;

/// Códigos de bug check
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BugCheckCode {
    // Errores del sistema
    SYSTEM_THREAD_EXCEPTION_NOT_HANDLED = 0x7E,
    KERNEL_DATA_INPAGE_ERROR = 0x7A,
    KERNEL_STACK_INPAGE_ERROR = 0x77,
    KERNEL_SECURITY_CHECK_FAILURE = 0x139,
    
    // Errores de excepciones
    UNEXPECTED_KERNEL_MODE_TRAP = 0x7F,
    DOUBLE_FAULT = 0x8,
    
    // Errores de memoria
    MEMORY_MANAGEMENT = 0x1A,
    PFN_LIST_CORRUPT = 0x4E,
    
    // Errores de hardware
    MACHINE_CHECK_EXCEPTION = 0x9C,
    CLOCK_WATCHDOG_TIMEOUT = 0x101,
    
    // Errores de driver
    DRIVER_IRQL_NOT_LESS_OR_EQUAL = 0xD1,
    SYSTEM_SERVICE_EXCEPTION = 0x3B,
}

impl fmt::Display for BugCheckCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BugCheckCode::SYSTEM_THREAD_EXCEPTION_NOT_HANDLED => {
                write!(f, "SYSTEM_THREAD_EXCEPTION_NOT_HANDLED")
            }
            BugCheckCode::KERNEL_DATA_INPAGE_ERROR => {
                write!(f, "KERNEL_DATA_INPAGE_ERROR")
            }
            BugCheckCode::KERNEL_STACK_INPAGE_ERROR => {
                write!(f, "KERNEL_STACK_INPAGE_ERROR")
            }
            BugCheckCode::KERNEL_SECURITY_CHECK_FAILURE => {
                write!(f, "KERNEL_SECURITY_CHECK_FAILURE")
            }
            BugCheckCode::UNEXPECTED_KERNEL_MODE_TRAP => {
                write!(f, "UNEXPECTED_KERNEL_MODE_TRAP")
            }
            BugCheckCode::DOUBLE_FAULT => {
                write!(f, "DOUBLE_FAULT")
            }
            BugCheckCode::MEMORY_MANAGEMENT => {
                write!(f, "MEMORY_MANAGEMENT")
            }
            BugCheckCode::PFN_LIST_CORRUPT => {
                write!(f, "PFN_LIST_CORRUPT")
            }
            BugCheckCode::MACHINE_CHECK_EXCEPTION => {
                write!(f, "MACHINE_CHECK_EXCEPTION")
            }
            BugCheckCode::CLOCK_WATCHDOG_TIMEOUT => {
                write!(f, "CLOCK_WATCHDOG_TIMEOUT")
            }
            BugCheckCode::DRIVER_IRQL_NOT_LESS_OR_EQUAL => {
                write!(f, "DRIVER_IRQL_NOT_LESS_OR_EQUAL")
            }
            BugCheckCode::SYSTEM_SERVICE_EXCEPTION => {
                write!(f, "SYSTEM_SERVICE_EXCEPTION")
            }
        }
    }
}

/// Información del bug check
#[derive(Debug)]
pub struct BugCheckInfo {
    pub code: BugCheckCode,
    pub parameter1: u64,
    pub parameter2: u64,
    pub parameter3: u64,
    pub parameter4: u64,
}

impl BugCheckInfo {
    pub fn new(
        code: BugCheckCode,
        param1: u64,
        param2: u64,
        param3: u64,
        param4: u64,
    ) -> Self {
        Self {
            code,
            parameter1: param1,
            parameter2: param2,
            parameter3: param3,
            parameter4: param4,
        }
    }
}

/// Función principal de bug check (optimizada)
pub fn bugcheck(
    code: BugCheckCode,
    param1: u64,
    param2: u64,
    param3: u64,
    param4: u64,
) -> ! {
    // Log del bug check (si es posible en el contexto del kernel)
    // TODO: Implementar logging cuando esté disponible
    
    // Detener el sistema inmediatamente
    halt_system();
}

/// Detiene el sistema de manera segura
fn halt_system() -> ! {
    // Deshabilitar interrupciones
    unsafe {
        core::arch::asm!("cli");
    }
    
    // Loop infinito
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Macro para bug check con mensaje
#[macro_export]
macro_rules! bugcheck {
    ($code:expr) => {
        $crate::ke::bugcheck::bugcheck($code, 0, 0, 0, 0)
    };
    ($code:expr, $param1:expr) => {
        $crate::ke::bugcheck::bugcheck($code, $param1, 0, 0, 0)
    };
    ($code:expr, $param1:expr, $param2:expr) => {
        $crate::ke::bugcheck::bugcheck($code, $param1, $param2, 0, 0)
    };
    ($code:expr, $param1:expr, $param2:expr, $param3:expr) => {
        $crate::ke::bugcheck::bugcheck($code, $param1, $param2, $param3, 0)
    };
    ($code:expr, $param1:expr, $param2:expr, $param3:expr, $param4:expr) => {
        $crate::ke::bugcheck::bugcheck($code, $param1, $param2, $param3, $param4)
    };
}

/// Macro para verificar condiciones y hacer bug check si fallan
#[macro_export]
macro_rules! assert_kernel {
    ($condition:expr, $code:expr) => {
        if !$condition {
            $crate::ke::bugcheck::bugcheck($code, 0, 0, 0, 0);
        }
    };
    ($condition:expr, $code:expr, $param1:expr) => {
        if !$condition {
            $crate::ke::bugcheck::bugcheck($code, $param1, 0, 0, 0);
        }
    };
}

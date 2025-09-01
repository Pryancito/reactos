//! # Foreign Function Interface
//! 
//! Enlaces entre C y Rust para interoperabilidad

use core::ffi::c_void;

pub mod kernel_bindings;

/// Estructura de trap frame compatible con C
#[repr(C)]
pub struct CTrapFrame {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
    pub rip: u64,
    pub cs: u16,
    pub rflags: u64,
    pub error_code: u64,
    pub previous_mode: u8,
    pub reserved: [u8; 7],
}

/// Función C para manejar Invalid Opcode
#[no_mangle]
pub extern "C" fn KiHandleInvalidOpcodeCompat(trap_frame: *mut CTrapFrame) -> u32 {
    if trap_frame.is_null() {
        return 0; // STATUS_UNSUCCESSFUL
    }
    
    let trap_frame = unsafe { &mut *trap_frame };
    
    // Convertir a TrapFrame de Rust
    let mut rust_trap_frame = crate::arch::x64::trap_frame::TrapFrame {
        rax: trap_frame.rax,
        rcx: trap_frame.rcx,
        rdx: trap_frame.rdx,
        rbx: trap_frame.rbx,
        rsp: trap_frame.rsp,
        rbp: trap_frame.rbp,
        rsi: trap_frame.rsi,
        rdi: trap_frame.rdi,
        r8: trap_frame.r8,
        r9: trap_frame.r9,
        r10: trap_frame.r10,
        r11: trap_frame.r11,
        r12: trap_frame.r12,
        r13: trap_frame.r13,
        r14: trap_frame.r14,
        r15: trap_frame.r15,
        ds: trap_frame.ds,
        es: trap_frame.es,
        fs: trap_frame.fs,
        gs: trap_frame.gs,
        ss: trap_frame.ss,
        rip: trap_frame.rip,
        cs: trap_frame.cs,
        rflags: trap_frame.rflags,
        error_code: trap_frame.error_code,
        previous_mode: trap_frame.previous_mode,
        reserved: trap_frame.reserved,
    };
    
    // Manejar la excepción
    match crate::ke::exception::handle_exception(&mut rust_trap_frame, crate::ke::exception::ExceptionCode::InvalidOpcode) {
        crate::ke::exception::ExceptionResult::Handled => {
            // Actualizar el trap frame original
            trap_frame.rip = rust_trap_frame.rip;
            1 // STATUS_SUCCESS
        }
        _ => 0 // STATUS_UNSUCCESSFUL
    }
}

/// Función C para bug check
#[no_mangle]
pub extern "C" fn KiBugCheck(code: u32, param1: u64, param2: u64, param3: u64, param4: u64) -> ! {
    let bug_check_code = match code {
        0x7E => crate::ke::bugcheck::BugCheckCode::SYSTEM_THREAD_EXCEPTION_NOT_HANDLED,
        0x7A => crate::ke::bugcheck::BugCheckCode::KERNEL_DATA_INPAGE_ERROR,
        0x77 => crate::ke::bugcheck::BugCheckCode::KERNEL_STACK_INPAGE_ERROR,
        0x139 => crate::ke::bugcheck::BugCheckCode::KERNEL_SECURITY_CHECK_FAILURE,
        0x7F => crate::ke::bugcheck::BugCheckCode::UNEXPECTED_KERNEL_MODE_TRAP,
        0x8 => crate::ke::bugcheck::BugCheckCode::DOUBLE_FAULT,
        0x1A => crate::ke::bugcheck::BugCheckCode::MEMORY_MANAGEMENT,
        0x4E => crate::ke::bugcheck::BugCheckCode::PFN_LIST_CORRUPT,
        0x9C => crate::ke::bugcheck::BugCheckCode::MACHINE_CHECK_EXCEPTION,
        0x101 => crate::ke::bugcheck::BugCheckCode::CLOCK_WATCHDOG_TIMEOUT,
        0xD1 => crate::ke::bugcheck::BugCheckCode::DRIVER_IRQL_NOT_LESS_OR_EQUAL,
        0x3B => crate::ke::bugcheck::BugCheckCode::SYSTEM_SERVICE_EXCEPTION,
        _ => crate::ke::bugcheck::BugCheckCode::UNEXPECTED_KERNEL_MODE_TRAP,
    };
    
    crate::ke::bugcheck::bugcheck(bug_check_code, param1, param2, param3, param4);
}

/// Función C para inicializar el kernel Rust
#[no_mangle]
pub extern "C" fn KiInitializeRustKernel() -> u32 {
    // Inicializar componentes del kernel Rust
    crate::arch::init();
    crate::ke::init();
    crate::mm::init();
    crate::io::init();
    crate::ps::init();
    
    1 // STATUS_SUCCESS
}

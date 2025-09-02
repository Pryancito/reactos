//! # Exception Handling
//! 
//! Manejo seguro de excepciones del kernel en Rust

use core::ptr;
use crate::arch::x64::trap_frame::TrapFrame;
use crate::ke::bugcheck::{BugCheckCode, bugcheck};

/// Códigos de excepción
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionCode {
    InvalidOpcode = 0x06,
    GeneralProtection = 0x0D,
    PageFault = 0x0E,
    DoubleFault = 0x08,
    StackFault = 0x0C,
}

/// Resultado del manejo de excepciones
#[derive(Debug)]
pub enum ExceptionResult {
    /// Excepción manejada exitosamente
    Handled,
    /// Excepción no manejada, debe causar bugcheck
    Unhandled,
    /// Excepción que debe ser enviada al usuario
    UserMode,
}

/// Handler principal de excepciones
pub fn handle_exception(
    trap_frame: &mut TrapFrame,
    exception_code: ExceptionCode,
) -> ExceptionResult {
    match exception_code {
        ExceptionCode::InvalidOpcode => handle_invalid_opcode(trap_frame),
        ExceptionCode::GeneralProtection => handle_general_protection(trap_frame),
        ExceptionCode::PageFault => handle_page_fault(trap_frame),
        ExceptionCode::DoubleFault => handle_double_fault(trap_frame),
        ExceptionCode::StackFault => handle_stack_fault(trap_frame),
    }
}

/// Manejo seguro de Invalid Opcode en Rust
fn handle_invalid_opcode(trap_frame: &mut TrapFrame) -> ExceptionResult {
    // Verificar si estamos en modo kernel
    if trap_frame.is_kernel_mode() {
        // Intentar manejar la excepción de manera segura
        match try_handle_invalid_opcode(trap_frame) {
            Ok(()) => ExceptionResult::Handled,
            Err(_) => ExceptionResult::Unhandled,
        }
    } else {
        // En modo usuario, enviar la excepción al proceso
        ExceptionResult::UserMode
    }
}

/// Intenta manejar una excepción de Invalid Opcode (optimizado)
fn try_handle_invalid_opcode(trap_frame: &mut TrapFrame) -> Result<(), InvalidOpcodeError> {
    // Optimización: leer múltiples bytes de una vez para mejor rendimiento
    let instruction_ptr = (trap_frame.rip - 1) as *const u16;
    let instruction_word = unsafe { ptr::read(instruction_ptr) };

    // Verificar patrones comunes de instrucciones SSE4.1/SSE4.2
    match instruction_word {
        0x380F => {
            // Patrón 0x0F 0x38 - instrucciones SSE4.1
            trap_frame.rip += 3;
            Ok(())
        }
        0x3A0F => {
            // Patrón 0x0F 0x3A - instrucciones SSE4.2
            trap_frame.rip += 3;
            Ok(())
        }
        _ => {
            // Fallback: verificar byte por byte para casos especiales
            let instruction_byte = unsafe { ptr::read((trap_frame.rip - 1) as *const u8) };
            if instruction_byte == 0x0F {
                let next_byte = unsafe { ptr::read((trap_frame.rip) as *const u8) };
                if next_byte == 0x38 || next_byte == 0x3A {
                    trap_frame.rip += 3;
                    Ok(())
                } else {
                    Err(InvalidOpcodeError::UnknownInstruction)
                }
            } else {
                Err(InvalidOpcodeError::UnknownInstruction)
            }
        }
    }
}

/// Errores específicos de Invalid Opcode
#[derive(Debug, Clone, Copy)]
pub enum InvalidOpcodeError {
    UnknownInstruction,
    UnsupportedInstruction,
    MemoryAccessViolation,
}

/// Manejo de General Protection Fault
fn handle_general_protection(trap_frame: &mut TrapFrame) -> ExceptionResult {
    if trap_frame.is_kernel_mode() {
        ExceptionResult::Unhandled
    } else {
        ExceptionResult::UserMode
    }
}

/// Manejo de Page Fault
fn handle_page_fault(_trap_frame: &mut TrapFrame) -> ExceptionResult {
    // TODO: Implementar manejo de page fault
    ExceptionResult::Unhandled
}

/// Manejo de Double Fault
fn handle_double_fault(trap_frame: &mut TrapFrame) -> ExceptionResult {
    bugcheck(
        BugCheckCode::DOUBLE_FAULT,
        trap_frame.rip,
        trap_frame.rsp,
        0,
        0
    );
}

/// Manejo de Stack Fault
fn handle_stack_fault(_trap_frame: &mut TrapFrame) -> ExceptionResult {
    ExceptionResult::Unhandled
}

/// Handler de excepciones no manejadas
pub fn handle_unhandled_exception(
    trap_frame: &TrapFrame,
    exception_code: ExceptionCode,
) -> ! {
    bugcheck(
        BugCheckCode::UNEXPECTED_KERNEL_MODE_TRAP,
        exception_code as u64,
        trap_frame.rip,
        trap_frame.rsp,
        0
    );
}

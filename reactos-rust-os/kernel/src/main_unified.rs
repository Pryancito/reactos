//! ReactOS Rust Kernel - Unified Binary Entry Point
//! 
//! Punto de entrada unificado para el kernel de ReactOS Rust OS
//! Combina funcionalidades del Kernel 1 y Kernel 2

#![no_std]
#![no_main]

extern crate reactos_rust_kernel;

use reactos_rust_kernel::KiSystemStartup;

/// Punto de entrada del kernel unificado
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Llamar al punto de entrada unificado
    KiSystemStartup();
}

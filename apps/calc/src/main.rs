//! ReactOS Rust Calculator
//! 
//! Calculadora del sistema en Rust para ReactOS Rust OS.

#![no_std]
#![no_main]

/// Función principal de la calculadora
#[no_mangle]
pub extern "C" fn main() {
    // TODO: Implementar calculadora
}

/// Panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
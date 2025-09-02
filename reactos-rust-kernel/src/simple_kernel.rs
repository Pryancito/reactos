//! Kernel simple para ReactOS Rust con header multiboot funcional

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Header multiboot en assembly inline
#[naked]
#[no_mangle]
#[link_section = ".multiboot"]
pub extern "C" fn multiboot_header() {
    unsafe {
        core::arch::asm!(
            ".align 4",
            ".long 0x1BADB002",  // magic
            ".long 0x00000000",  // flags
            ".long 0xE4524FFB",  // checksum
            options(noreturn)
        );
    }
}

/// Punto de entrada del kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Inicializar VGA básico
    init_vga();
    
    // Mostrar mensaje de bienvenida
    print_welcome();
    
    // Loop infinito
    loop {
        // Halt CPU
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

fn init_vga() {
    // Inicialización básica de VGA
    let vga_buffer = 0xb8000 as *mut u16;
    
    // Limpiar pantalla
    for i in 0..80 * 25 {
        unsafe {
            *vga_buffer.add(i) = 0x0F20; // Blanco sobre negro, espacio
        }
    }
}

fn print_welcome() {
    let vga_buffer = 0xb8000 as *mut u16;
    let message = b"ReactOS Rust Kernel v1.0 - AI en Tiempo Real!";
    
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.add(i) = 0x0F00 | byte as u16; // Blanco sobre negro
        }
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u16;
    let message = b"PANIC: Kernel crashed!";
    
    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *vga_buffer.add(i) = 0x0C00 | byte as u16; // Rojo sobre negro
        }
    }
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

//! Kernel de prueba ultra simple para ReactOS Rust
//! 
//! Este kernel solo muestra un mensaje en VGA y se queda en un loop infinito

#![no_std]
#![no_main]

// Incluir header multiboot
mod multiboot_header;

use core::arch::asm;
use core::panic::PanicInfo;

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

/// Punto de entrada del kernel (llamado por el bootloader)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Inicializar VGA básico
    init_vga();
    
    // Mostrar mensaje de prueba
    print_string("ReactOS Rust Kernel - PRUEBA SIMPLE");
    print_string("Kernel arrancado correctamente!");
    print_string("VGA funcionando");
    print_string("EXITO! El kernel funciona!");
    
    // Loop infinito
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

/// Inicializar VGA básico
fn init_vga() {
    // VGA text buffer está en 0xB8000
    let vga_buffer = 0xB8000 as *mut u16;
    
    // Limpiar pantalla (rellenar con espacios en blanco)
    for i in 0..80 * 25 {
        unsafe {
            *vga_buffer.add(i) = 0x0720; // Gris claro sobre negro, espacio
        }
    }
}

/// Imprimir string en VGA
fn print_string(s: &str) {
    static mut VGA_ROW: usize = 0;
    static mut VGA_COL: usize = 0;
    
    let vga_buffer = 0xB8000 as *mut u16;
    
    unsafe {
        for byte in s.bytes() {
            if VGA_ROW >= 25 {
                // Scroll pantalla
                scroll_screen();
                VGA_ROW = 24;
            }
            
            if byte == b'\n' {
                VGA_ROW += 1;
                VGA_COL = 0;
            } else {
                let color = 0x07; // Gris claro sobre negro
                let character = (color as u16) << 8 | byte as u16;
                
                *vga_buffer.add(VGA_ROW * 80 + VGA_COL) = character;
                
                VGA_COL += 1;
                if VGA_COL >= 80 {
                    VGA_COL = 0;
                    VGA_ROW += 1;
                }
            }
        }
        
        // Nueva línea después del string
        VGA_ROW += 1;
        VGA_COL = 0;
    }
}

/// Scroll de pantalla
fn scroll_screen() {
    let vga_buffer = 0xB8000 as *mut u16;
    
    // Mover todas las líneas una posición hacia arriba
    for row in 0..24 {
        for col in 0..80 {
            unsafe {
                let src = *vga_buffer.add((row + 1) * 80 + col);
                *vga_buffer.add(row * 80 + col) = src;
            }
        }
    }
    
    // Limpiar la última línea
    for col in 0..80 {
        unsafe {
            *vga_buffer.add(24 * 80 + col) = 0x0720;
        }
    }
}

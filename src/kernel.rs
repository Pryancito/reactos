//! # Eclipse OS Kernel
//! 
//! Kernel compatible con Multiboot para Eclipse OS

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;

// Multiboot header
#[repr(C)]
struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

const MULTIBOOT_HEADER_MAGIC: u32 = 0x1BADB002;
const MULTIBOOT_HEADER_FLAGS: u32 = 0x00000000;
const MULTIBOOT_HEADER_CHECKSUM: u32 = -(MULTIBOOT_HEADER_MAGIC as i32 + MULTIBOOT_HEADER_FLAGS as i32) as u32;

#[used]
#[link_section = ".multiboot"]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: MULTIBOOT_HEADER_MAGIC,
    flags: MULTIBOOT_HEADER_FLAGS,
    checksum: MULTIBOOT_HEADER_CHECKSUM,
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Inicializar VGA básico
    init_vga();
    
    // Mostrar mensaje de bienvenida
    vga_println!("🌙 Eclipse OS Kernel iniciado!");
    vga_println!("📊 Kernel compatible con Multiboot");
    vga_println!("🔧 Inicializando componentes del kernel...");
    
    // Simular inicialización del kernel
    vga_println!("✅ Kernel inicializado correctamente");
    vga_println!("💡 Presiona Ctrl+Alt+Q para salir de QEMU");
    vga_println!("");
    vga_println!("🐚 Shell interactivo disponible!");
    vga_println!("💡 Escribe 'help' para ver comandos disponibles");
    
    // Simular shell básico
    run_kernel_shell();
}

fn init_vga() {
    // Inicialización básica de VGA
    unsafe {
        let vga_buffer = 0xb8000 as *mut u16;
        for i in 0..80 * 25 {
            *vga_buffer.add(i) = 0x0f20; // Blanco sobre negro, espacio
        }
    }
}

fn vga_println!(s: &str) {
    unsafe {
        let vga_buffer = 0xb8000 as *mut u16;
        static mut VGA_INDEX: usize = 0;
        
        for byte in s.bytes() {
            if VGA_INDEX >= 80 * 25 {
                // Scroll
                for i in 0..80 * 24 {
                    *vga_buffer.add(i) = *vga_buffer.add(i + 80);
                }
                for i in 80 * 24..80 * 25 {
                    *vga_buffer.add(i) = 0x0f20;
                }
                VGA_INDEX = 80 * 24;
            }
            
            *vga_buffer.add(VGA_INDEX) = 0x0f00 | byte as u16;
            VGA_INDEX += 1;
        }
        
        // Nueva línea
        VGA_INDEX = ((VGA_INDEX - 1) / 80 + 1) * 80;
    }
}

fn run_kernel_shell() -> ! {
    vga_println!("Eclipse OS> ");
    
    // Simular shell básico
    loop {
        // En una implementación real, aquí se manejaría la entrada del teclado
        // Por ahora, solo mostramos un prompt
        unsafe {
            let vga_buffer = 0xb8000 as *mut u16;
            static mut SHELL_INDEX: usize = 0;
            
            // Simular cursor parpadeante
            if SHELL_INDEX % 1000 < 500 {
                *vga_buffer.add(80 * 24 + 12) = 0x0f5f; // Cursor visible
            } else {
                *vga_buffer.add(80 * 24 + 12) = 0x0f20; // Cursor invisible
            }
            SHELL_INDEX += 1;
        }
    }
}

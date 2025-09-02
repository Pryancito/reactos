//! ReactOS Rust Kernel - Bare Metal Entry Point
//! 
//! Punto de entrada para el kernel en entorno bare metal
//! Optimizado para testing y desarrollo

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use core::arch::asm;
use spin::Mutex;
use alloc::alloc::{GlobalAlloc, Layout};

// Módulos del kernel
mod memory;
mod process;
mod scheduler;
mod vga;
mod multiboot_header;

// Re-exportar funciones principales
use memory::{initialize_memory, get_memory_info};
use process::{initialize_process_manager, get_process_list};
use scheduler::{initialize_scheduler, get_scheduler_stats, SchedulingAlgorithm};

// Global allocator simple
struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op
    }
}

#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;

/// Panic handler para bare metal
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // En un entorno real, esto debería escribir a VGA o serial
    // Por ahora, simplemente halt
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

/// Función de inicialización del kernel
fn kernel_init() {
    // Inicializar componentes básicos del kernel
    initialize_memory();
    initialize_process_manager();
    initialize_scheduler(SchedulingAlgorithm::RoundRobin);
}

/// Función principal del kernel
fn kernel_main() -> ! {
    // Inicializar VGA primero
    vga::init();
    
    // Inicializar el kernel
    kernel_init();
    
    // Mostrar mensaje de bienvenida
    vga_println!("🎉 ReactOS Rust Kernel iniciado correctamente!");
    vga_println!("📊 Sistema inicializado:");
    
    // Obtener información del sistema
    let memory_info = get_memory_info();
    let process_list = get_process_list();
    let scheduler_stats = get_scheduler_stats();
    
    vga_println!("   • Memoria total: {} MB", memory_info.total_memory / (1024 * 1024));
    vga_println!("   • Memoria libre: {} MB", memory_info.free_memory / (1024 * 1024));
    vga_println!("   • Procesos activos: {}", process_list.len());
    if let Some(stats) = scheduler_stats {
        vga_println!("   • Context switches: {}", stats.context_switches);
    }
    
    vga_println!("🚀 Kernel funcionando en modo bare metal!");
    vga_println!("💡 Presiona Ctrl+Alt+Q para salir de QEMU");
    
    // Bucle principal del kernel
    kernel_loop();
}

/// Bucle principal del kernel
fn kernel_loop() -> ! {
    let mut counter = 0u64;
    
    loop {
        // Simular trabajo del kernel
        counter += 1;
        
        // Cada 1000 iteraciones, verificar el estado del sistema
        if counter % 1000 == 0 {
            // En un entorno real, aquí se verificarían interrupciones
            // y se planificarían procesos
        }
        
        // Halt del procesador para ahorrar energía
        unsafe {
            asm!("hlt");
        }
    }
}

/// Punto de entrada del kernel (llamado por el bootloader)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main()
}

/// Función de inicio alternativa para testing
#[no_mangle]
pub extern "C" fn kernel_entry() -> ! {
    kernel_main()
}

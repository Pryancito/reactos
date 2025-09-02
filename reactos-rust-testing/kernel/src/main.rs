//! ReactOS Rust Kernel - Binary Entry Point
//! 
//! Punto de entrada para el kernel de ReactOS Rust OS

extern crate reactos_rust_kernel;

use reactos_rust_kernel::{initialize_kernel, get_system_info};

// Incluir header multiboot
mod multiboot_header;

fn main() {
    println!("🚀 Inicializando ReactOS Rust Kernel...");
    
    // Inicializar el kernel
    initialize_kernel();
    
    println!("✅ ReactOS Rust Kernel inicializado correctamente");
    
    // Obtener información del sistema
    let system_info = get_system_info();
    
    println!("📊 Información del sistema:");
    println!("   • Memoria total: {} MB", system_info.memory_info.total_memory / (1024 * 1024));
    println!("   • Memoria libre: {} MB", system_info.memory_info.free_memory / (1024 * 1024));
    println!("   • Procesos activos: {}", system_info.process_count);
    
    if let Some(stats) = system_info.scheduler_stats {
        println!("   • Context switches: {}", stats.context_switches);
    }
    
    println!("🎉 ReactOS Rust Kernel funcionando correctamente!");
}
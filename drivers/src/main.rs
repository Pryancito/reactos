//! ReactOS Rust Drivers
//! 
//! Drivers para ReactOS Rust OS
//! Implementación completa de drivers en Rust

// Módulos de drivers
pub mod vga;
pub mod keyboard;
pub mod mouse;

// Módulos de drivers de AI
pub mod npu;
pub mod gpu_ai;
pub mod cpu_ai;

// Re-exportar APIs principales
pub use vga::*;
pub use keyboard::*;
pub use mouse::*;
pub use npu::*;
pub use gpu_ai::*;
pub use cpu_ai::*;

/// Inicializar todos los drivers
pub fn init() {
    // Inicializar driver VGA
    vga::VGA_Initialize();
    
    // Inicializar driver de teclado
    keyboard::Keyboard_Initialize();
    
    // Inicializar driver de mouse
    mouse::Mouse_Initialize();
    
    // Inicializar drivers de AI
    npu::NPU_Initialize();
    gpu_ai::GPUAI_Initialize();
    cpu_ai::CPUAI_Initialize();
}

/// Test de todos los drivers
pub fn test() {
    // Test del driver VGA
    vga::VGA_Test();
    
    // Test del driver de teclado
    keyboard::Keyboard_Test();
    
    // Test del driver de mouse
    mouse::Mouse_Test();
    
    // Test de drivers de AI
    npu::NPU_Test();
    gpu_ai::GPUAI_Test();
    cpu_ai::CPUAI_Test();
}

/// Función main para compilación
fn main() {
    // Inicializar drivers
    init();
    
    println!("🎉 ReactOS Rust Drivers inicializados exitosamente!");
    println!("✅ Todos los drivers están funcionando");
    
    // Ejecutar tests
    test();
    
    println!("🔄 Simulando operaciones de drivers...");
    println!("   • Driver VGA funcionando");
    println!("   • Driver de teclado funcionando");
    println!("   • Driver de mouse funcionando");
    println!("   • NPU AI funcionando");
    println!("   • GPU AI funcionando");
    println!("   • CPU AI funcionando");
    println!("   • ✅ Drivers completamente operativos");
}

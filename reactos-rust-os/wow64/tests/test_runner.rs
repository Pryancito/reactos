//! Ejecutor de pruebas para WOW64
//!
//! Punto de entrada para ejecutar todas las pruebas de integración

use std::env;
use std::process;

mod integration_tests;

/// Función principal del ejecutor de pruebas
fn main() {
    println!("🚀 ReactOS WOW64 - Ejecutor de Pruebas");
    println!("=====================================\n");

    // Verificar argumentos de línea de comandos
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--version" | "-v" => {
                print_version();
                return;
            }
            "--integration" | "-i" => {
                run_integration_tests();
                return;
            }
            _ => {
                println!("❌ Argumento desconocido: {}", args[1]);
                print_help();
                process::exit(1);
            }
        }
    }

    // Ejecutar todas las pruebas por defecto
    run_all_tests();
}

/// Ejecutar todas las pruebas
fn run_all_tests() {
    println!("🧪 Ejecutando todas las pruebas de WOW64...\n");
    
    // Ejecutar pruebas de integración
    integration_tests::run_integration_tests();
    
    println!("\n🎉 Todas las pruebas completadas!");
}

/// Ejecutar solo las pruebas de integración
fn run_integration_tests() {
    println!("🔧 Ejecutando pruebas de integración...\n");
    integration_tests::run_integration_tests();
}

/// Mostrar ayuda
fn print_help() {
    println!("ReactOS WOW64 - Ejecutor de Pruebas");
    println!("===================================");
    println!();
    println!("Uso: test_runner [OPCIÓN]");
    println!();
    println!("Opciones:");
    println!("  -h, --help      Mostrar esta ayuda");
    println!("  -v, --version   Mostrar versión");
    println!("  -i, --integration  Ejecutar solo pruebas de integración");
    println!();
    println!("Sin argumentos, ejecuta todas las pruebas.");
}

/// Mostrar versión
fn print_version() {
    println!("ReactOS WOW64 Test Runner v1.0.0");
    println!("Compatibilidad con aplicaciones Windows 32-bit");
    println!("Arquitectura: x86_64");
    println!("Rust: {}", env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()));
}

//! Sistema de Build Optimizado para ReactOS Rust OS
//!
//! Maneja la compilación automática para múltiples arquitecturas

use std::process::Command;
use std::env;

/// Función principal
fn main() {
    println!("🔧 Sistema de Build ReactOS Rust OS");
    println!("====================================");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "info" => {
            show_info();
        }
        "install-targets" => {
            install_targets();
        }
        "check-targets" => {
            check_targets();
        }
        "build-native" => {
            build_native();
        }
        "build-32bit" => {
            build_32bit();
        }
        "build-uefi" => {
            build_uefi();
        }
        "build-all" => {
            build_all();
        }
        "test-all" => {
            test_all();
        }
        "clean-all" => {
            clean_all();
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            println!("❌ Comando desconocido: {}", args[1]);
            print_help();
        }
    }
}

/// Mostrar información del sistema
fn show_info() {
    println!("🔧 Sistema de Build ReactOS Rust OS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📁 Workspace: {}", env::current_dir().unwrap().display());
    println!("📁 Build dir: target");
    println!("⚙️  Modo: Release");
    println!("🔢 Jobs paralelos: 1");
    println!();
    
    println!("🎯 Targets configurados:");
    println!("  ✅ native-64bit (x86_64-unknown-linux-gnu) - Arquitectura nativa 64-bit (Linux)");
    println!("  ✅ compat-32bit (i686-unknown-linux-gnu) - Arquitectura compatible 32-bit (Linux)");
    println!("  ✅ uefi-bootloader (x86_64-unknown-uefi) - Bootloader UEFI 64-bit");
    println!("  ❌ windows-32bit (i686-pc-windows-msvc) - Windows 32-bit (MSVC) - Deshabilitado");
    println!("  ❌ windows-64bit (x86_64-pc-windows-msvc) - Windows 64-bit (MSVC) - Deshabilitado");
}

/// Instalar targets necesarios
fn install_targets() {
    println!("🔧 Instalando targets necesarios...");
    
    let targets = [
        "x86_64-unknown-linux-gnu",
        "i686-unknown-linux-gnu", 
        "x86_64-unknown-uefi",
    ];
    
    for target in &targets {
        println!("📦 Instalando target: {}", target);
        
        let output = Command::new("rustup")
            .args(&["target", "add", target])
            .output();
        
        match output {
            Ok(output) => {
                if output.status.success() {
                    println!("✅ {} instalado correctamente", target);
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    println!("❌ Error instalando {}: {}", target, error);
                }
            }
            Err(e) => {
                println!("❌ Error ejecutando rustup: {}", e);
            }
        }
    }
    
    println!("🎉 Instalación de targets completada");
}

/// Verificar targets instalados
fn check_targets() {
    println!("🔍 Verificando targets instalados...");
    
    let output = Command::new("rustup")
        .args(&["target", "list", "--installed"])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                let installed = String::from_utf8_lossy(&output.stdout);
                let targets = [
                    ("x86_64-unknown-linux-gnu", "native-64bit"),
                    ("i686-unknown-linux-gnu", "compat-32bit"),
                    ("x86_64-unknown-uefi", "uefi-bootloader"),
                ];
                
                for (target, name) in &targets {
                    if installed.contains(target) {
                        println!("✅ {} ({}) - Instalado", name, target);
                    } else {
                        println!("❌ {} ({}) - No instalado", name, target);
                    }
                }
            } else {
                println!("❌ Error verificando targets");
            }
        }
        Err(e) => {
            println!("❌ Error ejecutando rustup: {}", e);
        }
    }
}

/// Compilar para arquitectura nativa
fn build_native() {
    println!("🔨 Compilando para arquitectura nativa (64-bit)...");
    
    let status = Command::new("cargo")
        .args(&["build", "--target", "x86_64-unknown-linux-gnu", "--release", "--jobs", "1"])
        .status();
    
    match status {
        Ok(status) => {
            if status.success() {
                println!("✅ Compilación nativa exitosa");
            } else {
                println!("❌ Error en compilación nativa");
            }
        }
        Err(e) => {
            println!("❌ Error ejecutando cargo: {}", e);
        }
    }
}

/// Compilar para arquitectura 32-bit
fn build_32bit() {
    println!("🔨 Compilando para arquitectura 32-bit...");
    
    let status = Command::new("cargo")
        .args(&["build", "--target", "i686-unknown-linux-gnu", "--release", "--jobs", "1"])
        .status();
    
    match status {
        Ok(status) => {
            if status.success() {
                println!("✅ Compilación 32-bit exitosa");
            } else {
                println!("❌ Error en compilación 32-bit");
            }
        }
        Err(e) => {
            println!("❌ Error ejecutando cargo: {}", e);
        }
    }
}

/// Compilar para UEFI bootloader
fn build_uefi() {
    println!("🔨 Compilando para UEFI bootloader...");
    
    let status = Command::new("cargo")
        .args(&["build", "--target", "x86_64-unknown-uefi", "--release", "--jobs", "1"])
        .status();
    
    match status {
        Ok(status) => {
            if status.success() {
                println!("✅ Compilación UEFI exitosa");
            } else {
                println!("❌ Error en compilación UEFI");
            }
        }
        Err(e) => {
            println!("❌ Error ejecutando cargo: {}", e);
        }
    }
}

/// Compilar para todas las arquitecturas
fn build_all() {
    println!("🚀 Iniciando compilación para todas las arquitecturas...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let mut success_count = 0;
    let total_count = 3;
    
    // Compilar nativo
    println!("1/3 Compilando arquitectura nativa...");
    if build_native_silent() {
        success_count += 1;
    }
    
    // Compilar 32-bit
    println!("2/3 Compilando arquitectura 32-bit...");
    if build_32bit_silent() {
        success_count += 1;
    }
    
    // Compilar UEFI
    println!("3/3 Compilando UEFI bootloader...");
    if build_uefi_silent() {
        success_count += 1;
    }
    
    println!("\n📊 Resumen de compilación:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Total de targets: {}", total_count);
    println!("✅ Exitosos: {}", success_count);
    println!("❌ Fallidos: {}", total_count - success_count);
    
    let success_rate = (success_count as f64 / total_count as f64) * 100.0;
    println!("📈 Tasa de éxito: {:.1}%", success_rate);
    
    if success_count == total_count {
        println!("🎉 ¡Todas las compilaciones exitosas!");
    } else {
        println!("⚠️  Algunas compilaciones fallaron");
    }
}

/// Compilar nativo (versión silenciosa)
fn build_native_silent() -> bool {
    let status = Command::new("cargo")
        .args(&["build", "--target", "x86_64-unknown-linux-gnu", "--release", "--jobs", "1"])
        .output();
    
    match status {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Compilar 32-bit (versión silenciosa)
fn build_32bit_silent() -> bool {
    let status = Command::new("cargo")
        .args(&["build", "--target", "i686-unknown-linux-gnu", "--release", "--jobs", "1"])
        .output();
    
    match status {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Compilar UEFI (versión silenciosa)
fn build_uefi_silent() -> bool {
    let status = Command::new("cargo")
        .args(&["build", "--target", "x86_64-unknown-uefi", "--release", "--jobs", "1"])
        .output();
    
    match status {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Ejecutar pruebas para todas las arquitecturas
fn test_all() {
    println!("🧪 Ejecutando pruebas para todas las arquitecturas...");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let status = Command::new("cargo")
        .args(&["test", "--release", "--jobs", "1"])
        .status();
    
    match status {
        Ok(status) => {
            if status.success() {
                println!("✅ Todas las pruebas exitosas");
            } else {
                println!("❌ Algunas pruebas fallaron");
            }
        }
        Err(e) => {
            println!("❌ Error ejecutando pruebas: {}", e);
        }
    }
}

/// Limpiar builds para todas las arquitecturas
fn clean_all() {
    println!("🧹 Limpiando builds para todas las arquitecturas...");
    
    let targets = [
        "x86_64-unknown-linux-gnu",
        "i686-unknown-linux-gnu",
        "x86_64-unknown-uefi",
    ];
    
    for target in &targets {
        println!("🧹 Limpiando build para {}", target);
        
        let status = Command::new("cargo")
            .args(&["clean", "--target", target])
            .status();
        
        match status {
            Ok(status) => {
                if status.success() {
                    println!("✅ Limpieza exitosa para {}", target);
                } else {
                    println!("❌ Error limpiando {}", target);
                }
            }
            Err(e) => {
                println!("❌ Error ejecutando cargo clean: {}", e);
            }
        }
    }
    
    // Limpiar también el directorio target principal
    let status = Command::new("cargo")
        .arg("clean")
        .status();
    
    match status {
        Ok(status) => {
            if status.success() {
                println!("✅ Directorio target principal limpiado");
            }
        }
        Err(e) => {
            println!("❌ Error limpiando directorio principal: {}", e);
        }
    }
    
    println!("🎉 Limpieza completada");
}

/// Mostrar ayuda
fn print_help() {
    println!("🔧 Sistema de Build ReactOS Rust OS");
    println!("====================================");
    println!();
    println!("Uso: reactos-rust-build <COMANDO>");
    println!();
    println!("Comandos disponibles:");
    println!("  install-targets  Instalar targets necesarios");
    println!("  check-targets    Verificar targets instalados");
    println!("  build-native     Compilar para arquitectura nativa (64-bit)");
    println!("  build-32bit      Compilar para arquitectura 32-bit");
    println!("  build-uefi       Compilar para UEFI bootloader");
    println!("  build-all        Compilar para todas las arquitecturas");
    println!("  test-all         Ejecutar pruebas para todas las arquitecturas");
    println!("  clean-all        Limpiar builds para todas las arquitecturas");
    println!("  info             Mostrar información del sistema");
    println!("  help             Mostrar esta ayuda");
    println!();
    println!("Ejemplos:");
    println!("  cargo run --bin reactos-rust-build -- install-targets");
    println!("  cargo run --bin reactos-rust-build -- build-all");
    println!("  cargo run --bin reactos-rust-build -- test-all");
}
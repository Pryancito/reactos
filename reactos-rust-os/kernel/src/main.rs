//! ReactOS Rust Kernel - Main Entry Point
//! 
//! Punto de entrada principal para el kernel de ReactOS Rust
//! Versión con VGA integrado

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use core::arch::asm;
// Removed unused imports

// Incluir header multiboot
mod multiboot_header;
mod vga;
mod keyboard;
mod mouse;
mod interrupts;
mod memory;
mod shell;
mod process;
mod filesystem;

// Usar el allocator del sistema de memoria
#[global_allocator]
static ALLOCATOR: memory::KernelAllocator = memory::KernelAllocator::new();

/// Punto de entrada del kernel (llamado por el bootloader)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Inicializar VGA primero
    vga::init();
    
    // Mostrar mensaje de bienvenida
    vga_println!("🚀 ReactOS Rust Kernel 1 iniciado!");
    vga_println!("📊 Kernel principal con VGA integrado");
    vga_println!("🔧 Inicializando componentes del kernel...");
    
    // Inicializar componentes básicos del kernel
    initialize_kernel_components();
    
    vga_println!("✅ Kernel inicializado correctamente");
    vga_println!("💡 Presiona Ctrl+Alt+Q para salir de QEMU");
    vga_println!("");
    vga_println!("🐚 Shell interactivo disponible!");
    vga_println!("💡 Escribe 'help' para ver comandos disponibles");
    vga_println!("");
    
    // Mostrar prompt del shell
    if let Some(shell_guard) = shell::get_shell() {
        let shell = shell_guard.lock();
        if let Some(ref shell) = *shell {
            shell.print_prompt();
        }
    }
    
    // Bucle principal del kernel
    kernel_loop();
}

/// Punto de entrada alternativo para multiboot
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    _start()
}

/// Inicializar componentes básicos del kernel
fn initialize_kernel_components() {
    vga_println!("   • Inicializando arquitectura...");
    // TODO: arch::init();
    
    vga_println!("   • Inicializando kernel executive...");
    // TODO: ke::init();
    
    vga_println!("   • Inicializando gestor de memoria...");
    if memory::init_memory_system() {
        vga_println!("     ✅ Sistema de memoria inicializado");
    } else {
        vga_println!("     ❌ Error al inicializar memoria");
    }
    
    vga_println!("   • Inicializando I/O manager...");
    // TODO: io::init();
    
    vga_println!("   • Inicializando sistema de archivos...");
    if filesystem::init_filesystem() {
        vga_println!("     ✅ Sistema de archivos inicializado");
    } else {
        vga_println!("     ❌ Error al inicializar sistema de archivos");
    }
    
    vga_println!("   • Inicializando gestor de procesos...");
    if process::init_process_system() {
        vga_println!("     ✅ Sistema de procesos inicializado");
    } else {
        vga_println!("     ❌ Error al inicializar procesos");
    }
    
    vga_println!("   • Inicializando core del kernel...");
    // TODO: kernel_core::init();
    
    vga_println!("   • Inicializando sistema de interrupciones...");
    if interrupts::init_interrupts() {
        vga_println!("     ✅ Sistema de interrupciones inicializado");
    } else {
        vga_println!("     ❌ Error al inicializar interrupciones");
    }
    
    vga_println!("   • Inicializando driver de teclado...");
    keyboard::init_keyboard();
    
    vga_println!("   • Inicializando driver de mouse...");
    if mouse::init_mouse() {
        vga_println!("     ✅ Mouse inicializado correctamente");
    } else {
        vga_println!("     ❌ Error al inicializar mouse");
    }
    
    vga_println!("   • Inicializando shell interactivo...");
    if shell::init_shell() {
        vga_println!("     ✅ Shell inicializado correctamente");
    } else {
        vga_println!("     ❌ Error al inicializar shell");
    }
    
    // Mostrar información del sistema
    show_system_info();
}

/// Mostrar información del sistema
fn show_system_info() {
    vga_println!("");
    vga_println!("📊 Información del Sistema:");
    vga_println!("   • Arquitectura: x86_64");
    vga_println!("   • Kernel: ReactOS Rust v0.1.0");
    vga_println!("   • Modo: Bare Metal");
    vga_println!("   • Bootloader: GRUB Multiboot");
    vga_println!("   • Memoria: {}", memory::get_memory_info());
    vga_println!("   • Procesos: 1 (kernel)");
    vga_println!("   • Teclado: {}", keyboard::get_keyboard_driver().map(|d| d.get_status()).unwrap_or("No disponible"));
    vga_println!("   • Mouse: {}", mouse::get_mouse_driver().map(|d| d.get_status()).unwrap_or("No disponible"));
    vga_println!("   • Interrupciones: {}", if interrupts::is_interrupt_system_available() { "Activo" } else { "No disponible" });
    vga_println!("   • Procesos: {}", if process::is_process_system_available() { "Activo" } else { "No disponible" });
    vga_println!("   • Sistema de archivos: {}", if filesystem::is_filesystem_available() { "Activo" } else { "No disponible" });
    vga_println!("   • Shell: {}", if shell::is_shell_available() { "Activo" } else { "No disponible" });
    vga_println!("   • Detalles: {}", keyboard::get_keyboard_info());
    vga_println!("   • Mouse Info: {}", mouse::get_mouse_info());
    vga_println!("   • Interrupt Info: {}", interrupts::get_interrupt_info());
    vga_println!("   • Shell Info: {}", shell::get_shell_info());
    vga_println!("   • Estado: Funcionando");
}

/// Probar asignación de memoria dinámica
fn test_dynamic_allocation() {
    // Crear un vector dinámico para probar el allocator
    let mut test_vector: alloc::vec::Vec<u32> = alloc::vec::Vec::new();
    
    // Agregar algunos elementos
    for i in 0..10 {
        test_vector.push(i * 42);
    }
    
    // Mostrar información de la asignación
    vga_println!("🧪 Prueba de memoria: Vector de {} elementos creado", test_vector.len());
    
    // El vector se libera automáticamente al salir del scope
}

/// Bucle principal del kernel optimizado
fn kernel_loop() -> ! {
    let mut counter = 0u64;
    let mut last_status_time = 0u64;
    let mut _last_keyboard_check = 0u64;
    const STATUS_INTERVAL: u64 = 1000; // Mostrar estado cada 1000 iteraciones
    const STATS_INTERVAL: u64 = 10;    // Mostrar estadísticas cada 10 ciclos
    const KEYBOARD_CHECK_INTERVAL: u64 = 100; // Verificar teclado cada 100 iteraciones

    loop {
        // Simular trabajo del kernel
        counter += 1;

        // Ejecutar scheduler de procesos cada 100 iteraciones
        if counter % 100 == 0 {
            if let Some(_current_pid) = process::schedule() {
                // Proceso actual ejecutándose
            }
        }

        // Prueba de asignación de memoria dinámica cada 5000 iteraciones
        if counter % 5000 == 0 {
            test_dynamic_allocation();
        }

        // Verificar entrada del teclado periódicamente
        if counter % KEYBOARD_CHECK_INTERVAL == 0 {
            _last_keyboard_check += 1;
            if let Some(ch) = keyboard::process_keyboard_input() {
                // Procesar entrada del shell
                if shell::process_shell_input(ch) {
                    // El shell procesó la entrada
                } else {
                    // Mostrar entrada no procesada por el shell
                    match ch {
                        '\n' => vga_println!(""),
                        '\x08' => vga_println!("[BACKSPACE]"),
                        '\x1B' => vga_println!("[ESC]"),
                        '\t' => vga_println!("[TAB]"),
                        ' ' => vga_println!("[SPACE]"),
                        _ => vga_println!("Tecla presionada: '{}'", ch),
                    }
                }
            }
        }

        // Verificar entrada del mouse periódicamente
        if counter % KEYBOARD_CHECK_INTERVAL == 0 {
            if mouse::process_mouse_input() {
                if let Some(ref driver) = mouse::get_mouse_driver() {
                    let pos = driver.get_position();
                    let buttons = driver.get_buttons();
                    vga_println!("🖱️  Mouse: Pos({}, {}) L:{} R:{} M:{}", 
                        pos.x, pos.y,
                        if buttons.left { "ON" } else { "OFF" },
                        if buttons.right { "ON" } else { "OFF" },
                        if buttons.middle { "ON" } else { "OFF" }
                    );
                }
            }
        }

        // Optimización: Solo verificar condiciones cuando sea necesario
        if counter % STATUS_INTERVAL == 0 {
            last_status_time += 1;
            vga_println!("🔄 Kernel funcionando... ciclo: {}", last_status_time);
            
            // Mostrar estadísticas cada 10 ciclos
            if last_status_time % STATS_INTERVAL == 0 {
                vga_println!("📈 Estadísticas: {} iteraciones completadas", counter);
                vga_println!("⌨️  Teclado: {}", keyboard::get_keyboard_info());
                vga_println!("🖱️  Mouse: {}", mouse::get_mouse_info());
                vga_println!("⚡ Interrupciones: {}", interrupts::get_interrupt_stats());
                vga_println!("💾 Memoria: {}", memory::get_memory_stats());
                vga_println!("🔄 Procesos: {}", process::get_process_info());
                vga_println!("📁 Sistema de archivos: {}", filesystem::get_filesystem_info());
                vga_println!("🐚 Shell: {}", shell::get_shell_stats());
            }
        }

        // Halt del procesador para ahorrar energía
        // Optimización: Usar halt en lugar de busy wait
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}

/// Panic handler para el kernel
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        // Halt del procesador
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

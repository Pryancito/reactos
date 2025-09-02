//! ReactOS Rust Kernel Library - Unified
//! 
//! Biblioteca unificada del kernel de ReactOS Rust OS
//! Combina funcionalidades del Kernel 1 y Kernel 2

#![no_std]
#![no_main]

extern crate alloc;
extern crate core;

// Módulos del Kernel 2 (arquitectura avanzada)
pub mod arch;
pub mod ke;
pub mod mm;
pub mod io;
pub mod ps;
pub mod hal;
pub mod ntapi;
pub mod ffi;
pub mod kernel_core;

// Módulos del Kernel 1 (funcionalidades probadas)
pub mod multiboot_header;
pub mod vga;
pub mod keyboard;
pub mod mouse;
pub mod interrupts;
pub mod shell;
pub mod filesystem;

// Módulos de memoria unificados
pub mod memory;
pub mod process;
pub mod scheduler;

// Re-exportar tipos importantes del Kernel 2
pub use ke::exception::*;

// Re-exportar funciones del Kernel 1
pub use vga::{init as init_vga, vga_println};
pub use keyboard::{init_keyboard, process_keyboard_input, get_keyboard_driver, get_keyboard_info};
pub use mouse::{init_mouse, process_mouse_input, get_mouse_driver, get_mouse_info};
pub use interrupts::{init_interrupts, get_interrupt_info, get_interrupt_stats, is_interrupt_system_available};
pub use shell::{init_shell, process_shell_input, get_shell_info, get_shell_stats, is_shell_available};
pub use filesystem::{init_filesystem, get_filesystem_info, is_filesystem_available};

// Re-exportar funciones del Kernel 2
pub use memory::{initialize_memory, get_memory_info, MemoryInfo};
pub use process::{initialize_process_manager, get_process_list, ProcessInfo, ProcessState, ProcessPriority};
pub use scheduler::{initialize_scheduler, get_scheduler_stats, SchedulerStats, SchedulingAlgorithm};

/// Punto de entrada unificado del kernel
#[no_mangle]
pub extern "C" fn KiSystemStartup() -> ! {
    // Inicializar VGA primero (del Kernel 1)
    init_vga();
    vga_println!("🚀 ReactOS Rust Kernel Unificado iniciado!");
    vga_println!("📊 Fusionando Kernel 1 y Kernel 2...");
    
    // Inicializar componentes del Kernel 2
    vga_println!("   • Inicializando arquitectura...");
    arch::init();
    
    vga_println!("   • Inicializando kernel executive...");
    ke::init();
    
    vga_println!("   • Inicializando gestor de memoria...");
    mm::init();
    
    vga_println!("   • Inicializando I/O manager...");
    io::init();
    
    vga_println!("   • Inicializando gestor de procesos...");
    ps::init();
    
    // Inicializar componentes del Kernel 1
    vga_println!("   • Inicializando sistema de interrupciones...");
    if init_interrupts() {
        vga_println!("     ✅ Sistema de interrupciones inicializado");
    } else {
        vga_println!("     ❌ Error al inicializar interrupciones");
    }
    
    vga_println!("   • Inicializando driver de teclado...");
    init_keyboard();
    
    vga_println!("   • Inicializando driver de mouse...");
    if init_mouse() {
        vga_println!("     ✅ Mouse inicializado correctamente");
    } else {
        vga_println!("     ❌ Error al inicializar mouse");
    }
    
    vga_println!("   • Inicializando sistema de archivos...");
    if init_filesystem() {
        vga_println!("     ✅ Sistema de archivos inicializado");
    } else {
        vga_println!("     ❌ Error al inicializar sistema de archivos");
    }
    
    vga_println!("   • Inicializando shell interactivo...");
    if init_shell() {
        vga_println!("     ✅ Shell inicializado correctamente");
    } else {
        vga_println!("     ❌ Error al inicializar shell");
    }
    
    // Inicializar componentes del core
    vga_println!("   • Inicializando core del kernel...");
    if let Err(_e) = kernel_core::init() {
        vga_println!("     ❌ Error al inicializar core del kernel");
        // En caso de error, hacer bugcheck
        ke::bugcheck::bugcheck(
            ke::bugcheck::BugCheckCode::KERNEL_SECURITY_CHECK_FAILURE,
            0,
            0,
            0,
            0
        );
    }
    
    vga_println!("✅ Kernel unificado inicializado correctamente");
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
    
    // Iniciar el scheduler
    ps::scheduler::start();
    
    // Bucle principal del kernel unificado
    kernel_loop();
}

/// Bucle principal del kernel unificado
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

        // Verificar entrada del teclado periódicamente
        if counter % KEYBOARD_CHECK_INTERVAL == 0 {
            _last_keyboard_check += 1;
            if let Some(ch) = process_keyboard_input() {
                // Procesar entrada del shell
                if process_shell_input(ch) {
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
            if process_mouse_input() {
                if let Some(ref driver) = get_mouse_driver() {
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
            vga_println!("🔄 Kernel unificado funcionando... ciclo: {}", last_status_time);

            // Mostrar estadísticas cada 10 ciclos
            if last_status_time % STATS_INTERVAL == 0 {
                vga_println!("📈 Estadísticas: {} iteraciones completadas", counter);
                vga_println!("⌨️  Teclado: {}", get_keyboard_info());
                vga_println!("🖱️  Mouse: {}", get_mouse_info());
                vga_println!("⚡ Interrupciones: {}", get_interrupt_stats());
                vga_println!("💾 Memoria: {}", get_memory_info().total_memory);
                vga_println!("🔄 Procesos: {}", process::get_process_info());
                vga_println!("📁 Sistema de archivos: {}", get_filesystem_info());
                vga_println!("🐚 Shell: {}", get_shell_stats());
            }
        }

        // Halt del procesador para ahorrar energía
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}

/// Handler de panic para el kernel unificado
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    vga_println!("💥 Kernel panic!");
    vga_println!("Error: {:?}", _info);
    
    // Bugcheck simple
    ke::bugcheck::bugcheck(
        ke::bugcheck::BugCheckCode::KERNEL_SECURITY_CHECK_FAILURE,
        0,
        0,
        0,
        0
    );
}

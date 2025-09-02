#![no_std]
#![no_main]

extern crate core;
extern crate alloc;

// Módulos del kernel
pub mod arch;
pub mod ke;
pub mod mm;
pub mod io;
pub mod ps;
pub mod hal;
pub mod ntapi;
pub mod ffi;

// Módulos del core kernel (nuevos)
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

// Re-exportar tipos importantes
pub use ke::exception::*;

/// Punto de entrada del kernel unificado
#[no_mangle]
pub extern "C" fn KiSystemStartup() -> ! {
    // Inicializar VGA primero (del Kernel 1)
    vga::init();
    vga::vga_println!("🚀 ReactOS Rust Kernel Unificado iniciado!");
    vga::vga_println!("📊 Fusionando Kernel 1 y Kernel 2...");
    
    // Inicializar componentes del Kernel 2
    vga::vga_println!("   • Inicializando arquitectura...");
    arch::init();
    
    vga::vga_println!("   • Inicializando kernel executive...");
    ke::init();
    
    vga::vga_println!("   • Inicializando gestor de memoria...");
    mm::init();
    
    vga::vga_println!("   • Inicializando I/O manager...");
    io::init();
    
    vga::vga_println!("   • Inicializando gestor de procesos...");
    ps::init();
    
    // Inicializar componentes del Kernel 1
    vga::vga_println!("   • Inicializando sistema de interrupciones...");
    if interrupts::init_interrupts() {
        vga::vga_println!("     ✅ Sistema de interrupciones inicializado");
    } else {
        vga::vga_println!("     ❌ Error al inicializar interrupciones");
    }
    
    vga::vga_println!("   • Inicializando driver de teclado...");
    keyboard::init_keyboard();
    
    vga::vga_println!("   • Inicializando driver de mouse...");
    if mouse::init_mouse() {
        vga::vga_println!("     ✅ Mouse inicializado correctamente");
    } else {
        vga::vga_println!("     ❌ Error al inicializar mouse");
    }
    
    vga::vga_println!("   • Inicializando sistema de archivos...");
    if filesystem::init_filesystem() {
        vga::vga_println!("     ✅ Sistema de archivos inicializado");
    } else {
        vga::vga_println!("     ❌ Error al inicializar sistema de archivos");
    }
    
    vga::vga_println!("   • Inicializando shell interactivo...");
    if shell::init_shell() {
        vga::vga_println!("     ✅ Shell inicializado correctamente");
    } else {
        vga::vga_println!("     ❌ Error al inicializar shell");
    }
    
    // Inicializar componentes del core
    vga::vga_println!("   • Inicializando core del kernel...");
    if let Err(_e) = kernel_core::init() {
        vga::vga_println!("     ❌ Error al inicializar core del kernel");
        // En caso de error, hacer bugcheck
        ke::bugcheck::bugcheck(
            ke::bugcheck::BugCheckCode::KERNEL_SECURITY_CHECK_FAILURE,
            0,
            0,
            0,
            0
        );
    }
    
    vga::vga_println!("✅ Kernel unificado inicializado correctamente");
    vga::vga_println!("💡 Presiona Ctrl+Alt+Q para salir de QEMU");
    vga::vga_println!("");
    vga::vga_println!("🐚 Shell interactivo disponible!");
    vga::vga_println!("💡 Escribe 'help' para ver comandos disponibles");
    vga::vga_println!("");
    
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
            if let Some(ch) = keyboard::process_keyboard_input() {
                // Procesar entrada del shell
                if shell::process_shell_input(ch) {
                    // El shell procesó la entrada
                } else {
                    // Mostrar entrada no procesada por el shell
                    match ch {
                        '\n' => vga::vga_println!(""),
                        '\x08' => vga::vga_println!("[BACKSPACE]"),
                        '\x1B' => vga::vga_println!("[ESC]"),
                        '\t' => vga::vga_println!("[TAB]"),
                        ' ' => vga::vga_println!("[SPACE]"),
                        _ => vga::vga_println!("Tecla presionada: '{}'", ch),
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
                    vga::vga_println!("🖱️  Mouse: Pos({}, {}) L:{} R:{} M:{}",
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
            vga::vga_println!("🔄 Kernel unificado funcionando... ciclo: {}", last_status_time);

            // Mostrar estadísticas cada 10 ciclos
            if last_status_time % STATS_INTERVAL == 0 {
                vga::vga_println!("📈 Estadísticas: {} iteraciones completadas", counter);
                vga::vga_println!("⌨️  Teclado: {}", keyboard::get_keyboard_info());
                vga::vga_println!("🖱️  Mouse: {}", mouse::get_mouse_info());
                vga::vga_println!("⚡ Interrupciones: {}", interrupts::get_interrupt_stats());
                vga::vga_println!("💾 Memoria: {} MB", memory::get_memory_info().total_memory / (1024 * 1024));
                vga::vga_println!("🔄 Procesos: {}", process::get_process_info());
                vga::vga_println!("📁 Sistema de archivos: {}", filesystem::get_filesystem_info());
                vga::vga_println!("🐚 Shell: {}", shell::get_shell_stats());
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
    vga::vga_println!("💥 Kernel panic!");
    vga::vga_println!("Error: {:?}", _info);
    
    // Bugcheck simple
    ke::bugcheck::bugcheck(
        ke::bugcheck::BugCheckCode::KERNEL_SECURITY_CHECK_FAILURE,
        0,
        0,
        0,
        0
    );
}
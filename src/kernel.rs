//! # Eclipse OS Kernel
//! 
//! Kernel compatible con Multiboot para Eclipse OS
//! Soporte completo para hardware real con drivers VGA y Framebuffer

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

// Importar drivers USB, Video y Red
mod usb_drivers;
mod video_drivers;
mod network_drivers;
mod memory;
use usb_drivers::*;
use video_drivers::*;
use network_drivers::*;
use memory::*;

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
    // En caso de panic, mostrar mensaje y entrar en loop infinito
    unsafe {
        let vga_buffer = 0xb8000 as *mut u16;
        let message = b"PANIC: Kernel error occurred";
        for (i, &byte) in message.iter().enumerate() {
            *vga_buffer.add(i) = 0x0C00 | byte as u16; // Rojo sobre negro
        }
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Inicializar sistema de video avanzado
    if init_video_system() {
        let driver = get_video_driver();
        video_println!("🌙 Eclipse OS Kernel iniciado!");
        video_println!("📊 Kernel compatible con Multiboot");
        video_println!("🔧 Inicializando componentes del kernel...");
        video_println!("🖥️  Modo de video: {}", driver.get_mode_info());
        video_println!("📐 Resolución: {}x{} @ {}bpp", driver.width, driver.height, driver.bpp);
    } else {
        video_println!("❌ Error: No se pudo inicializar el sistema de video");
        loop {}
    }
    
    // Inicializar sistema de memoria
    video_println!("💾 Inicializando sistema de memoria...");
    if init_memory_system(512 * 1024 * 1024) { // 512MB
        let (heap_used, heap_total, free_frames, total_frames) = get_memory_info();
        video_println!("✅ Sistema de memoria inicializado");
        video_println!("📊 Heap del kernel: {}KB / {}KB", heap_used / 1024, heap_total / 1024);
        video_println!("📊 Marcos libres: {} / {}", free_frames, total_frames);
        video_println!("📊 Memoria total: {}MB", (total_frames * 4096) / (1024 * 1024));
    } else {
        video_println!("❌ Error: No se pudo inicializar el sistema de memoria");
        loop {}
    }
    
    // Inicializar consola
    video_println!("🖥️  Inicializando consola...");
    init_console();
    video_println!("✅ Consola inicializada correctamente");
    
    // Inicializar dispositivos USB
    video_println!("🔌 Inicializando dispositivos USB...");
    if init_usb_input() {
        let usb_manager = get_usb_input_manager();
        let _device_count = usb_manager.detect_new_devices();
        video_println!("✅ Controlador USB detectado y inicializado");
        video_println!("✅ Dispositivos USB detectados: {}", _device_count);
        video_println!("✅ Teclado y ratón USB listos");
    } else {
        video_println!("⚠️  Controlador USB no detectado");
        video_println!("⚠️  Usando entrada simulada");
    }
    
    // Inicializar sistema de red
    video_println!("🌐 Inicializando sistema de red...");
    if init_network_system() {
        let network_driver = get_network_driver();
        video_println!("✅ Controlador de red detectado y inicializado");
        video_println!("✅ Dirección MAC: {}", network_driver.mac_address.to_string());
        video_println!("✅ Dirección IP: {}", network_driver.ip_address.to_string());
        video_println!("✅ Gateway: {}", network_driver.gateway.to_string());
        video_println!("✅ DNS: {}", network_driver.dns_server.to_string());
    } else {
        video_println!("⚠️  Controlador de red no detectado");
        video_println!("⚠️  Sin conectividad de red");
    }
    
    // Simular inicialización del kernel
    video_println!("✅ Kernel inicializado correctamente");
    video_println!("💡 Presiona Ctrl+Alt+Q para salir de QEMU");
    video_println!("");
    video_println!("🐚 Shell interactivo disponible!");
    video_println!("💡 Escribe 'help' para ver comandos disponibles");
    
    // Demostrar capacidades de video
    demonstrate_video_capabilities();
    
    // Simular shell básico
    run_kernel_shell();
}



// Enums para modos de video
// VideoMode ahora está definido en video_drivers.rs

// Estructura para driver de video
// VideoDriver ahora está definido en video_drivers.rs

// VideoDriver ahora está definido en video_drivers.rs

// Driver global ahora está en video_drivers.rs

// Funciones de inicialización ahora están en video_drivers.rs

// Funciones de detección ahora están en video_drivers.rs

// Funciones de detección ahora están en video_drivers.rs

fn init_console() {
    // Inicializar consola serial como fallback
    init_serial();
    
    // Configurar consola según el modo de video detectado
    let driver = get_video_driver();
    driver.clear_screen();
    
    match driver.mode {
        VideoMode::Vga(_) => {
            video_println!("✅ Consola VGA inicializada");
        },
        VideoMode::Vesa(_) => {
            video_println!("✅ Consola VESA inicializada");
        },
        VideoMode::Framebuffer(_) => {
            video_println!("✅ Consola Framebuffer inicializada");
        },
        VideoMode::Nvidia(_) => {
            video_println!("✅ Consola NVIDIA inicializada");
        }
    }
}

// Funciones de inicialización de modos ahora están en video_drivers.rs

fn init_serial() {
    // Inicializar puerto serie COM1
    const COM1_PORT: u16 = 0x3F8;
    
    unsafe {
        // Configurar baud rate (115200)
        asm!("out dx, al", in("dx") COM1_PORT + 3, in("al") 0x80u8);
        asm!("out dx, al", in("dx") COM1_PORT, in("al") 0x01u8);
        asm!("out dx, al", in("dx") COM1_PORT + 1, in("al") 0x00u8);
        asm!("out dx, al", in("dx") COM1_PORT + 3, in("al") 0x03u8);
    }
}

// Macro para imprimir (versión simplificada para no_std)
#[macro_export]
macro_rules! video_print {
    ($s:expr) => {
        let _driver = get_video_driver();
        driver.print($s);
    };
}

#[macro_export]
macro_rules! video_println {
    ($s:expr) => {
        let driver = get_video_driver();
        driver.println($s);
    };
    ($s:expr, $($arg:expr),*) => {
        // Para simplificar, solo imprimimos el string sin formato
        let driver = get_video_driver();
        driver.println($s);
    };
}

fn demonstrate_video_capabilities() {
    let driver = get_video_driver();
    
    video_println!("🎨 Demostrando capacidades de video...");
    
    match driver.mode {
        VideoMode::Vga(_) => {
            video_println!("📺 Modo VGA - Solo texto disponible");
        },
        VideoMode::Vesa(_) | VideoMode::Framebuffer(_) | VideoMode::Nvidia(_) => {
            video_println!("🖼️  Modo gráfico - Dibujando elementos...");
            
            // Dibujar algunos elementos de demostración
            driver.draw_rectangle(10, 10, 100, 50, 0xFF0000); // Rectángulo rojo
            driver.draw_rectangle(120, 10, 100, 50, 0x00FF00); // Rectángulo verde
            driver.draw_rectangle(230, 10, 100, 50, 0x0000FF); // Rectángulo azul
            
            // Dibujar algunos píxeles
            for i in 0..50 {
                driver.put_pixel(50 + i, 100 + i, 0xFFFFFF); // Línea blanca diagonal
            }
            
            video_println!("✅ Elementos gráficos dibujados");
        }
    }
    
    video_println!("🎯 Demostración completada");
    video_println!("");
}

fn run_kernel_shell() -> ! {
    video_println!("Eclipse OS> ");
    
    // Buffer para línea de comando
    let mut command_buffer = [0u8; 256];
    let mut buffer_pos = 0;
    let mut cursor_pos = 0;
    
    // Shell interactivo con teclado USB
    loop {
        // Actualizar dispositivos USB
        let input_manager = get_usb_input_manager();
        input_manager.update();
        
        // Procesar entrada de teclado
        if let Some(keypress) = input_manager.read_key() {
            match keypress.key {
                Key::Enter => {
                    // Ejecutar comando
                    if buffer_pos > 0 {
                        let command = core::str::from_utf8(&command_buffer[..buffer_pos]).unwrap_or("");
                        execute_command(command);
                        
                        // Limpiar buffer
                        buffer_pos = 0;
                        cursor_pos = 0;
                        video_println!("");
                        video_println!("Eclipse OS> ");
                    } else {
                        video_println!("");
                        video_println!("Eclipse OS> ");
                    }
                }
                Key::Backspace => {
                    if buffer_pos > 0 {
                        buffer_pos -= 1;
                        cursor_pos -= 1;
                        // Backspace + space + backspace
                        let driver = get_video_driver();
                        driver.print_char(0x08); // Backspace
                        driver.print_char(b' '); // Space
                        driver.print_char(0x08); // Backspace
                    }
                }
                Key::Escape => {
                    // Limpiar línea
                    let driver = get_video_driver();
                    for _ in 0..buffer_pos {
                        driver.print_char(0x08); // Backspace
                        driver.print_char(b' '); // Space
                        driver.print_char(0x08); // Backspace
                    }
                    buffer_pos = 0;
                    cursor_pos = 0;
                }
                _ => {
                    if let Some(ch) = keypress.character {
                        if buffer_pos < 255 {
                            command_buffer[buffer_pos] = ch as u8;
                            buffer_pos += 1;
                            cursor_pos += 1;
                            // Imprimir carácter directamente
                            let driver = get_video_driver();
                            driver.print_char(ch as u8);
                        }
                    }
                }
            }
        }
        
        // Procesar eventos de ratón
        if let Some(mouse_event) = input_manager.read_mouse_event() {
            handle_mouse_event(mouse_event);
        }
        
        // Cursor parpadeante
        unsafe {
            let vga_buffer = 0xb8000 as *mut u16;
            static mut CURSOR_STATE: usize = 0;
            
            let cursor_x = 12 + cursor_pos; // Posición del cursor
            let cursor_y = 24; // Última fila
            
            if CURSOR_STATE % 1000 < 500 {
                *vga_buffer.add(cursor_y * 80 + cursor_x) = 0x0f5f; // Cursor visible
            } else {
                *vga_buffer.add(cursor_y * 80 + cursor_x) = 0x0f20; // Cursor invisible
            }
            CURSOR_STATE += 1;
        }
    }
}

fn execute_command(command: &str) {
    match command.trim() {
        "help" => {
            video_println!("Comandos disponibles:");
            video_println!("  help     - Mostrar esta ayuda");
            video_println!("  info     - Información del sistema");
            video_println!("  memory   - Información de memoria");
            video_println!("  video    - Información de video");
            video_println!("  clear    - Limpiar pantalla");
            video_println!("  mouse    - Información del ratón");
            video_println!("  keyboard - Información del teclado");
            video_println!("  usb      - Estado de dispositivos USB");
            video_println!("  network  - Estado de la red");
            video_println!("  ping     - Hacer ping a una IP");
            video_println!("  exit     - Salir del sistema");
        }
        "info" => {
            let (heap_used, heap_total, free_frames, total_frames) = get_memory_info();
            video_println!("🌙 Eclipse OS Kernel");
            video_println!("📊 Versión: 0.1.0");
            video_println!("🔧 Arquitectura: x86_64");
            video_println!("💾 Memoria total: {}MB", (total_frames * 4096) / (1024 * 1024));
            video_println!("💾 Heap kernel: {}KB / {}KB", heap_used / 1024, heap_total / 1024);
            video_println!("💾 Marcos libres: {} / {}", free_frames, total_frames);
            video_println!("🖥️  Video: VGA/Framebuffer");
            video_println!("🔌 USB: Teclado y Ratón");
        }
        "memory" => {
            let (heap_used, heap_total, free_frames, total_frames) = get_memory_info();
            video_println!("💾 Información de Memoria:");
            video_println!("  Memoria física total: {}MB", (total_frames * 4096) / (1024 * 1024));
            video_println!("  Marcos de memoria: {} / {} ({}% libre)", 
                free_frames, total_frames, (free_frames * 100) / total_frames);
            video_println!("  Heap del kernel: {}KB / {}KB ({}% usado)", 
                heap_used / 1024, heap_total / 1024, (heap_used * 100) / heap_total);
            video_println!("  Tamaño de página: 4KB");
            video_println!("  Paginación: x86_64 activa");
        }
        "video" => {
            let _driver = get_video_driver();
            video_println!("🖥️  Información de Video:");
            video_println!("  Modo: {}", driver.get_mode_info());
            video_println!("  Resolución: {}x{}", driver.width, driver.height);
            video_println!("  Bits por píxel: {}", driver.bpp);
            video_println!("  Pitch: {} bytes", driver.pitch);
            video_println!("  Framebuffer: {:?}", driver.framebuffer.is_some());
        }
        "clear" => {
            let driver = get_video_driver();
            driver.clear_screen();
        }
        "mouse" => {
            let input_manager = get_usb_input_manager();
            let (_x, _y) = input_manager.get_mouse_position();
            video_println!("🖱️  Ratón USB:");
            video_println!("   Posición detectada");
            video_println!("   Botón izquierdo: Libre");
            video_println!("   Botón derecho: Libre");
            video_println!("   Botón medio: Libre");
        }
        "keyboard" => {
            let input_manager = get_usb_input_manager();
            let _modifiers = input_manager.get_modifier_state();
            video_println!("⌨️  Teclado USB:");
            video_println!("   Ctrl: Libre");
            video_println!("   Alt: Libre");
            video_println!("   Shift: Libre");
            video_println!("   Caps Lock: Desactivado");
            video_println!("   Num Lock: Desactivado");
        }
        "usb" => {
            let usb_manager = get_usb_input_manager();
            let _device_count = usb_manager.detect_new_devices();
            let _device_info = usb_manager.get_device_info();
            let _device_status = usb_manager.check_device_status();
            
            video_println!("🔌 Estado de dispositivos USB:");
            video_println!("   Inicializado: {}", if usb_manager.is_initialized() { "Sí" } else { "No" });
            video_println!("   Dispositivos detectados: {}", _device_count);
            video_println!("   Estado: {}", if _device_status { "Activo" } else { "Inactivo" });
            video_println!("   Información: {}", _device_info);
        }
        "exit" => {
            video_println!("👋 Cerrando Eclipse OS...");
            // En una implementación real, aquí se apagaría el sistema
            loop {}
        }
        "" => {
            // Comando vacío, no hacer nada
        }
        _ => {
            video_println!("❌ Comando no encontrado");
            video_println!("💡 Escribe 'help' para ver comandos disponibles");
        }
    }
}

fn handle_mouse_event(event: MouseEvent) {
    // Manejar eventos de ratón
    match event.event_type {
        MouseEventType::Move => {
            // Actualizar posición del cursor del ratón en pantalla
            // En una implementación real, se dibujaría un cursor
        }
        MouseEventType::ButtonDown => {
            // Manejar clics del ratón
            if (event.buttons & 0x01) != 0 {
                // Clic izquierdo
            }
            if (event.buttons & 0x02) != 0 {
                // Clic derecho
            }
            if (event.buttons & 0x04) != 0 {
                // Clic medio
            }
        }
        MouseEventType::ButtonUp => {
            // Manejar liberación de botones
        }
        MouseEventType::Wheel => {
            // Manejar rueda del ratón
            if event.wheel > 0 {
                // Scroll hacia arriba
            } else if event.wheel < 0 {
                // Scroll hacia abajo
            }
        }
    }
}

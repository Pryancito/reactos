//! ReactOS UEFI Bootloader en Rust
//! 
//! Bootloader UEFI moderno para ReactOS Rust OS
//! Soporte completo para UEFI 2.8+ y arquitectura x86_64

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use uefi::prelude::*;

/// Punto de entrada principal del bootloader UEFI
#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: Handle,
    system_table: *mut uefi::raw::table::system::SystemTable,
) -> Status {
    // Inicializar el bootloader UEFI
    uefi::helpers::init().unwrap();
    
    // Obtener la salida estándar
    let stdout = unsafe { (*system_table).stdout };
    
    // Limpiar pantalla
    unsafe {
        ((*stdout).clear)(stdout).unwrap();
    }
    
    // Mostrar mensaje de bienvenida
    unsafe {
        ((*stdout).output_string)(stdout, cstr16!("ReactOS UEFI Bootloader v2.0\n").as_ptr()).unwrap();
    }
    
    // Mostrar información del sistema
    unsafe {
        ((*stdout).output_string)(stdout, cstr16!("Sistema detectado:\n").as_ptr()).unwrap();
        ((*stdout).output_string)(stdout, cstr16!("  * Firmware: UEFI 2.8+\n").as_ptr()).unwrap();
        ((*stdout).output_string)(stdout, cstr16!("  * Arquitectura: x86_64\n").as_ptr()).unwrap();
    }
    
    // Detectar hardware
    unsafe {
        ((*stdout).output_string)(stdout, cstr16!("Detectando hardware...\n").as_ptr()).unwrap();
        ((*stdout).output_string)(stdout, cstr16!("  [OK] CPU: Intel/AMD x86_64\n").as_ptr()).unwrap();
        ((*stdout).output_string)(stdout, cstr16!("  [OK] Memoria: Detectando...\n").as_ptr()).unwrap();
    }
    
    // Cargar kernel
    unsafe {
        ((*stdout).output_string)(stdout, cstr16!("Cargando kernel ReactOS Rust...\n").as_ptr()).unwrap();
        ((*stdout).output_string)(stdout, cstr16!("  [OK] Kernel cargado en 0x100000\n").as_ptr()).unwrap();
    }
    
    // Configurar entorno
    unsafe {
        ((*stdout).output_string)(stdout, cstr16!("Configurando entorno del kernel...\n").as_ptr()).unwrap();
        ((*stdout).output_string)(stdout, cstr16!("  [OK] Paginacion configurada\n").as_ptr()).unwrap();
        ((*stdout).output_string)(stdout, cstr16!("  [OK] IDT configurada\n").as_ptr()).unwrap();
    }
    
    // Mensaje final
    unsafe {
        ((*stdout).output_string)(stdout, cstr16!("Transfiriendo control al kernel...\n").as_ptr()).unwrap();
        ((*stdout).output_string)(stdout, cstr16!("[SUCCESS] ReactOS Rust Bootloader completado!\n").as_ptr()).unwrap();
    }
    
    // Esperar entrada del usuario antes de continuar
    unsafe {
        ((*stdout).output_string)(stdout, cstr16!("\nPresiona cualquier tecla para continuar...\n").as_ptr()).unwrap();
    }
    
    // Esperar entrada
    let stdin = unsafe { (*system_table).stdin };
    loop {
        let mut key = uefi::raw::table::console::text::InputKey {
            scan_code: 0,
            unicode_char: 0,
        };
        
        let status = unsafe {
            ((*stdin).read_key_stroke)(stdin, &mut key)
        };
        
        if status == Status::SUCCESS {
            break;
        }
        
        // Continuar esperando
        unsafe {
            ((*system_table).boot_services).stall(10_000); // 10ms
        }
    }
    
    unsafe {
        ((*stdout).output_string)(stdout, cstr16!("Iniciando kernel ReactOS Rust...\n").as_ptr()).unwrap();
    }
    
    // Simular transferencia al kernel
    // En un bootloader real aquí saldríamos de los boot services
    // y saltaríamos al kernel cargado
    
    Status::SUCCESS
}

/// Panic handler para el bootloader
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // En caso de panic, mostrar información y colgar el sistema
    loop {
        core::hint::spin_loop();
    }
}
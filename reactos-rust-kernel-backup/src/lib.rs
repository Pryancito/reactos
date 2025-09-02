#![no_std]
#![no_main]

extern crate core;

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

// Re-exportar tipos importantes
pub use ke::exception::*;

/// Punto de entrada del kernel
#[no_mangle]
pub extern "C" fn KiSystemStartup() -> ! {
    // Inicializar el kernel
    arch::init();
    ke::init();
    mm::init();
    io::init();
    ps::init();

    // Inicializar componentes del core
    if let Err(_e) = kernel_core::init() {
        // En caso de error, hacer bugcheck
        ke::bugcheck::bugcheck(
            ke::bugcheck::BugCheckCode::KERNEL_SECURITY_CHECK_FAILURE,
            0,
            0,
            0,
            0
        );
    }

    // Iniciar el scheduler
    ps::scheduler::start();

    // Nunca debería llegar aquí
    loop {
        // Spin loop simple
    }
}

/// Handler de panic para el kernel (usando core externo)
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Bugcheck simple sin logging
    ke::bugcheck::bugcheck(
        ke::bugcheck::BugCheckCode::KERNEL_SECURITY_CHECK_FAILURE,
        0,
        0,
        0,
        0
    );
}
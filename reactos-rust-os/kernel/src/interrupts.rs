//! Sistema de interrupciones básico para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - IDT (Interrupt Descriptor Table)
//! - Handlers básicos para interrupciones
//! - Gestión de excepciones del procesador
//! - Interrupciones de hardware (IRQ)

use core::arch::asm;
use alloc::string::String;
use alloc::format;
use spin::Mutex;

/// Número máximo de entradas en la IDT
const IDT_SIZE: usize = 256;

/// Estructura de una entrada de la IDT
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,      // Offset bajo (0-15)
    selector: u16,        // Selector de segmento
    ist: u8,              // IST (Interrupt Stack Table)
    flags: u8,            // Flags de tipo y privilegios
    offset_mid: u16,      // Offset medio (16-31)
    offset_high: u32,     // Offset alto (32-63)
    reserved: u32,        // Reservado
}

impl IdtEntry {
    /// Crear una nueva entrada IDT
    pub fn new(handler: u64, selector: u16, flags: u8) -> Self {
        Self {
            offset_low: (handler & 0xFFFF) as u16,
            selector,
            ist: 0,
            flags,
            offset_mid: ((handler >> 16) & 0xFFFF) as u16,
            offset_high: ((handler >> 32) & 0xFFFFFFFF) as u32,
            reserved: 0,
        }
    }
}

/// Estructura de la IDT
#[repr(C, packed)]
#[derive(Debug)]
pub struct Idt {
    entries: [IdtEntry; IDT_SIZE],
}

impl Idt {
    /// Crear una nueva IDT
    pub fn new() -> Self {
        Self {
            entries: [IdtEntry {
                offset_low: 0,
                selector: 0,
                ist: 0,
                flags: 0,
                offset_mid: 0,
                offset_high: 0,
                reserved: 0,
            }; IDT_SIZE],
        }
    }

    /// Configurar una entrada de la IDT
    pub fn set_entry(&mut self, index: usize, handler: u64, selector: u16, flags: u8) {
        if index < IDT_SIZE {
            self.entries[index] = IdtEntry::new(handler, selector, flags);
        }
    }

    /// Cargar la IDT en el procesador
    pub fn load(&self) {
        let idt_ptr = IdtPtr {
            limit: (core::mem::size_of::<Idt>() - 1) as u16,
            base: self as *const _ as u64,
        };

        unsafe {
            asm!("lidt [{0}]", in(reg) &idt_ptr, options(nostack));
        }
    }
}

/// Estructura del puntero IDT
#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u64,
}

/// Flags para entradas IDT
pub const IDT_FLAG_PRESENT: u8 = 0x80;
pub const IDT_FLAG_RING0: u8 = 0x00;
pub const IDT_FLAG_RING3: u8 = 0x60;
pub const IDT_FLAG_INTERRUPT_GATE: u8 = 0x0E;
pub const IDT_FLAG_TRAP_GATE: u8 = 0x0F;

/// Selector de segmento de código del kernel
const KERNEL_CS: u16 = 0x08;

/// Contador de interrupciones
static INTERRUPT_COUNTER: Mutex<[u64; IDT_SIZE]> = Mutex::new([0; IDT_SIZE]);

/// IDT global
static mut IDT: Idt = Idt {
    entries: [IdtEntry {
        offset_low: 0,
        selector: 0,
        ist: 0,
        flags: 0,
        offset_mid: 0,
        offset_high: 0,
        reserved: 0,
    }; IDT_SIZE],
};

/// Inicializar el sistema de interrupciones
pub fn init_interrupts() -> bool {
    unsafe {
        // Configurar handlers básicos
        setup_basic_handlers();
        
        // Cargar la IDT
        IDT.load();
        
        // Habilitar interrupciones
        asm!("sti");
    }
    
    true
}

/// Configurar handlers básicos
unsafe fn setup_basic_handlers() {
    // Excepciones del procesador (0-31)
    for i in 0..32 {
        IDT.set_entry(
            i,
            get_exception_handler_address(i),
            KERNEL_CS,
            IDT_FLAG_PRESENT | IDT_FLAG_RING0 | IDT_FLAG_INTERRUPT_GATE,
        );
    }
    
    // IRQ 0-15 (32-47)
    for i in 0..16 {
        IDT.set_entry(
            32 + i,
            get_irq_handler_address(i),
            KERNEL_CS,
            IDT_FLAG_PRESENT | IDT_FLAG_RING0 | IDT_FLAG_INTERRUPT_GATE,
        );
    }
    
    // Interrupciones del sistema (48-255)
    for i in 48..256 {
        IDT.set_entry(
            i,
            get_system_interrupt_handler_address(i),
            KERNEL_CS,
            IDT_FLAG_PRESENT | IDT_FLAG_RING0 | IDT_FLAG_INTERRUPT_GATE,
        );
    }
}

/// Obtener dirección del handler de excepción
fn get_exception_handler_address(exception: usize) -> u64 {
    match exception {
        0 => divide_error_handler as u64,
        1 => debug_handler as u64,
        2 => nmi_handler as u64,
        3 => breakpoint_handler as u64,
        4 => overflow_handler as u64,
        5 => bound_range_handler as u64,
        6 => invalid_opcode_handler as u64,
        7 => device_not_available_handler as u64,
        8 => double_fault_handler as u64,
        9 => coprocessor_segment_handler as u64,
        10 => invalid_tss_handler as u64,
        11 => segment_not_present_handler as u64,
        12 => stack_fault_handler as u64,
        13 => general_protection_handler as u64,
        14 => page_fault_handler as u64,
        15 => reserved_handler as u64,
        16 => x87_fpu_handler as u64,
        17 => alignment_check_handler as u64,
        18 => machine_check_handler as u64,
        19 => simd_fpu_handler as u64,
        20 => virtualization_handler as u64,
        21..=31 => reserved_handler as u64,
        _ => reserved_handler as u64,
    }
}

/// Obtener dirección del handler de IRQ
fn get_irq_handler_address(irq: usize) -> u64 {
    match irq {
        0 => timer_handler as u64,
        1 => keyboard_irq_handler as u64,
        2 => cascade_handler as u64,
        3 => com2_handler as u64,
        4 => com1_handler as u64,
        5 => lpt2_handler as u64,
        6 => floppy_handler as u64,
        7 => lpt1_handler as u64,
        8 => rtc_handler as u64,
        9 => acpi_handler as u64,
        10 => reserved_irq_handler as u64,
        11 => reserved_irq_handler as u64,
        12 => mouse_irq_handler as u64,
        13 => fpu_handler as u64,
        14 => primary_ata_handler as u64,
        15 => secondary_ata_handler as u64,
        _ => reserved_irq_handler as u64,
    }
}

/// Obtener dirección del handler de interrupción del sistema
fn get_system_interrupt_handler_address(_interrupt: usize) -> u64 {
    system_interrupt_handler as u64
}

/// Handler genérico de interrupciones
#[no_mangle]
pub extern "C" fn generic_interrupt_handler(interrupt_number: u64) {
    // Incrementar contador
    {
        let mut counter = INTERRUPT_COUNTER.lock();
        if (interrupt_number as usize) < IDT_SIZE {
            counter[interrupt_number as usize] += 1;
        }
    }
    
    // Enviar EOI si es una IRQ
    if interrupt_number >= 32 && interrupt_number < 48 {
        send_eoi(interrupt_number - 32);
    }
}

/// Enviar End of Interrupt (EOI)
fn send_eoi(irq: u64) {
    if irq >= 8 {
        // IRQ 8-15 requieren EOI al slave PIC
        unsafe {
            asm!("mov al, 0x20", "out 0xA0, al", options(nostack));
        }
    }
    
    // EOI al master PIC
    unsafe {
        asm!("mov al, 0x20", "out 0x20, al", options(nostack));
    }
}

// Handlers de excepciones
#[no_mangle]
pub extern "C" fn divide_error_handler() {
    generic_interrupt_handler(0);
    // TODO: Manejar división por cero
}

#[no_mangle]
pub extern "C" fn debug_handler() {
    generic_interrupt_handler(1);
    // TODO: Manejar debug
}

#[no_mangle]
pub extern "C" fn nmi_handler() {
    generic_interrupt_handler(2);
    // TODO: Manejar NMI
}

#[no_mangle]
pub extern "C" fn breakpoint_handler() {
    generic_interrupt_handler(3);
    // TODO: Manejar breakpoint
}

#[no_mangle]
pub extern "C" fn overflow_handler() {
    generic_interrupt_handler(4);
    // TODO: Manejar overflow
}

#[no_mangle]
pub extern "C" fn bound_range_handler() {
    generic_interrupt_handler(5);
    // TODO: Manejar bound range
}

#[no_mangle]
pub extern "C" fn invalid_opcode_handler() {
    generic_interrupt_handler(6);
    // TODO: Manejar opcode inválido
}

#[no_mangle]
pub extern "C" fn device_not_available_handler() {
    generic_interrupt_handler(7);
    // TODO: Manejar dispositivo no disponible
}

#[no_mangle]
pub extern "C" fn double_fault_handler() {
    generic_interrupt_handler(8);
    // TODO: Manejar double fault
}

#[no_mangle]
pub extern "C" fn coprocessor_segment_handler() {
    generic_interrupt_handler(9);
    // TODO: Manejar coprocessor segment
}

#[no_mangle]
pub extern "C" fn invalid_tss_handler() {
    generic_interrupt_handler(10);
    // TODO: Manejar TSS inválido
}

#[no_mangle]
pub extern "C" fn segment_not_present_handler() {
    generic_interrupt_handler(11);
    // TODO: Manejar segmento no presente
}

#[no_mangle]
pub extern "C" fn stack_fault_handler() {
    generic_interrupt_handler(12);
    // TODO: Manejar stack fault
}

#[no_mangle]
pub extern "C" fn general_protection_handler() {
    generic_interrupt_handler(13);
    // TODO: Manejar general protection
}

#[no_mangle]
pub extern "C" fn page_fault_handler() {
    generic_interrupt_handler(14);
    // TODO: Manejar page fault
}

#[no_mangle]
pub extern "C" fn reserved_handler() {
    generic_interrupt_handler(15);
    // TODO: Manejar reservado
}

#[no_mangle]
pub extern "C" fn x87_fpu_handler() {
    generic_interrupt_handler(16);
    // TODO: Manejar x87 FPU
}

#[no_mangle]
pub extern "C" fn alignment_check_handler() {
    generic_interrupt_handler(17);
    // TODO: Manejar alignment check
}

#[no_mangle]
pub extern "C" fn machine_check_handler() {
    generic_interrupt_handler(18);
    // TODO: Manejar machine check
}

#[no_mangle]
pub extern "C" fn simd_fpu_handler() {
    generic_interrupt_handler(19);
    // TODO: Manejar SIMD FPU
}

#[no_mangle]
pub extern "C" fn virtualization_handler() {
    generic_interrupt_handler(20);
    // TODO: Manejar virtualización
}

// Handlers de IRQ
#[no_mangle]
pub extern "C" fn timer_handler() {
    generic_interrupt_handler(32);
    // TODO: Manejar timer
}

#[no_mangle]
pub extern "C" fn keyboard_irq_handler() {
    generic_interrupt_handler(33);
    // TODO: Manejar teclado IRQ
}

#[no_mangle]
pub extern "C" fn cascade_handler() {
    generic_interrupt_handler(34);
    // TODO: Manejar cascade
}

#[no_mangle]
pub extern "C" fn com2_handler() {
    generic_interrupt_handler(35);
    // TODO: Manejar COM2
}

#[no_mangle]
pub extern "C" fn com1_handler() {
    generic_interrupt_handler(36);
    // TODO: Manejar COM1
}

#[no_mangle]
pub extern "C" fn lpt2_handler() {
    generic_interrupt_handler(37);
    // TODO: Manejar LPT2
}

#[no_mangle]
pub extern "C" fn floppy_handler() {
    generic_interrupt_handler(38);
    // TODO: Manejar floppy
}

#[no_mangle]
pub extern "C" fn lpt1_handler() {
    generic_interrupt_handler(39);
    // TODO: Manejar LPT1
}

#[no_mangle]
pub extern "C" fn rtc_handler() {
    generic_interrupt_handler(40);
    // TODO: Manejar RTC
}

#[no_mangle]
pub extern "C" fn acpi_handler() {
    generic_interrupt_handler(41);
    // TODO: Manejar ACPI
}

#[no_mangle]
pub extern "C" fn reserved_irq_handler() {
    generic_interrupt_handler(42);
    // TODO: Manejar IRQ reservado
}

#[no_mangle]
pub extern "C" fn mouse_irq_handler() {
    generic_interrupt_handler(44);
    // TODO: Manejar mouse IRQ
}

#[no_mangle]
pub extern "C" fn fpu_handler() {
    generic_interrupt_handler(45);
    // TODO: Manejar FPU
}

#[no_mangle]
pub extern "C" fn primary_ata_handler() {
    generic_interrupt_handler(46);
    // TODO: Manejar ATA primario
}

#[no_mangle]
pub extern "C" fn secondary_ata_handler() {
    generic_interrupt_handler(47);
    // TODO: Manejar ATA secundario
}

#[no_mangle]
pub extern "C" fn system_interrupt_handler() {
    generic_interrupt_handler(48);
    // TODO: Manejar interrupción del sistema
}

/// Obtener estadísticas de interrupciones
pub fn get_interrupt_stats() -> String {
    let counter = INTERRUPT_COUNTER.lock();
    let mut stats = String::from("Interrupciones: ");
    let mut total = 0;
    
    for (i, &count) in counter.iter().enumerate() {
        if count > 0 {
            total += count;
            if i < 32 {
                stats.push_str(&format!("Exc{}:{} ", i, count));
            } else if i < 48 {
                stats.push_str(&format!("IRQ{}:{} ", i-32, count));
            } else {
                stats.push_str(&format!("Sys{}:{} ", i, count));
            }
        }
    }
    
    stats.push_str(&format!("Total:{}", total));
    stats
}

/// Obtener información del sistema de interrupciones
pub fn get_interrupt_info() -> String {
    String::from("IDT: 256 entradas, Handlers: Básicos, Estado: Activo")
}

/// Verificar si el sistema de interrupciones está disponible
pub fn is_interrupt_system_available() -> bool {
    true
}

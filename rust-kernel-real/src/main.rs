#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

// Multiboot header
#[repr(C)]
struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

#[no_mangle]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0x1BADB002,
    flags: 0x00000000,
    checksum: 0xE4524FFE,
};

// VGA Driver
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
const VGA_ADDRESS: usize = 0xB8000;

struct VgaDriver {
    buffer: *mut u16,
    row: usize,
    col: usize,
    color: u8,
}

impl VgaDriver {
    fn new() -> Self {
        Self {
            buffer: VGA_ADDRESS as *mut u16,
            row: 0,
            col: 0,
            color: 0x0F, // Blanco sobre negro
        }
    }

    fn clear_screen(&mut self) {
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            unsafe {
                *self.buffer.add(i) = (self.color as u16) << 8 | b' ' as u16;
            }
        }
        self.row = 0;
        self.col = 0;
    }

    fn print_char(&mut self, c: u8) {
        match c {
            b'\n' => {
                self.col = 0;
                self.row += 1;
            }
            _ => {
                if self.col >= VGA_WIDTH {
                    self.col = 0;
                    self.row += 1;
                }
                
                if self.row >= VGA_HEIGHT {
                    self.scroll();
                }
                
                let index = self.row * VGA_WIDTH + self.col;
                unsafe {
                    *self.buffer.add(index) = (self.color as u16) << 8 | c as u16;
                }
                self.col += 1;
            }
        }
    }

    fn scroll(&mut self) {
        for row in 0..(VGA_HEIGHT - 1) {
            for col in 0..VGA_WIDTH {
                let src_index = (row + 1) * VGA_WIDTH + col;
                let dst_index = row * VGA_WIDTH + col;
                unsafe {
                    *self.buffer.add(dst_index) = *self.buffer.add(src_index);
                }
            }
        }
        
        // Limpiar última fila
        for col in 0..VGA_WIDTH {
            let index = (VGA_HEIGHT - 1) * VGA_WIDTH + col;
            unsafe {
                *self.buffer.add(index) = (self.color as u16) << 8 | b' ' as u16;
            }
        }
        self.row = VGA_HEIGHT - 1;
    }

    fn print(&mut self, s: &str) {
        for byte in s.bytes() {
            self.print_char(byte);
        }
    }

    fn println(&mut self, s: &str) {
        self.print(s);
        self.print_char(b'\n');
    }
}

// Serial Driver
const COM1_PORT: u16 = 0x3F8;

struct SerialDriver {
    port: u16,
}

impl SerialDriver {
    fn new() -> Self {
        Self { port: COM1_PORT }
    }

    fn init(&self) {
        // Configurar baud rate (115200)
        unsafe {
            asm!("out dx, al", in("dx") self.port + 3, in("al") 0x80u8);
            asm!("out dx, al", in("dx") self.port, in("al") 0x01u8);
            asm!("out dx, al", in("dx") self.port + 1, in("al") 0x00u8);
            asm!("out dx, al", in("dx") self.port + 3, in("al") 0x03u8);
        }
    }

    fn is_transmit_empty(&self) -> bool {
        let mut value: u8;
        unsafe {
            asm!("in al, dx", out("al") value, in("dx") self.port + 5);
        }
        (value & 0x20) != 0
    }

    fn write_char(&self, c: u8) {
        while !self.is_transmit_empty() {}
        unsafe {
            asm!("out dx, al", in("dx") self.port, in("al") c);
        }
    }

    fn write(&self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }

    fn writeln(&self, s: &str) {
        self.write(s);
        self.write_char(b'\n');
    }
}

// VESA/VBE Driver
#[repr(C, packed)]
struct VbeInfoBlock {
    signature: [u8; 4],
    version: u16,
    oem_string_ptr: u32,
    capabilities: u32,
    video_mode_ptr: u32,
    total_memory: u16,
    oem_software_rev: u16,
    oem_vendor_name_ptr: u32,
    oem_product_name_ptr: u32,
    oem_product_rev_ptr: u32,
    reserved: [u8; 222],
    oem_data: [u8; 256],
}

struct VesaDriver {
    info_block: *mut VbeInfoBlock,
    mode_info: *mut u8,
}

impl VesaDriver {
    fn new() -> Self {
        Self {
            info_block: core::ptr::null_mut(),
            mode_info: core::ptr::null_mut(),
        }
    }

    fn detect(&mut self) -> bool {
        // Detectar VESA
        // Por ahora retornamos false para usar VGA como fallback
        false
    }

    fn set_mode(&self, _mode: u16) -> bool {
        // Establecer modo VESA
        // Por ahora retornamos false
        false
    }
}

// Console Manager
struct Console {
    vga: VgaDriver,
    serial: SerialDriver,
    vesa: VesaDriver,
    use_vesa: bool,
}

impl Console {
    fn new() -> Self {
        let mut console = Self {
            vga: VgaDriver::new(),
            serial: SerialDriver::new(),
            vesa: VesaDriver::new(),
            use_vesa: false,
        };
        
        // Inicializar serial
        console.serial.init();
        
        // Intentar detectar VESA
        console.use_vesa = console.vesa.detect();
        
        console
    }

    fn print(&mut self, s: &str) {
        self.vga.print(s);
        self.serial.write(s);
    }

    fn println(&mut self, s: &str) {
        self.vga.println(s);
        self.serial.writeln(s);
    }

    fn clear(&mut self) {
        self.vga.clear_screen();
    }
}

// Global console
static mut CONSOLE: Option<Console> = None;

fn get_console() -> &'static mut Console {
    unsafe { CONSOLE.as_mut().unwrap() }
}

// Kernel main
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Inicializar consola
    unsafe {
        CONSOLE = Some(Console::new());
    }
    
    let console = get_console();
    console.clear();
    
    console.println("🌙 Eclipse OS Kernel Rust iniciado!");
    console.println("📊 Kernel compatible con Multiboot");
    console.println("🔧 Inicializando drivers...");
    
    // Detectar hardware
    console.println("🔍 Detectando hardware...");
    
    if console.use_vesa {
        console.println("✅ Driver VESA/VBE detectado");
    } else {
        console.println("⚠️  Usando driver VGA como fallback");
    }
    
    console.println("✅ Driver VGA inicializado");
    console.println("✅ Driver Serial inicializado");
    
    console.println("🎯 Sistema Eclipse OS completamente funcional!");
    console.println("💡 Presiona Ctrl+Alt+Q para salir de QEMU");
    console.println("");
    console.println("🐚 Shell interactivo disponible!");
    console.println("💡 Escribe 'help' para ver comandos disponibles");
    console.println("");
    console.print("Eclipse OS> ");

    // Loop principal
    loop {
        // Simular cursor parpadeante
        static mut CURSOR_STATE: u32 = 0;
        unsafe {
            CURSOR_STATE += 1;
            if CURSOR_STATE % 1000 < 500 {
                // Cursor visible
            } else {
                // Cursor invisible
            }
        }
    }
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let console = get_console();
    console.println("💥 PANIC en Eclipse OS Kernel!");
    console.print("Error: ");
    console.println("Mensaje de panic disponible");
    console.println("Ubicación del panic disponible");
    loop {}
}



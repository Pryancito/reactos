//! VGA Text Mode Driver
//! 
//! Driver simple para escribir texto en modo VGA 80x25

use core::fmt;

/// Colores disponibles para VGA
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Color de primer plano y fondo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Carácter en pantalla VGA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// Tamaño del buffer VGA
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// Buffer VGA
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Writer para VGA
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Crear un nuevo Writer
    pub fn new() -> Self {
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::LightCyan, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    /// Escribir un byte
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    /// Escribir una cadena
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // ASCII imprimible o nueva línea
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // No imprimible
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Nueva línea
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Limpiar una fila
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }

    /// Limpiar toda la pantalla
    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
    }

    /// Cambiar color
    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Writer global
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

/// Macro para imprimir en VGA
#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

/// Macro para imprimir en VGA con nueva línea
#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga::_print(format_args!("\n")));
    ($($arg:tt)*) => ($crate::vga::_print(format_args!("{}\n", format_args!($($arg)*))));
}

/// Función interna para imprimir
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// Inicializar VGA
pub fn init() {
    let mut writer = WRITER.lock();
    writer.clear_screen();
    writer.set_color(Color::LightGreen, Color::Black);
    vga_println!("🚀 ReactOS Rust Kernel - VGA Initialized!");
    writer.set_color(Color::LightCyan, Color::Black);
}

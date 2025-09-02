//! Driver completo de teclado para ReactOS Rust Kernel
//! 
//! Implementación completa de driver de teclado PS/2 con buffer y funcionalidad avanzada

use core::arch::asm;
use alloc::string::String;
use alloc::format;

// Puertos de E/S del teclado
const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;
const KEYBOARD_COMMAND_PORT: u16 = 0x64;

// Estados del teclado
const KEYBOARD_STATUS_OUTPUT_BUFFER_FULL: u8 = 0x01;
const KEYBOARD_STATUS_INPUT_BUFFER_FULL: u8 = 0x02;

// Códigos de teclas especiales
const KEY_ENTER: u8 = 0x1C;
const KEY_BACKSPACE: u8 = 0x0E;
const KEY_ESC: u8 = 0x01;
const KEY_SPACE: u8 = 0x39;
const KEY_TAB: u8 = 0x0F;
const KEY_CAPS_LOCK: u8 = 0x3A;
const KEY_SHIFT_LEFT: u8 = 0x2A;
const KEY_SHIFT_RIGHT: u8 = 0x36;
const KEY_CTRL_LEFT: u8 = 0x1D;
const KEY_ALT_LEFT: u8 = 0x38;

// Códigos de liberación de teclas (prefijo 0x80)
const KEY_RELEASE_PREFIX: u8 = 0x80;

/// Estado de las teclas modificadoras
#[derive(Debug, Clone, Copy)]
pub struct ModifierState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub caps_lock: bool,
}

impl Default for ModifierState {
    fn default() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            caps_lock: false,
        }
    }
}

/// Driver completo de teclado
pub struct KeyboardDriver {
    buffer: [u8; 256],
    buffer_index: usize,
    buffer_read_index: usize,
    modifier_state: ModifierState,
    is_initialized: bool,
}

impl KeyboardDriver {
    /// Crear un nuevo driver de teclado
    pub fn new() -> Self {
        Self {
            buffer: [0; 256],
            buffer_index: 0,
            buffer_read_index: 0,
            modifier_state: ModifierState::default(),
            is_initialized: false,
        }
    }

    /// Inicializar el driver de teclado
    pub fn init(&mut self) {
        // Limpiar buffer
        self.buffer = [0; 256];
        self.buffer_index = 0;
        self.buffer_read_index = 0;
        self.modifier_state = ModifierState::default();
        
        // Habilitar teclado
        self.enable_keyboard();
        self.is_initialized = true;
    }

    /// Habilitar el teclado
    fn enable_keyboard(&self) {
        unsafe {
            // Comando para habilitar teclado
            asm!("out dx, al", in("dx") KEYBOARD_COMMAND_PORT, in("al") 0xAEu8);
        }
    }

    /// Verificar si hay datos disponibles
    pub fn has_data(&self) -> bool {
        unsafe {
            let status: u8;
            asm!("in al, dx", out("al") status, in("dx") KEYBOARD_STATUS_PORT);
            (status & KEYBOARD_STATUS_OUTPUT_BUFFER_FULL) != 0
        }
    }

    /// Leer un byte del teclado
    pub fn read_byte(&self) -> u8 {
        unsafe {
            let data: u8;
            asm!("in al, dx", out("al") data, in("dx") KEYBOARD_DATA_PORT);
            data
        }
    }

    /// Procesar entrada del teclado
    pub fn process_input(&mut self) -> Option<char> {
        if !self.has_data() {
            return None;
        }

        let scancode = self.read_byte();
        
        // Verificar si es liberación de tecla
        if scancode & KEY_RELEASE_PREFIX != 0 {
            let key_code = scancode & !KEY_RELEASE_PREFIX;
            self.handle_key_release(key_code);
            return None;
        }
        
        // Procesar presión de tecla
        self.handle_key_press(scancode)
    }

    /// Manejar presión de tecla
    fn handle_key_press(&mut self, scancode: u8) -> Option<char> {
        match scancode {
            KEY_ENTER => Some('\n'),
            KEY_BACKSPACE => Some('\x08'),
            KEY_ESC => Some('\x1B'),
            KEY_SPACE => Some(' '),
            KEY_TAB => Some('\t'),
            KEY_SHIFT_LEFT | KEY_SHIFT_RIGHT => {
                self.modifier_state.shift = true;
                None
            }
            KEY_CTRL_LEFT => {
                self.modifier_state.ctrl = true;
                None
            }
            KEY_ALT_LEFT => {
                self.modifier_state.alt = true;
                None
            }
            KEY_CAPS_LOCK => {
                self.modifier_state.caps_lock = !self.modifier_state.caps_lock;
                None
            }
            _ => {
                // Convertir scancode a carácter
                self.scancode_to_char(scancode)
            }
        }
    }

    /// Manejar liberación de tecla
    fn handle_key_release(&mut self, scancode: u8) {
        match scancode {
            KEY_SHIFT_LEFT | KEY_SHIFT_RIGHT => {
                self.modifier_state.shift = false;
            }
            KEY_CTRL_LEFT => {
                self.modifier_state.ctrl = false;
            }
            KEY_ALT_LEFT => {
                self.modifier_state.alt = false;
            }
            _ => {}
        }
    }

    /// Convertir scancode a carácter
    fn scancode_to_char(&self, scancode: u8) -> Option<char> {
        let is_shift = self.modifier_state.shift;
        let is_caps = self.modifier_state.caps_lock;
        let is_upper = is_shift ^ is_caps;

        match scancode {
            // Números
            0x02..=0x0B => {
                let digit = if scancode == 0x0B { '0' } else { (b'1' + scancode - 2) as char };
                Some(digit)
            }
            // Letras Q-P
            0x10..=0x19 => {
                let base_char = (b'Q' + scancode - 0x10) as char;
                Some(if is_upper { base_char } else { base_char.to_ascii_lowercase() })
            }
            // Letras A-L
            0x1E..=0x26 => {
                let base_char = (b'A' + scancode - 0x1E) as char;
                Some(if is_upper { base_char } else { base_char.to_ascii_lowercase() })
            }
            // Letras Z-M
            0x2C..=0x32 => {
                let base_char = (b'Z' + scancode - 0x2C) as char;
                Some(if is_upper { base_char } else { base_char.to_ascii_lowercase() })
            }
            _ => None
        }
    }

    /// Obtener estado del driver
    pub fn get_status(&self) -> &str {
        if !self.is_initialized {
            "No inicializado"
        } else if self.buffer_index > 0 {
            "Activo con datos"
        } else {
            "Activo"
        }
    }

    /// Obtener estado de las teclas modificadoras
    pub fn get_modifier_state(&self) -> ModifierState {
        self.modifier_state
    }

    /// Verificar si el driver está inicializado
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    /// Obtener información detallada del driver
    pub fn get_info(&self) -> String {
        format!(
            "Teclado: {} | Shift: {} | Ctrl: {} | Alt: {} | Caps: {}",
            if self.is_initialized { "OK" } else { "ERROR" },
            if self.modifier_state.shift { "ON" } else { "OFF" },
            if self.modifier_state.ctrl { "ON" } else { "OFF" },
            if self.modifier_state.alt { "ON" } else { "OFF" },
            if self.modifier_state.caps_lock { "ON" } else { "OFF" }
        )
    }
}

// Instancia global del driver de teclado
static mut KEYBOARD_DRIVER: Option<KeyboardDriver> = None;

/// Inicializar el driver de teclado global
pub fn init_keyboard() {
    unsafe {
        KEYBOARD_DRIVER = Some(KeyboardDriver::new());
        if let Some(ref mut driver) = KEYBOARD_DRIVER {
            driver.init();
        }
    }
}

/// Obtener instancia del driver de teclado
pub fn get_keyboard_driver() -> Option<&'static mut KeyboardDriver> {
    unsafe {
        KEYBOARD_DRIVER.as_mut()
    }
}

/// Procesar entrada del teclado global
pub fn process_keyboard_input() -> Option<char> {
    if let Some(ref mut driver) = get_keyboard_driver() {
        driver.process_input()
    } else {
        None
    }
}

/// Obtener información del teclado global
pub fn get_keyboard_info() -> String {
    if let Some(ref driver) = get_keyboard_driver() {
        driver.get_info()
    } else {
        String::from("Teclado: NO DISPONIBLE")
    }
}

/// Verificar si el teclado está disponible
pub fn is_keyboard_available() -> bool {
    get_keyboard_driver().map(|d| d.is_initialized()).unwrap_or(false)
}

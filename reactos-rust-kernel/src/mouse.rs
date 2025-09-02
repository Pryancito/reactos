//! Driver básico de mouse para ReactOS Rust Kernel
//! 
//! Implementación básica de driver de mouse PS/2

use core::arch::asm;
use alloc::string::String;
use alloc::format;

// Puertos de E/S del mouse
const MOUSE_DATA_PORT: u16 = 0x60;
const MOUSE_STATUS_PORT: u16 = 0x64;
const MOUSE_COMMAND_PORT: u16 = 0x64;

// Estados del mouse
const MOUSE_STATUS_OUTPUT_BUFFER_FULL: u8 = 0x01;
const MOUSE_STATUS_INPUT_BUFFER_FULL: u8 = 0x02;

// Comandos del mouse
const MOUSE_CMD_ENABLE_AUXILIARY: u8 = 0xA8;
const MOUSE_CMD_SET_DEFAULTS: u8 = 0xF6;
const MOUSE_CMD_ENABLE_PACKET_STREAMING: u8 = 0xF4;
const MOUSE_CMD_RESET: u8 = 0xFF;

// Respuestas del mouse
const MOUSE_ACK: u8 = 0xFA;
const MOUSE_SELF_TEST_PASS: u8 = 0xAA;

// Estados de los botones del mouse
#[derive(Debug, Clone, Copy)]
pub struct MouseButtons {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

impl Default for MouseButtons {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            middle: false,
        }
    }
}

// Posición del mouse
#[derive(Debug, Clone, Copy)]
pub struct MousePosition {
    pub x: i16,
    pub y: i16,
}

impl Default for MousePosition {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

// Driver de mouse
pub struct MouseDriver {
    position: MousePosition,
    buttons: MouseButtons,
    is_initialized: bool,
    packet_buffer: [u8; 3],
    packet_index: usize,
}

impl MouseDriver {
    /// Crear un nuevo driver de mouse
    pub fn new() -> Self {
        Self {
            position: MousePosition::default(),
            buttons: MouseButtons::default(),
            is_initialized: false,
            packet_buffer: [0; 3],
            packet_index: 0,
        }
    }

    /// Inicializar el driver de mouse
    pub fn init(&mut self) -> bool {
        // Limpiar estado
        self.position = MousePosition::default();
        self.buttons = MouseButtons::default();
        self.packet_buffer = [0; 3];
        self.packet_index = 0;

        // Habilitar puerto auxiliar (mouse)
        if !self.enable_auxiliary_port() {
            return false;
        }

        // Configurar mouse
        if !self.configure_mouse() {
            return false;
        }

        self.is_initialized = true;
        true
    }

    /// Habilitar puerto auxiliar
    fn enable_auxiliary_port(&self) -> bool {
        unsafe {
            // Comando para habilitar puerto auxiliar
            asm!("out dx, al", in("dx") MOUSE_COMMAND_PORT, in("al") MOUSE_CMD_ENABLE_AUXILIARY);
        }
        true
    }

    /// Configurar mouse
    fn configure_mouse(&self) -> bool {
        // Enviar comando de reset
        if !self.send_mouse_command(MOUSE_CMD_RESET) {
            return false;
        }

        // Esperar respuesta
        if !self.wait_for_ack() {
            return false;
        }

        // Configurar valores por defecto
        if !self.send_mouse_command(MOUSE_CMD_SET_DEFAULTS) {
            return false;
        }

        // Habilitar streaming de paquetes
        if !self.send_mouse_command(MOUSE_CMD_ENABLE_PACKET_STREAMING) {
            return false;
        }

        true
    }

    /// Enviar comando al mouse
    fn send_mouse_command(&self, command: u8) -> bool {
        // Esperar a que el buffer de entrada esté vacío
        while self.is_input_buffer_full() {
            // Esperar
        }

        // Enviar comando
        unsafe {
            asm!("out dx, al", in("dx") MOUSE_DATA_PORT, in("al") command);
        }

        true
    }

    /// Esperar ACK del mouse
    fn wait_for_ack(&self) -> bool {
        // Esperar respuesta
        for _ in 0..1000 {
            if self.has_data() {
                let response = self.read_byte();
                if response == MOUSE_ACK {
                    return true;
                }
            }
        }
        false
    }

    /// Verificar si hay datos disponibles
    pub fn has_data(&self) -> bool {
        unsafe {
            let status: u8;
            asm!("in al, dx", out("al") status, in("dx") MOUSE_STATUS_PORT);
            (status & MOUSE_STATUS_OUTPUT_BUFFER_FULL) != 0
        }
    }

    /// Verificar si el buffer de entrada está lleno
    fn is_input_buffer_full(&self) -> bool {
        unsafe {
            let status: u8;
            asm!("in al, dx", out("al") status, in("dx") MOUSE_STATUS_PORT);
            (status & MOUSE_STATUS_INPUT_BUFFER_FULL) != 0
        }
    }

    /// Leer un byte del mouse
    fn read_byte(&self) -> u8 {
        unsafe {
            let data: u8;
            asm!("in al, dx", out("al") data, in("dx") MOUSE_DATA_PORT);
            data
        }
    }

    /// Procesar entrada del mouse
    pub fn process_input(&mut self) -> bool {
        if !self.has_data() {
            return false;
        }

        let byte = self.read_byte();
        self.packet_buffer[self.packet_index] = byte;
        self.packet_index += 1;

        // Procesar paquete completo (3 bytes)
        if self.packet_index >= 3 {
            self.process_packet();
            self.packet_index = 0;
            return true;
        }

        false
    }

    /// Procesar paquete del mouse
    fn process_packet(&mut self) {
        let status = self.packet_buffer[0];
        let x_delta = self.packet_buffer[1] as i8 as i16;
        let y_delta = self.packet_buffer[2] as i8 as i16;

        // Actualizar botones
        self.buttons.left = (status & 0x01) != 0;
        self.buttons.right = (status & 0x02) != 0;
        self.buttons.middle = (status & 0x04) != 0;

        // Actualizar posición
        self.position.x += x_delta;
        self.position.y -= y_delta; // Invertir Y para coordenadas de pantalla

        // Limitar posición a la pantalla (80x25)
        if self.position.x < 0 { self.position.x = 0; }
        if self.position.x >= 80 { self.position.x = 79; }
        if self.position.y < 0 { self.position.y = 0; }
        if self.position.y >= 25 { self.position.y = 24; }
    }

    /// Obtener posición del mouse
    pub fn get_position(&self) -> MousePosition {
        self.position
    }

    /// Obtener estado de los botones
    pub fn get_buttons(&self) -> MouseButtons {
        self.buttons
    }

    /// Verificar si el driver está inicializado
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    /// Obtener información del driver
    pub fn get_info(&self) -> String {
        format!(
            "Mouse: {} | Pos: ({}, {}) | Botones: L:{} R:{} M:{}",
            if self.is_initialized { "OK" } else { "ERROR" },
            self.position.x,
            self.position.y,
            if self.buttons.left { "ON" } else { "OFF" },
            if self.buttons.right { "ON" } else { "OFF" },
            if self.buttons.middle { "ON" } else { "OFF" }
        )
    }

    /// Obtener estado del driver
    pub fn get_status(&self) -> &str {
        if !self.is_initialized {
            "No inicializado"
        } else {
            "Activo"
        }
    }
}

// Instancia global del driver de mouse
static mut MOUSE_DRIVER: Option<MouseDriver> = None;

/// Inicializar el driver de mouse global
pub fn init_mouse() -> bool {
    unsafe {
        MOUSE_DRIVER = Some(MouseDriver::new());
        if let Some(ref mut driver) = MOUSE_DRIVER {
            driver.init()
        } else {
            false
        }
    }
}

/// Obtener instancia del driver de mouse
pub fn get_mouse_driver() -> Option<&'static mut MouseDriver> {
    unsafe {
        MOUSE_DRIVER.as_mut()
    }
}

/// Procesar entrada del mouse global
pub fn process_mouse_input() -> bool {
    if let Some(ref mut driver) = get_mouse_driver() {
        driver.process_input()
    } else {
        false
    }
}

/// Obtener información del mouse global
pub fn get_mouse_info() -> String {
    if let Some(ref driver) = get_mouse_driver() {
        driver.get_info()
    } else {
        String::from("Mouse: NO DISPONIBLE")
    }
}

/// Verificar si el mouse está disponible
pub fn is_mouse_available() -> bool {
    get_mouse_driver().map(|d| d.is_initialized()).unwrap_or(false)
}

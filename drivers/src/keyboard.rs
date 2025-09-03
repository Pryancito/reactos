//! Keyboard Driver - Implementación en Rust
//! 
//! Driver de teclado para ReactOS Rust OS
//! Soporte para teclado PS/2 y USB

use core::ffi::{c_char, c_void};
use core::ptr;
use core::sync::atomic::{AtomicU8, AtomicU32, Ordering};

// Tipos de datos
pub type KeyboardHandle = *mut c_void;
pub type KeyboardResult = i32;
pub type KeyboardError = i32;

// Constantes de éxito y error
pub const KEYBOARD_SUCCESS: KeyboardResult = 0;
pub const KEYBOARD_ERROR_INVALID_PARAM: KeyboardError = 0x80000001u32 as i32;
pub const KEYBOARD_ERROR_NOT_SUPPORTED: KeyboardError = 0x80000002u32 as i32;
pub const KEYBOARD_ERROR_HARDWARE_ERROR: KeyboardError = 0x80000003u32 as i32;
pub const KEYBOARD_ERROR_BUFFER_FULL: KeyboardError = 0x80000004u32 as i32;

// Constantes de teclado
pub const KEYBOARD_BUFFER_SIZE: usize = 256;
pub const KEYBOARD_PS2_DATA_PORT: u16 = 0x60;
pub const KEYBOARD_PS2_STATUS_PORT: u16 = 0x64;
pub const KEYBOARD_PS2_COMMAND_PORT: u16 = 0x64;

// Códigos de tecla especiales
pub const KEY_ESCAPE: u8 = 0x01;
pub const KEY_BACKSPACE: u8 = 0x0E;
pub const KEY_TAB: u8 = 0x0F;
pub const KEY_ENTER: u8 = 0x1C;
pub const KEY_LEFT_CTRL: u8 = 0x1D;
pub const KEY_LEFT_SHIFT: u8 = 0x2A;
pub const KEY_RIGHT_SHIFT: u8 = 0x36;
pub const KEY_LEFT_ALT: u8 = 0x38;
pub const KEY_SPACE: u8 = 0x39;
pub const KEY_CAPS_LOCK: u8 = 0x3A;
pub const KEY_F1: u8 = 0x3B;
pub const KEY_F2: u8 = 0x3C;
pub const KEY_F3: u8 = 0x3D;
pub const KEY_F4: u8 = 0x3E;
pub const KEY_F5: u8 = 0x3F;
pub const KEY_F6: u8 = 0x40;
pub const KEY_F7: u8 = 0x41;
pub const KEY_F8: u8 = 0x42;
pub const KEY_F9: u8 = 0x43;
pub const KEY_F10: u8 = 0x44;
pub const KEY_F11: u8 = 0x57;
pub const KEY_F12: u8 = 0x58;
pub const KEY_NUM_LOCK: u8 = 0x45;
pub const KEY_SCROLL_LOCK: u8 = 0x46;
pub const KEY_INSERT: u8 = 0x52;
pub const KEY_DELETE: u8 = 0x53;
pub const KEY_HOME: u8 = 0x47;
pub const KEY_END: u8 = 0x4F;
pub const KEY_PAGE_UP: u8 = 0x49;
pub const KEY_PAGE_DOWN: u8 = 0x51;
pub const KEY_UP: u8 = 0x48;
pub const KEY_DOWN: u8 = 0x50;
pub const KEY_LEFT: u8 = 0x4B;
pub const KEY_RIGHT: u8 = 0x4D;

// Flags de estado
pub const KEYBOARD_FLAG_SHIFT: u8 = 0x01;
pub const KEYBOARD_FLAG_CTRL: u8 = 0x02;
pub const KEYBOARD_FLAG_ALT: u8 = 0x04;
pub const KEYBOARD_FLAG_CAPS_LOCK: u8 = 0x08;
pub const KEYBOARD_FLAG_NUM_LOCK: u8 = 0x10;
pub const KEYBOARD_FLAG_SCROLL_LOCK: u8 = 0x20;

// Estructuras

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct KeyboardEvent {
    pub key_code: u8,               // Código de tecla
    pub character: u8,              // Carácter
    pub flags: u8,                  // Flags de estado
    pub pressed: bool,              // Tecla presionada o liberada
    pub timestamp: u64,             // Timestamp del evento
}

#[repr(C, packed)]
pub struct KeyboardBuffer {
    pub events: [KeyboardEvent; KEYBOARD_BUFFER_SIZE], // Buffer de eventos
    pub head: usize,                // Índice de cabeza
    pub tail: usize,                // Índice de cola
    pub count: usize,               // Número de eventos
}

#[repr(C, packed)]
pub struct KeyboardState {
    pub flags: u8,                  // Flags de estado
    pub last_key: u8,               // Última tecla presionada
    pub repeat_count: u8,           // Contador de repetición
    pub repeat_delay: u32,          // Retraso de repetición
    pub repeat_rate: u32,           // Tasa de repetición
}

// Variables globales
static mut KEYBOARD_BUFFER: KeyboardBuffer = KeyboardBuffer {
    events: [KeyboardEvent { key_code: 0, character: 0, flags: 0, pressed: false, timestamp: 0 }; KEYBOARD_BUFFER_SIZE],
    head: 0,
    tail: 0,
    count: 0,
};

static mut KEYBOARD_STATE: KeyboardState = KeyboardState {
    flags: 0,
    last_key: 0,
    repeat_count: 0,
    repeat_delay: 500, // 500ms
    repeat_rate: 50,   // 50ms
};

// Tabla de caracteres (sin shift)
static KEYBOARD_CHARS_NORMAL: [u8; 128] = [
    0,   0,   b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 0,   0,
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', 0,   0,   b'a', b's',
    b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', 0,   0,   0,   b'\\', b'z', b'x', b'c',
    b'v', b'b', b'n', b'm', b',', b'.', b'/', 0,   0,   0,   b' ', 0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
];

// Tabla de caracteres (con shift)
static KEYBOARD_CHARS_SHIFT: [u8; 128] = [
    0,   0,   b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 0,   0,
    b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', 0,   0,   b'A', b'S',
    b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', 0,   0,   0,   b'|', b'Z', b'X', b'C',
    b'V', b'B', b'N', b'M', b'<', b'>', b'?', 0,   0,   0,   b' ', 0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
    0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
];

// Funciones principales de teclado

/// Keyboard_Initialize - Inicializar teclado
#[no_mangle]
pub extern "C" fn Keyboard_Initialize() -> KeyboardResult {
    unsafe {
        // Limpiar buffer
        KEYBOARD_BUFFER.head = 0;
        KEYBOARD_BUFFER.tail = 0;
        KEYBOARD_BUFFER.count = 0;
        
        // Inicializar estado
        KEYBOARD_STATE.flags = 0;
        KEYBOARD_STATE.last_key = 0;
        KEYBOARD_STATE.repeat_count = 0;
        
        // Configurar teclado PS/2
        Keyboard_PS2_Initialize();
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_PS2_Initialize - Inicializar teclado PS/2
#[no_mangle]
pub extern "C" fn Keyboard_PS2_Initialize() -> KeyboardResult {
    unsafe {
        // Habilitar teclado
        core::arch::asm!(
            "out dx, al",
            in("dx") KEYBOARD_PS2_COMMAND_PORT,
            in("al") 0xAEu8,
            options(nostack)
        );
        
        // Limpiar buffer de entrada
        Keyboard_PS2_ClearBuffer();
        
        // Configurar LED
        Keyboard_PS2_SetLEDs(0);
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_PS2_ClearBuffer - Limpiar buffer PS/2
#[no_mangle]
pub extern "C" fn Keyboard_PS2_ClearBuffer() -> KeyboardResult {
    unsafe {
        // Leer y descartar todos los datos del buffer
        for _ in 0..256 {
            let status: u8;
            core::arch::asm!(
                "in al, dx",
                out("al") status,
                in("dx") KEYBOARD_PS2_STATUS_PORT,
                options(nostack)
            );
            
            if (status & 0x01) == 0 {
                break; // No hay datos
            }
            
            let _data: u8;
            core::arch::asm!(
                "in al, dx",
                out("al") _data,
                in("dx") KEYBOARD_PS2_DATA_PORT,
                options(nostack)
            );
        }
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_PS2_SetLEDs - Establecer LEDs del teclado
#[no_mangle]
pub extern "C" fn Keyboard_PS2_SetLEDs(leds: u8) -> KeyboardResult {
    unsafe {
        // Enviar comando para establecer LEDs
        Keyboard_PS2_SendCommand(0xED);
        Keyboard_PS2_WaitForAck();
        Keyboard_PS2_SendData(leds);
        Keyboard_PS2_WaitForAck();
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_PS2_SendCommand - Enviar comando al teclado
#[no_mangle]
pub extern "C" fn Keyboard_PS2_SendCommand(command: u8) -> KeyboardResult {
    unsafe {
        // Esperar a que el buffer de entrada esté vacío
        Keyboard_PS2_WaitForInputEmpty();
        
        // Enviar comando
        core::arch::asm!(
            "out dx, al",
            in("dx") KEYBOARD_PS2_DATA_PORT,
            in("al") command,
            options(nostack)
        );
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_PS2_SendData - Enviar datos al teclado
#[no_mangle]
pub extern "C" fn Keyboard_PS2_SendData(data: u8) -> KeyboardResult {
    unsafe {
        // Esperar a que el buffer de entrada esté vacío
        Keyboard_PS2_WaitForInputEmpty();
        
        // Enviar datos
        core::arch::asm!(
            "out dx, al",
            in("dx") KEYBOARD_PS2_DATA_PORT,
            in("al") data,
            options(nostack)
        );
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_PS2_WaitForInputEmpty - Esperar a que el buffer de entrada esté vacío
#[no_mangle]
pub extern "C" fn Keyboard_PS2_WaitForInputEmpty() -> KeyboardResult {
    unsafe {
        for _ in 0..10000 {
            let status: u8;
            core::arch::asm!(
                "in al, dx",
                out("al") status,
                in("dx") KEYBOARD_PS2_STATUS_PORT,
                options(nostack)
            );
            
            if (status & 0x02) == 0 {
                return KEYBOARD_SUCCESS;
            }
        }
    }
    
    KEYBOARD_ERROR_HARDWARE_ERROR
}

/// Keyboard_PS2_WaitForAck - Esperar ACK del teclado
#[no_mangle]
pub extern "C" fn Keyboard_PS2_WaitForAck() -> KeyboardResult {
    unsafe {
        for _ in 0..10000 {
            let status: u8;
            core::arch::asm!(
                "in al, dx",
                out("al") status,
                in("dx") KEYBOARD_PS2_STATUS_PORT,
                options(nostack)
            );
            
            if (status & 0x01) != 0 {
                let data: u8;
                core::arch::asm!(
                    "in al, dx",
                    out("al") data,
                    in("dx") KEYBOARD_PS2_DATA_PORT,
                    options(nostack)
                );
                
                if data == 0xFA { // ACK
                    return KEYBOARD_SUCCESS;
                }
            }
        }
    }
    
    KEYBOARD_ERROR_HARDWARE_ERROR
}

/// Keyboard_ReadKey - Leer tecla
#[no_mangle]
pub extern "C" fn Keyboard_ReadKey(event: *mut KeyboardEvent) -> KeyboardResult {
    unsafe {
        if KEYBOARD_BUFFER.count == 0 {
            return KEYBOARD_ERROR_BUFFER_FULL;
        }
        
        *event = KEYBOARD_BUFFER.events[KEYBOARD_BUFFER.tail];
        KEYBOARD_BUFFER.tail = (KEYBOARD_BUFFER.tail + 1) % KEYBOARD_BUFFER_SIZE;
        KEYBOARD_BUFFER.count -= 1;
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_ReadChar - Leer carácter
#[no_mangle]
pub extern "C" fn Keyboard_ReadChar(character: *mut u8) -> KeyboardResult {
    unsafe {
        let mut event = KeyboardEvent { key_code: 0, character: 0, flags: 0, pressed: false, timestamp: 0 };
        
        loop {
            if Keyboard_ReadKey(&mut event) != KEYBOARD_SUCCESS {
                return KEYBOARD_ERROR_BUFFER_FULL;
            }
            
            if event.pressed && event.character != 0 {
                *character = event.character;
                return KEYBOARD_SUCCESS;
            }
        }
    }
}

/// Keyboard_GetKeyState - Obtener estado de tecla
#[no_mangle]
pub extern "C" fn Keyboard_GetKeyState(key_code: u8) -> bool {
    unsafe {
        match key_code {
            KEY_LEFT_SHIFT | KEY_RIGHT_SHIFT => (KEYBOARD_STATE.flags & KEYBOARD_FLAG_SHIFT) != 0,
            KEY_LEFT_CTRL => (KEYBOARD_STATE.flags & KEYBOARD_FLAG_CTRL) != 0,
            KEY_LEFT_ALT => (KEYBOARD_STATE.flags & KEYBOARD_FLAG_ALT) != 0,
            KEY_CAPS_LOCK => (KEYBOARD_STATE.flags & KEYBOARD_FLAG_CAPS_LOCK) != 0,
            KEY_NUM_LOCK => (KEYBOARD_STATE.flags & KEYBOARD_FLAG_NUM_LOCK) != 0,
            KEY_SCROLL_LOCK => (KEYBOARD_STATE.flags & KEYBOARD_FLAG_SCROLL_LOCK) != 0,
            _ => false,
        }
    }
}

/// Keyboard_GetFlags - Obtener flags de estado
#[no_mangle]
pub extern "C" fn Keyboard_GetFlags() -> u8 {
    unsafe {
        KEYBOARD_STATE.flags
    }
}

/// Keyboard_SetFlags - Establecer flags de estado
#[no_mangle]
pub extern "C" fn Keyboard_SetFlags(flags: u8) -> KeyboardResult {
    unsafe {
        KEYBOARD_STATE.flags = flags;
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_ProcessScancode - Procesar código de escaneo
#[no_mangle]
pub extern "C" fn Keyboard_ProcessScancode(scancode: u8) -> KeyboardResult {
    unsafe {
        let mut event = KeyboardEvent {
            key_code: 0,
            character: 0,
            flags: KEYBOARD_STATE.flags,
            pressed: false,
            timestamp: 0, // TODO: Implementar timestamp
        };
        
        // Determinar si es tecla presionada o liberada
        if scancode & 0x80 != 0 {
            // Tecla liberada
            event.pressed = false;
            event.key_code = scancode & 0x7F;
        } else {
            // Tecla presionada
            event.pressed = true;
            event.key_code = scancode;
        }
        
        // Procesar teclas especiales
        match event.key_code {
            KEY_LEFT_SHIFT | KEY_RIGHT_SHIFT => {
                if event.pressed {
                    KEYBOARD_STATE.flags |= KEYBOARD_FLAG_SHIFT;
                } else {
                    KEYBOARD_STATE.flags &= !KEYBOARD_FLAG_SHIFT;
                }
            }
            KEY_LEFT_CTRL => {
                if event.pressed {
                    KEYBOARD_STATE.flags |= KEYBOARD_FLAG_CTRL;
                } else {
                    KEYBOARD_STATE.flags &= !KEYBOARD_FLAG_CTRL;
                }
            }
            KEY_LEFT_ALT => {
                if event.pressed {
                    KEYBOARD_STATE.flags |= KEYBOARD_FLAG_ALT;
                } else {
                    KEYBOARD_STATE.flags &= !KEYBOARD_FLAG_ALT;
                }
            }
            KEY_CAPS_LOCK => {
                if event.pressed {
                    KEYBOARD_STATE.flags ^= KEYBOARD_FLAG_CAPS_LOCK;
                }
            }
            KEY_NUM_LOCK => {
                if event.pressed {
                    KEYBOARD_STATE.flags ^= KEYBOARD_FLAG_NUM_LOCK;
                }
            }
            KEY_SCROLL_LOCK => {
                if event.pressed {
                    KEYBOARD_STATE.flags ^= KEYBOARD_FLAG_SCROLL_LOCK;
                }
            }
            _ => {
                // Procesar carácter normal
                if event.pressed && event.key_code < 128 {
                    let shift_pressed = (KEYBOARD_STATE.flags & KEYBOARD_FLAG_SHIFT) != 0;
                    let caps_lock = (KEYBOARD_STATE.flags & KEYBOARD_FLAG_CAPS_LOCK) != 0;
                    
                    let char_table = if shift_pressed ^ caps_lock {
                        &KEYBOARD_CHARS_SHIFT
                    } else {
                        &KEYBOARD_CHARS_NORMAL
                    };
                    
                    event.character = char_table[event.key_code as usize];
                }
            }
        }
        
        // Agregar evento al buffer
        Keyboard_AddEvent(&event);
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_AddEvent - Agregar evento al buffer
#[no_mangle]
pub extern "C" fn Keyboard_AddEvent(event: *const KeyboardEvent) -> KeyboardResult {
    unsafe {
        if KEYBOARD_BUFFER.count >= KEYBOARD_BUFFER_SIZE {
            return KEYBOARD_ERROR_BUFFER_FULL;
        }
        
        KEYBOARD_BUFFER.events[KEYBOARD_BUFFER.head] = *event;
        KEYBOARD_BUFFER.head = (KEYBOARD_BUFFER.head + 1) % KEYBOARD_BUFFER_SIZE;
        KEYBOARD_BUFFER.count += 1;
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_GetBufferCount - Obtener número de eventos en buffer
#[no_mangle]
pub extern "C" fn Keyboard_GetBufferCount() -> usize {
    unsafe {
        KEYBOARD_BUFFER.count
    }
}

/// Keyboard_ClearBuffer - Limpiar buffer
#[no_mangle]
pub extern "C" fn Keyboard_ClearBuffer() -> KeyboardResult {
    unsafe {
        KEYBOARD_BUFFER.head = 0;
        KEYBOARD_BUFFER.tail = 0;
        KEYBOARD_BUFFER.count = 0;
    }
    
    KEYBOARD_SUCCESS
}

/// Keyboard_Test - Test del driver de teclado
#[no_mangle]
pub extern "C" fn Keyboard_Test() -> KeyboardResult {
    unsafe {
        // Test básico del teclado
        Keyboard_PS2_SetLEDs(0x07); // Encender todos los LEDs
        
        // Esperar un poco
        for _ in 0..1000000 {
            core::arch::asm!("nop");
        }
        
        Keyboard_PS2_SetLEDs(0x00); // Apagar todos los LEDs
    }
    
    KEYBOARD_SUCCESS
}

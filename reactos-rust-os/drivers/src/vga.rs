//! VGA Driver - Implementación en Rust
//! 
//! Driver de VGA para ReactOS Rust OS
//! Soporte para modo texto y gráficos básicos

use core::ffi::{c_char, c_void};
use core::ptr;

// Tipos de datos
pub type VGAHandle = *mut c_void;
pub type VGAResult = i32;
pub type VGAError = i32;

// Constantes de éxito y error
pub const VGA_SUCCESS: VGAResult = 0;
pub const VGA_ERROR_INVALID_PARAM: VGAError = 0x80000001u32 as i32;
pub const VGA_ERROR_NOT_SUPPORTED: VGAError = 0x80000002u32 as i32;
pub const VGA_ERROR_HARDWARE_ERROR: VGAError = 0x80000003u32 as i32;

// Constantes de VGA
pub const VGA_WIDTH: u32 = 80;
pub const VGA_HEIGHT: u32 = 25;
pub const VGA_BUFFER_SIZE: usize = VGA_WIDTH as usize * VGA_HEIGHT as usize;
pub const VGA_BUFFER_ADDRESS: u32 = 0xB8000;

// Colores de VGA
pub const VGA_COLOR_BLACK: u8 = 0x00;
pub const VGA_COLOR_BLUE: u8 = 0x01;
pub const VGA_COLOR_GREEN: u8 = 0x02;
pub const VGA_COLOR_CYAN: u8 = 0x03;
pub const VGA_COLOR_RED: u8 = 0x04;
pub const VGA_COLOR_MAGENTA: u8 = 0x05;
pub const VGA_COLOR_BROWN: u8 = 0x06;
pub const VGA_COLOR_LIGHT_GRAY: u8 = 0x07;
pub const VGA_COLOR_DARK_GRAY: u8 = 0x08;
pub const VGA_COLOR_LIGHT_BLUE: u8 = 0x09;
pub const VGA_COLOR_LIGHT_GREEN: u8 = 0x0A;
pub const VGA_COLOR_LIGHT_CYAN: u8 = 0x0B;
pub const VGA_COLOR_LIGHT_RED: u8 = 0x0C;
pub const VGA_COLOR_LIGHT_MAGENTA: u8 = 0x0D;
pub const VGA_COLOR_YELLOW: u8 = 0x0E;
pub const VGA_COLOR_WHITE: u8 = 0x0F;

// Estructuras

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VGAColor {
    pub foreground: u8,              // Color de primer plano
    pub background: u8,              // Color de fondo
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct VGACharacter {
    pub character: u8,               // Carácter
    pub color: VGAColor,             // Color
}

#[repr(C, packed)]
pub struct VGABuffer {
    pub characters: [VGACharacter; VGA_BUFFER_SIZE], // Buffer de caracteres
    pub cursor_x: u32,               // Posición X del cursor
    pub cursor_y: u32,               // Posición Y del cursor
    pub current_color: VGAColor,     // Color actual
}

#[repr(C, packed)]
pub struct VGAMode {
    pub mode: u32,                   // Modo de VGA
    pub width: u32,                  // Ancho
    pub height: u32,                 // Alto
    pub bpp: u32,                    // Bits por píxel
    pub buffer_address: u32,         // Dirección del buffer
}

// Variables globales
static mut VGA_BUFFER: VGABuffer = VGABuffer {
    characters: [VGACharacter { character: 0, color: VGAColor { foreground: 0, background: 0 } }; VGA_BUFFER_SIZE],
    cursor_x: 0,
    cursor_y: 0,
    current_color: VGAColor { foreground: VGA_COLOR_WHITE, background: VGA_COLOR_BLACK },
};

// Funciones principales de VGA

/// VGA_Initialize - Inicializar VGA
#[no_mangle]
pub extern "C" fn VGA_Initialize() -> VGAResult {
    unsafe {
        // Limpiar pantalla
        VGA_ClearScreen();
        
        // Configurar cursor
        VGA_SetCursor(0, 0);
        
        // Configurar color por defecto
        VGA_SetColor(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
    }
    
    VGA_SUCCESS
}

/// VGA_ClearScreen - Limpiar pantalla
#[no_mangle]
pub extern "C" fn VGA_ClearScreen() -> VGAResult {
    unsafe {
        for i in 0..VGA_BUFFER_SIZE {
            VGA_BUFFER.characters[i].character = b' ';
            VGA_BUFFER.characters[i].color = VGA_BUFFER.current_color;
        }
        
        VGA_BUFFER.cursor_x = 0;
        VGA_BUFFER.cursor_y = 0;
        
        VGA_UpdateDisplay();
    }
    
    VGA_SUCCESS
}

/// VGA_SetColor - Establecer color
#[no_mangle]
pub extern "C" fn VGA_SetColor(foreground: u8, background: u8) -> VGAResult {
    unsafe {
        VGA_BUFFER.current_color.foreground = foreground;
        VGA_BUFFER.current_color.background = background;
    }
    
    VGA_SUCCESS
}

/// VGA_SetCursor - Establecer posición del cursor
#[no_mangle]
pub extern "C" fn VGA_SetCursor(x: u32, y: u32) -> VGAResult {
    if x >= VGA_WIDTH || y >= VGA_HEIGHT {
        return VGA_ERROR_INVALID_PARAM;
    }
    
    unsafe {
        VGA_BUFFER.cursor_x = x;
        VGA_BUFFER.cursor_y = y;
        
        VGA_UpdateCursor();
    }
    
    VGA_SUCCESS
}

/// VGA_GetCursor - Obtener posición del cursor
#[no_mangle]
pub extern "C" fn VGA_GetCursor(x: *mut u32, y: *mut u32) -> VGAResult {
    unsafe {
        *x = VGA_BUFFER.cursor_x;
        *y = VGA_BUFFER.cursor_y;
    }
    
    VGA_SUCCESS
}

/// VGA_PutChar - Escribir carácter
#[no_mangle]
pub extern "C" fn VGA_PutChar(character: u8) -> VGAResult {
    unsafe {
        if character == b'\n' {
            VGA_NewLine();
        } else if character == b'\r' {
            VGA_BUFFER.cursor_x = 0;
        } else if character == b'\t' {
            VGA_Tab();
        } else if character == b'\x08' { // Backspace
            VGA_Backspace();
        } else {
            let index = (VGA_BUFFER.cursor_y * VGA_WIDTH + VGA_BUFFER.cursor_x) as usize;
            if index < VGA_BUFFER_SIZE {
                VGA_BUFFER.characters[index].character = character;
                VGA_BUFFER.characters[index].color = VGA_BUFFER.current_color;
                
                VGA_BUFFER.cursor_x += 1;
                if VGA_BUFFER.cursor_x >= VGA_WIDTH {
                    VGA_NewLine();
                }
            }
        }
        
        VGA_UpdateDisplay();
    }
    
    VGA_SUCCESS
}

/// VGA_PutString - Escribir cadena
#[no_mangle]
pub extern "C" fn VGA_PutString(string: *const c_char) -> VGAResult {
    unsafe {
        let mut i = 0;
        loop {
            let character = *string.add(i);
            if character == 0 {
                break;
            }
            
            VGA_PutChar(character as u8);
            i += 1;
        }
    }
    
    VGA_SUCCESS
}

/// VGA_Printf - Imprimir con formato
#[no_mangle]
pub extern "C" fn VGA_Printf(format: *const c_char) -> VGAResult {
    // Implementación básica de printf
    // Por ahora, solo imprimir la cadena de formato
    VGA_PutString(format)
}

/// VGA_ScrollUp - Desplazar hacia arriba
#[no_mangle]
pub extern "C" fn VGA_ScrollUp() -> VGAResult {
    unsafe {
        // Mover todas las líneas una posición hacia arriba
        for y in 0..(VGA_HEIGHT - 1) {
            for x in 0..VGA_WIDTH {
                let src_index = ((y + 1) * VGA_WIDTH + x) as usize;
                let dst_index = (y * VGA_WIDTH + x) as usize;
                
                if src_index < VGA_BUFFER_SIZE && dst_index < VGA_BUFFER_SIZE {
                    VGA_BUFFER.characters[dst_index] = VGA_BUFFER.characters[src_index];
                }
            }
        }
        
        // Limpiar la última línea
        let last_line_start = ((VGA_HEIGHT - 1) * VGA_WIDTH) as usize;
        for i in last_line_start..VGA_BUFFER_SIZE {
            VGA_BUFFER.characters[i].character = b' ';
            VGA_BUFFER.characters[i].color = VGA_BUFFER.current_color;
        }
        
        VGA_UpdateDisplay();
    }
    
    VGA_SUCCESS
}

/// VGA_NewLine - Nueva línea
#[no_mangle]
pub extern "C" fn VGA_NewLine() -> VGAResult {
    unsafe {
        VGA_BUFFER.cursor_x = 0;
        VGA_BUFFER.cursor_y += 1;
        
        if VGA_BUFFER.cursor_y >= VGA_HEIGHT {
            VGA_ScrollUp();
            VGA_BUFFER.cursor_y = VGA_HEIGHT - 1;
        }
        
        VGA_UpdateCursor();
    }
    
    VGA_SUCCESS
}

/// VGA_Tab - Tabulación
#[no_mangle]
pub extern "C" fn VGA_Tab() -> VGAResult {
    unsafe {
        VGA_BUFFER.cursor_x = (VGA_BUFFER.cursor_x + 8) & !7;
        if VGA_BUFFER.cursor_x >= VGA_WIDTH {
            VGA_NewLine();
        }
    }
    
    VGA_SUCCESS
}

/// VGA_Backspace - Retroceso
#[no_mangle]
pub extern "C" fn VGA_Backspace() -> VGAResult {
    unsafe {
        if VGA_BUFFER.cursor_x > 0 {
            VGA_BUFFER.cursor_x -= 1;
        } else if VGA_BUFFER.cursor_y > 0 {
            VGA_BUFFER.cursor_y -= 1;
            VGA_BUFFER.cursor_x = VGA_WIDTH - 1;
        }
        
        let index = (VGA_BUFFER.cursor_y * VGA_WIDTH + VGA_BUFFER.cursor_x) as usize;
        if index < VGA_BUFFER_SIZE {
            VGA_BUFFER.characters[index].character = b' ';
            VGA_BUFFER.characters[index].color = VGA_BUFFER.current_color;
        }
        
        VGA_UpdateDisplay();
    }
    
    VGA_SUCCESS
}

/// VGA_UpdateDisplay - Actualizar pantalla
#[no_mangle]
pub extern "C" fn VGA_UpdateDisplay() -> VGAResult {
    unsafe {
        let vga_memory = VGA_BUFFER_ADDRESS as *mut u16;
        
        for i in 0..VGA_BUFFER_SIZE {
            let character = VGA_BUFFER.characters[i].character as u16;
            let color = ((VGA_BUFFER.characters[i].color.background as u16) << 4) | 
                       (VGA_BUFFER.characters[i].color.foreground as u16);
            let vga_char = character | (color << 8);
            
            *vga_memory.add(i) = vga_char;
        }
    }
    
    VGA_SUCCESS
}

/// VGA_UpdateCursor - Actualizar cursor
#[no_mangle]
pub extern "C" fn VGA_UpdateCursor() -> VGAResult {
    unsafe {
        let cursor_position = VGA_BUFFER.cursor_y * VGA_WIDTH + VGA_BUFFER.cursor_x;
        
        // Enviar comando al puerto de VGA
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3D4u16,
            in("al") 0x0Fu8,
            options(nostack)
        );
        
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3D5u16,
            in("al") (cursor_position & 0xFF) as u8,
            options(nostack)
        );
        
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3D4u16,
            in("al") 0x0Eu8,
            options(nostack)
        );
        
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3D5u16,
            in("al") ((cursor_position >> 8) & 0xFF) as u8,
            options(nostack)
        );
    }
    
    VGA_SUCCESS
}

/// VGA_GetMode - Obtener modo actual
#[no_mangle]
pub extern "C" fn VGA_GetMode(mode: *mut VGAMode) -> VGAResult {
    unsafe {
        (*mode).mode = 0x03; // Modo texto 80x25
        (*mode).width = VGA_WIDTH;
        (*mode).height = VGA_HEIGHT;
        (*mode).bpp = 4; // 4 bits por píxel (16 colores)
        (*mode).buffer_address = VGA_BUFFER_ADDRESS;
    }
    
    VGA_SUCCESS
}

/// VGA_SetMode - Establecer modo
#[no_mangle]
pub extern "C" fn VGA_SetMode(mode: u32) -> VGAResult {
    // Por ahora, solo soportamos modo texto 80x25
    if mode != 0x03 {
        return VGA_ERROR_NOT_SUPPORTED;
    }
    
    VGA_SUCCESS
}

/// VGA_GetWidth - Obtener ancho
#[no_mangle]
pub extern "C" fn VGA_GetWidth() -> u32 {
    VGA_WIDTH
}

/// VGA_GetHeight - Obtener alto
#[no_mangle]
pub extern "C" fn VGA_GetHeight() -> u32 {
    VGA_HEIGHT
}

/// VGA_GetBufferSize - Obtener tamaño del buffer
#[no_mangle]
pub extern "C" fn VGA_GetBufferSize() -> usize {
    VGA_BUFFER_SIZE
}

/// VGA_GetBufferAddress - Obtener dirección del buffer
#[no_mangle]
pub extern "C" fn VGA_GetBufferAddress() -> u32 {
    VGA_BUFFER_ADDRESS
}

/// VGA_EnableCursor - Habilitar cursor
#[no_mangle]
pub extern "C" fn VGA_EnableCursor() -> VGAResult {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3D4u16,
            in("al") 0x0Au8,
            options(nostack)
        );
        
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3D5u16,
            in("al") 0x20u8,
            options(nostack)
        );
    }
    
    VGA_SUCCESS
}

/// VGA_DisableCursor - Deshabilitar cursor
#[no_mangle]
pub extern "C" fn VGA_DisableCursor() -> VGAResult {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3D4u16,
            in("al") 0x0Au8,
            options(nostack)
        );
        
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3D5u16,
            in("al") 0x20u8,
            options(nostack)
        );
    }
    
    VGA_SUCCESS
}

/// VGA_Test - Test del driver VGA
#[no_mangle]
pub extern "C" fn VGA_Test() -> VGAResult {
    unsafe {
        VGA_ClearScreen();
        VGA_SetColor(VGA_COLOR_GREEN, VGA_COLOR_BLACK);
        VGA_PutString(b"VGA Driver Test\0".as_ptr() as *const c_char);
        VGA_NewLine();
        
        VGA_SetColor(VGA_COLOR_YELLOW, VGA_COLOR_BLACK);
        VGA_PutString(b"Testing colors:\0".as_ptr() as *const c_char);
        VGA_NewLine();
        
        for i in 0..16 {
            VGA_SetColor(i, VGA_COLOR_BLACK);
            VGA_PutChar(b'X');
        }
        
        VGA_NewLine();
        VGA_SetColor(VGA_COLOR_WHITE, VGA_COLOR_BLACK);
        VGA_PutString(b"VGA Driver Test Complete!\0".as_ptr() as *const c_char);
    }
    
    VGA_SUCCESS
}

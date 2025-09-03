//! Módulo de thunks para WOW64
//! 
//! Implementa los thunks que permiten a las aplicaciones 32-bit
//! llamar a las APIs 64-bit del sistema

use core::ptr;
use kernel::prelude::*;

/// Tabla de thunks para APIs
pub struct ThunkTable {
    /// Thunks para kernel32.dll
    kernel32_thunks: Kernel32Thunks,
    /// Thunks para ntdll.dll
    ntdll_thunks: NtdllThunks,
    /// Thunks para user32.dll
    user32_thunks: User32Thunks,
    /// Thunks para gdi32.dll
    gdi32_thunks: Gdi32Thunks,
}

/// Thunks para kernel32.dll
pub struct Kernel32Thunks {
    /// Thunk para CreateFileA
    pub create_file_a: Thunk,
    /// Thunk para ReadFile
    pub read_file: Thunk,
    /// Thunk para WriteFile
    pub write_file: Thunk,
    /// Thunk para CloseHandle
    pub close_handle: Thunk,
    /// Thunk para GetLastError
    pub get_last_error: Thunk,
    /// Thunk para SetLastError
    pub set_last_error: Thunk,
}

/// Thunks para ntdll.dll
pub struct NtdllThunks {
    /// Thunk para NtCreateFile
    pub nt_create_file: Thunk,
    /// Thunk para NtReadFile
    pub nt_read_file: Thunk,
    /// Thunk para NtWriteFile
    pub nt_write_file: Thunk,
    /// Thunk para NtClose
    pub nt_close: Thunk,
}

/// Thunks para user32.dll
pub struct User32Thunks {
    /// Thunk para CreateWindowExA
    pub create_window_ex_a: Thunk,
    /// Thunk para DefWindowProcA
    pub def_window_proc_a: Thunk,
    /// Thunk para GetMessageA
    pub get_message_a: Thunk,
    /// Thunk para DispatchMessageA
    pub dispatch_message_a: Thunk,
}

/// Thunks para gdi32.dll
pub struct Gdi32Thunks {
    /// Thunk para CreateDC
    pub create_dc: Thunk,
    /// Thunk para DeleteDC
    pub delete_dc: Thunk,
    /// Thunk para BitBlt
    pub bit_blt: Thunk,
}

/// Thunk individual
pub struct Thunk {
    /// Dirección 32-bit
    pub addr_32: u32,
    /// Dirección 64-bit
    pub addr_64: u64,
    /// Código del thunk
    pub code: Vec<u8>,
}

impl ThunkTable {
    /// Crear nueva tabla de thunks
    pub fn new() -> Result<Self> {
        let kernel32_thunks = Kernel32Thunks::new()?;
        let ntdll_thunks = NtdllThunks::new()?;
        let user32_thunks = User32Thunks::new()?;
        let gdi32_thunks = Gdi32Thunks::new()?;
        
        Ok(Self {
            kernel32_thunks,
            ntdll_thunks,
            user32_thunks,
            gdi32_thunks,
        })
    }
    
    /// Obtener thunk por dirección 32-bit
    pub fn get_thunk(&self, addr_32: u32) -> Option<&Thunk> {
        // Buscar en kernel32
        if let Some(thunk) = self.kernel32_thunks.get_thunk(addr_32) {
            return Some(thunk);
        }
        
        // Buscar en ntdll
        if let Some(thunk) = self.ntdll_thunks.get_thunk(addr_32) {
            return Some(thunk);
        }
        
        // Buscar en user32
        if let Some(thunk) = self.user32_thunks.get_thunk(addr_32) {
            return Some(thunk);
        }
        
        // Buscar en gdi32
        if let Some(thunk) = self.gdi32_thunks.get_thunk(addr_32) {
            return Some(thunk);
        }
        
        None
    }
}

impl Kernel32Thunks {
    /// Crear nuevos thunks para kernel32
    pub fn new() -> Result<Self> {
        let create_file_a = Thunk::new(0x7C810000, 0x7FFE0000)?;
        let read_file = Thunk::new(0x7C810010, 0x7FFE0010)?;
        let write_file = Thunk::new(0x7C810020, 0x7FFE0020)?;
        let close_handle = Thunk::new(0x7C810030, 0x7FFE0030)?;
        let get_last_error = Thunk::new(0x7C810040, 0x7FFE0040)?;
        let set_last_error = Thunk::new(0x7C810050, 0x7FFE0050)?;
        
        Ok(Self {
            create_file_a,
            read_file,
            write_file,
            close_handle,
            get_last_error,
            set_last_error,
        })
    }
    
    /// Obtener thunk por dirección
    pub fn get_thunk(&self, addr_32: u32) -> Option<&Thunk> {
        match addr_32 {
            0x7C810000 => Some(&self.create_file_a),
            0x7C810010 => Some(&self.read_file),
            0x7C810020 => Some(&self.write_file),
            0x7C810030 => Some(&self.close_handle),
            0x7C810040 => Some(&self.get_last_error),
            0x7C810050 => Some(&self.set_last_error),
            _ => None,
        }
    }
}

impl NtdllThunks {
    /// Crear nuevos thunks para ntdll
    pub fn new() -> Result<Self> {
        let nt_create_file = Thunk::new(0x7C820000, 0x7FFE0100)?;
        let nt_read_file = Thunk::new(0x7C820010, 0x7FFE0110)?;
        let nt_write_file = Thunk::new(0x7C820020, 0x7FFE0120)?;
        let nt_close = Thunk::new(0x7C820030, 0x7FFE0130)?;
        
        Ok(Self {
            nt_create_file,
            nt_read_file,
            nt_write_file,
            nt_close,
        })
    }
    
    /// Obtener thunk por dirección
    pub fn get_thunk(&self, addr_32: u32) -> Option<&Thunk> {
        match addr_32 {
            0x7C820000 => Some(&self.nt_create_file),
            0x7C820010 => Some(&self.nt_read_file),
            0x7C820020 => Some(&self.nt_write_file),
            0x7C820030 => Some(&self.nt_close),
            _ => None,
        }
    }
}

impl User32Thunks {
    /// Crear nuevos thunks para user32
    pub fn new() -> Result<Self> {
        let create_window_ex_a = Thunk::new(0x7C830000, 0x7FFE0200)?;
        let def_window_proc_a = Thunk::new(0x7C830010, 0x7FFE0210)?;
        let get_message_a = Thunk::new(0x7C830020, 0x7FFE0220)?;
        let dispatch_message_a = Thunk::new(0x7C830030, 0x7FFE0230)?;
        
        Ok(Self {
            create_window_ex_a,
            def_window_proc_a,
            get_message_a,
            dispatch_message_a,
        })
    }
    
    /// Obtener thunk por dirección
    pub fn get_thunk(&self, addr_32: u32) -> Option<&Thunk> {
        match addr_32 {
            0x7C830000 => Some(&self.create_window_ex_a),
            0x7C830010 => Some(&self.def_window_proc_a),
            0x7C830020 => Some(&self.get_message_a),
            0x7C830030 => Some(&self.dispatch_message_a),
            _ => None,
        }
    }
}

impl Gdi32Thunks {
    /// Crear nuevos thunks para gdi32
    pub fn new() -> Result<Self> {
        let create_dc = Thunk::new(0x7C840000, 0x7FFE0300)?;
        let delete_dc = Thunk::new(0x7C840010, 0x7FFE0310)?;
        let bit_blt = Thunk::new(0x7C840020, 0x7FFE0320)?;
        
        Ok(Self {
            create_dc,
            delete_dc,
            bit_blt,
        })
    }
    
    /// Obtener thunk por dirección
    pub fn get_thunk(&self, addr_32: u32) -> Option<&Thunk> {
        match addr_32 {
            0x7C840000 => Some(&self.create_dc),
            0x7C840010 => Some(&self.delete_dc),
            0x7C840020 => Some(&self.bit_blt),
            _ => None,
        }
    }
}

impl Thunk {
    /// Crear nuevo thunk
    pub fn new(addr_32: u32, addr_64: u64) -> Result<Self> {
        let code = Self::generate_thunk_code(addr_64)?;
        
        Ok(Self {
            addr_32,
            addr_64,
            code,
        })
    }
    
    /// Generar código del thunk
    fn generate_thunk_code(addr_64: u64) -> Result<Vec<u8>> {
        let mut code = Vec::new();
        
        // Código del thunk en ensamblador
        // 1. Guardar registros 32-bit
        code.extend_from_slice(&[0x50, 0x51, 0x52, 0x53]); // push eax, ecx, edx, ebx
        
        // 2. Convertir parámetros 32-bit a 64-bit
        code.extend_from_slice(&[0x48, 0x31, 0xC0]); // xor rax, rax
        code.extend_from_slice(&[0x48, 0x31, 0xC9]); // xor rcx, rcx
        code.extend_from_slice(&[0x48, 0x31, 0xD2]); // xor rdx, rdx
        code.extend_from_slice(&[0x48, 0x31, 0xDB]); // xor rbx, rbx
        
        // 3. Llamar a la función 64-bit
        code.push(0x48); // REX.W
        code.push(0xB8); // mov rax, imm64
        code.extend_from_slice(&addr_64.to_le_bytes());
        code.push(0xFF); // call rax
        code.push(0xD0);
        
        // 4. Convertir resultado 64-bit a 32-bit
        code.extend_from_slice(&[0x48, 0x89, 0xC0]); // mov rax, rax (truncar a 32-bit)
        
        // 5. Restaurar registros 32-bit
        code.extend_from_slice(&[0x5B, 0x5A, 0x59, 0x58]); // pop ebx, edx, ecx, eax
        
        // 6. Retornar
        code.push(0xC3); // ret
        
        Ok(code)
    }
    
    /// Ejecutar thunk
    pub fn execute(&self, args: &[u32]) -> Result<u32> {
        // Ejecutar código del thunk
        unsafe {
            let code_ptr = self.code.as_ptr();
            let func: extern "C" fn() -> u32 = core::mem::transmute(code_ptr);
            Ok(func())
        }
    }
}

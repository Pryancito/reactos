//! ReactOS Rust Kernel - Memory Management
//!
//! Sistema de gestión de memoria del kernel.

use core::arch::asm;

/// Estructura para representar una página de memoria
#[derive(Debug, Clone, Copy)]
pub struct MemoryPage {
    pub address: *mut u8,
    pub size: usize,
    pub allocated: bool,
}

/// Administrador de memoria del kernel
pub struct MemoryManager {
    pub pages: [MemoryPage; 1024], // 1024 páginas de 4KB cada una
    pub total_pages: usize,
    pub free_pages: usize,
}

impl MemoryManager {
    /// Crear un nuevo administrador de memoria
    pub fn new() -> Self {
        Self {
            pages: [MemoryPage {
                address: core::ptr::null_mut(),
                size: 4096, // 4KB por página
                allocated: false,
            }; 1024],
            total_pages: 1024,
            free_pages: 1024,
        }
    }
    
    /// Inicializar el administrador de memoria
    pub fn init(&mut self) {
        // Inicializar páginas de memoria
        for i in 0..self.total_pages {
            self.pages[i].address = (0x100000 + (i * 4096)) as *mut u8;
            self.pages[i].size = 4096;
            self.pages[i].allocated = false;
        }
        
        // Marcar las primeras páginas como ocupadas (kernel)
        for i in 0..16 {
            self.pages[i].allocated = true;
            self.free_pages -= 1;
        }
    }
    
    /// Asignar una página de memoria
    pub fn allocate_page(&mut self) -> Option<*mut u8> {
        if self.free_pages == 0 {
            return None;
        }
        
        for i in 0..self.total_pages {
            if !self.pages[i].allocated {
                self.pages[i].allocated = true;
                self.free_pages -= 1;
                return Some(self.pages[i].address);
            }
        }
        
        None
    }
    
    /// Liberar una página de memoria
    pub fn free_page(&mut self, address: *mut u8) -> bool {
        for i in 0..self.total_pages {
            if self.pages[i].address == address && self.pages[i].allocated {
                self.pages[i].allocated = false;
                self.free_pages += 1;
                return true;
            }
        }
        false
    }
    
    /// Obtener estadísticas de memoria
    pub fn get_stats(&self) -> (usize, usize, usize) {
        (self.total_pages, self.free_pages, self.total_pages - self.free_pages)
    }
}

/// Instancia global del administrador de memoria
static mut MEMORY_MANAGER: Option<MemoryManager> = None;

/// Inicializar el sistema de memoria
pub fn init() {
    unsafe {
        MEMORY_MANAGER = Some(MemoryManager::new());
        if let Some(ref mut manager) = MEMORY_MANAGER {
            manager.init();
        }
    }
}

/// Asignar una página de memoria
pub fn allocate_page() -> Option<*mut u8> {
    unsafe {
        if let Some(ref mut manager) = MEMORY_MANAGER {
            manager.allocate_page()
        } else {
            None
        }
    }
}

/// Liberar una página de memoria
pub fn free_page(address: *mut u8) -> bool {
    unsafe {
        if let Some(ref mut manager) = MEMORY_MANAGER {
            manager.free_page(address)
        } else {
            false
        }
    }
}

/// Obtener estadísticas de memoria
pub fn get_memory_stats() -> (usize, usize, usize) {
    unsafe {
        if let Some(ref manager) = MEMORY_MANAGER {
            manager.get_stats()
        } else {
            (0, 0, 0)
        }
    }
}

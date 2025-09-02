//! Sistema de gestión de memoria para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Memory Manager básico
//! - Allocator personalizado para no_std
//! - Gestión de páginas de memoria
//! - Heap dinámico
//! - Información de memoria del sistema

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use alloc::string::String;
use alloc::format;
use spin::Mutex;

/// Tamaño de una página de memoria (4KB)
const PAGE_SIZE: usize = 4096;

/// Tamaño del heap del kernel (1MB)
const KERNEL_HEAP_SIZE: usize = 1024 * 1024;

/// Alineación mínima para asignaciones
const MIN_ALIGN: usize = 8;

/// Número máximo de páginas que podemos manejar
const MAX_PAGES: usize = 1024 * 1024; // 4GB / 4KB

/// Estado de una página de memoria
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageState {
    Free,       // Página libre
    Allocated,  // Página asignada
    Reserved,   // Página reservada
    Kernel,     // Página del kernel
}

/// Información de una página de memoria
#[derive(Debug, Clone, Copy)]
pub struct PageInfo {
    pub state: PageState,
    pub ref_count: u32,
    pub flags: u32,
}

impl PageInfo {
    pub fn new() -> Self {
        Self {
            state: PageState::Free,
            ref_count: 0,
            flags: 0,
        }
    }
}

/// Estructura del Memory Manager
pub struct MemoryManager {
    pages: [PageInfo; MAX_PAGES],
    heap_start: usize,
    heap_end: usize,
    heap_used: usize,
    total_memory: usize,
    free_memory: usize,
    allocated_memory: usize,
    page_count: usize,
    free_page_count: usize,
}

impl MemoryManager {
    /// Crear un nuevo Memory Manager
    pub fn new() -> Self {
        Self {
            pages: [PageInfo::new(); MAX_PAGES],
            heap_start: 0,
            heap_end: 0,
            heap_used: 0,
            total_memory: 0,
            free_memory: 0,
            allocated_memory: 0,
            page_count: 0,
            free_page_count: 0,
        }
    }

    /// Inicializar el Memory Manager
    pub fn init(&mut self, memory_start: usize, memory_size: usize) -> bool {
        // Configurar información básica de memoria
        self.total_memory = memory_size;
        self.free_memory = memory_size;
        self.page_count = memory_size / PAGE_SIZE;
        self.free_page_count = self.page_count;

        // Configurar heap del kernel
        self.heap_start = memory_start + (memory_size / 2); // Usar segunda mitad
        self.heap_end = self.heap_start + KERNEL_HEAP_SIZE;
        self.heap_used = 0;

        // Marcar páginas del kernel como reservadas
        let kernel_pages = (self.heap_start - memory_start) / PAGE_SIZE;
        for i in 0..kernel_pages {
            if i < MAX_PAGES {
                self.pages[i].state = PageState::Kernel;
                self.pages[i].ref_count = 1;
            }
        }

        // Marcar páginas del heap como libres
        let heap_start_page = (self.heap_start - memory_start) / PAGE_SIZE;
        let heap_pages = KERNEL_HEAP_SIZE / PAGE_SIZE;
        for i in heap_start_page..(heap_start_page + heap_pages) {
            if i < MAX_PAGES {
                self.pages[i].state = PageState::Free;
                self.pages[i].ref_count = 0;
            }
        }

        true
    }

    /// Asignar páginas de memoria
    pub fn allocate_pages(&mut self, count: usize) -> Option<usize> {
        if count == 0 {
            return None;
        }

        // Buscar páginas contiguas libres
        for i in 0..=(self.page_count - count) {
            let mut found = true;
            for j in 0..count {
                if i + j >= MAX_PAGES || self.pages[i + j].state != PageState::Free {
                    found = false;
                    break;
                }
            }

            if found {
                // Marcar páginas como asignadas
                for j in 0..count {
                    self.pages[i + j].state = PageState::Allocated;
                    self.pages[i + j].ref_count = 1;
                }

                self.free_page_count -= count;
                self.allocated_memory += count * PAGE_SIZE;
                self.free_memory -= count * PAGE_SIZE;

                return Some(i * PAGE_SIZE);
            }
        }

        None
    }

    /// Liberar páginas de memoria
    pub fn deallocate_pages(&mut self, address: usize, count: usize) -> bool {
        if count == 0 || address % PAGE_SIZE != 0 {
            return false;
        }

        let start_page = address / PAGE_SIZE;
        if start_page + count > MAX_PAGES {
            return false;
        }

        // Verificar que todas las páginas estén asignadas
        for i in 0..count {
            if self.pages[start_page + i].state != PageState::Allocated {
                return false;
            }
        }

        // Marcar páginas como libres
        for i in 0..count {
            self.pages[start_page + i].state = PageState::Free;
            self.pages[start_page + i].ref_count = 0;
        }

        self.free_page_count += count;
        self.allocated_memory -= count * PAGE_SIZE;
        self.free_memory += count * PAGE_SIZE;

        true
    }

    /// Obtener información de memoria
    pub fn get_memory_info(&self) -> String {
        format!(
            "Memoria: Total:{}MB Libre:{}MB Asignada:{}MB Páginas:{} Libres:{}",
            self.total_memory / (1024 * 1024),
            self.free_memory / (1024 * 1024),
            self.allocated_memory / (1024 * 1024),
            self.page_count,
            self.free_page_count
        )
    }

    /// Obtener estadísticas de memoria
    pub fn get_memory_stats(&self) -> String {
        let used_percent = if self.total_memory > 0 {
            (self.allocated_memory * 100) / self.total_memory
        } else {
            0
        };

        format!(
            "Memoria: {}% usada, {}KB heap usado, {} páginas libres",
            used_percent,
            self.heap_used / 1024,
            self.free_page_count
        )
    }

    /// Verificar si una dirección está en el heap
    pub fn is_heap_address(&self, addr: usize) -> bool {
        addr >= self.heap_start && addr < self.heap_end
    }

    /// Obtener tamaño del heap disponible
    pub fn get_heap_available(&self) -> usize {
        KERNEL_HEAP_SIZE - self.heap_used
    }
}

/// Allocator personalizado para el kernel
pub struct KernelAllocator {
    pub memory_manager: Mutex<MemoryManager>,
}

impl KernelAllocator {
    /// Crear un nuevo allocator
    pub const fn new() -> Self {
        Self {
            memory_manager: Mutex::new(MemoryManager {
                pages: [PageInfo {
                    state: PageState::Free,
                    ref_count: 0,
                    flags: 0,
                }; MAX_PAGES],
                heap_start: 0,
                heap_end: 0,
                heap_used: 0,
                total_memory: 0,
                free_memory: 0,
                allocated_memory: 0,
                page_count: 0,
                free_page_count: 0,
            }),
        }
    }

    /// Inicializar el allocator
    pub fn init(&self, memory_start: usize, memory_size: usize) -> bool {
        let mut mm = self.memory_manager.lock();
        mm.init(memory_start, memory_size)
    }

    /// Obtener información del allocator
    pub fn get_info(&self) -> String {
        let mm = self.memory_manager.lock();
        mm.get_memory_info()
    }

    /// Obtener estadísticas del allocator
    pub fn get_stats(&self) -> String {
        let mm = self.memory_manager.lock();
        mm.get_memory_stats()
    }
}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Alineación mínima
        let align = layout.align().max(MIN_ALIGN);
        let size = layout.size().max(align);

        // Alinear tamaño
        let aligned_size = (size + align - 1) & !(align - 1);

        let mut mm = self.memory_manager.lock();
        
        // Verificar si hay espacio en el heap
        if aligned_size > mm.get_heap_available() {
            return ptr::null_mut();
        }

        // Asignar en el heap (implementación simple)
        let heap_ptr = mm.heap_start + mm.heap_used;
        mm.heap_used += aligned_size;

        // Verificar que no excedamos el heap
        if heap_ptr + aligned_size > mm.heap_end {
            mm.heap_used -= aligned_size; // Revertir
            return ptr::null_mut();
        }

        heap_ptr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            return;
        }

        let align = layout.align().max(MIN_ALIGN);
        let size = layout.size().max(align);
        let _aligned_size = (size + align - 1) & !(align - 1);

        let mm = self.memory_manager.lock();
        let addr = ptr as usize;
        
        // Verificar que la dirección esté en el heap
        if mm.is_heap_address(addr) {
            // En una implementación real, aquí se marcaría como libre
            // Por simplicidad, no implementamos deallocación individual
            // El heap se reinicia cuando se llena
        }
    }
}

/// Instancia global del allocator
pub static KERNEL_ALLOCATOR: KernelAllocator = KernelAllocator::new();

/// Inicializar el sistema de memoria
pub fn init_memory_system() -> bool {
    // Configurar memoria básica (512MB como en el sistema)
    let memory_start = 0x100000; // 1MB (después del kernel)
    let memory_size = 512 * 1024 * 1024; // 512MB

    KERNEL_ALLOCATOR.init(memory_start, memory_size)
}

/// Obtener información del sistema de memoria
pub fn get_memory_info() -> String {
    KERNEL_ALLOCATOR.get_info()
}

/// Obtener estadísticas del sistema de memoria
pub fn get_memory_stats() -> String {
    KERNEL_ALLOCATOR.get_stats()
}

/// Verificar si el sistema de memoria está disponible
pub fn is_memory_system_available() -> bool {
    true
}

/// Obtener el allocator global
pub fn get_global_allocator() -> &'static KernelAllocator {
    &KERNEL_ALLOCATOR
}

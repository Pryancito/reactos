//! Sistema de Pools de Memoria
//! 
//! Implementa pools de memoria pre-asignados para operaciones
//! frecuentes y de alto rendimiento.

use core::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use core::ptr::NonNull;

/// Tamaño de bloque pequeño (64 bytes)
pub const SMALL_BLOCK_SIZE: usize = 64;

/// Tamaño de bloque mediano (256 bytes)
pub const MEDIUM_BLOCK_SIZE: usize = 256;

/// Tamaño de bloque grande (1024 bytes)
pub const LARGE_BLOCK_SIZE: usize = 1024;

/// Número de bloques por pool
pub const BLOCKS_PER_POOL: usize = 256;

/// Bloque de memoria en el pool
#[derive(Debug)]
pub struct MemoryBlock {
    pub data: [u8; LARGE_BLOCK_SIZE],
    pub size: usize,
    pub is_allocated: bool,
    pub next: Option<NonNull<MemoryBlock>>,
    pub prev: Option<NonNull<MemoryBlock>>,
    pub allocation_count: u64,
    pub last_used: u64,
}

impl MemoryBlock {
    pub fn new(size: usize) -> Self {
        Self {
            data: [0; LARGE_BLOCK_SIZE],
            size,
            is_allocated: false,
            next: None,
            prev: None,
            allocation_count: 0,
            last_used: 0,
        }
    }
    
    pub fn allocate(&mut self, size: usize, timestamp: u64) -> bool {
        if self.is_allocated || size > self.size {
            return false;
        }
        
        self.is_allocated = true;
        self.allocation_count += 1;
        self.last_used = timestamp;
        true
    }
    
    pub fn deallocate(&mut self) {
        self.is_allocated = false;
    }
    
    pub fn get_data(&mut self) -> &mut [u8] {
        &mut self.data[..self.size]
    }
}

/// Pool de memoria para bloques de tamaño específico
pub struct MemoryPool {
    pub block_size: usize,
    pub blocks: [MemoryBlock; BLOCKS_PER_POOL],
    pub free_list: Option<NonNull<MemoryBlock>>,
    pub allocated_count: AtomicUsize,
    pub total_allocations: AtomicU64,
    pub total_deallocations: AtomicU64,
    pub current_timestamp: AtomicU64,
}

impl MemoryPool {
    pub fn new(block_size: usize) -> Self {
        let mut pool = Self {
            block_size,
            blocks: unsafe { core::mem::zeroed() },
            free_list: None,
            allocated_count: AtomicUsize::new(0),
            total_allocations: AtomicU64::new(0),
            total_deallocations: AtomicU64::new(0),
            current_timestamp: AtomicU64::new(0),
        };
        
        // Inicializar bloques
        for i in 0..BLOCKS_PER_POOL {
            pool.blocks[i] = MemoryBlock::new(block_size);
        }
        
        // Construir lista libre
        pool.rebuild_free_list();
        
        pool
    }
    
    /// Reconstruir lista de bloques libres
    fn rebuild_free_list(&mut self) {
        self.free_list = None;
        
        for i in 0..BLOCKS_PER_POOL {
            if !self.blocks[i].is_allocated {
                let block_ptr = NonNull::new(&mut self.blocks[i] as *mut MemoryBlock).unwrap();
                unsafe {
                    (*block_ptr.as_ptr()).next = self.free_list;
                    self.free_list = Some(block_ptr);
                }
            }
        }
    }
    
    /// Asignar bloque del pool
    pub fn allocate(&mut self, size: usize) -> Option<&mut [u8]> {
        if size > self.block_size {
            return None;
        }
        
        self.current_timestamp.fetch_add(1, Ordering::Relaxed);
        let timestamp = self.current_timestamp.load(Ordering::Relaxed);
        
        if let Some(block_ptr) = self.free_list {
            unsafe {
                let block = &mut *block_ptr.as_ptr();
                if block.allocate(size, timestamp) {
                    self.free_list = block.next;
                    self.allocated_count.fetch_add(1, Ordering::Relaxed);
                    self.total_allocations.fetch_add(1, Ordering::Relaxed);
                    return Some(&mut block.data[..size]);
                }
            }
        }
        
        None
    }
    
    /// Liberar bloque del pool
    pub fn deallocate(&mut self, ptr: *mut u8) -> bool {
        // Encontrar el bloque que contiene este puntero
        for i in 0..BLOCKS_PER_POOL {
            let block_ptr = &mut self.blocks[i] as *mut MemoryBlock;
            let block_data_ptr = self.blocks[i].data.as_mut_ptr();
            
            if ptr >= block_data_ptr && ptr < block_data_ptr.add(self.block_size) {
                if self.blocks[i].is_allocated {
                    self.blocks[i].deallocate();
                    self.allocated_count.fetch_sub(1, Ordering::Relaxed);
                    self.total_deallocations.fetch_add(1, Ordering::Relaxed);
                    
                    // Reconstruir lista libre
                    self.rebuild_free_list();
                    return true;
                }
                break;
            }
        }
        
        false
    }
    
    /// Obtener estadísticas del pool
    pub fn get_stats(&self) -> PoolStats {
        PoolStats {
            block_size: self.block_size,
            total_blocks: BLOCKS_PER_POOL,
            allocated_blocks: self.allocated_count.load(Ordering::Relaxed),
            free_blocks: BLOCKS_PER_POOL - self.allocated_count.load(Ordering::Relaxed),
            total_allocations: self.total_allocations.load(Ordering::Relaxed),
            total_deallocations: self.total_deallocations.load(Ordering::Relaxed),
            utilization_rate: self.allocated_count.load(Ordering::Relaxed) as f64 / BLOCKS_PER_POOL as f64,
        }
    }
    
    /// Limpiar pool (liberar todos los bloques)
    pub fn cleanup(&mut self) {
        for block in &mut self.blocks {
            block.deallocate();
        }
        self.allocated_count.store(0, Ordering::Relaxed);
        self.rebuild_free_list();
    }
    
    /// Optimizar pool (limpiar bloques poco usados)
    pub fn optimize(&mut self) {
        let current_time = self.current_timestamp.load(Ordering::Relaxed);
        let threshold = current_time.saturating_sub(1000); // Bloques no usados por 1000 ticks
        
        for block in &mut self.blocks {
            if block.is_allocated && block.last_used < threshold && block.allocation_count < 2 {
                block.deallocate();
            }
        }
        
        self.rebuild_free_list();
    }
}

/// Estadísticas del pool
#[derive(Debug, Clone, Copy)]
pub struct PoolStats {
    pub block_size: usize,
    pub total_blocks: usize,
    pub allocated_blocks: usize,
    pub free_blocks: usize,
    pub total_allocations: u64,
    pub total_deallocations: u64,
    pub utilization_rate: f64,
}

/// Gestor de pools de memoria
pub struct PoolManager {
    pub small_pool: MemoryPool,
    pub medium_pool: MemoryPool,
    pub large_pool: MemoryPool,
    pub total_allocations: AtomicU64,
    pub total_deallocations: AtomicU64,
}

impl PoolManager {
    pub fn new() -> Self {
        Self {
            small_pool: MemoryPool::new(SMALL_BLOCK_SIZE),
            medium_pool: MemoryPool::new(MEDIUM_BLOCK_SIZE),
            large_pool: MemoryPool::new(LARGE_BLOCK_SIZE),
            total_allocations: AtomicU64::new(0),
            total_deallocations: AtomicU64::new(0),
        }
    }
    
    /// Asignar memoria del pool apropiado
    pub fn allocate(&mut self, size: usize) -> Option<&mut [u8]> {
        let result = if size <= SMALL_BLOCK_SIZE {
            self.small_pool.allocate(size)
        } else if size <= MEDIUM_BLOCK_SIZE {
            self.medium_pool.allocate(size)
        } else if size <= LARGE_BLOCK_SIZE {
            self.large_pool.allocate(size)
        } else {
            None
        };
        
        if result.is_some() {
            self.total_allocations.fetch_add(1, Ordering::Relaxed);
        }
        
        result
    }
    
    /// Liberar memoria del pool apropiado
    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) -> bool {
        let result = if size <= SMALL_BLOCK_SIZE {
            self.small_pool.deallocate(ptr)
        } else if size <= MEDIUM_BLOCK_SIZE {
            self.medium_pool.deallocate(ptr)
        } else if size <= LARGE_BLOCK_SIZE {
            self.large_pool.deallocate(ptr)
        } else {
            false
        };
        
        if result {
            self.total_deallocations.fetch_add(1, Ordering::Relaxed);
        }
        
        result
    }
    
    /// Obtener estadísticas de todos los pools
    pub fn get_stats(&self) -> ManagerStats {
        let small_stats = self.small_pool.get_stats();
        let medium_stats = self.medium_pool.get_stats();
        let large_stats = self.large_pool.get_stats();
        
        ManagerStats {
            small_pool: small_stats,
            medium_pool: medium_stats,
            large_pool: large_stats,
            total_allocations: self.total_allocations.load(Ordering::Relaxed),
            total_deallocations: self.total_deallocations.load(Ordering::Relaxed),
        }
    }
    
    /// Limpiar todos los pools
    pub fn cleanup_all(&mut self) {
        self.small_pool.cleanup();
        self.medium_pool.cleanup();
        self.large_pool.cleanup();
    }
    
    /// Optimizar todos los pools
    pub fn optimize_all(&mut self) {
        self.small_pool.optimize();
        self.medium_pool.optimize();
        self.large_pool.optimize();
    }
}

/// Estadísticas del gestor de pools
#[derive(Debug, Clone, Copy)]
pub struct ManagerStats {
    pub small_pool: PoolStats,
    pub medium_pool: PoolStats,
    pub large_pool: PoolStats,
    pub total_allocations: u64,
    pub total_deallocations: u64,
}

/// Pool manager global
static mut POOL_MANAGER: Option<PoolManager> = None;

/// Inicializar pools de memoria
pub fn init_memory_pools() {
    let manager = PoolManager::new();
    unsafe {
        POOL_MANAGER = Some(manager);
    }
}

/// Obtener gestor de pools
pub fn get_pool_manager() -> Option<&'static mut PoolManager> {
    unsafe {
        POOL_MANAGER.as_mut()
    }
}

/// Asignar memoria del pool
pub fn pool_allocate(size: usize) -> Option<&'static mut [u8]> {
    get_pool_manager().and_then(|manager| manager.allocate(size))
}

/// Liberar memoria del pool
pub fn pool_deallocate(ptr: *mut u8, size: usize) -> bool {
    get_pool_manager().map_or(false, |manager| manager.deallocate(ptr, size))
}

/// Obtener estadísticas de uso de pools
pub fn get_pool_usage() -> f64 {
    if let Some(manager) = get_pool_manager() {
        let stats = manager.get_stats();
        let total_blocks = stats.small_pool.total_blocks + 
                          stats.medium_pool.total_blocks + 
                          stats.large_pool.total_blocks;
        let allocated_blocks = stats.small_pool.allocated_blocks + 
                              stats.medium_pool.allocated_blocks + 
                              stats.large_pool.allocated_blocks;
        
        if total_blocks > 0 {
            allocated_blocks as f64 / total_blocks as f64
        } else {
            0.0
        }
    } else {
        0.0
    }
}

/// Limpiar todos los pools
pub fn cleanup_memory_pools() {
    if let Some(manager) = get_pool_manager() {
        manager.cleanup_all();
    }
}

/// Optimizar todos los pools
pub fn optimize_memory_pools() {
    if let Some(manager) = get_pool_manager() {
        manager.optimize_all();
    }
}

/// Pool especializado para estructuras de red
pub struct NetworkBufferPool {
    pub pool: MemoryPool,
    pub buffer_size: usize,
}

impl NetworkBufferPool {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            pool: MemoryPool::new(buffer_size),
            buffer_size,
        }
    }
    
    /// Obtener buffer de red
    pub fn get_buffer(&mut self) -> Option<&mut [u8]> {
        self.pool.allocate(self.buffer_size)
    }
    
    /// Devolver buffer de red
    pub fn return_buffer(&mut self, ptr: *mut u8) -> bool {
        self.pool.deallocate(ptr)
    }
    
    /// Obtener estadísticas
    pub fn get_stats(&self) -> PoolStats {
        self.pool.get_stats()
    }
}

/// Pool especializado para estructuras de proceso
pub struct ProcessStructPool {
    pub pool: MemoryPool,
    pub struct_size: usize,
}

impl ProcessStructPool {
    pub fn new(struct_size: usize) -> Self {
        Self {
            pool: MemoryPool::new(struct_size),
            struct_size,
        }
    }
    
    /// Obtener estructura de proceso
    pub fn get_struct(&mut self) -> Option<&mut [u8]> {
        self.pool.allocate(self.struct_size)
    }
    
    /// Devolver estructura de proceso
    pub fn return_struct(&mut self, ptr: *mut u8) -> bool {
        self.pool.deallocate(ptr)
    }
    
    /// Obtener estadísticas
    pub fn get_stats(&self) -> PoolStats {
        self.pool.get_stats()
    }
}

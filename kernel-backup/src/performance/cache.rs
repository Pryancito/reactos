//! Sistema de Cache de Memoria Optimizado
//! 
//! Implementa un cache de memoria de alto rendimiento para
//! operaciones frecuentes del kernel.

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use core::ptr::NonNull;

/// Tamaño de entrada de cache
pub const CACHE_ENTRY_SIZE: usize = 64;

/// Número máximo de entradas en el cache
pub const MAX_CACHE_ENTRIES: usize = 1024;

/// Entrada de cache
#[derive(Debug, Clone, Copy)]
pub struct CacheEntry {
    pub key: u64,
    pub data: [u8; CACHE_ENTRY_SIZE],
    pub size: usize,
    pub access_count: u32,
    pub last_access: u64,
    pub is_valid: bool,
}

impl CacheEntry {
    pub fn new() -> Self {
        Self {
            key: 0,
            data: [0; CACHE_ENTRY_SIZE],
            size: 0,
            access_count: 0,
            last_access: 0,
            is_valid: false,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        !self.is_valid
    }
    
    pub fn access(&mut self, timestamp: u64) {
        self.access_count += 1;
        self.last_access = timestamp;
    }
    
    pub fn invalidate(&mut self) {
        self.is_valid = false;
        self.key = 0;
        self.size = 0;
        self.access_count = 0;
        self.last_access = 0;
    }
}

/// Política de reemplazo de cache
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CachePolicy {
    LRU,    // Least Recently Used
    LFU,    // Least Frequently Used
    FIFO,   // First In, First Out
    Random, // Random replacement
}

/// Cache de memoria
pub struct MemoryCache {
    pub entries: [CacheEntry; MAX_CACHE_ENTRIES],
    pub policy: CachePolicy,
    pub hit_count: AtomicU64,
    pub miss_count: AtomicU64,
    pub eviction_count: AtomicU64,
    pub current_timestamp: AtomicU64,
    pub next_entry: AtomicUsize,
}

impl MemoryCache {
    pub fn new(policy: CachePolicy) -> Self {
        Self {
            entries: [CacheEntry::new(); MAX_CACHE_ENTRIES],
            policy,
            hit_count: AtomicU64::new(0),
            miss_count: AtomicU64::new(0),
            eviction_count: AtomicU64::new(0),
            current_timestamp: AtomicU64::new(0),
            next_entry: AtomicUsize::new(0),
        }
    }
    
    /// Buscar entrada en el cache
    pub fn get(&mut self, key: u64) -> Option<&mut [u8]> {
        self.current_timestamp.fetch_add(1, Ordering::Relaxed);
        let timestamp = self.current_timestamp.load(Ordering::Relaxed);
        
        for entry in &mut self.entries {
            if entry.is_valid && entry.key == key {
                entry.access(timestamp);
                self.hit_count.fetch_add(1, Ordering::Relaxed);
                return Some(&mut entry.data[..entry.size]);
            }
        }
        
        self.miss_count.fetch_add(1, Ordering::Relaxed);
        None
    }
    
    /// Insertar entrada en el cache
    pub fn put(&mut self, key: u64, data: &[u8]) -> bool {
        if data.len() > CACHE_ENTRY_SIZE {
            return false; // Datos demasiado grandes
        }
        
        self.current_timestamp.fetch_add(1, Ordering::Relaxed);
        let timestamp = self.current_timestamp.load(Ordering::Relaxed);
        
        // Buscar entrada existente
        for entry in &mut self.entries {
            if entry.is_valid && entry.key == key {
                entry.data[..data.len()].copy_from_slice(data);
                entry.size = data.len();
                entry.access(timestamp);
                return true;
            }
        }
        
        // Buscar entrada vacía
        for entry in &mut self.entries {
            if entry.is_empty() {
                entry.key = key;
                entry.data[..data.len()].copy_from_slice(data);
                entry.size = data.len();
                entry.is_valid = true;
                entry.access(timestamp);
                return true;
            }
        }
        
        // Cache lleno, evictar entrada
        if let Some(evict_index) = self.find_eviction_candidate() {
            self.entries[evict_index].invalidate();
            self.entries[evict_index].key = key;
            self.entries[evict_index].data[..data.len()].copy_from_slice(data);
            self.entries[evict_index].size = data.len();
            self.entries[evict_index].is_valid = true;
            self.entries[evict_index].access(timestamp);
            self.eviction_count.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }
    
    /// Encontrar candidato para evicción
    fn find_eviction_candidate(&self) -> Option<usize> {
        match self.policy {
            CachePolicy::LRU => {
                let mut oldest_time = u64::MAX;
                let mut oldest_index = None;
                
                for (i, entry) in self.entries.iter().enumerate() {
                    if entry.is_valid && entry.last_access < oldest_time {
                        oldest_time = entry.last_access;
                        oldest_index = Some(i);
                    }
                }
                oldest_index
            }
            CachePolicy::LFU => {
                let mut least_frequent = u32::MAX;
                let mut least_index = None;
                
                for (i, entry) in self.entries.iter().enumerate() {
                    if entry.is_valid && entry.access_count < least_frequent {
                        least_frequent = entry.access_count;
                        least_index = Some(i);
                    }
                }
                least_index
            }
            CachePolicy::FIFO => {
                // Usar next_entry como índice FIFO
                Some(self.next_entry.load(Ordering::Relaxed) % MAX_CACHE_ENTRIES)
            }
            CachePolicy::Random => {
                // Selección aleatoria simple (usando timestamp como seed)
                let timestamp = self.current_timestamp.load(Ordering::Relaxed);
                Some((timestamp as usize) % MAX_CACHE_ENTRIES)
            }
        }
    }
    
    /// Invalidar entrada específica
    pub fn invalidate(&mut self, key: u64) {
        for entry in &mut self.entries {
            if entry.is_valid && entry.key == key {
                entry.invalidate();
                break;
            }
        }
    }
    
    /// Limpiar todo el cache
    pub fn clear(&mut self) {
        for entry in &mut self.entries {
            entry.invalidate();
        }
        self.hit_count.store(0, Ordering::Relaxed);
        self.miss_count.store(0, Ordering::Relaxed);
        self.eviction_count.store(0, Ordering::Relaxed);
    }
    
    /// Obtener estadísticas del cache
    pub fn get_stats(&self) -> CacheStats {
        let hits = self.hit_count.load(Ordering::Relaxed);
        let misses = self.miss_count.load(Ordering::Relaxed);
        let total = hits + misses;
        
        CacheStats {
            hit_count: hits,
            miss_count: misses,
            hit_rate: if total > 0 { hits as f64 / total as f64 } else { 0.0 },
            eviction_count: self.eviction_count.load(Ordering::Relaxed),
            used_entries: self.count_used_entries(),
            total_entries: MAX_CACHE_ENTRIES,
        }
    }
    
    /// Contar entradas usadas
    fn count_used_entries(&self) -> usize {
        self.entries.iter().filter(|e| e.is_valid).count()
    }
    
    /// Optimizar cache (limpiar entradas poco usadas)
    pub fn optimize(&mut self) {
        let current_time = self.current_timestamp.load(Ordering::Relaxed);
        let threshold = current_time.saturating_sub(1000); // Entradas más antiguas que 1000 ticks
        
        for entry in &mut self.entries {
            if entry.is_valid && entry.last_access < threshold && entry.access_count < 2 {
                entry.invalidate();
            }
        }
    }
}

/// Estadísticas del cache
#[derive(Debug, Clone, Copy)]
pub struct CacheStats {
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
    pub eviction_count: u64,
    pub used_entries: usize,
    pub total_entries: usize,
}

/// Cache especializado para páginas de memoria
pub struct PageCache {
    pub cache: MemoryCache,
    pub page_size: usize,
}

impl PageCache {
    pub fn new(page_size: usize) -> Self {
        Self {
            cache: MemoryCache::new(CachePolicy::LRU),
            page_size,
        }
    }
    
    /// Obtener página del cache
    pub fn get_page(&mut self, page_addr: u64) -> Option<&mut [u8]> {
        self.cache.get(page_addr)
    }
    
    /// Insertar página en el cache
    pub fn put_page(&mut self, page_addr: u64, page_data: &[u8]) -> bool {
        if page_data.len() != self.page_size {
            return false;
        }
        self.cache.put(page_addr, page_data)
    }
    
    /// Invalidar página específica
    pub fn invalidate_page(&mut self, page_addr: u64) {
        self.cache.invalidate(page_addr);
    }
    
    /// Limpiar cache de páginas
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

/// Cache especializado para datos de red
pub struct NetworkCache {
    pub cache: MemoryCache,
    pub max_packet_size: usize,
}

impl NetworkCache {
    pub fn new(max_packet_size: usize) -> Self {
        Self {
            cache: MemoryCache::new(CachePolicy::FIFO),
            max_packet_size,
        }
    }
    
    /// Cachear paquete de red
    pub fn cache_packet(&mut self, packet_id: u64, packet_data: &[u8]) -> bool {
        if packet_data.len() > self.max_packet_size {
            return false;
        }
        self.cache.put(packet_id, packet_data)
    }
    
    /// Obtener paquete del cache
    pub fn get_packet(&mut self, packet_id: u64) -> Option<&mut [u8]> {
        self.cache.get(packet_id)
    }
}

/// Caches globales del sistema
static mut SYSTEM_PAGE_CACHE: Option<PageCache> = None;
static mut SYSTEM_NETWORK_CACHE: Option<NetworkCache> = None;
static mut SYSTEM_DATA_CACHE: Option<MemoryCache> = None;

/// Inicializar sistema de cache
pub fn init_memory_cache() {
    unsafe {
        SYSTEM_PAGE_CACHE = Some(PageCache::new(4096)); // 4KB pages
        SYSTEM_NETWORK_CACHE = Some(NetworkCache::new(1500)); // Ethernet MTU
        SYSTEM_DATA_CACHE = Some(MemoryCache::new(CachePolicy::LRU));
    }
}

/// Obtener cache de páginas
pub fn get_page_cache() -> Option<&'static mut PageCache> {
    unsafe {
        SYSTEM_PAGE_CACHE.as_mut()
    }
}

/// Obtener cache de red
pub fn get_network_cache() -> Option<&'static mut NetworkCache> {
    unsafe {
        SYSTEM_NETWORK_CACHE.as_mut()
    }
}

/// Obtener cache de datos
pub fn get_data_cache() -> Option<&'static mut MemoryCache> {
    unsafe {
        SYSTEM_DATA_CACHE.as_mut()
    }
}

/// Optimizar todos los caches
pub fn optimize_cache() {
    if let Some(page_cache) = get_page_cache() {
        page_cache.cache.optimize();
    }
    
    if let Some(network_cache) = get_network_cache() {
        network_cache.cache.optimize();
    }
    
    if let Some(data_cache) = get_data_cache() {
        data_cache.optimize();
    }
}

/// Obtener tasa de aciertos del cache
pub fn get_cache_hit_rate() -> f64 {
    let mut total_hits = 0u64;
    let mut total_misses = 0u64;
    
    if let Some(page_cache) = get_page_cache() {
        let stats = page_cache.cache.get_stats();
        total_hits += stats.hit_count;
        total_misses += stats.miss_count;
    }
    
    if let Some(network_cache) = get_network_cache() {
        let stats = network_cache.cache.get_stats();
        total_hits += stats.hit_count;
        total_misses += stats.miss_count;
    }
    
    if let Some(data_cache) = get_data_cache() {
        let stats = data_cache.get_stats();
        total_hits += stats.hit_count;
        total_misses += stats.miss_count;
    }
    
    let total = total_hits + total_misses;
    if total > 0 {
        total_hits as f64 / total as f64
    } else {
        0.0
    }
}

/// Limpiar todos los caches
pub fn clear_all_caches() {
    if let Some(page_cache) = get_page_cache() {
        page_cache.clear();
    }
    
    if let Some(network_cache) = get_network_cache() {
        network_cache.cache.clear();
    }
    
    if let Some(data_cache) = get_data_cache() {
        data_cache.clear();
    }
}

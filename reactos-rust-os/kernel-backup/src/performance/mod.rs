//! Sistema de Optimización de Rendimiento
//! 
//! Implementa optimizaciones avanzadas para mejorar el rendimiento
//! del kernel y sus componentes críticos.

pub mod cache;
pub mod pool;
pub mod metrics;
pub mod profiler;
pub mod scheduler_optimizer;

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Inicializar el sistema de optimización de rendimiento
pub fn init() {
    // Inicializar cache de memoria
    cache::init_memory_cache();
    
    // Inicializar pools de memoria
    pool::init_memory_pools();
    
    // Inicializar sistema de métricas
    metrics::init_performance_metrics();
    
    // Inicializar profiler
    profiler::init_profiler();
    
    // Inicializar optimizador del planificador
    scheduler_optimizer::init_scheduler_optimizer();
}

/// Procesar optimizaciones de rendimiento
pub fn process_performance_optimizations() {
    // Actualizar métricas
    metrics::update_metrics();
    
    // Optimizar cache
    cache::optimize_cache();
    
    // Limpiar pools de memoria
    pool::cleanup_memory_pools();
    
    // Procesar perfil de rendimiento
    profiler::process_profiling_data();
    
    // Optimizar planificador
    scheduler_optimizer::optimize_scheduler();
}

/// Obtener estadísticas de rendimiento
pub fn get_performance_stats() -> PerformanceStats {
    PerformanceStats {
        cache_hit_rate: cache::get_cache_hit_rate(),
        memory_pool_usage: pool::get_pool_usage(),
        context_switches_per_second: metrics::get_context_switches_per_second(),
        memory_allocations_per_second: metrics::get_allocations_per_second(),
        network_packets_per_second: metrics::get_network_packets_per_second(),
        cpu_utilization: metrics::get_cpu_utilization(),
        memory_utilization: metrics::get_memory_utilization(),
        average_response_time: metrics::get_average_response_time(),
    }
}

/// Estadísticas de rendimiento del sistema
#[derive(Debug, Clone, Copy)]
pub struct PerformanceStats {
    pub cache_hit_rate: f64,
    pub memory_pool_usage: f64,
    pub context_switches_per_second: u64,
    pub memory_allocations_per_second: u64,
    pub network_packets_per_second: u64,
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub average_response_time: f64,
}

/// Configuración de optimización
#[derive(Debug, Clone, Copy)]
pub struct OptimizationConfig {
    pub enable_memory_cache: bool,
    pub enable_memory_pools: bool,
    pub enable_scheduler_optimization: bool,
    pub enable_network_optimization: bool,
    pub cache_size_mb: usize,
    pub pool_size_mb: usize,
    pub optimization_interval_ms: u64,
}

impl OptimizationConfig {
    pub fn default() -> Self {
        Self {
            enable_memory_cache: true,
            enable_memory_pools: true,
            enable_scheduler_optimization: true,
            enable_network_optimization: true,
            cache_size_mb: 16,
            pool_size_mb: 8,
            optimization_interval_ms: 100,
        }
    }
    
    pub fn high_performance() -> Self {
        Self {
            enable_memory_cache: true,
            enable_memory_pools: true,
            enable_scheduler_optimization: true,
            enable_network_optimization: true,
            cache_size_mb: 64,
            pool_size_mb: 32,
            optimization_interval_ms: 50,
        }
    }
    
    pub fn low_memory() -> Self {
        Self {
            enable_memory_cache: true,
            enable_memory_pools: false,
            enable_scheduler_optimization: true,
            enable_network_optimization: false,
            cache_size_mb: 4,
            pool_size_mb: 0,
            optimization_interval_ms: 200,
        }
    }
}

/// Configuración global de optimización
static mut OPTIMIZATION_CONFIG: OptimizationConfig = OptimizationConfig {
    enable_memory_cache: true,
    enable_memory_pools: true,
    enable_scheduler_optimization: true,
    enable_network_optimization: true,
    cache_size_mb: 16,
    pool_size_mb: 8,
    optimization_interval_ms: 100,
};

/// Establecer configuración de optimización
pub fn set_optimization_config(config: OptimizationConfig) {
    unsafe {
        OPTIMIZATION_CONFIG = config;
    }
}

/// Obtener configuración de optimización
pub fn get_optimization_config() -> OptimizationConfig {
    unsafe {
        OPTIMIZATION_CONFIG
    }
}

/// Contador global de optimizaciones aplicadas
static OPTIMIZATION_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Incrementar contador de optimizaciones
pub fn increment_optimization_counter() {
    OPTIMIZATION_COUNTER.fetch_add(1, Ordering::Relaxed);
}

/// Obtener número de optimizaciones aplicadas
pub fn get_optimization_count() -> u64 {
    OPTIMIZATION_COUNTER.load(Ordering::Relaxed)
}

/// Estado de optimización del sistema
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationState {
    Disabled,
    Low,
    Medium,
    High,
    Maximum,
}

impl OptimizationState {
    pub fn to_config(&self) -> OptimizationConfig {
        match self {
            OptimizationState::Disabled => OptimizationConfig {
                enable_memory_cache: false,
                enable_memory_pools: false,
                enable_scheduler_optimization: false,
                enable_network_optimization: false,
                cache_size_mb: 0,
                pool_size_mb: 0,
                optimization_interval_ms: 1000,
            },
            OptimizationState::Low => OptimizationConfig {
                enable_memory_cache: true,
                enable_memory_pools: false,
                enable_scheduler_optimization: true,
                enable_network_optimization: false,
                cache_size_mb: 4,
                pool_size_mb: 0,
                optimization_interval_ms: 500,
            },
            OptimizationState::Medium => OptimizationConfig::default(),
            OptimizationState::High => OptimizationConfig::high_performance(),
            OptimizationState::Maximum => OptimizationConfig {
                enable_memory_cache: true,
                enable_memory_pools: true,
                enable_scheduler_optimization: true,
                enable_network_optimization: true,
                cache_size_mb: 128,
                pool_size_mb: 64,
                optimization_interval_ms: 25,
            },
        }
    }
}

/// Establecer nivel de optimización
pub fn set_optimization_level(level: OptimizationState) {
    let config = level.to_config();
    set_optimization_config(config);
}

/// Obtener nivel de optimización actual
pub fn get_optimization_level() -> OptimizationState {
    let config = get_optimization_config();
    
    if !config.enable_memory_cache && !config.enable_memory_pools {
        OptimizationState::Disabled
    } else if config.cache_size_mb <= 4 {
        OptimizationState::Low
    } else if config.cache_size_mb <= 16 {
        OptimizationState::Medium
    } else if config.cache_size_mb <= 64 {
        OptimizationState::High
    } else {
        OptimizationState::Maximum
    }
}

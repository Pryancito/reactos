//! Sistema de Métricas de Rendimiento
//! 
//! Recopila y analiza métricas de rendimiento del sistema
//! en tiempo real.

use core::sync::atomic::{AtomicU64, AtomicUsize, AtomicBool, Ordering};
use core::time::Duration;

/// Métricas del sistema
pub struct PerformanceMetrics {
    // Métricas de CPU
    pub cpu_utilization: AtomicUsize,
    pub context_switches: AtomicU64,
    pub interrupts_handled: AtomicU64,
    pub system_calls: AtomicU64,
    
    // Métricas de memoria
    pub memory_allocations: AtomicU64,
    pub memory_deallocations: AtomicU64,
    pub page_faults: AtomicU64,
    pub memory_usage_bytes: AtomicU64,
    
    // Métricas de red
    pub network_packets_sent: AtomicU64,
    pub network_packets_received: AtomicU64,
    pub network_bytes_sent: AtomicU64,
    pub network_bytes_received: AtomicU64,
    
    // Métricas de I/O
    pub disk_reads: AtomicU64,
    pub disk_writes: AtomicU64,
    pub disk_bytes_read: AtomicU64,
    pub disk_bytes_written: AtomicU64,
    
    // Métricas de tiempo
    pub boot_time: AtomicU64,
    pub uptime_seconds: AtomicU64,
    pub last_update: AtomicU64,
    
    // Métricas de rendimiento
    pub average_response_time: AtomicU64,
    pub throughput_ops_per_second: AtomicU64,
    pub error_count: AtomicU64,
    
    // Estado del sistema
    pub is_collecting: AtomicBool,
    pub collection_interval_ms: AtomicU64,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            cpu_utilization: AtomicUsize::new(0),
            context_switches: AtomicU64::new(0),
            interrupts_handled: AtomicU64::new(0),
            system_calls: AtomicU64::new(0),
            
            memory_allocations: AtomicU64::new(0),
            memory_deallocations: AtomicU64::new(0),
            page_faults: AtomicU64::new(0),
            memory_usage_bytes: AtomicU64::new(0),
            
            network_packets_sent: AtomicU64::new(0),
            network_packets_received: AtomicU64::new(0),
            network_bytes_sent: AtomicU64::new(0),
            network_bytes_received: AtomicU64::new(0),
            
            disk_reads: AtomicU64::new(0),
            disk_writes: AtomicU64::new(0),
            disk_bytes_read: AtomicU64::new(0),
            disk_bytes_written: AtomicU64::new(0),
            
            boot_time: AtomicU64::new(0),
            uptime_seconds: AtomicU64::new(0),
            last_update: AtomicU64::new(0),
            
            average_response_time: AtomicU64::new(0),
            throughput_ops_per_second: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            
            is_collecting: AtomicBool::new(true),
            collection_interval_ms: AtomicU64::new(100),
        }
    }
    
    /// Incrementar contador de cambios de contexto
    pub fn increment_context_switches(&self) {
        self.context_switches.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Incrementar contador de interrupciones
    pub fn increment_interrupts(&self) {
        self.interrupts_handled.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Incrementar contador de llamadas al sistema
    pub fn increment_system_calls(&self) {
        self.system_calls.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Incrementar contador de asignaciones de memoria
    pub fn increment_memory_allocations(&self) {
        self.memory_allocations.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Incrementar contador de liberaciones de memoria
    pub fn increment_memory_deallocations(&self) {
        self.memory_deallocations.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Incrementar contador de fallos de página
    pub fn increment_page_faults(&self) {
        self.page_faults.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Actualizar uso de memoria
    pub fn update_memory_usage(&self, bytes: u64) {
        self.memory_usage_bytes.store(bytes, Ordering::Relaxed);
    }
    
    /// Incrementar contador de paquetes de red enviados
    pub fn increment_network_packets_sent(&self) {
        self.network_packets_sent.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Incrementar contador de paquetes de red recibidos
    pub fn increment_network_packets_received(&self) {
        self.network_packets_received.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Actualizar bytes de red enviados
    pub fn add_network_bytes_sent(&self, bytes: u64) {
        self.network_bytes_sent.fetch_add(bytes, Ordering::Relaxed);
    }
    
    /// Actualizar bytes de red recibidos
    pub fn add_network_bytes_received(&self, bytes: u64) {
        self.network_bytes_received.fetch_add(bytes, Ordering::Relaxed);
    }
    
    /// Incrementar contador de lecturas de disco
    pub fn increment_disk_reads(&self) {
        self.disk_reads.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Incrementar contador de escrituras de disco
    pub fn increment_disk_writes(&self) {
        self.disk_writes.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Actualizar bytes leídos del disco
    pub fn add_disk_bytes_read(&self, bytes: u64) {
        self.disk_bytes_read.fetch_add(bytes, Ordering::Relaxed);
    }
    
    /// Actualizar bytes escritos al disco
    pub fn add_disk_bytes_written(&self, bytes: u64) {
        self.disk_bytes_written.fetch_add(bytes, Ordering::Relaxed);
    }
    
    /// Incrementar contador de errores
    pub fn increment_errors(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Actualizar tiempo de respuesta promedio
    pub fn update_response_time(&self, time_us: u64) {
        self.average_response_time.store(time_us, Ordering::Relaxed);
    }
    
    /// Actualizar throughput
    pub fn update_throughput(&self, ops_per_second: u64) {
        self.throughput_ops_per_second.store(ops_per_second, Ordering::Relaxed);
    }
    
    /// Actualizar tiempo de actividad
    pub fn update_uptime(&self, seconds: u64) {
        self.uptime_seconds.store(seconds, Ordering::Relaxed);
    }
    
    /// Obtener todas las métricas
    pub fn get_all_metrics(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            cpu_utilization: self.cpu_utilization.load(Ordering::Relaxed),
            context_switches: self.context_switches.load(Ordering::Relaxed),
            interrupts_handled: self.interrupts_handled.load(Ordering::Relaxed),
            system_calls: self.system_calls.load(Ordering::Relaxed),
            
            memory_allocations: self.memory_allocations.load(Ordering::Relaxed),
            memory_deallocations: self.memory_deallocations.load(Ordering::Relaxed),
            page_faults: self.page_faults.load(Ordering::Relaxed),
            memory_usage_bytes: self.memory_usage_bytes.load(Ordering::Relaxed),
            
            network_packets_sent: self.network_packets_sent.load(Ordering::Relaxed),
            network_packets_received: self.network_packets_received.load(Ordering::Relaxed),
            network_bytes_sent: self.network_bytes_sent.load(Ordering::Relaxed),
            network_bytes_received: self.network_bytes_received.load(Ordering::Relaxed),
            
            disk_reads: self.disk_reads.load(Ordering::Relaxed),
            disk_writes: self.disk_writes.load(Ordering::Relaxed),
            disk_bytes_read: self.disk_bytes_read.load(Ordering::Relaxed),
            disk_bytes_written: self.disk_bytes_written.load(Ordering::Relaxed),
            
            uptime_seconds: self.uptime_seconds.load(Ordering::Relaxed),
            average_response_time: self.average_response_time.load(Ordering::Relaxed),
            throughput_ops_per_second: self.throughput_ops_per_second.load(Ordering::Relaxed),
            error_count: self.error_count.load(Ordering::Relaxed),
        }
    }
    
    /// Resetear todas las métricas
    pub fn reset(&self) {
        self.cpu_utilization.store(0, Ordering::Relaxed);
        self.context_switches.store(0, Ordering::Relaxed);
        self.interrupts_handled.store(0, Ordering::Relaxed);
        self.system_calls.store(0, Ordering::Relaxed);
        
        self.memory_allocations.store(0, Ordering::Relaxed);
        self.memory_deallocations.store(0, Ordering::Relaxed);
        self.page_faults.store(0, Ordering::Relaxed);
        
        self.network_packets_sent.store(0, Ordering::Relaxed);
        self.network_packets_received.store(0, Ordering::Relaxed);
        self.network_bytes_sent.store(0, Ordering::Relaxed);
        self.network_bytes_received.store(0, Ordering::Relaxed);
        
        self.disk_reads.store(0, Ordering::Relaxed);
        self.disk_writes.store(0, Ordering::Relaxed);
        self.disk_bytes_read.store(0, Ordering::Relaxed);
        self.disk_bytes_written.store(0, Ordering::Relaxed);
        
        self.average_response_time.store(0, Ordering::Relaxed);
        self.throughput_ops_per_second.store(0, Ordering::Relaxed);
        self.error_count.store(0, Ordering::Relaxed);
    }
}

/// Instantánea de métricas
#[derive(Debug, Clone, Copy)]
pub struct MetricsSnapshot {
    pub cpu_utilization: usize,
    pub context_switches: u64,
    pub interrupts_handled: u64,
    pub system_calls: u64,
    
    pub memory_allocations: u64,
    pub memory_deallocations: u64,
    pub page_faults: u64,
    pub memory_usage_bytes: u64,
    
    pub network_packets_sent: u64,
    pub network_packets_received: u64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    
    pub disk_reads: u64,
    pub disk_writes: u64,
    pub disk_bytes_read: u64,
    pub disk_bytes_written: u64,
    
    pub uptime_seconds: u64,
    pub average_response_time: u64,
    pub throughput_ops_per_second: u64,
    pub error_count: u64,
}

/// Analizador de rendimiento
pub struct PerformanceAnalyzer {
    pub metrics: PerformanceMetrics,
    pub previous_snapshot: Option<MetricsSnapshot>,
    pub analysis_interval: u64,
    pub last_analysis: u64,
}

impl PerformanceAnalyzer {
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics::new(),
            previous_snapshot: None,
            analysis_interval: 1000, // 1 segundo
            last_analysis: 0,
        }
    }
    
    /// Analizar rendimiento
    pub fn analyze(&mut self, current_time: u64) -> Option<PerformanceAnalysis> {
        if current_time - self.last_analysis < self.analysis_interval {
            return None;
        }
        
        let current_snapshot = self.metrics.get_all_metrics();
        
        if let Some(previous) = self.previous_snapshot {
            let analysis = self.compare_snapshots(&previous, &current_snapshot);
            self.previous_snapshot = Some(current_snapshot);
            self.last_analysis = current_time;
            Some(analysis)
        } else {
            self.previous_snapshot = Some(current_snapshot);
            self.last_analysis = current_time;
            None
        }
    }
    
    /// Comparar dos instantáneas de métricas
    fn compare_snapshots(&self, previous: &MetricsSnapshot, current: &MetricsSnapshot) -> PerformanceAnalysis {
        let time_delta = 1.0; // Asumimos 1 segundo entre mediciones
        
        PerformanceAnalysis {
            context_switches_per_second: (current.context_switches - previous.context_switches) as f64 / time_delta,
            memory_allocations_per_second: (current.memory_allocations - previous.memory_allocations) as f64 / time_delta,
            network_packets_per_second: (current.network_packets_sent + current.network_packets_received - 
                                        previous.network_packets_sent - previous.network_packets_received) as f64 / time_delta,
            disk_operations_per_second: (current.disk_reads + current.disk_writes - 
                                        previous.disk_reads - previous.disk_writes) as f64 / time_delta,
            cpu_utilization: current.cpu_utilization as f64,
            memory_utilization: self.calculate_memory_utilization(current),
            average_response_time: current.average_response_time as f64,
            throughput: current.throughput_ops_per_second as f64,
            error_rate: (current.error_count - previous.error_count) as f64 / time_delta,
        }
    }
    
    /// Calcular utilización de memoria
    fn calculate_memory_utilization(&self, snapshot: &MetricsSnapshot) -> f64 {
        // Asumimos 1GB de memoria total para el cálculo
        const TOTAL_MEMORY_BYTES: u64 = 1024 * 1024 * 1024;
        snapshot.memory_usage_bytes as f64 / TOTAL_MEMORY_BYTES as f64
    }
}

/// Análisis de rendimiento
#[derive(Debug, Clone, Copy)]
pub struct PerformanceAnalysis {
    pub context_switches_per_second: f64,
    pub memory_allocations_per_second: f64,
    pub network_packets_per_second: f64,
    pub disk_operations_per_second: f64,
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub average_response_time: f64,
    pub throughput: f64,
    pub error_rate: f64,
}

/// Métricas globales del sistema
static mut SYSTEM_METRICS: Option<PerformanceMetrics> = None;
static mut SYSTEM_ANALYZER: Option<PerformanceAnalyzer> = None;

/// Inicializar sistema de métricas
pub fn init_performance_metrics() {
    let metrics = PerformanceMetrics::new();
    let analyzer = PerformanceAnalyzer::new();
    
    unsafe {
        SYSTEM_METRICS = Some(metrics);
        SYSTEM_ANALYZER = Some(analyzer);
    }
}

/// Obtener métricas del sistema
pub fn get_system_metrics() -> Option<&'static PerformanceMetrics> {
    unsafe {
        SYSTEM_METRICS.as_ref()
    }
}

/// Obtener analizador del sistema
pub fn get_system_analyzer() -> Option<&'static mut PerformanceAnalyzer> {
    unsafe {
        SYSTEM_ANALYZER.as_mut()
    }
}

/// Actualizar métricas
pub fn update_metrics() {
    if let Some(analyzer) = get_system_analyzer() {
        let current_time = analyzer.metrics.uptime_seconds.load(Ordering::Relaxed);
        analyzer.metrics.update_uptime(current_time + 1);
        
        if let Some(analysis) = analyzer.analyze(current_time) {
            // Procesar análisis de rendimiento
            process_performance_analysis(&analysis);
        }
    }
}

/// Procesar análisis de rendimiento
fn process_performance_analysis(analysis: &PerformanceAnalysis) {
    // Aquí se pueden implementar acciones basadas en el análisis
    // Por ejemplo, ajustar parámetros del sistema, generar alertas, etc.
    
    // Ejemplo: Si la utilización de CPU es muy alta, reducir prioridad de algunos procesos
    if analysis.cpu_utilization > 90.0 {
        // TODO: Implementar lógica de ajuste de CPU
    }
    
    // Ejemplo: Si la tasa de errores es alta, activar modo de depuración
    if analysis.error_rate > 10.0 {
        // TODO: Implementar lógica de manejo de errores
    }
}

/// Funciones de conveniencia para métricas específicas
pub fn get_context_switches_per_second() -> u64 {
    if let Some(analyzer) = get_system_analyzer() {
        if let Some(analysis) = analyzer.analyze(analyzer.metrics.uptime_seconds.load(Ordering::Relaxed)) {
            analysis.context_switches_per_second as u64
        } else {
            0
        }
    } else {
        0
    }
}

pub fn get_allocations_per_second() -> u64 {
    if let Some(analyzer) = get_system_analyzer() {
        if let Some(analysis) = analyzer.analyze(analyzer.metrics.uptime_seconds.load(Ordering::Relaxed)) {
            analysis.memory_allocations_per_second as u64
        } else {
            0
        }
    } else {
        0
    }
}

pub fn get_network_packets_per_second() -> u64 {
    if let Some(analyzer) = get_system_analyzer() {
        if let Some(analysis) = analyzer.analyze(analyzer.metrics.uptime_seconds.load(Ordering::Relaxed)) {
            analysis.network_packets_per_second as u64
        } else {
            0
        }
    } else {
        0
    }
}

pub fn get_cpu_utilization() -> f64 {
    if let Some(metrics) = get_system_metrics() {
        metrics.cpu_utilization.load(Ordering::Relaxed) as f64
    } else {
        0.0
    }
}

pub fn get_memory_utilization() -> f64 {
    if let Some(analyzer) = get_system_analyzer() {
        let current_snapshot = analyzer.metrics.get_all_metrics();
        analyzer.calculate_memory_utilization(&current_snapshot)
    } else {
        0.0
    }
}

pub fn get_average_response_time() -> f64 {
    if let Some(metrics) = get_system_metrics() {
        metrics.average_response_time.load(Ordering::Relaxed) as f64
    } else {
        0.0
    }
}

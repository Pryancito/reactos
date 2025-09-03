//! Funciones comunes para herramientas de depuración
//! 
//! Este módulo contiene funciones y estructuras comunes
//! utilizadas por todas las herramientas de depuración.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Tipo de evento de depuración
#[derive(Debug, Clone, PartialEq)]
pub enum DebugEventType {
    ProcessCreated,
    ProcessTerminated,
    ThreadCreated,
    ThreadTerminated,
    MemoryAllocated,
    MemoryFreed,
    MemoryLeak,
    SystemCall,
    Interrupt,
    Exception,
    PerformanceIssue,
    SystemError,
    Custom(String),
}

/// Evento de depuración
#[derive(Debug, Clone)]
pub struct DebugEvent {
    pub event_type: DebugEventType,
    pub timestamp: Instant,
    pub process_id: Option<u32>,
    pub thread_id: Option<u32>,
    pub message: String,
    pub data: HashMap<String, String>,
    pub severity: DebugSeverity,
}

/// Severidad del evento
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DebugSeverity {
    Low,
    Medium,
    High,
    Critical,
    Info,
}

/// Información del proceso
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub parent_pid: Option<u32>,
    pub state: ProcessState,
    pub priority: ProcessPriority,
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub start_time: Instant,
    pub threads: Vec<ThreadInfo>,
}

/// Estado del proceso
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Dead,
}

/// Prioridad del proceso
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessPriority {
    Idle,
    Low,
    Normal,
    High,
    RealTime,
}

/// Información del thread
#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub tid: u32,
    pub name: String,
    pub state: ThreadState,
    pub priority: ThreadPriority,
    pub cpu_usage: f64,
    pub stack_size: u64,
    pub stack_pointer: u64,
    pub instruction_pointer: u64,
}

/// Estado del thread
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadState {
    Running,
    Ready,
    Blocked,
    Suspended,
    Terminated,
}

/// Prioridad del thread
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreadPriority {
    Idle,
    Low,
    Normal,
    High,
    Critical,
    RealTime,
}

/// Información de memoria
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub address: u64,
    pub size: u64,
    pub type_: MemoryType,
    pub permissions: MemoryPermissions,
    pub is_allocated: bool,
    pub allocation_time: Option<Instant>,
    pub deallocation_time: Option<Instant>,
    pub stack_trace: Vec<String>,
}

/// Tipo de memoria
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryType {
    Code,
    Data,
    Stack,
    Heap,
    Mapped,
    Shared,
    Kernel,
    Unknown,
}

/// Permisos de memoria
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub shared: bool,
}

/// Información de rendimiento
#[derive(Debug, Clone)]
pub struct PerformanceInfo {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
    pub context_switches: u64,
    pub page_faults: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub timestamp: Instant,
}

/// Información del sistema
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub architecture: String,
    pub cpu_count: usize,
    pub total_memory: u64,
    pub available_memory: u64,
    pub total_disk_space: u64,
    pub available_disk_space: u64,
    pub uptime: Duration,
    pub load_average: [f64; 3],
}

/// Utilidades de depuración
pub struct DebugUtils;

impl DebugUtils {
    /// Formatear tamaño de memoria
    pub fn format_memory_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }
    
    /// Formatear duración
    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        
        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
    
    /// Formatear timestamp
    pub fn format_timestamp(instant: Instant) -> String {
        let duration = instant.elapsed();
        format!("{}", Self::format_duration(duration))
    }
    
    /// Formatear porcentaje
    pub fn format_percentage(value: f64) -> String {
        format!("{:.2}%", value * 100.0)
    }
    
    /// Formatear dirección de memoria
    pub fn format_memory_address(address: u64) -> String {
        format!("0x{:016X}", address)
    }
    
    /// Formatear PID
    pub fn format_pid(pid: u32) -> String {
        format!("PID:{}", pid)
    }
    
    /// Formatear TID
    pub fn format_tid(tid: u32) -> String {
        format!("TID:{}", tid)
    }
    
    /// Obtener stack trace simulado
    pub fn get_stack_trace() -> Vec<String> {
        vec![
            "main()".to_string(),
            "process_start()".to_string(),
            "kernel_init()".to_string(),
            "system_boot()".to_string(),
        ]
    }
    
    /// Obtener información del sistema
    pub fn get_system_info() -> SystemInfo {
        SystemInfo {
            os_name: "ReactOS Rust".to_string(),
            os_version: "1.0.0".to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            cpu_count: 4, // Simulado
            total_memory: 8 * 1024 * 1024 * 1024, // 8GB simulado
            available_memory: 4 * 1024 * 1024 * 1024, // 4GB simulado
            total_disk_space: 500 * 1024 * 1024 * 1024, // 500GB simulado
            available_disk_space: 200 * 1024 * 1024 * 1024, // 200GB simulado
            uptime: Duration::from_secs(3600), // 1 hora simulado
            load_average: [0.5, 0.7, 0.6], // Simulado
        }
    }
    
    /// Obtener información de rendimiento
    pub fn get_performance_info() -> PerformanceInfo {
        let now = Instant::now();
        PerformanceInfo {
            cpu_usage: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() % 100) as f64 / 100.0,
            memory_usage: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() % 80) as f64 / 100.0,
            disk_usage: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() % 60) as f64 / 100.0,
            network_usage: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() % 40) as f64 / 100.0,
            context_switches: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() % 1000) as u64,
            page_faults: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() % 500) as u64,
            cache_hits: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() % 2000) as u64,
            cache_misses: (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() % 200) as u64,
            timestamp: now,
        }
    }
    
    /// Crear evento de depuración
    pub fn create_debug_event(
        event_type: DebugEventType,
        message: String,
        severity: DebugSeverity,
    ) -> DebugEvent {
        DebugEvent {
            event_type,
            timestamp: Instant::now(),
            process_id: None,
            thread_id: None,
            message,
            data: HashMap::new(),
            severity,
        }
    }
    
    /// Crear evento de depuración con datos
    pub fn create_debug_event_with_data(
        event_type: DebugEventType,
        message: String,
        severity: DebugSeverity,
        data: HashMap<String, String>,
    ) -> DebugEvent {
        DebugEvent {
            event_type,
            timestamp: Instant::now(),
            process_id: None,
            thread_id: None,
            message,
            data,
            severity,
        }
    }
    
    /// Crear evento de depuración de proceso
    pub fn create_process_debug_event(
        event_type: DebugEventType,
        process_id: u32,
        message: String,
        severity: DebugSeverity,
    ) -> DebugEvent {
        DebugEvent {
            event_type,
            timestamp: Instant::now(),
            process_id: Some(process_id),
            thread_id: None,
            message,
            data: HashMap::new(),
            severity,
        }
    }
    
    /// Crear evento de depuración de thread
    pub fn create_thread_debug_event(
        event_type: DebugEventType,
        process_id: u32,
        thread_id: u32,
        message: String,
        severity: DebugSeverity,
    ) -> DebugEvent {
        DebugEvent {
            event_type,
            timestamp: Instant::now(),
            process_id: Some(process_id),
            thread_id: Some(thread_id),
            message,
            data: HashMap::new(),
            severity,
        }
    }
    
    /// Crear evento de depuración de memoria
    pub fn create_memory_debug_event(
        event_type: DebugEventType,
        address: u64,
        size: u64,
        message: String,
        severity: DebugSeverity,
    ) -> DebugEvent {
        let mut data = HashMap::new();
        data.insert("address".to_string(), Self::format_memory_address(address));
        data.insert("size".to_string(), Self::format_memory_size(size));
        
        DebugEvent {
            event_type,
            timestamp: Instant::now(),
            process_id: None,
            thread_id: None,
            message,
            data,
            severity,
        }
    }
    
    /// Crear evento de depuración de rendimiento
    pub fn create_performance_debug_event(
        event_type: DebugEventType,
        performance_info: PerformanceInfo,
        message: String,
        severity: DebugSeverity,
    ) -> DebugEvent {
        let mut data = HashMap::new();
        data.insert("cpu_usage".to_string(), Self::format_percentage(performance_info.cpu_usage));
        data.insert("memory_usage".to_string(), Self::format_percentage(performance_info.memory_usage));
        data.insert("disk_usage".to_string(), Self::format_percentage(performance_info.disk_usage));
        data.insert("network_usage".to_string(), Self::format_percentage(performance_info.network_usage));
        data.insert("context_switches".to_string(), performance_info.context_switches.to_string());
        data.insert("page_faults".to_string(), performance_info.page_faults.to_string());
        data.insert("cache_hits".to_string(), performance_info.cache_hits.to_string());
        data.insert("cache_misses".to_string(), performance_info.cache_misses.to_string());
        
        DebugEvent {
            event_type,
            timestamp: performance_info.timestamp,
            process_id: None,
            thread_id: None,
            message,
            data,
            severity,
        }
    }
    
    /// Crear evento de depuración del sistema
    pub fn create_system_debug_event(
        event_type: DebugEventType,
        system_info: SystemInfo,
        message: String,
        severity: DebugSeverity,
    ) -> DebugEvent {
        let mut data = HashMap::new();
        data.insert("os_name".to_string(), system_info.os_name);
        data.insert("os_version".to_string(), system_info.os_version);
        data.insert("architecture".to_string(), system_info.architecture);
        data.insert("cpu_count".to_string(), system_info.cpu_count.to_string());
        data.insert("total_memory".to_string(), Self::format_memory_size(system_info.total_memory));
        data.insert("available_memory".to_string(), Self::format_memory_size(system_info.available_memory));
        data.insert("total_disk_space".to_string(), Self::format_memory_size(system_info.total_disk_space));
        data.insert("available_disk_space".to_string(), Self::format_memory_size(system_info.available_disk_space));
        data.insert("uptime".to_string(), Self::format_duration(system_info.uptime));
        data.insert("load_average".to_string(), format!("{:.2}, {:.2}, {:.2}", 
            system_info.load_average[0], system_info.load_average[1], system_info.load_average[2]));
        
        DebugEvent {
            event_type,
            timestamp: Instant::now(),
            process_id: None,
            thread_id: None,
            message,
            data,
            severity,
        }
    }
    
    /// Formatear evento de depuración
    pub fn format_debug_event(event: &DebugEvent) -> String {
        let mut result = String::new();
        
        // Timestamp
        result.push_str(&format!("[{}] ", Self::format_timestamp(event.timestamp)));
        
        // Severidad
        result.push_str(&format!("{:?} ", event.severity));
        
        // Tipo de evento
        result.push_str(&format!("{:?} ", event.event_type));
        
        // PID y TID si están disponibles
        if let Some(pid) = event.process_id {
            result.push_str(&format!("{} ", Self::format_pid(pid)));
        }
        if let Some(tid) = event.thread_id {
            result.push_str(&format!("{} ", Self::format_tid(tid)));
        }
        
        // Mensaje
        result.push_str(&event.message);
        
        // Datos adicionales
        if !event.data.is_empty() {
            result.push_str(" [");
            let mut first = true;
            for (key, value) in &event.data {
                if !first {
                    result.push_str(", ");
                }
                result.push_str(&format!("{}={}", key, value));
                first = false;
            }
            result.push_str("]");
        }
        
        result
    }
    
    /// Filtrar eventos por tipo
    pub fn filter_events_by_type<'a>(events: &'a [DebugEvent], event_type: &DebugEventType) -> Vec<&'a DebugEvent> {
        events.iter().filter(|event| &event.event_type == event_type).collect()
    }
    
    /// Filtrar eventos por severidad
    pub fn filter_events_by_severity(events: &[DebugEvent], min_severity: DebugSeverity) -> Vec<&DebugEvent> {
        events.iter().filter(|event| event.severity >= min_severity).collect()
    }
    
    /// Filtrar eventos por proceso
    pub fn filter_events_by_process(events: &[DebugEvent], process_id: u32) -> Vec<&DebugEvent> {
        events.iter().filter(|event| event.process_id == Some(process_id)).collect()
    }
    
    /// Filtrar eventos por thread
    pub fn filter_events_by_thread(events: &[DebugEvent], thread_id: u32) -> Vec<&DebugEvent> {
        events.iter().filter(|event| event.thread_id == Some(thread_id)).collect()
    }
    
    /// Filtrar eventos por rango de tiempo
    pub fn filter_events_by_time_range(events: &[DebugEvent], start: Instant, end: Instant) -> Vec<&DebugEvent> {
        events.iter().filter(|event| event.timestamp >= start && event.timestamp <= end).collect()
    }
    
    /// Obtener estadísticas de eventos
    pub fn get_event_statistics(events: &[DebugEvent]) -> EventStatistics {
        let mut stats = EventStatistics::new();
        
        for event in events {
            stats.total_events += 1;
            
            match event.severity {
                DebugSeverity::Low => stats.low_severity += 1,
                DebugSeverity::Medium => stats.medium_severity += 1,
                DebugSeverity::High => stats.high_severity += 1,
                DebugSeverity::Critical => stats.critical_severity += 1,
                DebugSeverity::Info => stats.low_severity += 1, // Info se cuenta como Low
            }
            
            match event.event_type {
                DebugEventType::ProcessCreated => stats.process_created += 1,
                DebugEventType::ProcessTerminated => stats.process_terminated += 1,
                DebugEventType::ThreadCreated => stats.thread_created += 1,
                DebugEventType::ThreadTerminated => stats.thread_terminated += 1,
                DebugEventType::MemoryAllocated => stats.memory_allocated += 1,
                DebugEventType::MemoryFreed => stats.memory_freed += 1,
                DebugEventType::MemoryLeak => stats.memory_leaks += 1,
                DebugEventType::SystemCall => stats.system_calls += 1,
                DebugEventType::Interrupt => stats.interrupts += 1,
                DebugEventType::Exception => stats.exceptions += 1,
                DebugEventType::PerformanceIssue => stats.performance_issues += 1,
                DebugEventType::SystemError => stats.system_errors += 1,
                DebugEventType::Custom(_) => stats.custom_events += 1,
            }
        }
        
        stats
    }
}

/// Estadísticas de eventos
#[derive(Debug, Clone)]
pub struct EventStatistics {
    pub total_events: usize,
    pub low_severity: usize,
    pub medium_severity: usize,
    pub high_severity: usize,
    pub critical_severity: usize,
    pub process_created: usize,
    pub process_terminated: usize,
    pub thread_created: usize,
    pub thread_terminated: usize,
    pub memory_allocated: usize,
    pub memory_freed: usize,
    pub memory_leaks: usize,
    pub system_calls: usize,
    pub interrupts: usize,
    pub exceptions: usize,
    pub performance_issues: usize,
    pub system_errors: usize,
    pub custom_events: usize,
}

impl EventStatistics {
    pub fn new() -> Self {
        Self {
            total_events: 0,
            low_severity: 0,
            medium_severity: 0,
            high_severity: 0,
            critical_severity: 0,
            process_created: 0,
            process_terminated: 0,
            thread_created: 0,
            thread_terminated: 0,
            memory_allocated: 0,
            memory_freed: 0,
            memory_leaks: 0,
            system_calls: 0,
            interrupts: 0,
            exceptions: 0,
            performance_issues: 0,
            system_errors: 0,
            custom_events: 0,
        }
    }
    
    pub fn get_severity_percentage(&self, severity: DebugSeverity) -> f64 {
        if self.total_events == 0 {
            return 0.0;
        }
        
        let count = match severity {
            DebugSeverity::Low => self.low_severity,
            DebugSeverity::Medium => self.medium_severity,
            DebugSeverity::High => self.high_severity,
            DebugSeverity::Critical => self.critical_severity,
            DebugSeverity::Info => self.low_severity, // Info se cuenta como Low
        };
        
        (count as f64 / self.total_events as f64) * 100.0
    }
    
    pub fn get_event_type_percentage(&self, event_type: &DebugEventType) -> f64 {
        if self.total_events == 0 {
            return 0.0;
        }
        
        let count = match event_type {
            DebugEventType::ProcessCreated => self.process_created,
            DebugEventType::ProcessTerminated => self.process_terminated,
            DebugEventType::ThreadCreated => self.thread_created,
            DebugEventType::ThreadTerminated => self.thread_terminated,
            DebugEventType::MemoryAllocated => self.memory_allocated,
            DebugEventType::MemoryFreed => self.memory_freed,
            DebugEventType::MemoryLeak => self.memory_leaks,
            DebugEventType::SystemCall => self.system_calls,
            DebugEventType::Interrupt => self.interrupts,
            DebugEventType::Exception => self.exceptions,
            DebugEventType::PerformanceIssue => self.performance_issues,
            DebugEventType::SystemError => self.system_errors,
            DebugEventType::Custom(_) => self.custom_events,
        };
        
        (count as f64 / self.total_events as f64) * 100.0
    }
}

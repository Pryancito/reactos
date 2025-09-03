//! Funciones comunes para pruebas de estrés
//! 
//! Este módulo contiene funciones y estructuras comunes
//! utilizadas por todas las pruebas de estrés.

use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Utilidades para pruebas de estrés
pub struct StressTestUtils;

impl StressTestUtils {
    /// Formatear duración
    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        let milliseconds = duration.subsec_millis();
        
        if hours > 0 {
            format!("{}h {}m {}s {}ms", hours, minutes, seconds, milliseconds)
        } else if minutes > 0 {
            format!("{}m {}s {}ms", minutes, seconds, milliseconds)
        } else if seconds > 0 {
            format!("{}s {}ms", seconds, milliseconds)
        } else {
            format!("{}ms", milliseconds)
        }
    }
    
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
    
    /// Formatear porcentaje
    pub fn format_percentage(value: f64) -> String {
        format!("{:.2}%", value * 100.0)
    }
    
    /// Formatear número con separadores de miles
    pub fn format_number(number: u64) -> String {
        let mut result = String::new();
        let number_str = number.to_string();
        let chars: Vec<char> = number_str.chars().collect();
        
        for (i, ch) in chars.iter().enumerate() {
            if i > 0 && (chars.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(*ch);
        }
        
        result
    }
    
    /// Formatear throughput
    pub fn format_throughput(operations: u64, duration: Duration) -> String {
        let seconds = duration.as_secs_f64();
        if seconds > 0.0 {
            let ops_per_second = operations as f64 / seconds;
            if ops_per_second >= 1_000_000.0 {
                format!("{:.2}M ops/s", ops_per_second / 1_000_000.0)
            } else if ops_per_second >= 1_000.0 {
                format!("{:.2}K ops/s", ops_per_second / 1_000.0)
            } else {
                format!("{:.2} ops/s", ops_per_second)
            }
        } else {
            "0 ops/s".to_string()
        }
    }
    
    /// Formatear latencia
    pub fn format_latency(nanoseconds: u64) -> String {
        if nanoseconds >= 1_000_000_000 {
            format!("{:.2}s", nanoseconds as f64 / 1_000_000_000.0)
        } else if nanoseconds >= 1_000_000 {
            format!("{:.2}ms", nanoseconds as f64 / 1_000_000.0)
        } else if nanoseconds >= 1_000 {
            format!("{:.2}μs", nanoseconds as f64 / 1_000.0)
        } else {
            format!("{}ns", nanoseconds)
        }
    }
    
    /// Generar datos de prueba
    pub fn generate_test_data(size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);
        for i in 0..size {
            data.push((i % 256) as u8);
        }
        data
    }
    
    /// Generar datos aleatorios
    pub fn generate_random_data(size: usize) -> Vec<u8> {
        let mut data = Vec::with_capacity(size);
        let mut seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        for _ in 0..size {
            // Generador de números pseudoaleatorios simple
            seed = (seed * 1103515245 + 12345) % (1 << 31);
            data.push((seed % 256) as u8);
        }
        
        data
    }
    
    /// Calcular hash simple
    pub fn simple_hash(data: &[u8]) -> u64 {
        let mut hash = 0u64;
        for &byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }
    
    /// Verificar integridad de datos
    pub fn verify_data_integrity(original: &[u8], current: &[u8]) -> bool {
        if original.len() != current.len() {
            return false;
        }
        
        for (a, b) in original.iter().zip(current.iter()) {
            if a != b {
                return false;
            }
        }
        
        true
    }
    
    /// Medir tiempo de ejecución
    pub fn measure_execution_time<F>(f: F) -> (Duration, F::Output)
    where
        F: FnOnce() -> (),
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (duration, result)
    }
    
    /// Medir tiempo de ejecución con retorno de error
    pub fn measure_execution_time_result<F, T, E>(f: F) -> (Duration, Result<T, E>)
    where
        F: FnOnce() -> Result<T, E>,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (duration, result)
    }
    
    /// Calcular estadísticas de una serie de valores
    pub fn calculate_statistics(values: &[f64]) -> Statistics {
        if values.is_empty() {
            return Statistics::new();
        }
        
        let mut sum = 0.0;
        let mut min = values[0];
        let mut max = values[0];
        
        for &value in values {
            sum += value;
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
        }
        
        let mean = sum / values.len() as f64;
        
        let mut variance = 0.0;
        for &value in values {
            let diff = value - mean;
            variance += diff * diff;
        }
        variance /= values.len() as f64;
        let std_dev = variance.sqrt();
        
        let mut sorted_values = values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let median = if sorted_values.len() % 2 == 0 {
            (sorted_values[sorted_values.len() / 2 - 1] + sorted_values[sorted_values.len() / 2]) / 2.0
        } else {
            sorted_values[sorted_values.len() / 2]
        };
        
        Statistics {
            count: values.len(),
            mean,
            median,
            min,
            max,
            std_dev,
            variance,
        }
    }
    
    /// Calcular percentiles
    pub fn calculate_percentiles(values: &[f64], percentiles: &[f64]) -> Vec<(f64, f64)> {
        let mut sorted_values = values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let mut result = Vec::new();
        
        for &percentile in percentiles {
            if percentile < 0.0 || percentile > 100.0 {
                continue;
            }
            
            let index = (percentile / 100.0 * (sorted_values.len() - 1) as f64) as usize;
            let value = if index >= sorted_values.len() {
                sorted_values[sorted_values.len() - 1]
            } else {
                sorted_values[index]
            };
            
            result.push((percentile, value));
        }
        
        result
    }
    
    /// Crear reporte de rendimiento
    pub fn create_performance_report(
        test_name: &str,
        duration: Duration,
        iterations: u64,
        successes: u64,
        failures: u64,
        errors: u64,
        metrics: &HashMap<String, f64>,
    ) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("=== Reporte de Rendimiento: {} ===\n", test_name));
        report.push_str(&format!("Duración: {}\n", Self::format_duration(duration)));
        report.push_str(&format!("Iteraciones: {}\n", Self::format_number(iterations)));
        report.push_str(&format!("Éxitos: {}\n", Self::format_number(successes)));
        report.push_str(&format!("Fallos: {}\n", Self::format_number(failures)));
        report.push_str(&format!("Errores: {}\n", Self::format_number(errors)));
        
        let total_operations = successes + failures + errors;
        if total_operations > 0 {
            let success_rate = (successes as f64 / total_operations as f64) * 100.0;
            report.push_str(&format!("Tasa de éxito: {}\n", Self::format_percentage(success_rate / 100.0)));
        }
        
        if duration.as_secs() > 0 {
            let throughput = iterations as f64 / duration.as_secs_f64();
            report.push_str(&format!("Throughput: {}\n", Self::format_throughput(iterations, duration)));
        }
        
        if !metrics.is_empty() {
            report.push_str("\nMétricas:\n");
            for (key, value) in metrics {
                report.push_str(&format!("  {}: {:.2}\n", key, value));
            }
        }
        
        report
    }
    
    /// Crear reporte de comparación
    pub fn create_comparison_report(
        test_name: &str,
        baseline: &HashMap<String, f64>,
        current: &HashMap<String, f64>,
    ) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("=== Reporte de Comparación: {} ===\n", test_name));
        report.push_str("Métrica\t\tBaseline\tActual\t\tDiferencia\tCambio\n");
        report.push_str("-------\t\t--------\t-------\t\t----------\t------\n");
        
        for (key, baseline_value) in baseline {
            if let Some(current_value) = current.get(key) {
                let difference = current_value - baseline_value;
                let change_percent = if *baseline_value != 0.0 {
                    (difference / baseline_value) * 100.0
                } else {
                    0.0
                };
                
                let change_symbol = if change_percent > 0.0 {
                    "↗"
                } else if change_percent < 0.0 {
                    "↘"
                } else {
                    "→"
                };
                
                report.push_str(&format!(
                    "{}\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t{:.1}% {}\n",
                    key, baseline_value, current_value, difference, change_percent, change_symbol
                ));
            }
        }
        
        report
    }
    
    /// Validar configuración de prueba
    pub fn validate_test_config(config: &crate::TestConfig) -> Result<(), String> {
        if config.test_name.is_empty() {
            return Err("Test name cannot be empty".to_string());
        }
        
        if config.duration.as_secs() == 0 {
            return Err("Test duration must be greater than 0".to_string());
        }
        
        if config.threads == 0 {
            return Err("Number of threads must be greater than 0".to_string());
        }
        
        if let Some(memory_limit) = config.memory_limit {
            if memory_limit == 0 {
                return Err("Memory limit must be greater than 0".to_string());
            }
        }
        
        if let Some(cpu_limit) = config.cpu_limit {
            if cpu_limit <= 0.0 || cpu_limit > 1.0 {
                return Err("CPU limit must be between 0.0 and 1.0".to_string());
            }
        }
        
        if let Some(network_bandwidth) = config.network_bandwidth {
            if network_bandwidth == 0 {
                return Err("Network bandwidth must be greater than 0".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Crear directorio de prueba
    pub fn create_test_directory(base_path: &str, test_name: &str) -> Result<String, String> {
        let test_dir = format!("{}/{}", base_path, test_name);
        std::fs::create_dir_all(&test_dir)
            .map_err(|e| format!("Failed to create test directory: {}", e))?;
        Ok(test_dir)
    }
    
    /// Limpiar directorio de prueba
    pub fn cleanup_test_directory(test_dir: &str) -> Result<(), String> {
        std::fs::remove_dir_all(test_dir)
            .map_err(|e| format!("Failed to cleanup test directory: {}", e))?;
        Ok(())
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
    
    /// Obtener métricas de rendimiento del sistema
    pub fn get_system_performance_metrics() -> SystemPerformanceMetrics {
        let now = Instant::now();
        SystemPerformanceMetrics {
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
}

/// Estadísticas
#[derive(Debug, Clone)]
pub struct Statistics {
    pub count: usize,
    pub mean: f64,
    pub median: f64,
    pub min: f64,
    pub max: f64,
    pub std_dev: f64,
    pub variance: f64,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            median: 0.0,
            min: 0.0,
            max: 0.0,
            std_dev: 0.0,
            variance: 0.0,
        }
    }
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

/// Métricas de rendimiento del sistema
#[derive(Debug, Clone)]
pub struct SystemPerformanceMetrics {
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

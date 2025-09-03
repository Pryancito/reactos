//! Ejecutor de Pruebas de Estrés para ReactOS Rust
//! 
//! Herramienta principal para ejecutar y gestionar
//! todas las pruebas de estrés del sistema.

use reactos_rust_stress_tests::*;
use std::time::Duration;

/// Función principal del ejecutor de pruebas de estrés
fn main() {
    println!("=== Ejecutor de Pruebas de Estrés - ReactOS Rust ===");
    
    // Inicializar gestor de pruebas de estrés
    if let Err(e) = init_stress_test_manager() {
        eprintln!("Error inicializando gestor de pruebas de estrés: {}", e);
        return;
    }
    
    println!("Gestor de pruebas de estrés inicializado");
    
    // Ejecutar suite completa de pruebas de estrés
    run_complete_stress_test_suite();
    
    // Mostrar estadísticas finales
    if let Some(stats) = get_performance_statistics() {
        print_performance_statistics(&stats);
    }
    
    // Limpiar
    if let Some(manager) = get_stress_test_manager() {
        let _ = manager.cleanup();
    }
    
    println!("Ejecutor de pruebas de estrés completado");
}

/// Ejecutar suite completa de pruebas de estrés
fn run_complete_stress_test_suite() {
    println!("\n=== Ejecutando Suite Completa de Pruebas de Estrés ===");
    
    // Prueba de estrés de memoria
    run_memory_stress_test();
    
    // Prueba de estrés de CPU
    run_cpu_stress_test();
    
    // Prueba de estrés de red
    run_network_stress_test();
    
    // Prueba de estrés de concurrencia
    run_concurrency_stress_test();
    
    // Prueba de estrés del sistema de archivos
    run_filesystem_stress_test();
    
    // Prueba de estrés del sistema
    run_system_stress_test();
    
    println!("\n=== Suite de Pruebas de Estrés Completada ===");
}

/// Ejecutar prueba de estrés de memoria
fn run_memory_stress_test() {
    println!("\n--- Ejecutando Prueba de Estrés de Memoria ---");
    
    let config = TestConfig {
        test_type: StressTestType::Memory,
        test_name: "Memory Stress Test".to_string(),
        duration: Duration::from_secs(30),
        iterations: None,
        threads: 4,
        memory_limit: Some(1024 * 1024 * 1024), // 1GB
        cpu_limit: None,
        network_bandwidth: None,
        enable_monitoring: true,
        log_level: LogLevel::Info,
        output_file: None,
    };
    
    match run_stress_test(config) {
        Ok(result) => {
            print_test_result(&result);
        }
        Err(e) => {
            eprintln!("Error ejecutando prueba de estrés de memoria: {}", e);
        }
    }
}

/// Ejecutar prueba de estrés de CPU
fn run_cpu_stress_test() {
    println!("\n--- Ejecutando Prueba de Estrés de CPU ---");
    
    let config = TestConfig {
        test_type: StressTestType::CPU,
        test_name: "CPU Stress Test".to_string(),
        duration: Duration::from_secs(30),
        iterations: None,
        threads: 4,
        memory_limit: None,
        cpu_limit: Some(0.8), // 80% de CPU
        network_bandwidth: None,
        enable_monitoring: true,
        log_level: LogLevel::Info,
        output_file: None,
    };
    
    match run_stress_test(config) {
        Ok(result) => {
            print_test_result(&result);
        }
        Err(e) => {
            eprintln!("Error ejecutando prueba de estrés de CPU: {}", e);
        }
    }
}

/// Ejecutar prueba de estrés de red
fn run_network_stress_test() {
    println!("\n--- Ejecutando Prueba de Estrés de Red ---");
    
    let config = TestConfig {
        test_type: StressTestType::Network,
        test_name: "Network Stress Test".to_string(),
        duration: Duration::from_secs(30),
        iterations: None,
        threads: 4,
        memory_limit: None,
        cpu_limit: None,
        network_bandwidth: Some(100 * 1024 * 1024), // 100MB/s
        enable_monitoring: true,
        log_level: LogLevel::Info,
        output_file: None,
    };
    
    match run_stress_test(config) {
        Ok(result) => {
            print_test_result(&result);
        }
        Err(e) => {
            eprintln!("Error ejecutando prueba de estrés de red: {}", e);
        }
    }
}

/// Ejecutar prueba de estrés de concurrencia
fn run_concurrency_stress_test() {
    println!("\n--- Ejecutando Prueba de Estrés de Concurrencia ---");
    
    let config = TestConfig {
        test_type: StressTestType::Concurrency,
        test_name: "Concurrency Stress Test".to_string(),
        duration: Duration::from_secs(30),
        iterations: None,
        threads: 8,
        memory_limit: None,
        cpu_limit: None,
        network_bandwidth: None,
        enable_monitoring: true,
        log_level: LogLevel::Info,
        output_file: None,
    };
    
    match run_stress_test(config) {
        Ok(result) => {
            print_test_result(&result);
        }
        Err(e) => {
            eprintln!("Error ejecutando prueba de estrés de concurrencia: {}", e);
        }
    }
}

/// Ejecutar prueba de estrés del sistema de archivos
fn run_filesystem_stress_test() {
    println!("\n--- Ejecutando Prueba de Estrés del Sistema de Archivos ---");
    
    let config = TestConfig {
        test_type: StressTestType::Filesystem,
        test_name: "Filesystem Stress Test".to_string(),
        duration: Duration::from_secs(30),
        iterations: None,
        threads: 4,
        memory_limit: None,
        cpu_limit: None,
        network_bandwidth: None,
        enable_monitoring: true,
        log_level: LogLevel::Info,
        output_file: None,
    };
    
    match run_stress_test(config) {
        Ok(result) => {
            print_test_result(&result);
        }
        Err(e) => {
            eprintln!("Error ejecutando prueba de estrés del sistema de archivos: {}", e);
        }
    }
}

/// Ejecutar prueba de estrés del sistema
fn run_system_stress_test() {
    println!("\n--- Ejecutando Prueba de Estrés del Sistema ---");
    
    let config = TestConfig {
        test_type: StressTestType::System,
        test_name: "System Stress Test".to_string(),
        duration: Duration::from_secs(30),
        iterations: None,
        threads: 4,
        memory_limit: Some(512 * 1024 * 1024), // 512MB
        cpu_limit: Some(0.7), // 70% de CPU
        network_bandwidth: Some(50 * 1024 * 1024), // 50MB/s
        enable_monitoring: true,
        log_level: LogLevel::Info,
        output_file: None,
    };
    
    match run_stress_test(config) {
        Ok(result) => {
            print_test_result(&result);
        }
        Err(e) => {
            eprintln!("Error ejecutando prueba de estrés del sistema: {}", e);
        }
    }
}

/// Imprimir resultado de prueba
fn print_test_result(result: &TestResult) {
    println!("Resultado de la prueba: {}", result.test_name);
    println!("  Tipo: {:?}", result.test_type);
    println!("  Estado: {:?}", result.state);
    
    if let Some(duration) = result.duration {
        println!("  Duración: {}", common::StressTestUtils::format_duration(duration));
    }
    
    println!("  Iteraciones: {}", common::StressTestUtils::format_number(result.iterations));
    println!("  Éxitos: {}", common::StressTestUtils::format_number(result.success_count));
    println!("  Fallos: {}", common::StressTestUtils::format_number(result.failure_count));
    println!("  Errores: {}", common::StressTestUtils::format_number(result.error_count));
    
    let total_operations = result.success_count + result.failure_count + result.error_count;
    if total_operations > 0 {
        let success_rate = (result.success_count as f64 / total_operations as f64) * 100.0;
        println!("  Tasa de éxito: {:.2}%", success_rate);
    }
    
    if let Some(duration) = result.duration {
        if duration.as_secs() > 0 {
            let throughput = result.iterations as f64 / duration.as_secs_f64();
            println!("  Throughput: {:.2} ops/s", throughput);
        }
    }
    
    if !result.metrics.is_empty() {
        println!("  Métricas:");
        for (key, value) in &result.metrics {
            println!("    {}: {:.2}", key, value);
        }
    }
    
    if !result.errors.is_empty() {
        println!("  Errores:");
        for error in &result.errors {
            println!("    - {}", error);
        }
    }
    
    if !result.warnings.is_empty() {
        println!("  Advertencias:");
        for warning in &result.warnings {
            println!("    - {}", warning);
        }
    }
}

/// Imprimir estadísticas de rendimiento
fn print_performance_statistics(stats: &PerformanceStatistics) {
    println!("\n=== Estadísticas de Rendimiento ===");
    println!("Total de pruebas: {}", stats.total_tests);
    println!("Pruebas completadas: {}", stats.completed_tests);
    println!("Pruebas fallidas: {}", stats.failed_tests);
    println!("Pruebas canceladas: {}", stats.cancelled_tests);
    println!("Total de iteraciones: {}", common::StressTestUtils::format_number(stats.total_iterations));
    println!("Total de éxitos: {}", common::StressTestUtils::format_number(stats.total_successes));
    println!("Total de fallos: {}", common::StressTestUtils::format_number(stats.total_failures));
    println!("Total de errores: {}", common::StressTestUtils::format_number(stats.total_errors));
    println!("Duración total: {}", common::StressTestUtils::format_duration(stats.total_duration));
    println!("Duración promedio: {}", common::StressTestUtils::format_duration(stats.average_duration));
    
    if let Some(min_duration) = stats.min_duration {
        println!("Duración mínima: {}", common::StressTestUtils::format_duration(min_duration));
    }
    
    println!("Duración máxima: {}", common::StressTestUtils::format_duration(stats.max_duration));
    println!("Tasa de éxito: {:.2}%", stats.success_rate);
    
    if stats.total_tests > 0 {
        let completion_rate = (stats.completed_tests as f64 / stats.total_tests as f64) * 100.0;
        println!("Tasa de finalización: {:.2}%", completion_rate);
    }
}

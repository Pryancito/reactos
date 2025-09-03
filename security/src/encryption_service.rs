//! Servicio de Cifrado para ReactOS Rust
//! 
//! Servicio dedicado para manejar el cifrado
//! y descifrado de datos del sistema.

use reactos_rust_security::*;
// use std::collections::HashMap;

/// Función principal del servicio de cifrado
fn main() {
    println!("=== Servicio de Cifrado - ReactOS Rust ===");
    
    // Inicializar gestor de seguridad
    if let Err(e) = init_security_manager() {
        eprintln!("Error inicializando gestor de seguridad: {}", e);
        return;
    }
    
    println!("Servicio de cifrado inicializado");
    
    // Ejecutar demostración del servicio de cifrado
    run_encryption_demonstration();
    
    // Mostrar estadísticas de cifrado
    if let Some(manager) = get_security_manager() {
        print_encryption_statistics(manager);
    }
    
    println!("Servicio de cifrado completado");
}

/// Ejecutar demostración del servicio de cifrado
fn run_encryption_demonstration() {
    println!("\n=== Demostración del Servicio de Cifrado ===");
    
    // Demostrar diferentes tipos de cifrado
    demonstrate_basic_encryption();
    demonstrate_large_data_encryption();
    demonstrate_multiple_keys();
    demonstrate_encryption_performance();
    demonstrate_encryption_algorithms();
    
    println!("\n=== Demostración Completada ===");
}

/// Demostrar cifrado básico
fn demonstrate_basic_encryption() {
    println!("\n--- Demostrando Cifrado Básico ---");
    
    let test_cases = vec![
        ("Hello, World!", "key123"),
        ("ReactOS Rust Security", "secretkey"),
        ("Sensitive Data", "password"),
        ("", "emptykey"), // Datos vacíos
    ];
    
    for (data, key) in test_cases {
        let data_bytes = data.as_bytes();
        let key_bytes = key.as_bytes();
        
        println!("📝 Cifrando: '{}' con clave: '{}'", data, key);
        
        // Cifrar datos
        match encrypt_data(data_bytes, key_bytes) {
            Ok(encrypted) => {
                println!("✅ Datos cifrados exitosamente");
                println!("   Tamaño original: {} bytes", data_bytes.len());
                println!("   Tamaño cifrado: {} bytes", encrypted.len());
                println!("   Datos cifrados: {:?}", encrypted);
                
                // Descifrar datos
                match decrypt_data(&encrypted, key_bytes) {
                    Ok(decrypted) => {
                        println!("✅ Datos descifrados exitosamente");
                        println!("   Datos descifrados: {:?}", decrypted);
                        
                        // Verificar integridad
                        if data_bytes == decrypted.as_slice() {
                            println!("✅ Integridad de datos verificada");
                        } else {
                            println!("❌ Error en la integridad de datos");
                            println!("   Original: {:?}", data_bytes);
                            println!("   Descifrado: {:?}", decrypted);
                        }
                    }
                    Err(e) => {
                        println!("❌ Error al descifrar datos: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Error al cifrar datos: {}", e);
            }
        }
        
        println!();
    }
}

/// Demostrar cifrado de datos grandes
fn demonstrate_large_data_encryption() {
    println!("\n--- Demostrando Cifrado de Datos Grandes ---");
    
    // Generar datos de diferentes tamaños
    let data_sizes = vec![1024, 10240, 102400, 1024000]; // 1KB, 10KB, 100KB, 1MB
    
    for size in data_sizes {
        let data = generate_test_data(size);
        let key = b"large_data_key";
        
        println!("📊 Cifrando datos de {} bytes", size);
        
        let start_time = std::time::Instant::now();
        
        // Cifrar datos
        match encrypt_data(&data, key) {
            Ok(encrypted) => {
                let encrypt_time = start_time.elapsed();
                
                let decrypt_start = std::time::Instant::now();
                
                // Descifrar datos
                match decrypt_data(&encrypted, key) {
                    Ok(decrypted) => {
                        let decrypt_time = decrypt_start.elapsed();
                        let total_time = start_time.elapsed();
                        
                        println!("✅ Cifrado y descifrado exitosos");
                        println!("   Tiempo de cifrado: {:?}", encrypt_time);
                        println!("   Tiempo de descifrado: {:?}", decrypt_time);
                        println!("   Tiempo total: {:?}", total_time);
                        
                        // Calcular throughput
                        let encrypt_throughput = (size as f64 / encrypt_time.as_secs_f64()) / 1024.0; // KB/s
                        let decrypt_throughput = (size as f64 / decrypt_time.as_secs_f64()) / 1024.0; // KB/s
                        
                        println!("   Throughput de cifrado: {:.2} KB/s", encrypt_throughput);
                        println!("   Throughput de descifrado: {:.2} KB/s", decrypt_throughput);
                        
                        // Verificar integridad
                        if data == decrypted {
                            println!("✅ Integridad de datos verificada");
                        } else {
                            println!("❌ Error en la integridad de datos");
                        }
                    }
                    Err(e) => {
                        println!("❌ Error al descifrar datos: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Error al cifrar datos: {}", e);
            }
        }
        
        println!();
    }
}

/// Demostrar cifrado con múltiples claves
fn demonstrate_multiple_keys() {
    println!("\n--- Demostrando Cifrado con Múltiples Claves ---");
    
    let data = b"Data to be encrypted with multiple keys";
    let keys = vec![
        b"key1",
        b"key2",
        b"key3",
        b"very_long_key_for_encryption".as_slice(),
        b"short",
    ];
    
    println!("📝 Datos originales: {:?}", String::from_utf8_lossy(data));
    
    for (i, key) in keys.iter().enumerate() {
        println!("🔑 Usando clave {}: '{}'", i + 1, String::from_utf8_lossy(key));
        
        // Cifrar con esta clave
        match encrypt_data(data, key) {
            Ok(encrypted) => {
                println!("✅ Cifrado exitoso con clave {}", i + 1);
                println!("   Datos cifrados: {:?}", encrypted);
                
                // Descifrar con la misma clave
                match decrypt_data(&encrypted, key) {
                    Ok(decrypted) => {
                        println!("✅ Descifrado exitoso con clave {}", i + 1);
                        
                        // Verificar integridad
                        if data == decrypted.as_slice() {
                            println!("✅ Integridad verificada con clave {}", i + 1);
                        } else {
                            println!("❌ Error de integridad con clave {}", i + 1);
                        }
                    }
                    Err(e) => {
                        println!("❌ Error al descifrar con clave {}: {}", i + 1, e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Error al cifrar con clave {}: {}", i + 1, e);
            }
        }
        
        println!();
    }
}

/// Demostrar rendimiento del cifrado
fn demonstrate_encryption_performance() {
    println!("\n--- Demostrando Rendimiento del Cifrado ---");
    
    let data_size = 1024 * 1024; // 1MB
    let data = generate_test_data(data_size);
    let key = b"performance_test_key";
    
    println!("📊 Prueba de rendimiento con datos de {} bytes", data_size);
    
    // Ejecutar múltiples iteraciones para obtener un promedio
    let iterations = 10;
    let mut encrypt_times = Vec::new();
    let mut decrypt_times = Vec::new();
    
    for i in 1..=iterations {
        println!("🔄 Iteración {}/{}", i, iterations);
        
        // Medir tiempo de cifrado
        let start_time = std::time::Instant::now();
        match encrypt_data(&data, key) {
            Ok(encrypted) => {
                let encrypt_time = start_time.elapsed();
                encrypt_times.push(encrypt_time);
                
                // Medir tiempo de descifrado
                let decrypt_start = std::time::Instant::now();
                match decrypt_data(&encrypted, key) {
                    Ok(_) => {
                        let decrypt_time = decrypt_start.elapsed();
                        decrypt_times.push(decrypt_time);
                        
                        println!("   Cifrado: {:?}, Descifrado: {:?}", encrypt_time, decrypt_time);
                    }
                    Err(e) => {
                        println!("❌ Error en descifrado: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Error en cifrado: {}", e);
            }
        }
    }
    
    // Calcular estadísticas
    if !encrypt_times.is_empty() && !decrypt_times.is_empty() {
        let avg_encrypt_time = calculate_average_duration(&encrypt_times);
        let avg_decrypt_time = calculate_average_duration(&decrypt_times);
        
        let min_encrypt_time = encrypt_times.iter().min().unwrap();
        let max_encrypt_time = encrypt_times.iter().max().unwrap();
        let min_decrypt_time = decrypt_times.iter().min().unwrap();
        let max_decrypt_time = decrypt_times.iter().max().unwrap();
        
        println!("📈 Estadísticas de rendimiento:");
        println!("   Tiempo promedio de cifrado: {:?}", avg_encrypt_time);
        println!("   Tiempo promedio de descifrado: {:?}", avg_decrypt_time);
        println!("   Tiempo mínimo de cifrado: {:?}", min_encrypt_time);
        println!("   Tiempo máximo de cifrado: {:?}", max_encrypt_time);
        println!("   Tiempo mínimo de descifrado: {:?}", min_decrypt_time);
        println!("   Tiempo máximo de descifrado: {:?}", max_decrypt_time);
        
        // Calcular throughput promedio
        let avg_encrypt_throughput = (data_size as f64 / avg_encrypt_time.as_secs_f64()) / 1024.0 / 1024.0; // MB/s
        let avg_decrypt_throughput = (data_size as f64 / avg_decrypt_time.as_secs_f64()) / 1024.0 / 1024.0; // MB/s
        
        println!("   Throughput promedio de cifrado: {:.2} MB/s", avg_encrypt_throughput);
        println!("   Throughput promedio de descifrado: {:.2} MB/s", avg_decrypt_throughput);
    }
}

/// Demostrar diferentes algoritmos de cifrado
fn demonstrate_encryption_algorithms() {
    println!("\n--- Demostrando Algoritmos de Cifrado ---");
    
    let data = b"Test data for encryption algorithm comparison";
    let key = b"algorithm_test_key";
    
    println!("📝 Datos de prueba: {:?}", String::from_utf8_lossy(data));
    
    // Simular diferentes algoritmos (en una implementación real, estos serían algoritmos diferentes)
    let algorithms = vec![
        ("XOR Simple", "Cifrado XOR básico"),
        ("XOR Mejorado", "Cifrado XOR con rotación"),
        ("XOR Avanzado", "Cifrado XOR con múltiples pasadas"),
    ];
    
    for (algo_name, algo_desc) in algorithms {
        println!("🔐 Algoritmo: {} - {}", algo_name, algo_desc);
        
        let start_time = std::time::Instant::now();
        
        // Cifrar con este algoritmo simulado
        match encrypt_data(data, key) {
            Ok(encrypted) => {
                let encrypt_time = start_time.elapsed();
                
                let decrypt_start = std::time::Instant::now();
                
                // Descifrar
                match decrypt_data(&encrypted, key) {
                    Ok(decrypted) => {
                        let decrypt_time = decrypt_start.elapsed();
                        
                        println!("✅ Cifrado y descifrado exitosos");
                        println!("   Tiempo de cifrado: {:?}", encrypt_time);
                        println!("   Tiempo de descifrado: {:?}", decrypt_time);
                        
                        // Verificar integridad
                        if data == decrypted.as_slice() {
                            println!("✅ Integridad verificada");
                        } else {
                            println!("❌ Error de integridad");
                        }
                    }
                    Err(e) => {
                        println!("❌ Error en descifrado: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Error en cifrado: {}", e);
            }
        }
        
        println!();
    }
}

/// Generar datos de prueba
fn generate_test_data(size: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(size);
    for i in 0..size {
        data.push((i % 256) as u8);
    }
    data
}

/// Calcular duración promedio
fn calculate_average_duration(durations: &[std::time::Duration]) -> std::time::Duration {
    let total_nanos: u128 = durations.iter().map(|d| d.as_nanos()).sum();
    let avg_nanos = total_nanos / durations.len() as u128;
    std::time::Duration::from_nanos(avg_nanos as u64)
}

/// Imprimir estadísticas de cifrado
fn print_encryption_statistics(manager: &SecurityManager) {
    println!("\n=== Estadísticas de Cifrado ===");
    
    // Configuración de cifrado
    println!("🔐 Configuración de cifrado:");
    println!("   Cifrado habilitado: {}", manager.config.enable_encryption);
    println!("   Algoritmo: {:?}", manager.config.encryption_algorithm);
    
    // Estadísticas de eventos de cifrado
    let encryption_events: Vec<_> = manager.get_security_events()
        .iter()
        .filter(|e| e.event_type == SecurityEventType::Encryption || e.event_type == SecurityEventType::Decryption)
        .collect();
    
    println!("📊 Estadísticas de eventos de cifrado:");
    println!("   Total de eventos: {}", encryption_events.len());
    
    let encrypt_events = encryption_events.iter().filter(|e| e.event_type == SecurityEventType::Encryption).count();
    let decrypt_events = encryption_events.iter().filter(|e| e.event_type == SecurityEventType::Decryption).count();
    
    println!("   Eventos de cifrado: {}", encrypt_events);
    println!("   Eventos de descifrado: {}", decrypt_events);
    
    // Eventos por severidad
    let mut severity_counts = std::collections::HashMap::new();
    for event in &encryption_events {
        *severity_counts.entry(event.severity).or_insert(0) += 1;
    }
    
    println!("   Eventos por severidad:");
    for (severity, count) in severity_counts {
        println!("     {:?}: {} eventos", severity, count);
    }
    
    // Eventos exitosos vs fallidos
    let successful_events = encryption_events.iter().filter(|e| e.success).count();
    let failed_events = encryption_events.len() - successful_events;
    
    println!("   Eventos exitosos: {}", successful_events);
    println!("   Eventos fallidos: {}", failed_events);
    
    if encryption_events.len() > 0 {
        let success_rate = (successful_events as f64 / encryption_events.len() as f64) * 100.0;
        println!("   Tasa de éxito: {:.2}%", success_rate);
    }
    
    // Recomendaciones de seguridad
    println!("💡 Recomendaciones de cifrado:");
    
    if !manager.config.enable_encryption {
        println!("   ⚠️  El cifrado está deshabilitado - considera habilitarlo para proteger datos sensibles");
    }
    
    match manager.config.encryption_algorithm {
        EncryptionAlgorithm::AES128 => {
            println!("   ℹ️  AES-128 es adecuado para la mayoría de aplicaciones");
        }
        EncryptionAlgorithm::AES256 => {
            println!("   ✅ AES-256 proporciona excelente seguridad");
        }
        EncryptionAlgorithm::RSA2048 => {
            println!("   ⚠️  RSA-2048 puede ser vulnerable a ataques futuros - considera RSA-4096");
        }
        EncryptionAlgorithm::RSA4096 => {
            println!("   ✅ RSA-4096 proporciona excelente seguridad");
        }
        _ => {
            println!("   ℹ️  Algoritmo de cifrado configurado: {:?}", manager.config.encryption_algorithm);
        }
    }
    
    if failed_events > 0 {
        println!("   ⚠️  Hay {} eventos de cifrado fallidos que requieren investigación", failed_events);
    }
    
    // Verificar frecuencia de eventos
    if encryption_events.len() > 1000 {
        println!("   ℹ️  Alta frecuencia de eventos de cifrado - considera optimizar el rendimiento");
    }
    
    // Verificar eventos críticos
    let critical_events = encryption_events.iter().filter(|e| e.severity == SecuritySeverity::Critical).count();
    if critical_events > 0 {
        println!("   🚨 Hay {} eventos críticos de cifrado que requieren atención inmediata", critical_events);
    }
}

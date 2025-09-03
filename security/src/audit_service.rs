//! Servicio de Auditoría para ReactOS Rust
//! 
//! Servicio dedicado para manejar la auditoría
//! y registro de eventos de seguridad.

use reactos_rust_security::*;
use std::collections::HashMap;

/// Función principal del servicio de auditoría
fn main() {
    println!("=== Servicio de Auditoría - ReactOS Rust ===");
    
    // Inicializar gestor de seguridad
    if let Err(e) = init_security_manager() {
        eprintln!("Error inicializando gestor de seguridad: {}", e);
        return;
    }
    
    println!("Servicio de auditoría inicializado");
    
    // Ejecutar demostración del servicio de auditoría
    run_audit_demonstration();
    
    // Mostrar estadísticas de auditoría
    if let Some(manager) = get_security_manager() {
        print_audit_statistics(manager);
    }
    
    println!("Servicio de auditoría completado");
}

/// Ejecutar demostración del servicio de auditoría
fn run_audit_demonstration() {
    println!("\n=== Demostración del Servicio de Auditoría ===");
    
    // Demostrar diferentes tipos de auditoría
    demonstrate_authentication_audit();
    demonstrate_authorization_audit();
    demonstrate_encryption_audit();
    demonstrate_system_audit();
    demonstrate_audit_analysis();
    
    println!("\n=== Demostración Completada ===");
}

/// Demostrar auditoría de autenticación
fn demonstrate_authentication_audit() {
    println!("\n--- Demostrando Auditoría de Autenticación ---");
    
    // Simular eventos de autenticación
    let auth_events = vec![
        ("admin", "admin123", "192.168.1.100", true),
        ("admin", "wrongpass", "192.168.1.101", false),
        ("user1", "password", "192.168.1.102", true),
        ("admin", "wrongpass", "192.168.1.100", false),
    ];
    
    for (username, password, ip, expected_success) in auth_events {
        match authenticate_user(username, password, ip) {
            Ok(result) => {
                println!("✅ Evento de autenticación registrado: {}", result.message);
                if !expected_success {
                    println!("   ⚠️  Resultado inesperado: se esperaba fallo");
                }
            }
            Err(e) => {
                println!("❌ Evento de autenticación registrado: {}", e);
                if expected_success {
                    println!("   ⚠️  Resultado inesperado: se esperaba éxito");
                }
            }
        }
    }
}

/// Demostrar auditoría de autorización
fn demonstrate_authorization_audit() {
    println!("\n--- Demostrando Auditoría de Autorización ---");
    
    // Simular eventos de autorización
    let authz_events = vec![
        ("admin", "system", "admin"),
        ("admin", "user", "manage"),
        ("user1", "files", "read"),
        ("user1", "system", "admin"),
    ];
    
    for (user_id, resource, action) in authz_events {
        match authorize_access(user_id, resource, action) {
            Ok(result) => {
                println!("✅ Evento de autorización registrado: {}", result.message);
            }
            Err(e) => {
                println!("❌ Evento de autorización registrado: {}", e);
            }
        }
    }
}

/// Demostrar auditoría de cifrado
fn demonstrate_encryption_audit() {
    println!("\n--- Demostrando Auditoría de Cifrado ---");
    
    let test_data = b"Sensitive data for encryption audit";
    let key = b"audit_test_key";
    
    // Cifrar datos
    match encrypt_data(test_data, key) {
        Ok(encrypted) => {
            println!("✅ Evento de cifrado registrado");
            println!("   Datos originales: {} bytes", test_data.len());
            println!("   Datos cifrados: {} bytes", encrypted.len());
            
            // Descifrar datos
            match decrypt_data(&encrypted, key) {
                Ok(decrypted) => {
                    println!("✅ Evento de descifrado registrado");
                    println!("   Datos descifrados: {} bytes", decrypted.len());
                    
                    if test_data == decrypted.as_slice() {
                        println!("✅ Integridad de datos verificada");
                    } else {
                        println!("❌ Error en la integridad de datos");
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
}

/// Demostrar auditoría del sistema
fn demonstrate_system_audit() {
    println!("\n--- Demostrando Auditoría del Sistema ---");
    
    if let Some(manager) = get_security_manager() {
        // Simular eventos del sistema
        let system_events = vec![
            ("System startup", SecurityEventType::SystemAccess, SecuritySeverity::Low),
            ("Configuration change", SecurityEventType::ConfigurationChange, SecuritySeverity::Medium),
            ("Policy violation", SecurityEventType::PolicyViolation, SecuritySeverity::High),
            ("Intrusion attempt", SecurityEventType::IntrusionAttempt, SecuritySeverity::Critical),
        ];
        
        for (description, event_type, severity) in system_events {
            let event = SecurityEvent {
                event_id: manager.generate_event_id(),
                event_type,
                severity,
                timestamp: std::time::Instant::now(),
                source: "AuditService".to_string(),
                target: "System".to_string(),
                description: description.to_string(),
                details: HashMap::new(),
                user_id: None,
                session_id: None,
                ip_address: None,
                success: true,
            };
            
            manager.log_security_event(event);
            println!("📝 Evento del sistema registrado: {}", description);
        }
    }
}

/// Demostrar análisis de auditoría
fn demonstrate_audit_analysis() {
    println!("\n--- Demostrando Análisis de Auditoría ---");
    
    if let Some(manager) = get_security_manager() {
        let events = manager.get_security_events();
        
        // Análisis por tipo de evento
        let mut event_type_counts = HashMap::new();
        for event in events {
            *event_type_counts.entry(event.event_type).or_insert(0) += 1;
        }
        
        println!("📊 Análisis por tipo de evento:");
        for (event_type, count) in event_type_counts {
            println!("   {:?}: {} eventos", event_type, count);
        }
        
        // Análisis por severidad
        let mut severity_counts = HashMap::new();
        for event in events {
            *severity_counts.entry(event.severity).or_insert(0) += 1;
        }
        
        println!("📈 Análisis por severidad:");
        for (severity, count) in severity_counts {
            println!("   {:?}: {} eventos", severity, count);
        }
        
        // Análisis de eventos exitosos vs fallidos
        let successful_events = events.iter().filter(|e| e.success).count();
        let failed_events = events.len() - successful_events;
        
        println!("✅ Análisis de éxito/fallo:");
        println!("   Eventos exitosos: {}", successful_events);
        println!("   Eventos fallidos: {}", failed_events);
        
        if events.len() > 0 {
            let success_rate = (successful_events as f64 / events.len() as f64) * 100.0;
            println!("   Tasa de éxito: {:.2}%", success_rate);
        }
        
        // Análisis temporal (últimos eventos)
        let recent_events: Vec<_> = events.iter().rev().take(5).collect();
        println!("⏰ Últimos 5 eventos:");
        for (i, event) in recent_events.iter().enumerate() {
            println!("   {}. [{}] {} - {}", 
                i + 1,
                format!("{:?}", event.severity),
                format!("{:?}", event.event_type),
                event.description
            );
        }
    }
}

/// Imprimir estadísticas de auditoría
fn print_audit_statistics(manager: &SecurityManager) {
    println!("\n=== Estadísticas de Auditoría ===");
    
    // Configuración de auditoría
    println!("🔍 Configuración de auditoría:");
    println!("   Auditoría habilitada: {}", manager.config.enable_audit);
    println!("   Nivel de auditoría: {:?}", manager.config.audit_level);
    println!("   Retención de logs: {} días", manager.config.log_retention_days);
    
    // Estadísticas generales
    let events = manager.get_security_events();
    println!("📊 Estadísticas generales:");
    println!("   Total de eventos: {}", events.len());
    
    // Estadísticas por tipo de evento
    let mut event_type_counts = HashMap::new();
    for event in events {
        *event_type_counts.entry(event.event_type).or_insert(0) += 1;
    }
    
    println!("📋 Eventos por tipo:");
    for (event_type, count) in event_type_counts {
        println!("   {:?}: {} eventos", event_type, count);
    }
    
    // Estadísticas por severidad
    let mut severity_counts = HashMap::new();
    for event in events {
        *severity_counts.entry(event.severity).or_insert(0) += 1;
    }
    
    println!("🚨 Eventos por severidad:");
    for (severity, count) in severity_counts {
        println!("   {:?}: {} eventos", severity, count);
    }
    
    // Estadísticas de éxito/fallo
    let successful_events = events.iter().filter(|e| e.success).count();
    let failed_events = events.len() - successful_events;
    
    println!("✅ Estadísticas de éxito/fallo:");
    println!("   Eventos exitosos: {}", successful_events);
    println!("   Eventos fallidos: {}", failed_events);
    
    if events.len() > 0 {
        let success_rate = (successful_events as f64 / events.len() as f64) * 100.0;
        println!("   Tasa de éxito: {:.2}%", success_rate);
    }
    
    // Análisis de fuentes
    let mut source_counts = HashMap::new();
    for event in events {
        *source_counts.entry(&event.source).or_insert(0) += 1;
    }
    
    println!("🔍 Eventos por fuente:");
    for (source, count) in source_counts {
        println!("   {}: {} eventos", source, count);
    }
    
    // Análisis de usuarios
    let mut user_counts = HashMap::new();
    for event in events {
        if let Some(user_id) = &event.user_id {
            *user_counts.entry(user_id).or_insert(0) += 1;
        }
    }
    
    if !user_counts.is_empty() {
        println!("👥 Eventos por usuario:");
        for (user_id, count) in user_counts {
            println!("   {}: {} eventos", user_id, count);
        }
    }
    
    // Análisis de IPs
    let mut ip_counts = HashMap::new();
    for event in events {
        if let Some(ip_address) = &event.ip_address {
            *ip_counts.entry(ip_address).or_insert(0) += 1;
        }
    }
    
    if !ip_counts.is_empty() {
        println!("🌐 Eventos por IP:");
        for (ip_address, count) in ip_counts {
            println!("   {}: {} eventos", ip_address, count);
        }
    }
    
    // Recomendaciones de auditoría
    println!("💡 Recomendaciones de auditoría:");
    
    if !manager.config.enable_audit {
        println!("   ⚠️  La auditoría está deshabilitada - considera habilitarla para cumplimiento");
    }
    
    match manager.config.audit_level {
        AuditLevel::None => {
            println!("   ⚠️  Nivel de auditoría: None - considera habilitar auditoría básica");
        }
        AuditLevel::Minimal => {
            println!("   ℹ️  Nivel de auditoría: Minimal - considera aumentar a Standard");
        }
        AuditLevel::Standard => {
            println!("   ✅ Nivel de auditoría: Standard - adecuado para la mayoría de casos");
        }
        AuditLevel::Detailed => {
            println!("   ✅ Nivel de auditoría: Detailed - proporciona información detallada");
        }
        AuditLevel::Comprehensive => {
            println!("   ✅ Nivel de auditoría: Comprehensive - máximo nivel de auditoría");
        }
    }
    
    if manager.config.log_retention_days < 30 {
        println!("   ⚠️  Retención de logs: {} días - considera aumentar a 30 días mínimo", manager.config.log_retention_days);
    }
    
    // Verificar eventos críticos
    let critical_events = events.iter().filter(|e| e.severity == SecuritySeverity::Critical).count();
    if critical_events > 0 {
        println!("   🚨 Hay {} eventos críticos que requieren atención inmediata", critical_events);
    }
    
    let high_events = events.iter().filter(|e| e.severity == SecuritySeverity::High).count();
    if high_events > 5 {
        println!("   ⚠️  Hay {} eventos de alta severidad que requieren revisión", high_events);
    }
    
    // Verificar patrones sospechosos
    let failed_auth_events = events.iter()
        .filter(|e| e.event_type == SecurityEventType::Authentication && !e.success)
        .count();
    
    if failed_auth_events > 10 {
        println!("   ⚠️  Hay {} eventos de autenticación fallida - revisar posibles ataques", failed_auth_events);
    }
    
    let intrusion_events = events.iter()
        .filter(|e| e.event_type == SecurityEventType::IntrusionAttempt)
        .count();
    
    if intrusion_events > 0 {
        println!("   🚨 Hay {} intentos de intrusión detectados", intrusion_events);
    }
}

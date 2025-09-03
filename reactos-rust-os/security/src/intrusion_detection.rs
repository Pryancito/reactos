//! Sistema de Detección de Intrusiones para ReactOS Rust
//! 
//! Sistema dedicado para detectar y responder
//! a intentos de intrusión y amenazas de seguridad.

use reactos_rust_security::*;
use std::collections::HashMap;

/// Función principal del sistema de detección de intrusiones
fn main() {
    println!("=== Sistema de Detección de Intrusiones - ReactOS Rust ===");
    
    // Inicializar gestor de seguridad
    if let Err(e) = init_security_manager() {
        eprintln!("Error inicializando gestor de seguridad: {}", e);
        return;
    }
    
    println!("Sistema de detección de intrusiones inicializado");
    
    // Ejecutar demostración del sistema de detección de intrusiones
    run_intrusion_detection_demonstration();
    
    // Mostrar estadísticas de detección de intrusiones
    if let Some(manager) = get_security_manager() {
        print_intrusion_detection_statistics(manager);
    }
    
    println!("Sistema de detección de intrusiones completado");
}

/// Ejecutar demostración del sistema de detección de intrusiones
fn run_intrusion_detection_demonstration() {
    println!("\n=== Demostración del Sistema de Detección de Intrusiones ===");
    
    // Demostrar diferentes tipos de detección
    demonstrate_brute_force_detection();
    demonstrate_anomaly_detection();
    demonstrate_pattern_detection();
    demonstrate_threat_response();
    demonstrate_intrusion_analysis();
    
    println!("\n=== Demostración Completada ===");
}

/// Demostrar detección de ataques de fuerza bruta
fn demonstrate_brute_force_detection() {
    println!("\n--- Demostrando Detección de Ataques de Fuerza Bruta ---");
    
    // Simular múltiples intentos de login fallidos
    let username = "admin";
    let ip_address = "192.168.1.100";
    let wrong_passwords = vec![
        "password", "123456", "admin", "root", "test",
        "guest", "user", "login", "pass", "secret"
    ];
    
    println!("🔍 Simulando ataques de fuerza bruta desde {} para usuario {}", ip_address, username);
    
    for (i, password) in wrong_passwords.iter().enumerate() {
        match authenticate_user(username, password, ip_address) {
            Ok(result) => {
                println!("✅ Intento {}: Autenticación exitosa (inesperado)", i + 1);
            }
            Err(e) => {
                println!("❌ Intento {}: Autenticación fallida: {}", i + 1, e);
                
                // Detectar patrón de fuerza bruta
                if i >= 4 { // Después de 5 intentos fallidos
                    detect_brute_force_attack(username, ip_address, (i + 1) as u32);
                }
            }
        }
    }
}

/// Detectar ataque de fuerza bruta
fn detect_brute_force_attack(username: &str, ip_address: &str, attempt_count: u32) {
    println!("🚨 ATAQUE DE FUERZA BRUTA DETECTADO!");
    println!("   Usuario objetivo: {}", username);
    println!("   IP atacante: {}", ip_address);
    println!("   Número de intentos: {}", attempt_count);
    println!("   Severidad: CRÍTICA");
    
    // Simular respuesta automática
    respond_to_brute_force_attack(username, ip_address);
}

/// Responder a ataque de fuerza bruta
fn respond_to_brute_force_attack(username: &str, ip_address: &str) {
    println!("🛡️  Ejecutando respuesta automática:");
    println!("   1. Bloqueando IP: {}", ip_address);
    println!("   2. Bloqueando cuenta: {}", username);
    println!("   3. Enviando alerta a administradores");
    println!("   4. Registrando evento crítico");
    
    // Simular bloqueo de IP
    block_ip_address(ip_address);
    
    // Simular bloqueo de cuenta
    block_user_account(username);
    
    // Simular envío de alerta
    send_security_alert("Brute Force Attack", username, ip_address);
}

/// Demostrar detección de anomalías
fn demonstrate_anomaly_detection() {
    println!("\n--- Demostrando Detección de Anomalías ---");
    
    // Simular comportamiento normal vs anómalo
    let normal_ips = vec!["192.168.1.100", "192.168.1.101", "192.168.1.102"];
    let anomalous_ips = vec!["10.0.0.1", "172.16.0.1", "203.0.113.1"];
    
    println!("🔍 Analizando patrones de acceso:");
    
    // Comportamiento normal
    for ip in normal_ips {
        println!("✅ Acceso normal desde IP: {}", ip);
        analyze_access_pattern(ip, "normal");
    }
    
    // Comportamiento anómalo
    for ip in anomalous_ips {
        println!("⚠️  Acceso anómalo detectado desde IP: {}", ip);
        analyze_access_pattern(ip, "anomalous");
        detect_anomalous_behavior(ip);
    }
}

/// Analizar patrón de acceso
fn analyze_access_pattern(ip_address: &str, pattern_type: &str) {
    println!("   📊 Analizando patrón de acceso desde {}", ip_address);
    println!("   📈 Tipo de patrón: {}", pattern_type);
    
    match pattern_type {
        "normal" => {
            println!("   ✅ Patrón normal - sin amenazas detectadas");
        }
        "anomalous" => {
            println!("   ⚠️  Patrón anómalo - posible amenaza");
            println!("   🔍 Características anómalas:");
            println!("      - IP externa no reconocida");
            println!("      - Horario de acceso inusual");
            println!("      - Frecuencia de acceso alta");
        }
        _ => {
            println!("   ❓ Patrón desconocido - requiere análisis");
        }
    }
}

/// Detectar comportamiento anómalo
fn detect_anomalous_behavior(ip_address: &str) {
    println!("🚨 COMPORTAMIENTO ANÓMALO DETECTADO!");
    println!("   IP sospechosa: {}", ip_address);
    println!("   Severidad: ALTA");
    
    // Simular respuesta automática
    respond_to_anomalous_behavior(ip_address);
}

/// Responder a comportamiento anómalo
fn respond_to_anomalous_behavior(ip_address: &str) {
    println!("🛡️  Ejecutando respuesta automática:");
    println!("   1. Monitoreando IP: {}", ip_address);
    println!("   2. Aumentando nivel de alerta");
    println!("   3. Registrando comportamiento sospechoso");
    println!("   4. Notificando a administradores");
    
    // Simular monitoreo aumentado
    increase_monitoring_level(ip_address);
    
    // Simular notificación
    send_security_alert("Anomalous Behavior", "Unknown", ip_address);
}

/// Demostrar detección de patrones
fn demonstrate_pattern_detection() {
    println!("\n--- Demostrando Detección de Patrones ---");
    
    // Simular diferentes patrones de ataque
    let attack_patterns = vec![
        ("Port Scanning", vec!["22", "80", "443", "3389", "5900"]),
        ("Directory Traversal", vec!["../", "../../", "../../../"]),
        ("SQL Injection", vec!["' OR 1=1", "'; DROP TABLE", "UNION SELECT"]),
        ("XSS Attempt", vec!["<script>", "javascript:", "onload="]),
    ];
    
    for (pattern_name, patterns) in attack_patterns {
        println!("🔍 Detectando patrón: {}", pattern_name);
        
        for pattern in patterns {
            if detect_attack_pattern(pattern_name, pattern) {
                println!("🚨 Patrón de ataque detectado: {} - {}", pattern_name, pattern);
                respond_to_attack_pattern(pattern_name, pattern);
            } else {
                println!("✅ Patrón normal: {}", pattern);
            }
        }
    }
}

/// Detectar patrón de ataque
fn detect_attack_pattern(pattern_type: &str, pattern: &str) -> bool {
    match pattern_type {
        "Port Scanning" => {
            // Simular detección de escaneo de puertos
            pattern.parse::<u16>().is_ok() && pattern.parse::<u16>().unwrap() < 10000
        }
        "Directory Traversal" => {
            // Simular detección de directory traversal
            pattern.contains("../")
        }
        "SQL Injection" => {
            // Simular detección de SQL injection
            pattern.to_uppercase().contains("OR") || pattern.to_uppercase().contains("UNION")
        }
        "XSS Attempt" => {
            // Simular detección de XSS
            pattern.contains("<script>") || pattern.contains("javascript:")
        }
        _ => false,
    }
}

/// Responder a patrón de ataque
fn respond_to_attack_pattern(pattern_type: &str, pattern: &str) {
    println!("🛡️  Ejecutando respuesta a patrón de ataque:");
    println!("   Tipo: {}", pattern_type);
    println!("   Patrón: {}", pattern);
    
    match pattern_type {
        "Port Scanning" => {
            println!("   1. Bloqueando IP atacante");
            println!("   2. Aumentando monitoreo de red");
        }
        "Directory Traversal" => {
            println!("   1. Bloqueando solicitud maliciosa");
            println!("   2. Registrando intento de acceso no autorizado");
        }
        "SQL Injection" => {
            println!("   1. Bloqueando consulta maliciosa");
            println!("   2. Alertando a administradores de base de datos");
        }
        "XSS Attempt" => {
            println!("   1. Sanitizando entrada maliciosa");
            println!("   2. Bloqueando script malicioso");
        }
        _ => {
            println!("   1. Aplicando respuesta genérica");
        }
    }
}

/// Demostrar respuesta a amenazas
fn demonstrate_threat_response() {
    println!("\n--- Demostrando Respuesta a Amenazas ---");
    
    // Simular diferentes tipos de amenazas
    let threats = vec![
        ("Malware Detection", "virus.exe", "CRITICAL"),
        ("Unauthorized Access", "admin", "HIGH"),
        ("Data Exfiltration", "sensitive_data.txt", "HIGH"),
        ("System Compromise", "rootkit", "CRITICAL"),
    ];
    
    for (threat_type, target, severity) in threats {
        println!("🚨 AMENAZA DETECTADA: {}", threat_type);
        println!("   Objetivo: {}", target);
        println!("   Severidad: {}", severity);
        
        respond_to_threat(threat_type, target, severity);
    }
}

/// Responder a amenaza
fn respond_to_threat(_threat_type: &str, _target: &str, severity: &str) {
    println!("🛡️  Ejecutando respuesta a amenaza:");
    
    match severity {
        "CRITICAL" => {
            println!("   🚨 RESPUESTA CRÍTICA:");
            println!("   1. Aislamiento inmediato del sistema");
            println!("   2. Bloqueo de todas las conexiones");
            println!("   3. Notificación de emergencia");
            println!("   4. Activación del protocolo de incidentes");
        }
        "HIGH" => {
            println!("   ⚠️  RESPUESTA DE ALTA PRIORIDAD:");
            println!("   1. Bloqueo del objetivo específico");
            println!("   2. Aumento del monitoreo");
            println!("   3. Notificación a administradores");
            println!("   4. Análisis forense");
        }
        "MEDIUM" => {
            println!("   ℹ️  RESPUESTA ESTÁNDAR:");
            println!("   1. Registro del evento");
            println!("   2. Monitoreo aumentado");
            println!("   3. Notificación estándar");
        }
        _ => {
            println!("   ℹ️  RESPUESTA BÁSICA:");
            println!("   1. Registro del evento");
            println!("   2. Monitoreo continuo");
        }
    }
}

/// Demostrar análisis de intrusiones
fn demonstrate_intrusion_analysis() {
    println!("\n--- Demostrando Análisis de Intrusiones ---");
    
    if let Some(manager) = get_security_manager() {
        let events = manager.get_security_events();
        
        // Análisis de eventos de intrusión
        let intrusion_events: Vec<_> = events.iter()
            .filter(|e| e.event_type == SecurityEventType::IntrusionAttempt)
            .collect();
        
        println!("🔍 Análisis de eventos de intrusión:");
        println!("   Total de eventos de intrusión: {}", intrusion_events.len());
        
        if !intrusion_events.is_empty() {
            // Análisis por severidad
            let mut severity_counts = HashMap::new();
            for event in &intrusion_events {
                *severity_counts.entry(event.severity).or_insert(0) += 1;
            }
            
            println!("   Eventos por severidad:");
            for (severity, count) in severity_counts {
                println!("     {:?}: {} eventos", severity, count);
            }
            
            // Análisis por fuente
            let mut source_counts = HashMap::new();
            for event in &intrusion_events {
                *source_counts.entry(&event.source).or_insert(0) += 1;
            }
            
            println!("   Eventos por fuente:");
            for (source, count) in source_counts {
                println!("     {}: {} eventos", source, count);
            }
            
            // Análisis temporal
            let recent_intrusions: Vec<_> = intrusion_events.iter().rev().take(3).collect();
            println!("   Últimos 3 eventos de intrusión:");
            for (i, event) in recent_intrusions.iter().enumerate() {
                println!("     {}. [{}] {} - {}", 
                    i + 1,
                    format!("{:?}", event.severity),
                    event.source,
                    event.description
                );
            }
        }
        
        // Análisis de patrones de ataque
        analyze_attack_patterns(events);
        
        // Análisis de tendencias
        analyze_security_trends(events);
    }
}

/// Analizar patrones de ataque
fn analyze_attack_patterns(events: &[SecurityEvent]) {
    println!("📊 Análisis de patrones de ataque:");
    
    // Contar eventos por tipo
    let mut event_type_counts = HashMap::new();
    for event in events {
        *event_type_counts.entry(event.event_type).or_insert(0) += 1;
    }
    
    // Identificar patrones sospechosos
    let suspicious_patterns = vec![
        (SecurityEventType::Authentication, 10, "Múltiples intentos de autenticación fallida"),
        (SecurityEventType::Authorization, 5, "Múltiples intentos de acceso no autorizado"),
        (SecurityEventType::IntrusionAttempt, 1, "Intento de intrusión detectado"),
    ];
    
    for (event_type, threshold, description) in suspicious_patterns {
        if let Some(count) = event_type_counts.get(&event_type) {
            if *count >= threshold {
                println!("   🚨 Patrón sospechoso detectado: {} ({} eventos)", description, count);
            } else {
                println!("   ✅ Patrón normal: {} ({} eventos)", description, count);
            }
        }
    }
}

/// Analizar tendencias de seguridad
fn analyze_security_trends(events: &[SecurityEvent]) {
    println!("📈 Análisis de tendencias de seguridad:");
    
    // Análisis por severidad
    let mut severity_counts = HashMap::new();
    for event in events {
        *severity_counts.entry(event.severity).or_insert(0) += 1;
    }
    
    let critical_events = severity_counts.get(&SecuritySeverity::Critical).unwrap_or(&0);
    let high_events = severity_counts.get(&SecuritySeverity::High).unwrap_or(&0);
    let medium_events = severity_counts.get(&SecuritySeverity::Medium).unwrap_or(&0);
    let low_events = severity_counts.get(&SecuritySeverity::Low).unwrap_or(&0);
    
    println!("   Eventos críticos: {}", critical_events);
    println!("   Eventos de alta severidad: {}", high_events);
    println!("   Eventos de severidad media: {}", medium_events);
    println!("   Eventos de baja severidad: {}", low_events);
    
    // Evaluar tendencias
    if *critical_events > 0 {
        println!("   🚨 TENDENCIA CRÍTICA: Hay eventos críticos que requieren atención inmediata");
    } else if *high_events > 5 {
        println!("   ⚠️  TENDENCIA ALTA: Hay muchos eventos de alta severidad");
    } else if *medium_events > 10 {
        println!("   ℹ️  TENDENCIA MEDIA: Hay varios eventos de severidad media");
    } else {
        println!("   ✅ TENDENCIA NORMAL: Nivel de seguridad dentro de parámetros normales");
    }
}

/// Funciones auxiliares para simulación

fn block_ip_address(ip_address: &str) {
    println!("   🔒 IP {} bloqueada", ip_address);
}

fn block_user_account(username: &str) {
    println!("   🔒 Cuenta {} bloqueada", username);
}

fn send_security_alert(alert_type: &str, username: &str, ip_address: &str) {
    println!("   📧 Alerta de seguridad enviada: {} - {} desde {}", alert_type, username, ip_address);
}

fn increase_monitoring_level(ip_address: &str) {
    println!("   👁️  Nivel de monitoreo aumentado para IP: {}", ip_address);
}

/// Imprimir estadísticas de detección de intrusiones
fn print_intrusion_detection_statistics(manager: &SecurityManager) {
    println!("\n=== Estadísticas de Detección de Intrusiones ===");
    
    // Configuración de detección de intrusiones
    println!("🔍 Configuración de detección de intrusiones:");
    println!("   Detección de intrusiones habilitada: {}", manager.config.enable_intrusion_detection);
    
    // Estadísticas de eventos de intrusión
    let events = manager.get_security_events();
    let intrusion_events: Vec<_> = events.iter()
        .filter(|e| e.event_type == SecurityEventType::IntrusionAttempt)
        .collect();
    
    println!("📊 Estadísticas de eventos de intrusión:");
    println!("   Total de eventos de intrusión: {}", intrusion_events.len());
    
    if !intrusion_events.is_empty() {
        // Análisis por severidad
        let mut severity_counts = HashMap::new();
        for event in &intrusion_events {
            *severity_counts.entry(event.severity).or_insert(0) += 1;
        }
        
        println!("   Eventos por severidad:");
        for (severity, count) in severity_counts {
            println!("     {:?}: {} eventos", severity, count);
        }
        
        // Análisis de éxito/fallo
        let successful_intrusions = intrusion_events.iter().filter(|e| e.success).count();
        let failed_intrusions = intrusion_events.len() - successful_intrusions;
        
        println!("   Intrusiones exitosas: {}", successful_intrusions);
        println!("   Intrusiones bloqueadas: {}", failed_intrusions);
        
        if intrusion_events.len() > 0 {
            let block_rate = (failed_intrusions as f64 / intrusion_events.len() as f64) * 100.0;
            println!("   Tasa de bloqueo: {:.2}%", block_rate);
        }
    }
    
    // Estadísticas de eventos de autenticación fallida
    let failed_auth_events: Vec<_> = events.iter()
        .filter(|e| e.event_type == SecurityEventType::Authentication && !e.success)
        .collect();
    
    println!("🔐 Estadísticas de autenticación fallida:");
    println!("   Total de intentos fallidos: {}", failed_auth_events.len());
    
    if failed_auth_events.len() > 10 {
        println!("   ⚠️  Alto número de intentos fallidos - posible ataque de fuerza bruta");
    }
    
    // Estadísticas de eventos de autorización fallida
    let failed_authz_events: Vec<_> = events.iter()
        .filter(|e| e.event_type == SecurityEventType::Authorization && !e.success)
        .collect();
    
    println!("🚫 Estadísticas de autorización fallida:");
    println!("   Total de accesos denegados: {}", failed_authz_events.len());
    
    if failed_authz_events.len() > 5 {
        println!("   ⚠️  Alto número de accesos denegados - posible intento de escalación");
    }
    
    // Recomendaciones de seguridad
    println!("💡 Recomendaciones de detección de intrusiones:");
    
    if !manager.config.enable_intrusion_detection {
        println!("   ⚠️  La detección de intrusiones está deshabilitada - considera habilitarla");
    }
    
    if intrusion_events.len() > 0 {
        println!("   🚨 Hay eventos de intrusión registrados - revisar y tomar medidas");
    }
    
    if failed_auth_events.len() > 20 {
        println!("   ⚠️  Muchos intentos de autenticación fallida - revisar políticas de contraseñas");
    }
    
    if failed_authz_events.len() > 10 {
        println!("   ⚠️  Muchos accesos denegados - revisar permisos y roles");
    }
    
    // Verificar eventos críticos
    let critical_events = events.iter().filter(|e| e.severity == SecuritySeverity::Critical).count();
    if critical_events > 0 {
        println!("   🚨 Hay {} eventos críticos que requieren atención inmediata", critical_events);
    }
}

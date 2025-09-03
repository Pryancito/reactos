//! Gestor de Seguridad para ReactOS Rust
//! 
//! Herramienta principal para gestionar y monitorear
//! la seguridad del sistema.

use reactos_rust_security::*;
// use std::time::Duration;

/// Función principal del gestor de seguridad
fn main() {
    println!("=== Gestor de Seguridad - ReactOS Rust ===");
    
    // Inicializar gestor de seguridad
    if let Err(e) = init_security_manager() {
        eprintln!("Error inicializando gestor de seguridad: {}", e);
        return;
    }
    
    println!("Gestor de seguridad inicializado");
    
    // Ejecutar demostración del sistema de seguridad
    run_security_demonstration();
    
    // Mostrar estado de seguridad
    if let Some(manager) = get_security_manager() {
        print_security_status(manager);
    }
    
    println!("Gestor de seguridad completado");
}

/// Ejecutar demostración del sistema de seguridad
fn run_security_demonstration() {
    println!("\n=== Demostración del Sistema de Seguridad ===");
    
    // Demostrar autenticación
    demonstrate_authentication();
    
    // Demostrar autorización
    demonstrate_authorization();
    
    // Demostrar cifrado
    demonstrate_encryption();
    
    // Demostrar auditoría
    demonstrate_audit();
    
    // Demostrar gestión de sesiones
    demonstrate_session_management();
    
    println!("\n=== Demostración Completada ===");
}

/// Demostrar sistema de autenticación
fn demonstrate_authentication() {
    println!("\n--- Demostrando Sistema de Autenticación ---");
    
    // Intentar autenticación exitosa
    match authenticate_user("admin", "admin123", "192.168.1.100") {
        Ok(result) => {
            println!("✅ Autenticación exitosa: {}", result.message);
            println!("   Severidad: {:?}", result.severity);
            println!("   Timestamp: {:?}", result.timestamp);
        }
        Err(e) => {
            println!("❌ Error de autenticación: {}", e);
        }
    }
    
    // Intentar autenticación fallida
    match authenticate_user("admin", "wrongpassword", "192.168.1.100") {
        Ok(result) => {
            println!("✅ Autenticación exitosa: {}", result.message);
        }
        Err(e) => {
            println!("❌ Autenticación fallida (esperado): {}", e);
        }
    }
    
    // Intentar autenticación con usuario inexistente
    match authenticate_user("nonexistent", "password", "192.168.1.100") {
        Ok(result) => {
            println!("✅ Autenticación exitosa: {}", result.message);
        }
        Err(e) => {
            println!("❌ Usuario no encontrado (esperado): {}", e);
        }
    }
}

/// Demostrar sistema de autorización
fn demonstrate_authorization() {
    println!("\n--- Demostrando Sistema de Autorización ---");
    
    // Autorizar acceso a recurso permitido
    match authorize_access("admin", "system", "admin") {
        Ok(result) => {
            println!("✅ Acceso autorizado: {}", result.message);
            println!("   Severidad: {:?}", result.severity);
        }
        Err(e) => {
            println!("❌ Acceso denegado: {}", e);
        }
    }
    
    // Intentar acceso a recurso no permitido
    match authorize_access("admin", "restricted", "access") {
        Ok(result) => {
            println!("✅ Acceso autorizado: {}", result.message);
        }
        Err(e) => {
            println!("❌ Acceso denegado (esperado): {}", e);
        }
    }
}

/// Demostrar sistema de cifrado
fn demonstrate_encryption() {
    println!("\n--- Demostrando Sistema de Cifrado ---");
    
    let original_data = b"Hello, ReactOS Rust Security!";
    let key = b"secretkey123";
    
    // Cifrar datos
    match encrypt_data(original_data, key) {
        Ok(encrypted) => {
            println!("✅ Datos cifrados exitosamente");
            println!("   Datos originales: {:?}", String::from_utf8_lossy(original_data));
            println!("   Datos cifrados: {:?}", encrypted);
            
            // Descifrar datos
            match decrypt_data(&encrypted, key) {
                Ok(decrypted) => {
                    println!("✅ Datos descifrados exitosamente");
                    println!("   Datos descifrados: {:?}", String::from_utf8_lossy(&decrypted));
                    
                    // Verificar integridad
                    if original_data == decrypted.as_slice() {
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
}

/// Demostrar sistema de auditoría
fn demonstrate_audit() {
    println!("\n--- Demostrando Sistema de Auditoría ---");
    
    if let Some(manager) = get_security_manager() {
        let events = manager.get_security_events();
        println!("📊 Total de eventos de seguridad: {}", events.len());
        
        // Mostrar últimos 5 eventos
        let recent_events: Vec<_> = events.iter().rev().take(5).collect();
        println!("📋 Últimos 5 eventos de seguridad:");
        
        for (i, event) in recent_events.iter().enumerate() {
            println!("   {}. [{}] {} - {} ({})", 
                i + 1,
                format!("{:?}", event.severity),
                format!("{:?}", event.event_type),
                event.description,
                if event.success { "✅" } else { "❌" }
            );
        }
        
        // Mostrar estadísticas por severidad
        let mut severity_counts = std::collections::HashMap::new();
        for event in events {
            *severity_counts.entry(event.severity).or_insert(0) += 1;
        }
        
        println!("📈 Estadísticas por severidad:");
        for (severity, count) in severity_counts {
            println!("   {:?}: {} eventos", severity, count);
        }
    }
}

/// Demostrar gestión de sesiones
fn demonstrate_session_management() {
    println!("\n--- Demostrando Gestión de Sesiones ---");
    
    if let Some(manager) = get_security_manager() {
        println!("📊 Total de sesiones activas: {}", manager.sessions.len());
        
        // Mostrar sesiones activas
        for (session_id, session) in &manager.sessions {
            println!("   Sesión: {}", session_id);
            println!("     Usuario: {}", session.user_id);
            println!("     IP: {}", session.ip_address);
            println!("     Creada: {:?}", session.created_at);
            println!("     Expira: {:?}", session.expires_at);
            println!("     Activa: {}", session.is_active);
        }
        
        // Limpiar sesiones expiradas
        manager.cleanup_expired_sessions();
        println!("🧹 Sesiones expiradas limpiadas");
    }
}

/// Imprimir estado de seguridad
fn print_security_status(manager: &SecurityManager) {
    println!("\n=== Estado de Seguridad del Sistema ===");
    
    // Estado general
    println!("🔒 Estado de seguridad: {:?}", manager.get_security_state());
    
    // Configuración
    println!("⚙️  Configuración de seguridad:");
    println!("   Autenticación: {}", manager.config.enable_authentication);
    println!("   Autorización: {}", manager.config.enable_authorization);
    println!("   Cifrado: {}", manager.config.enable_encryption);
    println!("   Auditoría: {}", manager.config.enable_audit);
    println!("   Detección de intrusiones: {}", manager.config.enable_intrusion_detection);
    println!("   Gestión de claves: {}", manager.config.enable_key_management);
    
    // Política de contraseñas
    println!("🔑 Política de contraseñas:");
    println!("   Longitud mínima: {}", manager.config.password_policy.min_length);
    println!("   Requiere mayúsculas: {}", manager.config.password_policy.require_uppercase);
    println!("   Requiere minúsculas: {}", manager.config.password_policy.require_lowercase);
    println!("   Requiere números: {}", manager.config.password_policy.require_numbers);
    println!("   Requiere caracteres especiales: {}", manager.config.password_policy.require_special_chars);
    println!("   Edad máxima (días): {}", manager.config.password_policy.max_age_days);
    
    // Configuración de sesiones
    println!("⏱️  Configuración de sesiones:");
    println!("   Timeout de sesión: {} segundos", manager.config.session_timeout.as_secs());
    println!("   Máximo intentos de login: {}", manager.config.max_login_attempts);
    println!("   Duración de bloqueo: {} segundos", manager.config.lockout_duration.as_secs());
    
    // Estadísticas de usuarios
    println!("👥 Estadísticas de usuarios:");
    println!("   Total de usuarios: {}", manager.users.len());
    let active_users = manager.users.values().filter(|u| u.is_active).count();
    println!("   Usuarios activos: {}", active_users);
    
    // Estadísticas de roles
    println!("🎭 Estadísticas de roles:");
    println!("   Total de roles: {}", manager.roles.len());
    for (role_id, role) in &manager.roles {
        println!("     {}: {} ({} permisos)", role_id, role.name, role.permissions.len());
    }
    
    // Estadísticas de permisos
    println!("🔐 Estadísticas de permisos:");
    println!("   Total de permisos: {}", manager.permissions.len());
    
    // Estadísticas de eventos
    let events = manager.get_security_events();
    println!("📊 Estadísticas de eventos:");
    println!("   Total de eventos: {}", events.len());
    
    let successful_events = events.iter().filter(|e| e.success).count();
    let failed_events = events.iter().filter(|e| !e.success).count();
    println!("   Eventos exitosos: {}", successful_events);
    println!("   Eventos fallidos: {}", failed_events);
    
    // Eventos por tipo
    let mut event_type_counts = std::collections::HashMap::new();
    for event in events {
        *event_type_counts.entry(event.event_type).or_insert(0) += 1;
    }
    
    println!("   Eventos por tipo:");
    for (event_type, count) in event_type_counts {
        println!("     {:?}: {} eventos", event_type, count);
    }
    
    // Recomendaciones de seguridad
    println!("💡 Recomendaciones de seguridad:");
    
    if manager.config.password_policy.min_length < 12 {
        println!("   ⚠️  Considera aumentar la longitud mínima de contraseñas a 12 caracteres");
    }
    
    if manager.config.session_timeout.as_secs() > 3600 {
        println!("   ⚠️  Considera reducir el timeout de sesión a 1 hora o menos");
    }
    
    if manager.config.max_login_attempts > 3 {
        println!("   ⚠️  Considera reducir el máximo de intentos de login a 3");
    }
    
    if events.len() > 1000 {
        println!("   ⚠️  Considera revisar los logs de seguridad para detectar patrones sospechosos");
    }
    
    let critical_events = events.iter().filter(|e| e.severity == SecuritySeverity::Critical).count();
    if critical_events > 0 {
        println!("   🚨 Hay {} eventos críticos que requieren atención inmediata", critical_events);
    }
    
    let high_events = events.iter().filter(|e| e.severity == SecuritySeverity::High).count();
    if high_events > 5 {
        println!("   ⚠️  Hay {} eventos de alta severidad que requieren revisión", high_events);
    }
}

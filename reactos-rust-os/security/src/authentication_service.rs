//! Servicio de Autenticación para ReactOS Rust
//! 
//! Servicio dedicado para manejar la autenticación
//! de usuarios y gestión de sesiones.

use reactos_rust_security::*;
use std::time::Duration;

/// Función principal del servicio de autenticación
fn main() {
    println!("=== Servicio de Autenticación - ReactOS Rust ===");
    
    // Inicializar gestor de seguridad
    if let Err(e) = init_security_manager() {
        eprintln!("Error inicializando gestor de seguridad: {}", e);
        return;
    }
    
    println!("Servicio de autenticación inicializado");
    
    // Ejecutar demostración del servicio de autenticación
    run_authentication_demonstration();
    
    // Mostrar estadísticas de autenticación
    if let Some(manager) = get_security_manager() {
        print_authentication_statistics(manager);
    }
    
    println!("Servicio de autenticación completado");
}

/// Ejecutar demostración del servicio de autenticación
fn run_authentication_demonstration() {
    println!("\n=== Demostración del Servicio de Autenticación ===");
    
    // Demostrar diferentes escenarios de autenticación
    demonstrate_successful_authentication();
    demonstrate_failed_authentication();
    demonstrate_account_lockout();
    demonstrate_session_management();
    demonstrate_password_policy();
    
    println!("\n=== Demostración Completada ===");
}

/// Demostrar autenticación exitosa
fn demonstrate_successful_authentication() {
    println!("\n--- Demostrando Autenticación Exitosa ---");
    
    let test_cases = vec![
        ("admin", "admin123", "192.168.1.100"),
        ("admin", "admin123", "192.168.1.101"),
        ("admin", "admin123", "10.0.0.50"),
    ];
    
    for (username, password, ip) in test_cases {
        match authenticate_user(username, password, ip) {
            Ok(result) => {
                println!("✅ Autenticación exitosa para {} desde {}", username, ip);
                println!("   Mensaje: {}", result.message);
                println!("   Severidad: {:?}", result.severity);
                println!("   Timestamp: {:?}", result.timestamp);
            }
            Err(e) => {
                println!("❌ Error de autenticación para {} desde {}: {}", username, ip, e);
            }
        }
    }
}

/// Demostrar autenticación fallida
fn demonstrate_failed_authentication() {
    println!("\n--- Demostrando Autenticación Fallida ---");
    
    let test_cases = vec![
        ("admin", "wrongpassword", "192.168.1.100"),
        ("admin", "admin", "192.168.1.101"),
        ("admin", "password", "10.0.0.50"),
        ("admin", "123456", "192.168.1.102"),
    ];
    
    for (username, password, ip) in test_cases {
        match authenticate_user(username, password, ip) {
            Ok(result) => {
                println!("✅ Autenticación exitosa para {} desde {}: {}", username, ip, result.message);
            }
            Err(e) => {
                println!("❌ Autenticación fallida para {} desde {}: {}", username, ip, e);
            }
        }
    }
}

/// Demostrar bloqueo de cuenta
fn demonstrate_account_lockout() {
    println!("\n--- Demostrando Bloqueo de Cuenta ---");
    
    // Intentar múltiples autenticaciones fallidas para activar el bloqueo
    for i in 1..=6 {
        match authenticate_user("admin", "wrongpassword", "192.168.1.100") {
            Ok(result) => {
                println!("✅ Intento {}: Autenticación exitosa: {}", i, result.message);
            }
            Err(e) => {
                println!("❌ Intento {}: Autenticación fallida: {}", i, e);
            }
        }
    }
    
    // Intentar autenticación después del bloqueo
    match authenticate_user("admin", "admin123", "192.168.1.100") {
        Ok(result) => {
            println!("✅ Autenticación exitosa después del bloqueo: {}", result.message);
        }
        Err(e) => {
            println!("❌ Cuenta bloqueada (esperado): {}", e);
        }
    }
}

/// Demostrar gestión de sesiones
fn demonstrate_session_management() {
    println!("\n--- Demostrando Gestión de Sesiones ---");
    
    if let Some(manager) = get_security_manager() {
        println!("📊 Estado actual de sesiones:");
        println!("   Total de sesiones: {}", manager.sessions.len());
        
        // Mostrar detalles de cada sesión
        for (session_id, session) in &manager.sessions {
            println!("   Sesión: {}", session_id);
            println!("     Usuario: {}", session.user_id);
            println!("     IP: {}", session.ip_address);
            println!("     Creada: {:?}", session.created_at);
            println!("     Última actividad: {:?}", session.last_activity);
            println!("     Expira: {:?}", session.expires_at);
            println!("     Activa: {}", session.is_active);
            
            // Calcular tiempo restante
            let now = std::time::Instant::now();
            if session.expires_at > now {
                let remaining = session.expires_at.duration_since(now);
                println!("     Tiempo restante: {} segundos", remaining.as_secs());
            } else {
                println!("     Sesión expirada");
            }
        }
        
        // Limpiar sesiones expiradas
        let sessions_before = manager.sessions.len();
        manager.cleanup_expired_sessions();
        let sessions_after = manager.sessions.len();
        
        if sessions_before > sessions_after {
            println!("🧹 {} sesiones expiradas fueron limpiadas", sessions_before - sessions_after);
        } else {
            println!("✅ No hay sesiones expiradas para limpiar");
        }
    }
}

/// Demostrar política de contraseñas
fn demonstrate_password_policy() {
    println!("\n--- Demostrando Política de Contraseñas ---");
    
    if let Some(manager) = get_security_manager() {
        let policy = &manager.config.password_policy;
        
        println!("🔑 Política de contraseñas actual:");
        println!("   Longitud mínima: {} caracteres", policy.min_length);
        println!("   Requiere mayúsculas: {}", policy.require_uppercase);
        println!("   Requiere minúsculas: {}", policy.require_lowercase);
        println!("   Requiere números: {}", policy.require_numbers);
        println!("   Requiere caracteres especiales: {}", policy.require_special_chars);
        println!("   Edad máxima: {} días", policy.max_age_days);
        println!("   Historial de contraseñas: {} contraseñas", policy.history_count);
        
        // Demostrar validación de contraseñas
        let test_passwords = vec![
            "123",           // Muy corta
            "password",      // Sin mayúsculas, números, caracteres especiales
            "Password",      // Sin números, caracteres especiales
            "Password1",     // Sin caracteres especiales
            "Password1!",    // Válida
            "MySecure123!",  // Válida
        ];
        
        println!("\n🧪 Validación de contraseñas de prueba:");
        for password in test_passwords {
            let is_valid = validate_password(password, policy);
            println!("   '{}': {}", password, if is_valid { "✅ Válida" } else { "❌ Inválida" });
        }
    }
}

/// Validar contraseña según la política
fn validate_password(password: &str, policy: &PasswordPolicy) -> bool {
    // Verificar longitud mínima
    if password.len() < policy.min_length {
        return false;
    }
    
    // Verificar mayúsculas
    if policy.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
        return false;
    }
    
    // Verificar minúsculas
    if policy.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
        return false;
    }
    
    // Verificar números
    if policy.require_numbers && !password.chars().any(|c| c.is_numeric()) {
        return false;
    }
    
    // Verificar caracteres especiales
    if policy.require_special_chars {
        let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        if !password.chars().any(|c| special_chars.contains(c)) {
            return false;
        }
    }
    
    true
}

/// Imprimir estadísticas de autenticación
fn print_authentication_statistics(manager: &SecurityManager) {
    println!("\n=== Estadísticas de Autenticación ===");
    
    // Estadísticas de usuarios
    println!("👥 Estadísticas de usuarios:");
    println!("   Total de usuarios: {}", manager.users.len());
    
    let active_users = manager.users.values().filter(|u| u.is_active).count();
    let inactive_users = manager.users.len() - active_users;
    println!("   Usuarios activos: {}", active_users);
    println!("   Usuarios inactivos: {}", inactive_users);
    
    // Estadísticas de sesiones
    println!("🔐 Estadísticas de sesiones:");
    println!("   Total de sesiones: {}", manager.sessions.len());
    
    let active_sessions = manager.sessions.values().filter(|s| s.is_active).count();
    let expired_sessions = manager.sessions.len() - active_sessions;
    println!("   Sesiones activas: {}", active_sessions);
    println!("   Sesiones expiradas: {}", expired_sessions);
    
    // Estadísticas de eventos de autenticación
    let auth_events: Vec<_> = manager.get_security_events()
        .iter()
        .filter(|e| e.event_type == SecurityEventType::Authentication)
        .collect();
    
    println!("📊 Estadísticas de eventos de autenticación:");
    println!("   Total de eventos: {}", auth_events.len());
    
    let successful_auths = auth_events.iter().filter(|e| e.success).count();
    let failed_auths = auth_events.len() - successful_auths;
    println!("   Autenticaciones exitosas: {}", successful_auths);
    println!("   Autenticaciones fallidas: {}", failed_auths);
    
    if auth_events.len() > 0 {
        let success_rate = (successful_auths as f64 / auth_events.len() as f64) * 100.0;
        println!("   Tasa de éxito: {:.2}%", success_rate);
    }
    
    // Eventos por severidad
    let mut severity_counts = std::collections::HashMap::new();
    for event in auth_events {
        *severity_counts.entry(event.severity).or_insert(0) += 1;
    }
    
    println!("   Eventos por severidad:");
    for (severity, count) in severity_counts {
        println!("     {:?}: {} eventos", severity, count);
    }
    
    // Estadísticas de intentos de login
    println!("🔒 Estadísticas de intentos de login:");
    for (username, user) in &manager.users {
        println!("   Usuario: {}", username);
        println!("     Intentos de login: {}", user.login_attempts);
        println!("     Último login: {:?}", user.last_login);
        println!("     Bloqueado hasta: {:?}", user.locked_until);
        println!("     Contraseña cambiada: {:?}", user.password_changed_at);
    }
    
    // Recomendaciones de seguridad
    println!("💡 Recomendaciones de autenticación:");
    
    // Verificar usuarios con muchas fallas
    for (username, user) in &manager.users {
        if user.login_attempts > 3 {
            println!("   ⚠️  Usuario '{}' tiene {} intentos fallidos", username, user.login_attempts);
        }
    }
    
    // Verificar sesiones de larga duración
    let now = std::time::Instant::now();
    for (session_id, session) in &manager.sessions {
        if session.is_active {
            let duration = now.duration_since(session.created_at);
            if duration > Duration::from_secs(7200) { // 2 horas
                println!("   ⚠️  Sesión '{}' ha estado activa por más de 2 horas", session_id);
            }
        }
    }
    
    // Verificar política de contraseñas
    let policy = &manager.config.password_policy;
    if policy.min_length < 8 {
        println!("   ⚠️  Considera aumentar la longitud mínima de contraseñas a 8 caracteres");
    }
    
    if !policy.require_special_chars {
        println!("   ⚠️  Considera requerir caracteres especiales en las contraseñas");
    }
    
    if policy.max_age_days > 90 {
        println!("   ⚠️  Considera reducir la edad máxima de contraseñas a 90 días");
    }
    
    // Verificar configuración de bloqueo
    if manager.config.max_login_attempts > 5 {
        println!("   ⚠️  Considera reducir el máximo de intentos de login a 5");
    }
    
    if manager.config.lockout_duration < Duration::from_secs(900) {
        println!("   ⚠️  Considera aumentar la duración de bloqueo a 15 minutos");
    }
}

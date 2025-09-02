//! Sistema de logging para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Sistema de logging estructurado
//! - Niveles de log (Debug, Info, Warn, Error)
//! - Timestamps y contexto
//! - Rotación de logs

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Niveles de logging
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

/// Entrada de log
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub context: Option<String>,
}

/// Sistema de logging
pub struct Logger {
    entries: Vec<LogEntry>,
    max_entries: usize,
    current_level: LogLevel,
    timestamp_counter: AtomicUsize,
}

impl Logger {
    /// Crear un nuevo logger
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 1000,
            current_level: LogLevel::Info,
            timestamp_counter: AtomicUsize::new(0),
        }
    }

    /// Configurar nivel de logging
    pub fn set_level(&mut self, level: LogLevel) {
        self.current_level = level;
    }

    /// Agregar entrada de log
    pub fn log(&mut self, level: LogLevel, module: &str, message: &str, context: Option<&str>) {
        if level < self.current_level {
            return;
        }

        let timestamp = self.timestamp_counter.fetch_add(1, Ordering::SeqCst) as u64;
        
        let entry = LogEntry {
            timestamp,
            level,
            module: module.to_string(),
            message: message.to_string(),
            context: context.map(|s| s.to_string()),
        };

        self.entries.push(entry);

        // Rotar logs si es necesario
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    /// Obtener entradas de log
    pub fn get_entries(&self) -> &Vec<LogEntry> {
        &self.entries
    }

    /// Obtener estadísticas del logger
    pub fn get_stats(&self) -> String {
        let total = self.entries.len();
        let debug_count = self.entries.iter().filter(|e| e.level == LogLevel::Debug).count();
        let info_count = self.entries.iter().filter(|e| e.level == LogLevel::Info).count();
        let warn_count = self.entries.iter().filter(|e| e.level == LogLevel::Warn).count();
        let error_count = self.entries.iter().filter(|e| e.level == LogLevel::Error).count();

        format!(
            "Logger: {} entradas totales (Debug: {}, Info: {}, Warn: {}, Error: {})",
            total, debug_count, info_count, warn_count, error_count
        )
    }

    /// Limpiar logs
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Exportar logs a string
    pub fn export_logs(&self) -> String {
        let mut result = String::new();
        result.push_str("=== ReactOS Rust Kernel Logs ===\n");
        
        for entry in &self.entries {
            let level_str = match entry.level {
                LogLevel::Debug => "DEBUG",
                LogLevel::Info => "INFO ",
                LogLevel::Warn => "WARN ",
                LogLevel::Error => "ERROR",
            };

            result.push_str(&format!(
                "[{}] {} {}: {}\n",
                entry.timestamp,
                level_str,
                entry.module,
                entry.message
            ));

            if let Some(ref context) = entry.context {
                result.push_str(&format!("  Context: {}\n", context));
            }
        }

        result
    }
}

/// Instancia global del logger
static LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

/// Inicializar el sistema de logging
pub fn init_logging() -> bool {
    let mut logger_guard = LOGGER.lock();
    *logger_guard = Some(Logger::new());
    true
}

/// Log de debug
pub fn debug(module: &str, message: &str) {
    log(LogLevel::Debug, module, message, None);
}

/// Log de info
pub fn info(module: &str, message: &str) {
    log(LogLevel::Info, module, message, None);
}

/// Log de warning
pub fn warn(module: &str, message: &str) {
    log(LogLevel::Warn, module, message, None);
}

/// Log de error
pub fn error(module: &str, message: &str) {
    log(LogLevel::Error, module, message, None);
}

/// Log con contexto
pub fn log_with_context(level: LogLevel, module: &str, message: &str, context: &str) {
    log(level, module, message, Some(context));
}

/// Función de logging principal
fn log(level: LogLevel, module: &str, message: &str, context: Option<&str>) {
    let mut logger_guard = LOGGER.lock();
    if let Some(ref mut logger) = *logger_guard {
        logger.log(level, module, message, context);
    }
}

/// Obtener estadísticas del logging
pub fn get_logging_stats() -> String {
    let logger_guard = LOGGER.lock();
    if let Some(ref logger) = *logger_guard {
        logger.get_stats()
    } else {
        String::from("Sistema de logging: No disponible")
    }
}

/// Exportar logs
pub fn export_logs() -> String {
    let logger_guard = LOGGER.lock();
    if let Some(ref logger) = *logger_guard {
        logger.export_logs()
    } else {
        String::from("Sistema de logging: No disponible")
    }
}

/// Verificar si el sistema de logging está disponible
pub fn is_logging_available() -> bool {
    let logger_guard = LOGGER.lock();
    logger_guard.is_some()
}

/// Configurar nivel de logging
pub fn set_log_level(level: LogLevel) {
    let mut logger_guard = LOGGER.lock();
    if let Some(ref mut logger) = *logger_guard {
        logger.set_level(level);
    }
}

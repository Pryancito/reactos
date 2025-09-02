//! # Interrupt Handler
//! 
//! Manejo seguro de interrupciones del kernel en Rust

use core::sync::atomic::{AtomicU64, Ordering};
use crate::kernel_core::memory::{MemoryResult, MemoryError};

/// Número de interrupción
pub type InterruptNumber = u8;

/// Handler de interrupción
pub type InterruptHandler = fn(InterruptNumber) -> InterruptResult;

/// Resultado del manejo de interrupción
pub type InterruptResult = Result<(), InterruptError>;

/// Errores de interrupción
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptError {
    InvalidInterrupt,
    HandlerNotFound,
    HandlerAlreadyRegistered,
    HandlerNotRegistered,
    InterruptDisabled,
    InterruptEnabled,
}

/// Tipo de interrupción
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptType {
    Hardware,
    Software,
    Exception,
    SystemCall,
}

/// Prioridad de interrupción
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterruptPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Información de una interrupción (simplificado)
#[derive(Debug)]
pub struct InterruptInfo {
    pub number: InterruptNumber,
    pub name: &'static str,
    pub interrupt_type: InterruptType,
    pub priority: InterruptPriority,
    pub enabled: bool,
    pub call_count: u64,
}

impl InterruptInfo {
    pub fn new(number: InterruptNumber, name: &'static str, interrupt_type: InterruptType, priority: InterruptPriority) -> Self {
        Self {
            number,
            name,
            interrupt_type,
            priority,
            enabled: true,
            call_count: 0,
        }
    }
}

/// Manager de interrupciones del kernel (simplificado)
pub struct InterruptManager {
    interrupt_count: AtomicU64,
    total_interrupts: AtomicU64,
    interrupt_enabled: AtomicU64,
    current_interrupt: Option<InterruptNumber>,
}

impl InterruptManager {
    pub fn new() -> Self {
        Self {
            interrupt_count: AtomicU64::new(0),
            total_interrupts: AtomicU64::new(0),
            interrupt_enabled: AtomicU64::new(1),
            current_interrupt: None,
        }
    }

    /// Manejar una interrupción (simplificado)
    pub fn handle_interrupt(&mut self, number: InterruptNumber) -> InterruptResult {
        // Ya no necesitamos verificar >= 256 porque InterruptNumber es u8 (0-255)

        // Verificar si las interrupciones están habilitadas
        if self.interrupt_enabled.load(Ordering::SeqCst) == 0 {
            return Err(InterruptError::InterruptDisabled);
        }

        // Actualizar estadísticas
        self.interrupt_count.fetch_add(1, Ordering::SeqCst);
        self.total_interrupts.fetch_add(1, Ordering::SeqCst);
        self.current_interrupt = Some(number);

        Ok(())
    }

    /// Habilitar todas las interrupciones
    pub fn enable_all_interrupts(&mut self) {
        self.interrupt_enabled.store(1, Ordering::SeqCst);
    }

    /// Deshabilitar todas las interrupciones
    pub fn disable_all_interrupts(&mut self) {
        self.interrupt_enabled.store(0, Ordering::SeqCst);
    }

    /// Verificar si las interrupciones están habilitadas
    pub fn are_interrupts_enabled(&self) -> bool {
        self.interrupt_enabled.load(Ordering::SeqCst) != 0
    }

    /// Obtener la interrupción actual
    pub fn current_interrupt(&self) -> Option<InterruptNumber> {
        self.current_interrupt
    }

    /// Obtener estadísticas de interrupciones
    pub fn get_interrupt_stats(&self) -> InterruptStats {
        InterruptStats {
            total_interrupts: self.total_interrupts.load(Ordering::SeqCst),
            interrupt_count: self.interrupt_count.load(Ordering::SeqCst),
            interrupts_enabled: self.are_interrupts_enabled(),
            current_interrupt: self.current_interrupt,
        }
    }

    /// Limpiar estadísticas
    pub fn clear_stats(&mut self) {
        self.interrupt_count.store(0, Ordering::SeqCst);
        self.total_interrupts.store(0, Ordering::SeqCst);
    }
}

/// Estadísticas de interrupciones
#[derive(Debug, Clone, Copy)]
pub struct InterruptStats {
    pub total_interrupts: u64,
    pub interrupt_count: u64,
    pub interrupts_enabled: bool,
    pub current_interrupt: Option<InterruptNumber>,
}

/// Handlers de interrupciones comunes
pub mod handlers {
    use super::*;

    /// Handler para interrupciones de timer
    pub fn timer_handler(_number: InterruptNumber) -> InterruptResult {
        Ok(())
    }

    /// Handler para interrupciones de teclado
    pub fn keyboard_handler(_number: InterruptNumber) -> InterruptResult {
        Ok(())
    }

    /// Handler para interrupciones de mouse
    pub fn mouse_handler(_number: InterruptNumber) -> InterruptResult {
        Ok(())
    }

    /// Handler para interrupciones de red
    pub fn network_handler(_number: InterruptNumber) -> InterruptResult {
        Ok(())
    }

    /// Handler para interrupciones de disco
    pub fn disk_handler(_number: InterruptNumber) -> InterruptResult {
        Ok(())
    }
}

/// Inicializar el interrupt manager
pub fn init() -> MemoryResult<()> {
    // En una implementación completa, esto inicializaría:
    // - Tabla de interrupciones
    // - Handlers por defecto
    // - Configuración de interrupciones
    // - Estructuras de datos para interrupciones
    
    Ok(())
}
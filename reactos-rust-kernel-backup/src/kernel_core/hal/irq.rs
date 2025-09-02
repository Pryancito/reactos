//! # IRQ Manager
//! 
//! Gestión de interrupciones (Interrupt Request)

use crate::kernel_core::memory::{MemoryResult, MemoryError};
use core::sync::atomic::{AtomicU64, Ordering};

/// Tipo de interrupción
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqType {
    Hardware,   // Interrupción de hardware
    Software,   // Interrupción de software
    Timer,      // Interrupción de temporizador
    Keyboard,   // Interrupción de teclado
    Mouse,      // Interrupción de ratón
    Network,    // Interrupción de red
    Storage,    // Interrupción de almacenamiento
    Graphics,   // Interrupción de gráficos
    Audio,      // Interrupción de audio
    USB,        // Interrupción USB
    PCI,        // Interrupción PCI
    Unknown,    // Tipo desconocido
}

/// Prioridad de interrupción
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqPriority {
    Critical,   // Crítica
    High,       // Alta
    Normal,     // Normal
    Low,        // Baja
    Background, // Fondo
}

/// Estado de interrupción
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqState {
    Disabled,   // Deshabilitada
    Enabled,    // Habilitada
    Active,     // Activa
    Pending,    // Pendiente
    Handled,    // Manejada
    Error,      // Error
}

/// Información de interrupción
#[derive(Debug, Clone, Copy)]
pub struct IrqInfo {
    pub irq_id: u32,
    pub irq_number: u8,
    pub irq_type: IrqType,
    pub priority: IrqPriority,
    pub state: IrqState,
    pub device_id: u32,
    pub handler_address: u64,
    pub handler_data: u64,
    pub interrupt_count: u64,
    pub last_interrupt_time: u64,
    pub average_latency: u32,    // Latencia promedio en microsegundos
    pub max_latency: u32,        // Latencia máxima en microsegundos
    pub min_latency: u32,        // Latencia mínima en microsegundos
    pub error_count: u64,
    pub shared: bool,            // Interrupción compartida
    pub level_triggered: bool,   // Activada por nivel
    pub edge_triggered: bool,    // Activada por flanco
}

/// Manager de IRQ
pub struct IrqManager {
    interrupts: [Option<IrqInfo>; 256], // Array fijo para evitar Vec
    next_irq_id: AtomicU64,
    irq_count: AtomicU64,
    enabled_interrupts: AtomicU64,
    active_interrupts: AtomicU64,
    pending_interrupts: AtomicU64,
    error_interrupts: AtomicU64,
    total_interrupts: AtomicU64,
    handled_interrupts: AtomicU64,
    spurious_interrupts: AtomicU64,
    interrupt_latency_total: AtomicU64,
    interrupt_latency_count: AtomicU64,
    irq_switches: AtomicU64,     // Cambios de contexto por IRQ
    irq_nesting_depth: AtomicU64, // Profundidad de anidamiento
}

impl IrqManager {
    pub fn new() -> Self {
        Self {
            interrupts: [(); 256].map(|_| None),
            next_irq_id: AtomicU64::new(1),
            irq_count: AtomicU64::new(0),
            enabled_interrupts: AtomicU64::new(0),
            active_interrupts: AtomicU64::new(0),
            pending_interrupts: AtomicU64::new(0),
            error_interrupts: AtomicU64::new(0),
            total_interrupts: AtomicU64::new(0),
            handled_interrupts: AtomicU64::new(0),
            spurious_interrupts: AtomicU64::new(0),
            interrupt_latency_total: AtomicU64::new(0),
            interrupt_latency_count: AtomicU64::new(0),
            irq_switches: AtomicU64::new(0),
            irq_nesting_depth: AtomicU64::new(0),
        }
    }

    /// Registrar interrupción
    pub fn register_interrupt(&mut self, irq_number: u8, irq_type: IrqType, priority: IrqPriority, device_id: u32, handler_address: u64, handler_data: u64, shared: bool, level_triggered: bool, edge_triggered: bool) -> MemoryResult<u32> {
        let irq_id = self.next_irq_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if irq_id >= 256 {
            return Err(MemoryError::OutOfMemory);
        }

        // Verificar que no existe ya una interrupción con este número
        if self.find_interrupt_by_number(irq_number).is_some() {
            return Err(MemoryError::AlreadyMapped);
        }

        let irq_info = IrqInfo {
            irq_id,
            irq_number,
            irq_type,
            priority,
            state: IrqState::Disabled,
            device_id,
            handler_address,
            handler_data,
            interrupt_count: 0,
            last_interrupt_time: 0,
            average_latency: 0,
            max_latency: 0,
            min_latency: u32::MAX,
            error_count: 0,
            shared,
            level_triggered,
            edge_triggered,
        };

        self.interrupts[irq_id as usize] = Some(irq_info);
        self.irq_count.fetch_add(1, Ordering::SeqCst);

        Ok(irq_id)
    }

    /// Desregistrar interrupción
    pub fn unregister_interrupt(&mut self, irq_id: u32) -> MemoryResult<()> {
        if irq_id >= 256 {
            return Err(MemoryError::InvalidAddress);
        }

        if let Some(irq) = &self.interrupts[irq_id as usize] {
            // Actualizar contadores de estado
            match irq.state {
                IrqState::Enabled => { self.enabled_interrupts.fetch_sub(1, Ordering::SeqCst); }
                IrqState::Active => { self.active_interrupts.fetch_sub(1, Ordering::SeqCst); }
                IrqState::Pending => { self.pending_interrupts.fetch_sub(1, Ordering::SeqCst); }
                IrqState::Error => { self.error_interrupts.fetch_sub(1, Ordering::SeqCst); }
                _ => {}
            }

            self.interrupts[irq_id as usize] = None;
            self.irq_count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener información de interrupción
    pub fn get_interrupt_info(&self, irq_id: u32) -> Option<&IrqInfo> {
        if irq_id >= 256 {
            return None;
        }
        self.interrupts[irq_id as usize].as_ref()
    }

    /// Buscar interrupción por número
    pub fn find_interrupt_by_number(&self, irq_number: u8) -> Option<&IrqInfo> {
        for interrupt in &self.interrupts {
            if let Some(irq) = interrupt {
                if irq.irq_number == irq_number {
                    return Some(irq);
                }
            }
        }
        None
    }

    /// Buscar interrupciones por tipo
    pub fn find_interrupts_by_type(&self, irq_type: IrqType) -> u32 {
        let mut count = 0;
        for interrupt in &self.interrupts {
            if let Some(irq) = interrupt {
                if irq.irq_type == irq_type {
                    count += 1;
                }
            }
        }
        count
    }

    /// Buscar interrupciones por dispositivo
    pub fn find_interrupts_by_device(&self, device_id: u32) -> u32 {
        let mut count = 0;
        for interrupt in &self.interrupts {
            if let Some(irq) = interrupt {
                if irq.device_id == device_id {
                    count += 1;
                }
            }
        }
        count
    }

    /// Habilitar interrupción
    pub fn enable_interrupt(&mut self, irq_id: u32) -> MemoryResult<()> {
        if let Some(irq) = &mut self.interrupts[irq_id as usize] {
            if irq.state != IrqState::Disabled {
                return Err(MemoryError::PermissionDenied);
            }

            irq.state = IrqState::Enabled;
            self.enabled_interrupts.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Deshabilitar interrupción
    pub fn disable_interrupt(&mut self, irq_id: u32) -> MemoryResult<()> {
        if let Some(irq) = &mut self.interrupts[irq_id as usize] {
            if irq.state != IrqState::Enabled {
                return Err(MemoryError::PermissionDenied);
            }

            irq.state = IrqState::Disabled;
            self.enabled_interrupts.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Manejar interrupción
    pub fn handle_interrupt(&mut self, irq_number: u8, interrupt_time: u64) -> MemoryResult<()> {
        self.total_interrupts.fetch_add(1, Ordering::SeqCst);

        // Buscar interrupción por número
        for interrupt in &mut self.interrupts {
            if let Some(irq) = interrupt {
                if irq.irq_number == irq_number {
                    if irq.state != IrqState::Enabled {
                        return Err(MemoryError::PermissionDenied);
                    }

                    // Calcular latencia
                    let latency = if irq.last_interrupt_time > 0 {
                        (interrupt_time - irq.last_interrupt_time) as u32
                    } else {
                        0
                    };

                    // Actualizar estadísticas de latencia
                    if latency > 0 {
                        self.interrupt_latency_total.fetch_add(latency as u64, Ordering::SeqCst);
                        self.interrupt_latency_count.fetch_add(1, Ordering::SeqCst);

                        // Actualizar latencia promedio
                        let total_latency = self.interrupt_latency_total.load(Ordering::SeqCst);
                        let latency_count = self.interrupt_latency_count.load(Ordering::SeqCst);
                        if latency_count > 0 {
                            irq.average_latency = (total_latency / latency_count) as u32;
                        }

                        // Actualizar latencia máxima y mínima
                        if latency > irq.max_latency {
                            irq.max_latency = latency;
                        }
                        if latency < irq.min_latency {
                            irq.min_latency = latency;
                        }
                    }

                    // Actualizar información de interrupción
                    irq.interrupt_count += 1;
                    irq.last_interrupt_time = interrupt_time;
                    irq.state = IrqState::Active;
                    self.active_interrupts.fetch_add(1, Ordering::SeqCst);

                    // Simular manejo de interrupción
                    self.handled_interrupts.fetch_add(1, Ordering::SeqCst);

                    // Cambiar estado a manejada
                    irq.state = IrqState::Handled;
                    self.active_interrupts.fetch_sub(1, Ordering::SeqCst);

                    return Ok(());
                }
            }
        }

        // Interrupción no encontrada - spurious interrupt
        self.spurious_interrupts.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// Establecer interrupción como pendiente
    pub fn set_interrupt_pending(&mut self, irq_id: u32) -> MemoryResult<()> {
        if let Some(irq) = &mut self.interrupts[irq_id as usize] {
            if irq.state != IrqState::Enabled {
                return Err(MemoryError::PermissionDenied);
            }

            irq.state = IrqState::Pending;
            self.pending_interrupts.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Limpiar interrupción pendiente
    pub fn clear_interrupt_pending(&mut self, irq_id: u32) -> MemoryResult<()> {
        if let Some(irq) = &mut self.interrupts[irq_id as usize] {
            if irq.state != IrqState::Pending {
                return Err(MemoryError::PermissionDenied);
            }

            irq.state = IrqState::Enabled;
            self.pending_interrupts.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Establecer estado de error
    pub fn set_interrupt_error(&mut self, irq_id: u32) -> MemoryResult<()> {
        if let Some(irq) = &mut self.interrupts[irq_id as usize] {
            let old_state = irq.state;
            irq.state = IrqState::Error;
            irq.error_count += 1;

            // Actualizar contadores
            match old_state {
                IrqState::Enabled => { self.enabled_interrupts.fetch_sub(1, Ordering::SeqCst); }
                IrqState::Active => { self.active_interrupts.fetch_sub(1, Ordering::SeqCst); }
                IrqState::Pending => { self.pending_interrupts.fetch_sub(1, Ordering::SeqCst); }
                _ => {}
            }
            self.error_interrupts.fetch_add(1, Ordering::SeqCst);

            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Cambiar prioridad de interrupción
    pub fn set_interrupt_priority(&mut self, irq_id: u32, priority: IrqPriority) -> MemoryResult<()> {
        if let Some(irq) = &mut self.interrupts[irq_id as usize] {
            irq.priority = priority;
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener interrupciones por prioridad
    pub fn get_interrupts_by_priority(&self, priority: IrqPriority) -> u32 {
        let mut count = 0;
        for interrupt in &self.interrupts {
            if let Some(irq) = interrupt {
                if irq.priority == priority {
                    count += 1;
                }
            }
        }
        count
    }

    /// Obtener estadísticas de IRQ
    pub fn get_irq_stats(&self) -> IrqStats {
        let average_latency = if self.interrupt_latency_count.load(Ordering::SeqCst) > 0 {
            (self.interrupt_latency_total.load(Ordering::SeqCst) / self.interrupt_latency_count.load(Ordering::SeqCst)) as u32
        } else {
            0
        };

        IrqStats {
            irq_count: self.irq_count.load(Ordering::SeqCst),
            enabled_interrupts: self.enabled_interrupts.load(Ordering::SeqCst),
            active_interrupts: self.active_interrupts.load(Ordering::SeqCst),
            pending_interrupts: self.pending_interrupts.load(Ordering::SeqCst),
            error_interrupts: self.error_interrupts.load(Ordering::SeqCst),
            total_interrupts: self.total_interrupts.load(Ordering::SeqCst),
            handled_interrupts: self.handled_interrupts.load(Ordering::SeqCst),
            spurious_interrupts: self.spurious_interrupts.load(Ordering::SeqCst),
            average_latency,
            irq_switches: self.irq_switches.load(Ordering::SeqCst),
            irq_nesting_depth: self.irq_nesting_depth.load(Ordering::SeqCst),
        }
    }
}

/// Estadísticas de IRQ
#[derive(Debug, Clone, Copy)]
pub struct IrqStats {
    pub irq_count: u64,
    pub enabled_interrupts: u64,
    pub active_interrupts: u64,
    pub pending_interrupts: u64,
    pub error_interrupts: u64,
    pub total_interrupts: u64,
    pub handled_interrupts: u64,
    pub spurious_interrupts: u64,
    pub average_latency: u32,
    pub irq_switches: u64,
    pub irq_nesting_depth: u64,
}

/// Inicializar el IRQ manager
pub fn init() -> MemoryResult<()> {
    // En una implementación completa, esto inicializaría:
    // - IRQ manager
    // - Interrupt handlers
    // - Interrupt controllers
    // - Interrupt routing
    
    Ok(())
}

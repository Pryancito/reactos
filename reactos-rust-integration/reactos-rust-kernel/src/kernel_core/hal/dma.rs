//! # DMA Manager
//! 
//! Gestión de DMA (Direct Memory Access)

use crate::kernel_core::memory::{MemoryResult, MemoryError};
use core::sync::atomic::{AtomicU64, Ordering};

/// Tipo de transferencia DMA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaTransferType {
    MemoryToDevice,     // Memoria a dispositivo
    DeviceToMemory,     // Dispositivo a memoria
    MemoryToMemory,     // Memoria a memoria
    ScatterGather,      // Scatter-gather
    Cyclic,             // Cíclica
    Interleaved,        // Entrelazada
    Unknown,            // Tipo desconocido
}

/// Estado de transferencia DMA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaTransferState {
    Idle,               // Inactiva
    Prepared,           // Preparada
    InProgress,         // En progreso
    Completed,          // Completada
    Error,              // Error
    Cancelled,          // Cancelada
}

/// Prioridad de DMA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaPriority {
    Critical,           // Crítica
    High,               // Alta
    Normal,             // Normal
    Low,                // Baja
    Background,         // Fondo
}

/// Información de transferencia DMA
#[derive(Debug, Clone, Copy)]
pub struct DmaTransferInfo {
    pub transfer_id: u32,
    pub device_id: u32,
    pub transfer_type: DmaTransferType,
    pub state: DmaTransferState,
    pub priority: DmaPriority,
    pub source_address: u64,
    pub destination_address: u64,
    pub transfer_size: u32,
    pub transfer_count: u32,
    pub bytes_transferred: u32,
    pub transfer_rate: u32,        // Bytes por segundo
    pub start_time: u64,
    pub end_time: u64,
    pub error_count: u64,
    pub retry_count: u32,
    pub channel: u8,
    pub scatter_gather: bool,
    pub cyclic: bool,
    pub interleaved: bool,
}

/// Manager de DMA
pub struct DmaManager {
    transfers: [Option<DmaTransferInfo>; 64], // Array fijo para evitar Vec
    next_transfer_id: AtomicU64,
    transfer_count: AtomicU64,
    active_transfers: AtomicU64,
    completed_transfers: AtomicU64,
    error_transfers: AtomicU64,
    total_bytes_transferred: AtomicU64,
    total_transfer_time: AtomicU64,
    dma_channels: [bool; 16],      // Canales DMA disponibles
    channel_usage: [u32; 16],      // Uso de cada canal
    dma_errors: AtomicU64,
    dma_retries: AtomicU64,
    dma_timeouts: AtomicU64,
    dma_interrupts: AtomicU64,
    dma_aborts: AtomicU64,
}

impl DmaManager {
    pub fn new() -> Self {
        Self {
            transfers: [(); 64].map(|_| None),
            next_transfer_id: AtomicU64::new(1),
            transfer_count: AtomicU64::new(0),
            active_transfers: AtomicU64::new(0),
            completed_transfers: AtomicU64::new(0),
            error_transfers: AtomicU64::new(0),
            total_bytes_transferred: AtomicU64::new(0),
            total_transfer_time: AtomicU64::new(0),
            dma_channels: [true; 16], // Todos los canales disponibles inicialmente
            channel_usage: [0; 16],
            dma_errors: AtomicU64::new(0),
            dma_retries: AtomicU64::new(0),
            dma_timeouts: AtomicU64::new(0),
            dma_interrupts: AtomicU64::new(0),
            dma_aborts: AtomicU64::new(0),
        }
    }

    /// Crear transferencia DMA
    pub fn create_transfer(&mut self, device_id: u32, transfer_type: DmaTransferType, priority: DmaPriority, source_address: u64, destination_address: u64, transfer_size: u32, transfer_count: u32, scatter_gather: bool, cyclic: bool, interleaved: bool) -> MemoryResult<u32> {
        let transfer_id = self.next_transfer_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if transfer_id >= 64 {
            return Err(MemoryError::OutOfMemory);
        }

        // Buscar canal DMA disponible
        let channel = self.find_available_channel()?;

        let transfer_info = DmaTransferInfo {
            transfer_id,
            device_id,
            transfer_type,
            state: DmaTransferState::Idle,
            priority,
            source_address,
            destination_address,
            transfer_size,
            transfer_count,
            bytes_transferred: 0,
            transfer_rate: 0,
            start_time: 0,
            end_time: 0,
            error_count: 0,
            retry_count: 0,
            channel,
            scatter_gather,
            cyclic,
            interleaved,
        };

        self.transfers[transfer_id as usize] = Some(transfer_info);
        self.transfer_count.fetch_add(1, Ordering::SeqCst);
        self.channel_usage[channel as usize] += 1;

        Ok(transfer_id)
    }

    /// Eliminar transferencia DMA
    pub fn delete_transfer(&mut self, transfer_id: u32) -> MemoryResult<()> {
        if transfer_id >= 64 {
            return Err(MemoryError::InvalidAddress);
        }

        if let Some(transfer) = &self.transfers[transfer_id as usize] {
            // Liberar canal DMA
            self.dma_channels[transfer.channel as usize] = true;
            self.channel_usage[transfer.channel as usize] -= 1;

            // Actualizar contadores de estado
            match transfer.state {
                DmaTransferState::InProgress => { self.active_transfers.fetch_sub(1, Ordering::SeqCst); }
                DmaTransferState::Completed => { self.completed_transfers.fetch_sub(1, Ordering::SeqCst); }
                DmaTransferState::Error => { self.error_transfers.fetch_sub(1, Ordering::SeqCst); }
                _ => {}
            }

            self.transfers[transfer_id as usize] = None;
            self.transfer_count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener información de transferencia
    pub fn get_transfer_info(&self, transfer_id: u32) -> Option<&DmaTransferInfo> {
        if transfer_id >= 64 {
            return None;
        }
        self.transfers[transfer_id as usize].as_ref()
    }

    /// Preparar transferencia DMA
    pub fn prepare_transfer(&mut self, transfer_id: u32) -> MemoryResult<()> {
        if let Some(transfer) = &mut self.transfers[transfer_id as usize] {
            if transfer.state != DmaTransferState::Idle {
                return Err(MemoryError::PermissionDenied);
            }

            transfer.state = DmaTransferState::Prepared;
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Iniciar transferencia DMA
    pub fn start_transfer(&mut self, transfer_id: u32, start_time: u64) -> MemoryResult<()> {
        if let Some(transfer) = &mut self.transfers[transfer_id as usize] {
            if transfer.state != DmaTransferState::Prepared {
                return Err(MemoryError::PermissionDenied);
            }

            transfer.state = DmaTransferState::InProgress;
            transfer.start_time = start_time;
            self.active_transfers.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Completar transferencia DMA
    pub fn complete_transfer(&mut self, transfer_id: u32, end_time: u64, bytes_transferred: u32) -> MemoryResult<()> {
        if let Some(transfer) = &mut self.transfers[transfer_id as usize] {
            if transfer.state != DmaTransferState::InProgress {
                return Err(MemoryError::PermissionDenied);
            }

            transfer.state = DmaTransferState::Completed;
            transfer.end_time = end_time;
            transfer.bytes_transferred = bytes_transferred;

            // Calcular tasa de transferencia
            let transfer_time = end_time - transfer.start_time;
            if transfer_time > 0 {
                transfer.transfer_rate = (bytes_transferred as u64 * 1000000 / transfer_time) as u32; // Bytes por segundo
            }

            self.active_transfers.fetch_sub(1, Ordering::SeqCst);
            self.completed_transfers.fetch_add(1, Ordering::SeqCst);
            self.total_bytes_transferred.fetch_add(bytes_transferred as u64, Ordering::SeqCst);
            self.total_transfer_time.fetch_add(transfer_time, Ordering::SeqCst);

            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Cancelar transferencia DMA
    pub fn cancel_transfer(&mut self, transfer_id: u32) -> MemoryResult<()> {
        if let Some(transfer) = &mut self.transfers[transfer_id as usize] {
            if transfer.state != DmaTransferState::InProgress {
                return Err(MemoryError::PermissionDenied);
            }

            transfer.state = DmaTransferState::Cancelled;
            self.active_transfers.fetch_sub(1, Ordering::SeqCst);
            self.dma_aborts.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Establecer error en transferencia
    pub fn set_transfer_error(&mut self, transfer_id: u32) -> MemoryResult<()> {
        if let Some(transfer) = &mut self.transfers[transfer_id as usize] {
            let old_state = transfer.state;
            transfer.state = DmaTransferState::Error;
            transfer.error_count += 1;

            // Actualizar contadores
            match old_state {
                DmaTransferState::InProgress => { self.active_transfers.fetch_sub(1, Ordering::SeqCst); }
                _ => {}
            }
            self.error_transfers.fetch_add(1, Ordering::SeqCst);
            self.dma_errors.fetch_add(1, Ordering::SeqCst);

            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Reintentar transferencia
    pub fn retry_transfer(&mut self, transfer_id: u32) -> MemoryResult<()> {
        if let Some(transfer) = &mut self.transfers[transfer_id as usize] {
            if transfer.state != DmaTransferState::Error {
                return Err(MemoryError::PermissionDenied);
            }

            transfer.state = DmaTransferState::Prepared;
            transfer.retry_count += 1;
            self.dma_retries.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Manejar interrupción DMA
    pub fn handle_dma_interrupt(&mut self, channel: u8) -> MemoryResult<()> {
        self.dma_interrupts.fetch_add(1, Ordering::SeqCst);

        // Buscar transferencia en este canal
        for transfer in &mut self.transfers {
            if let Some(t) = transfer {
                if t.channel == channel && t.state == DmaTransferState::InProgress {
                    // Simular finalización de transferencia
                    t.state = DmaTransferState::Completed;
                    t.bytes_transferred = t.transfer_size;
                    self.active_transfers.fetch_sub(1, Ordering::SeqCst);
                    self.completed_transfers.fetch_add(1, Ordering::SeqCst);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Registrar timeout DMA
    pub fn register_dma_timeout(&mut self, transfer_id: u32) -> MemoryResult<()> {
        if let Some(transfer) = &mut self.transfers[transfer_id as usize] {
            if transfer.state != DmaTransferState::InProgress {
                return Err(MemoryError::PermissionDenied);
            }

            transfer.state = DmaTransferState::Error;
            transfer.error_count += 1;
            self.dma_timeouts.fetch_add(1, Ordering::SeqCst);
            self.dma_errors.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Buscar canal DMA disponible
    fn find_available_channel(&self) -> MemoryResult<u8> {
        for (i, &available) in self.dma_channels.iter().enumerate() {
            if available {
                return Ok(i as u8);
            }
        }
        Err(MemoryError::OutOfMemory)
    }

    /// Obtener transferencias por dispositivo
    pub fn get_transfers_by_device(&self, device_id: u32) -> u32 {
        let mut count = 0;
        for transfer in &self.transfers {
            if let Some(t) = transfer {
                if t.device_id == device_id {
                    count += 1;
                }
            }
        }
        count
    }

    /// Obtener transferencias por tipo
    pub fn get_transfers_by_type(&self, transfer_type: DmaTransferType) -> u32 {
        let mut count = 0;
        for transfer in &self.transfers {
            if let Some(t) = transfer {
                if t.transfer_type == transfer_type {
                    count += 1;
                }
            }
        }
        count
    }

    /// Obtener transferencias por prioridad
    pub fn get_transfers_by_priority(&self, priority: DmaPriority) -> u32 {
        let mut count = 0;
        for transfer in &self.transfers {
            if let Some(t) = transfer {
                if t.priority == priority {
                    count += 1;
                }
            }
        }
        count
    }

    /// Obtener estadísticas de DMA
    pub fn get_dma_stats(&self) -> DmaStats {
        let average_transfer_rate = if self.completed_transfers.load(Ordering::SeqCst) > 0 {
            (self.total_bytes_transferred.load(Ordering::SeqCst) * 1000000 / self.total_transfer_time.load(Ordering::SeqCst)) as u32
        } else {
            0
        };

        DmaStats {
            transfer_count: self.transfer_count.load(Ordering::SeqCst),
            active_transfers: self.active_transfers.load(Ordering::SeqCst),
            completed_transfers: self.completed_transfers.load(Ordering::SeqCst),
            error_transfers: self.error_transfers.load(Ordering::SeqCst),
            total_bytes_transferred: self.total_bytes_transferred.load(Ordering::SeqCst),
            average_transfer_rate,
            dma_errors: self.dma_errors.load(Ordering::SeqCst),
            dma_retries: self.dma_retries.load(Ordering::SeqCst),
            dma_timeouts: self.dma_timeouts.load(Ordering::SeqCst),
            dma_interrupts: self.dma_interrupts.load(Ordering::SeqCst),
            dma_aborts: self.dma_aborts.load(Ordering::SeqCst),
        }
    }
}

/// Estadísticas de DMA
#[derive(Debug, Clone, Copy)]
pub struct DmaStats {
    pub transfer_count: u64,
    pub active_transfers: u64,
    pub completed_transfers: u64,
    pub error_transfers: u64,
    pub total_bytes_transferred: u64,
    pub average_transfer_rate: u32,
    pub dma_errors: u64,
    pub dma_retries: u64,
    pub dma_timeouts: u64,
    pub dma_interrupts: u64,
    pub dma_aborts: u64,
}

/// Inicializar el DMA manager
pub fn init() -> MemoryResult<()> {
    // En una implementación completa, esto inicializaría:
    // - DMA manager
    // - DMA channels
    // - DMA controllers
    // - DMA interrupt handlers
    
    Ok(())
}

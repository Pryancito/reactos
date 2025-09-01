//! # Disk Manager
//! 
//! Gestión de discos y particiones

use crate::kernel_core::memory::{MemoryResult, MemoryError};
use core::sync::atomic::{AtomicU64, Ordering};

/// Tipo de tabla de particiones
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionTableType {
    MBR,        // Master Boot Record
    GPT,        // GUID Partition Table
    APM,        // Apple Partition Map
    Unknown,    // Tipo desconocido
}

/// Tipo de partición
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionType {
    Primary,    // Partición primaria
    Extended,   // Partición extendida
    Logical,    // Partición lógica
    EFI,        // Partición EFI
    Swap,       // Partición de intercambio
    Data,       // Partición de datos
    Unknown,    // Tipo desconocido
}

/// Estado de la partición
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionState {
    Unmounted,  // Desmontada
    Mounted,    // Montada
    Error,      // Error
    Corrupted,  // Corrupta
}

/// Información de la partición
#[derive(Debug, Clone, Copy)]
pub struct PartitionInfo {
    pub partition_id: u32,
    pub disk_id: u32,
    pub partition_type: PartitionType,
    pub state: PartitionState,
    pub start_sector: u64,
    pub end_sector: u64,
    pub size_bytes: u64,
    pub filesystem_type: &'static str,
    pub mount_point: &'static str,
    pub bootable: bool,
    pub encrypted: bool,
}

/// Información del disco
#[derive(Debug, Clone, Copy)]
pub struct DiskInfo {
    pub disk_id: u32,
    pub device_id: u32,
    pub partition_table_type: PartitionTableType,
    pub total_sectors: u64,
    pub sector_size: u32,
    pub partition_count: u8,
    pub bootable: bool,
    pub removable: bool,
    pub hot_swappable: bool,
}

/// Manager de discos
pub struct DiskManager {
    disks: [Option<DiskInfo>; 32],           // Array fijo para evitar Vec
    partitions: [Option<PartitionInfo>; 256], // Array fijo para evitar Vec
    next_disk_id: AtomicU64,
    next_partition_id: AtomicU64,
    disk_count: AtomicU64,
    partition_count: AtomicU64,
    total_disk_capacity: AtomicU64,          // Capacidad total en bytes
    total_partition_capacity: AtomicU64,     // Capacidad total de particiones en bytes
    disk_operations: AtomicU64,              // Operaciones de disco
    partition_operations: AtomicU64,         // Operaciones de partición
    mount_operations: AtomicU64,             // Operaciones de montaje
    unmount_operations: AtomicU64,           // Operaciones de desmontaje
}

impl DiskManager {
    pub fn new() -> Self {
        Self {
            disks: [(); 32].map(|_| None),
            partitions: [(); 256].map(|_| None),
            next_disk_id: AtomicU64::new(1),
            next_partition_id: AtomicU64::new(1),
            disk_count: AtomicU64::new(0),
            partition_count: AtomicU64::new(0),
            total_disk_capacity: AtomicU64::new(0),
            total_partition_capacity: AtomicU64::new(0),
            disk_operations: AtomicU64::new(0),
            partition_operations: AtomicU64::new(0),
            mount_operations: AtomicU64::new(0),
            unmount_operations: AtomicU64::new(0),
        }
    }

    /// Registrar disco
    pub fn register_disk(&mut self, device_id: u32, partition_table_type: PartitionTableType, total_sectors: u64, sector_size: u32, removable: bool, hot_swappable: bool) -> MemoryResult<u32> {
        let disk_id = self.next_disk_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if disk_id >= 32 {
            return Err(MemoryError::OutOfMemory);
        }

        let disk_info = DiskInfo {
            disk_id,
            device_id,
            partition_table_type,
            total_sectors,
            sector_size,
            partition_count: 0,
            bootable: false,
            removable,
            hot_swappable,
        };

        self.disks[disk_id as usize] = Some(disk_info);
        self.disk_count.fetch_add(1, Ordering::SeqCst);
        self.total_disk_capacity.fetch_add(total_sectors * sector_size as u64, Ordering::SeqCst);

        Ok(disk_id)
    }

    /// Desregistrar disco
    pub fn unregister_disk(&mut self, disk_id: u32) -> MemoryResult<()> {
        if disk_id >= 32 {
            return Err(MemoryError::InvalidAddress);
        }

        if let Some(disk) = &self.disks[disk_id as usize] {
            // Desregistrar todas las particiones del disco
            for partition in &mut self.partitions {
                if let Some(part) = partition {
                    if part.disk_id == disk_id {
                        *partition = None;
                        self.partition_count.fetch_sub(1, Ordering::SeqCst);
                    }
                }
            }

            self.total_disk_capacity.fetch_sub(disk.total_sectors * disk.sector_size as u64, Ordering::SeqCst);
            self.disks[disk_id as usize] = None;
            self.disk_count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Crear partición
    pub fn create_partition(&mut self, disk_id: u32, partition_type: PartitionType, start_sector: u64, end_sector: u64, filesystem_type: &'static str, bootable: bool, encrypted: bool) -> MemoryResult<u32> {
        let partition_id = self.next_partition_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if partition_id >= 256 {
            return Err(MemoryError::OutOfMemory);
        }

        // Verificar que el disco existe
        if let Some(disk) = &mut self.disks[disk_id as usize] {
            // Verificar límites
            if end_sector > disk.total_sectors {
                return Err(MemoryError::InvalidAddress);
            }

            let size_bytes = (end_sector - start_sector) * disk.sector_size as u64;

            let partition_info = PartitionInfo {
                partition_id,
                disk_id,
                partition_type,
                state: PartitionState::Unmounted,
                start_sector,
                end_sector,
                size_bytes,
                filesystem_type,
                mount_point: "",
                bootable,
                encrypted,
            };

            self.partitions[partition_id as usize] = Some(partition_info);
            self.partition_count.fetch_add(1, Ordering::SeqCst);
            self.total_partition_capacity.fetch_add(size_bytes, Ordering::SeqCst);
            disk.partition_count += 1;

            Ok(partition_id)
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Eliminar partición
    pub fn delete_partition(&mut self, partition_id: u32) -> MemoryResult<()> {
        if partition_id >= 256 {
            return Err(MemoryError::InvalidAddress);
        }

        if let Some(partition) = &self.partitions[partition_id as usize] {
            // Actualizar contador de particiones del disco
            if let Some(disk) = &mut self.disks[partition.disk_id as usize] {
                disk.partition_count -= 1;
            }

            self.total_partition_capacity.fetch_sub(partition.size_bytes, Ordering::SeqCst);
            self.partitions[partition_id as usize] = None;
            self.partition_count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Montar partición
    pub fn mount_partition(&mut self, partition_id: u32, mount_point: &'static str) -> MemoryResult<()> {
        if let Some(partition) = &mut self.partitions[partition_id as usize] {
            if partition.state == PartitionState::Mounted {
                return Err(MemoryError::AlreadyMapped);
            }

            partition.state = PartitionState::Mounted;
            partition.mount_point = mount_point;
            self.mount_operations.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Desmontar partición
    pub fn unmount_partition(&mut self, partition_id: u32) -> MemoryResult<()> {
        if let Some(partition) = &mut self.partitions[partition_id as usize] {
            if partition.state != PartitionState::Mounted {
                return Err(MemoryError::PermissionDenied);
            }

            partition.state = PartitionState::Unmounted;
            partition.mount_point = "";
            self.unmount_operations.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener información del disco
    pub fn get_disk_info(&self, disk_id: u32) -> Option<&DiskInfo> {
        if disk_id >= 32 {
            return None;
        }
        self.disks[disk_id as usize].as_ref()
    }

    /// Obtener información de la partición
    pub fn get_partition_info(&self, partition_id: u32) -> Option<&PartitionInfo> {
        if partition_id >= 256 {
            return None;
        }
        self.partitions[partition_id as usize].as_ref()
    }

    /// Obtener particiones de un disco
    pub fn get_disk_partitions(&self, disk_id: u32) -> u32 {
        let mut count = 0;
        for partition in &self.partitions {
            if let Some(part) = partition {
                if part.disk_id == disk_id {
                    count += 1;
                }
            }
        }
        count
    }

    /// Establecer disco como booteable
    pub fn set_disk_bootable(&mut self, disk_id: u32, bootable: bool) -> MemoryResult<()> {
        if let Some(disk) = &mut self.disks[disk_id as usize] {
            disk.bootable = bootable;
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener estadísticas de discos
    pub fn get_disk_stats(&self) -> DiskStats {
        DiskStats {
            disk_count: self.disk_count.load(Ordering::SeqCst),
            partition_count: self.partition_count.load(Ordering::SeqCst),
            total_disk_capacity: self.total_disk_capacity.load(Ordering::SeqCst),
            total_partition_capacity: self.total_partition_capacity.load(Ordering::SeqCst),
            disk_operations: self.disk_operations.load(Ordering::SeqCst),
            partition_operations: self.partition_operations.load(Ordering::SeqCst),
            mount_operations: self.mount_operations.load(Ordering::SeqCst),
            unmount_operations: self.unmount_operations.load(Ordering::SeqCst),
        }
    }
}

/// Estadísticas de discos
#[derive(Debug, Clone, Copy)]
pub struct DiskStats {
    pub disk_count: u64,
    pub partition_count: u64,
    pub total_disk_capacity: u64,
    pub total_partition_capacity: u64,
    pub disk_operations: u64,
    pub partition_operations: u64,
    pub mount_operations: u64,
    pub unmount_operations: u64,
}

/// Inicializar el disk manager
pub fn init() -> MemoryResult<()> {
    // En una implementación completa, esto inicializaría:
    // - Disk manager
    // - Partition management
    // - Mount/unmount operations
    // - Filesystem detection
    
    Ok(())
}

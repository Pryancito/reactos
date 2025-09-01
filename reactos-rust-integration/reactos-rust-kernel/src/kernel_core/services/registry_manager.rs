//! # Registry Manager
//! 
//! Gestor del registro del sistema

use crate::kernel_core::memory::{MemoryResult, MemoryError};
use core::sync::atomic::{AtomicU64, Ordering};

/// Tipo de clave del registro
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryKeyType {
    HKEY_LOCAL_MACHINE,    // HKLM
    HKEY_CURRENT_USER,     // HKCU
    HKEY_CLASSES_ROOT,     // HKCR
    HKEY_USERS,            // HKU
    HKEY_CURRENT_CONFIG,   // HKCC
    HKEY_DYN_DATA,         // HKDD
    HKEY_PERFORMANCE_DATA, // HKPD
    HKEY_PERFORMANCE_TEXT, // HKPT
    HKEY_PERFORMANCE_NLSTEXT, // HKPN
}

/// Tipo de valor del registro
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryValueType {
    REG_NONE,           // Ninguno
    REG_SZ,             // Cadena
    REG_EXPAND_SZ,      // Cadena expandible
    REG_BINARY,         // Binario
    REG_DWORD,          // DWORD
    REG_DWORD_BIG_ENDIAN, // DWORD big-endian
    REG_LINK,           // Enlace
    REG_MULTI_SZ,       // Múltiples cadenas
    REG_RESOURCE_LIST,  // Lista de recursos
    REG_FULL_RESOURCE_DESCRIPTOR, // Descriptor completo de recursos
    REG_RESOURCE_REQUIREMENTS_LIST, // Lista de requisitos de recursos
    REG_QWORD,          // QWORD
}

/// Estado de la clave del registro
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryKeyState {
    Open,       // Abierta
    Closed,     // Cerrada
    Locked,     // Bloqueada
    Error,      // Error
}

/// Información de clave del registro
#[derive(Debug, Clone, Copy)]
pub struct RegistryKeyInfo {
    pub key_id: u32,
    pub key_type: RegistryKeyType,
    pub state: RegistryKeyState,
    pub name: [u8; 256],       // Nombre de la clave
    pub path: [u8; 512],       // Ruta de la clave
    pub parent_id: u32,        // ID de la clave padre
    pub child_count: u32,      // Número de claves hijas
    pub value_count: u32,      // Número de valores
    pub last_modified: u64,    // Última modificación
    pub created_time: u64,     // Tiempo de creación
    pub access_rights: u32,    // Derechos de acceso
    pub security_descriptor: u64, // Descriptor de seguridad
    pub class_name: [u8; 128], // Nombre de clase
    pub max_value_name_len: u32, // Longitud máxima del nombre de valor
    pub max_value_data_len: u32, // Longitud máxima de los datos de valor
    pub max_class_name_len: u32, // Longitud máxima del nombre de clase
    pub max_subkey_name_len: u32, // Longitud máxima del nombre de subclave
}

/// Información de valor del registro
#[derive(Debug, Clone, Copy)]
pub struct RegistryValueInfo {
    pub value_id: u32,
    pub key_id: u32,
    pub value_type: RegistryValueType,
    pub name: [u8; 256],       // Nombre del valor
    pub data: [u8; 1024],      // Datos del valor
    pub data_size: u32,        // Tamaño de los datos
    pub last_modified: u64,    // Última modificación
    pub created_time: u64,     // Tiempo de creación
    pub access_rights: u32,    // Derechos de acceso
    pub security_descriptor: u64, // Descriptor de seguridad
    pub is_encrypted: bool,    // Encriptado
    pub is_volatile: bool,     // Volátil
    pub checksum: u32,         // Checksum
}

/// Manager del registro
pub struct RegistryManager {
    keys: [Option<RegistryKeyInfo>; 1024], // Array fijo para evitar Vec
    values: [Option<RegistryValueInfo>; 2048], // Array fijo para evitar Vec
    next_key_id: AtomicU64,
    next_value_id: AtomicU64,
    key_count: AtomicU64,
    value_count: AtomicU64,
    open_keys: AtomicU64,
    locked_keys: AtomicU64,
    error_keys: AtomicU64,
    total_keys: AtomicU64,
    total_values: AtomicU64,
    key_creates: AtomicU64,
    key_deletes: AtomicU64,
    key_opens: AtomicU64,
    key_closes: AtomicU64,
    value_creates: AtomicU64,
    value_deletes: AtomicU64,
    value_reads: AtomicU64,
    value_writes: AtomicU64,
    registry_backups: AtomicU64,
    registry_restores: AtomicU64,
    registry_compacts: AtomicU64,
    registry_repairs: AtomicU64,
}

impl RegistryManager {
    pub fn new() -> Self {
        Self {
            keys: [(); 1024].map(|_| None),
            values: [(); 2048].map(|_| None),
            next_key_id: AtomicU64::new(1),
            next_value_id: AtomicU64::new(1),
            key_count: AtomicU64::new(0),
            value_count: AtomicU64::new(0),
            open_keys: AtomicU64::new(0),
            locked_keys: AtomicU64::new(0),
            error_keys: AtomicU64::new(0),
            total_keys: AtomicU64::new(0),
            total_values: AtomicU64::new(0),
            key_creates: AtomicU64::new(0),
            key_deletes: AtomicU64::new(0),
            key_opens: AtomicU64::new(0),
            key_closes: AtomicU64::new(0),
            value_creates: AtomicU64::new(0),
            value_deletes: AtomicU64::new(0),
            value_reads: AtomicU64::new(0),
            value_writes: AtomicU64::new(0),
            registry_backups: AtomicU64::new(0),
            registry_restores: AtomicU64::new(0),
            registry_compacts: AtomicU64::new(0),
            registry_repairs: AtomicU64::new(0),
        }
    }

    /// Crear clave del registro
    pub fn create_key(&mut self, key_type: RegistryKeyType, name: &str, path: &str, parent_id: u32, access_rights: u32, security_descriptor: u64, class_name: &str, current_time: u64) -> MemoryResult<u32> {
        let key_id = self.next_key_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if key_id >= 1024 {
            return Err(MemoryError::OutOfMemory);
        }

        // Verificar que no existe ya una clave con este nombre
        if self.find_key_by_name(name).is_some() {
            return Err(MemoryError::AlreadyMapped);
        }

        let mut name_bytes = [0u8; 256];
        let name_data = name.as_bytes();
        let name_len = name_data.len().min(255);
        name_bytes[..name_len].copy_from_slice(&name_data[..name_len]);

        let mut path_bytes = [0u8; 512];
        let path_data = path.as_bytes();
        let path_len = path_data.len().min(511);
        path_bytes[..path_len].copy_from_slice(&path_data[..path_len]);

        let mut class_name_bytes = [0u8; 128];
        let class_name_data = class_name.as_bytes();
        let class_name_len = class_name_data.len().min(127);
        class_name_bytes[..class_name_len].copy_from_slice(&class_name_data[..class_name_len]);

        let key_info = RegistryKeyInfo {
            key_id,
            key_type,
            state: RegistryKeyState::Closed,
            name: name_bytes,
            path: path_bytes,
            parent_id,
            child_count: 0,
            value_count: 0,
            last_modified: current_time,
            created_time: current_time,
            access_rights,
            security_descriptor,
            class_name: class_name_bytes,
            max_value_name_len: 256,
            max_value_data_len: 1024,
            max_class_name_len: 128,
            max_subkey_name_len: 256,
        };

        self.keys[key_id as usize] = Some(key_info);
        self.key_count.fetch_add(1, Ordering::SeqCst);
        self.total_keys.fetch_add(1, Ordering::SeqCst);
        self.key_creates.fetch_add(1, Ordering::SeqCst);

        Ok(key_id)
    }

    /// Eliminar clave del registro
    pub fn delete_key(&mut self, key_id: u32) -> MemoryResult<()> {
        if key_id >= 1024 {
            return Err(MemoryError::InvalidAddress);
        }

        if let Some(key) = &self.keys[key_id as usize] {
            // Verificar que la clave esté cerrada
            if key.state != RegistryKeyState::Closed {
                return Err(MemoryError::PermissionDenied);
            }

            // Verificar que no tenga claves hijas
            if key.child_count > 0 {
                return Err(MemoryError::PermissionDenied);
            }

            // Eliminar valores asociados
            for value in &mut self.values {
                if let Some(v) = value {
                    if v.key_id == key_id {
                        *value = None;
                        self.value_count.fetch_sub(1, Ordering::SeqCst);
                    }
                }
            }

            self.keys[key_id as usize] = None;
            self.key_count.fetch_sub(1, Ordering::SeqCst);
            self.key_deletes.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener información de clave
    pub fn get_key_info(&self, key_id: u32) -> Option<&RegistryKeyInfo> {
        if key_id >= 1024 {
            return None;
        }
        self.keys[key_id as usize].as_ref()
    }

    /// Buscar clave por nombre
    pub fn find_key_by_name(&self, name: &str) -> Option<&RegistryKeyInfo> {
        for key in &self.keys {
            if let Some(k) = key {
                let key_name = core::str::from_utf8(&k.name).unwrap_or("");
                if key_name == name {
                    return Some(k);
                }
            }
        }
        None
    }

    /// Buscar claves por tipo
    pub fn find_keys_by_type(&self, key_type: RegistryKeyType) -> u32 {
        let mut count = 0;
        for key in &self.keys {
            if let Some(k) = key {
                if k.key_type == key_type {
                    count += 1;
                }
            }
        }
        count
    }

    /// Abrir clave del registro
    pub fn open_key(&mut self, key_id: u32) -> MemoryResult<()> {
        if let Some(key) = &mut self.keys[key_id as usize] {
            if key.state != RegistryKeyState::Closed {
                return Err(MemoryError::PermissionDenied);
            }

            key.state = RegistryKeyState::Open;
            self.open_keys.fetch_add(1, Ordering::SeqCst);
            self.key_opens.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Cerrar clave del registro
    pub fn close_key(&mut self, key_id: u32) -> MemoryResult<()> {
        if let Some(key) = &mut self.keys[key_id as usize] {
            if key.state != RegistryKeyState::Open {
                return Err(MemoryError::PermissionDenied);
            }

            key.state = RegistryKeyState::Closed;
            self.open_keys.fetch_sub(1, Ordering::SeqCst);
            self.key_closes.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Bloquear clave del registro
    pub fn lock_key(&mut self, key_id: u32) -> MemoryResult<()> {
        if let Some(key) = &mut self.keys[key_id as usize] {
            if key.state != RegistryKeyState::Open {
                return Err(MemoryError::PermissionDenied);
            }

            key.state = RegistryKeyState::Locked;
            self.locked_keys.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Desbloquear clave del registro
    pub fn unlock_key(&mut self, key_id: u32) -> MemoryResult<()> {
        if let Some(key) = &mut self.keys[key_id as usize] {
            if key.state != RegistryKeyState::Locked {
                return Err(MemoryError::PermissionDenied);
            }

            key.state = RegistryKeyState::Open;
            self.locked_keys.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Crear valor del registro
    pub fn create_value(&mut self, key_id: u32, value_type: RegistryValueType, name: &str, data: &[u8], access_rights: u32, security_descriptor: u64, is_encrypted: bool, is_volatile: bool, current_time: u64) -> MemoryResult<u32> {
        let value_id = self.next_value_id.fetch_add(1, Ordering::SeqCst) as u32;
        
        if value_id >= 2048 {
            return Err(MemoryError::OutOfMemory);
        }

        // Verificar que la clave existe y está abierta
        if let Some(key) = &self.keys[key_id as usize] {
            if key.state != RegistryKeyState::Open && key.state != RegistryKeyState::Locked {
                return Err(MemoryError::PermissionDenied);
            }
        } else {
            return Err(MemoryError::InvalidAddress);
        }

        let mut name_bytes = [0u8; 256];
        let name_data = name.as_bytes();
        let name_len = name_data.len().min(255);
        name_bytes[..name_len].copy_from_slice(&name_data[..name_len]);

        let mut data_bytes = [0u8; 1024];
        let data_len = data.len().min(1024);
        data_bytes[..data_len].copy_from_slice(&data[..data_len]);

        let value_info = RegistryValueInfo {
            value_id,
            key_id,
            value_type,
            name: name_bytes,
            data: data_bytes,
            data_size: data_len as u32,
            last_modified: current_time,
            created_time: current_time,
            access_rights,
            security_descriptor,
            is_encrypted,
            is_volatile,
            checksum: 0,
        };

        self.values[value_id as usize] = Some(value_info);
        self.value_count.fetch_add(1, Ordering::SeqCst);
        self.total_values.fetch_add(1, Ordering::SeqCst);
        self.value_creates.fetch_add(1, Ordering::SeqCst);

        // Actualizar contador de valores en la clave
        if let Some(key) = &mut self.keys[key_id as usize] {
            key.value_count += 1;
        }

        Ok(value_id)
    }

    /// Eliminar valor del registro
    pub fn delete_value(&mut self, value_id: u32) -> MemoryResult<()> {
        if value_id >= 2048 {
            return Err(MemoryError::InvalidAddress);
        }

        if let Some(value) = &self.values[value_id as usize] {
            let key_id = value.key_id;

            // Verificar que la clave está abierta
            if let Some(key) = &self.keys[key_id as usize] {
                if key.state != RegistryKeyState::Open && key.state != RegistryKeyState::Locked {
                    return Err(MemoryError::PermissionDenied);
                }
            }

            self.values[value_id as usize] = None;
            self.value_count.fetch_sub(1, Ordering::SeqCst);
            self.value_deletes.fetch_add(1, Ordering::SeqCst);

            // Actualizar contador de valores en la clave
            if let Some(key) = &mut self.keys[key_id as usize] {
                key.value_count -= 1;
            }

            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Obtener información de valor
    pub fn get_value_info(&self, value_id: u32) -> Option<&RegistryValueInfo> {
        if value_id >= 2048 {
            return None;
        }
        self.values[value_id as usize].as_ref()
    }

    /// Leer valor del registro
    pub fn read_value(&mut self, value_id: u32) -> MemoryResult<&[u8]> {
        if let Some(value) = &self.values[value_id as usize] {
            // Verificar que la clave está abierta
            if let Some(key) = &self.keys[value.key_id as usize] {
                if key.state != RegistryKeyState::Open && key.state != RegistryKeyState::Locked {
                    return Err(MemoryError::PermissionDenied);
                }
            }

            self.value_reads.fetch_add(1, Ordering::SeqCst);
            Ok(&value.data[..value.data_size as usize])
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Escribir valor del registro
    pub fn write_value(&mut self, value_id: u32, data: &[u8], current_time: u64) -> MemoryResult<()> {
        if let Some(value) = &mut self.values[value_id as usize] {
            // Verificar que la clave está abierta
            if let Some(key) = &self.keys[value.key_id as usize] {
                if key.state != RegistryKeyState::Open && key.state != RegistryKeyState::Locked {
                    return Err(MemoryError::PermissionDenied);
                }
            }

            let data_len = data.len().min(1024);
            value.data[..data_len].copy_from_slice(&data[..data_len]);
            value.data_size = data_len as u32;
            value.last_modified = current_time;

            self.value_writes.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(MemoryError::InvalidAddress)
        }
    }

    /// Buscar valores por clave
    pub fn find_values_by_key(&self, key_id: u32) -> u32 {
        let mut count = 0;
        for value in &self.values {
            if let Some(v) = value {
                if v.key_id == key_id {
                    count += 1;
                }
            }
        }
        count
    }

    /// Buscar valores por tipo
    pub fn find_values_by_type(&self, value_type: RegistryValueType) -> u32 {
        let mut count = 0;
        for value in &self.values {
            if let Some(v) = value {
                if v.value_type == value_type {
                    count += 1;
                }
            }
        }
        count
    }

    /// Hacer backup del registro
    pub fn backup_registry(&mut self) -> MemoryResult<()> {
        self.registry_backups.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// Restaurar registro
    pub fn restore_registry(&mut self) -> MemoryResult<()> {
        self.registry_restores.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// Compactar registro
    pub fn compact_registry(&mut self) -> MemoryResult<()> {
        self.registry_compacts.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// Reparar registro
    pub fn repair_registry(&mut self) -> MemoryResult<()> {
        self.registry_repairs.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// Obtener estadísticas del registro
    pub fn get_registry_stats(&self) -> RegistryStats {
        RegistryStats {
            key_count: self.key_count.load(Ordering::SeqCst),
            value_count: self.value_count.load(Ordering::SeqCst),
            open_keys: self.open_keys.load(Ordering::SeqCst),
            locked_keys: self.locked_keys.load(Ordering::SeqCst),
            error_keys: self.error_keys.load(Ordering::SeqCst),
            total_keys: self.total_keys.load(Ordering::SeqCst),
            total_values: self.total_values.load(Ordering::SeqCst),
            key_creates: self.key_creates.load(Ordering::SeqCst),
            key_deletes: self.key_deletes.load(Ordering::SeqCst),
            key_opens: self.key_opens.load(Ordering::SeqCst),
            key_closes: self.key_closes.load(Ordering::SeqCst),
            value_creates: self.value_creates.load(Ordering::SeqCst),
            value_deletes: self.value_deletes.load(Ordering::SeqCst),
            value_reads: self.value_reads.load(Ordering::SeqCst),
            value_writes: self.value_writes.load(Ordering::SeqCst),
            registry_backups: self.registry_backups.load(Ordering::SeqCst),
            registry_restores: self.registry_restores.load(Ordering::SeqCst),
            registry_compacts: self.registry_compacts.load(Ordering::SeqCst),
            registry_repairs: self.registry_repairs.load(Ordering::SeqCst),
        }
    }
}

/// Estadísticas del registro
#[derive(Debug, Clone, Copy)]
pub struct RegistryStats {
    pub key_count: u64,
    pub value_count: u64,
    pub open_keys: u64,
    pub locked_keys: u64,
    pub error_keys: u64,
    pub total_keys: u64,
    pub total_values: u64,
    pub key_creates: u64,
    pub key_deletes: u64,
    pub key_opens: u64,
    pub key_closes: u64,
    pub value_creates: u64,
    pub value_deletes: u64,
    pub value_reads: u64,
    pub value_writes: u64,
    pub registry_backups: u64,
    pub registry_restores: u64,
    pub registry_compacts: u64,
    pub registry_repairs: u64,
}

/// Inicializar el registry manager
pub fn init() -> MemoryResult<()> {
    // En una implementación completa, esto inicializaría:
    // - Registry manager
    // - Registry storage
    // - Registry security
    // - Registry backup
    // - Registry restore
    // - Registry compaction
    // - Registry repair
    
    Ok(())
}


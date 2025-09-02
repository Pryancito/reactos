//! Operaciones Avanzadas de Archivos
//!
//! Sistema completo de operaciones de archivos: copiar, mover, eliminar, comprimir, etc.

use alloc::{vec::Vec, string::String, format};

/// Tipo de operación de archivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOperationType {
    Copy,       // Copiar archivo/directorio
    Move,       // Mover archivo/directorio
    Delete,     // Eliminar archivo/directorio
    Rename,     // Renombrar archivo/directorio
    Compress,   // Comprimir archivo/directorio
    Decompress, // Descomprimir archivo/directorio
    Backup,     // Crear backup
    Restore,    // Restaurar desde backup
    Sync,       // Sincronizar directorios
    Search,     // Buscar archivos
    Compare,    // Comparar archivos
    Merge,      // Fusionar archivos
}

/// Estado de la operación
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationStatus {
    Pending,    // Pendiente
    Running,    // En ejecución
    Completed,  // Completada
    Failed,     // Falló
    Cancelled,  // Cancelada
    Paused,     // Pausada
}

/// Resultado de la operación
#[derive(Debug, Clone)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
    pub bytes_processed: u64,
    pub files_processed: usize,
    pub duration_ms: u64,
    pub error_details: Option<String>,
}

impl OperationResult {
    pub fn success(message: String, bytes_processed: u64, files_processed: usize, duration_ms: u64) -> Self {
        Self {
            success: true,
            message,
            bytes_processed,
            files_processed,
            duration_ms,
            error_details: None,
        }
    }

    pub fn failure(message: String, error_details: String) -> Self {
        Self {
            success: false,
            message,
            bytes_processed: 0,
            files_processed: 0,
            duration_ms: 0,
            error_details: Some(error_details),
        }
    }
}

/// Información de progreso
#[derive(Debug, Clone)]
pub struct ProgressInfo {
    pub current_file: String,
    pub current_size: u64,
    pub total_size: u64,
    pub percentage: f32,
    pub speed_bytes_per_sec: u64,
    pub estimated_remaining_ms: u64,
}

/// Operación de archivo
#[derive(Debug, Clone)]
pub struct FileOperation {
    pub id: usize,
    pub operation_type: FileOperationType,
    pub source_path: String,
    pub destination_path: String,
    pub status: OperationStatus,
    pub progress: ProgressInfo,
    pub result: Option<OperationResult>,
    pub created_time: u64,
    pub started_time: Option<u64>,
    pub completed_time: Option<u64>,
    pub options: OperationOptions,
}

/// Opciones de operación
#[derive(Debug, Clone)]
pub struct OperationOptions {
    pub overwrite_existing: bool,
    pub preserve_permissions: bool,
    pub preserve_timestamps: bool,
    pub recursive: bool,
    pub show_progress: bool,
    pub verify_integrity: bool,
    pub compression_level: u8,
    pub buffer_size: usize,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

impl Default for OperationOptions {
    fn default() -> Self {
        Self {
            overwrite_existing: false,
            preserve_permissions: true,
            preserve_timestamps: true,
            recursive: true,
            show_progress: true,
            verify_integrity: false,
            compression_level: 6,
            buffer_size: 8192,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}

impl FileOperation {
    pub fn new(
        id: usize,
        operation_type: FileOperationType,
        source_path: String,
        destination_path: String,
    ) -> Self {
        Self {
            id,
            operation_type,
            source_path,
            destination_path,
            status: OperationStatus::Pending,
            progress: ProgressInfo {
                current_file: String::new(),
                current_size: 0,
                total_size: 0,
                percentage: 0.0,
                speed_bytes_per_sec: 0,
                estimated_remaining_ms: 0,
            },
            result: None,
            created_time: get_system_time(),
            started_time: None,
            completed_time: None,
            options: OperationOptions::default(),
        }
    }

    pub fn with_options(mut self, options: OperationOptions) -> Self {
        self.options = options;
        self
    }

    pub fn get_info(&self) -> String {
        format!(
            "ID: {} | {} | {} -> {} | Estado: {} | Progreso: {:.1}%",
            self.id,
            self.operation_type_name(),
            self.source_path,
            self.destination_path,
            self.status_name(),
            self.progress.percentage
        )
    }

    pub fn operation_type_name(&self) -> &'static str {
        match self.operation_type {
            FileOperationType::Copy => "Copiar",
            FileOperationType::Move => "Mover",
            FileOperationType::Delete => "Eliminar",
            FileOperationType::Rename => "Renombrar",
            FileOperationType::Compress => "Comprimir",
            FileOperationType::Decompress => "Descomprimir",
            FileOperationType::Backup => "Backup",
            FileOperationType::Restore => "Restaurar",
            FileOperationType::Sync => "Sincronizar",
            FileOperationType::Search => "Buscar",
            FileOperationType::Compare => "Comparar",
            FileOperationType::Merge => "Fusionar",
        }
    }

    pub fn status_name(&self) -> &'static str {
        match self.status {
            OperationStatus::Pending => "Pendiente",
            OperationStatus::Running => "Ejecutando",
            OperationStatus::Completed => "Completada",
            OperationStatus::Failed => "Falló",
            OperationStatus::Cancelled => "Cancelada",
            OperationStatus::Paused => "Pausada",
        }
    }
}

/// Cola de operaciones
#[derive(Debug, Clone)]
pub struct OperationQueue {
    pub operations: Vec<FileOperation>,
    pub max_concurrent: usize,
    pub running_operations: Vec<usize>,
    pub completed_operations: Vec<FileOperation>,
    pub failed_operations: Vec<FileOperation>,
}

impl OperationQueue {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            operations: Vec::new(),
            max_concurrent,
            running_operations: Vec::new(),
            completed_operations: Vec::new(),
            failed_operations: Vec::new(),
        }
    }

    pub fn add_operation(&mut self, operation: FileOperation) -> usize {
        let id = operation.id;
        self.operations.push(operation);
        id
    }

    pub fn start_next_operation(&mut self) -> Option<usize> {
        if self.running_operations.len() >= self.max_concurrent {
            return None;
        }

        if let Some(operation) = self.operations.iter_mut().find(|op| op.status == OperationStatus::Pending) {
            operation.status = OperationStatus::Running;
            operation.started_time = Some(get_system_time());
            self.running_operations.push(operation.id);
            Some(operation.id)
        } else {
            None
        }
    }

    pub fn complete_operation(&mut self, operation_id: usize, result: OperationResult) {
        if let Some(pos) = self.running_operations.iter().position(|&id| id == operation_id) {
            self.running_operations.remove(pos);
        }

        if let Some(pos) = self.operations.iter().position(|op| op.id == operation_id) {
            let mut operation = self.operations.remove(pos);
            operation.status = if result.success {
                OperationStatus::Completed
            } else {
                OperationStatus::Failed
            };
            operation.completed_time = Some(get_system_time());
            operation.result = Some(result.clone());

            if result.success {
                self.completed_operations.push(operation);
            } else {
                self.failed_operations.push(operation);
            }
        }
    }

    pub fn cancel_operation(&mut self, operation_id: usize) -> bool {
        if let Some(operation) = self.operations.iter_mut().find(|op| op.id == operation_id) {
            operation.status = OperationStatus::Cancelled;
            true
        } else {
            false
        }
    }

    pub fn get_operation(&self, operation_id: usize) -> Option<&FileOperation> {
        self.operations.iter().find(|op| op.id == operation_id)
            .or_else(|| self.completed_operations.iter().find(|op| op.id == operation_id))
            .or_else(|| self.failed_operations.iter().find(|op| op.id == operation_id))
    }

    pub fn get_running_operations(&self) -> Vec<&FileOperation> {
        self.operations.iter().filter(|op| op.status == OperationStatus::Running).collect()
    }

    pub fn get_pending_operations(&self) -> Vec<&FileOperation> {
        self.operations.iter().filter(|op| op.status == OperationStatus::Pending).collect()
    }
}

/// Gestor de operaciones de archivos
#[derive(Debug, Clone)]
pub struct FileOperationsManager {
    pub queue: OperationQueue,
    pub next_operation_id: usize,
    pub statistics: OperationStatistics,
    pub clipboard: Vec<String>, // Archivos en portapapeles
    pub clipboard_operation: Option<FileOperationType>,
}

#[derive(Debug, Clone)]
pub struct OperationStatistics {
    pub total_operations: usize,
    pub completed_operations: usize,
    pub failed_operations: usize,
    pub cancelled_operations: usize,
    pub total_bytes_processed: u64,
    pub total_files_processed: usize,
    pub average_speed_bytes_per_sec: u64,
    pub total_duration_ms: u64,
}

impl Default for OperationStatistics {
    fn default() -> Self {
        Self {
            total_operations: 0,
            completed_operations: 0,
            failed_operations: 0,
            cancelled_operations: 0,
            total_bytes_processed: 0,
            total_files_processed: 0,
            average_speed_bytes_per_sec: 0,
            total_duration_ms: 0,
        }
    }
}

impl FileOperationsManager {
    pub fn new() -> Self {
        Self {
            queue: OperationQueue::new(3), // Máximo 3 operaciones concurrentes
            next_operation_id: 1,
            statistics: OperationStatistics::default(),
            clipboard: Vec::new(),
            clipboard_operation: None,
        }
    }

    pub fn copy_file(&mut self, source: String, destination: String, options: Option<OperationOptions>) -> usize {
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Copy,
            source,
            destination,
        ).with_options(options.unwrap_or_default());

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn move_file(&mut self, source: String, destination: String, options: Option<OperationOptions>) -> usize {
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Move,
            source,
            destination,
        ).with_options(options.unwrap_or_default());

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn delete_file(&mut self, path: String, options: Option<OperationOptions>) -> usize {
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Delete,
            path,
            String::new(), // No hay destino para eliminar
        ).with_options(options.unwrap_or_default());

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn rename_file(&mut self, source: String, new_name: String, options: Option<OperationOptions>) -> usize {
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Rename,
            source,
            new_name,
        ).with_options(options.unwrap_or_default());

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn compress_file(&mut self, source: String, destination: String, options: Option<OperationOptions>) -> usize {
        let mut opts = options.unwrap_or_default();
        opts.compression_level = 6; // Nivel por defecto
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Compress,
            source,
            destination,
        ).with_options(opts);

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn decompress_file(&mut self, source: String, destination: String, options: Option<OperationOptions>) -> usize {
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Decompress,
            source,
            destination,
        ).with_options(options.unwrap_or_default());

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn backup_file(&mut self, source: String, destination: String, options: Option<OperationOptions>) -> usize {
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Backup,
            source,
            destination,
        ).with_options(options.unwrap_or_default());

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn search_files(&mut self, directory: String, pattern: String, options: Option<OperationOptions>) -> usize {
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Search,
            directory,
            pattern,
        ).with_options(options.unwrap_or_default());

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn compare_files(&mut self, file1: String, file2: String, options: Option<OperationOptions>) -> usize {
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Compare,
            file1,
            file2,
        ).with_options(options.unwrap_or_default());

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn sync_directories(&mut self, source: String, destination: String, options: Option<OperationOptions>) -> usize {
        let operation = FileOperation::new(
            self.next_operation_id,
            FileOperationType::Sync,
            source,
            destination,
        ).with_options(options.unwrap_or_default());

        self.next_operation_id += 1;
        self.statistics.total_operations += 1;
        self.queue.add_operation(operation)
    }

    pub fn copy_to_clipboard(&mut self, files: Vec<String>) {
        self.clipboard = files;
        self.clipboard_operation = Some(FileOperationType::Copy);
    }

    pub fn cut_to_clipboard(&mut self, files: Vec<String>) {
        self.clipboard = files;
        self.clipboard_operation = Some(FileOperationType::Move);
    }

    pub fn paste_from_clipboard(&mut self, destination: String) -> Vec<usize> {
        let mut operation_ids = Vec::new();
        
        if let Some(operation_type) = self.clipboard_operation {
            let clipboard_files = self.clipboard.clone(); // Clone to avoid borrow issues
            for file in clipboard_files {
                let operation_id = match operation_type {
                    FileOperationType::Copy => {
                        self.copy_file(file.clone(), format!("{}/{}", destination, get_filename(&file)), None)
                    },
                    FileOperationType::Move => {
                        self.move_file(file.clone(), format!("{}/{}", destination, get_filename(&file)), None)
                    },
                    _ => continue,
                };
                operation_ids.push(operation_id);
            }
        }
        
        operation_ids
    }

    pub fn clear_clipboard(&mut self) {
        self.clipboard.clear();
        self.clipboard_operation = None;
    }

    pub fn get_clipboard_info(&self) -> String {
        if self.clipboard.is_empty() {
            String::from("Portapapeles vacío")
        } else {
            let operation_name = match self.clipboard_operation {
                Some(FileOperationType::Copy) => "Copiar",
                Some(FileOperationType::Move) => "Mover",
                _ => "Desconocido",
            };
            format!("{} archivos para {}", self.clipboard.len(), operation_name)
        }
    }

    pub fn process_operations(&mut self) {
        // Procesar operaciones pendientes
        while let Some(operation_id) = self.queue.start_next_operation() {
            self.execute_operation(operation_id);
        }
    }

    fn execute_operation(&mut self, operation_id: usize) {
        if let Some(operation) = self.queue.get_operation(operation_id) {
            let result = match operation.operation_type {
                FileOperationType::Copy => self.execute_copy_operation(operation),
                FileOperationType::Move => self.execute_move_operation(operation),
                FileOperationType::Delete => self.execute_delete_operation(operation),
                FileOperationType::Rename => self.execute_rename_operation(operation),
                FileOperationType::Compress => self.execute_compress_operation(operation),
                FileOperationType::Decompress => self.execute_decompress_operation(operation),
                FileOperationType::Backup => self.execute_backup_operation(operation),
                FileOperationType::Search => self.execute_search_operation(operation),
                FileOperationType::Compare => self.execute_compare_operation(operation),
                FileOperationType::Sync => self.execute_sync_operation(operation),
                FileOperationType::Restore => self.execute_restore_operation(operation),
                FileOperationType::Merge => self.execute_merge_operation(operation),
            };

            self.queue.complete_operation(operation_id, result);
            self.update_statistics();
        }
    }

    fn execute_copy_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de copia
        let start_time = get_system_time();
        
        // Simular procesamiento
        let bytes_processed = 1024 * 1024; // 1MB simulado
        let files_processed = 1;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Archivo copiado: {} -> {}", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_move_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de movimiento
        let start_time = get_system_time();
        
        let bytes_processed = 1024 * 1024; // 1MB simulado
        let files_processed = 1;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Archivo movido: {} -> {}", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_delete_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de eliminación
        let start_time = get_system_time();
        
        let bytes_processed = 0; // No se procesan bytes al eliminar
        let files_processed = 1;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Archivo eliminado: {}", operation.source_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_rename_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de renombrado
        let start_time = get_system_time();
        
        let bytes_processed = 0; // Solo cambio de nombre
        let files_processed = 1;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Archivo renombrado: {} -> {}", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_compress_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de compresión
        let start_time = get_system_time();
        
        let bytes_processed = 512 * 1024; // 512KB comprimido
        let files_processed = 1;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Archivo comprimido: {} -> {}", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_decompress_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de descompresión
        let start_time = get_system_time();
        
        let bytes_processed = 1024 * 1024; // 1MB descomprimido
        let files_processed = 1;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Archivo descomprimido: {} -> {}", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_backup_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de backup
        let start_time = get_system_time();
        
        let bytes_processed = 1024 * 1024; // 1MB backup
        let files_processed = 1;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Backup creado: {} -> {}", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_search_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de búsqueda
        let start_time = get_system_time();
        
        let bytes_processed = 0; // No se procesan bytes en búsqueda
        let files_processed = 5; // 5 archivos encontrados
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Búsqueda completada en: {} (patrón: {})", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_compare_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de comparación
        let start_time = get_system_time();
        
        let bytes_processed = 1024 * 1024; // 1MB comparado
        let files_processed = 2;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Archivos comparados: {} vs {}", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_sync_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de sincronización
        let start_time = get_system_time();
        
        let bytes_processed = 2048 * 1024; // 2MB sincronizado
        let files_processed = 10;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Directorios sincronizados: {} <-> {}", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_restore_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de restauración
        let start_time = get_system_time();
        
        let bytes_processed = 1024 * 1024; // 1MB restaurado
        let files_processed = 1;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Archivo restaurado: {} -> {}", operation.source_path, operation.destination_path),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn execute_merge_operation(&self, operation: &FileOperation) -> OperationResult {
        // Simular operación de fusión
        let start_time = get_system_time();
        
        let bytes_processed = 1536 * 1024; // 1.5MB fusionado
        let files_processed = 2;
        let duration_ms = get_system_time() - start_time;
        
        OperationResult::success(
            format!("Archivos fusionados: {} + {} -> {}", operation.source_path, operation.destination_path, "merged_file.txt"),
            bytes_processed,
            files_processed,
            duration_ms,
        )
    }

    fn update_statistics(&mut self) {
        self.statistics.completed_operations = self.queue.completed_operations.len();
        self.statistics.failed_operations = self.queue.failed_operations.len();
        
        // Calcular estadísticas de operaciones completadas
        let mut total_bytes = 0u64;
        let mut total_files = 0usize;
        let mut total_duration = 0u64;
        
        for operation in &self.queue.completed_operations {
            if let Some(ref result) = operation.result {
                total_bytes += result.bytes_processed;
                total_files += result.files_processed;
                total_duration += result.duration_ms;
            }
        }
        
        self.statistics.total_bytes_processed = total_bytes;
        self.statistics.total_files_processed = total_files;
        self.statistics.total_duration_ms = total_duration;
        
        if total_duration > 0 {
            self.statistics.average_speed_bytes_per_sec = (total_bytes * 1000) / total_duration;
        }
    }

    pub fn get_info(&self) -> String {
        format!(
            "Gestor de Operaciones - Pendientes: {} | Ejecutando: {} | Completadas: {} | Fallidas: {}",
            self.queue.get_pending_operations().len(),
            self.queue.get_running_operations().len(),
            self.queue.completed_operations.len(),
            self.queue.failed_operations.len()
        )
    }

    pub fn get_statistics(&self) -> String {
        format!(
            "Total: {} | Completadas: {} | Fallidas: {} | Canceladas: {} | Bytes: {} | Archivos: {} | Velocidad: {}/s",
            self.statistics.total_operations,
            self.statistics.completed_operations,
            self.statistics.failed_operations,
            self.statistics.cancelled_operations,
            format_bytes(self.statistics.total_bytes_processed),
            self.statistics.total_files_processed,
            format_bytes(self.statistics.average_speed_bytes_per_sec)
        )
    }
}

// Funciones auxiliares
fn get_system_time() -> u64 {
    // Simulación simple de tiempo del sistema
    1234567890
}

fn get_filename(path: &str) -> &str {
    if let Some(pos) = path.rfind('/') {
        &path[pos + 1..]
    } else {
        path
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

// Gestor global de operaciones de archivos
use spin::Mutex;

pub static FILE_OPERATIONS_MANAGER: Mutex<Option<FileOperationsManager>> = Mutex::new(None);

/// Inicializar el gestor de operaciones de archivos
pub fn init_file_operations_manager() {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    *manager = Some(FileOperationsManager::new());
    crate::logging::info("file_operations", "Gestor de operaciones de archivos inicializado");
}

/// Obtener información del gestor de operaciones
pub fn get_file_operations_info() -> String {
    if let Some(ref manager) = *FILE_OPERATIONS_MANAGER.lock() {
        manager.get_info()
    } else {
        String::from("Gestor de operaciones no inicializado")
    }
}

/// Obtener estadísticas del gestor de operaciones
pub fn get_file_operations_stats() -> String {
    if let Some(ref manager) = *FILE_OPERATIONS_MANAGER.lock() {
        manager.get_statistics()
    } else {
        String::from("Gestor de operaciones no inicializado")
    }
}

/// Copiar archivo
pub fn copy_file(source: String, destination: String) -> usize {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref mut fm) = *manager {
        fm.copy_file(source, destination, None)
    } else {
        0
    }
}

/// Mover archivo
pub fn move_file(source: String, destination: String) -> usize {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref mut fm) = *manager {
        fm.move_file(source, destination, None)
    } else {
        0
    }
}

/// Eliminar archivo
pub fn delete_file(path: String) -> usize {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref mut fm) = *manager {
        fm.delete_file(path, None)
    } else {
        0
    }
}

/// Renombrar archivo
pub fn rename_file(source: String, new_name: String) -> usize {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref mut fm) = *manager {
        fm.rename_file(source, new_name, None)
    } else {
        0
    }
}

/// Copiar al portapapeles
pub fn copy_to_clipboard(files: Vec<String>) {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref mut fm) = *manager {
        fm.copy_to_clipboard(files);
    }
}

/// Cortar al portapapeles
pub fn cut_to_clipboard(files: Vec<String>) {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref mut fm) = *manager {
        fm.cut_to_clipboard(files);
    }
}

/// Pegar desde portapapeles
pub fn paste_from_clipboard(destination: String) -> Vec<usize> {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref mut fm) = *manager {
        fm.paste_from_clipboard(destination)
    } else {
        Vec::new()
    }
}

/// Obtener información del portapapeles
pub fn get_clipboard_info() -> String {
    let manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref fm) = *manager {
        fm.get_clipboard_info()
    } else {
        String::from("Gestor de operaciones no inicializado")
    }
}

/// Procesar operaciones pendientes
pub fn process_operations() {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref mut fm) = *manager {
        fm.process_operations();
    }
}

/// Limpiar portapapeles
pub fn clear_clipboard() {
    let mut manager = FILE_OPERATIONS_MANAGER.lock();
    if let Some(ref mut fm) = *manager {
        fm.clear_clipboard();
    }
}

/// Verificar si el gestor de operaciones está disponible
pub fn is_file_operations_available() -> bool {
    let manager = FILE_OPERATIONS_MANAGER.lock();
    manager.is_some()
}

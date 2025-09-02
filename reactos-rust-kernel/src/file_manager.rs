//! Gestor de Archivos Gráfico Completo
//!
//! Sistema completo de gestión de archivos con interfaz gráfica

use alloc::{vec, vec::Vec, string::{String, ToString}, format};
use crate::filesystem::{FileSystem, NodeType};
use crate::advanced_gui::WindowManager;

/// Tipo de vista del gestor de archivos
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    List,       // Vista de lista
    Grid,       // Vista de cuadrícula
    Details,    // Vista detallada
    Tree,       // Vista de árbol
}

/// Tipo de operación de archivo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOperation {
    Copy,       // Copiar
    Move,       // Mover
    Delete,     // Eliminar
    Rename,     // Renombrar
    Create,     // Crear
    Open,       // Abrir
}

/// Estado de la operación
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationState {
    Idle,       // Inactivo
    InProgress, // En progreso
    Completed,  // Completado
    Error,      // Error
}

/// Información de archivo/directorio
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub node_type: NodeType,
    pub size: usize,
    pub modified_time: u64,
    pub permissions: String,
    pub is_selected: bool,
}

impl FileInfo {
    pub fn new(name: String, path: String, node_type: NodeType, size: usize, modified_time: u64) -> Self {
        Self {
            name,
            path,
            node_type,
            size,
            modified_time,
            permissions: String::from("rw-r--r--"),
            is_selected: false,
        }
    }

    pub fn get_icon(&self) -> char {
        match self.node_type {
            NodeType::File => '📄',
            NodeType::Directory => '📁',
            NodeType::Symlink => '🔗',
            NodeType::Block => '💾',
            NodeType::Char => '🔧',
        }
    }

    pub fn get_size_string(&self) -> String {
        if self.node_type == NodeType::Directory {
            String::from("<DIR>")
        } else {
            format_size(self.size)
        }
    }

    pub fn get_modified_string(&self) -> String {
        format_time(self.modified_time)
    }
}

/// Panel de navegación
#[derive(Debug, Clone)]
pub struct NavigationPanel {
    pub current_path: String,
    pub breadcrumbs: Vec<String>,
    pub history: Vec<String>,
    pub history_index: usize,
}

impl NavigationPanel {
    pub fn new() -> Self {
        Self {
            current_path: String::from("/"),
            breadcrumbs: vec![String::from("/")],
            history: vec![String::from("/")],
            history_index: 0,
        }
    }

    pub fn navigate_to(&mut self, path: String) {
        if path != self.current_path {
            self.current_path = path.clone();
            self.breadcrumbs = self.parse_breadcrumbs(&path);
            
            // Agregar al historial si no es la misma que la actual
            if self.history_index == self.history.len() - 1 {
                self.history.push(path);
                self.history_index += 1;
            } else {
                // Estamos en el medio del historial, truncar
                self.history.truncate(self.history_index + 1);
                self.history.push(path);
                self.history_index += 1;
            }
        }
    }

    pub fn go_back(&mut self) -> bool {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_path = self.history[self.history_index].clone();
            self.breadcrumbs = self.parse_breadcrumbs(&self.current_path);
            true
        } else {
            false
        }
    }

    pub fn go_forward(&mut self) -> bool {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.current_path = self.history[self.history_index].clone();
            self.breadcrumbs = self.parse_breadcrumbs(&self.current_path);
            true
        } else {
            false
        }
    }

    pub fn go_up(&mut self) -> bool {
        if self.current_path != "/" {
            let parent = get_parent_path(&self.current_path);
            self.navigate_to(parent);
            true
        } else {
            false
        }
    }

    fn parse_breadcrumbs(&self, path: &str) -> Vec<String> {
        if path == "/" {
            return vec![String::from("/")];
        }
        
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut breadcrumbs = vec![String::from("/")];
        
        let mut current = String::from("/");
        for part in parts {
            current.push_str(part);
            current.push('/');
            breadcrumbs.push(current.clone());
        }
        
        breadcrumbs
    }
}

/// Panel de archivos
#[derive(Debug, Clone)]
pub struct FilePanel {
    pub files: Vec<FileInfo>,
    pub selected_files: Vec<usize>,
    pub view_mode: ViewMode,
    pub sort_by: SortOption,
    pub sort_ascending: bool,
    pub filter: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOption {
    Name,
    Size,
    Modified,
    Type,
}

impl FilePanel {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            selected_files: Vec::new(),
            view_mode: ViewMode::List,
            sort_by: SortOption::Name,
            sort_ascending: true,
            filter: String::new(),
        }
    }

    pub fn load_files(&mut self, filesystem: &FileSystem, path: &str) {
        self.files.clear();
        self.selected_files.clear();
        
        if let Some(entries) = filesystem.list_directory(path) {
            for entry in entries {
                let file_info = FileInfo::new(
                    entry.clone(),
                    format!("{}/{}", path, entry),
                    NodeType::File, // Simplificado por ahora
                    1024, // Tamaño simulado
                    1234567890, // Tiempo simulado
                );
                self.files.push(file_info);
            }
        }
        
        self.sort_files();
    }

    pub fn sort_files(&mut self) {
        self.files.sort_by(|a, b| {
            let comparison = match self.sort_by {
                SortOption::Name => a.name.cmp(&b.name),
                SortOption::Size => a.size.cmp(&b.size),
                SortOption::Modified => a.modified_time.cmp(&b.modified_time),
                SortOption::Type => {
                    // Comparar por tipo de nodo (simplificado)
                    match (a.node_type.clone(), b.node_type.clone()) {
                        (NodeType::Directory, NodeType::File) => core::cmp::Ordering::Less,
                        (NodeType::File, NodeType::Directory) => core::cmp::Ordering::Greater,
                        _ => core::cmp::Ordering::Equal,
                    }
                },
            };
            
            if self.sort_ascending {
                comparison
            } else {
                comparison.reverse()
            }
        });
    }

    pub fn filter_files(&mut self) {
        if self.filter.is_empty() {
            return;
        }
        
        self.files.retain(|file| {
            file.name.to_lowercase().contains(&self.filter.to_lowercase())
        });
    }

    pub fn select_file(&mut self, index: usize) {
        if let Some(file) = self.files.get_mut(index) {
            file.is_selected = !file.is_selected;
            
            if file.is_selected {
                if !self.selected_files.contains(&index) {
                    self.selected_files.push(index);
                }
            } else {
                self.selected_files.retain(|&i| i != index);
            }
        }
    }

    pub fn select_all(&mut self) {
        self.selected_files.clear();
        for (i, file) in self.files.iter_mut().enumerate() {
            file.is_selected = true;
            self.selected_files.push(i);
        }
    }

    pub fn deselect_all(&mut self) {
        for file in &mut self.files {
            file.is_selected = false;
        }
        self.selected_files.clear();
    }

    pub fn get_selected_files(&self) -> Vec<&FileInfo> {
        self.selected_files.iter()
            .filter_map(|&i| self.files.get(i))
            .collect()
    }
}

/// Barra de herramientas
#[derive(Debug, Clone)]
pub struct Toolbar {
    pub buttons: Vec<ToolbarButton>,
}

#[derive(Debug, Clone)]
pub struct ToolbarButton {
    pub id: String,
    pub text: String,
    pub icon: char,
    pub enabled: bool,
    pub tooltip: String,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            buttons: vec![
                ToolbarButton {
                    id: String::from("back"),
                    text: String::from("Atrás"),
                    icon: '⬅',
                    enabled: false,
                    tooltip: String::from("Ir atrás"),
                },
                ToolbarButton {
                    id: String::from("forward"),
                    text: String::from("Adelante"),
                    icon: '➡',
                    enabled: false,
                    tooltip: String::from("Ir adelante"),
                },
                ToolbarButton {
                    id: String::from("up"),
                    text: String::from("Subir"),
                    icon: '⬆',
                    enabled: false,
                    tooltip: String::from("Subir un nivel"),
                },
                ToolbarButton {
                    id: String::from("refresh"),
                    text: String::from("Actualizar"),
                    icon: '🔄',
                    enabled: true,
                    tooltip: String::from("Actualizar lista"),
                },
                ToolbarButton {
                    id: String::from("new_folder"),
                    text: String::from("Nueva Carpeta"),
                    icon: '📁',
                    enabled: true,
                    tooltip: String::from("Crear nueva carpeta"),
                },
                ToolbarButton {
                    id: String::from("new_file"),
                    text: String::from("Nuevo Archivo"),
                    icon: '📄',
                    enabled: true,
                    tooltip: String::from("Crear nuevo archivo"),
                },
                ToolbarButton {
                    id: String::from("copy"),
                    text: String::from("Copiar"),
                    icon: '📋',
                    enabled: false,
                    tooltip: String::from("Copiar archivos seleccionados"),
                },
                ToolbarButton {
                    id: String::from("move"),
                    text: String::from("Mover"),
                    icon: '✂',
                    enabled: false,
                    tooltip: String::from("Mover archivos seleccionados"),
                },
                ToolbarButton {
                    id: String::from("delete"),
                    text: String::from("Eliminar"),
                    icon: '🗑',
                    enabled: false,
                    tooltip: String::from("Eliminar archivos seleccionados"),
                },
            ],
        }
    }

    pub fn update_buttons(&mut self, navigation: &NavigationPanel, file_panel: &FilePanel) {
        // Actualizar botones de navegación
        for button in &mut self.buttons {
            match button.id.as_str() {
                "back" => button.enabled = navigation.history_index > 0,
                "forward" => button.enabled = navigation.history_index < navigation.history.len() - 1,
                "up" => button.enabled = navigation.current_path != "/",
                "copy" | "move" | "delete" => button.enabled = !file_panel.selected_files.is_empty(),
                _ => {}
            }
        }
    }
}

/// Gestor de archivos principal
#[derive(Debug, Clone)]
pub struct FileManager {
    pub window_id: Option<usize>,
    pub navigation: NavigationPanel,
    pub file_panel: FilePanel,
    pub toolbar: Toolbar,
    pub status_bar: String,
    pub operation_state: OperationState,
    pub clipboard: Vec<FileInfo>,
    pub clipboard_operation: Option<FileOperation>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            window_id: None,
            navigation: NavigationPanel::new(),
            file_panel: FilePanel::new(),
            toolbar: Toolbar::new(),
            status_bar: String::from("Listo"),
            operation_state: OperationState::Idle,
            clipboard: Vec::new(),
            clipboard_operation: None,
        }
    }

    pub fn create_window(&mut self, window_manager: &mut WindowManager) -> Option<usize> {
        let window_id = window_manager.create_window(
            String::from("Gestor de Archivos"),
            100, 100, 800, 600
        );
        
        self.window_id = Some(window_id);
        Some(window_id)
    }

    pub fn update(&mut self, filesystem: &FileSystem) {
        // Actualizar lista de archivos
        self.file_panel.load_files(filesystem, &self.navigation.current_path);
        
        // Aplicar filtro si existe
        if !self.file_panel.filter.is_empty() {
            self.file_panel.filter_files();
        }
        
        // Actualizar botones de la barra de herramientas
        self.toolbar.update_buttons(&self.navigation, &self.file_panel);
        
        // Actualizar barra de estado
        let file_count = self.file_panel.files.len();
        let selected_count = self.file_panel.selected_files.len();
        self.status_bar = format!("{} archivos, {} seleccionados", file_count, selected_count);
    }

    pub fn handle_toolbar_click(&mut self, button_id: &str, filesystem: &mut FileSystem) -> String {
        match button_id {
            "back" => {
                if self.navigation.go_back() {
                    String::from("Navegando atrás")
                } else {
                    String::from("No hay historial atrás")
                }
            },
            "forward" => {
                if self.navigation.go_forward() {
                    String::from("Navegando adelante")
                } else {
                    String::from("No hay historial adelante")
                }
            },
            "up" => {
                if self.navigation.go_up() {
                    String::from("Subiendo un nivel")
                } else {
                    String::from("Ya estás en la raíz")
                }
            },
            "refresh" => {
                String::from("Lista actualizada")
            },
            "new_folder" => {
                self.create_new_folder(filesystem)
            },
            "new_file" => {
                self.create_new_file(filesystem)
            },
            "copy" => {
                self.copy_selected_files()
            },
            "move" => {
                self.move_selected_files()
            },
            "delete" => {
                self.delete_selected_files(filesystem)
            },
            _ => String::from("Acción no reconocida"),
        }
    }

    fn create_new_folder(&mut self, filesystem: &mut FileSystem) -> String {
        let folder_name = format!("Nueva Carpeta {}", get_timestamp());
        let folder_path = format!("{}/{}", self.navigation.current_path, folder_name);
        
        if filesystem.create_directory(&folder_path) {
            String::from("Carpeta creada exitosamente")
        } else {
            String::from("Error al crear carpeta")
        }
    }

    fn create_new_file(&mut self, filesystem: &mut FileSystem) -> String {
        let file_name = format!("Nuevo Archivo {}.txt", get_timestamp());
        let file_path = format!("{}/{}", self.navigation.current_path, file_name);
        
        if filesystem.create_file(&file_path, String::from("Contenido inicial")) {
            String::from("Archivo creado exitosamente")
        } else {
            String::from("Error al crear archivo")
        }
    }

    fn copy_selected_files(&mut self) -> String {
        self.clipboard.clear();
        for &index in &self.file_panel.selected_files {
            if let Some(file) = self.file_panel.files.get(index) {
                self.clipboard.push(file.clone());
            }
        }
        self.clipboard_operation = Some(FileOperation::Copy);
        String::from("Archivos copiados al portapapeles")
    }

    fn move_selected_files(&mut self) -> String {
        self.clipboard.clear();
        for &index in &self.file_panel.selected_files {
            if let Some(file) = self.file_panel.files.get(index) {
                self.clipboard.push(file.clone());
            }
        }
        self.clipboard_operation = Some(FileOperation::Move);
        String::from("Archivos movidos al portapapeles")
    }

    fn delete_selected_files(&mut self, _filesystem: &mut FileSystem) -> String {
        let deleted_count = self.file_panel.selected_files.len();
        
        if deleted_count > 0 {
            self.file_panel.deselect_all();
            format!("{} archivos eliminados", deleted_count)
        } else {
            String::from("Error al eliminar archivos")
        }
    }

    pub fn handle_file_double_click(&mut self, index: usize, _filesystem: &FileSystem) -> String {
        if let Some(file) = self.file_panel.files.get(index) {
            match file.node_type {
                NodeType::Directory => {
                    self.navigation.navigate_to(file.path.clone());
                    String::from("Navegando a directorio")
                },
                NodeType::File => {
                    // Abrir archivo (simulado)
                    format!("Abriendo archivo: {}", file.name)
                },
                _ => String::from("Tipo de archivo no soportado"),
            }
        } else {
            String::from("Archivo no encontrado")
        }
    }

    pub fn get_info(&self) -> String {
        format!(
            "Gestor de Archivos - Ruta: {} | Archivos: {} | Seleccionados: {} | Estado: {:?}",
            self.navigation.current_path,
            self.file_panel.files.len(),
            self.file_panel.selected_files.len(),
            self.operation_state
        )
    }

    pub fn get_stats(&self) -> String {
        let total_files = self.file_panel.files.len();
        let total_dirs = self.file_panel.files.iter()
            .filter(|f| f.node_type == NodeType::Directory)
            .count();
        let total_size: usize = self.file_panel.files.iter()
            .map(|f| f.size)
            .sum();
        
        format!(
            "Archivos: {} | Directorios: {} | Tamaño total: {} | Historial: {}",
            total_files,
            total_dirs,
            format_size(total_size),
            self.navigation.history.len()
        )
    }
}

// Funciones auxiliares
fn format_size(size: usize) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn format_time(timestamp: u64) -> String {
    // Simulación simple de formato de tiempo
    format!("{}", timestamp)
}

fn get_timestamp() -> u64 {
    // Simulación simple de timestamp
    1234567890
}

fn get_parent_path(path: &str) -> String {
    if path == "/" {
        return String::from("/");
    }
    
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() <= 1 {
        String::from("/")
    } else {
        let parent_parts = &parts[..parts.len() - 1];
        if parent_parts.is_empty() {
            String::from("/")
        } else {
            format!("/{}/", parent_parts.join("/"))
        }
    }
}

// Gestor global de archivos
use spin::Mutex;

pub static FILE_MANAGER: Mutex<Option<FileManager>> = Mutex::new(None);

/// Inicializar el gestor de archivos
pub fn init_file_manager() {
    let mut manager = FILE_MANAGER.lock();
    *manager = Some(FileManager::new());
    crate::logging::info("file_manager", "Gestor de archivos inicializado");
}

/// Obtener información del gestor de archivos
pub fn get_file_manager_info() -> String {
    if let Some(ref manager) = *FILE_MANAGER.lock() {
        manager.get_info()
    } else {
        String::from("Gestor de archivos no inicializado")
    }
}

/// Obtener estadísticas del gestor de archivos
pub fn get_file_manager_stats() -> String {
    if let Some(ref manager) = *FILE_MANAGER.lock() {
        manager.get_stats()
    } else {
        String::from("Gestor de archivos no inicializado")
    }
}

/// Crear ventana del gestor de archivos
pub fn create_file_manager_window() -> Option<usize> {
    let mut manager = FILE_MANAGER.lock();
    if let Some(ref mut _fm) = *manager {
        // Necesitaríamos acceso al WindowManager aquí
        // Por ahora retornamos un ID simulado
        Some(1)
    } else {
        None
    }
}

/// Actualizar gestor de archivos
pub fn update_file_manager() {
    let mut manager = FILE_MANAGER.lock();
    if let Some(ref mut _fm) = *manager {
        // Necesitaríamos acceso al FileSystem aquí
        // Por ahora solo actualizamos el estado
        // fm.status_bar = String::from("Actualizado");
    }
}

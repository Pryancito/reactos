//! Aplicaciones de usuario básicas para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Editor de texto básico
//! - Calculadora
//! - Visor de archivos
//! - Monitor de sistema
//! - Juego simple

use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use alloc::format;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Tipo de aplicación
#[derive(Debug, Clone, PartialEq)]
pub enum AppType {
    Editor,
    Calculator,
    FileViewer,
    SystemMonitor,
    Game,
    Terminal,
}

/// Estado de aplicación
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Running,
    Paused,
    Stopped,
    Error,
}

/// Aplicación de usuario
#[derive(Debug, Clone)]
pub struct UserApp {
    pub id: usize,
    pub name: String,
    pub app_type: AppType,
    pub state: AppState,
    pub pid: usize,
    pub memory_usage: usize,
    pub cpu_usage: f32,
    pub created_time: u64,
    pub last_activity: u64,
    pub data: String, // Datos específicos de la aplicación
}

impl UserApp {
    /// Crear una nueva aplicación
    pub fn new(id: usize, name: String, app_type: AppType) -> Self {
        Self {
            id,
            name,
            app_type,
            state: AppState::Running,
            pid: id + 1000, // PID simulado
            memory_usage: 0,
            cpu_usage: 0.0,
            created_time: 0,
            last_activity: 0,
            data: String::new(),
        }
    }
    
    /// Obtener información de la aplicación
    pub fn get_info(&self) -> String {
        format!(
            "ID: {} | Nombre: {} | Tipo: {:?} | Estado: {:?} | PID: {} | Memoria: {}KB | CPU: {:.1}%",
            self.id,
            self.name,
            self.app_type,
            self.state,
            self.pid,
            self.memory_usage,
            self.cpu_usage
        )
    }
}

/// Editor de texto básico
pub struct TextEditor {
    pub content: String,
    pub cursor_position: usize,
    pub filename: String,
    pub modified: bool,
    pub line_count: usize,
    pub char_count: usize,
}

impl TextEditor {
    /// Crear un nuevo editor
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            filename: "untitled.txt".to_string(),
            modified: false,
            line_count: 0,
            char_count: 0,
        }
    }
    
    /// Insertar texto en la posición del cursor
    pub fn insert_text(&mut self, text: &str) {
        self.content.insert_str(self.cursor_position, text);
        self.cursor_position += text.len();
        self.modified = true;
        self.update_counts();
    }
    
    /// Eliminar carácter en la posición del cursor
    pub fn delete_char(&mut self) -> bool {
        if self.cursor_position > 0 && !self.content.is_empty() {
            self.cursor_position -= 1;
            self.content.remove(self.cursor_position);
            self.modified = true;
            self.update_counts();
            true
        } else {
            false
        }
    }
    
    /// Mover cursor
    pub fn move_cursor(&mut self, direction: CursorDirection) {
        match direction {
            CursorDirection::Left => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            },
            CursorDirection::Right => {
                if self.cursor_position < self.content.len() {
                    self.cursor_position += 1;
                }
            },
            CursorDirection::Up => {
                // Simulación simple de movimiento vertical
                if self.cursor_position > 20 {
                    self.cursor_position -= 20;
                } else {
                    self.cursor_position = 0;
                }
            },
            CursorDirection::Down => {
                if self.cursor_position + 20 < self.content.len() {
                    self.cursor_position += 20;
                } else {
                    self.cursor_position = self.content.len();
                }
            },
        }
    }
    
    /// Actualizar contadores
    fn update_counts(&mut self) {
        self.char_count = self.content.len();
        self.line_count = self.content.matches('\n').count() + 1;
    }
    
    /// Guardar archivo
    pub fn save(&mut self) -> bool {
        if crate::filesystem::write_file(&self.filename, self.content.clone()) {
            self.modified = false;
            crate::logging::info("editor", &format!("Archivo guardado: {}", self.filename));
            true
        } else {
            crate::logging::error("editor", &format!("Error al guardar archivo: {}", self.filename));
            false
        }
    }
    
    /// Cargar archivo
    pub fn load(&mut self, filename: &str) -> bool {
        if let Some(content) = crate::filesystem::read_file(filename) {
            self.content = content;
            self.filename = filename.to_string();
            self.cursor_position = 0;
            self.modified = false;
            self.update_counts();
            crate::logging::info("editor", &format!("Archivo cargado: {}", filename));
            true
        } else {
            crate::logging::error("editor", &format!("Error al cargar archivo: {}", filename));
            false
        }
    }
    
    /// Obtener información del editor
    pub fn get_info(&self) -> String {
        format!(
            "Editor: {} | Líneas: {} | Caracteres: {} | Cursor: {} | Modificado: {}",
            self.filename,
            self.line_count,
            self.char_count,
            self.cursor_position,
            if self.modified { "Sí" } else { "No" }
        )
    }
}

/// Dirección del cursor
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Calculadora básica
pub struct Calculator {
    pub display: String,
    pub memory: f64,
    pub operation: Option<Operation>,
    pub operand1: Option<f64>,
    pub operand2: Option<f64>,
    pub result: Option<f64>,
}

/// Operaciones de la calculadora
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Sqrt,
    Percent,
}

impl Calculator {
    /// Crear una nueva calculadora
    pub fn new() -> Self {
        Self {
            display: "0".to_string(),
            memory: 0.0,
            operation: None,
            operand1: None,
            operand2: None,
            result: None,
        }
    }
    
    /// Ingresar número
    pub fn input_number(&mut self, digit: u8) {
        if self.display == "0" {
            self.display = digit.to_string();
        } else {
            self.display.push_str(&digit.to_string());
        }
    }
    
    /// Ingresar punto decimal
    pub fn input_decimal(&mut self) {
        if !self.display.contains('.') {
            self.display.push('.');
        }
    }
    
    /// Establecer operación
    pub fn set_operation(&mut self, op: Operation) {
        if let Ok(num) = self.display.parse::<f64>() {
            self.operand1 = Some(num);
            self.operation = Some(op);
            self.display = "0".to_string();
        }
    }
    
    /// Calcular resultado
    pub fn calculate(&mut self) -> bool {
        if let (Some(op1), Some(op), Ok(op2)) = (self.operand1, self.operation, self.display.parse::<f64>()) {
            let result = match op {
                Operation::Add => op1 + op2,
                Operation::Subtract => op1 - op2,
                Operation::Multiply => op1 * op2,
                Operation::Divide => {
                    if op2 != 0.0 {
                        op1 / op2
                    } else {
                        return false; // División por cero
                    }
                },
                Operation::Power => {
                    // Simulación simple de potencia
                    let mut result = 1.0;
                    for _ in 0..op2 as i32 {
                        result *= op1;
                    }
                    result
                },
                Operation::Sqrt => {
                    // Simulación simple de raíz cuadrada
                    if op1 >= 0.0 {
                        op1 * 0.5 // Aproximación simple
                    } else {
                        0.0
                    }
                },
                Operation::Percent => op1 * (op2 / 100.0),
            };
            
            self.result = Some(result);
            self.display = result.to_string();
            self.operand1 = None;
            self.operation = None;
            self.operand2 = None;
            true
        } else {
            false
        }
    }
    
    /// Limpiar calculadora
    pub fn clear(&mut self) {
        self.display = "0".to_string();
        self.operation = None;
        self.operand1 = None;
        self.operand2 = None;
        self.result = None;
    }
    
    /// Obtener información de la calculadora
    pub fn get_info(&self) -> String {
        format!(
            "Calculadora: {} | Memoria: {} | Operación: {:?} | Operando1: {:?}",
            self.display,
            self.memory,
            self.operation,
            self.operand1
        )
    }
}

/// Visor de archivos
pub struct FileViewer {
    pub current_path: String,
    pub files: Vec<String>,
    pub selected_index: usize,
    pub view_mode: ViewMode,
    pub filter: String,
}

/// Modo de visualización
#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    List,
    Details,
    Tree,
}

impl FileViewer {
    /// Crear un nuevo visor de archivos
    pub fn new() -> Self {
        Self {
            current_path: "/".to_string(),
            files: Vec::new(),
            selected_index: 0,
            view_mode: ViewMode::List,
            filter: String::new(),
        }
    }
    
    /// Refrescar lista de archivos
    pub fn refresh(&mut self) {
        if let Some(file_list) = crate::filesystem::list_directory(&self.current_path) {
            self.files = file_list;
            if self.selected_index >= self.files.len() {
                self.selected_index = 0;
            }
        }
    }
    
    /// Navegar a directorio
    pub fn navigate_to(&mut self, path: &str) -> bool {
        if crate::filesystem::is_filesystem_available() {
            self.current_path = path.to_string();
            self.refresh();
            true
        } else {
            false
        }
    }
    
    /// Seleccionar archivo
    pub fn select_file(&mut self, index: usize) -> bool {
        if index < self.files.len() {
            self.selected_index = index;
            true
        } else {
            false
        }
    }
    
    /// Obtener información del visor
    pub fn get_info(&self) -> String {
        format!(
            "Visor: {} | Archivos: {} | Seleccionado: {} | Modo: {:?} | Filtro: '{}'",
            self.current_path,
            self.files.len(),
            self.selected_index,
            self.view_mode,
            self.filter
        )
    }
}

/// Monitor de sistema
pub struct SystemMonitor {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub network_usage: f32,
    pub processes: Vec<String>,
    pub uptime: u64,
    pub last_update: u64,
}

impl SystemMonitor {
    /// Crear un nuevo monitor de sistema
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_usage: 0.0,
            processes: Vec::new(),
            uptime: 0,
            last_update: 0,
        }
    }
    
    /// Actualizar estadísticas
    pub fn update(&mut self) {
        // Simular actualización de estadísticas
        self.cpu_usage = 25.5;
        self.memory_usage = 45.2;
        self.disk_usage = 60.8;
        self.network_usage = 12.3;
        self.uptime += 1;
        self.last_update = self.uptime;
        
        // Obtener lista de procesos
        self.processes = vec![
            "kernel".to_string(),
            "shell".to_string(),
            "gui".to_string(),
            "editor".to_string(),
            "calculator".to_string(),
        ];
    }
    
    /// Obtener información del monitor
    pub fn get_info(&self) -> String {
        format!(
            "Monitor: CPU: {:.1}% | Memoria: {:.1}% | Disco: {:.1}% | Red: {:.1}% | Procesos: {} | Uptime: {}s",
            self.cpu_usage,
            self.memory_usage,
            self.disk_usage,
            self.network_usage,
            self.processes.len(),
            self.uptime
        )
    }
}

/// Juego simple (Snake)
pub struct SnakeGame {
    pub score: u32,
    pub level: u32,
    pub snake: Vec<(i32, i32)>,
    pub food: (i32, i32),
    pub direction: Direction,
    pub game_over: bool,
    pub paused: bool,
}

/// Dirección del snake
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeGame {
    /// Crear un nuevo juego
    pub fn new() -> Self {
        Self {
            score: 0,
            level: 1,
            snake: vec![(10, 10), (9, 10), (8, 10)],
            food: (15, 15),
            direction: Direction::Right,
            game_over: false,
            paused: false,
        }
    }
    
    /// Cambiar dirección
    pub fn change_direction(&mut self, new_direction: Direction) {
        // Evitar que el snake se mueva en dirección opuesta
        match (self.direction, new_direction) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) |
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => return,
            _ => self.direction = new_direction,
        }
    }
    
    /// Actualizar juego
    pub fn update(&mut self) {
        if self.game_over || self.paused {
            return;
        }
        
        // Mover snake
        let head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };
        
        // Verificar colisiones
        if new_head.0 < 0 || new_head.0 >= 20 || new_head.1 < 0 || new_head.1 >= 20 {
            self.game_over = true;
            return;
        }
        
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }
        
        self.snake.insert(0, new_head);
        
        // Verificar si comió comida
        if new_head == self.food {
            self.score += 10;
            if self.score % 100 == 0 {
                self.level += 1;
            }
            // Generar nueva comida
            self.food = (5, 5); // Posición fija por simplicidad
        } else {
            self.snake.pop();
        }
    }
    
    /// Obtener información del juego
    pub fn get_info(&self) -> String {
        format!(
            "Snake: Puntuación: {} | Nivel: {} | Longitud: {} | Estado: {} | Dirección: {:?}",
            self.score,
            self.level,
            self.snake.len(),
            if self.game_over { "Game Over" } else if self.paused { "Pausado" } else { "Jugando" },
            self.direction
        )
    }
}

/// Gestor de aplicaciones
pub struct AppManager {
    pub apps: Vec<UserApp>,
    pub next_app_id: AtomicUsize,
    pub text_editor: Option<TextEditor>,
    pub calculator: Option<Calculator>,
    pub file_viewer: Option<FileViewer>,
    pub system_monitor: Option<SystemMonitor>,
    pub snake_game: Option<SnakeGame>,
    pub apps_launched: AtomicUsize,
    pub apps_terminated: AtomicUsize,
}

impl AppManager {
    /// Crear un nuevo gestor de aplicaciones
    pub fn new() -> Self {
        Self {
            apps: Vec::new(),
            next_app_id: AtomicUsize::new(1),
            text_editor: None,
            calculator: None,
            file_viewer: None,
            system_monitor: None,
            snake_game: None,
            apps_launched: AtomicUsize::new(0),
            apps_terminated: AtomicUsize::new(0),
        }
    }
    
    /// Inicializar el gestor de aplicaciones
    pub fn initialize(&mut self) -> bool {
        // Crear aplicaciones del sistema
        self.create_system_apps();
        crate::logging::info("apps", "Gestor de aplicaciones inicializado correctamente");
        true
    }
    
    /// Crear aplicaciones del sistema
    fn create_system_apps(&mut self) {
        // Editor de texto
        self.text_editor = Some(TextEditor::new());
        
        // Calculadora
        self.calculator = Some(Calculator::new());
        
        // Visor de archivos
        self.file_viewer = Some(FileViewer::new());
        
        // Monitor de sistema
        self.system_monitor = Some(SystemMonitor::new());
        
        // Juego Snake
        self.snake_game = Some(SnakeGame::new());
    }
    
    /// Lanzar aplicación
    pub fn launch_app(&mut self, app_type: AppType, name: String) -> Option<usize> {
        let id = self.next_app_id.fetch_add(1, Ordering::SeqCst);
        let mut app = UserApp::new(id, name.clone(), app_type.clone());
        app.created_time = 1; // Simulado
        app.last_activity = 1;
        
        // Configurar aplicación específica
        match app_type {
            AppType::Editor => {
                app.memory_usage = 1024; // 1MB
                app.cpu_usage = 5.0;
            },
            AppType::Calculator => {
                app.memory_usage = 512; // 512KB
                app.cpu_usage = 1.0;
            },
            AppType::FileViewer => {
                app.memory_usage = 768; // 768KB
                app.cpu_usage = 3.0;
            },
            AppType::SystemMonitor => {
                app.memory_usage = 256; // 256KB
                app.cpu_usage = 2.0;
            },
            AppType::Game => {
                app.memory_usage = 2048; // 2MB
                app.cpu_usage = 15.0;
            },
            AppType::Terminal => {
                app.memory_usage = 512; // 512KB
                app.cpu_usage = 2.0;
            },
        }
        
        self.apps.push(app);
        self.apps_launched.fetch_add(1, Ordering::SeqCst);
        
        crate::logging::info("apps", &format!("Aplicación lanzada: {} (ID: {})", name, id));
        Some(id)
    }
    
    /// Terminar aplicación
    pub fn terminate_app(&mut self, app_id: usize) -> bool {
        if let Some(pos) = self.apps.iter().position(|app| app.id == app_id) {
            let app = self.apps.remove(pos);
            self.apps_terminated.fetch_add(1, Ordering::SeqCst);
            crate::logging::info("apps", &format!("Aplicación terminada: {} (ID: {})", app.name, app_id));
            true
        } else {
            false
        }
    }
    
    /// Obtener información del gestor
    pub fn get_info(&self) -> String {
        format!(
            "Apps: {} ejecutándose | Lanzadas: {} | Terminadas: {} | Editor: {} | Calculadora: {} | Visor: {} | Monitor: {} | Juego: {}",
            self.apps.len(),
            self.apps_launched.load(Ordering::SeqCst),
            self.apps_terminated.load(Ordering::SeqCst),
            if self.text_editor.is_some() { "Disponible" } else { "No disponible" },
            if self.calculator.is_some() { "Disponible" } else { "No disponible" },
            if self.file_viewer.is_some() { "Disponible" } else { "No disponible" },
            if self.system_monitor.is_some() { "Disponible" } else { "No disponible" },
            if self.snake_game.is_some() { "Disponible" } else { "No disponible" }
        )
    }
    
    /// Obtener estadísticas del gestor
    pub fn get_stats(&self) -> String {
        let total_memory: usize = self.apps.iter().map(|app| app.memory_usage).sum();
        let total_cpu: f32 = self.apps.iter().map(|app| app.cpu_usage).sum();
        
        format!(
            "Apps: {} ejecutándose | Memoria total: {}KB | CPU total: {:.1}% | Lanzadas: {} | Terminadas: {}",
            self.apps.len(),
            total_memory,
            total_cpu,
            self.apps_launched.load(Ordering::SeqCst),
            self.apps_terminated.load(Ordering::SeqCst)
        )
    }
}

/// Instancia global del gestor de aplicaciones
static APP_MANAGER: Mutex<Option<AppManager>> = Mutex::new(None);

/// Inicializar el gestor de aplicaciones
pub fn init_apps() -> bool {
    let mut manager_guard = APP_MANAGER.lock();
    if manager_guard.is_none() {
        let mut manager = AppManager::new();
        if manager.initialize() {
            *manager_guard = Some(manager);
            return true;
        }
    }
    false
}

/// Lanzar aplicación
pub fn launch_app(app_type: AppType, name: String) -> Option<usize> {
    let mut manager_guard = APP_MANAGER.lock();
    if let Some(ref mut manager) = *manager_guard {
        manager.launch_app(app_type, name)
    } else {
        None
    }
}

/// Terminar aplicación
pub fn terminate_app(app_id: usize) -> bool {
    let mut manager_guard = APP_MANAGER.lock();
    if let Some(ref mut manager) = *manager_guard {
        manager.terminate_app(app_id)
    } else {
        false
    }
}

/// Obtener información del gestor de aplicaciones
pub fn get_apps_info() -> String {
    let manager_guard = APP_MANAGER.lock();
    if let Some(ref manager) = *manager_guard {
        manager.get_info()
    } else {
        String::from("Gestor de aplicaciones: No disponible")
    }
}

/// Obtener estadísticas del gestor de aplicaciones
pub fn get_apps_stats() -> String {
    let manager_guard = APP_MANAGER.lock();
    if let Some(ref manager) = *manager_guard {
        manager.get_stats()
    } else {
        String::from("Estadísticas de aplicaciones: No disponible")
    }
}

/// Verificar si el gestor de aplicaciones está disponible
pub fn is_apps_available() -> bool {
    let manager_guard = APP_MANAGER.lock();
    manager_guard.is_some()
}

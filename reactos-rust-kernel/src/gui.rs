//! Sistema gráfico básico para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Sistema de ventanas básico
//! - Elementos de interfaz de usuario
//! - Gestión de eventos gráficos
//! - Renderizado de elementos

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

/// Color RGB
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Crear un nuevo color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    
    /// Colores predefinidos
    pub fn black() -> Self { Self::new(0, 0, 0) }
    pub fn white() -> Self { Self::new(255, 255, 255) }
    pub fn red() -> Self { Self::new(255, 0, 0) }
    pub fn green() -> Self { Self::new(0, 255, 0) }
    pub fn blue() -> Self { Self::new(0, 0, 255) }
    pub fn yellow() -> Self { Self::new(255, 255, 0) }
    pub fn cyan() -> Self { Self::new(0, 255, 255) }
    pub fn magenta() -> Self { Self::new(255, 0, 255) }
    pub fn gray() -> Self { Self::new(128, 128, 128) }
    pub fn dark_gray() -> Self { Self::new(64, 64, 64) }
    pub fn light_gray() -> Self { Self::new(192, 192, 192) }
}

/// Punto 2D
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Rectángulo
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x && point.x < (self.x + self.width as i32) &&
        point.y >= self.y && point.y < (self.y + self.height as i32)
    }
    
    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.x < (other.x + other.width as i32) &&
        (self.x + self.width as i32) > other.x &&
        self.y < (other.y + other.height as i32) &&
        (self.y + self.height as i32) > other.y
    }
}

/// Tipo de elemento GUI
#[derive(Debug, Clone, PartialEq)]
pub enum GuiElementType {
    Window,
    Button,
    Label,
    TextBox,
    Panel,
    Menu,
    MenuItem,
}

/// Estado de elemento GUI
#[derive(Debug, Clone, PartialEq)]
pub enum GuiElementState {
    Normal,
    Hover,
    Pressed,
    Disabled,
    Focused,
}

/// Elemento de interfaz gráfica
#[derive(Debug, Clone)]
pub struct GuiElement {
    pub id: usize,
    pub element_type: GuiElementType,
    pub bounds: Rectangle,
    pub text: String,
    pub background_color: Color,
    pub text_color: Color,
    pub border_color: Color,
    pub state: GuiElementState,
    pub visible: bool,
    pub enabled: bool,
    pub children: Vec<usize>, // IDs de elementos hijos
    pub parent: Option<usize>, // ID del elemento padre
}

impl GuiElement {
    /// Crear un nuevo elemento GUI
    pub fn new(id: usize, element_type: GuiElementType, bounds: Rectangle) -> Self {
        Self {
            id,
            element_type,
            bounds,
            text: String::new(),
            background_color: Color::light_gray(),
            text_color: Color::black(),
            border_color: Color::dark_gray(),
            state: GuiElementState::Normal,
            visible: true,
            enabled: true,
            children: Vec::new(),
            parent: None,
        }
    }
    
    /// Crear una ventana
    pub fn new_window(id: usize, title: String, bounds: Rectangle) -> Self {
        let mut window = Self::new(id, GuiElementType::Window, bounds);
        window.text = title;
        window.background_color = Color::light_gray();
        window.border_color = Color::dark_gray();
        window
    }
    
    /// Crear un botón
    pub fn new_button(id: usize, text: String, bounds: Rectangle) -> Self {
        let mut button = Self::new(id, GuiElementType::Button, bounds);
        button.text = text;
        button.background_color = Color::gray();
        button.border_color = Color::dark_gray();
        button
    }
    
    /// Crear una etiqueta
    pub fn new_label(id: usize, text: String, bounds: Rectangle) -> Self {
        let mut label = Self::new(id, GuiElementType::Label, bounds);
        label.text = text;
        label.background_color = Color::white();
        label.border_color = Color::white();
        label
    }
    
    /// Crear un panel
    pub fn new_panel(id: usize, bounds: Rectangle) -> Self {
        let mut panel = Self::new(id, GuiElementType::Panel, bounds);
        panel.background_color = Color::white();
        panel.border_color = Color::gray();
        panel
    }
    
    /// Verificar si el elemento contiene un punto
    pub fn contains_point(&self, point: Point) -> bool {
        self.visible && self.bounds.contains(point)
    }
    
    /// Obtener información del elemento
    pub fn get_info(&self) -> String {
        format!(
            "ID: {} | Tipo: {:?} | Pos: ({}, {}) | Tamaño: {}x{} | Texto: '{}' | Estado: {:?} | Visible: {} | Habilitado: {}",
            self.id,
            self.element_type,
            self.bounds.x,
            self.bounds.y,
            self.bounds.width,
            self.bounds.height,
            self.text,
            self.state,
            self.visible,
            self.enabled
        )
    }
}

/// Evento de interfaz gráfica
#[derive(Debug, Clone, PartialEq)]
pub enum GuiEvent {
    MouseMove { x: i32, y: i32 },
    MouseClick { x: i32, y: i32, button: u8 },
    MousePress { x: i32, y: i32, button: u8 },
    MouseRelease { x: i32, y: i32, button: u8 },
    KeyPress { key: u8 },
    KeyRelease { key: u8 },
    WindowClose { window_id: usize },
    ButtonClick { button_id: usize },
    TextInput { text: String },
}

/// Estado del sistema GUI
#[derive(Debug, Clone, PartialEq)]
pub enum GuiState {
    Initializing,
    Running,
    Paused,
    Error,
}

/// Sistema de interfaz gráfica
pub struct GuiSystem {
    pub state: GuiState,
    pub elements: Vec<GuiElement>,
    pub next_element_id: AtomicUsize,
    pub active_window: Option<usize>,
    pub focused_element: Option<usize>,
    pub mouse_position: Point,
    pub screen_width: u32,
    pub screen_height: u32,
    pub background_color: Color,
    pub is_initialized: AtomicBool,
    pub events_processed: AtomicUsize,
    pub elements_rendered: AtomicUsize,
}

impl GuiSystem {
    /// Crear un nuevo sistema GUI
    pub fn new() -> Self {
        Self {
            state: GuiState::Initializing,
            elements: Vec::new(),
            next_element_id: AtomicUsize::new(1),
            active_window: None,
            focused_element: None,
            mouse_position: Point::new(0, 0),
            screen_width: 800,
            screen_height: 600,
            background_color: Color::dark_gray(),
            is_initialized: AtomicBool::new(false),
            events_processed: AtomicUsize::new(0),
            elements_rendered: AtomicUsize::new(0),
        }
    }
    
    /// Inicializar el sistema GUI
    pub fn initialize(&mut self) -> bool {
        self.state = GuiState::Running;
        self.is_initialized.store(true, Ordering::SeqCst);
        
        // Crear ventana principal
        let main_window = GuiElement::new_window(
            0,
            "ReactOS Rust Kernel - GUI System".to_string(),
            Rectangle::new(50, 50, 700, 500)
        );
        self.elements.push(main_window);
        
        // Crear elementos de la ventana principal
        self.create_main_window_elements();
        
        // Log de inicialización
        crate::logging::info("gui", "Sistema gráfico inicializado correctamente");
        
        true
    }
    
    /// Crear elementos de la ventana principal
    fn create_main_window_elements(&mut self) {
        // Panel de información del sistema
        let info_panel = GuiElement::new_panel(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            Rectangle::new(20, 40, 300, 200)
        );
        self.elements.push(info_panel);
        
        // Etiqueta de título
        let title_label = GuiElement::new_label(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Información del Sistema".to_string(),
            Rectangle::new(30, 50, 280, 20)
        );
        self.elements.push(title_label);
        
        // Botón de información
        let info_button = GuiElement::new_button(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Mostrar Info".to_string(),
            Rectangle::new(30, 80, 100, 30)
        );
        self.elements.push(info_button);
        
        // Botón de procesos
        let process_button = GuiElement::new_button(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Procesos".to_string(),
            Rectangle::new(140, 80, 100, 30)
        );
        self.elements.push(process_button);
        
        // Botón de archivos
        let files_button = GuiElement::new_button(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Archivos".to_string(),
            Rectangle::new(250, 80, 100, 30)
        );
        self.elements.push(files_button);
        
        // Panel de logs
        let logs_panel = GuiElement::new_panel(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            Rectangle::new(20, 260, 300, 200)
        );
        self.elements.push(logs_panel);
        
        // Etiqueta de logs
        let logs_label = GuiElement::new_label(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Logs del Sistema".to_string(),
            Rectangle::new(30, 270, 280, 20)
        );
        self.elements.push(logs_label);
        
        // Panel de red
        let network_panel = GuiElement::new_panel(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            Rectangle::new(340, 40, 300, 200)
        );
        self.elements.push(network_panel);
        
        // Etiqueta de red
        let network_label = GuiElement::new_label(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Estado de Red".to_string(),
            Rectangle::new(350, 50, 280, 20)
        );
        self.elements.push(network_label);
        
        // Botón de ping
        let ping_button = GuiElement::new_button(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Ping Test".to_string(),
            Rectangle::new(350, 80, 100, 30)
        );
        self.elements.push(ping_button);
        
        // Panel de audio
        let audio_panel = GuiElement::new_panel(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            Rectangle::new(340, 260, 300, 200)
        );
        self.elements.push(audio_panel);
        
        // Etiqueta de audio
        let audio_label = GuiElement::new_label(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Control de Audio".to_string(),
            Rectangle::new(350, 270, 280, 20)
        );
        self.elements.push(audio_label);
        
        // Botón de play
        let play_button = GuiElement::new_button(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Play".to_string(),
            Rectangle::new(350, 300, 60, 30)
        );
        self.elements.push(play_button);
        
        // Botón de stop
        let stop_button = GuiElement::new_button(
            self.next_element_id.fetch_add(1, Ordering::SeqCst),
            "Stop".to_string(),
            Rectangle::new(420, 300, 60, 30)
        );
        self.elements.push(stop_button);
    }
    
    /// Agregar un elemento al sistema
    pub fn add_element(&mut self, mut element: GuiElement) -> usize {
        let id = self.next_element_id.fetch_add(1, Ordering::SeqCst);
        element.id = id;
        self.elements.push(element);
        id
    }
    
    /// Obtener un elemento por ID
    pub fn get_element(&self, id: usize) -> Option<&GuiElement> {
        self.elements.iter().find(|e| e.id == id)
    }
    
    /// Obtener un elemento por ID (mutable)
    pub fn get_element_mut(&mut self, id: usize) -> Option<&mut GuiElement> {
        self.elements.iter_mut().find(|e| e.id == id)
    }
    
    /// Procesar evento de mouse
    pub fn handle_mouse_event(&mut self, event: GuiEvent) {
        self.events_processed.fetch_add(1, Ordering::SeqCst);
        
        match event {
            GuiEvent::MouseMove { x, y } => {
                self.mouse_position = Point::new(x, y);
                self.update_hover_states();
            },
            GuiEvent::MouseClick { x, y, button } => {
                if let Some(element_id) = self.find_element_at_point(Point::new(x, y)) {
                    self.handle_element_click(element_id, button);
                }
            },
            _ => {}
        }
    }
    
    /// Actualizar estados de hover
    fn update_hover_states(&mut self) {
        for element in &mut self.elements {
            if element.contains_point(self.mouse_position) {
                if element.state == GuiElementState::Normal {
                    element.state = GuiElementState::Hover;
                }
            } else {
                if element.state == GuiElementState::Hover {
                    element.state = GuiElementState::Normal;
                }
            }
        }
    }
    
    /// Encontrar elemento en un punto
    fn find_element_at_point(&self, point: Point) -> Option<usize> {
        // Buscar desde el final (elementos superiores)
        for element in self.elements.iter().rev() {
            if element.contains_point(point) {
                return Some(element.id);
            }
        }
        None
    }
    
    /// Manejar clic en elemento
    fn handle_element_click(&mut self, element_id: usize, _button: u8) {
        if let Some(element) = self.get_element_mut(element_id) {
            match element.element_type {
                GuiElementType::Button => {
                    element.state = GuiElementState::Pressed;
                    self.handle_button_click(element_id);
                },
                GuiElementType::Window => {
                    self.active_window = Some(element_id);
                },
                _ => {}
            }
        }
    }
    
    /// Manejar clic en botón
    fn handle_button_click(&mut self, button_id: usize) {
        if let Some(button) = self.get_element(button_id) {
            match button.text.as_str() {
                "Mostrar Info" => {
                    crate::logging::info("gui", "Botón 'Mostrar Info' presionado");
                },
                "Procesos" => {
                    crate::logging::info("gui", "Botón 'Procesos' presionado");
                },
                "Archivos" => {
                    crate::logging::info("gui", "Botón 'Archivos' presionado");
                },
                "Ping Test" => {
                    crate::logging::info("gui", "Botón 'Ping Test' presionado");
                },
                "Play" => {
                    crate::logging::info("gui", "Botón 'Play' presionado");
                },
                "Stop" => {
                    crate::logging::info("gui", "Botón 'Stop' presionado");
                },
                _ => {
                    crate::logging::info("gui", &format!("Botón '{}' presionado", button.text));
                }
            }
        }
    }
    
    /// Renderizar todos los elementos
    pub fn render(&mut self) {
        self.elements_rendered.fetch_add(1, Ordering::SeqCst);
        
        // En una implementación real, aquí se renderizarían los elementos
        // Por ahora, solo loggeamos la operación
        crate::logging::info("gui", "Renderizando elementos GUI");
    }
    
    /// Obtener información del sistema GUI
    pub fn get_info(&self) -> String {
        format!(
            "GUI: {} | Estado: {:?} | Elementos: {} | Ventana activa: {} | Foco: {} | Mouse: ({}, {}) | Pantalla: {}x{} | Eventos: {} | Renders: {}",
            if self.is_initialized.load(Ordering::SeqCst) { "Activo" } else { "Inactivo" },
            self.state,
            self.elements.len(),
            if let Some(id) = self.active_window { id.to_string() } else { "Ninguna".to_string() },
            if let Some(id) = self.focused_element { id.to_string() } else { "Ninguno".to_string() },
            self.mouse_position.x,
            self.mouse_position.y,
            self.screen_width,
            self.screen_height,
            self.events_processed.load(Ordering::SeqCst),
            self.elements_rendered.load(Ordering::SeqCst)
        )
    }
    
    /// Obtener estadísticas del sistema GUI
    pub fn get_stats(&self) -> String {
        let visible_elements = self.elements.iter().filter(|e| e.visible).count();
        let enabled_elements = self.elements.iter().filter(|e| e.enabled).count();
        let windows = self.elements.iter().filter(|e| e.element_type == GuiElementType::Window).count();
        let buttons = self.elements.iter().filter(|e| e.element_type == GuiElementType::Button).count();
        
        format!(
            "GUI: {} | Estado: {:?} | Elementos: {}/{} visibles, {}/{} habilitados | Ventanas: {} | Botones: {} | Eventos: {} | Renders: {}",
            if self.is_initialized.load(Ordering::SeqCst) { "Activo" } else { "Inactivo" },
            self.state,
            visible_elements,
            self.elements.len(),
            enabled_elements,
            self.elements.len(),
            windows,
            buttons,
            self.events_processed.load(Ordering::SeqCst),
            self.elements_rendered.load(Ordering::SeqCst)
        )
    }
}

/// Instancia global del sistema GUI
static GUI_SYSTEM: Mutex<Option<GuiSystem>> = Mutex::new(None);

/// Inicializar el sistema GUI
pub fn init_gui() -> bool {
    let mut gui_guard = GUI_SYSTEM.lock();
    if gui_guard.is_none() {
        let mut gui = GuiSystem::new();
        if gui.initialize() {
            *gui_guard = Some(gui);
            return true;
        }
    }
    false
}

/// Procesar evento GUI
pub fn handle_gui_event(event: GuiEvent) {
    let mut gui_guard = GUI_SYSTEM.lock();
    if let Some(ref mut gui) = *gui_guard {
        gui.handle_mouse_event(event);
    }
}

/// Renderizar sistema GUI
pub fn render_gui() {
    let mut gui_guard = GUI_SYSTEM.lock();
    if let Some(ref mut gui) = *gui_guard {
        gui.render();
    }
}

/// Obtener información del sistema GUI
pub fn get_gui_info() -> String {
    let gui_guard = GUI_SYSTEM.lock();
    if let Some(ref gui) = *gui_guard {
        gui.get_info()
    } else {
        String::from("Sistema GUI: No disponible")
    }
}

/// Obtener estadísticas del sistema GUI
pub fn get_gui_stats() -> String {
    let gui_guard = GUI_SYSTEM.lock();
    if let Some(ref gui) = *gui_guard {
        gui.get_stats()
    } else {
        String::from("Estadísticas GUI: No disponible")
    }
}

/// Verificar si el sistema GUI está disponible
pub fn is_gui_available() -> bool {
    let gui_guard = GUI_SYSTEM.lock();
    gui_guard.is_some()
}

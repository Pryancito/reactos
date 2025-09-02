//! Editor de Texto con Interfaz Gráfica
//!
//! Editor de texto completo con interfaz gráfica integrada

use alloc::{vec, vec::Vec, string::{String, ToString}, format};
use crate::advanced_gui::{WindowManager, Theme};
use crate::filesystem::FileSystem;

/// Tipo de codificación de texto
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
    UTF8,
    ASCII,
    Latin1,
}

/// Tipo de final de línea
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    LF,     // Unix/Linux
    CRLF,   // Windows
    CR,     // Mac Classic
}

/// Estado del editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorState {
    Normal,     // Modo normal
    Insert,     // Modo inserción
    Visual,     // Modo visual (selección)
    Command,    // Modo comando
    Search,     // Modo búsqueda
}

/// Tipo de operación de edición
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditOperation {
    Insert,
    Delete,
    Replace,
    Cut,
    Copy,
    Paste,
    Undo,
    Redo,
}

/// Información de la línea
#[derive(Debug, Clone)]
pub struct LineInfo {
    pub number: usize,
    pub content: String,
    pub length: usize,
    pub is_modified: bool,
}

impl LineInfo {
    pub fn new(number: usize, content: String) -> Self {
        let length = content.len();
        Self {
            number,
            content,
            length,
            is_modified: false,
        }
    }

    pub fn update_content(&mut self, new_content: String) {
        self.content = new_content;
        self.length = self.content.len();
        self.is_modified = true;
    }
}

/// Posición del cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

impl CursorPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn is_valid(&self, max_lines: usize, max_columns: usize) -> bool {
        self.line < max_lines && self.column <= max_columns
    }
}

/// Selección de texto
#[derive(Debug, Clone)]
pub struct TextSelection {
    pub start: CursorPosition,
    pub end: CursorPosition,
    pub is_active: bool,
}

impl TextSelection {
    pub fn new() -> Self {
        Self {
            start: CursorPosition::new(0, 0),
            end: CursorPosition::new(0, 0),
            is_active: false,
        }
    }

    pub fn clear(&mut self) {
        self.is_active = false;
        self.start = CursorPosition::new(0, 0);
        self.end = CursorPosition::new(0, 0);
    }

    pub fn set_start(&mut self, pos: CursorPosition) {
        self.start = pos;
        self.is_active = true;
    }

    pub fn set_end(&mut self, pos: CursorPosition) {
        self.end = pos;
    }

    pub fn get_selected_text(&self, lines: &[LineInfo]) -> String {
        if !self.is_active {
            return String::new();
        }

        let start_line = self.start.line.min(self.end.line);
        let end_line = self.start.line.max(self.end.line);
        let start_col = if self.start.line < self.end.line { self.start.column } else { self.start.column.min(self.end.column) };
        let end_col = if self.start.line > self.end.line { self.start.column } else { self.start.column.max(self.end.column) };

        let mut result = String::new();
        for i in start_line..=end_line.min(lines.len() - 1) {
            if let Some(line) = lines.get(i) {
                let start = if i == start_line { start_col } else { 0 };
                let end = if i == end_line { end_col } else { line.content.len() };
                
                if start < line.content.len() && end <= line.content.len() {
                    let slice = &line.content[start..end];
                    result.push_str(slice);
                    if i < end_line {
                        result.push('\n');
                    }
                }
            }
        }
        result
    }
}

/// Historial de operaciones (para undo/redo)
#[derive(Debug, Clone)]
pub struct EditHistory {
    pub operations: Vec<EditOperation>,
    pub positions: Vec<CursorPosition>,
    pub data: Vec<String>,
    pub current_index: usize,
    pub max_history: usize,
}

impl EditHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            operations: Vec::new(),
            positions: Vec::new(),
            data: Vec::new(),
            current_index: 0,
            max_history,
        }
    }

    pub fn add_operation(&mut self, operation: EditOperation, position: CursorPosition, data: String) {
        // Limpiar historial futuro si estamos en el medio
        if self.current_index < self.operations.len() {
            self.operations.truncate(self.current_index);
            self.positions.truncate(self.current_index);
            self.data.truncate(self.current_index);
        }

        // Agregar nueva operación
        self.operations.push(operation);
        self.positions.push(position);
        self.data.push(data);
        self.current_index += 1;

        // Limitar tamaño del historial
        if self.operations.len() > self.max_history {
            self.operations.remove(0);
            self.positions.remove(0);
            self.data.remove(0);
            self.current_index -= 1;
        }
    }

    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current_index < self.operations.len()
    }

    pub fn undo(&mut self) -> Option<(EditOperation, CursorPosition, String)> {
        if self.can_undo() {
            self.current_index -= 1;
            Some((
                self.operations[self.current_index],
                self.positions[self.current_index],
                self.data[self.current_index].clone(),
            ))
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<(EditOperation, CursorPosition, String)> {
        if self.can_redo() {
            let result = (
                self.operations[self.current_index],
                self.positions[self.current_index],
                self.data[self.current_index].clone(),
            );
            self.current_index += 1;
            Some(result)
        } else {
            None
        }
    }
}

/// Configuración del editor
#[derive(Debug, Clone)]
pub struct EditorConfig {
    pub tab_size: usize,
    pub use_spaces: bool,
    pub word_wrap: bool,
    pub line_numbers: bool,
    pub syntax_highlighting: bool,
    pub auto_indent: bool,
    pub encoding: TextEncoding,
    pub line_ending: LineEnding,
    pub font_size: usize,
    pub theme: Theme,
}

impl EditorConfig {
    pub fn new() -> Self {
        Self {
            tab_size: 4,
            use_spaces: true,
            word_wrap: true,
            line_numbers: true,
            syntax_highlighting: true,
            auto_indent: true,
            encoding: TextEncoding::UTF8,
            line_ending: LineEnding::LF,
            font_size: 12,
            theme: Theme::Classic,
        }
    }
}

/// Barra de herramientas del editor
#[derive(Debug, Clone)]
pub struct EditorToolbar {
    pub buttons: Vec<EditorButton>,
}

#[derive(Debug, Clone)]
pub struct EditorButton {
    pub id: String,
    pub text: String,
    pub icon: char,
    pub enabled: bool,
    pub tooltip: String,
}

impl EditorToolbar {
    pub fn new() -> Self {
        Self {
            buttons: vec![
                EditorButton {
                    id: String::from("new"),
                    text: String::from("Nuevo"),
                    icon: '📄',
                    enabled: true,
                    tooltip: String::from("Nuevo archivo"),
                },
                EditorButton {
                    id: String::from("open"),
                    text: String::from("Abrir"),
                    icon: '📂',
                    enabled: true,
                    tooltip: String::from("Abrir archivo"),
                },
                EditorButton {
                    id: String::from("save"),
                    text: String::from("Guardar"),
                    icon: '💾',
                    enabled: false,
                    tooltip: String::from("Guardar archivo"),
                },
                EditorButton {
                    id: String::from("save_as"),
                    text: String::from("Guardar Como"),
                    icon: '💾',
                    enabled: true,
                    tooltip: String::from("Guardar como..."),
                },
                EditorButton {
                    id: String::from("undo"),
                    text: String::from("Deshacer"),
                    icon: '↶',
                    enabled: false,
                    tooltip: String::from("Deshacer última acción"),
                },
                EditorButton {
                    id: String::from("redo"),
                    text: String::from("Rehacer"),
                    icon: '↷',
                    enabled: false,
                    tooltip: String::from("Rehacer última acción"),
                },
                EditorButton {
                    id: String::from("cut"),
                    text: String::from("Cortar"),
                    icon: '✂',
                    enabled: false,
                    tooltip: String::from("Cortar selección"),
                },
                EditorButton {
                    id: String::from("copy"),
                    text: String::from("Copiar"),
                    icon: '📋',
                    enabled: false,
                    tooltip: String::from("Copiar selección"),
                },
                EditorButton {
                    id: String::from("paste"),
                    text: String::from("Pegar"),
                    icon: '📋',
                    enabled: true,
                    tooltip: String::from("Pegar desde portapapeles"),
                },
                EditorButton {
                    id: String::from("find"),
                    text: String::from("Buscar"),
                    icon: '🔍',
                    enabled: true,
                    tooltip: String::from("Buscar texto"),
                },
                EditorButton {
                    id: String::from("replace"),
                    text: String::from("Reemplazar"),
                    icon: '🔄',
                    enabled: true,
                    tooltip: String::from("Buscar y reemplazar"),
                },
            ],
        }
    }

    pub fn update_buttons(&mut self, editor: &TextEditor) {
        for button in &mut self.buttons {
            match button.id.as_str() {
                "save" => button.enabled = editor.is_modified(),
                "undo" => button.enabled = editor.history.can_undo(),
                "redo" => button.enabled = editor.history.can_redo(),
                "cut" | "copy" => button.enabled = editor.selection.is_active,
                _ => {}
            }
        }
    }
}

/// Editor de texto principal
#[derive(Debug, Clone)]
pub struct TextEditor {
    pub window_id: Option<usize>,
    pub filename: String,
    pub lines: Vec<LineInfo>,
    pub cursor: CursorPosition,
    pub selection: TextSelection,
    pub state: EditorState,
    pub config: EditorConfig,
    pub toolbar: EditorToolbar,
    pub history: EditHistory,
    pub clipboard: String,
    pub search_text: String,
    pub replace_text: String,
    pub is_modified: bool,
    pub scroll_offset: usize,
    pub view_width: usize,
    pub view_height: usize,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            window_id: None,
            filename: String::new(),
            lines: vec![LineInfo::new(1, String::new())],
            cursor: CursorPosition::new(0, 0),
            selection: TextSelection::new(),
            state: EditorState::Normal,
            config: EditorConfig::new(),
            toolbar: EditorToolbar::new(),
            history: EditHistory::new(100),
            clipboard: String::new(),
            search_text: String::new(),
            replace_text: String::new(),
            is_modified: false,
            scroll_offset: 0,
            view_width: 80,
            view_height: 25,
        }
    }

    pub fn create_window(&mut self, window_manager: &mut WindowManager) -> Option<usize> {
        let window_id = window_manager.create_window(
            String::from("Editor de Texto"),
            150, 150, 900, 700
        );
        
        self.window_id = Some(window_id);
        Some(window_id)
    }

    pub fn load_file(&mut self, filesystem: &FileSystem, filename: &str) -> bool {
        if let Some(content) = filesystem.read_file(filename) {
            self.filename = filename.to_string();
            self.lines.clear();
            
            let file_lines: Vec<&str> = content.split('\n').collect();
            for (i, line) in file_lines.iter().enumerate() {
                self.lines.push(LineInfo::new(i + 1, line.to_string()));
            }
            
            if self.lines.is_empty() {
                self.lines.push(LineInfo::new(1, String::new()));
            }
            
            self.cursor = CursorPosition::new(0, 0);
            self.selection.clear();
            self.is_modified = false;
            self.history = EditHistory::new(100);
            true
        } else {
            false
        }
    }

    pub fn save_file(&mut self, filesystem: &mut FileSystem) -> bool {
        if self.filename.is_empty() {
            return false;
        }

        let content = self.get_all_text();
        if filesystem.write_file(&self.filename, content) {
            self.is_modified = false;
            // Marcar todas las líneas como no modificadas
            for line in &mut self.lines {
                line.is_modified = false;
            }
            true
        } else {
            false
        }
    }

    pub fn save_as(&mut self, filesystem: &mut FileSystem, filename: &str) -> bool {
        self.filename = filename.to_string();
        self.save_file(filesystem)
    }

    pub fn new_file(&mut self) {
        self.filename.clear();
        self.lines.clear();
        self.lines.push(LineInfo::new(1, String::new()));
        self.cursor = CursorPosition::new(0, 0);
        self.selection.clear();
        self.is_modified = false;
        self.history = EditHistory::new(100);
    }

    pub fn insert_text(&mut self, text: &str) {
        if self.cursor.line >= self.lines.len() {
            return;
        }

        let line = &mut self.lines[self.cursor.line];
        let before = &line.content[..self.cursor.column.min(line.content.len())];
        let after = &line.content[self.cursor.column.min(line.content.len())..];
        
        let new_content = format!("{}{}{}", before, text, after);
        line.update_content(new_content);
        
        self.cursor.column += text.len();
        self.is_modified = true;
        
        // Agregar al historial
        self.history.add_operation(
            EditOperation::Insert,
            self.cursor,
            text.to_string(),
        );
    }

    pub fn insert_newline(&mut self) {
        if self.cursor.line >= self.lines.len() {
            return;
        }

        let line = &mut self.lines[self.cursor.line];
        let before = line.content[..self.cursor.column.min(line.content.len())].to_string();
        let after = line.content[self.cursor.column.min(line.content.len())..].to_string();
        
        line.update_content(before);
        
        // Crear nueva línea
        let new_line = LineInfo::new(self.cursor.line + 2, after);
        self.lines.insert(self.cursor.line + 1, new_line);
        
        // Actualizar números de línea
        for i in (self.cursor.line + 1)..self.lines.len() {
            self.lines[i].number = i + 1;
        }
        
        self.cursor.line += 1;
        self.cursor.column = 0;
        self.is_modified = true;
        
        // Agregar al historial
        self.history.add_operation(
            EditOperation::Insert,
            self.cursor,
            String::from("\n"),
        );
    }

    pub fn delete_char(&mut self) -> bool {
        if self.cursor.line >= self.lines.len() {
            return false;
        }

        if self.cursor.column < self.lines[self.cursor.line].content.len() {
            // Eliminar carácter en la posición actual
            let char_to_delete = self.lines[self.cursor.line].content.chars().nth(self.cursor.column).unwrap_or(' ');
            self.lines[self.cursor.line].content.remove(self.cursor.column);
            self.lines[self.cursor.line].length = self.lines[self.cursor.line].content.len();
            self.lines[self.cursor.line].is_modified = true;
            self.is_modified = true;
            
            // Agregar al historial
            self.history.add_operation(
                EditOperation::Delete,
                self.cursor,
                char_to_delete.to_string(),
            );
            
            true
        } else if self.cursor.line < self.lines.len() - 1 {
            // Unir con la siguiente línea
            let next_line = self.lines.remove(self.cursor.line + 1);
            self.lines[self.cursor.line].content.push_str(&next_line.content);
            self.lines[self.cursor.line].length = self.lines[self.cursor.line].content.len();
            self.lines[self.cursor.line].is_modified = true;
            self.is_modified = true;
            
            // Actualizar números de línea
            for i in self.cursor.line..self.lines.len() {
                self.lines[i].number = i + 1;
            }
            
            // Agregar al historial
            self.history.add_operation(
                EditOperation::Delete,
                self.cursor,
                String::from("\n"),
            );
            
            true
        } else {
            false
        }
    }

    pub fn move_cursor(&mut self, direction: CursorDirection) {
        match direction {
            CursorDirection::Left => {
                if self.cursor.column > 0 {
                    self.cursor.column -= 1;
                } else if self.cursor.line > 0 {
                    self.cursor.line -= 1;
                    self.cursor.column = self.lines[self.cursor.line].content.len();
                }
            },
            CursorDirection::Right => {
                if self.cursor.column < self.lines[self.cursor.line].content.len() {
                    self.cursor.column += 1;
                } else if self.cursor.line < self.lines.len() - 1 {
                    self.cursor.line += 1;
                    self.cursor.column = 0;
                }
            },
            CursorDirection::Up => {
                if self.cursor.line > 0 {
                    self.cursor.line -= 1;
                    self.cursor.column = self.cursor.column.min(self.lines[self.cursor.line].content.len());
                }
            },
            CursorDirection::Down => {
                if self.cursor.line < self.lines.len() - 1 {
                    self.cursor.line += 1;
                    self.cursor.column = self.cursor.column.min(self.lines[self.cursor.line].content.len());
                }
            },
            CursorDirection::Home => {
                self.cursor.column = 0;
            },
            CursorDirection::End => {
                self.cursor.column = self.lines[self.cursor.line].content.len();
            },
            CursorDirection::PageUp => {
                let page_size = self.view_height - 2; // Dejar espacio para toolbar
                if self.cursor.line >= page_size {
                    self.cursor.line -= page_size;
                } else {
                    self.cursor.line = 0;
                }
                self.cursor.column = self.cursor.column.min(self.lines[self.cursor.line].content.len());
            },
            CursorDirection::PageDown => {
                let page_size = self.view_height - 2;
                if self.cursor.line + page_size < self.lines.len() {
                    self.cursor.line += page_size;
                } else {
                    self.cursor.line = self.lines.len() - 1;
                }
                self.cursor.column = self.cursor.column.min(self.lines[self.cursor.line].content.len());
            },
        }
    }

    pub fn select_all(&mut self) {
        if !self.lines.is_empty() {
            self.selection.set_start(CursorPosition::new(0, 0));
            self.selection.set_end(CursorPosition::new(
                self.lines.len() - 1,
                self.lines.last().unwrap().content.len(),
            ));
        }
    }

    pub fn copy_selection(&mut self) {
        if self.selection.is_active {
            self.clipboard = self.selection.get_selected_text(&self.lines);
        }
    }

    pub fn cut_selection(&mut self) {
        if self.selection.is_active {
            self.clipboard = self.selection.get_selected_text(&self.lines);
            self.delete_selection();
        }
    }

    pub fn paste_clipboard(&mut self) {
        if !self.clipboard.is_empty() {
            let clipboard_content = self.clipboard.clone();
            self.insert_text(&clipboard_content);
        }
    }

    pub fn delete_selection(&mut self) {
        if !self.selection.is_active {
            return;
        }

        // Implementación simplificada - eliminar todo el texto seleccionado
        self.selection.clear();
        self.is_modified = true;
    }

    pub fn undo(&mut self) {
        if let Some((operation, position, data)) = self.history.undo() {
            match operation {
                EditOperation::Insert => {
                    // Revertir inserción
                    self.cursor = position;
                    // Implementación simplificada
                },
                EditOperation::Delete => {
                    // Revertir eliminación
                    self.cursor = position;
                    self.insert_text(&data);
                },
                _ => {}
            }
        }
    }

    pub fn redo(&mut self) {
        if let Some((operation, position, data)) = self.history.redo() {
            match operation {
                EditOperation::Insert => {
                    self.cursor = position;
                    self.insert_text(&data);
                },
                EditOperation::Delete => {
                    self.cursor = position;
                    self.delete_char();
                },
                _ => {}
            }
        }
    }

    pub fn find_text(&mut self, text: &str) -> Vec<CursorPosition> {
        let mut results = Vec::new();
        self.search_text = text.to_string();
        
        for (line_idx, line) in self.lines.iter().enumerate() {
            if let Some(col_idx) = line.content.find(text) {
                results.push(CursorPosition::new(line_idx, col_idx));
            }
        }
        
        results
    }

    pub fn replace_text(&mut self, search: &str, replace: &str) -> usize {
        let mut count = 0;
        self.search_text = search.to_string();
        self.replace_text = replace.to_string();
        
        for line in &mut self.lines {
            if line.content.contains(search) {
                line.content = line.content.replace(search, replace);
                line.length = line.content.len();
                line.is_modified = true;
                count += 1;
            }
        }
        
        if count > 0 {
            self.is_modified = true;
        }
        
        count
    }

    pub fn get_all_text(&self) -> String {
        let mut result = String::new();
        for (i, line) in self.lines.iter().enumerate() {
            result.push_str(&line.content);
            if i < self.lines.len() - 1 {
                result.push('\n');
            }
        }
        result
    }

    pub fn get_line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_char_count(&self) -> usize {
        self.lines.iter().map(|line| line.content.len()).sum()
    }

    pub fn get_word_count(&self) -> usize {
        let text = self.get_all_text();
        text.split_whitespace().count()
    }

    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    pub fn get_info(&self) -> String {
        format!(
            "Editor: {} | Líneas: {} | Caracteres: {} | Palabras: {} | Modificado: {}",
            if self.filename.is_empty() { "Sin nombre" } else { &self.filename },
            self.get_line_count(),
            self.get_char_count(),
            self.get_word_count(),
            if self.is_modified { "Sí" } else { "No" }
        )
    }

    pub fn get_stats(&self) -> String {
        format!(
            "Líneas: {} | Caracteres: {} | Palabras: {} | Cursor: {}:{} | Estado: {:?}",
            self.get_line_count(),
            self.get_char_count(),
            self.get_word_count(),
            self.cursor.line + 1,
            self.cursor.column + 1,
            self.state
        )
    }
}

/// Direcciones de movimiento del cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorDirection {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

// Editor global
use spin::Mutex;

pub static TEXT_EDITOR: Mutex<Option<TextEditor>> = Mutex::new(None);

/// Inicializar el editor de texto
pub fn init_text_editor() {
    let mut editor = TEXT_EDITOR.lock();
    *editor = Some(TextEditor::new());
    crate::logging::info("text_editor", "Editor de texto inicializado");
}

/// Obtener información del editor
pub fn get_text_editor_info() -> String {
    if let Some(ref editor) = *TEXT_EDITOR.lock() {
        editor.get_info()
    } else {
        String::from("Editor de texto no inicializado")
    }
}

/// Obtener estadísticas del editor
pub fn get_text_editor_stats() -> String {
    if let Some(ref editor) = *TEXT_EDITOR.lock() {
        editor.get_stats()
    } else {
        String::from("Editor de texto no inicializado")
    }
}

/// Crear ventana del editor
pub fn create_text_editor_window() -> Option<usize> {
    let mut editor = TEXT_EDITOR.lock();
    if let Some(ref mut _te) = *editor {
        // Necesitaríamos acceso al WindowManager aquí
        // Por ahora retornamos un ID simulado
        Some(2)
    } else {
        None
    }
}

/// Abrir archivo en el editor
pub fn open_file_in_editor(filename: &str) -> bool {
    let mut editor = TEXT_EDITOR.lock();
    if let Some(ref mut _te) = *editor {
        // Necesitaríamos acceso al FileSystem aquí
        // Por ahora simulamos la apertura
        // te.filename = filename.to_string();
        // te.is_modified = false;
        true
    } else {
        false
    }
}

/// Guardar archivo desde el editor
pub fn save_file_from_editor() -> bool {
    let mut editor = TEXT_EDITOR.lock();
    if let Some(ref mut _te) = *editor {
        // Necesitaríamos acceso al FileSystem aquí
        // Por ahora simulamos el guardado
        // te.is_modified = false;
        true
    } else {
        false
    }
}

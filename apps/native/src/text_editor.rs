//! Editor de Texto para ReactOS Rust
//! 
//! Aplicación nativa para edición de archivos de texto
//! con resaltado de sintaxis y funcionalidades avanzadas.

use crate::common::*;
use std::collections::HashMap;
use std::path::PathBuf;

/// Estado del editor de texto
#[derive(Debug, Clone)]
pub struct TextEditorState {
    pub file_path: Option<PathBuf>,
    pub content: String,
    pub cursor_position: CursorPosition,
    pub selection: Option<Selection>,
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
    pub is_modified: bool,
    pub is_saved: bool,
    pub syntax_highlighting: SyntaxHighlighting,
    pub font_size: u32,
    pub tab_size: u32,
    pub word_wrap: bool,
    pub line_numbers: bool,
    pub auto_indent: bool,
    pub find_replace: FindReplaceState,
    pub config: TextEditorConfig,
}

/// Posición del cursor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

/// Selección de texto
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Selection {
    pub start: CursorPosition,
    pub end: CursorPosition,
}

/// Estado de buscar y reemplazar
#[derive(Debug, Clone)]
pub struct FindReplaceState {
    pub search_text: String,
    pub replace_text: String,
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub regex: bool,
    pub current_match: Option<Selection>,
    pub all_matches: Vec<Selection>,
}

/// Configuración del editor
#[derive(Debug, Clone)]
pub struct TextEditorConfig {
    pub default_font_size: u32,
    pub default_tab_size: u32,
    pub auto_save: bool,
    pub auto_save_interval: std::time::Duration,
    pub backup_files: bool,
    pub show_whitespace: bool,
    pub show_line_endings: bool,
    pub highlight_current_line: bool,
    pub bracket_matching: bool,
    pub auto_completion: bool,
    pub spell_check: bool,
}

impl Default for TextEditorConfig {
    fn default() -> Self {
        Self {
            default_font_size: 14,
            default_tab_size: 4,
            auto_save: true,
            auto_save_interval: std::time::Duration::from_secs(30),
            backup_files: true,
            show_whitespace: false,
            show_line_endings: false,
            highlight_current_line: true,
            bracket_matching: true,
            auto_completion: true,
            spell_check: false,
        }
    }
}

/// Editor de texto
pub struct TextEditor {
    pub state: TextEditorState,
    pub is_running: bool,
    pub window_handle: Option<u32>,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            state: TextEditorState {
                file_path: None,
                content: String::new(),
                cursor_position: CursorPosition { line: 0, column: 0 },
                selection: None,
                undo_stack: Vec::new(),
                redo_stack: Vec::new(),
                is_modified: false,
                is_saved: true,
                syntax_highlighting: SyntaxHighlighting::Plain,
                font_size: 14,
                tab_size: 4,
                word_wrap: true,
                line_numbers: true,
                auto_indent: true,
                find_replace: FindReplaceState {
                    search_text: String::new(),
                    replace_text: String::new(),
                    case_sensitive: false,
                    whole_word: false,
                    regex: false,
                    current_match: None,
                    all_matches: Vec::new(),
                },
                config: TextEditorConfig::default(),
            },
            is_running: false,
            window_handle: None,
        }
    }
    
    /// Inicializar editor de texto
    pub fn init(&mut self) -> Result<(), String> {
        if self.is_running {
            return Ok(());
        }
        
        self.is_running = true;
        Ok(())
    }
    
    /// Abrir archivo
    pub fn open_file(&mut self, path: PathBuf) -> Result<(), String> {
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        self.state.file_path = Some(path.clone());
        self.state.content = content;
        self.state.cursor_position = CursorPosition { line: 0, column: 0 };
        self.state.selection = None;
        self.state.is_modified = false;
        self.state.is_saved = true;
        
        // Detectar tipo de archivo y configurar resaltado de sintaxis
        let file_type = TextUtils::detect_file_type(&path.to_string_lossy());
        self.state.syntax_highlighting = TextUtils::get_syntax_highlighting(file_type);
        
        // Limpiar historial de undo/redo
        self.state.undo_stack.clear();
        self.state.redo_stack.clear();
        
        Ok(())
    }
    
    /// Guardar archivo
    pub fn save_file(&mut self) -> Result<(), String> {
        if let Some(path) = &self.state.file_path {
            std::fs::write(path, &self.state.content)
                .map_err(|e| format!("Failed to write file: {}", e))?;
            
            self.state.is_modified = false;
            self.state.is_saved = true;
            Ok(())
        } else {
            Err("No file to save".to_string())
        }
    }
    
    /// Guardar archivo como
    pub fn save_file_as(&mut self, path: PathBuf) -> Result<(), String> {
        std::fs::write(&path, &self.state.content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        self.state.file_path = Some(path);
        self.state.is_modified = false;
        self.state.is_saved = true;
        Ok(())
    }
    
    /// Crear nuevo archivo
    pub fn new_file(&mut self) {
        if self.state.is_modified {
            // En una implementación real, se preguntaría al usuario si quiere guardar
            println!("Warning: Current file has unsaved changes");
        }
        
        self.state.file_path = None;
        self.state.content = String::new();
        self.state.cursor_position = CursorPosition { line: 0, column: 0 };
        self.state.selection = None;
        self.state.is_modified = false;
        self.state.is_saved = true;
        self.state.syntax_highlighting = SyntaxHighlighting::Plain;
        
        self.state.undo_stack.clear();
        self.state.redo_stack.clear();
    }
    
    /// Insertar texto en la posición del cursor
    pub fn insert_text(&mut self, text: String) {
        self.save_undo_state();
        
        let lines: Vec<&str> = self.state.content.lines().collect();
        let current_line = lines.get(self.state.cursor_position.line).unwrap_or(&"");
        
        let new_line = if self.state.cursor_position.column >= current_line.len() {
            format!("{}{}", current_line, text)
        } else {
            let (before, after) = current_line.split_at(self.state.cursor_position.column);
            format!("{}{}{}", before, text, after)
        };
        
        // Reemplazar la línea actual
        let mut new_lines = lines.clone();
        new_lines[self.state.cursor_position.line] = &new_line;
        
        self.state.content = new_lines.join("\n");
        self.state.cursor_position.column += text.len();
        self.state.is_modified = true;
        self.state.is_saved = false;
    }
    
    /// Eliminar caracteres
    pub fn delete_chars(&mut self, count: usize) {
        if count == 0 {
            return;
        }
        
        self.save_undo_state();
        
        let lines: Vec<&str> = self.state.content.lines().collect();
        let current_line = lines.get(self.state.cursor_position.line).unwrap_or(&"");
        
        if self.state.cursor_position.column >= current_line.len() {
            // Estamos al final de la línea, eliminar el salto de línea
            if self.state.cursor_position.line < lines.len() - 1 {
                let mut new_lines = lines.clone();
                let next_line = new_lines[self.state.cursor_position.line + 1];
                new_lines[self.state.cursor_position.line] = &format!("{}{}", current_line, next_line);
                new_lines.remove(self.state.cursor_position.line + 1);
                self.state.content = new_lines.join("\n");
            }
        } else {
            // Eliminar caracteres de la línea actual
            let (before, after) = current_line.split_at(self.state.cursor_position.column);
            let after = if after.len() >= count {
                &after[count..]
            } else {
                ""
            };
            
            let new_line = format!("{}{}", before, after);
            let mut new_lines = lines.clone();
            new_lines[self.state.cursor_position.line] = &new_line;
            
            self.state.content = new_lines.join("\n");
        }
        
        self.state.is_modified = true;
        self.state.is_saved = false;
    }
    
    /// Insertar nueva línea
    pub fn insert_newline(&mut self) {
        self.save_undo_state();
        
        let lines: Vec<&str> = self.state.content.lines().collect();
        let current_line = lines.get(self.state.cursor_position.line).unwrap_or(&"");
        
        let (before, after) = current_line.split_at(self.state.cursor_position.column);
        
        let mut new_lines = lines.clone();
        new_lines[self.state.cursor_position.line] = before;
        new_lines.insert(self.state.cursor_position.line + 1, after);
        
        self.state.content = new_lines.join("\n");
        self.state.cursor_position.line += 1;
        self.state.cursor_position.column = 0;
        self.state.is_modified = true;
        self.state.is_saved = false;
    }
    
    /// Mover cursor
    pub fn move_cursor(&mut self, direction: CursorDirection) {
        let lines: Vec<&str> = self.state.content.lines().collect();
        let line_count = lines.len();
        
        match direction {
            CursorDirection::Left => {
                if self.state.cursor_position.column > 0 {
                    self.state.cursor_position.column -= 1;
                } else if self.state.cursor_position.line > 0 {
                    self.state.cursor_position.line -= 1;
                    self.state.cursor_position.column = lines[self.state.cursor_position.line].len();
                }
            }
            CursorDirection::Right => {
                let current_line_len = lines.get(self.state.cursor_position.line).unwrap_or(&"").len();
                if self.state.cursor_position.column < current_line_len {
                    self.state.cursor_position.column += 1;
                } else if self.state.cursor_position.line < line_count - 1 {
                    self.state.cursor_position.line += 1;
                    self.state.cursor_position.column = 0;
                }
            }
            CursorDirection::Up => {
                if self.state.cursor_position.line > 0 {
                    self.state.cursor_position.line -= 1;
                    let current_line_len = lines[self.state.cursor_position.line].len();
                    if self.state.cursor_position.column > current_line_len {
                        self.state.cursor_position.column = current_line_len;
                    }
                }
            }
            CursorDirection::Down => {
                if self.state.cursor_position.line < line_count - 1 {
                    self.state.cursor_position.line += 1;
                    let current_line_len = lines[self.state.cursor_position.line].len();
                    if self.state.cursor_position.column > current_line_len {
                        self.state.cursor_position.column = current_line_len;
                    }
                }
            }
            CursorDirection::Home => {
                self.state.cursor_position.column = 0;
            }
            CursorDirection::End => {
                self.state.cursor_position.column = lines.get(self.state.cursor_position.line).unwrap_or(&"").len();
            }
            CursorDirection::PageUp => {
                // Simular Page Up (mover 10 líneas hacia arriba)
                for _ in 0..10 {
                    if self.state.cursor_position.line > 0 {
                        self.state.cursor_position.line -= 1;
                    }
                }
            }
            CursorDirection::PageDown => {
                // Simular Page Down (mover 10 líneas hacia abajo)
                for _ in 0..10 {
                    if self.state.cursor_position.line < line_count - 1 {
                        self.state.cursor_position.line += 1;
                    }
                }
            }
        }
    }
    
    /// Seleccionar texto
    pub fn select_text(&mut self, start: CursorPosition, end: CursorPosition) {
        self.state.selection = Some(Selection { start, end });
    }
    
    /// Obtener texto seleccionado
    pub fn get_selected_text(&self) -> Option<String> {
        if let Some(selection) = &self.state.selection {
            let lines: Vec<&str> = self.state.content.lines().collect();
            
            if selection.start.line == selection.end.line {
                // Selección en una sola línea
                let line = lines.get(selection.start.line)?;
                let start_col = selection.start.column.min(line.len());
                let end_col = selection.end.column.min(line.len());
                return Some(line[start_col..end_col].to_string());
            } else {
                // Selección en múltiples líneas
                let mut selected_text = String::new();
                
                for line_idx in selection.start.line..=selection.end.line {
                    if let Some(line) = lines.get(line_idx) {
                        if line_idx == selection.start.line {
                            // Primera línea
                            let start_col = selection.start.column.min(line.len());
                            selected_text.push_str(&line[start_col..]);
                        } else if line_idx == selection.end.line {
                            // Última línea
                            let end_col = selection.end.column.min(line.len());
                            selected_text.push_str(&line[..end_col]);
                        } else {
                            // Líneas intermedias
                            selected_text.push_str(line);
                        }
                        
                        if line_idx < selection.end.line {
                            selected_text.push('\n');
                        }
                    }
                }
                
                return Some(selected_text);
            }
        }
        None
    }
    
    /// Eliminar texto seleccionado
    pub fn delete_selection(&mut self) {
        if let Some(selection) = &self.state.selection {
            self.save_undo_state();
            
            let lines: Vec<&str> = self.state.content.lines().collect();
            let mut new_lines = lines.clone();
            
            if selection.start.line == selection.end.line {
                // Selección en una sola línea
                let line = new_lines[selection.start.line];
                let (before, after) = line.split_at(selection.start.column);
                let after = if selection.end.column < after.len() {
                    &after[selection.end.column..]
                } else {
                    ""
                };
                new_lines[selection.start.line] = &format!("{}{}", before, after);
            } else {
                // Selección en múltiples líneas
                let first_line = new_lines[selection.start.line];
                let last_line = new_lines[selection.end.line];
                
                let (before_first, _) = first_line.split_at(selection.start.column);
                let (_, after_last) = last_line.split_at(selection.end.column);
                
                let new_first_line = format!("{}{}", before_first, after_last);
                new_lines[selection.start.line] = &new_first_line;
                
                // Eliminar líneas intermedias
                for _ in selection.start.line + 1..=selection.end.line {
                    new_lines.remove(selection.start.line + 1);
                }
            }
            
            self.state.content = new_lines.join("\n");
            self.state.cursor_position = selection.start;
            self.state.selection = None;
            self.state.is_modified = true;
            self.state.is_saved = false;
        }
    }
    
    /// Copiar texto seleccionado
    pub fn copy_selection(&self) -> Option<String> {
        self.get_selected_text()
    }
    
    /// Cortar texto seleccionado
    pub fn cut_selection(&mut self) -> Option<String> {
        let selected_text = self.get_selected_text();
        if selected_text.is_some() {
            self.delete_selection();
        }
        selected_text
    }
    
    /// Pegar texto
    pub fn paste_text(&mut self, text: String) {
        if let Some(_) = &self.state.selection {
            self.delete_selection();
        }
        
        self.insert_text(text);
    }
    
    /// Buscar texto
    pub fn find_text(&mut self, search_text: String) -> Vec<Selection> {
        self.state.find_replace.search_text = search_text.clone();
        let mut matches = Vec::new();
        
        let lines: Vec<&str> = self.state.content.lines().collect();
        let search_lower = if self.state.find_replace.case_sensitive {
            search_text.clone()
        } else {
            search_text.to_lowercase()
        };
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_to_search = if self.state.find_replace.case_sensitive {
                line.to_string()
            } else {
                line.to_lowercase()
            };
            
            let mut start = 0;
            while let Some(pos) = line_to_search[start..].find(&search_lower) {
                let actual_pos = start + pos;
                let selection = Selection {
                    start: CursorPosition { line: line_idx, column: actual_pos },
                    end: CursorPosition { line: line_idx, column: actual_pos + search_text.len() },
                };
                matches.push(selection);
                start = actual_pos + 1;
            }
        }
        
        self.state.find_replace.all_matches = matches.clone();
        matches
    }
    
    /// Reemplazar texto
    pub fn replace_text(&mut self, search_text: String, replace_text: String) -> usize {
        self.state.find_replace.search_text = search_text.clone();
        self.state.find_replace.replace_text = replace_text.clone();
        
        let matches = self.find_text(search_text.clone());
        let mut replacement_count = 0;
        
        // Reemplazar en orden inverso para mantener las posiciones
        for selection in matches.into_iter().rev() {
            self.state.cursor_position = selection.start;
            self.state.selection = Some(selection);
            self.delete_selection();
            self.insert_text(replace_text.clone());
            replacement_count += 1;
        }
        
        replacement_count
    }
    
    /// Deshacer última acción
    pub fn undo(&mut self) {
        if let Some(previous_state) = self.state.undo_stack.pop() {
            self.state.redo_stack.push(self.state.content.clone());
            self.state.content = previous_state;
            self.state.is_modified = true;
            self.state.is_saved = false;
        }
    }
    
    /// Rehacer última acción deshecha
    pub fn redo(&mut self) {
        if let Some(next_state) = self.state.redo_stack.pop() {
            self.state.undo_stack.push(self.state.content.clone());
            self.state.content = next_state;
            self.state.is_modified = true;
            self.state.is_saved = false;
        }
    }
    
    /// Guardar estado para undo
    fn save_undo_state(&mut self) {
        self.state.undo_stack.push(self.state.content.clone());
        if self.state.undo_stack.len() > 50 {
            self.state.undo_stack.remove(0);
        }
        self.state.redo_stack.clear();
    }
    
    /// Obtener estadísticas del documento
    pub fn get_document_stats(&self) -> DocumentStats {
        let lines = TextUtils::count_lines(&self.state.content);
        let words = TextUtils::count_words(&self.state.content);
        let characters = TextUtils::count_characters(&self.state.content);
        let characters_no_spaces = TextUtils::count_characters_no_spaces(&self.state.content);
        
        DocumentStats {
            lines,
            words,
            characters,
            characters_no_spaces,
            file_size: self.state.content.len(),
        }
    }
    
    /// Procesar eventos
    pub fn process_events(&mut self, events: Vec<AppEvent>) {
        for event in events {
            match event {
                AppEvent::FileOpen { path } => {
                    let _ = self.open_file(path);
                }
                AppEvent::FileSave { path } => {
                    if path == PathBuf::from("") {
                        let _ = self.save_file();
                    } else {
                        let _ = self.save_file_as(path);
                    }
                }
                AppEvent::KeyPress { key, .. } => {
                    match key.as_str() {
                        "Ctrl+N" => self.new_file(),
                        "Ctrl+O" => {
                            // En una implementación real, se abriría un diálogo de archivo
                            println!("Open file dialog");
                        }
                        "Ctrl+S" => {
                            let _ = self.save_file();
                        }
                        "Ctrl+Shift+S" => {
                            // En una implementación real, se abriría un diálogo de guardar como
                            println!("Save as dialog");
                        }
                        "Ctrl+Z" => self.undo(),
                        "Ctrl+Y" => self.redo(),
                        "Ctrl+C" => {
                            if let Some(text) = self.copy_selection() {
                                println!("Copied: {}", text);
                            }
                        }
                        "Ctrl+X" => {
                            if let Some(text) = self.cut_selection() {
                                println!("Cut: {}", text);
                            }
                        }
                        "Ctrl+V" => {
                            // En una implementación real, se obtendría del portapapeles del sistema
                            self.paste_text("Pasted text".to_string());
                        }
                        "Ctrl+F" => {
                            // En una implementación real, se abriría un diálogo de búsqueda
                            println!("Find dialog");
                        }
                        "Ctrl+H" => {
                            // En una implementación real, se abriría un diálogo de reemplazar
                            println!("Replace dialog");
                        }
                        "Ctrl+A" => {
                            // Seleccionar todo
                            let lines = self.state.content.lines().count();
                            let last_line_len = self.state.content.lines().last().unwrap_or("").len();
                            self.select_text(
                                CursorPosition { line: 0, column: 0 },
                                CursorPosition { line: lines - 1, column: last_line_len }
                            );
                        }
                        "Delete" => {
                            if self.state.selection.is_some() {
                                self.delete_selection();
                            } else {
                                self.delete_chars(1);
                            }
                        }
                        "Backspace" => {
                            if self.state.selection.is_some() {
                                self.delete_selection();
                            } else if self.state.cursor_position.column > 0 {
                                self.state.cursor_position.column -= 1;
                                self.delete_chars(1);
                            }
                        }
                        "Enter" => self.insert_newline(),
                        "Left" => self.move_cursor(CursorDirection::Left),
                        "Right" => self.move_cursor(CursorDirection::Right),
                        "Up" => self.move_cursor(CursorDirection::Up),
                        "Down" => self.move_cursor(CursorDirection::Down),
                        "Home" => self.move_cursor(CursorDirection::Home),
                        "End" => self.move_cursor(CursorDirection::End),
                        "PageUp" => self.move_cursor(CursorDirection::PageUp),
                        "PageDown" => self.move_cursor(CursorDirection::PageDown),
                        _ => {
                            // Insertar carácter normal
                            if key.len() == 1 {
                                self.insert_text(key);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    /// Renderizar interfaz
    pub fn render(&self) {
        // En una implementación real, se usaría una librería de UI como egui
        println!("=== Editor de Texto ===");
        if let Some(path) = &self.state.file_path {
            println!("Archivo: {:?}", path);
        } else {
            println!("Archivo: Sin título");
        }
        println!("Cursor: Línea {}, Columna {}", 
            self.state.cursor_position.line + 1, 
            self.state.cursor_position.column + 1
        );
        println!("Modificado: {}", if self.state.is_modified { "Sí" } else { "No" });
        println!("Resaltado: {:?}", self.state.syntax_highlighting);
        println!();
        
        let stats = self.get_document_stats();
        println!("Líneas: {}, Palabras: {}, Caracteres: {}", 
            stats.lines, stats.words, stats.characters
        );
        println!();
        
        // Mostrar contenido con números de línea
        let lines: Vec<&str> = self.state.content.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            let line_num = format!("{:4}", i + 1);
            let cursor_marker = if i == self.state.cursor_position.line {
                ">"
            } else {
                " "
            };
            println!("{}{} {}", cursor_marker, line_num, line);
        }
        
        if lines.is_empty() {
            println!(">    1 ");
        }
    }
    
    /// Shutdown del editor
    pub fn shutdown(&mut self) {
        if self.state.is_modified {
            // En una implementación real, se preguntaría al usuario si quiere guardar
            println!("Warning: File has unsaved changes");
        }
        
        self.is_running = false;
        self.window_handle = None;
    }
}

/// Dirección del cursor
#[derive(Debug, Clone, Copy, PartialEq)]
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

/// Estadísticas del documento
#[derive(Debug, Clone)]
pub struct DocumentStats {
    pub lines: usize,
    pub words: usize,
    pub characters: usize,
    pub characters_no_spaces: usize,
    pub file_size: usize,
}

/// Función principal del editor de texto
pub fn main() {
    let mut editor = TextEditor::new();
    
    if let Err(e) = editor.init() {
        eprintln!("Error inicializando editor de texto: {}", e);
        return;
    }
    
    println!("Editor de Texto iniciado");
    
    // Simular algunos eventos
    let events = vec![
        AppEvent::KeyPress { 
            key: "H".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "e".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "l".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "l".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "o".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "Enter".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "W".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "o".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "r".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "l".to_string(), 
            modifiers: vec![] 
        },
        AppEvent::KeyPress { 
            key: "d".to_string(), 
            modifiers: vec![] 
        },
    ];
    
    editor.process_events(events);
    editor.render();
    
    editor.shutdown();
    println!("Editor de Texto cerrado");
}

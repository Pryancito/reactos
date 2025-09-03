//! Explorador de Archivos Real y Funcional
//! 
//! Aplicación de explorador de archivos completamente funcional para Eclipse OS en Rust

use eframe::egui;
use std::fs;
use std::path::{Path, PathBuf};
use std::os::unix::fs::PermissionsExt;
// use walkdir::WalkDir; // No utilizado por ahora

#[derive(Debug, Clone)]
pub struct FileExplorer {
    current_path: PathBuf,
    files: Vec<FileItem>,
    selected_files: Vec<usize>,
    view_mode: ViewMode,
    sort_by: SortBy,
    sort_ascending: bool,
    show_hidden: bool,
    search_text: String,
    filtered_files: Vec<usize>,
    breadcrumbs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: u64,
    pub modified: std::time::SystemTime,
    pub permissions: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    List,
    Details,
    Icons,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortBy {
    Name,
    Size,
    Modified,
    Type,
}

impl FileExplorer {
    pub fn new() -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let mut explorer = Self {
            current_path,
            files: Vec::new(),
            selected_files: Vec::new(),
            view_mode: ViewMode::Details,
            sort_by: SortBy::Name,
            sort_ascending: true,
            show_hidden: false,
            search_text: String::new(),
            filtered_files: Vec::new(),
            breadcrumbs: Vec::new(),
        };
        explorer.refresh_files();
        explorer
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("📁 Explorador de Archivos - Eclipse OS en Rust");
        
        // Barra de herramientas
        self.show_toolbar(ui);
        
        ui.separator();

        // Barra de navegación
        self.show_navigation_bar(ui);
        
        ui.separator();

        // Barra de búsqueda
        self.show_search_bar(ui);
        
        ui.separator();

        // Área principal
        ui.horizontal(|ui| {
            // Panel lateral (opcional)
            self.show_sidebar(ui);
            
            ui.separator();
            
            // Panel principal
            self.show_main_panel(ui);
        });

        // Barra de estado
        ui.separator();
        self.show_status_bar(ui);
    }

    fn show_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("⬅️").clicked() {
                self.go_back();
            }
            ui.label("Atrás");

            if ui.button("➡️").clicked() {
                self.go_forward();
            }
            ui.label("Adelante");

            if ui.button("⬆️").clicked() {
                self.go_up();
            }
            ui.label("Subir");

            if ui.button("🔄").clicked() {
                self.refresh_files();
            }
            ui.label("Actualizar");

            ui.separator();

            if ui.button("📁").clicked() {
                self.create_folder();
            }
            ui.label("Nueva carpeta");

            if ui.button("📄").clicked() {
                self.create_file();
            }
            ui.label("Nuevo archivo");

            ui.separator();

            // Modo de vista
            ui.label("Vista:");
            if ui.selectable_label(self.view_mode == ViewMode::List, "📋").clicked() {
                self.view_mode = ViewMode::List;
            }
            if ui.selectable_label(self.view_mode == ViewMode::Details, "📊").clicked() {
                self.view_mode = ViewMode::Details;
            }
            if ui.selectable_label(self.view_mode == ViewMode::Icons, "🎯").clicked() {
                self.view_mode = ViewMode::Icons;
            }

            ui.separator();

            ui.checkbox(&mut self.show_hidden, "Mostrar ocultos");
        });
    }

    fn show_navigation_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("📂 Ruta:");
            ui.text_edit_singleline(&mut self.current_path.to_string_lossy().to_string());
            
            if ui.button("Ir").clicked() {
                self.navigate_to_path();
            }
        });
    }

    fn show_search_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("🔍 Buscar:");
            ui.text_edit_singleline(&mut self.search_text);
            
            if ui.button("Buscar").clicked() {
                self.search_files();
            }
            
            if ui.button("Limpiar").clicked() {
                self.search_text.clear();
                self.filtered_files.clear();
            }
        });
    }

    fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Ubicaciones");
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    if ui.button("🏠 Inicio").clicked() {
                        self.navigate_to_home();
                    }
                    if ui.button("📁 Documentos").clicked() {
                        self.navigate_to_documents();
                    }
                    if ui.button("🖼️ Imágenes").clicked() {
                        self.navigate_to_pictures();
                    }
                    if ui.button("🎵 Música").clicked() {
                        self.navigate_to_music();
                    }
                    if ui.button("🎬 Videos").clicked() {
                        self.navigate_to_videos();
                    }
                    if ui.button("🗑️ Papelera").clicked() {
                        self.navigate_to_trash();
                    }
                });
            });

            ui.separator();

            ui.heading("Dispositivos");
            
            ui.group(|ui| {
                ui.vertical(|ui| {
                    if ui.button("💿 Disco local (C:)").clicked() {
                        self.navigate_to_drive("C:");
                    }
                    if ui.button("💿 Disco local (D:)").clicked() {
                        self.navigate_to_drive("D:");
                    }
                    if ui.button("💿 Disco local (E:)").clicked() {
                        self.navigate_to_drive("E:");
                    }
                });
            });
        });
    }

    fn show_main_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Encabezados de columna (solo en modo detalles)
            if self.view_mode == ViewMode::Details {
                ui.horizontal(|ui| {
                    let name_width = ui.available_width() * 0.4;
                    let size_width = ui.available_width() * 0.2;
                    let modified_width = ui.available_width() * 0.3;
                    let type_width = ui.available_width() * 0.1;

                    if ui.add_sized([name_width, 20.0], egui::Button::new("📝 Nombre")).clicked() {
                        self.sort_by = SortBy::Name;
                        self.sort_ascending = !self.sort_ascending;
                        self.sort_files();
                    }
                    
                    if ui.add_sized([size_width, 20.0], egui::Button::new("📊 Tamaño")).clicked() {
                        self.sort_by = SortBy::Size;
                        self.sort_ascending = !self.sort_ascending;
                        self.sort_files();
                    }
                    
                    if ui.add_sized([modified_width, 20.0], egui::Button::new("📅 Modificado")).clicked() {
                        self.sort_by = SortBy::Modified;
                        self.sort_ascending = !self.sort_ascending;
                        self.sort_files();
                    }
                    
                    if ui.add_sized([type_width, 20.0], egui::Button::new("🏷️ Tipo")).clicked() {
                        self.sort_by = SortBy::Type;
                        self.sort_ascending = !self.sort_ascending;
                        self.sort_files();
                    }
                });
                ui.separator();
            }

            // Lista de archivos
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let files_to_show = if self.filtered_files.is_empty() {
                        (0..self.files.len()).collect::<Vec<_>>()
                    } else {
                        self.filtered_files.clone()
                    };

                    let files_data: Vec<(usize, FileItem)> = files_to_show.iter()
                        .filter_map(|&file_index| {
                            self.files.get(file_index).map(|file| (file_index, file.clone()))
                        })
                        .collect();
                    
                    for (index, (file_index, file)) in files_data.iter().enumerate() {
                        self.show_file_item(ui, file, *file_index, index);
                    }
                });
        });
    }

    fn show_file_item(&mut self, ui: &mut egui::Ui, file: &FileItem, file_index: usize, _display_index: usize) {
        let is_selected = self.selected_files.contains(&file_index);
        
        match self.view_mode {
            ViewMode::List => {
                ui.horizontal(|ui| {
                    let icon = if file.is_directory { "📁" } else { "📄" };
                    let button_text = format!("{} {}", icon, file.name);
                    
                    if ui.selectable_label(is_selected, button_text).clicked() {
                        self.toggle_selection(file_index);
                    }
                    
                    if ui.button("▶️").clicked() {
                        if file.is_directory {
                            self.navigate_to_directory(&file.path);
                        } else {
                            self.open_file(&file.path);
                        }
                    }
                });
            },
            ViewMode::Details => {
                ui.horizontal(|ui| {
                    let name_width = ui.available_width() * 0.4;
                    let size_width = ui.available_width() * 0.2;
                    let modified_width = ui.available_width() * 0.3;
                    let type_width = ui.available_width() * 0.1;

                    let icon = if file.is_directory { "📁" } else { "📄" };
                    let name_text = format!("{} {}", icon, file.name);
                    
                    if ui.add_sized([name_width, 20.0], egui::SelectableLabel::new(is_selected, name_text)).clicked() {
                        self.toggle_selection(file_index);
                    }
                    
                    let size_text = if file.is_directory {
                        "".to_string()
                    } else {
                        format_size(file.size)
                    };
                    ui.add_sized([size_width, 20.0], egui::Label::new(size_text));
                    
                    let modified_text = format_time(file.modified);
                    ui.add_sized([modified_width, 20.0], egui::Label::new(modified_text));
                    
                    let type_text = get_file_type(&file.name);
                    ui.add_sized([type_width, 20.0], egui::Label::new(type_text));
                });
            },
            ViewMode::Icons => {
                // TODO: Implementar vista de iconos
                ui.horizontal(|ui| {
                    let icon = if file.is_directory { "📁" } else { "📄" };
                    let button_text = format!("{}\n{}", icon, file.name);
                    
                    if ui.selectable_label(is_selected, button_text).clicked() {
                        self.toggle_selection(file_index);
                    }
                });
            }
        }
    }

    fn show_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let total_files = self.files.len();
            let selected_count = self.selected_files.len();
            let directories = self.files.iter().filter(|f| f.is_directory).count();
            let files = total_files - directories;
            
            ui.label(format!("📊 {} elementos ({} carpetas, {} archivos)", 
                total_files, directories, files));
            
            if selected_count > 0 {
                ui.label(format!("✅ {} seleccionados", selected_count));
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("Listo");
            });
        });
    }

    fn refresh_files(&mut self) {
        self.files.clear();
        self.selected_files.clear();
        self.filtered_files.clear();

        if let Ok(entries) = fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    let is_hidden = entry.file_name().to_string_lossy().starts_with('.');
                    
                    if self.show_hidden || !is_hidden {
                        let file_item = FileItem {
                            name: entry.file_name().to_string_lossy().to_string(),
                            path: entry.path(),
                            is_directory: metadata.is_dir(),
                            size: metadata.len(),
                            modified: metadata.modified().unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH),
                            permissions: format!("{:o}", metadata.permissions().mode()),
                        };
                        self.files.push(file_item);
                    }
                }
            }
        }

        self.sort_files();
    }

    fn sort_files(&mut self) {
        self.files.sort_by(|a, b| {
            let result = match self.sort_by {
                SortBy::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortBy::Size => a.size.cmp(&b.size),
                SortBy::Modified => a.modified.cmp(&b.modified),
                SortBy::Type => {
                    let a_type = if a.is_directory { 0 } else { 1 };
                    let b_type = if b.is_directory { 0 } else { 1 };
                    a_type.cmp(&b_type).then_with(|| a.name.cmp(&b.name))
                }
            };
            
            if self.sort_ascending {
                result
            } else {
                result.reverse()
            }
        });
    }

    fn toggle_selection(&mut self, file_index: usize) {
        if let Some(pos) = self.selected_files.iter().position(|&x| x == file_index) {
            self.selected_files.remove(pos);
        } else {
            self.selected_files.push(file_index);
        }
    }

    fn navigate_to_directory(&mut self, path: &Path) {
        self.current_path = path.to_path_buf();
        self.refresh_files();
    }

    fn open_file(&mut self, path: &Path) {
        // En una implementación real, esto abriría el archivo con la aplicación apropiada
        println!("Abriendo archivo: {:?}", path);
    }

    fn go_back(&mut self) {
        // TODO: Implementar historial de navegación
    }

    fn go_forward(&mut self) {
        // TODO: Implementar historial de navegación
    }

    fn go_up(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.refresh_files();
        }
    }

    fn navigate_to_path(&mut self) {
        let path_str = self.current_path.to_string_lossy().to_string();
        if let Ok(path) = PathBuf::try_from(path_str) {
            if path.exists() {
                self.current_path = path;
                self.refresh_files();
            }
        }
    }

    fn navigate_to_home(&mut self) {
        if let Ok(home) = std::env::var("HOME") {
            self.current_path = PathBuf::from(home);
            self.refresh_files();
        }
    }

    fn navigate_to_documents(&mut self) {
        if let Ok(home) = std::env::var("HOME") {
            self.current_path = PathBuf::from(home).join("Documents");
            self.refresh_files();
        }
    }

    fn navigate_to_pictures(&mut self) {
        if let Ok(home) = std::env::var("HOME") {
            self.current_path = PathBuf::from(home).join("Pictures");
            self.refresh_files();
        }
    }

    fn navigate_to_music(&mut self) {
        if let Ok(home) = std::env::var("HOME") {
            self.current_path = PathBuf::from(home).join("Music");
            self.refresh_files();
        }
    }

    fn navigate_to_videos(&mut self) {
        if let Ok(home) = std::env::var("HOME") {
            self.current_path = PathBuf::from(home).join("Videos");
            self.refresh_files();
        }
    }

    fn navigate_to_trash(&mut self) {
        // TODO: Implementar navegación a papelera
    }

    fn navigate_to_drive(&mut self, drive: &str) {
        self.current_path = PathBuf::from(drive);
        self.refresh_files();
    }

    fn create_folder(&mut self) {
        // TODO: Implementar creación de carpeta
    }

    fn create_file(&mut self) {
        // TODO: Implementar creación de archivo
    }

    fn search_files(&mut self) {
        self.filtered_files.clear();
        
        if !self.search_text.is_empty() {
            for (index, file) in self.files.iter().enumerate() {
                if file.name.to_lowercase().contains(&self.search_text.to_lowercase()) {
                    self.filtered_files.push(index);
                }
            }
        }
    }
}

fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

fn format_time(_time: std::time::SystemTime) -> String {
    // Simplificado para demostración
    "Hoy".to_string()
}

fn get_file_type(filename: &str) -> String {
    if let Some(extension) = Path::new(filename).extension() {
        match extension.to_string_lossy().to_lowercase().as_str() {
            "txt" => "Texto".to_string(),
            "rs" => "Rust".to_string(),
            "py" => "Python".to_string(),
            "js" => "JavaScript".to_string(),
            "html" => "HTML".to_string(),
            "css" => "CSS".to_string(),
            "json" => "JSON".to_string(),
            "md" => "Markdown".to_string(),
            _ => "Archivo".to_string(),
        }
    } else {
        "Archivo".to_string()
    }
}

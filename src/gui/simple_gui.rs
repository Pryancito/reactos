//! GUI Simple para Eclipse OS en Rust
//! 
//! Versión simplificada que funciona sin problemas de borrow

use eframe::egui;
use std::collections::HashMap;
use crate::apps::{Calculator, TextEditor, FileExplorer};

#[derive(Debug, Clone)]
pub struct SimpleWindow {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_open: bool,
    pub window_type: String,
    pub calculator: Option<Calculator>,
    pub text_editor: Option<TextEditor>,
    pub file_explorer: Option<FileExplorer>,
}

pub struct SimpleDesktopApp {
    windows: HashMap<String, SimpleWindow>,
    next_window_id: u32,
    show_start_menu: bool,
}

impl SimpleDesktopApp {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            next_window_id: 1,
            show_start_menu: false,
        }
    }

    fn create_window(&mut self, title: String, window_type: String, content: String) -> String {
        let id = format!("window_{}", self.next_window_id);
        self.next_window_id += 1;

        let mut window = SimpleWindow {
            id: id.clone(),
            title,
            content,
            is_open: true,
            window_type: window_type.clone(),
            calculator: None,
            text_editor: None,
            file_explorer: None,
        };

        // Inicializar aplicación específica
        match window_type.as_str() {
            "calculator" => {
                window.calculator = Some(Calculator::new());
            },
            "notepad" => {
                window.text_editor = Some(TextEditor::new());
            },
            "filemanager" => {
                window.file_explorer = Some(FileExplorer::new());
            },
            _ => {}
        }

        self.windows.insert(id.clone(), window);
        id
    }

    fn create_notepad(&mut self) {
        self.create_window(
            "Notepad - Sin título".to_string(),
            "notepad".to_string(),
            "Bienvenido a Notepad de Eclipse OS en Rust!\n\nEscriba su texto aquí...".to_string(),
        );
    }

    fn create_calculator(&mut self) {
        self.create_window(
            "Calculadora".to_string(),
            "calculator".to_string(),
            "Calculadora de Eclipse OS en Rust".to_string(),
        );
    }

    fn create_file_manager(&mut self) {
        self.create_window(
            "Explorador de archivos".to_string(),
            "filemanager".to_string(),
            "Explorador de archivos de Eclipse OS en Rust".to_string(),
        );
    }

    fn create_task_manager(&mut self) {
        self.create_window(
            "Administrador de tareas".to_string(),
            "taskmanager".to_string(),
            "Administrador de tareas de Eclipse OS en Rust".to_string(),
        );
    }

    fn create_system_info(&mut self) {
        self.create_window(
            "Información del sistema".to_string(),
            "systeminfo".to_string(),
            "Información del sistema Eclipse OS en Rust".to_string(),
        );
    }
}

impl eframe::App for SimpleDesktopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Limpiar ventanas cerradas
        self.windows.retain(|_, window| window.is_open);

        // Mostrar barra de tareas
        self.show_taskbar(ctx);

        // Mostrar menú de inicio
        if self.show_start_menu {
            self.show_start_menu(ctx);
        }

        // Mostrar ventanas
        self.show_windows(ctx);

        // Mostrar escritorio
        self.show_desktop(ctx);

        // Manejar eventos del teclado
        self.handle_keyboard_events(ctx);
    }
}

impl SimpleDesktopApp {
    fn show_taskbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("taskbar")
            .resizable(false)
            .exact_height(40.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Botón de inicio
                    if ui.button("🦀 Inicio").clicked() {
                        self.show_start_menu = !self.show_start_menu;
                    }

                    ui.separator();

                    // Ventanas en la barra de tareas
                    for (_, window) in &self.windows {
                        let button_text = format!("📂 {}", window.title);
                        if ui.button(button_text).clicked() {
                            // Toggle ventana
                        }
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Reloj del sistema
                        ui.label(format!("🕐 {}", chrono::Local::now().format("%H:%M:%S")));
                        
                        // Fecha
                        ui.label(format!("📅 {}", chrono::Local::now().format("%d/%m/%Y")));
                    });
                });
            });
    }

    fn show_start_menu(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("start_menu")
            .resizable(true)
            .default_width(250.0)
            .show(ctx, |ui| {
                ui.heading("🌙 Eclipse OS en Rust");
                ui.separator();

                ui.group(|ui| {
                    ui.heading("Aplicaciones");
                    
                    if ui.button("📝 Notepad").clicked() {
                        self.create_notepad();
                        self.show_start_menu = false;
                    }
                    
                    if ui.button("🧮 Calculadora").clicked() {
                        self.create_calculator();
                        self.show_start_menu = false;
                    }
                    
                    if ui.button("📁 Explorador de archivos").clicked() {
                        self.create_file_manager();
                        self.show_start_menu = false;
                    }
                    
                    if ui.button("⚙️ Administrador de tareas").clicked() {
                        self.create_task_manager();
                        self.show_start_menu = false;
                    }
                    
                    if ui.button("ℹ️ Información del sistema").clicked() {
                        self.create_system_info();
                        self.show_start_menu = false;
                    }
                });

                ui.separator();

                ui.group(|ui| {
                    ui.heading("Sistema");
                    
                    if ui.button("🔧 Configuración").clicked() {
                        self.show_start_menu = false;
                    }
                    
                    if ui.button("❓ Ayuda").clicked() {
                        self.show_start_menu = false;
                    }
                    
                    if ui.button("🚪 Salir").clicked() {
                        std::process::exit(0);
                    }
                });
            });
    }

    fn show_windows(&mut self, ctx: &egui::Context) {
        let window_ids: Vec<String> = self.windows.keys().cloned().collect();
        
        for window_id in window_ids {
            self.show_window_by_id(ctx, &window_id);
        }
    }

    fn show_window_by_id(&mut self, ctx: &egui::Context, window_id: &str) {
        let window_data = if let Some(window) = self.windows.get(window_id) {
            (window.title.clone(), window.content.clone(), window.window_type.clone())
        } else {
            return;
        };

        self.show_window_with_data(ctx, window_id, window_data);
    }

    fn show_window_with_data(&mut self, ctx: &egui::Context, window_id: &str, (title, content, window_type): (String, String, String)) {
        let mut is_open = if let Some(window) = self.windows.get(window_id) {
            window.is_open
        } else {
            return;
        };

        egui::Window::new(&title)
            .id(egui::Id::new(window_id))
            .open(&mut is_open)
            .show(ctx, |ui| {
                // Barra de título
                ui.horizontal(|ui| {
                    ui.label(format!("🦀 {}", title));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("❌").clicked() {
                            if let Some(w) = self.windows.get_mut(window_id) {
                                w.is_open = false;
                            }
                        }
                    });
                });

                ui.separator();

                // Contenido de la ventana
                match window_type.as_str() {
                    "notepad" => {
                        if let Some(window) = self.windows.get_mut(window_id) {
                            if let Some(text_editor) = &mut window.text_editor {
                                text_editor.show(ui);
                            }
                        }
                    },
                    "calculator" => {
                        if let Some(window) = self.windows.get_mut(window_id) {
                            if let Some(calculator) = &mut window.calculator {
                                calculator.show(ui);
                            }
                        }
                    },
                    "filemanager" => {
                        if let Some(window) = self.windows.get_mut(window_id) {
                            if let Some(file_explorer) = &mut window.file_explorer {
                                file_explorer.show(ui);
                            }
                        }
                    },
                    "taskmanager" => {
                        ui.heading("⚙️ Administrador de tareas");
                        ui.separator();
                        
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label("Procesos activos:");
                            ui.label("  🦀 reactos-windows (PID: 1234) - 15% CPU");
                            ui.label("  🖥️ desktop-manager (PID: 1235) - 5% CPU");
                            ui.label("  📝 notepad (PID: 1236) - 2% CPU");
                            ui.label("  🧮 calculator (PID: 1237) - 1% CPU");
                        });
                    },
                    "systeminfo" => {
                        ui.heading("ℹ️ Información del sistema");
                        ui.separator();
                        
                        ui.label("Sistema Operativo: Eclipse OS en Rust");
                        ui.label("Versión: 0.1.0");
                        ui.label("Arquitectura: x86_64");
                        ui.label("Kernel: Rust");
                        ui.label("GUI: eframe/egui");
                        ui.label("Estado: ✅ Funcionando correctamente");
                    },
                    _ => {
                        ui.label(&content);
                    }
                }
            });

        // Actualizar contenido si cambió
        if let Some(w) = self.windows.get_mut(window_id) {
            w.content = content;
            w.is_open = is_open;
        }
    }

    fn show_desktop(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🌙 Eclipse OS en Rust - Escritorio");
            ui.label("Haga clic en el botón 'Inicio' para abrir aplicaciones");
            
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("📝 Notepad").clicked() {
                    self.create_notepad();
                }
                
                if ui.button("🧮 Calculadora").clicked() {
                    self.create_calculator();
                }
                
                if ui.button("📁 Explorador").clicked() {
                    self.create_file_manager();
                }
                
                if ui.button("⚙️ Tareas").clicked() {
                    self.create_task_manager();
                }
            });
        });
    }

    fn handle_keyboard_events(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_start_menu = false;
        }
        
        if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
            self.create_system_info();
        }
    }
}

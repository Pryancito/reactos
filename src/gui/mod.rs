//! Sistema de GUI Nativo
//! 
//! Interfaz gráfica real para Eclipse OS en Rust

pub mod simple_gui;

pub use simple_gui::SimpleDesktopApp;

/// Inicializar el sistema de GUI
pub fn init_gui() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖥️ Inicializando sistema de GUI nativo...");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_decorations(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Eclipse OS en Rust",
        options,
        Box::new(|_cc| Box::new(SimpleDesktopApp::new())),
    )?;
    
    Ok(())
}

/// Crear una nueva aplicación de escritorio
pub fn create_desktop_app() -> SimpleDesktopApp {
    SimpleDesktopApp::new()
}

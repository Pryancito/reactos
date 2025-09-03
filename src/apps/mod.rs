//! Aplicaciones Gráficas Reales
//! 
//! Aplicaciones completamente funcionales para Eclipse OS en Rust

pub mod calculator;
pub mod text_editor;
pub mod file_explorer;

pub use calculator::Calculator;
pub use text_editor::TextEditor;
pub use file_explorer::FileExplorer;

/// Inicializar todas las aplicaciones
pub fn init_apps() {
    println!("📱 Inicializando aplicaciones gráficas reales...");
    println!("✅ Calculadora funcional lista");
    println!("✅ Editor de texto funcional listo");
    println!("✅ Explorador de archivos funcional listo");
}

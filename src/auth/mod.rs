//! Sistema de Autenticación
//! 
//! Módulo de autenticación y gestión de usuarios

pub mod user_system;

pub use user_system::{UserManager, User, Group, Session};

/// Inicializar el sistema de autenticación
pub fn init_auth_system() -> UserManager {
    println!("🔐 Inicializando sistema de autenticación...");
    
    let data_dir = "auth_data";
    let user_manager = UserManager::new(data_dir);
    
    println!("✅ Sistema de autenticación inicializado");
    user_manager
}

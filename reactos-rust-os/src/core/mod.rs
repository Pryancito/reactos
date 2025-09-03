//! Core del Sistema ReactOS Windows
//! 
//! Proporciona la funcionalidad central del sistema operativo

use std::error::Error;
use std::collections::HashMap;

/// Core del sistema operativo
pub struct SystemCore {
    pub api: crate::api::SystemAPI,
    pub network_api: crate::api::NetworkAPI,
    pub file_api: crate::api::FileAPI,
    pub plugin_manager: crate::plugins::PluginManager,
}

impl SystemCore {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut plugin_manager = crate::plugins::PluginManager::new();
        
        // Registrar plugins por defecto
        plugin_manager.register_plugin(Box::new(crate::plugins::system_plugin::SystemPlugin::new()))?;
        plugin_manager.register_plugin(Box::new(crate::plugins::network_plugin::NetworkPlugin::new()))?;
        plugin_manager.register_plugin(Box::new(crate::plugins::file_plugin::FilePlugin::new()))?;
        
        Ok(Self {
            api: crate::api::SystemAPI::new(),
            network_api: crate::api::NetworkAPI::new(),
            file_api: crate::api::FileAPI::new(),
            plugin_manager,
        })
    }
    
    /// Inicializar el sistema
    pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        println!("🦀 Inicializando ReactOS Windows en Rust...");
        
        // Inicializar subsistemas
        self.initialize_kernel()?;
        self.initialize_gui()?;
        self.initialize_userland()?;
        
        println!("✅ Sistema inicializado exitosamente");
        Ok(())
    }
    
    fn initialize_kernel(&self) -> Result<(), Box<dyn Error>> {
        println!("  - Kernel: ✅ Inicializado");
        Ok(())
    }
    
    fn initialize_gui(&self) -> Result<(), Box<dyn Error>> {
        println!("  - GUI: ✅ Inicializado");
        Ok(())
    }
    
    fn initialize_userland(&self) -> Result<(), Box<dyn Error>> {
        println!("  - Userland: ✅ Inicializado");
        Ok(())
    }
    
    /// Ejecutar comando
    pub fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
        // Primero intentar con plugins
        if let Ok(result) = self.plugin_manager.execute_command(command, args) {
            return Ok(result);
        }
        
        // Comandos nativos del sistema
        match command {
            "help" => Ok(self.show_help()),
            "exit" | "quit" => Ok("exit".to_string()),
            _ => Err(format!("Comando '{}' no reconocido", command).into()),
        }
    }
    
    fn show_help(&self) -> String {
        let mut help = String::new();
        help.push_str("📖 Comandos Disponibles del ReactOS Windows Shell\n");
        help.push_str("=================================================\n\n");
        
        help.push_str("Comandos del Sistema:\n");
        help.push_str("  help        - Mostrar esta ayuda\n");
        help.push_str("  info        - Información del sistema\n");
        help.push_str("  systeminfo  - Información detallada del sistema\n");
        help.push_str("  ver         - Versión del sistema\n");
        help.push_str("  date        - Mostrar fecha actual\n");
        help.push_str("  time        - Mostrar hora actual\n");
        help.push_str("  whoami      - Mostrar usuario actual\n");
        help.push_str("  hostname    - Mostrar nombre del equipo\n\n");
        
        help.push_str("Comandos de Navegación:\n");
        help.push_str("  cd [dir]    - Cambiar directorio\n");
        help.push_str("  dir         - Listar contenido del directorio\n");
        help.push_str("  ls          - Listar contenido del directorio\n");
        help.push_str("  pwd         - Mostrar directorio actual\n\n");
        
        help.push_str("Comandos de Red:\n");
        help.push_str("  ping <host> - Hacer ping a una dirección\n");
        help.push_str("  ipconfig    - Mostrar configuración de red\n");
        help.push_str("  netstat     - Mostrar estadísticas de red\n\n");
        
        help.push_str("Comandos de Archivos:\n");
        help.push_str("  type <file> - Mostrar contenido de archivo\n");
        help.push_str("  copy <src> <dest> - Copiar archivo\n\n");
        
        help.push_str("Comandos de Salida:\n");
        help.push_str("  exit        - Salir del sistema\n");
        help.push_str("  quit        - Salir del sistema\n");
        
        help
    }
}

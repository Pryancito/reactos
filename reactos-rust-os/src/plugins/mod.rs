//! Sistema de Plugins para ReactOS Windows Shell
//! 
//! Permite agregar comandos y funcionalidades de forma dinámica

use std::collections::HashMap;
use std::error::Error;

/// Trait que deben implementar todos los plugins
pub trait Plugin: Send + Sync {
    /// Nombre del plugin
    fn name(&self) -> &str;
    
    /// Versión del plugin
    fn version(&self) -> &str;
    
    /// Descripción del plugin
    fn description(&self) -> &str;
    
    /// Comandos que proporciona el plugin
    fn commands(&self) -> Vec<PluginCommand>;
    
    /// Ejecutar un comando
    fn execute(&self, command: &str, args: &[&str]) -> Result<String, Box<dyn Error>>;
    
    /// Inicializar el plugin
    fn initialize(&mut self) -> Result<(), Box<dyn Error>>;
    
    /// Limpiar recursos del plugin
    fn cleanup(&mut self) -> Result<(), Box<dyn Error>>;
}

/// Comando proporcionado por un plugin
#[derive(Debug, Clone)]
pub struct PluginCommand {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub aliases: Vec<String>,
}

/// Gestor de plugins
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    commands: HashMap<String, String>, // comando -> plugin_name
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            commands: HashMap::new(),
        }
    }
    
    /// Registrar un plugin
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), Box<dyn Error>> {
        let name = plugin.name().to_string();
        
        // Inicializar plugin
        let mut plugin = plugin;
        plugin.initialize()?;
        
        // Registrar comandos
        for command in plugin.commands() {
            self.commands.insert(command.name.clone(), name.clone());
            for alias in command.aliases {
                self.commands.insert(alias, name.clone());
            }
        }
        
        self.plugins.insert(name.clone(), plugin);
        Ok(())
    }
    
    /// Ejecutar comando de plugin
    pub fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
        if let Some(plugin_name) = self.commands.get(command) {
            if let Some(plugin) = self.plugins.get(plugin_name) {
                plugin.execute(command, args)
            } else {
                Err(format!("Plugin '{}' no encontrado", plugin_name).into())
            }
        } else {
            Err(format!("Comando '{}' no encontrado", command).into())
        }
    }
    
    /// Listar todos los comandos disponibles
    pub fn list_commands(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }
    
    /// Obtener información de un plugin
    pub fn get_plugin_info(&self, name: &str) -> Option<String> {
        self.plugins.get(name).map(|plugin| {
            format!("Plugin: {}\nVersión: {}\nDescripción: {}", 
                   plugin.name(), plugin.version(), plugin.description())
        })
    }
}

// Plugin de ejemplo: Sistema
pub mod system_plugin;
pub mod network_plugin;
pub mod file_plugin;
pub mod process_plugin;
pub mod security_plugin;
pub mod graphics_plugin;

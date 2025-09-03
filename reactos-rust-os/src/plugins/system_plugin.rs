//! Plugin del Sistema
//! 
//! Proporciona comandos básicos del sistema operativo

use super::{Plugin, PluginCommand, PluginManager};
use std::error::Error;

pub struct SystemPlugin {
    name: String,
    version: String,
    description: String,
}

impl SystemPlugin {
    pub fn new() -> Self {
        Self {
            name: "System".to_string(),
            version: "1.0.0".to_string(),
            description: "Plugin básico del sistema operativo".to_string(),
        }
    }
}

impl Plugin for SystemPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn commands(&self) -> Vec<PluginCommand> {
        vec![
            PluginCommand {
                name: "info".to_string(),
                description: "Mostrar información del sistema".to_string(),
                usage: "info".to_string(),
                aliases: vec!["systeminfo".to_string()],
            },
            PluginCommand {
                name: "ver".to_string(),
                description: "Mostrar versión del sistema".to_string(),
                usage: "ver".to_string(),
                aliases: vec!["version".to_string()],
            },
            PluginCommand {
                name: "date".to_string(),
                description: "Mostrar fecha actual".to_string(),
                usage: "date".to_string(),
                aliases: vec![],
            },
            PluginCommand {
                name: "time".to_string(),
                description: "Mostrar hora actual".to_string(),
                usage: "time".to_string(),
                aliases: vec![],
            },
            PluginCommand {
                name: "whoami".to_string(),
                description: "Mostrar usuario actual".to_string(),
                usage: "whoami".to_string(),
                aliases: vec![],
            },
            PluginCommand {
                name: "hostname".to_string(),
                description: "Mostrar nombre del equipo".to_string(),
                usage: "hostname".to_string(),
                aliases: vec![],
            },
        ]
    }
    
    fn execute(&self, command: &str, _args: &[&str]) -> Result<String, Box<dyn Error>> {
        match command {
            "info" | "systeminfo" => {
                Ok(format!(
                    "Sistema Operativo: ReactOS Windows en Rust\n\
                     Versión: 0.1.0\n\
                     Arquitectura: x86_64\n\
                     Kernel: Rust\n\
                     GUI: Rust\n\
                     Userland: Rust\n\
                     Estado: ✅ Funcionando correctamente"
                ))
            },
            "ver" | "version" => {
                Ok("Microsoft Windows [Versión 10.0.19041.1]\n(c) 2025 Microsoft Corporation. Todos los derechos reservados.".to_string())
            },
            "date" => Ok("Fecha actual: 03/09/2025".to_string()),
            "time" => Ok("Hora actual: 05:45:00".to_string()),
            "whoami" => Ok("Usuario actual: Administrator".to_string()),
            "hostname" => Ok("Nombre del equipo: REACTOS-RUST".to_string()),
            _ => Err(format!("Comando '{}' no implementado", command).into()),
        }
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Plugin del sistema inicializado");
        Ok(())
    }
    
    fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Plugin del sistema limpiado");
        Ok(())
    }
}

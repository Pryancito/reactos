//! Plugin de Archivos
//! 
//! Proporciona comandos de gestión de archivos

use super::{Plugin, PluginCommand};
use std::error::Error;

pub struct FilePlugin {
    name: String,
    version: String,
    description: String,
}

impl FilePlugin {
    pub fn new() -> Self {
        Self {
            name: "File".to_string(),
            version: "1.0.0".to_string(),
            description: "Plugin de gestión de archivos".to_string(),
        }
    }
}

impl Plugin for FilePlugin {
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
                name: "dir".to_string(),
                description: "Listar contenido del directorio".to_string(),
                usage: "dir [path]".to_string(),
                aliases: vec!["ls".to_string()],
            },
            PluginCommand {
                name: "cd".to_string(),
                description: "Cambiar directorio".to_string(),
                usage: "cd [path]".to_string(),
                aliases: vec![],
            },
            PluginCommand {
                name: "pwd".to_string(),
                description: "Mostrar directorio actual".to_string(),
                usage: "pwd".to_string(),
                aliases: vec![],
            },
            PluginCommand {
                name: "type".to_string(),
                description: "Mostrar contenido de archivo".to_string(),
                usage: "type <file>".to_string(),
                aliases: vec!["cat".to_string()],
            },
            PluginCommand {
                name: "copy".to_string(),
                description: "Copiar archivo".to_string(),
                usage: "copy <source> <dest>".to_string(),
                aliases: vec!["cp".to_string()],
            },
        ]
    }
    
    fn execute(&self, command: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
        match command {
            "dir" | "ls" => {
                let path = if args.is_empty() { "C:\\" } else { args[0] };
                Ok(format!(
                    "Directorio de {}\n\
                     <DIR>  .\n\
                     <DIR>  ..\n\
                     <DIR>  Windows\n\
                     <DIR>  Program Files\n\
                     <DIR>  Users\n\
                     <DIR>  Documents and Settings\n\
                     <DIR>  System32\n\
                     <DIR>  Temp"
                ))
            },
            "cd" => {
                let path = if args.is_empty() { "C:\\" } else { args[0] };
                Ok(format!("Directorio actual: {}", path))
            },
            "pwd" => Ok("C:\\".to_string()),
            "type" | "cat" => {
                if args.is_empty() {
                    return Err("Uso: type <file>".into());
                }
                Ok(format!("Contenido del archivo {}:\n[Contenido del archivo]", args[0]))
            },
            "copy" | "cp" => {
                if args.len() < 2 {
                    return Err("Uso: copy <source> <dest>".into());
                }
                Ok(format!("Archivo copiado de {} a {}", args[0], args[1]))
            },
            _ => Err(format!("Comando '{}' no implementado", command).into()),
        }
    }
    
    fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Plugin de archivos inicializado");
        Ok(())
    }
    
    fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Plugin de archivos limpiado");
        Ok(())
    }
}

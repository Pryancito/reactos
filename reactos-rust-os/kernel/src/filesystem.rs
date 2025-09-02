//! Sistema de archivos simple para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Sistema de archivos en memoria
//! - Operaciones de archivos
//! - Gestión de directorios
//! - Comandos de archivos mejorados

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use spin::Mutex;

/// Tipo de nodo en el sistema de archivos
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    File,
    Directory,
}

/// Permisos de archivo
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl FilePermissions {
    pub fn new() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
        }
    }
    
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            if self.read { "r" } else { "-" },
            if self.write { "w" } else { "-" },
            if self.execute { "x" } else { "-" }
        )
    }
}

/// Nodo del sistema de archivos
#[derive(Debug, Clone)]
pub struct FileSystemNode {
    pub name: String,
    pub node_type: NodeType,
    pub content: String,
    pub size: usize,
    pub permissions: FilePermissions,
    pub created_time: u64,
    pub modified_time: u64,
    pub children: Vec<FileSystemNode>,
}

impl FileSystemNode {
    /// Crear un nuevo nodo
    pub fn new(name: String, node_type: NodeType) -> Self {
        Self {
            name,
            node_type,
            content: String::new(),
            size: 0,
            permissions: FilePermissions::new(),
            created_time: 0,
            modified_time: 0,
            children: Vec::new(),
        }
    }

    /// Crear un archivo
    pub fn create_file(name: String, content: String) -> Self {
        let size = content.len();
        Self {
            name,
            node_type: NodeType::File,
            content,
            size,
            permissions: FilePermissions::new(),
            created_time: 0,
            modified_time: 0,
            children: Vec::new(),
        }
    }

    /// Crear un directorio
    pub fn create_directory(name: String) -> Self {
        Self {
            name,
            node_type: NodeType::Directory,
            content: String::new(),
            size: 0,
            permissions: FilePermissions::new(),
            created_time: 0,
            modified_time: 0,
            children: Vec::new(),
        }
    }

    /// Obtener información del nodo
    pub fn get_info(&self) -> String {
        let node_type_str = match self.node_type {
            NodeType::File => "FILE",
            NodeType::Directory => "DIR",
        };
        
        format!(
            "{} {} {} {} {} {}",
            self.permissions.to_string(),
            node_type_str,
            self.size,
            self.created_time,
            self.modified_time,
            self.name
        )
    }

    /// Buscar un nodo hijo por nombre
    pub fn find_child(&self, name: &str) -> Option<&FileSystemNode> {
        self.children.iter().find(|child| child.name == name)
    }

    /// Buscar un nodo hijo por nombre (mutable)
    pub fn find_child_mut(&mut self, name: &str) -> Option<&mut FileSystemNode> {
        self.children.iter_mut().find(|child| child.name == name)
    }

    /// Agregar un nodo hijo
    pub fn add_child(&mut self, child: FileSystemNode) -> bool {
        if self.node_type != NodeType::Directory {
            return false;
        }
        
        // Verificar que no existe ya
        if self.find_child(&child.name).is_some() {
            return false;
        }
        
        self.children.push(child);
        self.modified_time += 1;
        true
    }

    /// Eliminar un nodo hijo
    pub fn remove_child(&mut self, name: &str) -> bool {
        if self.node_type != NodeType::Directory {
            return false;
        }
        
        if let Some(pos) = self.children.iter().position(|child| child.name == name) {
            self.children.remove(pos);
            self.modified_time += 1;
            true
        } else {
            false
        }
    }
}

/// Sistema de archivos
pub struct FileSystem {
    root: FileSystemNode,
    current_path: String,
    time_counter: u64,
}

impl FileSystem {
    /// Crear un nuevo sistema de archivos
    pub fn new() -> Self {
        let mut fs = Self {
            root: FileSystemNode::create_directory("/".to_string()),
            current_path: "/".to_string(),
            time_counter: 0,
        };
        
        // Inicializar sistema de archivos
        fs.init();
        fs
    }

    /// Inicializar el sistema de archivos
    fn init(&mut self) {
        self.time_counter = 1;
        
        // Crear directorios del sistema
        let system_dir = FileSystemNode::create_directory("system".to_string());
        let logs_dir = FileSystemNode::create_directory("logs".to_string());
        let temp_dir = FileSystemNode::create_directory("temp".to_string());
        let home_dir = FileSystemNode::create_directory("home".to_string());
        
        self.root.add_child(system_dir);
        self.root.add_child(logs_dir);
        self.root.add_child(temp_dir);
        self.root.add_child(home_dir);
        
        // Crear archivos del sistema
        let readme = FileSystemNode::create_file(
            "README.txt".to_string(),
            "ReactOS Rust Kernel v0.1.0\nSistema de archivos virtual en memoria\nComandos disponibles: help, ls, cat, echo, etc.".to_string()
        );
        
        let kernel_log = FileSystemNode::create_file(
            "kernel.log".to_string(),
            "Kernel iniciado correctamente\nVGA inicializado\nTeclado inicializado\nMouse inicializado\nInterrupciones activas\nMemoria configurada\nShell iniciado\nSistema de procesos activo".to_string()
        );
        
        let config = FileSystemNode::create_file(
            "config.txt".to_string(),
            "kernel.version=0.1.0\nkernel.arch=x86_64\nmemory.size=512MB\nshell.prompt=reactos>\nfilesystem.type=virtual".to_string()
        );
        
        self.root.add_child(readme);
        self.root.add_child(kernel_log);
        self.root.add_child(config);
        
        // Crear archivos en directorios
        if let Some(system_dir) = self.root.find_child_mut("system") {
            let init_script = FileSystemNode::create_file(
                "init.rc".to_string(),
                "#!/bin/sh\n# Script de inicialización del sistema\necho 'Sistema iniciado'\n".to_string()
            );
            system_dir.add_child(init_script);
        }
        
        if let Some(logs_dir) = self.root.find_child_mut("logs") {
            let boot_log = FileSystemNode::create_file(
                "boot.log".to_string(),
                "Boot iniciado\nGRUB cargado\nKernel iniciado\nDrivers cargados\nSistema listo".to_string()
            );
            logs_dir.add_child(boot_log);
        }
    }

    /// Obtener nodo por ruta
    pub fn get_node(&self, path: &str) -> Option<&FileSystemNode> {
        if path == "/" {
            return Some(&self.root);
        }
        
        let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let mut current = &self.root;
        
        for part in parts {
            if part.is_empty() {
                continue;
            }
            
            if let Some(child) = current.find_child(part) {
                current = child;
            } else {
                return None;
            }
        }
        
        Some(current)
    }

    /// Obtener nodo por ruta (mutable)
    pub fn get_node_mut(&mut self, path: &str) -> Option<&mut FileSystemNode> {
        if path == "/" {
            return Some(&mut self.root);
        }
        
        let parts: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let mut current = &mut self.root;
        
        for part in parts {
            if part.is_empty() {
                continue;
            }
            
            if let Some(child) = current.find_child_mut(part) {
                current = child;
            } else {
                return None;
            }
        }
        
        Some(current)
    }

    /// Crear un archivo
    pub fn create_file(&mut self, path: &str, content: String) -> bool {
        self.time_counter += 1;
        
        if let Some((parent_path, filename)) = self.split_path(path) {
            if let Some(parent) = self.get_node_mut(parent_path) {
                let file = FileSystemNode::create_file(filename.to_string(), content);
                file.created_time = self.time_counter;
                file.modified_time = self.time_counter;
                return parent.add_child(file);
            }
        }
        
        false
    }

    /// Crear un directorio
    pub fn create_directory(&mut self, path: &str) -> bool {
        self.time_counter += 1;
        
        if let Some((parent_path, dirname)) = self.split_path(path) {
            if let Some(parent) = self.get_node_mut(parent_path) {
                let mut dir = FileSystemNode::create_directory(dirname.to_string());
                dir.created_time = self.time_counter;
                dir.modified_time = self.time_counter;
                return parent.add_child(dir);
            }
        }
        
        false
    }

    /// Eliminar un archivo o directorio
    pub fn remove(&mut self, path: &str) -> bool {
        if let Some((parent_path, name)) = self.split_path(path) {
            if let Some(parent) = self.get_node_mut(parent_path) {
                return parent.remove_child(name);
            }
        }
        
        false
    }

    /// Leer contenido de un archivo
    pub fn read_file(&self, path: &str) -> Option<String> {
        if let Some(node) = self.get_node(path) {
            if node.node_type == NodeType::File {
                return Some(node.content.clone());
            }
        }
        
        None
    }

    /// Escribir contenido a un archivo
    pub fn write_file(&mut self, path: &str, content: String) -> bool {
        self.time_counter += 1;
        
        if let Some(node) = self.get_node_mut(path) {
            if node.node_type == NodeType::File {
                node.content = content;
                node.size = node.content.len();
                node.modified_time = self.time_counter;
                return true;
            }
        }
        
        false
    }

    /// Listar contenido de un directorio
    pub fn list_directory(&self, path: &str) -> Option<Vec<String>> {
        if let Some(node) = self.get_node(path) {
            if node.node_type == NodeType::Directory {
                let mut result = Vec::new();
                for child in &node.children {
                    result.push(child.get_info());
                }
                return Some(result);
            }
        }
        
        None
    }

    /// Dividir ruta en directorio padre y nombre
    fn split_path(&self, path: &str) -> Option<(String, String)> {
        if let Some(last_slash) = path.rfind('/') {
            let parent = if last_slash == 0 {
                "/".to_string()
            } else {
                path[..last_slash].to_string()
            };
            let name = path[last_slash + 1..].to_string();
            Some((parent, name))
        } else {
            None
        }
    }

    /// Obtener información del sistema de archivos
    pub fn get_info(&self) -> String {
        format!(
            "Sistema de archivos: Virtual, {} nodos, tiempo: {}",
            self.count_nodes(&self.root),
            self.time_counter
        )
    }

    /// Contar nodos recursivamente
    fn count_nodes(&self, node: &FileSystemNode) -> usize {
        let mut count = 1;
        for child in &node.children {
            count += self.count_nodes(child);
        }
        count
    }
}

/// Instancia global del sistema de archivos
static FILESYSTEM: Mutex<Option<FileSystem>> = Mutex::new(None);

/// Inicializar el sistema de archivos
pub fn init_filesystem() -> bool {
    let mut fs_guard = FILESYSTEM.lock();
    *fs_guard = Some(FileSystem::new());
    true
}

/// Obtener el sistema de archivos
pub fn get_filesystem() -> Option<&'static Mutex<Option<FileSystem>>> {
    Some(&FILESYSTEM)
}

/// Crear un archivo
pub fn create_file(path: &str, content: String) -> bool {
    let mut fs_guard = FILESYSTEM.lock();
    if let Some(ref mut fs) = *fs_guard {
        fs.create_file(path, content)
    } else {
        false
    }
}

/// Crear un directorio
pub fn create_directory(path: &str) -> bool {
    let mut fs_guard = FILESYSTEM.lock();
    if let Some(ref mut fs) = *fs_guard {
        fs.create_directory(path)
    } else {
        false
    }
}

/// Leer un archivo
pub fn read_file(path: &str) -> Option<String> {
    let fs_guard = FILESYSTEM.lock();
    if let Some(ref fs) = *fs_guard {
        fs.read_file(path)
    } else {
        None
    }
}

/// Escribir a un archivo
pub fn write_file(path: &str, content: String) -> bool {
    let mut fs_guard = FILESYSTEM.lock();
    if let Some(ref mut fs) = *fs_guard {
        fs.write_file(path, content)
    } else {
        false
    }
}

/// Listar directorio
pub fn list_directory(path: &str) -> Option<Vec<String>> {
    let fs_guard = FILESYSTEM.lock();
    if let Some(ref fs) = *fs_guard {
        fs.list_directory(path)
    } else {
        None
    }
}

/// Eliminar archivo o directorio
pub fn remove(path: &str) -> bool {
    let mut fs_guard = FILESYSTEM.lock();
    if let Some(ref mut fs) = *fs_guard {
        fs.remove(path)
    } else {
        false
    }
}

/// Obtener información del sistema de archivos
pub fn get_filesystem_info() -> String {
    let fs_guard = FILESYSTEM.lock();
    if let Some(ref fs) = *fs_guard {
        fs.get_info()
    } else {
        String::from("Sistema de archivos: No disponible")
    }
}

/// Verificar si el sistema de archivos está disponible
pub fn is_filesystem_available() -> bool {
    let fs_guard = FILESYSTEM.lock();
    fs_guard.is_some()
}

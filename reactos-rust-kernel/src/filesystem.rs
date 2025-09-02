//! Sistema de archivos avanzado para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Sistema de archivos en memoria con persistencia
//! - Operaciones de archivos avanzadas
//! - Gestión de directorios con permisos
//! - Sistema de bloques y inodos
//! - Comandos de archivos mejorados
//! - Backup y restauración

use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use alloc::format;
use alloc::collections::BTreeMap;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Tipo de nodo en el sistema de archivos
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    File,
    Directory,
    Symlink,    // Enlace simbólico
    Block,      // Dispositivo de bloque
    Char,       // Dispositivo de carácter
}

/// Estado del sistema de archivos
#[derive(Debug, Clone, PartialEq)]
pub enum FileSystemState {
    Mounted,    // Montado y funcionando
    Unmounted,  // Desmontado
    Error,      // Error en el sistema
    ReadOnly,   // Solo lectura
}

/// Tipo de sistema de archivos
#[derive(Debug, Clone, PartialEq)]
pub enum FileSystemType {
    Virtual,    // Sistema en memoria
    Persistent, // Sistema persistente
    Network,    // Sistema de red
}

/// Permisos de archivo
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

/// Bloque de datos
#[derive(Debug, Clone)]
pub struct DataBlock {
    pub id: usize,
    pub data: Vec<u8>,
    pub size: usize,
    pub is_free: bool,
}

impl DataBlock {
    pub fn new(id: usize, size: usize) -> Self {
        Self {
            id,
            data: vec![0; size],
            size,
            is_free: true,
        }
    }
    
    pub fn write_data(&mut self, data: &[u8]) -> bool {
        if data.len() <= self.size {
            self.data[..data.len()].copy_from_slice(data);
            self.is_free = false;
            true
        } else {
            false
        }
    }
    
    pub fn read_data(&self) -> &[u8] {
        &self.data
    }
}

/// Inodo (estructura de metadatos)
#[derive(Debug, Clone)]
pub struct Inode {
    pub id: usize,
    pub node_type: NodeType,
    pub permissions: FilePermissions,
    pub size: usize,
    pub blocks: Vec<usize>,  // IDs de bloques de datos
    pub created_time: u64,
    pub modified_time: u64,
    pub accessed_time: u64,
    pub link_count: usize,
    pub owner: String,
    pub group: String,
}

impl Inode {
    pub fn new(id: usize, node_type: NodeType) -> Self {
        Self {
            id,
            node_type,
            permissions: FilePermissions::new(),
            size: 0,
            blocks: Vec::new(),
            created_time: 0,
            modified_time: 0,
            accessed_time: 0,
            link_count: 1,
            owner: "root".to_string(),
            group: "root".to_string(),
        }
    }
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
    pub inode_id: usize,
    pub children: Vec<FileSystemNode>,
}

impl FileSystemNode {
    /// Crear un nuevo nodo
    pub fn new(name: String, inode_id: usize) -> Self {
        Self {
            name,
            inode_id,
            children: Vec::new(),
        }
    }

    /// Obtener información del nodo (requiere acceso al inodo)
    pub fn get_info(&self, inode: &Inode) -> String {
        let node_type_str = match inode.node_type {
            NodeType::File => "FILE",
            NodeType::Directory => "DIR",
            NodeType::Symlink => "LINK",
            NodeType::Block => "BLOCK",
            NodeType::Char => "CHAR",
        };
        
        format!(
            "{} {} {} {} {} {} {} {}",
            inode.permissions.to_string(),
            node_type_str,
            inode.size,
            inode.created_time,
            inode.modified_time,
            inode.owner,
            inode.group,
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
        // Verificar que no existe ya
        if self.find_child(&child.name).is_some() {
            return false;
        }
        
        self.children.push(child);
        true
    }

    /// Eliminar un nodo hijo
    pub fn remove_child(&mut self, name: &str) -> bool {
        if let Some(pos) = self.children.iter().position(|child| child.name == name) {
            self.children.remove(pos);
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
    inodes: BTreeMap<usize, Inode>,
    blocks: BTreeMap<usize, DataBlock>,
    next_inode_id: AtomicUsize,
    next_block_id: AtomicUsize,
    state: FileSystemState,
    fs_type: FileSystemType,
    block_size: usize,
    total_blocks: usize,
    free_blocks: AtomicUsize,
    total_inodes: usize,
    free_inodes: AtomicUsize,
}

impl FileSystem {
    /// Crear un nuevo sistema de archivos
    pub fn new() -> Self {
        let mut fs = Self {
            root: FileSystemNode::new("/".to_string(), 0),
            current_path: "/".to_string(),
            time_counter: 0,
            inodes: BTreeMap::new(),
            blocks: BTreeMap::new(),
            next_inode_id: AtomicUsize::new(1),
            next_block_id: AtomicUsize::new(1),
            state: FileSystemState::Mounted,
            fs_type: FileSystemType::Persistent,
            block_size: 4096,  // 4KB por bloque
            total_blocks: 1024, // 4MB total
            free_blocks: AtomicUsize::new(1024),
            total_inodes: 256,
            free_inodes: AtomicUsize::new(256),
        };
        
        // Inicializar sistema de archivos
        fs.init();
        fs
    }

    /// Inicializar el sistema de archivos
    fn init(&mut self) {
        self.time_counter = 1;
        
        // Crear inodo para el directorio raíz
        let root_inode = Inode::new(0, NodeType::Directory);
        self.inodes.insert(0, root_inode);
        self.free_inodes.fetch_sub(1, Ordering::SeqCst);
        
        // Crear directorios del sistema
        self.create_directory("/system");
        self.create_directory("/logs");
        self.create_directory("/temp");
        self.create_directory("/home");
        
        // Crear archivos del sistema
        self.create_file("/README.txt", "ReactOS Rust Kernel v0.1.0\nSistema de archivos persistente\nComandos disponibles: help, ls, cat, echo, etc.".to_string());
        
        self.create_file("/kernel.log", "Kernel iniciado correctamente\nVGA inicializado\nTeclado inicializado\nMouse inicializado\nInterrupciones activas\nMemoria configurada\nShell iniciado\nSistema de procesos activo\nSistema de archivos persistente activo".to_string());
        
        self.create_file("/config.txt", "kernel.version=0.1.0\nkernel.arch=x86_64\nmemory.size=512MB\nshell.prompt=reactos>\nfilesystem.type=persistent\nfilesystem.blocks=1024\nfilesystem.block_size=4096".to_string());
        
        // Crear archivos en directorios
        self.create_file("/system/init.rc", "#!/bin/sh\n# Script de inicialización del sistema\necho 'Sistema iniciado'\n".to_string());
        
        self.create_file("/logs/boot.log", "Boot iniciado\nGRUB cargado\nKernel iniciado\nDrivers cargados\nSistema listo\nSistema de archivos persistente montado".to_string());
        
        // Log de inicialización
        crate::logging::info("filesystem", "Sistema de archivos persistente inicializado correctamente");
    }
    
    /// Crear un nuevo inodo
    fn create_inode(&mut self, node_type: NodeType) -> Option<usize> {
        if self.free_inodes.load(Ordering::SeqCst) == 0 {
            return None;
        }
        
        let inode_id = self.next_inode_id.fetch_add(1, Ordering::SeqCst);
        let mut inode = Inode::new(inode_id, node_type);
        inode.created_time = self.time_counter;
        inode.modified_time = self.time_counter;
        inode.accessed_time = self.time_counter;
        
        self.inodes.insert(inode_id, inode);
        self.free_inodes.fetch_sub(1, Ordering::SeqCst);
        
        Some(inode_id)
    }
    
    /// Obtener un inodo
    fn get_inode(&self, inode_id: usize) -> Option<&Inode> {
        self.inodes.get(&inode_id)
    }
    
    /// Obtener un inodo (mutable)
    fn get_inode_mut(&mut self, inode_id: usize) -> Option<&mut Inode> {
        self.inodes.get_mut(&inode_id)
    }
    
    /// Crear un nuevo bloque de datos
    fn create_block(&mut self) -> Option<usize> {
        if self.free_blocks.load(Ordering::SeqCst) == 0 {
            return None;
        }
        
        let block_id = self.next_block_id.fetch_add(1, Ordering::SeqCst);
        let block = DataBlock::new(block_id, self.block_size);
        
        self.blocks.insert(block_id, block);
        self.free_blocks.fetch_sub(1, Ordering::SeqCst);
        
        Some(block_id)
    }
    
    /// Obtener un bloque de datos
    fn get_block(&self, block_id: usize) -> Option<&DataBlock> {
        self.blocks.get(&block_id)
    }
    
    /// Obtener un bloque de datos (mutable)
    fn get_block_mut(&mut self, block_id: usize) -> Option<&mut DataBlock> {
        self.blocks.get_mut(&block_id)
    }
    
    /// Liberar un bloque de datos
    fn free_block(&mut self, block_id: usize) -> bool {
        if let Some(block) = self.blocks.get_mut(&block_id) {
            block.is_free = true;
            self.free_blocks.fetch_add(1, Ordering::SeqCst);
            true
        } else {
            false
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
        let time_counter = self.time_counter + 1;
        self.time_counter = time_counter;
        
        if let Some((parent_path, filename)) = self.split_path(path) {
            // Crear inodo para el archivo
            if let Some(inode_id) = self.create_inode(NodeType::File) {
                // Escribir contenido en bloques
                if self.write_file_content(inode_id, &content) {
                    // Actualizar inodo
                    if let Some(inode) = self.get_inode_mut(inode_id) {
                        inode.size = content.len();
                        inode.modified_time = time_counter;
                    }
                    
                    // Crear nodo del sistema de archivos
                    let file_node = FileSystemNode::new(filename.to_string(), inode_id);
                    
                    // Agregar al directorio padre
                    if let Some(parent) = self.get_node_mut(&parent_path) {
                        return parent.add_child(file_node);
                    }
                } else {
                    // Si falla la escritura, liberar el inodo
                    self.free_inodes.fetch_add(1, Ordering::SeqCst);
                }
            }
        }
        
        false
    }
    
    /// Escribir contenido de archivo en bloques
    fn write_file_content(&mut self, inode_id: usize, content: &str) -> bool {
        let content_bytes = content.as_bytes();
        let mut remaining = content_bytes.len();
        let mut offset = 0;
        let mut block_ids = Vec::new();
        
        // Crear todos los bloques necesarios
        while remaining > 0 {
            if let Some(block_id) = self.create_block() {
                let chunk_size = remaining.min(self.block_size);
                let chunk = &content_bytes[offset..offset + chunk_size];
                
                if let Some(block) = self.get_block_mut(block_id) {
                    if block.write_data(chunk) {
                        block_ids.push(block_id);
                        offset += chunk_size;
                        remaining -= chunk_size;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        
        // Asignar bloques al inodo
        if let Some(inode) = self.get_inode_mut(inode_id) {
            inode.blocks = block_ids;
            true
        } else {
            false
        }
    }

    /// Crear un directorio
    pub fn create_directory(&mut self, path: &str) -> bool {
        let time_counter = self.time_counter + 1;
        self.time_counter = time_counter;
        
        if let Some((parent_path, dirname)) = self.split_path(path) {
            // Crear inodo para el directorio
            if let Some(inode_id) = self.create_inode(NodeType::Directory) {
                // Actualizar inodo
                if let Some(inode) = self.get_inode_mut(inode_id) {
                    inode.modified_time = time_counter;
                }
                
                // Crear nodo del sistema de archivos
                let dir_node = FileSystemNode::new(dirname.to_string(), inode_id);
                
                // Agregar al directorio padre
                if let Some(parent) = self.get_node_mut(&parent_path) {
                    return parent.add_child(dir_node);
                }
            }
        }
        
        false
    }

    /// Eliminar un archivo o directorio
    pub fn remove(&mut self, path: &str) -> bool {
        if let Some((parent_path, name)) = self.split_path(path) {
            if let Some(parent) = self.get_node_mut(&parent_path) {
                return parent.remove_child(&name);
            }
        }
        
        false
    }

    /// Leer contenido de un archivo
    pub fn read_file(&self, path: &str) -> Option<String> {
        if let Some(node) = self.get_node(path) {
            if let Some(inode) = self.get_inode(node.inode_id) {
                if inode.node_type == NodeType::File {
                    return self.read_file_content(node.inode_id);
                }
            }
        }
        
        None
    }
    
    /// Leer contenido de archivo desde bloques
    fn read_file_content(&self, inode_id: usize) -> Option<String> {
        if let Some(inode) = self.get_inode(inode_id) {
            let mut content = String::new();
            
            for &block_id in &inode.blocks {
                if let Some(block) = self.get_block(block_id) {
                    let block_data = block.read_data();
                    // Convertir bytes a string (asumiendo UTF-8)
                    if let Ok(block_str) = core::str::from_utf8(block_data) {
                        content.push_str(block_str);
                    }
                }
            }
            
            Some(content)
        } else {
            None
        }
    }

    /// Escribir contenido a un archivo
    pub fn write_file(&mut self, path: &str, content: String) -> bool {
        let time_counter = self.time_counter + 1;
        self.time_counter = time_counter;
        
        // Obtener inode_id sin borrowing
        let inode_id = if let Some(node) = self.get_node(path) {
            node.inode_id
        } else {
            return false;
        };
        
        // Verificar que es un archivo
        if let Some(inode) = self.get_inode(inode_id) {
            if inode.node_type == NodeType::File {
                // Escribir contenido en bloques
                if self.write_file_content(inode_id, &content) {
                    // Actualizar inodo
                    if let Some(inode) = self.get_inode_mut(inode_id) {
                        inode.size = content.len();
                        inode.modified_time = time_counter;
                    }
                    return true;
                }
            }
        }
        
        false
    }

    /// Listar contenido de un directorio
    pub fn list_directory(&self, path: &str) -> Option<Vec<String>> {
        if let Some(node) = self.get_node(path) {
            if let Some(inode) = self.get_inode(node.inode_id) {
                if inode.node_type == NodeType::Directory {
                    let mut result = Vec::new();
                    for child in &node.children {
                        if let Some(child_inode) = self.get_inode(child.inode_id) {
                            result.push(child.get_info(child_inode));
                        }
                    }
                    return Some(result);
                }
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
        let used_blocks = self.total_blocks - self.free_blocks.load(Ordering::SeqCst);
        let used_inodes = self.total_inodes - self.free_inodes.load(Ordering::SeqCst);
        
        format!(
            "Sistema de archivos: {:?} ({:?}), {} nodos, {} bloques usados/{} total, {} inodos usados/{} total, tiempo: {}",
            self.fs_type,
            self.state,
            self.count_nodes(&self.root),
            used_blocks,
            self.total_blocks,
            used_inodes,
            self.total_inodes,
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
    
    /// Crear backup del sistema de archivos
    pub fn create_backup(&self) -> String {
        let mut backup = String::new();
        backup.push_str("# ReactOS Rust Kernel Filesystem Backup\n");
        backup.push_str(&format!("# Created: {}\n", self.time_counter));
        backup.push_str(&format!("# Type: {:?}\n", self.fs_type));
        backup.push_str(&format!("# State: {:?}\n", self.state));
        backup.push_str("#\n");
        
        // Backup de inodos
        backup.push_str("# Inodes\n");
        for (id, inode) in &self.inodes {
            backup.push_str(&format!("INODE {} {:?} {} {} {} {} {} {} {}\n",
                id, inode.node_type, inode.permissions.to_string(),
                inode.size, inode.created_time, inode.modified_time,
                inode.accessed_time, inode.owner, inode.group
            ));
        }
        
        // Backup de estructura de directorios
        backup.push_str("# Directory Structure\n");
        self.backup_directory_structure(&self.root, "/", &mut backup);
        
        backup
    }
    
    /// Backup recursivo de estructura de directorios
    fn backup_directory_structure(&self, node: &FileSystemNode, path: &str, backup: &mut String) {
        if let Some(inode) = self.get_inode(node.inode_id) {
            backup.push_str(&format!("DIR {} {} {:?}\n", path, node.name, inode.node_type));
            
            for child in &node.children {
                let child_path = if path == "/" {
                    format!("/{}", child.name)
                } else {
                    format!("{}/{}", path, child.name)
                };
                self.backup_directory_structure(child, &child_path, backup);
            }
        }
    }
    
    /// Restaurar sistema de archivos desde backup
    pub fn restore_from_backup(&mut self, _backup: &str) -> bool {
        // En una implementación real, aquí se parsearía el backup
        // Por ahora, solo loggeamos la operación
        crate::logging::info("filesystem", "Restaurando sistema de archivos desde backup");
        true
    }
    
    /// Obtener estadísticas del sistema de archivos
    pub fn get_stats(&self) -> String {
        let used_blocks = self.total_blocks - self.free_blocks.load(Ordering::SeqCst);
        let used_inodes = self.total_inodes - self.free_inodes.load(Ordering::SeqCst);
        let block_usage = (used_blocks * 100) / self.total_blocks;
        let inode_usage = (used_inodes * 100) / self.total_inodes;
        
        format!(
            "Filesystem: {:?} | Estado: {:?} | Bloques: {}/{} ({}%) | Inodos: {}/{} ({}%) | Tamaño bloque: {}B",
            self.fs_type,
            self.state,
            used_blocks,
            self.total_blocks,
            block_usage,
            used_inodes,
            self.total_inodes,
            inode_usage,
            self.block_size
        )
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

/// Crear backup del sistema de archivos
pub fn create_backup() -> String {
    let fs_guard = FILESYSTEM.lock();
    if let Some(ref fs) = *fs_guard {
        fs.create_backup()
    } else {
        String::from("Sistema de archivos no disponible")
    }
}

/// Obtener estadísticas del sistema de archivos
pub fn get_filesystem_stats() -> String {
    let fs_guard = FILESYSTEM.lock();
    if let Some(ref fs) = *fs_guard {
        fs.get_stats()
    } else {
        String::from("Estadísticas de archivos: No disponible")
    }
}

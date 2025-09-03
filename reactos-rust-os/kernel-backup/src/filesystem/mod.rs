//! Sistema de Archivos para ReactOS Rust Kernel
//! 
//! Este módulo contiene el sistema de archivos virtual (VFS) y los drivers
//! para diferentes sistemas de archivos como FAT32 y NTFS.

pub mod vfs;
pub mod fat32;
pub mod ntfs;

/// Inicializar el sistema de archivos
pub fn init() {
    // Inicializar VFS
    vfs::init_vfs();
    
    // Inicializar drivers de sistemas de archivos
    // TODO: Implementar inicialización de drivers
}

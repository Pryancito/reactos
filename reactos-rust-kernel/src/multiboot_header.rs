//! Multiboot Header y estructuras para ReactOS Rust Kernel
//! 
//! Header Multiboot 1 requerido por GRUB con soporte completo para módulos

use core::mem::size_of;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

#[repr(C, packed)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u32; 4],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
}

#[repr(C, packed)]
pub struct MultibootModule {
    pub mod_start: u32,
    pub mod_end: u32,
    pub string: u32,
    pub reserved: u32,
}

const MULTIBOOT_HEADER_MAGIC: u32 = 0x1BADB002;
const MULTIBOOT_HEADER_FLAGS: u32 = 0x00000003; // Align modules and provide memory info
const MULTIBOOT_HEADER_CHECKSUM: u32 = 0xE4524FFB; // Calculated checksum

#[used]
#[no_mangle]
#[link_section = ".multiboot"]
pub static MULTIBOOT_HEADER: [u32; 3] = [
    MULTIBOOT_HEADER_MAGIC,
    MULTIBOOT_HEADER_FLAGS,
    MULTIBOOT_HEADER_CHECKSUM,
];

// Función para obtener información multiboot
pub fn get_multiboot_info() -> *const u8 {
    0 as *const u8 // Placeholder
}

// Función para procesar módulos Multiboot
pub fn process_multiboot_modules(mb_info: *const MultibootInfo) {
    unsafe {
        if mb_info.is_null() {
            return;
        }
        
        let flags = core::ptr::read_unaligned(mb_info.cast::<u32>().add(0));
        let mods_count = core::ptr::read_unaligned(mb_info.cast::<u32>().add(6));
        let mods_addr = core::ptr::read_unaligned(mb_info.cast::<u32>().add(7));
        
        if (flags & (1 << 3)) == 0 {
            return; // No hay módulos
        }
        
        let modules = core::slice::from_raw_parts(
            mods_addr as *const MultibootModule,
            mods_count as usize
        );
        
        for (_i, module) in modules.iter().enumerate() {
            let module_ptr = module as *const MultibootModule as *const u32;
            let mod_start = core::ptr::read_unaligned(module_ptr.add(0));
            let mod_end = core::ptr::read_unaligned(module_ptr.add(1));
            let string = core::ptr::read_unaligned(module_ptr.add(2));
            
            let module_data = core::slice::from_raw_parts(
                mod_start as *const u8,
                (mod_end - mod_start) as usize
            );
            
            let module_name = if string != 0 {
                core::str::from_utf8_unchecked(
                    core::slice::from_raw_parts(
                        string as *const u8,
                        256 // Máximo 256 caracteres
                    )
                ).trim_end_matches('\0')
            } else {
                "unknown"
            };
            
            // Llamar al PE loader para procesar el módulo
            crate::pe_loader::load_pe64_module(module_data, module_name);
        }
    }
}

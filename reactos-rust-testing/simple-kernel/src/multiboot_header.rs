//! Multiboot Header para kernel simple

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

const MULTIBOOT_HEADER_MAGIC: u32 = 0x1BADB002;
const MULTIBOOT_HEADER_FLAGS: u32 = 0x00000003; // Align modules and provide memory info
const MULTIBOOT_HEADER_CHECKSUM: u32 = 0xE4524FFB; // Calculated checksum

#[used]
#[no_mangle]
#[link_section = ".multiboot_header"]
pub static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: MULTIBOOT_HEADER_MAGIC,
    flags: MULTIBOOT_HEADER_FLAGS,
    checksum: MULTIBOOT_HEADER_CHECKSUM,
};

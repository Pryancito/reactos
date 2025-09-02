/* Linker script para el kernel Rust con bootloader */

ENTRY(_start)

SECTIONS
{
    /* El bootloader va al inicio */
    . = 0x100000;
    
    .multiboot : {
        *(.multiboot)
    }
    
    .text : {
        *(.text)
    }
    
    .rodata : {
        *(.rodata*)
    }
    
    .data : {
        *(.data)
    }
    
    .bss : {
        *(COMMON)
        *(.bss)
    }
    
    /DISCARD/ : {
        *(.comment)
        *(.note*)
    }
}

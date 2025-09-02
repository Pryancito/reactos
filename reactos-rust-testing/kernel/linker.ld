/* Linker script para ReactOS Rust Kernel */

ENTRY(_start)

SECTIONS
{
    /* El kernel se carga en 1MB */
    . = 0x100000;
    
    /* Sección de código */
    .text : {
        *(.text .text.*)
    }
    
    /* Sección de datos de solo lectura */
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    /* Sección de datos */
    .data : {
        *(.data .data.*)
    }
    
    /* Sección BSS (datos no inicializados) */
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }
    
    /* Alinear al final */
    . = ALIGN(4096);
    
    /* Marcar el final del kernel */
    _kernel_end = .;
}

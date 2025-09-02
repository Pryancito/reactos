/* Linker script para ReactOS Rust Kernel con Multiboot */

ENTRY(_start)

SECTIONS
{
    /* El header multiboot debe estar al inicio */
    .multiboot : {
        *(.multiboot)
    }
    
    /* Código del kernel */
    .text : {
        *(.text)
    }
    
    /* Datos de solo lectura */
    .rodata : {
        *(.rodata*)
    }
    
    /* Datos inicializados */
    .data : {
        *(.data)
    }
    
    /* Datos no inicializados */
    .bss : {
        *(.bss)
    }
    
    /* Stack del kernel */
    . = ALIGN(16);
    . += 0x1000; /* 4KB de stack */
    _stack_top = .;
}
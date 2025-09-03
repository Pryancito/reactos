/* Script de enlazado para Eclipse OS Kernel */
ENTRY(_start)

SECTIONS
{
    /* Colocar la sección .multiboot al inicio */
    . = 0x100000;
    
    .multiboot : {
        *(.multiboot)
    }
    
    .text : {
        *(.text)
    }
    
    .rodata : {
        *(.rodata)
    }
    
    .data : {
        *(.data)
    }
    
    .bss : {
        *(.bss)
    }
}

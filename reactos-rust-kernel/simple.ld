/* Linker script simple para ReactOS Rust Kernel */

ENTRY(_start)

SECTIONS
{
    /* El header multiboot debe estar al inicio */
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
        *(.bss)
    }
    
    /* Stack del kernel */
    . = ALIGN(16);
    . += 0x1000;
    _stack_top = .;
}

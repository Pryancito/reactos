ENTRY(_start)

SECTIONS {
    /* Colocar el header multiboot al inicio absoluto */
    . = 0x100000;
    
    .multiboot_header : {
        *(.multiboot_header)
    }

    .text : ALIGN(4K) {
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    .bss : ALIGN(4K) {
        *(.bss .bss.*)
    }

    .align_bottom : ALIGN(4K) {
        /* Ensure the kernel ends on a page boundary */
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.gcc_except_table)
    }
}

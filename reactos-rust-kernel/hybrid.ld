/* Linker script para kernel híbrido: Assembly bootloader + Rust kernel */

ENTRY(_start)

SECTIONS
{
    /* Forzar .multiboot al inicio del fichero (offset 0) */
    . = 0x00100000;

    /* Colocar .multiboot como primer output section y primer LOAD */
    .multiboot 0x00100000 : AT(0) {
        KEEP(*(.multiboot))
    }

    /* Hacer que .text siga inmediatamente y empiece también en memoria 1MiB */
    .text 0x00100000 + SIZEOF(.multiboot) : AT(SIZEOF(.multiboot)) {
        *(.text)
        _load_end = .;
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
        _bss_end = .;
    }

    /* Stack del kernel */
    . = ALIGN(16);
    . += 0x1000;
    _stack_top = .;
}

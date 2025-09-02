; Bootloader para el kernel Rust
; Este bootloader carga el kernel Rust y lo ejecuta

BITS 32

; Multiboot header
MULTIBOOT_HEADER_MAGIC equ 0x1BADB002
MULTIBOOT_HEADER_FLAGS equ 0x00000003
MULTIBOOT_HEADER_CHECKSUM equ -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

section .multiboot
align 4
    dd MULTIBOOT_HEADER_MAGIC
    dd MULTIBOOT_HEADER_FLAGS
    dd MULTIBOOT_HEADER_CHECKSUM

section .text
global _start
extern kernel_main

_start:
    ; Configurar stack
    mov esp, stack_top
    
    ; Limpiar flags
    push 0
    popf
    
    ; Llamar al kernel Rust
    call kernel_main
    
    ; Si el kernel retorna, colgar el sistema
hang:
    hlt
    jmp hang

section .bss
align 16
stack_bottom:
    resb 16384  ; 16KB de stack
stack_top:

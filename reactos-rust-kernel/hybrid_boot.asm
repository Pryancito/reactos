; hybrid_boot.asm
; Bootloader híbrido: Assembly + Rust Kernel
; Este archivo contiene el header multiboot y salta al kernel Rust

BITS 64

; Header multiboot al inicio
section .multiboot
    align 4
    dd 0x1BADB002            ; magic
    dd 0x00000003            ; flags (align modules + memory info)
    dd -(0x1BADB002 + 0x00000003) ; checksum

; Stack del kernel
section .bss
    align 16
    stack_bottom:
    resb 16384               ; 16KB de stack
    stack_top:

; Código del bootloader
section .text
    global _start
    extern kernel_main       ; Función del kernel Rust

_start:
    ; Configurar stack
    mov esp, stack_top
    
    ; Configurar modo de video básico
    mov ax, 0x03             ; Modo texto 80x25
    int 0x10
    
    ; Limpiar pantalla
    mov ah, 0x06             ; Scroll up
    mov al, 0x00             ; Clear entire screen
    mov bh, 0x07             ; White on black
    mov cx, 0x0000           ; Top-left corner
    mov dx, 0x184f           ; Bottom-right corner
    int 0x10
    
    ; Posicionar cursor
    mov ah, 0x02             ; Set cursor position
    mov bh, 0x00             ; Page 0
    mov dx, 0x0000           ; Row 0, column 0
    int 0x10
    
    ; Imprimir mensaje de bootloader
    mov esi, boot_msg
    call print_string
    
    ; Saltar al kernel Rust
    call kernel_main
    
    ; Si el kernel retorna, colgar el sistema
hang:
    hlt
    jmp hang

; Función para imprimir string
print_string:
    pusha
    mov ah, 0x0e             ; Función teletipo
print_loop:
    lodsb                    ; Cargar byte de [esi] en al
    test al, al              ; ¿Es null?
    jz print_done            ; Si es null, terminar
    int 0x10                 ; Imprimir carácter
    jmp print_loop           ; Continuar
print_done:
    popa
    ret

; Mensaje del bootloader
boot_msg db '🚀 ReactOS Rust Bootloader iniciado...', 0x0A, 0x0D, 0

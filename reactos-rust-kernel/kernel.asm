; Kernel simple para ReactOS Rust con multiboot
; Ensamblador para crear un kernel que funcione

section .multiboot
align 4
    dd 0x1BADB002    ; magic
    dd 0x00000000    ; flags
    dd 0xE4524FFB    ; checksum

section .text
global _start
_start:
    ; Configurar stack
    mov esp, stack_top
    
    ; Limpiar pantalla VGA
    mov edi, 0xB8000
    mov ecx, 80 * 25
    mov ax, 0x0F20    ; Blanco sobre negro, espacio
    rep stosw
    
    ; Mostrar mensaje
    mov esi, message
    mov edi, 0xB8000
    mov ah, 0x0F      ; Blanco sobre negro
    
print_loop:
    lodsb
    test al, al
    jz halt_loop
    stosw
    jmp print_loop

halt_loop:
    hlt
    jmp halt_loop

section .data
message db 'ReactOS Rust Kernel v1.0 - AI en Tiempo Real!', 0

section .bss
align 16
stack_bottom:
    resb 16384
stack_top:

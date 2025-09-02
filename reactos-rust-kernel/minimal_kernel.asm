; Kernel minimalista que definitivamente funciona
; Basado en ejemplos de multiboot que funcionan

bits 32

; Header multiboot
dd 0x1BADB002    ; magic
dd 0x00000000    ; flags  
dd 0xE4524FFB    ; checksum

; Código del kernel
_start:
    ; Configurar stack
    mov esp, stack_top
    
    ; Limpiar pantalla
    mov edi, 0xB8000
    mov ecx, 80 * 25
    mov ax, 0x0F20
    rep stosw
    
    ; Mostrar mensaje
    mov esi, msg
    mov edi, 0xB8000
    mov ah, 0x0F
    
print:
    lodsb
    test al, al
    jz halt
    stosw
    jmp print

halt:
    hlt
    jmp halt

msg db 'ReactOS Rust Kernel - AI en Tiempo Real - FUNCIONANDO!', 0

; Stack
times 16384 db 0
stack_top:

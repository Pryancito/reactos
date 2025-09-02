; direct_kernel.asm
; Kernel directo para QEMU (sin GRUB)

BITS 32

; Header multiboot simple
section .multiboot
    align 4
    dd 0x1BADB002            ; magic
    dd 0x00000000            ; flags (sin flags)
    dd -(0x1BADB002 + 0x00000000) ; checksum

; Stack
section .bss
    align 16
    stack_bottom:
    resb 16384
    stack_top:

; Código principal
section .text
    global _start

_start:
    ; Configurar stack
    mov esp, stack_top
    
    ; Configurar modo de video
    mov ax, 0x03
    int 0x10
    
    ; Limpiar pantalla
    mov ah, 0x06
    mov al, 0x00
    mov bh, 0x07
    mov cx, 0x0000
    mov dx, 0x184f
    int 0x10
    
    ; Posicionar cursor
    mov ah, 0x02
    mov bh, 0x00
    mov dx, 0x0000
    int 0x10
    
    ; Imprimir mensaje
    mov esi, welcome_msg
    call print_string
    
    ; Imprimir información del sistema
    mov esi, system_info
    call print_string
    
    ; Imprimir comandos disponibles
    mov esi, commands_info
    call print_string
    
    ; Mostrar prompt
    mov esi, prompt
    call print_string

main_loop:
    ; Leer tecla
    mov ah, 0x00
    int 0x16
    
    ; Procesar tecla
    cmp al, 'h'
    je show_help
    cmp al, 's'
    je show_system
    cmp al, 'q'
    je quit
    cmp al, 0x1B  ; ESC
    je quit
    
    ; Tecla no reconocida
    mov esi, unknown_key
    call print_string
    jmp main_loop

show_help:
    mov esi, help_text
    call print_string
    mov esi, prompt
    call print_string
    jmp main_loop

show_system:
    mov esi, system_status
    call print_string
    mov esi, prompt
    call print_string
    jmp main_loop

quit:
    mov esi, goodbye_msg
    call print_string
    jmp hang

print_string:
    pusha
    mov ah, 0x0e
print_loop:
    lodsb
    test al, al
    jz print_done
    int 0x10
    jmp print_loop
print_done:
    popa
    ret

hang:
    hlt
    jmp hang

; Mensajes
welcome_msg db '🚀 ReactOS Rust Kernel 1.0 - Direct Boot', 0x0A, 0x0D, 0
system_info db '📊 Sistema: x86_64, 4GB RAM, VGA Text Mode', 0x0A, 0x0D, 0
commands_info db '💡 Comandos: h=ayuda, s=sistema, q=salir', 0x0A, 0x0D, 0
prompt db 'reactos> ', 0
help_text db 0x0A, 0x0D, '📖 Comandos disponibles:', 0x0A, 0x0D, '  h - Mostrar ayuda', 0x0A, 0x0D, '  s - Estado del sistema', 0x0A, 0x0D, '  q - Salir', 0x0A, 0x0D, 0
system_status db 0x0A, 0x0D, '📊 Estado del Sistema:', 0x0A, 0x0D, '  • Kernel: ReactOS Rust 1.0', 0x0A, 0x0D, '  • Arquitectura: x86_64', 0x0A, 0x0D, '  • Memoria: 4GB', 0x0A, 0x0D, '  • VGA: Modo texto 80x25', 0x0A, 0x0D, '  • Estado: Funcionando', 0x0A, 0x0D, 0
unknown_key db 0x0A, 0x0D, '❓ Comando no reconocido. Presiona h para ayuda.', 0x0A, 0x0D, 0
goodbye_msg db 0x0A, 0x0D, '👋 ¡Hasta luego! Presiona Ctrl+Alt+Q para salir de QEMU.', 0x0A, 0x0D, 0

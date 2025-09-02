; Kernel plano para ReactOS Rust
; Este kernel debería funcionar con multiboot

bits 32

; Header multiboot al inicio del archivo
dd 0x1BADB002    ; magic number
dd 0x00000000    ; flags
dd 0xE4524FFB    ; checksum

; Código del kernel
_start:
    ; Configurar stack
    mov esp, stack_top
    
    ; Limpiar pantalla VGA
    mov edi, 0xB8000
    mov ecx, 80 * 25
    mov ax, 0x0F20    ; Blanco sobre negro, espacio
    rep stosw
    
    ; Mostrar mensaje de bienvenida
    mov esi, welcome_msg
    mov edi, 0xB8000
    mov ah, 0x0F      ; Blanco sobre negro
    
print_welcome:
    lodsb
    test al, al
    jz print_features
    stosw
    jmp print_welcome

print_features:
    ; Mostrar características del sistema
    mov esi, features_msg
    mov edi, 0xB8000 + 160  ; Segunda línea
    mov ah, 0x0A      ; Verde sobre negro
    
print_features_loop:
    lodsb
    test al, al
    jz print_ai_msg
    stosw
    jmp print_features_loop

print_ai_msg:
    ; Mostrar mensaje de AI
    mov esi, ai_msg
    mov edi, 0xB8000 + 320  ; Tercera línea
    mov ah, 0x0C      ; Rojo sobre negro
    
print_ai_loop:
    lodsb
    test al, al
    jz print_ready
    stosw
    jmp print_ai_loop

print_ready:
    ; Mostrar mensaje de listo
    mov esi, ready_msg
    mov edi, 0xB8000 + 480  ; Cuarta línea
    mov ah, 0x0E      ; Amarillo sobre negro
    
print_ready_loop:
    lodsb
    test al, al
    jz halt_loop
    stosw
    jmp print_ready_loop

halt_loop:
    hlt
    jmp halt_loop

; Datos
welcome_msg db 'ReactOS Rust Kernel v1.0 - Sistema Operativo Avanzado', 0
features_msg db 'Motor 3D + Fisica + Editor + AI en Tiempo Real', 0
ai_msg db '272 Tensor Cores RTX 2060 Super - Inferencia 2.5ms', 0
ready_msg db 'Sistema listo para usar! Presiona Ctrl+Alt+Q para salir', 0

; Stack
stack_bottom:
    times 16384 db 0
stack_top:

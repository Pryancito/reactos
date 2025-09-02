BITS 32
ORG 0x00100000

; Multiboot1 header (a.out kludge) al byte 0
MULTIBOOT_HEADER_MAGIC equ 0x1BADB002
MULTIBOOT_HEADER_FLAGS equ 0x00010003 ; ALIGN | MEMINFO | AOUT_KLUDGE
MULTIBOOT_HEADER_CHECKSUM equ -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

multiboot_header:
    dd MULTIBOOT_HEADER_MAGIC
    dd MULTIBOOT_HEADER_FLAGS
    dd MULTIBOOT_HEADER_CHECKSUM
    dd multiboot_header    ; header_addr
    dd 0x00100000          ; load_addr (nuestro ORG)
    dd _load_end           ; load_end_addr
    dd _bss_end            ; bss_end_addr
    dd _start              ; entry_addr

; Entrada
_start:
    ; Limpiar dirección de video/estado
    mov dword [cursor_x], 0
    mov dword [cursor_y], 0
    ; Imprimir banner
    mov esi, banner
    call vga_print
    ; Prompt
    mov esi, prompt
    call vga_print

main_loop:
    call kbd_read_char
    cmp al, 'h'
    je show_help
    cmp al, 's'
    je show_system
    cmp al, 'q'
    je quit
    jmp main_loop

show_help:
    mov esi, help_text
    call vga_print
    mov esi, prompt
    call vga_print
    jmp main_loop

show_system:
    mov esi, system_status
    call vga_print
    mov esi, prompt
    call vga_print
    jmp main_loop

quit:
    mov esi, goodbye_msg
    call vga_print
hang:
    hlt
    jmp hang

; ===== VGA =====
vga_putc:
    pusha
    cmp al, 0x0D
    je .cr
    cmp al, 0x0A
    je .lf
    cmp al, 0x08
    je .bs
    mov eax, [cursor_y]
    imul eax, 80
    add eax, [cursor_x]
    shl eax, 1
    mov ebx, 0xB8000
    add ebx, eax
    mov [ebx], al
    mov byte [ebx+1], 0x0F
    inc dword [cursor_x]
    mov eax, [cursor_x]
    cmp eax, 80
    jl .done
    mov dword [cursor_x], 0
    inc dword [cursor_y]
    jmp .clamp
.cr:
    mov dword [cursor_x], 0
    jmp .done
.lf:
    inc dword [cursor_y]
    jmp .clamp
.bs:
    mov eax, [cursor_x]
    cmp eax, 0
    jz .done
    dec dword [cursor_x]
    mov eax, [cursor_y]
    imul eax, 80
    add eax, [cursor_x]
    shl eax, 1
    mov ebx, 0xB8000
    add ebx, eax
    mov byte [ebx], ' '
    mov byte [ebx+1], 0x0F
    jmp .done
.clamp:
    mov eax, [cursor_y]
    cmp eax, 25
    jl .done
    mov dword [cursor_y], 24
.done:
    popa
    ret

vga_print:
    pusha
.print:
    lodsb
    test al, al
    jz .out
    call vga_putc
    jmp .print
.out:
    popa
    ret

; ===== Teclado PS/2 =====
kbd_read_char:
    pusha
.wait:
    in al, 0x64
    test al, 1
    jz .wait
    in al, 0x60
    mov bl, al
    cmp bl, 0x80
    jae .wait
    mov al, 0
    cmp bl, 0x23 ; h
    je .h
    cmp bl, 0x1F ; s
    je .s
    cmp bl, 0x10 ; q
    je .q
    jmp .wait
.h: mov al, 'h' 
    jmp .out
.s: mov al, 's' 
    jmp .out
.q: mov al, 'q' 
    jmp .out
.out:
    popa
    ret

; ===== Datos =====
banner db '🚀 ReactOS Rust Kernel (flat, multiboot1) OK', 0x0A, 0x0D, 0
prompt db 'reactos> ', 0
help_text db 0x0A, 0x0D, 'h/s/q disponibles', 0x0A, 0x0D, 0
system_status db 0x0A, 0x0D, 'Sistema x86, VGA texto', 0x0A, 0x0D, 0
goodbye_msg db 0x0A, 0x0D, 'Adios', 0x0A, 0x0D, 0

; ===== BSS =====
ALIGN 16
cursor_x dd 0
cursor_y dd 0

_load_end:
_bss_end:

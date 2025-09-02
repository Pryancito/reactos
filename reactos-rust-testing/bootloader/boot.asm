; Bootloader simple para ReactOS Rust Kernel
; Ensamblador NASM para x86_64

bits 16
org 0x7C00

start:
    ; Configurar segmentos
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    
    ; Habilitar A20
    call enable_a20
    
    ; Cargar GDT
    lgdt [gdt_descriptor]
    
    ; Habilitar modo protegido
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    
    ; Saltar a código de 32 bits
    jmp 0x08:protected_mode

enable_a20:
    ; Método simple para habilitar A20
    in al, 0x92
    or al, 2
    out 0x92, al
    ret

bits 32
protected_mode:
    ; Configurar segmentos de datos
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    
    ; Configurar stack
    mov esp, 0x90000
    
    ; Cargar kernel desde disco
    call load_kernel
    
    ; Saltar al kernel
    jmp 0x100000

load_kernel:
    ; Cargar kernel desde el sector 2 en adelante
    mov edi, 0x100000  ; Dirección de destino
    mov ecx, 64        ; Número de sectores a cargar
    mov ebx, 2         ; Sector inicial
    
load_sector:
    push ecx
    push edi
    push ebx
    
    ; Leer sector
    mov ah, 0x02
    mov al, 1
    mov ch, 0
    mov cl, bl
    mov dh, 0
    mov dl, 0x80
    int 0x13
    
    pop ebx
    pop edi
    pop ecx
    
    ; Verificar error
    jc error
    
    ; Siguiente sector
    inc ebx
    add edi, 512
    loop load_sector
    
    ret

error:
    ; Manejar error de carga
    hlt
    jmp error

; GDT
gdt_start:
    ; Descriptor nulo
    dd 0x0
    dd 0x0
    
    ; Descriptor de código
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 10011010b
    db 11001111b
    db 0x0
    
    ; Descriptor de datos
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 10010010b
    db 11001111b
    db 0x0

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

; Rellenar hasta 510 bytes
times 510-($-$$) db 0
dw 0xAA55

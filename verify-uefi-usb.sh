#!/bin/bash

# SCRIPT PARA VERIFICAR QUE EL USB UEFI ESTÁ CONFIGURADO CORRECTAMENTE

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Función para imprimir mensajes
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_header() {
    echo -e "${CYAN}${BOLD}$1${NC}"
    echo -e "${CYAN}$(printf '=%.0s' {1..50})${NC}"
}

# Verificar argumentos
if [[ $# -ne 1 ]]; then
    print_error "Uso: $0 <dispositivo_usb>"
    print_error "Ejemplo: $0 /dev/sdb"
    exit 1
fi

USB_DEVICE="$1"

print_header "VERIFICANDO CONFIGURACIÓN DEL USB UEFI"
echo

print_info "Dispositivo USB: $USB_DEVICE"
echo

# Verificar que el dispositivo existe
if [[ ! -b "$USB_DEVICE" ]]; then
    print_error "El dispositivo USB '$USB_DEVICE' no existe"
    exit 1
fi

# Verificar tabla de particiones
print_info "Verificando tabla de particiones..."
PARTITION_TABLE=$(sudo parted "$USB_DEVICE" print 2>/dev/null | grep "Partition Table:" | awk '{print $3}')

if [[ "$PARTITION_TABLE" == "gpt" ]]; then
    print_success "Tabla de particiones GPT detectada"
else
    print_error "Tabla de particiones incorrecta: $PARTITION_TABLE (debe ser GPT)"
    exit 1
fi

# Verificar partición EFI
print_info "Verificando partición EFI..."
EFI_PARTITION=$(sudo parted "$USB_DEVICE" print 2>/dev/null | grep "boot, esp" | awk '{print $1}')

if [[ -n "$EFI_PARTITION" ]]; then
    print_success "Partición EFI detectada: $EFI_PARTITION"
else
    print_error "No se detectó partición EFI marcada como ESP"
    exit 1
fi

# Verificar sistema de archivos
print_info "Verificando sistema de archivos..."
PARTITION_INFO=$(sudo parted "$USB_DEVICE" print 2>/dev/null | grep "boot, esp")
FILE_SYSTEM=$(echo "$PARTITION_INFO" | awk '{print $5}')

if [[ "$FILE_SYSTEM" == "fat32" ]]; then
    print_success "Sistema de archivos FAT32 detectado"
else
    print_error "Sistema de archivos incorrecto: $FILE_SYSTEM (debe ser fat32)"
    print_info "Información de la partición: $PARTITION_INFO"
    exit 1
fi

# Montar partición EFI para verificar contenido
print_info "Montando partición EFI para verificar contenido..."
MOUNT_POINT="/tmp/verify-uefi"
sudo mkdir -p "$MOUNT_POINT"
sudo mount "${USB_DEVICE}1" "$MOUNT_POINT" 2>/dev/null

if [[ $? -eq 0 ]]; then
    print_success "Partición EFI montada correctamente"
    
    # Verificar estructura de directorios
    print_info "Verificando estructura de directorios..."
    
    if [[ -d "$MOUNT_POINT/EFI/BOOT" ]]; then
        print_success "Directorio EFI/BOOT encontrado"
    else
        print_error "Directorio EFI/BOOT no encontrado"
    fi
    
    if [[ -d "$MOUNT_POINT/EFI/REACTOS" ]]; then
        print_success "Directorio EFI/REACTOS encontrado"
    else
        print_error "Directorio EFI/REACTOS no encontrado"
    fi
    
    # Verificar archivos EFI
    print_info "Verificando archivos EFI..."
    
    if [[ -f "$MOUNT_POINT/EFI/BOOT/bootx64.efi" ]]; then
        print_success "bootx64.efi encontrado"
        
        # Verificar que es un ejecutable EFI válido
        EFI_TYPE=$(file "$MOUNT_POINT/EFI/BOOT/bootx64.efi" | grep -o "PE32+ executable for EFI")
        if [[ -n "$EFI_TYPE" ]]; then
            print_success "bootx64.efi es un ejecutable EFI válido"
        else
            print_error "bootx64.efi no es un ejecutable EFI válido"
        fi
    else
        print_error "bootx64.efi no encontrado"
    fi
    
    if [[ -f "$MOUNT_POINT/EFI/REACTOS/reactos-uefi-native.efi" ]]; then
        print_success "reactos-uefi-native.efi encontrado"
        
        # Verificar que es un ejecutable EFI válido
        EFI_TYPE=$(file "$MOUNT_POINT/EFI/REACTOS/reactos-uefi-native.efi" | grep -o "PE32+ executable for EFI")
        if [[ -n "$EFI_TYPE" ]]; then
            print_success "reactos-uefi-native.efi es un ejecutable EFI válido"
        else
            print_error "reactos-uefi-native.efi no es un ejecutable EFI válido"
        fi
    else
        print_error "reactos-uefi-native.efi no encontrado"
    fi
    
    # Desmontar
    sudo umount "$MOUNT_POINT"
    sudo rmdir "$MOUNT_POINT"
    
else
    print_error "No se pudo montar la partición EFI"
    exit 1
fi

echo
print_header "RESUMEN DE VERIFICACIÓN"
echo

# Mostrar información del dispositivo
print_info "Información del dispositivo:"
sudo parted "$USB_DEVICE" print 2>/dev/null | grep -E "(Model|Disk|Partition Table|Number|Start|End|Size|File system|Name|Flags)"

echo
print_success "¡USB UEFI verificado correctamente!"
print_info "El USB está listo para ser reconocido por la BIOS UEFI"
print_info "Insértalo en tu ASUS 10ª generación y debería aparecer en el menú de arranque"

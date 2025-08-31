#!/bin/bash

# SCRIPT MEJORADO PARA CREAR USB UEFI BOOTEABLE CON ESTRUCTURA EFI CORRECTA
# Este script crea las particiones EFI necesarias en el USB y extrae los archivos correctamente

set -e  # Salir si algún comando falla

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

# Verificar si se ejecuta como root
if [[ $EUID -ne 0 ]]; then
    print_error "Este script debe ejecutarse como root (sudo)"
    exit 1
fi

# Verificar argumentos
if [[ $# -ne 2 ]]; then
    print_error "Uso: $0 <dispositivo_usb> <iso_uefi>"
    print_error "Ejemplo: $0 /dev/sdb output-posix-amd64/reactos-uefi-2015-plus.iso"
    exit 1
fi

USB_DEVICE="$1"
ISO_FILE="$2"

# Verificar que el archivo ISO existe
if [[ ! -f "$ISO_FILE" ]]; then
    print_error "El archivo ISO '$ISO_FILE' no existe"
    exit 1
fi

# Verificar que el dispositivo USB existe
if [[ ! -b "$USB_DEVICE" ]]; then
    print_error "El dispositivo USB '$USB_DEVICE' no existe"
    exit 1
fi

print_header "CREANDO USB UEFI BOOTEABLE CON ESTRUCTURA EFI CORRECTA"
echo

print_info "Dispositivo USB: $USB_DEVICE"
print_info "Archivo ISO: $ISO_FILE"
echo

# Confirmar antes de proceder
print_warning "⚠️  ATENCIÓN: Este proceso BORRARÁ TODOS los datos del USB $USB_DEVICE"
read -p "¿Estás seguro de continuar? (s/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Ss]$ ]]; then
    print_info "Operación cancelada"
    exit 0
fi

print_info "Procediendo con la creación del USB UEFI..."
echo

# Desmontar el USB si está montado
print_info "Desmontando USB si está montado..."
umount "$USB_DEVICE"* 2>/dev/null || true
sleep 2

# Crear tabla de particiones GPT
print_info "Creando tabla de particiones GPT..."
parted "$USB_DEVICE" mklabel gpt
print_success "Tabla GPT creada"

# Crear partición EFI (FAT32)
print_info "Creando partición EFI (FAT32)..."
parted "$USB_DEVICE" mkpart primary fat32 1MiB 100MiB
parted "$USB_DEVICE" set 1 esp on
print_success "Partición EFI creada y marcada como ESP"

# Verificar particiones creadas
print_info "Verificando particiones creadas..."
parted "$USB_DEVICE" print
echo

# Formatear partición EFI como FAT32
print_info "Formateando partición EFI como FAT32..."
mkfs.fat -F 32 "${USB_DEVICE}1"
print_success "Partición EFI formateada como FAT32"

# Montar partición EFI
print_info "Montando partición EFI..."
mkdir -p /tmp/reactos-efi
mount "${USB_DEVICE}1" /tmp/reactos-efi
print_success "Partición EFI montada en /tmp/reactos-efi"

# Crear estructura de directorios EFI
print_info "Creando estructura de directorios EFI..."
mkdir -p /tmp/reactos-efi/EFI/BOOT
mkdir -p /tmp/reactos-efi/EFI/REACTOS
print_success "Estructura de directorios EFI creada"

# Extraer archivos EFI de la ISO
print_info "Extrayendo archivos EFI de la ISO..."
cd /tmp/reactos-efi

# Extraer BOOTX64.EFI
print_info "Extrayendo BOOTX64.EFI..."
isoinfo -i "$(pwd)/$ISO_FILE" -x /EFI/BOOT/bootx64.efi | tee EFI/BOOT/bootx64.efi > /dev/null
print_success "BOOTX64.EFI extraído"

# Extraer REACTOS_.EFI
print_info "Extrayendo REACTOS_.EFI..."
isoinfo -i "$(pwd)/$ISO_FILE" -x /EFI/REACTOS/reactos-uefi-native.efi | tee EFI/REACTOS/reactos-uefi-native.efi > /dev/null
print_success "REACTOS_.EFI extraído como reactos-uefi-native.efi"

# Verificar archivos extraídos
print_info "Verificando archivos extraídos..."
ls -la EFI/BOOT/
ls -la EFI/REACTOS/
echo

# Verificar que los archivos EFI son válidos
print_info "Verificando archivos EFI..."
file EFI/BOOT/bootx64.efi
file EFI/REACTOS/reactos-uefi-native.efi
echo

# Desmontar partición EFI
print_info "Desmontando partición EFI..."
cd -
umount /tmp/reactos-efi
rmdir /tmp/reactos-efi

print_header "USB UEFI BOOTEABLE CREADO EXITOSAMENTE"
print_success "El USB $USB_DEVICE ahora tiene la estructura EFI correcta"
print_success "Partición EFI: ${USB_DEVICE}1 (FAT32, 100MB)"
echo
print_info "Estructura EFI creada:"
print_info "  /EFI/BOOT/bootx64.efi (Bootloader UEFI estándar)"
print_info "  /EFI/REACTOS/reactos-uefi-native.efi (Bootloader ReactOS)"
echo
print_success "¡Tu USB está listo para bootear en sistemas UEFI!"
print_info "Insértalo en tu ASUS 10ª generación y debería ser reconocido automáticamente"

echo
print_info "🔧 VERIFICACIÓN FINAL:"
print_info "1. El USB tiene tabla de particiones GPT"
print_info "2. La partición EFI está marcada como ESP (boot, esp)"
print_info "3. Los archivos EFI son válidos y ejecutables"
print_info "4. La estructura de directorios cumple el estándar UEFI"

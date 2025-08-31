#!/bin/bash

# SCRIPT SIMPLE PARA EXTRAER ARCHIVOS EFI DE LA ISO
# Este script solo extrae los archivos para verificar que funcionan

set -e

# Colores para output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🔍 EXTRAYENDO ARCHIVOS EFI DE LA ISO${NC}"
echo "=========================================="
echo

# Verificar que la ISO existe
if [[ ! -f "reactos-uefi-efi.iso" ]]; then
    echo "❌ Error: reactos-uefi-efi.iso no encontrada"
    exit 1
fi

# Crear directorio temporal
mkdir -p efi_extract
cd efi_extract

echo "📁 Creando estructura de directorios..."
mkdir -p EFI/BOOT
mkdir -p EFI/REACTOS

echo "📤 Extrayendo archivos EFI..."

# Extraer BOOTX64.EFI
echo "  - Extrayendo BOOTX64.EFI..."
isoinfo -i "../reactos-uefi-efi.iso" -x /EFI/BOOT/BOOTX64.EFI > EFI/BOOT/bootx64.efi

# Extraer REACTOS_.EFI
echo "  - Extrayendo REACTOS_.EFI..."
isoinfo -i "../reactos-uefi-efi.iso" -x /EFI/REACTOS/REACTOS_.EFI > EFI/REACTOS/reactos-uefi-native.efi

# Extraer GRUB.CFG
echo "  - Extrayendo GRUB.CFG..."
isoinfo -i "../reactos-uefi-efi.iso" -x /EFI/BOOT/GRUB.CFG > EFI/BOOT/grub.cfg

# Extraer REACTOS_.INI
echo "  - Extrayendo REACTOS_.INI..."
isoinfo -i "../reactos-uefi-efi.iso" -x /EFI/REACTOS/REACTOS_.INI > EFI/REACTOS/reactos-uefi-config.ini

echo
echo -e "${GREEN}✅ Archivos extraídos exitosamente${NC}"
echo

echo "📋 Verificando archivos extraídos:"
echo "=================================="
ls -la EFI/BOOT/
echo
ls -la EFI/REACTOS/
echo

echo "🔍 Verificando tipos de archivo:"
echo "================================"
file EFI/BOOT/bootx64.efi
file EFI/REACTOS/reactos-uefi-native.efi
echo

echo "📊 Tamaños de archivos:"
echo "======================="
du -h EFI/BOOT/*
du -h EFI/REACTOS/*
echo

echo -e "${GREEN}🎉 Extracción completada. Los archivos están en el directorio 'efi_extract'${NC}"
echo "Ahora puedes copiar estos archivos a un USB con partición EFI formateada como FAT32"

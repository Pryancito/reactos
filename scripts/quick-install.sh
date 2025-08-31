#!/bin/bash

# SCRIPT DE INSTALACIÓN RÁPIDA PARA REACTOS UEFI
# Uso: ./quick-install.sh <dispositivo_usb> <iso_file>

set -e

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}🚀 INSTALACIÓN RÁPIDA REACTOS UEFI${NC}"
echo

if [[ $# -ne 2 ]]; then
    echo -e "${RED}Uso: $0 <dispositivo_usb> <iso_file>${NC}"
    echo -e "${RED}Ejemplo: $0 /dev/sdb output-posix-amd64/reactos-uefi-2015-plus.iso${NC}"
    exit 1
fi

USB_DEVICE="$1"
ISO_FILE="$2"

echo -e "${BLUE}Dispositivo USB: $USB_DEVICE${NC}"
echo -e "${BLUE}Archivo ISO: $ISO_FILE${NC}"
echo

# Verificar que se ejecuta como root
if [[ $EUID -ne 0 ]]; then
    echo -e "${RED}Este script debe ejecutarse como root (sudo)${NC}"
    exit 1
fi

# Verificar archivos
if [[ ! -f "$ISO_FILE" ]]; then
    echo -e "${RED}El archivo ISO '$ISO_FILE' no existe${NC}"
    exit 1
fi

if [[ ! -b "$USB_DEVICE" ]]; then
    echo -e "${RED}El dispositivo USB '$USB_DEVICE' no existe${NC}"
    exit 1
fi

echo -e "${BLUE}⚠️  ATENCIÓN: Este proceso BORRARÁ TODOS los datos del USB $USB_DEVICE${NC}"
read -p "¿Estás seguro de continuar? (s/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Ss]$ ]]; then
    echo "Operación cancelada"
    exit 0
fi

echo -e "${BLUE}Procediendo con la instalación...${NC}"

# Desmontar USB
umount "$USB_DEVICE"* 2>/dev/null || true
sleep 2

# Crear tabla GPT
echo -e "${BLUE}Creando tabla GPT...${NC}"
parted "$USB_DEVICE" mklabel gpt

# Crear partición EFI
echo -e "${BLUE}Creando partición EFI...${NC}"
parted "$USB_DEVICE" mkpart primary fat32 1MiB 100MiB
parted "$USB_DEVICE" set 1 esp on

# Formatear
echo -e "${BLUE}Formateando partición EFI...${NC}"
mkfs.fat -F 32 "${USB_DEVICE}1"

# Montar y crear estructura
echo -e "${BLUE}Creando estructura EFI...${NC}"
mkdir -p /tmp/reactos-efi
mount "${USB_DEVICE}1" /tmp/reactos-efi
mkdir -p /tmp/reactos-efi/EFI/BOOT
mkdir -p /tmp/reactos-efi/EFI/REACTOS

# Extraer archivos
echo -e "${BLUE}Extrayendo archivos EFI...${NC}"
cd /tmp/reactos-efi
isoinfo -i "$(pwd)/$ISO_FILE" -x /EFI/BOOT/bootx64.efi | tee EFI/BOOT/bootx64.efi > /dev/null
isoinfo -i "$(pwd)/$ISO_FILE" -x /EFI/REACTOS/reactos-uefi-native.efi | tee EFI/REACTOS/reactos-uefi-native.efi > /dev/null

# Desmontar
cd -
umount /tmp/reactos-efi
rmdir /tmp/reactos-efi

echo -e "${GREEN}✅ ¡Instalación completada exitosamente!${NC}"
echo -e "${BLUE}El USB está listo para bootear ReactOS en sistemas UEFI${NC}"

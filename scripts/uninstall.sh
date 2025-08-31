#!/bin/bash

# SCRIPT DE DESINSTALACIÓN
# Uso: ./uninstall.sh

set -e

# Colores
RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${BLUE}🗑️  DESINSTALANDO REACTOS UEFI BOOTLOADER${NC}"
echo

echo -e "${BLUE}¿Estás seguro de que quieres desinstalar? (s/N):${NC}"
read -p "" -n 1 -r
echo
if [[ ! $REPLY =~ ^[Ss]$ ]]; then
    echo "Desinstalación cancelada"
    exit 0
fi

echo -e "${BLUE}Desinstalando archivos...${NC}"

# Remover directorios
rm -rf scripts/
rm -rf docs/
rm -rf build/

# Remover archivos principales
rm -f Makefile
rm -f LICENSE
rm -f .gitignore

# Remover archivos de instalación
rm -f create-uefi-usb-fixed.sh
rm -f verify-uefi-usb.sh
rm -f test-uefi-usb-qemu.sh
rm -f SOLUCION-USB-UEFI-ASUS-10GEN.md

echo -e "${GREEN}✅ Desinstalación completada${NC}"
echo -e "${BLUE}Los archivos han sido removidos del sistema${NC}"

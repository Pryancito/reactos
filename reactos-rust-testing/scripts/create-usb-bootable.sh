#!/bin/bash

# Script para crear USB booteable con ReactOS Rust Kernel
# Uso: ./create-usb-bootable.sh /dev/sdX

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para mostrar ayuda
show_help() {
    echo -e "${BLUE}🚀 ReactOS Rust Kernel - USB Booteable${NC}"
    echo "=============================================="
    echo ""
    echo "Uso: $0 <dispositivo_usb>"
    echo ""
    echo "Ejemplos:"
    echo "  $0 /dev/sdb    # Para USB en /dev/sdb"
    echo "  $0 /dev/sdc    # Para USB en /dev/sdc"
    echo ""
    echo "⚠️  ADVERTENCIA: Este script formateará completamente el dispositivo USB"
    echo "⚠️  Asegúrate de hacer backup de los datos importantes"
    echo ""
    echo "Para listar dispositivos USB disponibles:"
    echo "  lsblk | grep -E 'sd[b-z]'"
    echo ""
}

# Verificar argumentos
if [ $# -eq 0 ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    show_help
    exit 0
fi

USB_DEVICE="$1"

# Verificar que el dispositivo existe
if [ ! -b "$USB_DEVICE" ]; then
    echo -e "${RED}❌ Error: El dispositivo $USB_DEVICE no existe o no es un dispositivo de bloque${NC}"
    echo ""
    echo "Dispositivos disponibles:"
    lsblk | grep -E 'sd[b-z]'
    exit 1
fi

# Verificar que no es el disco del sistema
if [[ "$USB_DEVICE" =~ ^/dev/sd[a-z]$ ]] && [[ "$USB_DEVICE" =~ ^/dev/sd[a]$ ]]; then
    echo -e "${RED}❌ Error: No puedes usar el disco del sistema ($USB_DEVICE)${NC}"
    exit 1
fi

# Verificar tamaño del dispositivo (mínimo 1GB)
DEVICE_SIZE=$(lsblk -b -d -o SIZE "$USB_DEVICE" | tail -1)
MIN_SIZE=$((1024 * 1024 * 1024)) # 1GB en bytes

if [ "$DEVICE_SIZE" -lt "$MIN_SIZE" ]; then
    echo -e "${RED}❌ Error: El dispositivo USB debe tener al menos 1GB de espacio${NC}"
    echo "Tamaño actual: $((DEVICE_SIZE / 1024 / 1024))MB"
    exit 1
fi

echo -e "${BLUE}🚀 Creando USB Booteable con ReactOS Rust Kernel${NC}"
echo "=================================================="
echo ""
echo -e "${YELLOW}📱 Dispositivo USB:${NC} $USB_DEVICE"
echo -e "${YELLOW}💾 Tamaño:${NC} $((DEVICE_SIZE / 1024 / 1024))MB"
echo ""

# Confirmación del usuario
echo -e "${RED}⚠️  ADVERTENCIA: Este proceso formateará completamente $USB_DEVICE${NC}"
echo -e "${RED}⚠️  Todos los datos en el dispositivo serán eliminados${NC}"
echo ""
read -p "¿Estás seguro de continuar? (escribe 'SI' para confirmar): " confirm

if [ "$confirm" != "SI" ]; then
    echo -e "${YELLOW}❌ Operación cancelada por el usuario${NC}"
    exit 0
fi

echo ""
echo -e "${BLUE}🔧 Preparando USB booteable...${NC}"

# Desmontar particiones existentes
echo "📤 Desmontando particiones existentes..."
umount "${USB_DEVICE}"* 2>/dev/null || true

# Crear tabla de particiones MBR
echo "💾 Creando tabla de particiones MBR..."
parted -s "$USB_DEVICE" mklabel msdos

# Crear partición primaria booteable
echo "📁 Creando partición primaria booteable..."
parted -s "$USB_DEVICE" mkpart primary fat32 1MiB 100%
parted -s "$USB_DEVICE" set 1 boot on

# Esperar a que el kernel reconozca la nueva partición
echo "⏳ Esperando reconocimiento de partición..."
sleep 2

# Formatear la partición como FAT32
USB_PARTITION="${USB_DEVICE}1"
echo "💿 Formateando como FAT32..."
mkfs.fat -F 32 -n "REACTOS_RUST" "$USB_PARTITION"

# Montar la partición
MOUNT_POINT="/tmp/reactos_usb_$$"
mkdir -p "$MOUNT_POINT"
echo "📂 Montando partición USB..."
mount "$USB_PARTITION" "$MOUNT_POINT"

# Instalar GRUB
echo "🔧 Instalando GRUB..."
grub-install --target=i386-pc --boot-directory="$MOUNT_POINT/boot" "$USB_DEVICE"

# Crear configuración GRUB
echo "⚙️ Creando configuración GRUB..."
mkdir -p "$MOUNT_POINT/boot/grub"

cat > "$MOUNT_POINT/boot/grub/grub.cfg" << 'EOF'
set timeout=10
set default=0

menuentry "ReactOS Rust Kernel" {
    set root='hd0,msdos1'
    multiboot /reactos-rust-kernel-elf
    boot
}

menuentry "ReactOS Rust Kernel (Modo Seguro)" {
    set root='hd0,msdos1'
    multiboot /reactos-rust-kernel-elf --safe-mode
    boot
}

menuentry "ReactOS Rust Kernel (Debug)" {
    set root='hd0,msdos1'
    multiboot /reactos-rust-kernel-elf --debug
    boot
}
EOF

# Copiar el kernel
echo "📦 Copiando kernel ReactOS Rust..."
cp /home/moebius/reactos/reactos-rust-kernel/target/x86_64-unknown-none/release/reactos-rust-kernel "$MOUNT_POINT/reactos-rust-kernel-elf"

# Crear archivo de información
echo "📄 Creando archivo de información..."
cat > "$MOUNT_POINT/README.txt" << 'EOF'
ReactOS Rust Kernel - USB Booteable
===================================

Este USB contiene el kernel ReactOS Rust compilado para x86_64.

Características:
- Sistema de archivos avanzado
- GUI moderno con ventanas
- Aplicaciones integradas (editor, calculadora, etc.)
- Sistema de audio avanzado
- Protocolos de red (HTTP, FTP)
- Shell interactivo completo
- Gestor de archivos gráfico
- Sistema de señales
- Panel de configuración

Para arrancar:
1. Configura tu BIOS/UEFI para arrancar desde USB
2. Selecciona "ReactOS Rust Kernel" en el menú GRUB
3. El sistema se iniciará automáticamente

Comandos del shell:
- help: Mostrar ayuda
- apps: Listar aplicaciones
- gui: Abrir interfaz gráfica
- netproto: Información de protocolos de red
- audio: Control de audio
- files: Gestor de archivos

Desarrollado con Rust para máxima seguridad y rendimiento.
EOF

# Sincronizar y desmontar
echo "💾 Sincronizando datos..."
sync

echo "📤 Desmontando USB..."
umount "$MOUNT_POINT"
rmdir "$MOUNT_POINT"

echo ""
echo -e "${GREEN}✅ USB booteable creado exitosamente!${NC}"
echo "=================================="
echo ""
echo -e "${BLUE}📱 Dispositivo:${NC} $USB_DEVICE"
echo -e "${BLUE}💾 Tamaño:${NC} $((DEVICE_SIZE / 1024 / 1024))MB"
echo -e "${BLUE}🏷️  Etiqueta:${NC} REACTOS_RUST"
echo -e "${BLUE}🔧 Bootloader:${NC} GRUB 2"
echo ""
echo -e "${YELLOW}🚀 Para arrancar desde USB:${NC}"
echo "1. Configura tu BIOS/UEFI para arrancar desde USB"
echo "2. Reinicia el sistema"
echo "3. Selecciona 'ReactOS Rust Kernel' en el menú GRUB"
echo ""
echo -e "${GREEN}🎉 ¡Tu kernel ReactOS Rust está listo para hardware real!${NC}"

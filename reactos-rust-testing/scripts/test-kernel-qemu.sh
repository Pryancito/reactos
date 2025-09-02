#!/bin/bash

# Script para probar ReactOS Rust Kernel con QEMU

set -e

echo "🚀 Probando ReactOS Rust Kernel con QEMU..."

# Configurar variables
OUTPUT_DIR="$(dirname "$0")/../test-data"
ISO_NAME="reactos-rust-kernel-testing.iso"
ISO_PATH="$OUTPUT_DIR/$ISO_NAME"

# Verificar que la ISO existe
if [ ! -f "$ISO_PATH" ]; then
    echo "❌ Error: ISO no encontrada en $ISO_PATH"
    echo "💡 Ejecuta primero: ./scripts/create-grub-iso.sh"
    exit 1
fi

echo "📀 Iniciando QEMU con ISO: $ISO_NAME"

# Ejecutar QEMU
qemu-system-x86_64 \
    -cdrom "$ISO_PATH" \
    -m 512M \
    -boot d \
    -monitor stdio \
    -name "ReactOS Rust Kernel Testing"

echo "🎉 Testing completado!"

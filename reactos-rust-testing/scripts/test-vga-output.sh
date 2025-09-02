#!/bin/bash

# Script para probar el ReactOS Rust Kernel con salida VGA

set -e

echo "🚀 Probando ReactOS Rust Kernel con salida VGA..."
echo "================================================"

# Configurar variables
OUTPUT_DIR="$(dirname "$0")/../test-data"
ISO_PATH="$OUTPUT_DIR/reactos-rust-kernel-testing.iso"

# Verificar si la ISO existe
if [ ! -f "$ISO_PATH" ]; then
    echo "❌ Error: ISO no encontrada en $ISO_PATH"
    echo "💡 Ejecuta ./scripts/create-grub-iso.sh primero."
    exit 1
fi

echo "📀 ISO encontrada: $(basename "$ISO_PATH")"
echo "📊 Tamaño: $(ls -lh "$ISO_PATH" | awk '{print $5}')"
echo ""

echo "🎯 Iniciando QEMU con salida VGA..."
echo "💡 El kernel debería mostrar mensajes en la pantalla VGA"
echo "💡 Presiona Ctrl+Alt+Q para salir de QEMU"
echo ""

echo "🚀 Iniciando ReactOS Rust Kernel con VGA..."
echo "==========================================="

# Ejecutar QEMU con la ISO
qemu-system-x86_64 \
    -cdrom "$ISO_PATH" \
    -m 512M \
    -boot d \
    -name "ReactOS Rust Kernel - VGA Test" \
    -display gtk \
    -no-reboot \
    -no-shutdown

echo "🎉 Testing completado!"

#!/bin/bash

# Script para probar ReactOS Rust Kernel con salida serial

echo "🚀 Probando ReactOS Rust Kernel con salida serial..."
echo "=================================================="

# Configurar variables
ISO_PATH="test-data/reactos-rust-kernel-testing.iso"

# Verificar que la ISO existe
if [ ! -f "$ISO_PATH" ]; then
    echo "❌ Error: ISO no encontrada en $ISO_PATH"
    echo "💡 Ejecuta primero: ./scripts/create-grub-iso.sh"
    exit 1
fi

echo "📀 ISO encontrada: $(basename "$ISO_PATH")"
echo "📊 Tamaño: $(ls -lh "$ISO_PATH" | awk '{print $5}')"
echo ""

echo "🎯 Iniciando QEMU con salida serial..."
echo "💡 La salida del kernel aparecerá en esta terminal"
echo "💡 Presiona Ctrl+C para salir"
echo ""

echo "🚀 Iniciando ReactOS Rust Kernel..."
echo "=================================="

# Ejecutar QEMU con salida serial
qemu-system-x86_64 \
    -cdrom "$ISO_PATH" \
    -m 512M \
    -boot d \
    -serial stdio \
    -nographic \
    -name "ReactOS Rust Kernel Testing"

echo ""
echo "🎉 Testing completado!"

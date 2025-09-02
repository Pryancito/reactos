#!/bin/bash

# Script simple para probar ReactOS Rust Kernel

echo "🚀 Probando ReactOS Rust Kernel..."
echo "================================="

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

echo "🎯 Iniciando QEMU..."
echo "💡 Controles:"
echo "   • Ctrl+Alt+Q - Salir de QEMU"
echo "   • Ctrl+Alt+F - Cambiar a consola"
echo "   • Ctrl+Alt+G - Liberar mouse"
echo ""

echo "🚀 Iniciando ReactOS Rust Kernel..."
echo "=================================="

# Ejecutar QEMU con configuración mínima
qemu-system-x86_64 \
    -cdrom "$ISO_PATH" \
    -m 512M \
    -boot d

echo ""
echo "🎉 Testing completado!"

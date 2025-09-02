#!/bin/bash

# Script de testing rápido para ReactOS Rust Kernel

set -e

echo "🚀 ReactOS Rust Kernel - Testing Rápido"
echo "========================================"

# Configurar variables
OUTPUT_DIR="$(dirname "$0")/../test-data"
ISO_NAME="reactos-rust-kernel-testing.iso"
ISO_PATH="$OUTPUT_DIR/$ISO_NAME"

echo "📋 Verificando entorno..."

# Verificar que la ISO existe
if [ ! -f "$ISO_PATH" ]; then
    echo "❌ Error: ISO no encontrada"
    echo "💡 Creando ISO..."
    ./scripts/create-grub-iso.sh
fi

echo "✅ ISO encontrada: $ISO_NAME"
echo "📊 Tamaño: $(ls -lh "$ISO_PATH" | awk '{print $5}')"

echo ""
echo "🎯 Opciones de testing:"
echo "1. Probar con QEMU (recomendado)"
echo "2. Ver información de la ISO"
echo "3. Recrear ISO"
echo "4. Salir"

read -p "Selecciona una opción (1-4): " choice

case $choice in
    1)
        echo "🚀 Iniciando QEMU..."
        echo "💡 Presiona Ctrl+Alt+G para salir del modo gráfico"
        echo "💡 Presiona Ctrl+A, X para salir de QEMU"
        echo ""
        qemu-system-x86_64 \
            -cdrom "$ISO_PATH" \
            -m 512M \
            -boot d \
            -monitor stdio \
            -name "ReactOS Rust Kernel Testing" \
            -display gtk
        ;;
    2)
        echo "📊 Información de la ISO:"
        file "$ISO_PATH"
        echo ""
        echo "📁 Contenido de la ISO:"
        isoinfo -l -i "$ISO_PATH" | head -20
        ;;
    3)
        echo "🔄 Recreando ISO..."
        ./scripts/create-grub-iso.sh
        ;;
    4)
        echo "👋 ¡Hasta luego!"
        exit 0
        ;;
    *)
        echo "❌ Opción inválida"
        exit 1
        ;;
esac

echo "🎉 Testing completado!"

#!/bin/bash

# Script para probar ReactOS Rust Kernel y capturar la salida

echo "🚀 Probando ReactOS Rust Kernel y capturando salida..."
echo "====================================================="

# Configurar variables
ISO_PATH="test-data/reactos-rust-kernel-testing.iso"
OUTPUT_FILE="test-data/kernel-output.log"

# Verificar que la ISO existe
if [ ! -f "$ISO_PATH" ]; then
    echo "❌ Error: ISO no encontrada en $ISO_PATH"
    echo "💡 Ejecuta primero: ./scripts/create-grub-iso.sh"
    exit 1
fi

echo "📀 ISO encontrada: $(basename "$ISO_PATH")"
echo "📊 Tamaño: $(ls -lh "$ISO_PATH" | awk '{print $5}')"
echo ""

echo "🎯 Iniciando QEMU con captura de salida..."
echo "💡 La salida se guardará en: $OUTPUT_FILE"
echo ""

# Crear directorio de salida si no existe
mkdir -p test-data

echo "🚀 Iniciando ReactOS Rust Kernel..."
echo "=================================="

# Ejecutar QEMU con captura de salida
timeout 10s qemu-system-x86_64 \
    -cdrom "$ISO_PATH" \
    -m 512M \
    -boot d \
    -serial file:$OUTPUT_FILE \
    -nographic \
    -name "ReactOS Rust Kernel Testing" 2>/dev/null

echo ""
echo "📄 Verificando salida del kernel..."

# Verificar si se generó el archivo de salida
if [ -f "$OUTPUT_FILE" ]; then
    echo "✅ Archivo de salida generado: $OUTPUT_FILE"
    echo "📊 Tamaño: $(ls -lh "$OUTPUT_FILE" | awk '{print $5}')"
    echo ""
    echo "📋 Contenido de la salida:"
    echo "=========================="
    cat "$OUTPUT_FILE"
    echo ""
    echo "=========================="
else
    echo "❌ No se generó archivo de salida"
    echo "💡 El kernel puede no estar generando salida serial"
fi

echo ""
echo "🎉 Testing completado!"

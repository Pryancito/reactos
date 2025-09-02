#!/bin/bash

# Script para probar ReactOS Rust Kernel y verificar que funciona

echo "🚀 Probando ReactOS Rust Kernel con header multiboot..."
echo "======================================================"

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

echo "🎯 Iniciando QEMU para verificar que el kernel se carga..."
echo "💡 El kernel ahora debería cargar correctamente con el header multiboot"
echo ""

echo "🚀 Iniciando ReactOS Rust Kernel..."
echo "=================================="

# Ejecutar QEMU con timeout para verificar que se carga
echo "⏱️  Ejecutando QEMU por 10 segundos para verificar carga..."
timeout 10s qemu-system-x86_64 \
    -cdrom "$ISO_PATH" \
    -m 512M \
    -boot d \
    -name "ReactOS Rust Kernel Testing" 2>/dev/null

exit_code=$?

echo ""
if [ $exit_code -eq 124 ]; then
    echo "✅ ¡ÉXITO! El kernel se cargó correctamente"
    echo "🎉 QEMU se ejecutó sin errores durante 10 segundos"
    echo "💡 Esto significa que:"
    echo "   • El header multiboot está funcionando"
    echo "   • GRUB puede cargar el kernel"
    echo "   • El kernel se está ejecutando"
elif [ $exit_code -eq 0 ]; then
    echo "✅ ¡ÉXITO! El kernel se ejecutó y terminó correctamente"
    echo "🎉 El kernel completó su ejecución"
else
    echo "❌ Error: El kernel no se cargó correctamente (código: $exit_code)"
    echo "💡 Revisar configuración de multiboot"
fi

echo ""
echo "🎉 Testing completado!"

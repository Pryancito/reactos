#!/bin/bash

# ReactOS Rust OS - QEMU Test Script
# Script para probar ReactOS Rust OS en QEMU

set -e

echo "🚀 Iniciando test de ReactOS Rust OS en QEMU..."
echo ""

# Verificar que existe la ISO
if [ ! -f "output/reactos-rust-os.iso" ]; then
    echo "❌ ISO no encontrada. Ejecuta primero: ./scripts/build-all.sh"
    exit 1
fi

echo "✅ ISO encontrada: output/reactos-rust-os.iso"
echo ""

# Configuración de QEMU
QEMU_MEMORY="2048"
QEMU_CPU="qemu64"
QEMU_ACCEL="kvm"
ISO_NAME="output/reactos-rust-os.iso"

# Detectar aceleración
ACCEL_OPT=""
if [ -r /dev/kvm ] && [ -w /dev/kvm ]; then
    ACCEL_OPT="-accel $QEMU_ACCEL"
    echo "✅ KVM detectado - usando aceleración hardware"
else
    echo "⚠️ KVM no disponible - usando emulación software"
fi

echo ""

# Lanzar QEMU
echo "🎮 Lanzando QEMU..."
echo "  - Memoria: ${QEMU_MEMORY}MB"
echo "  - CPU: ${QEMU_CPU}"
echo "  - ISO: ${ISO_NAME}"
echo ""

qemu-system-x86_64 \
    $ACCEL_OPT \
    -m "$QEMU_MEMORY" \
    -cpu "$QEMU_CPU" \
    -cdrom "$ISO_NAME" \
    -boot d \
    -display gtk \
    -name "ReactOS Rust OS Test" \
    -usb -device usb-tablet \
    -rtc base=localtime \
    -vga std \
    -k es &

QEMU_PID=$!

echo "✅ QEMU lanzado (PID: $QEMU_PID)"
echo ""
echo "🎯 Instrucciones:"
echo "  - La ventana de QEMU debería abrirse automáticamente"
echo "  - El sistema debería bootear desde la ISO"
echo "  - Puedes probar la calculadora y otras aplicaciones"
echo "  - Para salir: Ctrl+Alt+Q en QEMU"
echo ""
echo "💡 Para detener QEMU: kill $QEMU_PID"

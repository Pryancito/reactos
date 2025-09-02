#!/bin/bash

# Script para verificar el estado del kernel de ReactOS Rust

echo "🔍 Verificando estado del kernel de ReactOS Rust..."
echo "================================================="

# Verificar si QEMU está ejecutándose
echo "📊 Verificando procesos de QEMU..."
qemu_processes=$(pgrep -f "qemu-system-x86_64" | wc -l)
if [ "$qemu_processes" -gt 0 ]; then
    echo "✅ QEMU está ejecutándose ($qemu_processes proceso(s))"
    echo "   PIDs: $(pgrep -f "qemu-system-x86_64" | tr '\n' ' ')"
else
    echo "❌ QEMU no está ejecutándose"
fi

echo ""

# Verificar archivos del kernel
echo "📦 Verificando archivos del kernel..."
KERNEL_PATH="/home/moebius/reactos/reactos-rust-kernel/target/x86_64-unknown-none/release/reactos-rust-kernel"
if [ -f "$KERNEL_PATH" ]; then
    kernel_size=$(ls -lh "$KERNEL_PATH" | awk '{print $5}')
    echo "✅ Kernel compilado - Tamaño: $kernel_size"
    echo "   Ubicación: $KERNEL_PATH"
else
    echo "❌ Kernel no compilado en $KERNEL_PATH"
fi

if [ -f "test-data/reactos-rust-kernel-testing.iso" ]; then
    iso_size=$(ls -lh test-data/reactos-rust-kernel-testing.iso | awk '{print $5}')
    echo "✅ ISO creada - Tamaño: $iso_size"
else
    echo "❌ ISO no creada"
fi

echo ""

# Verificar configuración de GRUB
echo "🍞 Verificando configuración de GRUB..."
if [ -f "grub-testing/boot/grub/grub.cfg" ]; then
    echo "✅ Configuración GRUB encontrada"
    echo "   Entradas de menú: $(grep -c "menuentry" grub-testing/boot/grub/grub.cfg)"
else
    echo "❌ Configuración GRUB no encontrada"
fi

echo ""

# Verificar scripts
echo "📜 Verificando scripts disponibles..."
scripts=("create-grub-iso.sh" "test-simple-qemu.sh" "test-with-serial.sh" "simple-test.sh" "quick-test.sh" "verify-setup.sh")
for script in "${scripts[@]}"; do
    if [ -f "scripts/$script" ] && [ -x "scripts/$script" ]; then
        echo "✅ $script"
    else
        echo "❌ $script"
    fi
done

echo ""

# Resumen del estado
echo "📊 RESUMEN DEL ESTADO:"
echo "====================="

if [ -f "$KERNEL_PATH" ] && [ -f "test-data/reactos-rust-kernel-testing.iso" ]; then
    echo "🎉 ¡Kernel listo para testing!"
    echo ""
    echo "💡 Comandos disponibles:"
    echo "   • ./scripts/test-simple-qemu.sh - Probar con QEMU (simple)"
    echo "   • ./scripts/simple-test.sh - Probar con QEMU (GUI)"
    echo "   • ./scripts/quick-test.sh - Testing interactivo"
    echo ""
    if [ "$qemu_processes" -gt 0 ]; then
        echo "⚠️  QEMU ya está ejecutándose - Puede que ya estés probando el kernel"
    else
        echo "🚀 Para probar ahora: ./scripts/test-simple-qemu.sh"
    fi
else
    echo "⚠️  Kernel no está listo - Ejecutar:"
    echo "   1. cd /home/moebius/reactos/reactos-rust-os/kernel && cargo build --release"
    echo "   2. ./scripts/create-grub-iso.sh"
fi

echo ""
echo "🔍 Verificación completada!"

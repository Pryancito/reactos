#!/bin/bash

# Script para verificar que el entorno de testing está configurado correctamente

echo "🔍 Verificando entorno de ReactOS Rust Kernel Testing..."
echo "========================================================"

# Verificar directorios
echo "📁 Verificando estructura de directorios..."
required_dirs=("kernel" "grub-testing" "scripts" "test-data")
for dir in "${required_dirs[@]}"; do
    if [ -d "$dir" ]; then
        echo "✅ $dir - OK"
    else
        echo "❌ $dir - FALTANTE"
    fi
done

echo ""

# Verificar archivos del kernel
echo "📦 Verificando kernel compilado..."
if [ -f "kernel/target/release/reactos-rust-kernel" ]; then
    echo "✅ Kernel compilado - OK"
    echo "   Tamaño: $(ls -lh kernel/target/release/reactos-rust-kernel | awk '{print $5}')"
else
    echo "❌ Kernel no compilado - Ejecutar: cd kernel && cargo build --release"
fi

echo ""

# Verificar configuración de GRUB
echo "🍞 Verificando configuración de GRUB..."
if [ -f "grub-testing/boot/grub/grub.cfg" ]; then
    echo "✅ Configuración GRUB - OK"
else
    echo "❌ Configuración GRUB - FALTANTE"
fi

echo ""

# Verificar ISO
echo "💿 Verificando ISO..."
if [ -f "test-data/reactos-rust-kernel-testing.iso" ]; then
    echo "✅ ISO creada - OK"
    echo "   Tamaño: $(ls -lh test-data/reactos-rust-kernel-testing.iso | awk '{print $5}')"
    echo "   Tipo: $(file test-data/reactos-rust-kernel-testing.iso | cut -d: -f2)"
else
    echo "❌ ISO no creada - Ejecutar: ./scripts/create-grub-iso.sh"
fi

echo ""

# Verificar scripts
echo "📜 Verificando scripts..."
scripts=("create-grub-iso.sh" "test-kernel-qemu.sh" "quick-test.sh" "verify-setup.sh")
for script in "${scripts[@]}"; do
    if [ -f "scripts/$script" ] && [ -x "scripts/$script" ]; then
        echo "✅ $script - OK"
    else
        echo "❌ $script - FALTANTE o NO EJECUTABLE"
    fi
done

echo ""

# Verificar dependencias del sistema
echo "🔧 Verificando dependencias del sistema..."
deps=("grub-mkrescue" "qemu-system-x86_64" "cargo" "rustc")
for dep in "${deps[@]}"; do
    if command -v "$dep" >/dev/null 2>&1; then
        echo "✅ $dep - OK"
    else
        echo "❌ $dep - NO INSTALADO"
    fi
done

echo ""

# Resumen
echo "📊 RESUMEN:"
echo "==========="

if [ -f "kernel/target/release/reactos-rust-kernel" ] && [ -f "test-data/reactos-rust-kernel-testing.iso" ]; then
    echo "🎉 ¡Entorno de testing listo!"
    echo "💡 Para probar: ./scripts/quick-test.sh"
    echo "💡 Para testing directo: ./scripts/test-kernel-qemu.sh"
else
    echo "⚠️  Entorno incompleto - Revisar errores arriba"
    echo "💡 Pasos para completar:"
    echo "   1. cd kernel && cargo build --release"
    echo "   2. ./scripts/create-grub-iso.sh"
fi

echo ""
echo "🔍 Verificación completada!"

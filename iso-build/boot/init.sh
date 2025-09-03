#!/bin/bash
# Script de inicialización de ReactOS Windows en Rust

echo "🦀 Iniciando ReactOS Windows en Rust..."
echo "======================================"

# Verificar que el kernel existe
if [ ! -f /boot/reactos-kernel ]; then
    echo "❌ Error: Kernel no encontrado"
    exit 1
fi

echo "✅ Kernel encontrado"
echo "🚀 Iniciando sistema..."

# Ejecutar el kernel
exec /boot/reactos-kernel

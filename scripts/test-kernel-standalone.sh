#!/bin/bash

# Script para probar el kernel standalone de ReactOS Rust OS
# Este script verifica que el kernel se compile correctamente

set -e

echo "🧪 Probando kernel standalone de ReactOS Rust OS..."
echo "=================================================="

# Cambiar al directorio del kernel
cd "$(dirname "$0")/../kernel"

echo "📁 Directorio actual: $(pwd)"

# Verificar que el archivo standalone.rs existe
if [ ! -f "src/standalone.rs" ]; then
    echo "❌ Error: No se encontró src/standalone.rs"
    exit 1
fi

echo "✅ Archivo standalone.rs encontrado"

# Intentar compilar el kernel standalone
echo "🔨 Compilando kernel standalone..."
if cargo build --bin reactos-rust-kernel-standalone; then
    echo "✅ Kernel standalone compilado exitosamente"
    
    # Verificar que el binario se creó
    if [ -f "target/debug/reactos-rust-kernel-standalone" ]; then
        echo "✅ Binario standalone encontrado"
        echo "📊 Tamaño del binario: $(ls -lh target/debug/reactos-rust-kernel-standalone | awk '{print $5}')"
        
        # Mostrar información del binario
        echo "📋 Información del binario:"
        file target/debug/reactos-rust-kernel-standalone
        
        echo ""
        echo "🎉 ¡Kernel standalone compilado y verificado exitosamente!"
        echo "🚀 El kernel está listo para ser usado con GRUB"
        
    else
        echo "❌ Error: Binario standalone no encontrado después de la compilación"
        exit 1
    fi
    
else
    echo "❌ Error: Falló la compilación del kernel standalone"
    exit 1
fi

echo ""
echo "📝 Próximos pasos:"
echo "   1. Copiar el binario a /boot/reactos-rust-kernel.bin"
echo "   2. Configurar GRUB para cargar el kernel"
echo "   3. Crear ISO booteable"
echo "   4. Probar en QEMU o hardware real"

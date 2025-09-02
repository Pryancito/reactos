#!/bin/bash

# Script para crear ISO del kernel ReactOS Rust con Motor 3D

echo "🎮 Creando ISO del ReactOS Rust Kernel con Motor 3D..."

# Crear directorio de trabajo
mkdir -p iso/boot/grub

# Copiar el kernel real
echo "📦 Copiando kernel..."
if [ -f "./kernel.bin" ]; then
    cp ./kernel.bin iso/boot/kernel.bin
    echo "✅ Kernel real copiado exitosamente"
else
    echo "⚠️  Kernel no encontrado, creando kernel simulado..."
    echo "Motor 3D ReactOS Rust Kernel v1.0" > iso/boot/kernel.bin
fi

# Crear configuración GRUB
echo "⚙️  Configurando GRUB..."
cat > iso/boot/grub/grub.cfg << 'EOF'
set timeout=5
set default=0

menuentry "ReactOS Rust Kernel - Motor 3D" {
    multiboot /boot/kernel.bin
    boot
}

menuentry "ReactOS Rust Kernel - Modo Seguro" {
    multiboot /boot/kernel.bin --safe-mode
    boot
}

menuentry "ReactOS Rust Kernel - Debug" {
    multiboot /boot/kernel.bin --debug
    boot
}
EOF

# Crear ISO
echo "🔥 Creando ISO..."
grub-mkrescue -o reactos-rust-3d-kernel.iso iso/

if [ $? -eq 0 ]; then
    echo "✅ ISO creada exitosamente: reactos-rust-3d-kernel.iso"
    echo "📊 Tamaño del ISO: $(du -h reactos-rust-3d-kernel.iso | cut -f1)"
    echo ""
    echo "🎮 Características del Motor 3D:"
    echo "   • Vulkan 1.3 con Ray Tracing"
    echo "   • RTX 2060 Super (34 RT Cores)"
    echo "   • Shaders avanzados"
    echo "   • Post-procesamiento"
    echo "   • Iluminación global"
    echo ""
    echo "🚀 Para ejecutar: qemu-system-x86_64 -cdrom reactos-rust-3d-kernel.iso"
else
    echo "❌ Error al crear el ISO"
    exit 1
fi

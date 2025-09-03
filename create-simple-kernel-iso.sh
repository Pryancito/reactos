#!/bin/bash

# Script para crear una ISO simple con el kernel de Eclipse OS
# Sin dependencias del binario principal problemático

set -e

echo "🌙 Creando ISO simple con kernel de Eclipse OS..."

# Limpiar directorios anteriores
rm -rf iso
mkdir -p iso/boot/grub

# Copiar el kernel de Eclipse OS
if [ -f "eclipse-kernel" ]; then
    cp eclipse-kernel iso/boot/vmlinuz-eclipse
    chmod +x iso/boot/vmlinuz-eclipse
    echo "✅ Kernel de Eclipse OS copiado"
else
    echo "❌ Kernel de Eclipse OS no encontrado"
    exit 1
fi

# Crear configuración GRUB
cat > iso/boot/grub/grub.cfg << 'EOF'
# Configuración GRUB para Eclipse OS
set timeout=10
set default=0

menuentry "Eclipse OS Kernel" {
    echo "Cargando Eclipse OS Kernel..."
    multiboot /boot/vmlinuz-eclipse
}

menuentry "Eclipse OS (Debug)" {
    echo "Cargando Eclipse OS Kernel en modo debug..."
    multiboot /boot/vmlinuz-eclipse debug
}
EOF

echo "✅ Configuración GRUB creada"

# Crear ISO
echo "📦 Creando ISO..."
grub-mkrescue -o eclipse-os-kernel.iso iso/

echo "✅ ISO creada: eclipse-os-kernel.iso"
ls -lh eclipse-os-kernel.iso

echo ""
echo "🚀 Para probar la ISO:"
echo "qemu-system-x86_64 -cdrom eclipse-os-kernel.iso"

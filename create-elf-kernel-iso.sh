#!/bin/bash

# Script para crear una ISO con el kernel ELF de Eclipse OS

set -e

echo "🌙 Creando ISO con kernel ELF de Eclipse OS..."

# Limpiar directorios anteriores
rm -rf iso
mkdir -p iso/boot/grub

# Copiar el kernel ELF de Eclipse OS
if [ -f "simple-eclipse-kernel.elf" ]; then
    cp simple-eclipse-kernel.elf iso/boot/vmlinuz-eclipse
    chmod +x iso/boot/vmlinuz-eclipse
    echo "✅ Kernel ELF de Eclipse OS copiado"
else
    echo "❌ Kernel ELF de Eclipse OS no encontrado"
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
grub-mkrescue -o eclipse-os-elf-kernel.iso iso/

echo "✅ ISO creada: eclipse-os-elf-kernel.iso"
ls -lh eclipse-os-elf-kernel.iso

echo ""
echo "🚀 Para probar la ISO:"
echo "qemu-system-x86_64 -cdrom eclipse-os-elf-kernel.iso"

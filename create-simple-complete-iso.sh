#!/bin/bash

# Script simplificado para crear ISO con Eclipse OS completo
set -e

echo "🌙 Creando ISO con Eclipse OS completo..."

# Limpiar directorios anteriores
rm -rf simple-complete-iso
mkdir -p simple-complete-iso/boot/grub

# Copiar el kernel de Eclipse OS
if [ -f "fixed-eclipse-kernel.elf" ]; then
    cp fixed-eclipse-kernel.elf simple-complete-iso/boot/vmlinuz-eclipse
    chmod +x simple-complete-iso/boot/vmlinuz-eclipse
    echo "✅ Kernel de Eclipse OS copiado"
else
    echo "❌ Kernel de Eclipse OS no encontrado"
    exit 1
fi

# Copiar el binario eclipse-os
if [ -f "target/release/eclipse-os" ]; then
    cp target/release/eclipse-os simple-complete-iso/eclipse-os
    chmod +x simple-complete-iso/eclipse-os
    echo "✅ Binario eclipse-os copiado"
else
    echo "❌ Binario eclipse-os no encontrado"
    exit 1
fi

# Crear configuración GRUB simple
cat > simple-complete-iso/boot/grub/grub.cfg << 'EOF'
# Configuración GRUB para Eclipse OS
set timeout=10
set default=0

menuentry "Eclipse OS Kernel" {
    echo "Cargando Eclipse OS Kernel..."
    multiboot /boot/vmlinuz-eclipse
    boot
}

menuentry "Eclipse OS (Debug)" {
    echo "Cargando Eclipse OS Kernel en modo debug..."
    multiboot /boot/vmlinuz-eclipse debug
    boot
}
EOF

# Crear ISO
echo "📦 Creando ISO..."
grub-mkrescue -o eclipse-os-simple-complete.iso simple-complete-iso/

echo "✅ ISO creada: eclipse-os-simple-complete.iso"
ls -lh eclipse-os-simple-complete.iso

echo ""
echo "🚀 Para probar la ISO:"
echo "qemu-system-x86_64 -cdrom eclipse-os-simple-complete.iso -m 512M -display gtk"
echo ""
echo "📝 Nota: Esta ISO incluye tanto el kernel como el binario eclipse-os"
echo "   El kernel arranca y muestra el shell básico"
echo "   El binario eclipse-os está disponible en /eclipse-os"



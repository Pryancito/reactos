#!/bin/bash

# Script para crear ISO booteable para hardware real
set -e

echo "🖥️  Creando ISO booteable para hardware real..."
echo ""

# Limpiar directorios anteriores
rm -rf real-hardware-iso
mkdir -p real-hardware-iso/boot/grub

# Copiar el kernel para hardware real
if [ -f "real-hardware-kernel.elf" ]; then
    cp real-hardware-kernel.elf real-hardware-iso/boot/vmlinuz-eclipse
    chmod +x real-hardware-iso/boot/vmlinuz-eclipse
    echo "✅ Kernel para hardware real copiado"
else
    echo "❌ Kernel para hardware real no encontrado"
    exit 1
fi

# Crear configuración GRUB optimizada para hardware real
cat > real-hardware-iso/boot/grub/grub.cfg << 'EOF'
# Configuración GRUB para hardware real
set timeout=10
set default=0

menuentry "Eclipse OS - Hardware Real" {
    echo "Cargando Eclipse OS para hardware real..."
    multiboot /boot/vmlinuz-eclipse
    boot
}

menuentry "Eclipse OS (Modo Seguro)" {
    echo "Cargando Eclipse OS en modo seguro..."
    multiboot /boot/vmlinuz-eclipse safe
    boot
}

menuentry "Eclipse OS (Información del Hardware)" {
    echo "Cargando Eclipse OS con información del hardware..."
    multiboot /boot/vmlinuz-eclipse hardware-info
    boot
}

menuentry "Eclipse OS (Resolución VGA)" {
    echo "Cargando Eclipse OS con resolución VGA estándar..."
    multiboot /boot/vmlinuz-eclipse vga=1024x768
    boot
}
EOF

# Crear ISO booteable
echo "📦 Creando ISO booteable para hardware real..."
grub-mkrescue -o eclipse-os-real-hardware.iso real-hardware-iso/

echo "✅ ISO creada: eclipse-os-real-hardware.iso"
ls -lh eclipse-os-real-hardware.iso

echo ""
echo "🖥️  Para usar en hardware real:"
echo "1. Grabar la ISO en un CD/DVD o USB"
echo "2. Configurar la BIOS para arrancar desde CD/DVD o USB"
echo "3. El sistema Eclipse OS se ejecutará automáticamente"
echo ""
echo "💾 Para grabar en USB:"
echo "sudo dd if=eclipse-os-real-hardware.iso of=/dev/sdX bs=4M status=progress"
echo ""
echo "📱 Características del sistema para hardware real:"
echo "  • 🖥️  Kernel optimizado para hardware físico"
echo "  • 🎨 VGA 1024x768 @ 32bpp"
echo "  • 📝 Fuente 16x16 de alta calidad"
echo "  • 🌈 Gradientes y efectos visuales"
echo "  • 📊 Barras de progreso animadas"
echo "  • 🎯 Logo de Eclipse OS"
echo "  • 💻 Interfaz gráfica moderna"
echo "  • 🔧 Compatible con hardware real"
echo ""
echo "⚠️  IMPORTANTE: Esta ISO está diseñada para hardware real"
echo "   No requiere emulación ni virtualización"



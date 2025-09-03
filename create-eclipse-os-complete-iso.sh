#!/bin/bash

# Script para crear ISO con sistema Eclipse OS completo integrado
set -e

echo "🌙 Creando ISO con sistema Eclipse OS completo integrado..."

# Limpiar directorios anteriores
rm -rf eclipse-complete-iso
mkdir -p eclipse-complete-iso/boot/grub

# Copiar el kernel de Eclipse OS completo
if [ -f "eclipse-os-kernel.elf" ]; then
    cp eclipse-os-kernel.elf eclipse-complete-iso/boot/vmlinuz-eclipse
    chmod +x eclipse-complete-iso/boot/vmlinuz-eclipse
    echo "✅ Kernel de Eclipse OS completo copiado"
else
    echo "❌ Kernel de Eclipse OS completo no encontrado"
    exit 1
fi

# Crear configuración GRUB
cat > eclipse-complete-iso/boot/grub/grub.cfg << 'EOF'
# Configuración GRUB para Eclipse OS Completo
set timeout=10
set default=0

menuentry "Eclipse OS - Sistema Completo" {
    echo "Cargando Eclipse OS - Sistema Operativo Completo..."
    multiboot /boot/vmlinuz-eclipse
    boot
}

menuentry "Eclipse OS (Modo Debug)" {
    echo "Cargando Eclipse OS en modo debug..."
    multiboot /boot/vmlinuz-eclipse debug
    boot
}

menuentry "Eclipse OS (Información del Sistema)" {
    echo "Cargando Eclipse OS con información del sistema..."
    multiboot /boot/vmlinuz-eclipse info
    boot
}
EOF

# Crear ISO
echo "📦 Creando ISO con sistema Eclipse OS completo..."
grub-mkrescue -o eclipse-os-complete-system.iso eclipse-complete-iso/

echo "✅ ISO creada: eclipse-os-complete-system.iso"
ls -lh eclipse-os-complete-system.iso

echo ""
echo "🚀 Para probar el sistema Eclipse OS completo:"
echo "qemu-system-x86_64 -cdrom eclipse-os-complete-system.iso -m 512M -display gtk"
echo ""
echo "📱 Características del sistema Eclipse OS:"
echo "  • 🌙 Kernel compatible con Multiboot"
echo "  • 📱 Aplicaciones integradas (Editor, Explorador, Calculadora, Navegador)"
echo "  • 🔐 Sistema de autenticación completo"
echo "  • 🌐 Red y conectividad"
echo "  • 💻 Interfaz gráfica moderna"
echo "  • 🎯 Sistema completamente funcional"
echo ""
echo "💡 El sistema se ejecuta automáticamente al arrancar la ISO"



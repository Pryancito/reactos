#!/bin/bash

# Script para probar Eclipse OS con interfaz gráfica mejorada
set -e

echo "🌙 Probando Eclipse OS con interfaz gráfica mejorada..."
echo ""

# Verificar que la ISO existe
if [ ! -f "eclipse-os-complete-system.iso" ]; then
    echo "❌ ISO no encontrada. Ejecuta primero create-eclipse-os-complete-iso.sh"
    exit 1
fi

echo "✅ ISO encontrada: eclipse-os-complete-system.iso"
echo ""

echo "🚀 Iniciando Eclipse OS con interfaz gráfica..."
echo "📱 Características visuales mejoradas:"
echo "  • 🖥️  Framebuffer de alta resolución (1024x768)"
echo "  • 🎨 Colores RGB de 24 bits"
echo "  • 📝 Fuente personalizada 8x8"
echo "  • 🌈 Gradientes y efectos visuales"
echo "  • 📊 Barras de progreso animadas"
echo "  • 🎯 Logo de Eclipse OS"
echo "  • 💻 Interfaz gráfica moderna"
echo ""

echo "🎮 Controles:"
echo "  • Ctrl+Alt+Q: Salir de QEMU"
echo "  • Ctrl+Alt+F: Pantalla completa"
echo "  • Ctrl+Alt+G: Capturar/liberar mouse"
echo ""

echo "⏱️  Iniciando en 3 segundos..."
sleep 3

# Ejecutar QEMU con configuración optimizada para gráficos
qemu-system-x86_64 \
    -cdrom eclipse-os-complete-system.iso \
    -m 512M \
    -vga std \
    -display gtk \
    -no-reboot \
    -name "Eclipse OS - Sistema Completo"



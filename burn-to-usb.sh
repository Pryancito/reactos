#!/bin/bash

# Script para grabar Eclipse OS en USB para hardware real
set -e

echo "💾 Grabando Eclipse OS en USB para hardware real..."
echo ""

# Verificar que la ISO existe
if [ ! -f "eclipse-os-real-hardware.iso" ]; then
    echo "❌ ISO no encontrada. Ejecuta primero create-real-hardware-iso.sh"
    exit 1
fi

echo "✅ ISO encontrada: eclipse-os-real-hardware.iso"
echo ""

# Mostrar dispositivos USB disponibles
echo "🔍 Dispositivos USB disponibles:"
lsblk | grep -E "(sd[a-z]|mmcblk[0-9])" | grep -v loop
echo ""

# Solicitar dispositivo USB
echo "⚠️  IMPORTANTE: Selecciona el dispositivo USB correcto"
echo "   Ejemplo: /dev/sdb, /dev/sdc, /dev/mmcblk0"
echo "   NO uses /dev/sda (disco del sistema)"
echo ""
read -p "Ingresa el dispositivo USB (ej: /dev/sdb): " usb_device

# Verificar que el dispositivo existe
if [ ! -b "$usb_device" ]; then
    echo "❌ Dispositivo $usb_device no encontrado"
    exit 1
fi

# Mostrar información del dispositivo
echo ""
echo "📱 Información del dispositivo:"
lsblk "$usb_device"
echo ""

# Confirmar grabación
echo "⚠️  ADVERTENCIA: Esto borrará TODOS los datos en $usb_device"
read -p "¿Continuar? (s/N): " confirm

if [[ ! "$confirm" =~ ^[Ss]$ ]]; then
    echo "❌ Operación cancelada"
    exit 1
fi

# Desmontar particiones si están montadas
echo "🔧 Desmontando particiones..."
sudo umount "$usb_device"* 2>/dev/null || true

# Grabar la ISO
echo "📦 Grabando Eclipse OS en USB..."
echo "   Dispositivo: $usb_device"
echo "   ISO: eclipse-os-real-hardware.iso"
echo "   Tamaño: $(ls -lh eclipse-os-real-hardware.iso | awk '{print $5}')"
echo ""

sudo dd if=eclipse-os-real-hardware.iso of="$usb_device" bs=4M status=progress

echo ""
echo "✅ Eclipse OS grabado exitosamente en USB"
echo ""
echo "🖥️  Para usar en hardware real:"
echo "1. Inserta el USB en el equipo objetivo"
echo "2. Configura la BIOS para arrancar desde USB"
echo "3. El sistema Eclipse OS se ejecutará automáticamente"
echo ""
echo "📱 Características del sistema:"
echo "  • 🖥️  Kernel optimizado para hardware físico"
echo "  • 🎨 VGA 1024x768 @ 32bpp"
echo "  • 📝 Fuente 16x16 de alta calidad"
echo "  • 🌈 Gradientes y efectos visuales"
echo "  • 📊 Barras de progreso animadas"
echo "  • 🎯 Logo de Eclipse OS"
echo "  • 💻 Interfaz gráfica moderna"
echo ""
echo "🎯 ¡Eclipse OS listo para hardware real!"



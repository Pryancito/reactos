#!/bin/bash

# ReactOS Rust OS - Setup Bootloader
# Script para configurar el bootloader GRUB

set -e

echo "🔧 Configurando bootloader GRUB..."

# Crear directorios necesarios
mkdir -p output/iso/boot/grub

# Crear archivo de configuración de GRUB
cat > output/iso/boot/grub/grub.cfg << 'EOF'
set timeout=10
set default=0

menuentry "ReactOS Rust OS" {
    multiboot /boot/reactos-rust-kernel.bin
    module /boot/ntdll.dll
    module /boot/kernel32.dll
    module /boot/user32.dll
    module /boot/gdi32.dll
    module /boot/calc.exe
}
EOF

# Crear archivo de configuración de GRUB para ISO
cat > output/iso/boot/grub/stage2_eltorito << 'EOF'
# Este archivo será reemplazado por el bootloader real de GRUB
# Por ahora es solo un placeholder
EOF

echo "✅ Bootloader configurado"

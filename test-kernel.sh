#!/bin/bash

echo "🧪 Probando kernel de Eclipse OS..."

# Crear directorio de prueba
rm -rf test-iso
mkdir -p test-iso/boot/grub

# Copiar kernel
cp working-eclipse-kernel.elf test-iso/boot/vmlinuz-eclipse

# Crear configuración GRUB simple
cat > test-iso/boot/grub/grub.cfg << 'EOF'
set timeout=5
set default=0

menuentry "Eclipse OS Test" {
    multiboot /boot/vmlinuz-eclipse
    boot
}
EOF

# Crear ISO
grub-mkrescue -o test-eclipse-os.iso test-iso/

echo "✅ ISO de prueba creada: test-eclipse-os.iso"
echo "🚀 Para probar:"
echo "qemu-system-x86_64 -cdrom test-eclipse-os.iso -m 512M"

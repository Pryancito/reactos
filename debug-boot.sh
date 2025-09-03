#!/bin/bash

echo "🔍 Debugging Eclipse OS Boot..."

# Crear directorio de debug
rm -rf debug-iso
mkdir -p debug-iso/boot/grub

# Copiar kernel
cp fixed-eclipse-kernel.elf debug-iso/boot/vmlinuz-eclipse

# Crear configuración GRUB con debug
cat > debug-iso/boot/grub/grub.cfg << 'EOF'
set timeout=10
set default=0

menuentry "Eclipse OS Debug" {
    echo "Loading Eclipse OS Kernel..."
    multiboot /boot/vmlinuz-eclipse
    echo "Kernel loaded, booting..."
    boot
}

menuentry "GRUB Command Line" {
    echo "Entering GRUB command line..."
    terminal_output console
    terminal_input console
}
EOF

# Crear ISO
grub-mkrescue -o debug-eclipse-os.iso debug-iso/

echo "✅ ISO de debug creada: debug-eclipse-os.iso"
echo "🚀 Para probar con debug:"
echo "qemu-system-x86_64 -cdrom debug-eclipse-os.iso -m 512M -serial stdio -nographic"
echo ""
echo "🔍 Para ver el proceso de boot paso a paso:"
echo "qemu-system-x86_64 -cdrom debug-eclipse-os.iso -m 512M -display gtk"

#!/bin/bash

# Script para crear una ISO completa con Eclipse OS
set -e

echo "🌙 Creando ISO completa con Eclipse OS..."

# Limpiar directorios anteriores
rm -rf complete-iso
mkdir -p complete-iso/boot/grub
mkdir -p complete-iso/bin
mkdir -p complete-iso/sbin
mkdir -p complete-iso/usr/bin
mkdir -p complete-iso/etc
mkdir -p complete-iso/var/log
mkdir -p complete-iso/tmp
mkdir -p complete-iso/home/user

# Copiar el kernel de Eclipse OS
if [ -f "fixed-eclipse-kernel.elf" ]; then
    cp fixed-eclipse-kernel.elf complete-iso/boot/vmlinuz-eclipse
    chmod +x complete-iso/boot/vmlinuz-eclipse
    echo "✅ Kernel de Eclipse OS copiado"
else
    echo "❌ Kernel de Eclipse OS no encontrado"
    exit 1
fi

# Copiar el binario eclipse-os
if [ -f "target/release/eclipse-os" ]; then
    cp target/release/eclipse-os complete-iso/eclipse-os
    chmod +x complete-iso/eclipse-os
    echo "✅ Binario eclipse-os copiado"
else
    echo "❌ Binario eclipse-os no encontrado"
    exit 1
fi

# Crear /init que ejecuta eclipse-os
cat > complete-iso/init << 'EOF'
#!/bin/sh
echo "🌙 Eclipse OS: Iniciando sistema completo..."
if [ -x /eclipse-os ]; then
    echo "🚀 Ejecutando Eclipse OS completo..."
    exec /eclipse-os
else
    echo "❌ /eclipse-os no encontrado"
    exit 1
fi
EOF

chmod +x complete-iso/init

# Crear enlaces simbólicos
mkdir -p complete-iso/bin
mkdir -p complete-iso/sbin
ln -sf /init complete-iso/bin/init
ln -sf /init complete-iso/sbin/init
ln -sf /eclipse-os complete-iso/bin/eclipse-os

# Crear configuración GRUB
cat > complete-iso/boot/grub/grub.cfg << 'EOF'
# Configuración GRUB para Eclipse OS Completo
set timeout=10
set default=0

menuentry "Eclipse OS Completo" {
    echo "Cargando Eclipse OS Kernel..."
    multiboot /boot/vmlinuz-eclipse
    echo "Cargando initrd con Eclipse OS completo..."
    module /initrd.img
    boot
}

menuentry "Eclipse OS (Solo Kernel)" {
    echo "Cargando solo el kernel de Eclipse OS..."
    multiboot /boot/vmlinuz-eclipse
    boot
}

menuentry "Eclipse OS (Debug)" {
    echo "Cargando Eclipse OS en modo debug..."
    multiboot /boot/vmlinuz-eclipse debug
    module /initrd.img
    boot
}
EOF

# Crear initrd con el sistema completo
echo "📦 Creando initrd con Eclipse OS completo..."
mkdir -p initrd-temp
cp complete-iso/init initrd-temp/
cp complete-iso/eclipse-os initrd-temp/
cp complete-iso/bin/init initrd-temp/bin/
cp complete-iso/sbin/init initrd-temp/sbin/
cp complete-iso/bin/eclipse-os initrd-temp/bin/

# Crear directorios en initrd
mkdir -p initrd-temp/bin
mkdir -p initrd-temp/sbin
mkdir -p initrd-temp/usr/bin
mkdir -p initrd-temp/etc
mkdir -p initrd-temp/var/log
mkdir -p initrd-temp/tmp
mkdir -p initrd-temp/home/user

# Crear initrd
cd initrd-temp
find . | cpio -o -H newc | gzip > ../complete-iso/initrd.img
cd ..
rm -rf initrd-temp

# Crear ISO
echo "📦 Creando ISO completa..."
grub-mkrescue -o eclipse-os-complete.iso complete-iso/

echo "✅ ISO completa creada: eclipse-os-complete.iso"
ls -lh eclipse-os-complete.iso

echo ""
echo "🚀 Para probar la ISO completa:"
echo "qemu-system-x86_64 -cdrom eclipse-os-complete.iso -m 1G -display gtk"
echo ""
echo "🔧 Para probar solo el kernel:"
echo "qemu-system-x86_64 -cdrom eclipse-os-complete.iso -m 512M -display gtk"

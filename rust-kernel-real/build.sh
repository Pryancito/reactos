#!/bin/bash

set -e

echo "🔧 Compilando Kernel Rust para Hardware Real..."

# Crear directorio de salida
mkdir -p target/release

# Compilar kernel Rust
echo "📦 Compilando kernel Rust..."
cargo build --release --target x86_64-unknown-none

# Crear directorio ISO
echo "📁 Creando estructura ISO..."
mkdir -p iso/boot/grub

# Copiar kernel
echo "📋 Copiando kernel..."
cp target/x86_64-unknown-none/release/eclipse-kernel-rust iso/boot/vmlinuz-eclipse-rust

# Crear configuración GRUB
echo "⚙️  Configurando GRUB..."
cat > iso/boot/grub/grub.cfg << 'EOF'
set timeout=5
set default=0

menuentry "Eclipse OS Rust Kernel" {
    multiboot /boot/vmlinuz-eclipse-rust
    boot
}

menuentry "Eclipse OS Rust Kernel (VGA Fallback)" {
    multiboot /boot/vmlinuz-eclipse-rust vga=0x317
    boot
}

menuentry "Eclipse OS Rust Kernel (Serial Console)" {
    multiboot /boot/vmlinuz-eclipse-rust console=ttyS0,115200
    boot
}
EOF

# Crear ISO
echo "💿 Creando ISO..."
grub-mkrescue -o eclipse-os-rust-kernel.iso iso/

echo "✅ ISO creada: eclipse-os-rust-kernel.iso"
echo "🚀 Listo para hardware real!"

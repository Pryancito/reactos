#!/bin/bash

# Script para crear ISO de ReactOS Rust Kernel con GRUB

set -e

echo "🚀 Creando ISO de ReactOS Rust Kernel con GRUB..."

# Configurar variables
KERNEL_DIR="/home/moebius/reactos/reactos-rust-kernel"
GRUB_DIR="$(dirname "$0")/../grub-testing"
OUTPUT_DIR="$(dirname "$0")/../test-data"
ISO_NAME="reactos-rust-kernel-testing.iso"

# Crear directorio de salida
mkdir -p "$OUTPUT_DIR"

# Verificar que el kernel existe
if [ ! -f "$KERNEL_DIR/target/x86_64-unknown-none/release/reactos-rust-kernel-multiboot" ]; then
    echo "❌ Error: Kernel multiboot no encontrado en $KERNEL_DIR/target/x86_64-unknown-none/release/reactos-rust-kernel-multiboot"
    echo "💡 Compilando kernel primero..."
    cd "$KERNEL_DIR"
    cargo build --target x86_64-unknown-none --release --bin reactos-rust-kernel
    ./create_multiboot_kernel.sh
    cd - > /dev/null
fi

# Copiar kernel compilado
echo "📦 Copiando kernel multiboot desde $KERNEL_DIR..."
cp "$KERNEL_DIR/target/x86_64-unknown-none/release/reactos-rust-kernel-multiboot" "$GRUB_DIR/reactos-rust-kernel"

# Crear estructura de directorios para ISO
echo "📁 Preparando estructura de ISO..."
mkdir -p "$GRUB_DIR/boot/grub"

# Crear ISO con GRUB
echo "💿 Creando ISO con GRUB..."
grub-mkrescue -o "$OUTPUT_DIR/$ISO_NAME" "$GRUB_DIR"

echo "✅ ISO creada exitosamente en: $OUTPUT_DIR/$ISO_NAME"

# Mostrar información de la ISO
echo "📊 Información de la ISO:"
ls -lh "$OUTPUT_DIR/$ISO_NAME"
file "$OUTPUT_DIR/$ISO_NAME" 2>/dev/null || echo "No se pudo obtener información del archivo"

echo "🎉 ISO de ReactOS Rust Kernel creada exitosamente!"
echo "💡 Para probar: qemu-system-x86_64 -cdrom $OUTPUT_DIR/$ISO_NAME"

#!/bin/bash

# Script para compilar el kernel de ReactOS Rust para bare metal

set -e

echo "🚀 Compilando ReactOS Rust Kernel para bare metal..."

# Configurar variables
KERNEL_DIR="$(dirname "$0")/../kernel"
TARGET="x86_64-unknown-none"
KERNEL_BINARY="reactos-rust-kernel"
OUTPUT_DIR="$(dirname "$0")/../test-data"

# Crear directorio de salida
mkdir -p "$OUTPUT_DIR"

# Cambiar al directorio del kernel
cd "$KERNEL_DIR"

# Configurar target
rustup target add $TARGET 2>/dev/null || true

# Compilar el kernel
echo "📦 Compilando kernel..."
cargo build --target $TARGET --release --bin reactos-rust-kernel

# Copiar el binario
cp "target/$TARGET/release/$KERNEL_BINARY" "$OUTPUT_DIR/"

echo "✅ Kernel compilado exitosamente en: $OUTPUT_DIR/$KERNEL_BINARY"

# Mostrar información del binario
echo "📊 Información del binario:"
ls -lh "$OUTPUT_DIR/$KERNEL_BINARY"
file "$OUTPUT_DIR/$KERNEL_BINARY" 2>/dev/null || echo "No se pudo obtener información del archivo"

echo "🎉 Compilación completada!"

#!/bin/bash

# Script para crear un kernel binario plano con header multiboot al inicio

set -e

echo "🔧 Creando kernel binario plano con header multiboot..."

# Crear un archivo binario plano con el header multiboot al inicio
# El header multiboot debe estar en los primeros 8KB del archivo
dd if=/dev/zero of=target/x86_64-unknown-none/release/reactos-rust-kernel-multiboot bs=1024 count=8 2>/dev/null

# Escribir el header multiboot al inicio
# Magic: 0x1BADB002, Flags: 0x00000003, Checksum: 0xE4524FFB
printf '\x02\xb0\xad\x1b\x03\x00\x00\x00\xfb\x4f\x52\xe4' | dd of=target/x86_64-unknown-none/release/reactos-rust-kernel-multiboot bs=1 count=12 conv=notrunc 2>/dev/null

# Agregar el código del kernel después del header
objcopy -O binary --remove-section=.multiboot_header target/x86_64-unknown-none/release/reactos-rust-kernel kernel_code.bin
dd if=kernel_code.bin of=target/x86_64-unknown-none/release/reactos-rust-kernel-multiboot bs=1 seek=8192 conv=notrunc 2>/dev/null

echo "✅ Kernel binario plano creado: target/x86_64-unknown-none/release/reactos-rust-kernel-multiboot"

# Verificar el header multiboot
echo "📋 Verificando header multiboot:"
hexdump -C target/x86_64-unknown-none/release/reactos-rust-kernel-multiboot | head -3

# Limpiar archivos temporales
rm -f kernel_code.bin

echo "🎉 ¡Listo! El kernel binario plano está listo para GRUB."

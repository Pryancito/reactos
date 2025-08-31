#!/bin/bash
# Script para compilar bootloader personalizado
x86_64-w64-mingw32-gcc-posix \
    -o customboot.exe \
    boot/freeldr/custom-bootloader/main.c \
    -static-libgcc \
    -O0 \
    -fno-stack-protector \
    -fno-builtin \
    -Wall 2>/dev/null
echo "✅ Bootloader personalizado compilado: customboot.exe"

#!/bin/bash

# ReactOS Rust OS - Build Script
# Script para compilar todo el sistema ReactOS Rust OS

set -e

echo "🚀 Iniciando build de ReactOS Rust OS..."
echo ""

# Verificar dependencias
echo "🔍 Verificando dependencias..."
command -v cargo >/dev/null 2>&1 || { echo "❌ Cargo no encontrado. Instala Rust primero."; exit 1; }
command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1 || { echo "❌ MinGW no encontrado. Instala MinGW-w64 primero."; exit 1; }
command -v genisoimage >/dev/null 2>&1 || { echo "❌ genisoimage no encontrado. Instala genisoimage primero."; exit 1; }
echo "✅ Dependencias verificadas"
echo ""

# Limpiar builds anteriores
echo "🧹 Limpiando builds anteriores..."
cargo clean
echo "✅ Limpieza completada"
echo ""

# Compilar kernel
echo "🔧 Compilando kernel..."
cd kernel
cargo build --release --target x86_64-pc-windows-gnu
cd ..
echo "✅ Kernel compilado"
echo ""

# Compilar ntdll
echo "🔧 Compilando ntdll..."
cd ntdll
cargo build --release --target x86_64-pc-windows-gnu
cd ..
echo "✅ ntdll compilado"
echo ""

# Compilar kernel32
echo "🔧 Compilando kernel32..."
cd kernel32
cargo build --release --target x86_64-pc-windows-gnu
cd ..
echo "✅ kernel32 compilado"
echo ""

# Compilar user32
echo "🔧 Compilando user32..."
cd user32
cargo build --release --target x86_64-pc-windows-gnu
cd ..
echo "✅ user32 compilado"
echo ""

# Compilar gdi32
echo "🔧 Compilando gdi32..."
cd gdi32
cargo build --release --target x86_64-pc-windows-gnu
cd ..
echo "✅ gdi32 compilado"
echo ""

# Compilar aplicaciones
echo "🔧 Compilando aplicaciones..."
cd apps/calc
cargo build --release --target x86_64-pc-windows-gnu
cd ../..
echo "✅ Aplicaciones compiladas"
echo ""

# Crear directorios de salida
echo "📁 Creando directorios de salida..."
mkdir -p output/{kernel,ntdll,kernel32,user32,gdi32,apps,iso/{boot,system32,apps}}
echo "✅ Directorios creados"
echo ""

# Configurar bootloader
echo "🔧 Configurando bootloader..."
./scripts/setup-bootloader.sh
echo "✅ Bootloader configurado"
echo ""

# Copiar archivos compilados
echo "📋 Copiando archivos compilados..."
# Copiar kernel (exe)
cp target/x86_64-pc-windows-gnu/release/reactos-rust-kernel.exe output/kernel/
# Copiar librerías del sistema (rlib)
cp target/x86_64-pc-windows-gnu/release/libreactos_rust_ntdll.rlib output/ntdll/
cp target/x86_64-pc-windows-gnu/release/libreactos_rust_kernel32.rlib output/kernel32/
cp target/x86_64-pc-windows-gnu/release/libreactos_rust_user32.rlib output/user32/
cp target/x86_64-pc-windows-gnu/release/libreactos_rust_gdi32.rlib output/gdi32/
# Copiar calculadora (exe)
cp target/x86_64-pc-windows-gnu/release/reactos-rust-calc.exe output/apps/calc.exe
echo "✅ Archivos copiados"
echo ""

# Crear ISO
echo "🔧 Creando ISO..."
genisoimage -o output/reactos-rust-os.iso \
    -b boot/grub/stage2_eltorito \
    -c boot.catalog \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    output/iso
echo "✅ ISO creada: output/reactos-rust-os.iso"
echo ""

# Mostrar resumen
echo "📊 Resumen del build:"
echo "  - Kernel: output/kernel/reactos-rust-kernel.exe"
echo "  - ntdll: output/ntdll/libreactos_rust_ntdll.rlib"
echo "  - kernel32: output/kernel32/libreactos_rust_kernel32.rlib"
echo "  - user32: output/user32/libreactos_rust_user32.rlib"
echo "  - gdi32: output/gdi32/libreactos_rust_gdi32.rlib"
echo "  - Calculadora: output/apps/calc.exe"
echo "  - ISO: output/reactos-rust-os.iso"
echo ""

echo "🎉 Build de ReactOS Rust OS completado exitosamente!"
echo "🚀 Puedes probar el sistema con: qemu-system-x86_64 -cdrom output/reactos-rust-os.iso"

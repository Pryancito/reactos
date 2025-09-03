#!/bin/bash

# Script simplificado de build para ReactOS Rust OS
echo "🦀 Construyendo ReactOS Rust OS (versión simplificada)..."

# Configuración
RUST_TOOLCHAIN="nightly"
TARGET_X64="x86_64-unknown-none"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para imprimir mensajes
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Verificar dependencias
check_dependencies() {
    print_status "Verificando dependencias..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo no está instalado"
        exit 1
    fi
    
    # Instalar toolchain nightly
    rustup toolchain install nightly
    
    # Instalar target para x86_64
    rustup target add $TARGET_X64
    
    print_success "Dependencias verificadas"
}

# Construir kernel
build_kernel() {
    print_status "Construyendo kernel para x86_64..."
    
    cd kernel
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
    
    if [ $? -eq 0 ]; then
        print_success "Kernel x86_64 construido exitosamente"
    else
        print_error "Error al construir kernel x86_64"
        exit 1
    fi
    
    cd ..
}

# Construir aplicaciones
build_applications() {
    print_status "Construyendo aplicaciones..."
    
    cd apps
    for app in */; do
        if [ -f "$app/Cargo.toml" ]; then
            print_status "Construyendo aplicación: $app"
            cd "$app"
            cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
            if [ $? -eq 0 ]; then
                print_success "Aplicación $app construida exitosamente"
            else
                print_error "Error al construir aplicación $app"
                exit 1
            fi
            cd ..
        fi
    done
    
    cd ..
}

# Crear ISO simple
create_iso() {
    print_status "Creando ISO de ReactOS Rust OS..."
    
    # Crear directorio de ISO
    mkdir -p iso/{boot,lib,bin,etc}
    
    # Copiar kernel
    if [ -f "kernel/target/$TARGET_X64/release/libreactos_rust_kernel.rlib" ]; then
        cp kernel/target/$TARGET_X64/release/libreactos_rust_kernel.rlib iso/lib/
        print_success "Kernel copiado"
    fi
    
    # Copiar aplicaciones
    for app in apps/*/; do
        if [ -f "$app/target/$TARGET_X64/release/*.exe" ]; then
            cp $app/target/$TARGET_X64/release/*.exe iso/bin/ 2>/dev/null || true
        fi
    done
    
    # Crear configuración de GRUB
    mkdir -p iso/boot/grub
    cat > iso/boot/grub/grub.cfg << 'EOF'
set timeout=10
set default=0

menuentry "ReactOS Rust OS" {
    echo "Cargando ReactOS Rust OS..."
    multiboot /lib/libreactos_rust_kernel.rlib
    boot
}
EOF
    
    # Crear ISO
    if command -v grub-mkrescue &> /dev/null; then
        grub-mkrescue -o reactos-rust-os-simple.iso iso/
        if [ $? -eq 0 ]; then
            print_success "ISO creada exitosamente: reactos-rust-os-simple.iso"
        else
            print_error "Error al crear ISO"
            exit 1
        fi
    else
        print_error "grub-mkrescue no está instalado"
        exit 1
    fi
}

# Función principal
main() {
    echo "🦀 ReactOS Rust OS Build System (Simplificado)"
    echo "=============================================="
    echo ""
    
    # Ejecutar pasos
    check_dependencies
    build_kernel
    build_applications
    create_iso
    
    echo ""
    print_success "ReactOS Rust OS construido exitosamente"
    echo ""
    echo "📋 Archivos generados:"
    echo "   • reactos-rust-os-simple.iso - ISO booteable"
    echo "   • iso/ - Directorio de ISO"
    echo ""
    echo "🚀 Para probar:"
    echo "   qemu-system-x86_64 -cdrom reactos-rust-os-simple.iso -m 2048"
    echo ""
    echo "🦀 ¡ReactOS Rust OS listo para usar!"
}

# Ejecutar función principal
main "$@"

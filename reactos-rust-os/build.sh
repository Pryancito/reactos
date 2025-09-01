#!/bin/bash

# Script de build para ReactOS Rust OS
echo "🦀 Construyendo ReactOS Rust OS..."

# Configuración - Multi-arquitectura por defecto
RUST_TOOLCHAIN="nightly"
TARGET_X86="i686-unknown-none"
TARGET_X64="x86_64-unknown-none"
BUILD_X86=true
BUILD_X64=true
MULTI_ARCH=true

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

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Verificar dependencias
check_dependencies() {
    print_status "Verificando dependencias..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo no está instalado. Instalando Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    if ! command -v rustup &> /dev/null; then
        print_error "Rustup no está instalado"
        exit 1
    fi
    
    # Instalar toolchain nightly
    rustup toolchain install nightly
    
    # Instalar targets para ambas arquitecturas (multi-arquitectura por defecto)
    print_status "Instalando targets para multi-arquitectura..."
    rustup target add $TARGET_X86
    rustup target add $TARGET_X64
    
    print_success "Targets instalados: $TARGET_X86 y $TARGET_X64"
    
    print_success "Dependencias verificadas"
}

# Construir bootloader
build_bootloader() {
    print_status "Construyendo bootloader multi-arquitectura..."
    
    cd bootloader
    
    # Construir para x86 (32-bit) - Multi-arquitectura por defecto
    print_status "Construyendo bootloader para x86 (32-bit)..."
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X86 --release
    
    if [ $? -eq 0 ]; then
        print_success "Bootloader x86 construido exitosamente"
    else
        print_error "Error al construir bootloader x86"
        exit 1
    fi
    
    # Construir para x86_64 (64-bit) - Multi-arquitectura por defecto
    print_status "Construyendo bootloader para x86_64 (64-bit)..."
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
    
    if [ $? -eq 0 ]; then
        print_success "Bootloader x86_64 construido exitosamente"
    else
        print_error "Error al construir bootloader x86_64"
        exit 1
    fi
    
    cd ..
}

# Construir kernel
build_kernel() {
    print_status "Construyendo kernel multi-arquitectura..."
    
    cd kernel
    
    # Construir para x86 (32-bit) - Multi-arquitectura por defecto
    print_status "Construyendo kernel para x86 (32-bit)..."
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X86 --release
    
    if [ $? -eq 0 ]; then
        print_success "Kernel x86 construido exitosamente"
    else
        print_error "Error al construir kernel x86"
        exit 1
    fi
    
    # Construir para x86_64 (64-bit) - Multi-arquitectura por defecto
    print_status "Construyendo kernel para x86_64 (64-bit)..."
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
    
    if [ $? -eq 0 ]; then
        print_success "Kernel x86_64 construido exitosamente"
    else
        print_error "Error al construir kernel x86_64"
        exit 1
    fi
    
    cd ..
}

# Construir userland
build_userland() {
    print_status "Construyendo userland..."
    
    cd userland
    cargo +$RUST_TOOLCHAIN build --target $TARGET --release
    
    if [ $? -eq 0 ]; then
        print_success "Userland construido exitosamente"
    else
        print_error "Error al construir userland"
        exit 1
    fi
    
    cd ..
}

# Construir drivers
build_drivers() {
    print_status "Construyendo drivers..."
    
    cd drivers
    for driver in */; do
        if [ -f "$driver/Cargo.toml" ]; then
            print_status "Construyendo driver: $driver"
            cd "$driver"
            cargo +$RUST_TOOLCHAIN build --target $TARGET --release
            if [ $? -eq 0 ]; then
                print_success "Driver $driver construido exitosamente"
            else
                print_error "Error al construir driver $driver"
                exit 1
            fi
            cd ..
        fi
    done
    
    cd ..
}

# Construir servicios
build_services() {
    print_status "Construyendo servicios..."
    
    cd services
    for service in */; do
        if [ -f "$service/Cargo.toml" ]; then
            print_status "Construyendo servicio: $service"
            cd "$service"
            cargo +$RUST_TOOLCHAIN build --target $TARGET --release
            if [ $? -eq 0 ]; then
                print_success "Servicio $service construido exitosamente"
            else
                print_error "Error al construir servicio $service"
                exit 1
            fi
            cd ..
        fi
    done
    
    cd ..
}

# Construir GUI
build_gui() {
    print_status "Construyendo GUI..."
    
    cd gui
    if [ -f "Cargo.toml" ]; then
        cargo +$RUST_TOOLCHAIN build --target $TARGET --release
        if [ $? -eq 0 ]; then
            print_success "GUI construida exitosamente"
        else
            print_error "Error al construir GUI"
            exit 1
        fi
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
            cargo +$RUST_TOOLCHAIN build --target $TARGET --release
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

# Crear ISO
create_iso() {
    print_status "Creando ISO multi-arquitectura de ReactOS Rust OS..."
    
    # Crear directorio de ISO
    mkdir -p iso/{boot,lib,bin,etc,usr,arch/{x86,x86_64}}
    
    # Copiar archivos construidos para x86 (32-bit)
    print_status "Copiando archivos para x86 (32-bit)..."
    cp bootloader/target/$TARGET_X86/release/bootloader iso/arch/x86/
    cp kernel/target/$TARGET_X86/release/kernel iso/arch/x86/
    cp userland/target/$TARGET_X86/release/userland iso/arch/x86/
    
    # Copiar archivos construidos para x86_64 (64-bit)
    print_status "Copiando archivos para x86_64 (64-bit)..."
    cp bootloader/target/$TARGET_X64/release/bootloader iso/arch/x86_64/
    cp kernel/target/$TARGET_X64/release/kernel iso/arch/x86_64/
    cp userland/target/$TARGET_X64/release/userland iso/arch/x86_64/
    
    # Copiar drivers para ambas arquitecturas
    cp -r drivers/*/target/$TARGET_X86/release/*.a iso/arch/x86/ 2>/dev/null || true
    cp -r drivers/*/target/$TARGET_X64/release/*.a iso/arch/x86_64/ 2>/dev/null || true
    
    # Copiar servicios para ambas arquitecturas
    cp -r services/*/target/$TARGET_X86/release/*.a iso/arch/x86/ 2>/dev/null || true
    cp -r services/*/target/$TARGET_X64/release/*.a iso/arch/x86_64/ 2>/dev/null || true
    
    # Copiar GUI para ambas arquitecturas
    cp -r gui/target/$TARGET_X86/release/*.a iso/arch/x86/ 2>/dev/null || true
    cp -r gui/target/$TARGET_X64/release/*.a iso/arch/x86_64/ 2>/dev/null || true
    
    # Copiar aplicaciones para ambas arquitecturas
    cp -r apps/*/target/$TARGET_X86/release/*.a iso/arch/x86/ 2>/dev/null || true
    cp -r apps/*/target/$TARGET_X64/release/*.a iso/arch/x86_64/ 2>/dev/null || true
    
    # Crear configuración de GRUB para multi-arquitectura
    cat > iso/boot/grub/grub.cfg << 'EOF'
set timeout=10
set default=0

menuentry "ReactOS Rust OS (x86_64)" {
    echo "Cargando ReactOS Rust OS x86_64..."
    multiboot /arch/x86_64/kernel
    module /arch/x86_64/userland
    boot
}

menuentry "ReactOS Rust OS (x86)" {
    echo "Cargando ReactOS Rust OS x86..."
    multiboot /arch/x86/kernel
    module /arch/x86/userland
    boot
}

menuentry "ReactOS Rust OS (x86_64 Debug)" {
    echo "Cargando ReactOS Rust OS x86_64 en modo debug..."
    multiboot /arch/x86_64/kernel debug
    module /arch/x86_64/userland
    boot
}

menuentry "ReactOS Rust OS (x86 Debug)" {
    echo "Cargando ReactOS Rust OS x86 en modo debug..."
    multiboot /arch/x86/kernel debug
    module /arch/x86/userland
    boot
}
EOF
    
    # Crear ISO
    grub-mkrescue -o reactos-rust-os.iso iso/
    
    if [ $? -eq 0 ]; then
        print_success "ISO creada exitosamente: reactos-rust-os.iso"
    else
        print_error "Error al crear ISO"
        exit 1
    fi
}

# Función principal
main() {
    echo "🦀 ReactOS Rust OS Build System"
    echo "================================"
    echo ""
    
    # Ejecutar pasos
    check_dependencies
    build_bootloader
    build_kernel
    build_userland
    build_drivers
    build_services
    build_gui
    build_applications
    create_iso
    
    echo ""
    print_success "ReactOS Rust OS multi-arquitectura construido exitosamente"
    echo ""
    echo "📋 Archivos generados:"
    echo "   • reactos-rust-os.iso - ISO multi-arquitectura booteable"
    echo "   • iso/ - Directorio de ISO con soporte x86 y x86_64"
    echo "   • iso/arch/x86/ - Archivos para arquitectura x86 (32-bit)"
    echo "   • iso/arch/x86_64/ - Archivos para arquitectura x86_64 (64-bit)"
    echo ""
    echo "🚀 Para probar:"
    echo "   # x86_64 (64-bit):"
    echo "   qemu-system-x86_64 -cdrom reactos-rust-os.iso -m 2048"
    echo "   # x86 (32-bit):"
    echo "   qemu-system-i386 -cdrom reactos-rust-os.iso -m 1024"
    echo ""
    echo "🦀 ¡ReactOS Rust OS multi-arquitectura listo para usar!"
}

# Ejecutar función principal
main "$@"

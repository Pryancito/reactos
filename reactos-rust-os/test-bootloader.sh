#!/bin/bash

# Script de test para el bootloader de ReactOS Rust OS
echo "🦀 Probando bootloader de ReactOS Rust OS..."

# Configuración
RUST_TOOLCHAIN="nightly"
TARGET_X86="i686-unknown-none"
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
        print_error "Cargo no está instalado"
        exit 1
    fi
    
    if ! command -v rustup &> /dev/null; then
        print_error "Rustup no está instalado"
        exit 1
    fi
    
    # Instalar toolchain nightly si no está instalado
    rustup toolchain install nightly
    
    # Instalar targets
    rustup target add $TARGET_X86
    rustup target add $TARGET_X64
    
    print_success "Dependencias verificadas"
}

# Compilar bootloader para x86
compile_bootloader_x86() {
    print_status "Compilando bootloader para x86 (32-bit)..."
    
    cd bootloader
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X86 --release
    
    if [ $? -eq 0 ]; then
        print_success "Bootloader x86 compilado exitosamente"
    else
        print_error "Error al compilar bootloader x86"
        exit 1
    fi
    
    cd ..
}

# Compilar bootloader para x86_64
compile_bootloader_x64() {
    print_status "Compilando bootloader para x86_64 (64-bit)..."
    
    cd bootloader
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
    
    if [ $? -eq 0 ]; then
        print_success "Bootloader x86_64 compilado exitosamente"
    else
        print_error "Error al compilar bootloader x86_64"
        exit 1
    fi
    
    cd ..
}

# Crear ISO de test
create_test_iso() {
    print_status "Creando ISO de test..."
    
    # Crear directorio de ISO
    mkdir -p test-iso/{boot,arch/{x86,x86_64}}
    
    # Copiar bootloaders
    cp bootloader/target/$TARGET_X86/release/bootloader test-iso/arch/x86/
    cp bootloader/target/$TARGET_X64/release/bootloader test-iso/arch/x86_64/
    
    # Crear configuración de GRUB
    cat > test-iso/boot/grub/grub.cfg << 'EOF'
set timeout=10
set default=0

menuentry "ReactOS Rust Bootloader (x86_64)" {
    echo "Cargando bootloader x86_64..."
    multiboot /arch/x86_64/bootloader
    boot
}

menuentry "ReactOS Rust Bootloader (x86)" {
    echo "Cargando bootloader x86..."
    multiboot /arch/x86/bootloader
    boot
}
EOF
    
    # Crear ISO
    grub-mkrescue -o reactos-rust-bootloader-test.iso test-iso/
    
    if [ $? -eq 0 ]; then
        print_success "ISO de test creada: reactos-rust-bootloader-test.iso"
    else
        print_error "Error al crear ISO de test"
        exit 1
    fi
}

# Probar en QEMU
test_qemu() {
    print_status "Probando bootloader en QEMU..."
    
    if command -v qemu-system-x86_64 &> /dev/null; then
        print_status "Ejecutando QEMU x86_64..."
        qemu-system-x86_64 -cdrom reactos-rust-bootloader-test.iso -m 512 -nographic &
        QEMU_PID=$!
        
        sleep 5
        kill $QEMU_PID 2>/dev/null
        print_success "Test de QEMU x86_64 completado"
    else
        print_warning "QEMU no está instalado, saltando test"
    fi
    
    if command -v qemu-system-i386 &> /dev/null; then
        print_status "Ejecutando QEMU i386..."
        qemu-system-i386 -cdrom reactos-rust-bootloader-test.iso -m 256 -nographic &
        QEMU_PID=$!
        
        sleep 5
        kill $QEMU_PID 2>/dev/null
        print_success "Test de QEMU i386 completado"
    else
        print_warning "QEMU i386 no está instalado, saltando test"
    fi
}

# Función principal
main() {
    echo "🦀 Test del Bootloader de ReactOS Rust OS"
    echo "=========================================="
    echo ""
    
    # Ejecutar pasos
    check_dependencies
    compile_bootloader_x86
    compile_bootloader_x64
    create_test_iso
    test_qemu
    
    echo ""
    print_success "Test del bootloader completado exitosamente"
    echo ""
    echo "📋 Archivos generados:"
    echo "   • reactos-rust-bootloader-test.iso - ISO de test"
    echo "   • test-iso/ - Directorio de ISO de test"
    echo ""
    echo "🚀 Para probar manualmente:"
    echo "   qemu-system-x86_64 -cdrom reactos-rust-bootloader-test.iso -m 512"
    echo "   qemu-system-i386 -cdrom reactos-rust-bootloader-test.iso -m 256"
    echo ""
    echo "🦀 ¡Bootloader listo para usar!"
}

# Ejecutar función principal
main "$@"

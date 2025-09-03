#!/bin/bash

# Script para compilar el kernel de ReactOS Rust con soporte Multiboot2
# Autor: ReactOS Rust Team

set -e

echo "🚀 ReactOS Rust OS - Compilación del Kernel Multiboot2"
echo "====================================================="
echo

# Cambiar al directorio raíz del proyecto
cd "$(dirname "$0")/.."

echo "📁 Directorio de trabajo: $(pwd)"
echo

# Configuración
KERNEL_NAME="reactos-rust-kernel-multiboot2"
TARGET="x86_64-unknown-linux-gnu"
BUILD_MODE="release"

# Función para mostrar progreso
show_progress() {
    echo "🔧 $1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Función para verificar dependencias
check_dependencies() {
    show_progress "Verificando dependencias"
    
    # Verificar que Rust esté instalado
    if ! command -v cargo &> /dev/null; then
        echo "❌ Cargo no encontrado. Instalando Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
    else
        echo "✅ Cargo encontrado"
    fi
    
    # Verificar que el target esté instalado
    if ! rustup target list --installed | grep -q "$TARGET"; then
        echo "❌ Target $TARGET no instalado. Instalando..."
        rustup target add "$TARGET"
    else
        echo "✅ Target $TARGET instalado"
    fi
    
    echo
}

# Función para compilar el kernel
build_kernel() {
    show_progress "Compilando kernel con soporte Multiboot2"
    
    # Compilar el kernel
    if [ "$BUILD_MODE" = "release" ]; then
        echo "🔨 Compilando en modo release..."
        cargo build --target "$TARGET" --release --bin "$KERNEL_NAME"
    else
        echo "🔨 Compilando en modo debug..."
        cargo build --target "$TARGET" --bin "$KERNEL_NAME"
    fi
    
    if [ $? -eq 0 ]; then
        echo "✅ Kernel compilado exitosamente"
    else
        echo "❌ Error compilando kernel"
        exit 1
    fi
    echo
}

# Función para copiar el kernel compilado
copy_kernel() {
    show_progress "Copiando kernel compilado"
    
    # Crear directorio de salida si no existe
    mkdir -p "output/kernel"
    
    # Copiar el kernel compilado
    if [ "$BUILD_MODE" = "release" ]; then
        KERNEL_PATH="target/$TARGET/release/$KERNEL_NAME"
    else
        KERNEL_PATH="target/$TARGET/debug/$KERNEL_NAME"
    fi
    
    if [ -f "$KERNEL_PATH" ]; then
        cp "$KERNEL_PATH" "output/kernel/reactos-rust-kernel.bin"
        echo "✅ Kernel copiado a output/kernel/reactos-rust-kernel.bin"
    else
        echo "❌ Kernel compilado no encontrado en $KERNEL_PATH"
        exit 1
    fi
    echo
}

# Función para verificar el kernel
verify_kernel() {
    show_progress "Verificando kernel compilado"
    
    KERNEL_FILE="output/kernel/reactos-rust-kernel.bin"
    
    if [ -f "$KERNEL_FILE" ]; then
        echo "📊 Información del kernel:"
        echo "  📁 Archivo: $KERNEL_FILE"
        echo "  📏 Tamaño: $(du -h "$KERNEL_FILE" | cut -f1)"
        echo "  📅 Fecha: $(date -r "$KERNEL_FILE")"
        echo "  🔍 Tipo: $(file "$KERNEL_FILE")"
        echo
        echo "✅ Kernel verificado correctamente"
    else
        echo "❌ Kernel no encontrado"
        exit 1
    fi
    echo
}

# Función para crear ISO de prueba
create_test_iso() {
    show_progress "Creando ISO de prueba"
    
    # Verificar que el script de creación de ISO exista
    if [ -f "scripts/create-grub-iso-optimized.sh" ]; then
        echo "🔨 Creando ISO con GRUB..."
        ./scripts/create-grub-iso-optimized.sh
        
        if [ $? -eq 0 ]; then
            echo "✅ ISO creada exitosamente"
        else
            echo "⚠️  Error creando ISO, pero el kernel está listo"
        fi
    else
        echo "⚠️  Script de creación de ISO no encontrado"
        echo "💡 El kernel está listo para usar con GRUB"
    fi
    echo
}

# Función para mostrar información de uso
show_usage() {
    echo "📋 Información de uso:"
    echo
    echo "🎯 Kernel compilado:"
    echo "  • Archivo: output/kernel/reactos-rust-kernel.bin"
    echo "  • Compatible con: GRUB Multiboot2"
    echo "  • Arquitectura: x86_64"
    echo
    echo "🚀 Para probar el kernel:"
    echo "  1. Crear ISO: ./scripts/create-grub-iso-optimized.sh"
    echo "  2. Probar en QEMU: qemu-system-x86_64 -cdrom reactos-rust-os-optimized.iso -m 512"
    echo "  3. O grabar en USB/CD y arrancar desde el dispositivo"
    echo
    echo "🔧 Para desarrollo:"
    echo "  • Modo debug: ./scripts/build-kernel-multiboot2.sh --debug"
    echo "  • Modo release: ./scripts/build-kernel-multiboot2.sh --release"
    echo
}

# Función principal
main() {
    echo "🎯 Iniciando compilación del kernel Multiboot2..."
    echo
    
    # Parsear argumentos
    while [[ $# -gt 0 ]]; do
        case $1 in
            --debug)
                BUILD_MODE="debug"
                shift
                ;;
            --release)
                BUILD_MODE="release"
                shift
                ;;
            --help|-h)
                show_usage
                exit 0
                ;;
            *)
                echo "❌ Argumento desconocido: $1"
                echo "💡 Usa --help para ver opciones disponibles"
                exit 1
                ;;
        esac
    done
    
    echo "⚙️  Modo de compilación: $BUILD_MODE"
    echo
    
    check_dependencies
    build_kernel
    copy_kernel
    verify_kernel
    create_test_iso
    show_usage
    
    echo "🎉 ¡Kernel Multiboot2 compilado exitosamente!"
    echo
    echo "📋 Próximos pasos:"
    echo "  1. Probar el kernel con GRUB"
    echo "  2. Verificar que se carga correctamente"
    echo "  3. Desarrollar más funcionalidades del kernel"
    echo "  4. Integrar con el sistema de aplicaciones"
}

# Ejecutar función principal
main "$@"

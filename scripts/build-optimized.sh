#!/bin/bash

# Script de Build Optimizado para ReactOS Rust OS
# Autor: ReactOS Rust Team

set -e

echo "🚀 ReactOS Rust OS - Sistema de Build Optimizado"
echo "================================================"
echo

# Cambiar al directorio raíz del proyecto
cd "$(dirname "$0")/.."

echo "📁 Directorio de trabajo: $(pwd)"
echo

# Función para mostrar ayuda
show_help() {
    echo "Uso: $0 [COMANDO] [OPCIONES]"
    echo
    echo "Comandos disponibles:"
    echo "  install-targets  Instalar targets necesarios"
    echo "  check-targets    Verificar targets instalados"
    echo "  build-all        Compilar para todas las arquitecturas"
    echo "  build-native     Compilar solo para arquitectura nativa (64-bit)"
    echo "  build-32bit      Compilar solo para arquitectura 32-bit"
    echo "  build-uefi       Compilar solo para UEFI bootloader"
    echo "  test-all         Ejecutar pruebas para todas las arquitecturas"
    echo "  clean-all        Limpiar builds para todas las arquitecturas"
    echo "  info             Mostrar información del sistema"
    echo "  help             Mostrar esta ayuda"
    echo
    echo "Opciones:"
    echo "  --debug          Usar modo debug en lugar de release"
    echo "  --jobs N         Usar N jobs paralelos (default: 1)"
    echo
    echo "Ejemplos:"
    echo "  $0 install-targets"
    echo "  $0 build-all"
    echo "  $0 build-native --debug"
    echo "  $0 test-all --jobs 2"
}

# Variables por defecto
DEBUG_MODE=false
JOBS=1
COMMAND=""

# Parsear argumentos
while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            DEBUG_MODE=true
            shift
            ;;
        --jobs)
            JOBS="$2"
            shift 2
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
        *)
            if [[ -z "$COMMAND" ]]; then
                COMMAND="$1"
            else
                echo "❌ Argumento desconocido: $1"
                show_help
                exit 1
            fi
            shift
            ;;
    esac
done

# Si no se especifica comando, mostrar ayuda
if [[ -z "$COMMAND" ]]; then
    show_help
    exit 1
fi

# Función para ejecutar comando con manejo de errores
run_command() {
    local cmd="$1"
    local description="$2"
    
    echo "🔧 Ejecutando: $description"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    if eval "$cmd"; then
        echo "✅ $description: ÉXITO"
    else
        echo "❌ $description: FALLÓ"
        return 1
    fi
    
    echo
}

# Configurar variables de entorno
export CARGO_BUILD_JOBS="$JOBS"

if [[ "$DEBUG_MODE" == "true" ]]; then
    echo "🐛 Modo DEBUG habilitado"
    export CARGO_PROFILE="dev"
else
    echo "⚡ Modo RELEASE habilitado"
    export CARGO_PROFILE="release"
fi

echo "🔢 Jobs paralelos: $JOBS"
echo

# Ejecutar comando según la opción
case "$COMMAND" in
    "install-targets")
        run_command "cargo install-targets" "Instalación de targets"
        ;;
    "check-targets")
        run_command "cargo check-targets" "Verificación de targets"
        ;;
    "build-all")
        run_command "cargo build-all" "Compilación para todas las arquitecturas"
        ;;
    "build-native")
        if [[ "$DEBUG_MODE" == "true" ]]; then
            run_command "cargo build --target x86_64-unknown-linux-gnu" "Compilación nativa (debug)"
        else
            run_command "cargo build-native" "Compilación nativa (release)"
        fi
        ;;
    "build-32bit")
        if [[ "$DEBUG_MODE" == "true" ]]; then
            run_command "cargo build --target i686-unknown-linux-gnu" "Compilación 32-bit (debug)"
        else
            run_command "cargo build-32bit" "Compilación 32-bit (release)"
        fi
        ;;
    "build-uefi")
        if [[ "$DEBUG_MODE" == "true" ]]; then
            run_command "cargo build --target x86_64-unknown-uefi" "Compilación UEFI (debug)"
        else
            run_command "cargo build-uefi" "Compilación UEFI (release)"
        fi
        ;;
    "test-all")
        run_command "cargo test-all" "Pruebas para todas las arquitecturas"
        ;;
    "clean-all")
        run_command "cargo clean-all" "Limpieza de builds"
        ;;
    "info")
        run_command "cargo build-info" "Información del sistema"
        ;;
    "help")
        show_help
        ;;
    *)
        echo "❌ Comando desconocido: $COMMAND"
        show_help
        exit 1
        ;;
esac

echo "🎉 Operación completada exitosamente!"

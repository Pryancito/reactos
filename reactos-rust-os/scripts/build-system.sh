#!/bin/bash

# Sistema de Build Optimizado para ReactOS Rust OS
# Autor: ReactOS Rust Team

set -e

echo "🚀 ReactOS Rust OS - Sistema de Build Optimizado"
echo "================================================"
echo

# Cambiar al directorio raíz del proyecto
cd "$(dirname "$0")/.."

echo "📁 Directorio de trabajo: $(pwd)"
echo

# Configuración
TARGETS=(
    "x86_64-unknown-linux-gnu:native-64bit:Arquitectura nativa 64-bit (Linux)"
    "i686-unknown-linux-gnu:compat-32bit:Arquitectura compatible 32-bit (Linux)"
    "x86_64-unknown-uefi:uefi-bootloader:Bootloader UEFI 64-bit"
)

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
    BUILD_FLAGS=""
else
    echo "⚡ Modo RELEASE habilitado"
    BUILD_FLAGS="--release"
fi

echo "🔢 Jobs paralelos: $JOBS"
echo

# Función para instalar targets
install_targets() {
    echo "🔧 Instalando targets necesarios..."
    
    for target_info in "${TARGETS[@]}"; do
        IFS=':' read -r target name description <<< "$target_info"
        echo "📦 Instalando target: $name ($target)"
        
        if rustup target add "$target"; then
            echo "✅ $name instalado correctamente"
        else
            echo "❌ Error instalando $name"
            return 1
        fi
    done
    
    echo "🎉 Instalación de targets completada"
}

# Función para verificar targets
check_targets() {
    echo "🔍 Verificando targets instalados..."
    
    local installed_targets
    installed_targets=$(rustup target list --installed)
    
    for target_info in "${TARGETS[@]}"; do
        IFS=':' read -r target name description <<< "$target_info"
        
        if echo "$installed_targets" | grep -q "$target"; then
            echo "✅ $name ($target) - Instalado"
        else
            echo "❌ $name ($target) - No instalado"
        fi
    done
}

# Función para compilar un target específico
build_target() {
    local target="$1"
    local name="$2"
    local description="$3"
    
    echo "🔨 Compilando para $name ($target)..."
    
    local cmd="cargo build --target $target $BUILD_FLAGS --jobs $JOBS"
    
    if eval "$cmd"; then
        echo "✅ Compilación exitosa para $name"
        return 0
    else
        echo "❌ Error en compilación para $name"
        return 1
    fi
}

# Función para compilar todos los targets
build_all() {
    echo "🚀 Iniciando compilación para todas las arquitecturas..."
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    local success_count=0
    local total_count=${#TARGETS[@]}
    
    for target_info in "${TARGETS[@]}"; do
        IFS=':' read -r target name description <<< "$target_info"
        
        if build_target "$target" "$name" "$description"; then
            ((success_count++))
        fi
    done
    
    echo
    echo "📊 Resumen de compilación:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Total de targets: $total_count"
    echo "✅ Exitosos: $success_count"
    echo "❌ Fallidos: $((total_count - success_count))"
    
    local success_rate=$((success_count * 100 / total_count))
    echo "📈 Tasa de éxito: ${success_rate}%"
    
    if [[ $success_count -eq $total_count ]]; then
        echo "🎉 ¡Todas las compilaciones exitosas!"
    else
        echo "⚠️  Algunas compilaciones fallaron"
    fi
}

# Función para mostrar información
show_info() {
    echo "🔧 Sistema de Build ReactOS Rust OS"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📁 Workspace: $(pwd)"
    echo "📁 Build dir: target"
    echo "⚙️  Modo: $([ "$DEBUG_MODE" == "true" ] && echo "Debug" || echo "Release")"
    echo "🔢 Jobs paralelos: $JOBS"
    echo
    
    echo "🎯 Targets configurados:"
    for target_info in "${TARGETS[@]}"; do
        IFS=':' read -r target name description <<< "$target_info"
        echo "  ✅ $name ($target) - $description"
    done
}

# Función para ejecutar pruebas
test_all() {
    echo "🧪 Ejecutando pruebas para todas las arquitecturas..."
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    local cmd="cargo test $BUILD_FLAGS --jobs $JOBS"
    
    if eval "$cmd"; then
        echo "✅ Todas las pruebas exitosas"
    else
        echo "❌ Algunas pruebas fallaron"
    fi
}

# Función para limpiar builds
clean_all() {
    echo "🧹 Limpiando builds para todas las arquitecturas..."
    
    for target_info in "${TARGETS[@]}"; do
        IFS=':' read -r target name description <<< "$target_info"
        echo "🧹 Limpiando build para $name ($target)"
        
        if cargo clean --target "$target"; then
            echo "✅ Limpieza exitosa para $name"
        else
            echo "❌ Error limpiando $name"
        fi
    done
    
    # Limpiar también el directorio target principal
    echo "🧹 Limpiando directorio target principal"
    if cargo clean; then
        echo "✅ Directorio target principal limpiado"
    else
        echo "❌ Error limpiando directorio principal"
    fi
    
    echo "🎉 Limpieza completada"
}

# Ejecutar comando según la opción
case "$COMMAND" in
    "install-targets")
        run_command "install_targets" "Instalación de targets"
        ;;
    "check-targets")
        run_command "check_targets" "Verificación de targets"
        ;;
    "build-all")
        run_command "build_all" "Compilación para todas las arquitecturas"
        ;;
    "build-native")
        IFS=':' read -r target name description <<< "${TARGETS[0]}"
        run_command "build_target '$target' '$name' '$description'" "Compilación nativa"
        ;;
    "build-32bit")
        IFS=':' read -r target name description <<< "${TARGETS[1]}"
        run_command "build_target '$target' '$name' '$description'" "Compilación 32-bit"
        ;;
    "build-uefi")
        IFS=':' read -r target name description <<< "${TARGETS[2]}"
        run_command "build_target '$target' '$name' '$description'" "Compilación UEFI"
        ;;
    "test-all")
        run_command "test_all" "Pruebas para todas las arquitecturas"
        ;;
    "clean-all")
        run_command "clean_all" "Limpieza de builds"
        ;;
    "info")
        show_info
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

#!/bin/bash

# Script de configuración de build para ReactOS
# Usa nuestro toolchain POSIX personalizado

set -e

# Parámetros por defecto
BUILD_TYPE=${1:-Debug}
ARCH=${2:-amd64}
GENERATOR=${3:-"Unix Makefiles"}
SOURCE_DIR=${4:-".."}
BUILD_DIR="output-posix-${ARCH}"

echo "=== Configuración ReactOS POSIX ==="
echo "Build Type: $BUILD_TYPE"
echo "Architecture: $ARCH"
echo "Generator: $GENERATOR"
echo "Source Dir: $SOURCE_DIR"
echo "Build Dir: $BUILD_DIR"
echo

# Crear directorio de build
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Configurar con CMake
cmake -G "$GENERATOR" \
    -DCMAKE_BUILD_TYPE="$BUILD_TYPE" \
    -DARCH:STRING="$ARCH" \
    -DENABLE_CCACHE:BOOL=OFF \
    -DUSE_SEH_PLUGIN:BOOL=OFF \
    -DUSE_COMPILER_EXCEPTIONS:BOOL=OFF \
    -DCMAKE_TOOLCHAIN_FILE:FILEPATH="../toolchain-gcc-posix.cmake" \
    "$SOURCE_DIR"

echo
echo "✅ Configuración completada"
echo "📁 Directorio de build: $BUILD_DIR"
echo "🔨 Para compilar: cd $BUILD_DIR && make -j$(nproc)"

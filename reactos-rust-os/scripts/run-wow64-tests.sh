#!/bin/bash

# Script para ejecutar pruebas de WOW64
# Autor: ReactOS Rust Team

set -e

echo "🧪 ReactOS WOW64 - Ejecutor de Pruebas"
echo "======================================"
echo

# Cambiar al directorio de WOW64
cd "$(dirname "$0")/../wow64"

# Verificar que estamos en el directorio correcto
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: No se encontró Cargo.toml en el directorio actual"
    exit 1
fi

echo "📁 Directorio de trabajo: $(pwd)"
echo

# Compilar el ejecutor de pruebas
echo "🔨 Compilando ejecutor de pruebas..."
cargo build --release --bin test_runner

if [ $? -ne 0 ]; then
    echo "❌ Error: Falló la compilación del ejecutor de pruebas"
    exit 1
fi

echo "✅ Compilación exitosa"
echo

# Ejecutar las pruebas
echo "🚀 Ejecutando pruebas de WOW64..."
echo

./target/release/test_runner "$@"

echo
echo "🎉 Pruebas completadas!"

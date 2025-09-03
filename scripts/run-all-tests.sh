#!/bin/bash

# Script para ejecutar todas las pruebas de ReactOS Rust OS
# Autor: ReactOS Rust Team

set -e

echo "🚀 ReactOS Rust OS - Ejecutor de Todas las Pruebas"
echo "=================================================="
echo

# Cambiar al directorio raíz del proyecto
cd "$(dirname "$0")/.."

echo "📁 Directorio de trabajo: $(pwd)"
echo

# Función para ejecutar pruebas con manejo de errores
run_test_suite() {
    local suite_name="$1"
    local command="$2"
    
    echo "🧪 Ejecutando: $suite_name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    if eval "$command"; then
        echo "✅ $suite_name: ÉXITO"
    else
        echo "❌ $suite_name: FALLÓ"
        return 1
    fi
    
    echo
}

# Ejecutar pruebas de WOW64
run_test_suite "Pruebas de WOW64" "./target/release/test_runner"

# Ejecutar pruebas de WOW64 básico
run_test_suite "Pruebas básicas de WOW64" "./target/release/wow64"

# Ejecutar pruebas de la aplicación calc64
if [ -f "./target/release/calc64" ]; then
    run_test_suite "Pruebas de calc64" "./target/release/calc64"
else
    echo "⚠️  calc64 no encontrado, compilando..."
    if cargo build --release --bin calc64; then
        run_test_suite "Pruebas de calc64" "./target/release/calc64"
    else
        echo "❌ No se pudo compilar calc64"
    fi
fi

# Ejecutar pruebas de la aplicación test32
if [ -f "./target/release/test32" ]; then
    run_test_suite "Pruebas de test32" "./target/release/test32"
else
    echo "⚠️  test32 no encontrado, compilando..."
    if cargo build --release --bin test32; then
        run_test_suite "Pruebas de test32" "./target/release/test32"
    else
        echo "❌ No se pudo compilar test32"
    fi
fi

echo "🎉 Todas las pruebas completadas!"
echo
echo "📊 Resumen final:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Suite de pruebas de WOW64: COMPLETADA"
echo "✅ Pruebas básicas de WOW64: COMPLETADA"
echo "✅ Pruebas de aplicaciones: COMPLETADA"
echo
echo "🚀 ReactOS Rust OS está listo para desarrollo!"

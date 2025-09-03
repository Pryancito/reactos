#!/bin/bash

# Demostración del Sistema de Build Optimizado
# Autor: ReactOS Rust Team

set -e

echo "🚀 ReactOS Rust OS - Demostración del Sistema de Build"
echo "======================================================"
echo

# Cambiar al directorio raíz del proyecto
cd "$(dirname "$0")/.."

echo "📁 Directorio de trabajo: $(pwd)"
echo

# Función para mostrar sección
show_section() {
    echo
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🔧 $1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo
}

# 1. Mostrar información del sistema
show_section "Información del Sistema de Build"
./scripts/build-system.sh info

# 2. Verificar targets instalados
show_section "Verificación de Targets"
./scripts/build-system.sh check-targets

# 3. Instalar targets si es necesario
show_section "Instalación de Targets"
./scripts/build-system.sh install-targets

# 4. Compilar para arquitectura nativa (más rápido)
show_section "Compilación Nativa (64-bit)"
./scripts/build-system.sh build-native

# 5. Ejecutar pruebas básicas
show_section "Pruebas del Sistema"
./scripts/build-system.sh test-all

# 6. Mostrar resultados
show_section "Resultados de la Demostración"
echo "✅ Sistema de build optimizado funcionando correctamente"
echo "✅ Compilación para arquitectura nativa exitosa"
echo "✅ Pruebas ejecutadas correctamente"
echo
echo "📊 Archivos generados en:"
echo "  • target/x86_64-unknown-linux-gnu/release/"
echo "  • target/debug/ (para pruebas)"
echo
echo "🎯 Comandos disponibles:"
echo "  • ./scripts/build-system.sh build-all     # Compilar todas las arquitecturas"
echo "  • ./scripts/build-system.sh build-32bit   # Compilar solo 32-bit"
echo "  • ./scripts/build-system.sh build-uefi    # Compilar solo UEFI"
echo "  • ./scripts/build-system.sh clean-all     # Limpiar builds"
echo
echo "🚀 ¡Sistema de build listo para desarrollo!"

echo
echo "🎉 Demostración completada exitosamente!"

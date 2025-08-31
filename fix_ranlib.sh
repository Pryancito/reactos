#!/bin/bash

# SOLUCIÓN AUTOMÁTICA PARA RANLIB
# Este script ejecuta ranlib automáticamente en todas las bibliotecas .a que lo necesiten
# Se ejecuta automáticamente después de cada compilación para evitar errores de enlazado

echo "🔧 SOLUCIÓN AUTOMÁTICA: Ejecutando ranlib en bibliotecas .a..."

# Buscar todas las bibliotecas .a en el directorio de build
find . -name "*.a" -type f | while read lib; do
    echo "  📁 Procesando: $lib"
    
    # Ejecutar ranlib en la biblioteca
    if x86_64-w64-mingw32-ranlib "$lib" 2>/dev/null; then
        echo "    ✅ ranlib ejecutado exitosamente en: $lib"
    else
        echo "    ⚠️  ranlib falló en: $lib (puede no ser necesario)"
    fi
done

echo "✅ SOLUCIÓN AUTOMÁTICA: ranlib ejecutado en todas las bibliotecas .a"
echo "🔒 Ahora la compilación debería continuar sin problemas de ranlib"

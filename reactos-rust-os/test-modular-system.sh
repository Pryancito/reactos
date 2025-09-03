#!/bin/bash

echo "🧪 Probando Sistema Modular de ReactOS Windows"
echo "=============================================="

if [ -f "target/debug/reactos-windows" ]; then
    echo "✅ Ejecutable encontrado"
    echo ""
    echo "Ejecutando sistema modular..."
    echo "============================="
    echo ""
    echo "Comandos de prueba sugeridos:"
    echo "  help        - Ver comandos disponibles"
    echo "  info        - Información del sistema"
    echo "  ping google.com - Probar plugin de red"
    echo "  ipconfig    - Configuración de red"
    echo "  dir         - Listar directorio"
    echo "  cd Windows  - Cambiar directorio"
    echo "  pwd         - Mostrar directorio actual"
    echo "  exit        - Salir del sistema"
    echo ""
    echo "Presiona Enter para continuar..."
    read
    
    ./target/debug/reactos-windows
else
    echo "❌ Ejecutable no encontrado"
    echo "Compilando primero..."
    cargo build
    if [ -f "target/debug/reactos-windows" ]; then
        echo "✅ Compilación exitosa"
        ./target/debug/reactos-windows
    else
        echo "❌ Error en compilación"
    fi
fi

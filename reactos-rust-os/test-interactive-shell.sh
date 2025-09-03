#!/bin/bash

echo "🧪 Probando Shell Interactivo de ReactOS Windows"
echo "==============================================="

if [ -f "target/debug/reactos-windows" ]; then
    echo "✅ Ejecutable encontrado"
    echo ""
    echo "Ejecutando shell interactivo..."
    echo "=============================="
    echo ""
    echo "Comandos de prueba sugeridos:"
    echo "  help        - Ver comandos disponibles"
    echo "  info        - Información del sistema"
    echo "  dir         - Listar directorio"
    echo "  date        - Mostrar fecha"
    echo "  whoami      - Mostrar usuario"
    echo "  tasklist    - Mostrar procesos"
    echo "  services    - Mostrar servicios"
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

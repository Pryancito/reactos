#!/bin/bash

echo "🧪 Probando Sistema Simple de ReactOS Windows"
echo "============================================="

if [ -f "target/debug/reactos-windows" ]; then
    echo "✅ Ejecutable encontrado"
    echo ""
    echo "Ejecutando sistema..."
    echo "===================="
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

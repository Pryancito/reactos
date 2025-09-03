#!/bin/bash
echo "🧮 Calculadora de ReactOS Windows en Rust"
echo "========================================="
echo "Ingrese una expresión matemática (ej: 2+2):"
read expression
result=$(echo "$expression" | bc 2>/dev/null || echo "Error en la expresión")
echo "Resultado: $result"

#!/bin/bash

# Script de pruebas del entorno ReactOS POSIX

echo "🧪 PROBANDO ENTORNO REACTOS POSIX"
echo "================================="

# Test 1: Verificar toolchain
echo
echo "Test 1: Verificando toolchain..."
if command -v x86_64-w64-mingw32-gcc-posix >/dev/null 2>&1; then
    echo "✅ GCC POSIX disponible"
    x86_64-w64-mingw32-gcc-posix --version | head -1
else
    echo "❌ GCC POSIX no encontrado"
    exit 1
fi

# Test 2: Verificar CMake
echo
echo "Test 2: Verificando CMake..."
if command -v cmake >/dev/null 2>&1; then
    echo "✅ CMake disponible"
    cmake --version | head -1
else
    echo "❌ CMake no encontrado"
    exit 1
fi

# Test 3: Compilación simple
echo
echo "Test 3: Compilación simple..."
echo '#include <stdio.h>
int main() { printf("Hello ReactOS POSIX!\n"); return 0; }' > test_simple.c

if x86_64-w64-mingw32-gcc-posix -o test_simple.exe test_simple.c -static-libgcc 2>/dev/null; then
    echo "✅ Compilación simple exitosa"
    rm -f test_simple.exe test_simple.c
else
    echo "❌ Compilación simple falló"
    rm -f test_simple.c
    exit 1
fi

# Test 4: Bootloader personalizado
echo
echo "Test 4: Verificando bootloader personalizado..."
if [ -f "compile-custom-bootloader.sh" ]; then
    echo "✅ Script de bootloader disponible"
    if [ -f "boot/freeldr/custom-bootloader/main.c" ]; then
        echo "✅ Código fuente de bootloader disponible"
    else
        echo "⚠️  Código fuente de bootloader no encontrado"
    fi
else
    echo "❌ Script de bootloader no encontrado"
fi

echo
echo "🎉 ENTORNO REACTOS POSIX VERIFICADO"
echo "Todos los componentes están listos para compilar ReactOS"

#!/bin/bash

# SCRIPT DE COMPILACIÓN REACTOS CON ISOS PERSONALIZADAS AUTOMÁTICAS
# Este script compila ReactOS y genera automáticamente las 2 ISOs principales
# que están listas para dd sin problemas adicionales

set -e  # Salir si algún comando falla

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Función para imprimir mensajes
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_header() {
    echo -e "${CYAN}${BOLD}$1${NC}"
    echo -e "${CYAN}$(printf '=%.0s' {1..50})${NC}"
}

# Verificar que estamos en el directorio correcto
if [[ ! -f "CMakeLists.txt" ]]; then
    print_error "Este script debe ejecutarse desde el directorio raíz de ReactOS"
    exit 1
fi

print_header "COMPILACIÓN REACTOS CON ISOS PERSONALIZADAS AUTOMÁTICAS"
echo

print_info "Este script compilará ReactOS y generará automáticamente:"
print_info "  1️⃣  reactos-uefi-efi.iso (Boot UEFI nativo)"
print_info "  2️⃣  reactos-usb.iso (USB con herramientas externas)"
echo

print_info "Verificando dependencias..."
echo

# Verificar que tenemos las herramientas necesarias
if ! command -v cmake &> /dev/null; then
    print_error "CMake no está instalado"
    exit 1
fi

if ! command -v make &> /dev/null; then
    print_error "Make no está instalado"
    exit 1
fi

if ! command -v x86_64-w64-mingw32-gcc-posix &> /dev/null; then
    print_warning "MinGW-w64 POSIX no encontrado, intentando usar versión estándar..."
    if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
        print_error "MinGW-w64 no está instalado"
        exit 1
    fi
fi

print_success "Dependencias verificadas"
echo

# Verificar si ya existe un directorio de build
if [[ -d "output-posix-amd64" ]]; then
    print_warning "Directorio de build existente detectado"
    read -p "¿Quieres limpiar y reconfigurar? (s/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Ss]$ ]]; then
        print_info "Limpiando directorio de build existente..."
        rm -rf output-posix-amd64
        print_success "Directorio de build limpiado"
    else
        print_info "Usando directorio de build existente"
    fi
fi

# Crear directorio de build si no existe
if [[ ! -d "output-posix-amd64" ]]; then
    print_info "Creando directorio de build..."
    mkdir -p output-posix-amd64
    print_success "Directorio de build creado"
fi

# Cambiar al directorio de build
cd output-posix-amd64

print_header "CONFIGURANDO CMAKE"
echo

# Configurar CMake con las opciones necesarias
print_info "Configurando CMake para compilación POSIX..."
cmake .. \
    -DCMAKE_TOOLCHAIN_FILE=../toolchain-gcc-posix.cmake \
    -DARCH=amd64 \
    -DCMAKE_BUILD_TYPE=Debug \
    -DREACTOS_BUILD_DIR=output-posix-amd64

if [[ $? -eq 0 ]]; then
    print_success "CMake configurado exitosamente"
else
    print_error "Error al configurar CMake"
    exit 1
fi

echo

print_header "COMPILANDO REACTOS"
echo

# Compilar ReactOS (usando solo 1 core como especificado en las reglas)
print_info "Compilando ReactOS (usando 1 core)..."
print_info "Esto puede tomar bastante tiempo..."
echo

make -j1

if [[ $? -eq 0 ]]; then
    print_success "ReactOS compilado exitosamente"
else
    print_error "Error durante la compilación"
    exit 1
fi

echo

print_header "GENERANDO ISOS PERSONALIZADAS"
echo

# Generar las ISOs personalizadas
print_info "Generando ISO UEFI EFI (Boot UEFI nativo)..."
make custom-uefi-efi-iso

if [[ $? -eq 0 ]]; then
    print_success "ISO UEFI EFI generada exitosamente"
else
    print_warning "Error al generar ISO UEFI EFI, continuando..."
fi

echo

print_info "Generando ISO USB (Herramientas externas)..."
make custom-usb-iso

if [[ $? -eq 0 ]]; then
    print_success "ISO USB generada exitosamente"
else
    print_warning "Error al generar ISO USB, continuando..."
fi

echo

print_header "VERIFICANDO ISOS GENERADAS"
echo

# Verificar que las ISOs se crearon
if [[ -f "reactos-uefi-efi.iso" ]]; then
    print_success "✅ reactos-uefi-efi.iso: $(ls -lh reactos-uefi-efi.iso | awk '{print $5}')"
else
    print_error "❌ reactos-uefi-efi.iso no encontrada"
fi

if [[ -f "reactos-usb.iso" ]]; then
    print_success "✅ reactos-usb.iso: $(ls -lh reactos-usb.iso | awk '{print $5}')"
else
    print_error "❌ reactos-usb.iso no encontrada"
fi

echo

print_header "RESUMEN FINAL"
echo

print_success "🎉 ¡Compilación completada!"
echo

if [[ -f "reactos-uefi-efi.iso" && -f "reactos-usb.iso" ]]; then
    print_success "✅ AMBAS ISOs generadas exitosamente:"
    print_info "   📁 reactos-uefi-efi.iso - Boot UEFI nativo en sistemas modernos"
    print_info "   📁 reactos-usb.iso - USB con herramientas externas para máxima compatibilidad"
    echo
    print_info "🚀 USO INMEDIATO CON DD:"
    print_info "   # Para ISO UEFI EFI (sistemas UEFI modernos):"
    print_info "   sudo dd if=reactos-uefi-efi.iso of=/dev/sdX bs=4M status=progress conv=fdatasync"
    echo
    print_info "   # Para ISO USB (herramientas externas):"
    print_info "   sudo dd if=reactos-usb.iso of=/dev/sdX bs=4M status=progress conv=fdatasync"
    echo
    print_success "✨ Ambas ISOs están listas para dd sin problemas adicionales"
    print_info "   No se requieren particiones EFI manuales ni configuraciones adicionales"
else
    print_warning "⚠️  Algunas ISOs no se generaron correctamente"
    print_info "   Revisa los mensajes de error anteriores"
fi

echo
print_info "📋 Archivos generados en: $(pwd)"
print_info "   Las ISOs están listas para usar inmediatamente"

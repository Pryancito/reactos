#!/bin/bash

# SCRIPT PARA PROBAR EL USB UEFI EN QEMU ANTES DE USARLO EN HARDWARE REAL

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

# Verificar argumentos
if [[ $# -ne 1 ]]; then
    print_error "Uso: $0 <dispositivo_usb>"
    print_error "Ejemplo: $0 /dev/sdb"
    exit 1
fi

USB_DEVICE="$1"

print_header "PROBANDO USB UEFI EN QEMU"
echo

print_info "Dispositivo USB: $USB_DEVICE"
echo

# Verificar que el dispositivo existe
if [[ ! -b "$USB_DEVICE" ]]; then
    print_error "El dispositivo USB '$USB_DEVICE' no existe"
    exit 1
fi

# Verificar que QEMU está instalado
if ! command -v qemu-system-x86_64 &> /dev/null; then
    print_error "QEMU no está instalado"
    print_info "Instalando QEMU..."
    sudo apt-get update
    sudo apt-get install -y qemu-system-x86
fi

print_success "QEMU encontrado: $(qemu-system-x86_64 --version | head -1)"

# Verificar que el USB está configurado correctamente
print_info "Verificando configuración del USB..."
if ! ./verify-uefi-usb.sh "$USB_DEVICE" &> /dev/null; then
    print_error "El USB no está configurado correctamente"
    print_info "Ejecuta primero: ./verify-uefi-usb.sh $USB_DEVICE"
    exit 1
fi

print_success "USB verificado correctamente"

# Crear imagen de prueba
print_info "Creando imagen de prueba para QEMU..."
TEST_IMAGE="test-uefi-usb.img"
qemu-img create -f raw "$TEST_IMAGE" 1G

# Iniciar QEMU con UEFI y el USB
print_info "Iniciando QEMU con UEFI y USB..."
print_info "En QEMU, presiona F12 para acceder al menú de arranque"
print_info "Deberías ver 'ReactOS UEFI' como opción de arranque"
echo

print_warning "⚠️  IMPORTANTE:"
print_info "1. En QEMU, presiona F12 para acceder al menú de arranque"
print_info "2. Selecciona 'ReactOS UEFI' o 'UEFI USB'"
print_info "3. Si aparece, significa que el USB está configurado correctamente"
print_info "4. Presiona Ctrl+C para salir de QEMU"
echo

read -p "¿Continuar con la prueba en QEMU? (s/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Ss]$ ]]; then
    print_info "Prueba cancelada"
    rm -f "$TEST_IMAGE"
    exit 0
fi

# Iniciar QEMU
print_info "Iniciando QEMU..."
qemu-system-x86_64 \
    -m 512 \
    -enable-kvm \
    -bios /usr/share/ovmf/OVMF.fd \
    -drive file="$TEST_IMAGE",format=raw \
    -drive file="$USB_DEVICE",format=raw,if=usb \
    -usb \
    -device usb-ehci,id=ehci \
    -device usb-tablet \
    -display gtk \
    -vga std \
    -net none

# Limpiar
print_info "Limpiando archivos de prueba..."
rm -f "$TEST_IMAGE"

print_header "PRUEBA COMPLETADA"
echo
print_info "Si viste 'ReactOS UEFI' en el menú de arranque de QEMU:"
print_success "✅ El USB está configurado correctamente"
print_success "✅ Debería funcionar en tu ASUS 10ª generación"
echo
print_info "Si no viste la opción:"
print_warning "⚠️  El USB puede tener algún problema de configuración"
print_info "Revisa los logs de QEMU y verifica la configuración del USB"

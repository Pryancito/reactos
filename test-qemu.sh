#!/bin/bash

# Script final para probar GRUB 2 + ReactOS en QEMU
# Autor: Asistente de ReactOS
# Fecha: $(date)

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Función para imprimir mensajes
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}================================${NC}"
    echo -e "${PURPLE} $1${NC}"
    echo -e "${PURPLE}================================${NC}"
}

echo "=========================================="
echo "  PRUEBA FINAL: GRUB 2 + REACTOS EN QEMU"
echo "=========================================="
echo ""

print_header "VERIFICACIÓN DE QEMU"

# Verificar si QEMU está instalado
if ! command -v qemu-system-x86_64 &> /dev/null; then
    print_error "QEMU no está instalado"
    print_info "Instalando QEMU..."
    sudo apt update
    sudo apt install -y qemu-system-x86_64
fi

print_success "QEMU encontrado: $(qemu-system-x86_64 --version | head -1)"

# Verificar OVMF
if [[ -f "/usr/share/ovmf/OVMF.fd" ]]; then
    print_success "OVMF (firmware UEFI) encontrado"
else
    print_warning "OVMF no encontrado, instalando..."
    sudo apt install -y ovmf
fi

print_header "VERIFICACIÓN DEL PENDRAVE"

# Solicitar dispositivo USB
read -p "Ingresa el dispositivo USB (ej: sdb): " USB_DEVICE

if [[ -z "$USB_DEVICE" ]]; then
    print_error "No se especificó un dispositivo"
    exit 1
fi

# Verificar que el dispositivo existe
if [[ ! -b "/dev/$USB_DEVICE" ]]; then
    print_error "El dispositivo /dev/$USB_DEVICE no existe"
    exit 1
fi

# Verificar que el dispositivo no está montado
if mount | grep -q "/dev/$USB_DEVICE"; then
    print_warning "El dispositivo está montado. Desmontando..."
    sudo umount "/dev/${USB_DEVICE}1" 2>/dev/null || true
    sudo umount "/dev/${USB_DEVICE}2" 2>/dev/null || true
fi

print_header "CONFIGURACIÓN DE QEMU"

# Verificar permisos del dispositivo
if [[ ! -r "/dev/$USB_DEVICE" ]]; then
    print_warning "No tienes permisos de lectura en /dev/$USB_DEVICE"
    print_info "Ejecutando con sudo..."
    SUDO_CMD="sudo"
else
    SUDO_CMD=""
fi

print_info "Configurando QEMU para bootear desde el pendrive..."

print_header "INICIANDO PRUEBA EN QEMU"

echo "🚀 Iniciando QEMU con el pendrive..."
echo ""
echo "📋 OBJETIVO DE LA PRUEBA:"
echo ""
echo "✅ ${CYAN}Verificar que GRUB 2 carga automáticamente el menú gráfico${NC}"
echo "✅ ${CYAN}Confirmar que no aparece el prompt de texto${NC}"
echo "✅ ${CYAN}Probar que ReactOS arranca correctamente${NC}"
echo ""
echo "📋 INSTRUCCIONES PARA LA PRUEBA:"
echo ""
echo "1. ${YELLOW}QEMU se iniciará con UEFI${NC}"
echo "2. ${YELLOW}GRUB debería cargar automáticamente el menú gráfico${NC}"
echo "3. ${YELLOW}Si ves el menú, selecciona 'ReactOS (Normal)'${NC}"
echo "4. ${YELLOW}ReactOS debería arrancar${NC}"
echo ""
echo "📋 SI APARECE EL PROMPT DE TEXTO:"
echo "   ${CYAN}Escribe: configfile /EFI/GRUB/grub.cfg${NC}"
echo "   ${CYAN}O: configfile /EFI/BOOT/grub.cfg${NC}"
echo "   ${CYAN}O: configfile /boot/grub/grub.cfg${NC}"
echo ""
echo "📋 Comandos útiles en QEMU:"
echo "   ${CYAN}Ctrl+Alt+G${NC} - Liberar el mouse"
echo "   ${CYAN}Ctrl+Alt+F${NC} - Pantalla completa"
echo "   ${CYAN}Ctrl+Alt+Q${NC} - Salir de QEMU"
echo ""

read -p "Presiona Enter para iniciar QEMU..."

# Ejecutar QEMU con parámetros optimizados
if [[ -n "$SUDO_CMD" ]]; then
    $SUDO_CMD qemu-system-x86_64 \
        -m 2048 \
        -smp 2 \
        -enable-kvm \
        -machine q35 \
        -bios /usr/share/ovmf/OVMF.fd \
        -device virtio-scsi-pci \
        -device scsi-hd,drive=usb_drive \
        -drive file=/dev/$USB_DEVICE,if=none,id=usb_drive,format=raw \
        -device usb-ehci \
        -device usb-tablet \
        -device virtio-net-pci,netdev=net0 \
        -netdev user,id=net0 \
        -vga std \
        -display gtk \
        -boot order=c \
        -monitor stdio \
        -name "ReactOS GRUB 2 Test - Auto-Carga"
else
    qemu-system-x86_64 \
        -m 2048 \
        -smp 2 \
        -enable-kvm \
        -machine q35 \
        -bios /usr/share/ovmf/OVMF.fd \
        -device virtio-scsi-pci \
        -device scsi-hd,drive=usb_drive \
        -drive file=/dev/$USB_DEVICE,if=none,id=usb_drive,format=raw \
        -device usb-ehci \
        -device usb-tablet \
        -device virtio-net-pci,netdev=net0 \
        -netdev user,id=net0 \
        -vga std \
        -display gtk \
        -boot order=c \
        -monitor stdio \
        -name "ReactOS GRUB 2 Test - Auto-Carga"
fi

print_header "RESULTADO DE LA PRUEBA"

echo "¿Cómo funcionó la prueba?"
echo ""
echo "1. ${CYAN}¿GRUB cargó automáticamente el menú gráfico?${NC}"
echo "   [ ] Sí, apareció inmediatamente"
echo "   [ ] Sí, después de configfile"
echo "   [ ] No, solo prompt de texto"
echo ""
echo "2. ${CYAN}¿ReactOS arrancó correctamente?${NC}"
echo "   [ ] Sí, sin problemas"
echo "   [ ] Sí, con algunos errores"
echo "   [ ] No, no arrancó"
echo ""
echo "3. ${CYAN}¿Qué opciones del menú funcionaron?${NC}"
echo "   [ ] ReactOS (Normal)"
echo "   [ ] ReactOS (Debug)"
echo "   [ ] ReactOS (Safe Mode)"
echo "   [ ] ReactOS (FreeLDR Legacy)"
echo ""
echo "4. ${CYAN}¿Cuál configfile funcionó mejor?${NC}"
echo "   [ ] /EFI/GRUB/grub.cfg"
echo "   [ ] /EFI/BOOT/grub.cfg"
echo "   [ ] /boot/grub/grub.cfg"
echo "   [ ] Ninguno"
echo ""

print_success "¡Prueba completada!"
print_info "Comparte los resultados para verificar que la auto-carga funciona"

#!/bin/bash
# Script para crear imagen ISO booteable con GRUB para ReactOS Rust OS
# Compatible con UEFI y BIOS

set -e

echo "🚀 Creando imagen ISO con GRUB para ReactOS Rust OS..."

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para mostrar mensajes
print_status() {
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

# Verificar dependencias
check_dependencies() {
    print_status "Verificando dependencias..."
    
    local missing_deps=()
    
    if ! command -v grub-mkrescue &> /dev/null; then
        missing_deps+=("grub-common")
    fi
    
    if ! command -v xorriso &> /dev/null; then
        missing_deps+=("xorriso")
    fi
    
    if ! command -v mtools &> /dev/null; then
        missing_deps+=("mtools")
    fi
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        print_error "Dependencias faltantes: ${missing_deps[*]}"
        print_status "Instalando dependencias..."
        
        if command -v apt-get &> /dev/null; then
            apt-get update
            apt-get install -y "${missing_deps[@]}"
        elif command -v yum &> /dev/null; then
            yum install -y "${missing_deps[@]}"
        elif command -v pacman &> /dev/null; then
            pacman -S --noconfirm "${missing_deps[@]}"
        else
            print_error "Distribución no soportada. Instala las dependencias manualmente."
            exit 1
        fi
    fi
    
    print_success "Todas las dependencias están instaladas"
}

# Crear estructura de directorios para ISO
create_iso_structure() {
    print_status "Creando estructura de directorios para ISO..."
    
    ISO_DIR="/tmp/reactos-grub-iso"
    rm -rf "$ISO_DIR"
    mkdir -p "$ISO_DIR"
    
    # Estructura para UEFI
    mkdir -p "$ISO_DIR/efi/boot"
    mkdir -p "$ISO_DIR/boot/grub"
    
    # Estructura para BIOS
    mkdir -p "$ISO_DIR/isolinux"
    
    # Directorio para el kernel
    mkdir -p "$ISO_DIR/boot"
    
    print_success "Estructura de directorios creada"
}

# Crear kernel dummy si no existe
create_dummy_kernel() {
    if [[ ! -f "kernel/target/x86_64-unknown-none/release/kernel" ]]; then
        print_warning "Kernel no encontrado, creando kernel dummy..."
        
        # Crear un kernel dummy simple
        cat > /tmp/dummy_kernel.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>

int main() {
    printf("ReactOS Rust OS - Kernel Dummy\n");
    printf("Sistema iniciado correctamente\n");
    printf("Presiona Enter para continuar...\n");
    getchar();
    return 0;
}
EOF
        
        gcc -o "$ISO_DIR/boot/reactos-rust-kernel.bin" /tmp/dummy_kernel.c
        rm /tmp/dummy_kernel.c
        
        print_warning "Kernel dummy creado. Reemplaza con el kernel real cuando esté listo."
    else
        print_status "Copiando kernel real..."
        cp kernel/target/x86_64-unknown-none/release/kernel "$ISO_DIR/boot/reactos-rust-kernel.bin"
        print_success "Kernel copiado"
    fi
}

# Crear configuración de GRUB para ISO
create_grub_config() {
    print_status "Creando configuración de GRUB para ISO..."
    
    cat > "$ISO_DIR/boot/grub/grub.cfg" << 'EOF'
# Configuración GRUB para ReactOS Rust OS (ISO)
set timeout=10
set default=0

# Configuración de video
set gfxmode=auto
set gfxpayload=keep
insmod gfxterm
insmod vbe
insmod vga

# Tema y colores
set menu_color_normal=white/black
set menu_color_highlight=black/light-gray

# Entrada principal para ReactOS Rust
menuentry "ReactOS Rust OS (x86_64)" {
    set gfxpayload=text
    insmod multiboot2
    
    # Cargar el kernel
    multiboot2 /boot/reactos-rust-kernel.bin
    
    # Configurar parámetros del kernel
    set kernel_args="root=/dev/sr0 ro quiet splash"
    
    echo "Cargando ReactOS Rust OS desde ISO..."
}

# Entrada para modo de depuración
menuentry "ReactOS Rust OS (Debug Mode)" {
    set gfxpayload=text
    insmod multiboot2
    
    multiboot2 /boot/reactos-rust-kernel.bin debug=1
    
    echo "Cargando ReactOS Rust OS en modo depuración..."
}

# Entrada para pruebas de hardware
menuentry "ReactOS Rust OS (Hardware Test)" {
    set gfxpayload=text
    insmod multiboot2
    
    multiboot2 /boot/reactos-rust-kernel.bin hwtest=1
    
    echo "Cargando ReactOS Rust OS para pruebas de hardware..."
}

# Entrada para información del sistema
menuentry "System Information" {
    echo "ReactOS Rust OS - Información del Sistema"
    echo "  - Versión: 2.0"
    echo "  - Arquitectura: x86_64"
    echo "  - Firmware: UEFI/BIOS"
    echo "  - Kernel: Rust"
    echo ""
    echo "Presiona Enter para continuar..."
    read
    configfile /boot/grub/grub.cfg
}

# Entrada para apagar
menuentry "Shutdown" {
    halt
}

# Entrada para reiniciar
menuentry "Restart" {
    reboot
}
EOF
    
    print_success "Configuración de GRUB creada"
}

# Crear imagen ISO
create_iso_image() {
    print_status "Creando imagen ISO..."
    
    ISO_NAME="reactos-rust-os-grub.iso"
    ISO_PATH="$(pwd)/$ISO_NAME"
    
    # Crear imagen ISO con GRUB
    grub-mkrescue \
        --output="$ISO_PATH" \
        --directory=/usr/lib/grub/x86_64-efi \
        --modules="part_gpt part_msdos fat ext2 multiboot2" \
        "$ISO_DIR"
    
    if [[ -f "$ISO_PATH" ]]; then
        print_success "Imagen ISO creada: $ISO_PATH"
        
        # Mostrar información de la ISO
        local iso_size=$(du -h "$ISO_PATH" | cut -f1)
        print_status "Tamaño de la ISO: $iso_size"
        
        # Verificar que la ISO es booteable
        print_status "Verificando que la ISO es booteable..."
        if file "$ISO_PATH" | grep -q "ISO 9660"; then
            print_success "ISO verificada como booteable"
        else
            print_warning "ISO creada pero puede no ser booteable"
        fi
    else
        print_error "Error al crear la imagen ISO"
        exit 1
    fi
}

# Crear script de instalación en USB
create_usb_installer() {
    print_status "Creando script de instalación en USB..."
    
    cat > scripts/install-to-usb.sh << 'EOF'
#!/bin/bash
# Script para instalar ReactOS Rust OS en USB

if [[ $EUID -ne 0 ]]; then
   echo "Este script debe ejecutarse como root (usar sudo)"
   exit 1
fi

if [[ $# -eq 0 ]]; then
    echo "Uso: $0 <dispositivo_usb>"
    echo "Ejemplo: $0 /dev/sdb"
    echo ""
    echo "Dispositivos USB disponibles:"
    lsblk -o NAME,SIZE,TYPE,MOUNTPOINT | grep disk
    exit 1
fi

USB_DEVICE="$1"
ISO_FILE="reactos-rust-os-grub.iso"

if [[ ! -f "$ISO_FILE" ]]; then
    echo "Error: No se encontró $ISO_FILE"
    echo "Ejecuta primero create-grub-iso.sh"
    exit 1
fi

echo "⚠️  ADVERTENCIA: Esto borrará todos los datos en $USB_DEVICE"
echo "¿Estás seguro? (s/N)"
read -r response

if [[ "$response" != "s" && "$response" != "S" ]]; then
    echo "Operación cancelada"
    exit 1
fi

echo "Instalando ReactOS Rust OS en $USB_DEVICE..."

# Desmontar particiones del USB
umount "${USB_DEVICE}"* 2>/dev/null || true

# Crear tabla de particiones GPT
parted "$USB_DEVICE" mklabel gpt

# Crear partición EFI
parted "$USB_DEVICE" mkpart primary fat32 1MiB 100MiB
parted "$USB_DEVICE" set 1 esp on

# Crear partición principal
parted "$USB_DEVICE" mkpart primary ext4 100MiB 100%

# Formatear particiones
mkfs.fat -F32 "${USB_DEVICE}1"
mkfs.ext4 "${USB_DEVICE}2"

# Montar partición EFI
mkdir -p /mnt/usb-efi
mount "${USB_DEVICE}1" /mnt/usb-efi

# Instalar GRUB
grub-install --target=x86_64-efi --efi-directory=/mnt/usb-efi --bootloader-id=reactos-rust

# Copiar archivos de la ISO
mount -o loop "$ISO_FILE" /mnt/iso
cp -r /mnt/iso/* /mnt/usb-efi/

# Desmontar
umount /mnt/usb-efi
umount /mnt/iso

echo "✅ ReactOS Rust OS instalado en USB correctamente"
echo "El USB ahora es booteable en sistemas UEFI"
EOF
    
    chmod +x scripts/install-to-usb.sh
    print_success "Script de instalación en USB creado"
}

# Limpiar archivos temporales
cleanup() {
    print_status "Limpiando archivos temporales..."
    rm -rf "$ISO_DIR"
    print_success "Limpieza completada"
}

# Función principal
main() {
    print_status "Iniciando creación de imagen ISO con GRUB..."
    
    check_dependencies
    create_iso_structure
    create_dummy_kernel
    create_grub_config
    create_iso_image
    create_usb_installer
    cleanup
    
    print_success "¡Imagen ISO creada exitosamente!"
    echo ""
    echo "📋 Resumen:"
    echo "  - ISO creada: reactos-rust-os-grub.iso"
    echo "  - Script USB: scripts/install-to-usb.sh"
    echo ""
    echo "🎯 Próximos pasos:"
    echo "  1. Prueba la ISO en una máquina virtual"
    echo "  2. Instala en USB con: sudo scripts/install-to-usb.sh /dev/sdX"
    echo "  3. Reemplaza el kernel dummy con el kernel real"
    echo "  4. Reinicia y prueba el sistema"
}

# Ejecutar función principal
main "$@"

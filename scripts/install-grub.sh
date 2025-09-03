#!/bin/bash
# Script para instalar y configurar GRUB 2 para ReactOS Rust OS
# Compatible con sistemas UEFI y BIOS

set -e

echo "🚀 Instalando GRUB 2 para ReactOS Rust OS..."

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

# Verificar si estamos ejecutando como root
if [[ $EUID -ne 0 ]]; then
   print_error "Este script debe ejecutarse como root (usar sudo)"
   exit 1
fi

# Verificar si GRUB está instalado
if ! command -v grub-install &> /dev/null; then
    print_status "Instalando GRUB 2..."
    
    # Detectar distribución
    if command -v apt-get &> /dev/null; then
        apt-get update
        apt-get install -y grub-efi-amd64 grub-common
    elif command -v yum &> /dev/null; then
        yum install -y grub2-efi-x64 grub2-tools
    elif command -v pacman &> /dev/null; then
        pacman -S --noconfirm grub
    else
        print_error "Distribución no soportada. Instala GRUB 2 manualmente."
        exit 1
    fi
    
    print_success "GRUB 2 instalado correctamente"
fi

# Detectar si estamos en UEFI o BIOS
if [[ -d /sys/firmware/efi ]]; then
    FIRMWARE="UEFI"
    print_status "Sistema UEFI detectado"
else
    FIRMWARE="BIOS"
    print_status "Sistema BIOS detectado"
fi

# Crear directorio de trabajo
WORK_DIR="/tmp/reactos-grub-setup"
mkdir -p "$WORK_DIR"

# Crear estructura de directorios para GRUB
print_status "Creando estructura de directorios..."
mkdir -p "$WORK_DIR/boot/grub"
mkdir -p "$WORK_DIR/efi/boot"

# Copiar archivos de configuración
print_status "Copiando archivos de configuración..."
cp grub/grub.cfg "$WORK_DIR/boot/grub/"
cp grub/advanced.cfg "$WORK_DIR/boot/grub/"

# Crear un kernel dummy para pruebas (si no existe)
if [[ ! -f "$WORK_DIR/boot/reactos-rust-kernel.bin" ]]; then
    print_status "Creando kernel dummy para pruebas..."
    # Crear un binario simple que solo haga halt
    cat > "$WORK_DIR/boot/reactos-rust-kernel.bin" << 'EOF'
# Kernel dummy de ReactOS Rust
# Este es un placeholder hasta que tengamos el kernel real
EOF
    print_warning "Kernel dummy creado. Reemplaza con el kernel real cuando esté listo."
fi

# Función para instalar GRUB en UEFI
install_uefi_grub() {
    print_status "Instalando GRUB para UEFI..."
    
    # Buscar partición EFI
    EFI_PARTITION=$(lsblk -o NAME,MOUNTPOINT | grep -E '/boot/efi|/efi' | head -1 | awk '{print $1}')
    
    if [[ -z "$EFI_PARTITION" ]]; then
        print_error "No se encontró partición EFI. Crea una partición EFI primero."
        exit 1
    fi
    
    print_status "Partición EFI encontrada: $EFI_PARTITION"
    
    # Montar partición EFI
    EFI_MOUNT="/mnt/efi"
    mkdir -p "$EFI_MOUNT"
    mount "/dev/$EFI_PARTITION" "$EFI_MOUNT"
    
    # Instalar GRUB
    grub-install --target=x86_64-efi --efi-directory="$EFI_MOUNT" --bootloader-id=reactos-rust
    
    # Copiar archivos de configuración
    cp -r "$WORK_DIR/boot" "$EFI_MOUNT/"
    
    # Desmontar
    umount "$EFI_MOUNT"
    
    print_success "GRUB instalado en UEFI"
}

# Función para instalar GRUB en BIOS
install_bios_grub() {
    print_status "Instalando GRUB para BIOS..."
    
    # Detectar disco principal
    MAIN_DISK=$(lsblk -o NAME,TYPE | grep disk | head -1 | awk '{print $1}')
    
    if [[ -z "$MAIN_DISK" ]]; then
        print_error "No se pudo detectar disco principal"
        exit 1
    fi
    
    print_status "Disco principal detectado: $MAIN_DISK"
    
    # Instalar GRUB
    grub-install --target=i386-pc "/dev/$MAIN_DISK"
    
    # Copiar archivos de configuración
    cp -r "$WORK_DIR/boot" /
    
    # Actualizar configuración de GRUB
    update-grub 2>/dev/null || grub-mkconfig -o /boot/grub/grub.cfg
    
    print_success "GRUB instalado en BIOS"
}

# Instalar según el tipo de firmware
if [[ "$FIRMWARE" == "UEFI" ]]; then
    install_uefi_grub
else
    install_bios_grub
fi

# Crear script de actualización
print_status "Creando script de actualización..."
cat > /usr/local/bin/update-reactos-grub << 'EOF'
#!/bin/bash
# Script para actualizar GRUB después de cambios en el kernel

echo "Actualizando configuración de GRUB para ReactOS Rust..."

# Copiar nueva configuración
cp /path/to/reactos-rust-os/grub/grub.cfg /boot/grub/
cp /path/to/reactos-rust-os/grub/advanced.cfg /boot/grub/

# Actualizar GRUB
if command -v update-grub &> /dev/null; then
    update-grub
else
    grub-mkconfig -o /boot/grub/grub.cfg
fi

echo "GRUB actualizado correctamente"
EOF

chmod +x /usr/local/bin/update-reactos-grub

# Limpiar directorio temporal
rm -rf "$WORK_DIR"

print_success "Instalación de GRUB completada!"
print_status "Para actualizar la configuración, ejecuta: update-reactos-grub"
print_status "Reinicia el sistema para probar el nuevo bootloader"

echo ""
echo "📋 Resumen de la instalación:"
echo "  - Firmware: $FIRMWARE"
echo "  - Configuración: /boot/grub/grub.cfg"
echo "  - Configuración avanzada: /boot/grub/advanced.cfg"
echo "  - Script de actualización: /usr/local/bin/update-reactos-grub"
echo ""
echo "🎯 Próximos pasos:"
echo "  1. Compila el kernel de ReactOS Rust"
echo "  2. Copia el kernel a /boot/reactos-rust-kernel.bin"
echo "  3. Ejecuta update-reactos-grub"
echo "  4. Reinicia y prueba el sistema"

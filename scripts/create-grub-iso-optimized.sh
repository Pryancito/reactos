#!/bin/bash

# Script optimizado para crear ISO con GRUB para ReactOS Rust OS
# Autor: ReactOS Rust Team

set -e

echo "🚀 ReactOS Rust OS - Creación de ISO con GRUB Optimizado"
echo "========================================================"
echo

# Cambiar al directorio raíz del proyecto
cd "$(dirname "$0")/.."

echo "📁 Directorio de trabajo: $(pwd)"
echo

# Configuración
ISO_NAME="reactos-rust-os-optimized.iso"
ISO_DIR="iso-build"
BOOT_DIR="$ISO_DIR/boot"
GRUB_DIR="$BOOT_DIR/grub"
APPS_DIR="$ISO_DIR/apps"
SYSTEM_DIR="$ISO_DIR/system32"

# Función para mostrar progreso
show_progress() {
    echo "🔧 $1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Función para limpiar directorios
cleanup() {
    show_progress "Limpiando directorios anteriores"
    rm -rf "$ISO_DIR"
    mkdir -p "$ISO_DIR"
    echo "✅ Directorios limpiados"
    echo
}

# Función para crear estructura de directorios
create_structure() {
    show_progress "Creando estructura de directorios"
    mkdir -p "$BOOT_DIR"
    mkdir -p "$GRUB_DIR"
    mkdir -p "$APPS_DIR"
    mkdir -p "$SYSTEM_DIR"
    echo "✅ Estructura de directorios creada"
    echo
}

# Función para copiar archivos de GRUB
copy_grub_files() {
    show_progress "Copiando archivos de configuración GRUB"
    
    # Copiar configuración principal
    cp "grub/grub.cfg" "$GRUB_DIR/"
    cp "grub/advanced.cfg" "$GRUB_DIR/"
    cp "grub/applications.cfg" "$GRUB_DIR/"
    
    echo "✅ Archivos de GRUB copiados"
    echo
}

# Función para copiar kernel y aplicaciones
copy_system_files() {
    show_progress "Copiando kernel y aplicaciones"
    
    # Copiar kernel si existe
    if [ -f "target/x86_64-unknown-linux-gnu/release/reactos-rust-kernel" ]; then
        cp "target/x86_64-unknown-linux-gnu/release/reactos-rust-kernel" "$BOOT_DIR/reactos-rust-kernel.bin"
        echo "✅ Kernel copiado"
    else
        echo "⚠️  Kernel no encontrado, creando placeholder"
        echo "#!/bin/bash" > "$BOOT_DIR/reactos-rust-kernel.bin"
        echo "echo 'ReactOS Rust Kernel Placeholder'" >> "$BOOT_DIR/reactos-rust-kernel.bin"
        chmod +x "$BOOT_DIR/reactos-rust-kernel.bin"
    fi
    
    # Copiar aplicaciones si existen
    if [ -f "target/x86_64-unknown-linux-gnu/release/calc64" ]; then
        cp "target/x86_64-unknown-linux-gnu/release/calc64" "$APPS_DIR/calc64.exe"
        echo "✅ calc64.exe copiado"
    fi
    
    if [ -f "target/x86_64-unknown-linux-gnu/release/hello64" ]; then
        cp "target/x86_64-unknown-linux-gnu/release/hello64" "$APPS_DIR/hello64.exe"
        echo "✅ hello64.exe copiado"
    fi
    
    # Copiar aplicaciones 32-bit si existen
    if [ -f "target/i686-unknown-linux-gnu/release/test32" ]; then
        cp "target/i686-unknown-linux-gnu/release/test32" "$APPS_DIR/test32.exe"
        echo "✅ test32.exe copiado"
    fi
    
    echo
}

# Función para crear initrd si es necesario
create_initrd() {
    show_progress "Creando initrd básico"
    
    # Crear un initrd básico
    INITRD_DIR="initrd-temp"
    mkdir -p "$INITRD_DIR"
    
    # Crear script de inicialización básico
    cat > "$INITRD_DIR/init" << 'EOF'
#!/bin/bash
echo "ReactOS Rust OS - Inicializando sistema..."
echo "Cargando drivers..."
echo "Montando sistemas de archivos..."
echo "Iniciando servicios del sistema..."
echo "Sistema listo!"
EOF
    
    chmod +x "$INITRD_DIR/init"
    
    # Crear initrd
    cd "$INITRD_DIR"
    find . | cpio -o -H newc | gzip > "../$BOOT_DIR/initrd.img"
    cd ..
    rm -rf "$INITRD_DIR"
    
    echo "✅ initrd creado"
    echo
}

# Función para crear ISO
create_iso() {
    show_progress "Creando ISO con GRUB"
    
    # Verificar que grub-mkrescue esté disponible
    if ! command -v grub-mkrescue &> /dev/null; then
        echo "❌ grub-mkrescue no encontrado. Instalando GRUB..."
        if command -v apt-get &> /dev/null; then
            sudo apt-get update && sudo apt-get install -y grub-common grub-pc-bin
        elif command -v yum &> /dev/null; then
            sudo yum install -y grub2-tools
        else
            echo "❌ No se pudo instalar GRUB automáticamente"
            exit 1
        fi
    fi
    
    # Crear ISO
    grub-mkrescue -o "$ISO_NAME" "$ISO_DIR" --compress=xz
    
    if [ $? -eq 0 ]; then
        echo "✅ ISO creada exitosamente: $ISO_NAME"
    else
        echo "❌ Error creando ISO"
        exit 1
    fi
    echo
}

# Función para mostrar información de la ISO
show_iso_info() {
    show_progress "Información de la ISO creada"
    
    if [ -f "$ISO_NAME" ]; then
        echo "📁 Archivo: $ISO_NAME"
        echo "📊 Tamaño: $(du -h "$ISO_NAME" | cut -f1)"
        echo "📅 Fecha: $(date)"
        echo
        echo "🎯 Contenido de la ISO:"
        echo "  • /boot/grub/grub.cfg - Configuración principal"
        echo "  • /boot/grub/advanced.cfg - Opciones avanzadas"
        echo "  • /boot/grub/applications.cfg - Menú de aplicaciones"
        echo "  • /boot/reactos-rust-kernel.bin - Kernel del sistema"
        echo "  • /boot/initrd.img - Imagen de inicialización"
        echo "  • /apps/ - Aplicaciones del sistema"
        echo "  • /system32/ - Librerías del sistema"
        echo
        echo "🚀 Para probar la ISO:"
        echo "  qemu-system-x86_64 -cdrom $ISO_NAME -m 512"
        echo "  o grabar en USB/CD y arrancar desde el dispositivo"
    else
        echo "❌ ISO no encontrada"
    fi
    echo
}

# Función principal
main() {
    echo "🎯 Iniciando creación de ISO optimizada..."
    echo
    
    cleanup
    create_structure
    copy_grub_files
    copy_system_files
    create_initrd
    create_iso
    show_iso_info
    
    echo "🎉 ¡ISO con GRUB optimizada creada exitosamente!"
    echo
    echo "📋 Próximos pasos:"
    echo "  1. Probar la ISO en QEMU"
    echo "  2. Verificar que el kernel se carga correctamente"
    echo "  3. Probar las aplicaciones desde el menú"
    echo "  4. Optimizar configuración según necesidades"
}

# Ejecutar función principal
main

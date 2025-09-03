#!/bin/bash

# =============================================================================
# Script de Construcción Completo para Eclipse OS
# =============================================================================
# Este script compila el sistema Eclipse OS y genera una ISO booteable
# Autor: Eclipse OS Team
# Versión: 1.0
# =============================================================================

set -e  # Salir si cualquier comando falla

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Función para imprimir mensajes con colores
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

print_header() {
    echo -e "${PURPLE}==============================================================================${NC}"
    echo -e "${PURPLE} $1${NC}"
    echo -e "${PURPLE}==============================================================================${NC}"
}

# Banner del sistema
show_banner() {
    clear
    echo -e "${CYAN}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                    🌙 Eclipse OS Builder                     ║"
    echo "║                                                              ║"
    echo "║  Script de Construcción Completo                             ║"
    echo "║  Compilación + ISO Booteable                                 ║"
    echo "║  Versión: 1.0                                                ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

# Verificar dependencias
check_dependencies() {
    print_header "Verificando Dependencias"
    
    local missing_deps=()
    
    # Verificar Rust
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("rust")
    else
        print_success "Rust/Cargo encontrado: $(cargo --version)"
    fi
    
    # Verificar genisoimage
    if ! command -v genisoimage &> /dev/null; then
        if ! command -v mkisofs &> /dev/null; then
            missing_deps+=("genisoimage o mkisofs")
        else
            print_success "mkisofs encontrado: $(mkisofs --version | head -1)"
        fi
    else
        print_success "genisoimage encontrado: $(genisoimage --version | head -1)"
    fi
    
    # Verificar grub-mkrescue
    if ! command -v grub-mkrescue &> /dev/null; then
        missing_deps+=("grub-mkrescue")
    else
        print_success "grub-mkrescue encontrado: $(grub-mkrescue --version | head -1)"
    fi
    
    # Verificar cpio
    if ! command -v cpio &> /dev/null; then
        missing_deps+=("cpio")
    else
        print_success "cpio encontrado: $(cpio --version | head -1)"
    fi
    
    # Verificar gzip
    if ! command -v gzip &> /dev/null; then
        missing_deps+=("gzip")
    else
        print_success "gzip encontrado: $(gzip --version | head -1)"
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Dependencias faltantes:"
        for dep in "${missing_deps[@]}"; do
            echo "  - $dep"
        done
        echo ""
        print_error "Instale las dependencias faltantes antes de continuar."
        exit 1
    fi
    
    print_success "Todas las dependencias están disponibles"
}

# Limpiar compilaciones anteriores
clean_build() {
    print_header "Limpiando Compilaciones Anteriores"
    
    if [ -d "target" ]; then
        print_status "Eliminando directorio target..."
        rm -rf target
        print_success "Directorio target eliminado"
    fi
    
    if [ -d "iso" ]; then
        print_status "Eliminando directorio iso..."
        rm -rf iso
        print_success "Directorio iso eliminado"
    fi
    
    if [ -f "eclipse-os.iso" ]; then
        print_status "Eliminando ISO anterior..."
        rm -f eclipse-os.iso
        print_success "ISO anterior eliminada"
    fi
}

# Compilar el sistema
compile_system() {
    print_header "Compilando Eclipse OS"
    
    print_status "Iniciando compilación con Cargo..."
    
    # Compilar en modo release para mejor rendimiento
    if cargo build --release --bin eclipse-os --bin init; then
        print_success "Compilación exitosa"
    else
        print_error "Error en la compilación"
        exit 1
    fi
    
    # Verificar que el binario existe
    if [ -f "target/release/eclipse-os" ]; then
        print_success "Binario generado: target/release/eclipse-os"
        ls -lh target/release/eclipse-os
    else
        print_error "Binario no encontrado después de la compilación"
        exit 1
    fi
}

# Crear estructura de directorios para ISO
create_iso_structure() {
    print_header "Creando Estructura de Directorios para ISO"
    
    # Crear directorios
    mkdir -p iso/boot/grub
    mkdir -p iso/bin
    mkdir -p iso/sbin
    mkdir -p iso/usr/bin
    mkdir -p iso/etc
    mkdir -p iso/var/log
    mkdir -p iso/tmp
    mkdir -p iso/home/user
    
    print_success "Estructura de directorios creada"

    # Instalar binario principal y /init en el rootfs de la ISO
    if [ -f "target/release/eclipse-os" ]; then
        cp target/release/eclipse-os iso/eclipse-os
        chmod +x iso/eclipse-os
        # Crear /init que delega en eclipse-os cuando se usa la ISO como rootfs
        cat > iso/init << 'EOF'
#!/bin/sh
echo "🌙 Eclipse OS: init desde rootfs de ISO"
if [ -x /eclipse-os ]; then
    exec /eclipse-os
fi
echo "❌ /eclipse-os no encontrado, abriendo shell de emergencia"
exec /bin/sh
EOF
        chmod +x iso/init
        ln -sf /init iso/bin/init 2>/dev/null || true
        ln -sf /init iso/sbin/init 2>/dev/null || true
    else
        print_warning "Binario eclipse-os no encontrado aún; se omitirá copiar a ISO root"
    fi
}

# Crear initrd
create_initrd() {
    print_header "Creando Initrd"
    
    # Crear directorio temporal para initrd
    local initrd_dir="initrd_temp"
    rm -rf "$initrd_dir"
    mkdir -p "$initrd_dir"
    
    # Crear estructura de directorios completa del rootfs
    print_status "Creando estructura de directorios del rootfs..."
    mkdir -p "$initrd_dir"/{bin,sbin,etc,dev,proc,sys,tmp,mnt,usr/{bin,sbin,lib},var,run,lib,lib64,home,root}
    
    # Copiar binario principal
    cp target/release/eclipse-os "$initrd_dir/eclipse-os"
    chmod +x "$initrd_dir/eclipse-os"
    
    # Intentar usar busybox para reducir el tamaño
    local use_busybox=false
    if command -v busybox &> /dev/null; then
        print_status "Busybox encontrado, usándolo para reducir el tamaño del initrd..."
        cp "$(which busybox)" "$initrd_dir/bin/busybox"
        chmod +x "$initrd_dir/bin/busybox"
        use_busybox=true
        
        # Crear enlaces simbólicos para busybox
        local busybox_commands=(
            "sh" "bash" "mount" "umount" "mkdir" "mknod" "ls" "cat" "echo" "true" "false"
            "chmod" "chown" "ln" "rm" "cp" "mv" "ps" "kill" "sleep" "date" "which"
            "find" "grep" "sed" "awk" "cut" "sort" "uniq" "wc" "head" "tail"
            "less" "more" "vi" "nano" "ash" "init"
        )
        
        for cmd in "${busybox_commands[@]}"; do
            ln -sf /bin/busybox "$initrd_dir/bin/$cmd"
        done
        
        print_success "Busybox configurado con ${#busybox_commands[@]} comandos"
    else
        print_warning "Busybox no encontrado, copiando herramientas individuales..."
        
        # Copiar herramientas esenciales del sistema con sus dependencias
        local essential_tools=(
            "/bin/bash"
            "/bin/sh"
            "/bin/mount"
            "/bin/umount"
            "/bin/mkdir"
            "/bin/mknod"
            "/bin/ls"
            "/bin/cat"
            "/bin/echo"
            "/bin/true"
            "/bin/false"
            "/bin/chmod"
            "/bin/chown"
            "/bin/ln"
            "/bin/rm"
            "/bin/cp"
            "/bin/mv"
            "/bin/ps"
            "/bin/kill"
            "/bin/sleep"
            "/bin/date"
            "/bin/which"
            "/bin/whereis"
            "/bin/find"
            "/bin/grep"
            "/bin/sed"
            "/bin/awk"
            "/bin/cut"
            "/bin/sort"
            "/bin/uniq"
            "/bin/wc"
            "/bin/head"
            "/bin/tail"
            "/bin/less"
            "/bin/more"
            "/bin/vi"
            "/bin/nano"
        )
        
        print_status "Copiando herramientas esenciales..."
        for tool in "${essential_tools[@]}"; do
            if [ -f "$tool" ]; then
                cp "$tool" "$initrd_dir/bin/"
                chmod +x "$initrd_dir/bin/$(basename "$tool")"
                print_status "  Copiado: $(basename "$tool")"
            else
                print_warning "  No encontrado: $tool"
            fi
        done
    fi
    
    # Copiar librerías esenciales de forma optimizada
    print_status "Copiando librerías esenciales..."
    
    if [ "$use_busybox" = true ]; then
        # Para busybox, solo necesitamos librerías básicas
        local basic_libs=(
            "/lib/x86_64-linux-gnu/libc.so.6"
            "/lib/x86_64-linux-gnu/libm.so.6"
            "/lib/x86_64-linux-gnu/libdl.so.2"
            "/lib/x86_64-linux-gnu/libpthread.so.0"
        )
        
        mkdir -p "$initrd_dir/lib/x86_64-linux-gnu"
        for lib in "${basic_libs[@]}"; do
            if [ -f "$lib" ]; then
                cp "$lib" "$initrd_dir/lib/x86_64-linux-gnu/"
                print_status "  Copiada librería básica: $(basename "$lib")"
            fi
        done
    else
        # Para herramientas individuales, necesitamos más librerías
        local essential_libs=(
            "/lib/x86_64-linux-gnu/libc.so.6"
            "/lib/x86_64-linux-gnu/libm.so.6"
            "/lib/x86_64-linux-gnu/libdl.so.2"
            "/lib/x86_64-linux-gnu/libpthread.so.0"
            "/lib/x86_64-linux-gnu/librt.so.1"
            "/lib/x86_64-linux-gnu/libcrypt.so.1"
            "/lib/x86_64-linux-gnu/libnsl.so.1"
            "/lib/x86_64-linux-gnu/libresolv.so.2"
            "/lib/x86_64-linux-gnu/libutil.so.1"
            "/lib/x86_64-linux-gnu/libreadline.so.8"
            "/lib/x86_64-linux-gnu/libncurses.so.6"
            "/lib/x86_64-linux-gnu/libncursesw.so.6"
            "/lib/x86_64-linux-gnu/libhistory.so.8"
            "/lib/x86_64-linux-gnu/libedit.so.2"
            "/lib/x86_64-linux-gnu/libselinux.so.1"
            "/lib/x86_64-linux-gnu/libpcre2-8.so.0"
            "/lib/x86_64-linux-gnu/libz.so.1"
            "/lib/x86_64-linux-gnu/libbz2.so.1.0"
            "/lib/x86_64-linux-gnu/liblzma.so.5"
            "/lib/x86_64-linux-gnu/libzstd.so.1"
            "/lib/x86_64-linux-gnu/libgcc_s.so.1"
            "/lib/x86_64-linux-gnu/libstdc++.so.6"
        )
        
        mkdir -p "$initrd_dir/lib/x86_64-linux-gnu"
        for lib in "${essential_libs[@]}"; do
            if [ -f "$lib" ]; then
                cp "$lib" "$initrd_dir/lib/x86_64-linux-gnu/"
                print_status "  Copiada librería: $(basename "$lib")"
            else
                print_warning "  Librería no encontrada: $lib"
            fi
        done
    fi
    
    # Crear enlaces simbólicos para librerías
    cd "$initrd_dir/lib/x86_64-linux-gnu"
    ln -sf libc.so.6 libc.so 2>/dev/null || true
    ln -sf libm.so.6 libm.so 2>/dev/null || true
    ln -sf libdl.so.2 libdl.so 2>/dev/null || true
    ln -sf libpthread.so.0 libpthread.so 2>/dev/null || true
    ln -sf librt.so.1 librt.so 2>/dev/null || true
    ln -sf libcrypt.so.1 libcrypt.so 2>/dev/null || true
    ln -sf libnsl.so.1 libnsl.so 2>/dev/null || true
    ln -sf libresolv.so.2 libresolv.so 2>/dev/null || true
    ln -sf libutil.so.1 libutil.so 2>/dev/null || true
    cd - > /dev/null
    
    # Crear directorios adicionales
    mkdir -p "$initrd_dir/etc" "$initrd_dir/proc" "$initrd_dir/sys" "$initrd_dir/tmp" "$initrd_dir/dev"
    
    # Crear script de inicio optimizado siguiendo mejores prácticas
    cat > "$initrd_dir/init" << 'EOF'
#!/bin/sh
# Script de inicialización para Eclipse OS
# Sigue las mejores prácticas para init de sistemas embebidos

set -e

echo "🌙 Iniciando Eclipse OS..."
echo "Cargando sistema..."

# Configurar entorno básico
export PATH=/bin:/usr/bin:/sbin:/usr/sbin
export HOME=/root
export USER=root
export TERM=xterm
export LD_LIBRARY_PATH=/lib/x86_64-linux-gnu

# Montar sistemas de archivos esenciales (orden crítico)
echo "Montando sistemas de archivos esenciales..."
/bin/mount -t proc proc /proc
/bin/mount -t sysfs sysfs /sys
/bin/mount -t devtmpfs devtmpfs /dev
/bin/mount -t tmpfs tmpfs /tmp

# Configurar dispositivos básicos
echo "Configurando dispositivos básicos..."
[ ! -e /dev/console ] && /bin/mknod /dev/console c 5 1
[ ! -e /dev/null ] && /bin/mknod /dev/null c 1 3
[ ! -e /dev/zero ] && /bin/mknod /dev/zero c 1 5
[ ! -e /dev/tty ] && /bin/mknod /dev/tty c 5 0
[ ! -e /dev/tty1 ] && /bin/mknod /dev/tty1 c 4 1
[ ! -e /dev/tty2 ] && /bin/mknod /dev/tty2 c 4 2

# Configurar red básica
echo "Configurando red básica..."
echo "127.0.0.1 localhost" > /etc/hosts
echo "eclipse-os" > /etc/hostname

# Crear enlaces simbólicos para librerías
echo "Configurando librerías..."
ln -sf /lib/x86_64-linux-gnu/libc.so.6 /lib/libc.so.6
ln -sf /lib/x86_64-linux-gnu/libm.so.6 /lib/libm.so.6
ln -sf /lib/x86_64-linux-gnu/libdl.so.2 /lib/libdl.so.2
ln -sf /lib/x86_64-linux-gnu/libpthread.so.0 /lib/libpthread.so.0

echo "✅ Sistema cargado correctamente"
echo "🚀 Iniciando Eclipse OS Shell..."

# Verificar y ejecutar Eclipse OS
if [ -x /eclipse-os ]; then
    echo "Ejecutando Eclipse OS..."
    exec /eclipse-os
else
    echo "❌ Error: /eclipse-os no encontrado o no ejecutable"
    echo "📁 Archivos disponibles en /:"
    /bin/ls -la /
    echo ""
    echo "📁 Archivos disponibles en /bin:"
    /bin/ls -la /bin
    echo ""
    echo "📁 Librerías disponibles en /lib:"
    /bin/ls -la /lib
    echo ""
    echo "🔧 Iniciando shell de emergencia..."
    exec /bin/sh
fi
EOF
    
    chmod +x "$initrd_dir/init"

    # Crear alias estándar para init
    ln -sf /init "$initrd_dir/sbin/init"
    ln -sf /init "$initrd_dir/bin/init"

    # Crear initrd con cpio y gzip
    print_status "Empaquetando initrd..."
    cd "$initrd_dir"
    find . | cpio -o -H newc | gzip > ../iso/boot/initrd.img
    cd ..
    
    # Limpiar directorio temporal
    rm -rf "$initrd_dir"
    
    print_success "Initrd creado: iso/boot/initrd.img"
    ls -lh iso/boot/initrd.img
}

# Crear configuración GRUB
create_grub_config() {
    print_header "Creando Configuración GRUB"
    
    cat > iso/boot/grub/grub.cfg << 'EOF'
# Configuración GRUB para Eclipse OS
set timeout=10
set default=0

menuentry "Eclipse OS en Rust" {
    echo "Cargando Eclipse OS..."
    multiboot /boot/vmlinuz-eclipse
    module /boot/initrd.img
}

menuentry "Eclipse OS (Modo Debug)" {
    echo "Cargando Eclipse OS en modo debug..."
    multiboot /boot/vmlinuz-eclipse debug
    module /boot/initrd.img
}

menuentry "Eclipse OS (Recuperación)" {
    echo "Cargando Eclipse OS en modo recuperación..."
    multiboot /boot/vmlinuz-eclipse recovery
    module /boot/initrd.img
}

menuentry "Eclipse OS (Solo Kernel)" {
    echo "Cargando Eclipse OS sin initrd..."
    multiboot /boot/vmlinuz-eclipse
}
EOF
    
    print_success "Configuración GRUB creada"
}

# Crear kernel funcional
create_working_kernel() {
    print_header "Creando Kernel de Eclipse OS"
    
    # Compilar kernel Rust si no existe
    if [ ! -f "eclipse-kernel" ]; then
        print_status "Compilando kernel Rust..."
        if cargo build --release --bin eclipse-kernel; then
            print_success "Kernel Rust compilado exitosamente"
        else
            print_error "Error al compilar kernel Rust"
            exit 1
        fi
    fi
    
    # Usar el kernel de Eclipse OS
    if [ -f "target/release/eclipse-kernel" ]; then
        print_status "Usando kernel Rust compilado..."
        cp target/release/eclipse-kernel iso/boot/vmlinuz-eclipse
        chmod +x iso/boot/vmlinuz-eclipse
        print_success "Kernel de Eclipse OS creado exitosamente"
    elif [ -f "eclipse-kernel" ]; then
        print_status "Usando eclipse-kernel existente..."
        cp eclipse-kernel iso/boot/vmlinuz-eclipse
        chmod +x iso/boot/vmlinuz-eclipse
        print_success "Kernel de Eclipse OS creado exitosamente"
    else
        print_error "Binario eclipse-kernel no encontrado. Compile primero el kernel."
        exit 1
    fi
    
    ls -lh iso/boot/vmlinuz-eclipse
}

# Generar ISO booteable
generate_iso() {
    print_header "Generando ISO Booteable"
    
    print_status "Creando ISO con GRUB..."
    
    # Usar grub-mkrescue para crear ISO booteable
    if grub-mkrescue -o eclipse-os.iso iso/; then
        print_success "ISO generada exitosamente: eclipse-os.iso"
        ls -lh eclipse-os.iso
    else
        print_error "Error al generar ISO"
        exit 1
    fi
}

# Verificar ISO
verify_iso() {
    print_header "Verificando ISO"
    
    if [ -f "eclipse-os.iso" ]; then
        print_success "ISO encontrada: eclipse-os.iso"
        
        # Mostrar información de la ISO
        print_status "Información de la ISO:"
        file eclipse-os.iso
        ls -lh eclipse-os.iso
        
        # Verificar que es booteable
        if file eclipse-os.iso | grep -q "ISO 9660"; then
            print_success "ISO es válida (ISO 9660)"
        else
            print_warning "ISO puede no ser válida"
        fi
        
    else
        print_error "ISO no encontrada"
        exit 1
    fi
}

# Mostrar instrucciones de uso
show_usage_instructions() {
    print_header "Instrucciones de Uso"
    
    echo -e "${CYAN}🌙 Eclipse OS ISO Generada Exitosamente!${NC}"
    echo ""
    echo -e "${YELLOW}Para probar la ISO:${NC}"
    echo "  1. Con QEMU:"
    echo "     qemu-system-x86_64 -cdrom eclipse-os.iso -m 512M"
    echo ""
    echo "  2. Con VirtualBox:"
    echo "     - Crear nueva VM"
    echo "     - Seleccionar eclipse-os.iso como CD de arranque"
    echo ""
    echo "  3. Grabar en CD/DVD:"
    echo "     - Usar cualquier software de grabación"
    echo "     - Grabar eclipse-os.iso como imagen de disco"
    echo ""
    echo -e "${GREEN}¡Eclipse OS está listo para usar!${NC}"
}

# Función principal
main() {
    show_banner
    
    print_status "Iniciando proceso de construcción completo..."
    echo ""
    
    # Ejecutar pasos en orden
    check_dependencies
    echo ""
    
    clean_build
    echo ""
    
    compile_system
    echo ""
    
    create_iso_structure
    echo ""
    
    create_initrd
    echo ""
    
    create_grub_config
    echo ""
    
    create_working_kernel
    echo ""
    
    generate_iso
    echo ""
    
    verify_iso
    echo ""
    
    show_usage_instructions
    
    print_success "¡Construcción completada exitosamente!"
}

# Ejecutar función principal
main "$@"

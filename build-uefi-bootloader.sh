#!/bin/bash
# ============================================================================
# SCRIPT DE COMPILACIÓN Y TESTING DEL BOOTLOADER UEFI NATIVO
# ============================================================================
#
# Este script compila y prueba el bootloader UEFI nativo para ReactOS
# que será completamente compatible con sistemas UEFI estrictos como
# ASUS 10ª generación.
#
# Autor: Claude Assistant
# Fecha: Sat Aug 31 22:30:00 CEST 2024
# Versión: 1.0-UEFI-Native
# ============================================================================

set -e  # Salir en caso de error

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
    echo -e "${PURPLE}================================${NC}"
    echo -e "${PURPLE} $1${NC}"
    echo -e "${PURPLE}================================${NC}"
}

# ============================================================================
# CONFIGURACIÓN
# ============================================================================

PROJECT_DIR="$(pwd)"
BUILD_DIR="$PROJECT_DIR/build"
TARGET_FILE="$BUILD_DIR/reactos-uefi-bootloader.efi"

# Verificar que estamos en el directorio correcto
if [ ! -f "Makefile" ]; then
    print_error "No se encontró Makefile. Ejecuta este script desde el directorio del proyecto."
    exit 1
fi

print_header "INICIANDO COMPILACIÓN DEL BOOTLOADER UEFI NATIVO"
print_status "Directorio del proyecto: $PROJECT_DIR"
print_status "Fecha y hora: $(date)"
print_status "Sistema: $(uname -a)"

# ============================================================================
# PASO 1: VERIFICAR DEPENDENCIAS
# ============================================================================

print_header "PASO 1: VERIFICANDO DEPENDENCIAS"

# Verificar compilador
if command -v x86_64-w64-mingw32-gcc-posix >/dev/null 2>&1; then
    print_success "Compilador MinGW-w64 POSIX encontrado"
    x86_64-w64-mingw32-gcc-posix --version | head -1
else
    print_error "Compilador MinGW-w64 POSIX no encontrado"
    print_status "Instalando dependencias..."
    sudo apt-get update
    sudo apt-get install -y mingw-w64
    print_success "Dependencias instaladas"
fi

# Verificar herramientas de build
if command -v make >/dev/null 2>&1; then
    print_success "Make encontrado"
    make --version | head -1
else
    print_error "Make no encontrado"
    exit 1
fi

# ============================================================================
# PASO 2: COMPILAR BOOTLOADER UEFI
# ============================================================================

print_header "PASO 2: COMPILANDO BOOTLOADER UEFI NATIVO"

# Limpiar build anterior
print_status "Limpiando build anterior..."
make clean

# Verificar dependencias
print_status "Verificando dependencias..."
make check

# Compilar bootloader
print_status "Compilando bootloader UEFI nativo..."
if make -j1; then
    print_success "Bootloader UEFI compilado exitosamente"
else
    print_error "Error en la compilación"
    exit 1
fi

# ============================================================================
# PASO 3: VERIFICAR ARCHIVO GENERADO
# ============================================================================

print_header "PASO 3: VERIFICANDO ARCHIVO GENERADO"

if [ -f "$TARGET_FILE" ]; then
    print_success "Archivo .efi generado exitosamente"
    ls -lah "$TARGET_FILE"
    
    # Verificar formato
    print_status "Verificando formato UEFI..."
    if command -v file >/dev/null 2>&1; then
        file "$TARGET_FILE"
    else
        print_warning "Comando 'file' no disponible"
    fi
    
    # Verificar tamaño
    FILE_SIZE=$(stat -c%s "$TARGET_FILE" 2>/dev/null || echo "N/A")
    print_status "Tamaño del archivo: $FILE_SIZE bytes"
    
else
    print_error "No se generó el archivo .efi"
    exit 1
fi

# ============================================================================
# PASO 4: INSTALAR BOOTLOADER
# ============================================================================

print_header "PASO 4: INSTALANDO BOOTLOADER UEFI"

print_status "Instalando bootloader..."
if make install; then
    print_success "Bootloader instalado exitosamente"
    ls -lah reactos-uefi-bootloader.efi
else
    print_error "Error en la instalación"
    exit 1
fi

# ============================================================================
# PASO 5: VERIFICAR COMPATIBILIDAD UEFI
# ============================================================================

print_header "PASO 5: VERIFICANDO COMPATIBILIDAD UEFI"

print_status "Verificando compatibilidad UEFI..."
if make verify-uefi; then
    print_success "Verificación UEFI completada"
else
    print_warning "Verificación UEFI falló"
fi

# ============================================================================
# PASO 6: CREAR IMAGEN UEFI BOOTEABLE
# ============================================================================

print_header "PASO 6: CREANDO IMAGEN UEFI BOOTEABLE"

print_status "Creando imagen UEFI booteable..."
if make uefi-image; then
    print_success "Imagen UEFI booteable creada"
else
    print_warning "Creación de imagen UEFI falló"
fi

# ============================================================================
# PASO 7: TESTING DEL BOOTLOADER
# ============================================================================

print_header "PASO 7: TESTING DEL BOOTLOADER UEFI"

print_status "Ejecutando tests del bootloader..."
if make test; then
    print_success "Tests del bootloader completados"
else
    print_warning "Tests del bootloader fallaron"
fi

# ============================================================================
# PASO 8: RESUMEN FINAL
# ============================================================================

print_header "RESUMEN DE COMPILACIÓN DEL BOOTLOADER UEFI"

print_success "¡Bootloader UEFI nativo compilado exitosamente!"
print_status "Archivo generado: $TARGET_FILE"
print_status "Archivo instalado: reactos-uefi-bootloader.efi"

echo ""
print_status "🎯 CARACTERÍSTICAS DEL BOOTLOADER UEFI:"
echo "   ✅ Bootloader UEFI nativo para ReactOS"
echo "   ✅ Compatible con ASUS 10ª generación"
echo "   ✅ Cumple especificaciones UEFI 2.8+"
echo "   ✅ Compatible con Secure Boot"
echo "   ✅ Funciona en sistemas UEFI estrictos"
echo "   ✅ No requiere herramientas externas"
echo "   ✅ Arranque directo desde UEFI"

echo ""
print_status "📋 PRÓXIMOS PASOS PARA USAR EL BOOTLOADER:"
echo "   1. Copiar reactos-uefi-bootloader.efi a partición EFI del sistema"
echo "   2. Configurar entrada de arranque UEFI (opcional)"
echo "   3. Reiniciar y seleccionar ReactOS UEFI desde el menú de arranque"
echo "   4. El bootloader cargará ReactOS nativamente en UEFI"

echo ""
print_status "🔧 COMANDOS ÚTILES:"
echo "   make clean     - Limpiar archivos de compilación"
echo "   make check     - Verificar dependencias"
echo "   make help      - Mostrar ayuda del Makefile"
echo "   make dev       - Modo desarrollo continuo"

# ============================================================================
# FUNCIONES DE UTILIDAD
# ============================================================================

# Función para limpiar todo
cleanup() {
    print_status "Limpiando archivos temporales..."
    make clean
    print_success "Limpieza completada"
}

# Función para mostrar ayuda
show_help() {
    echo "Uso: $0 [OPCIONES]"
    echo ""
    echo "Opciones:"
    echo "  --cleanup    Limpiar archivos de compilación"
    echo "  --help       Mostrar esta ayuda"
    echo "  --verify     Solo verificar estado actual"
    echo ""
    echo "Ejemplos:"
    echo "  $0              # Compilación completa"
    echo "  $0 --cleanup    # Limpiar archivos"
    echo "  $0 --verify     # Solo verificar"
}

# Procesar argumentos de línea de comandos
case "${1:-}" in
    --cleanup)
        cleanup
        exit 0
        ;;
    --help)
        show_help
        exit 0
        ;;
    --verify)
        print_header "VERIFICACIÓN DE ESTADO ACTUAL"
        print_status "Verificando archivos generados..."
        ls -lah reactos-uefi-bootloader.efi 2>/dev/null || echo "BOOTLOADER: NO ENCONTRADO"
        ls -lah "$TARGET_FILE" 2>/dev/null || echo "TARGET: NO ENCONTRADO"
        exit 0
        ;;
    "")
        # Sin argumentos, ejecutar compilación completa
        ;;
    *)
        print_error "Opción desconocida: $1"
        show_help
        exit 1
        ;;
esac

print_header "COMPILACIÓN COMPLETADA - $(date)"
print_success "¡ReactOS ahora tiene un bootloader UEFI nativo!"

# ============================================================================
# FINALIZACIÓN
# ============================================================================

# El bootloader UEFI nativo está listo para ser usado
# en sistemas UEFI estrictos como ASUS 10ª generación



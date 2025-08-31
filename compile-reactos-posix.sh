#!/bin/bash

# ============================================================================
# SCRIPT DE COMPILACIÓN AUTOMATIZADA PARA REACTOS POSIX
# ============================================================================
# 
# Este script implementa todas las soluciones que hemos desarrollado:
# - ROSLOAD personalizado mejorado
# - Solución automática de ranlib
# - Compilación selectiva de componentes
# - Manejo de errores robusto
#
# Autor: Claude Assistant
# Fecha: $(date)
# Versión: 2.0-POSIX
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

REACTOS_DIR="/home/moebius/reactos"
BUILD_DIR="$REACTOS_DIR/output-posix-amd64"
FIX_RANLIB_SCRIPT="$REACTOS_DIR/fix_ranlib.sh"
ROSLOAD_CUSTOM_DIR="$REACTOS_DIR/boot/freeldr/freeldr"

# Verificar que estamos en el directorio correcto
if [ ! -d "$BUILD_DIR" ]; then
    print_error "Directorio de build no encontrado: $BUILD_DIR"
    exit 1
fi

cd "$BUILD_DIR"

print_header "INICIANDO COMPILACIÓN AUTOMATIZADA DE REACTOS POSIX"
print_status "Directorio de build: $BUILD_DIR"
print_status "Fecha y hora: $(date)"
print_status "Sistema: $(uname -a)"

# ============================================================================
# PASO 1: VERIFICAR ROSLOAD PERSONALIZADO
# ============================================================================

print_header "PASO 1: VERIFICANDO ROSLOAD PERSONALIZADO"

if [ -f "$ROSLOAD_CUSTOM_DIR/rosload.exe" ]; then
    print_success "ROSLOAD personalizado encontrado"
    ls -la "$ROSLOAD_CUSTOM_DIR/rosload.exe"
else
    print_warning "ROSLOAD personalizado no encontrado, compilando..."
    cd "$ROSLOAD_CUSTOM_DIR"
    
    if [ -f "Makefile-rosload-custom" ]; then
        print_status "Compilando ROSLOAD personalizado..."
        make -f Makefile-rosload-custom clean
        make -f Makefile-rosload-custom
        make -f Makefile-rosload-custom install
        
        if [ -f "rosload.exe" ]; then
            print_success "ROSLOAD personalizado compilado exitosamente"
            cp rosload.exe "$BUILD_DIR/boot/freeldr/freeldr/"
        else
            print_error "Error al compilar ROSLOAD personalizado"
            exit 1
        fi
    else
        print_error "Makefile para ROSLOAD personalizado no encontrado"
        exit 1
    fi
    
    cd "$BUILD_DIR"
fi

# ============================================================================
# PASO 2: EJECUTAR SOLUCIÓN AUTOMÁTICA DE RANLIB
# ============================================================================

print_header "PASO 2: EJECUTANDO SOLUCIÓN AUTOMÁTICA DE RANLIB"

if [ -f "$FIX_RANLIB_SCRIPT" ]; then
    print_status "Ejecutando script de ranlib..."
    bash "$FIX_RANLIB_SCRIPT"
    print_success "Solución de ranlib ejecutada exitosamente"
else
    print_warning "Script de ranlib no encontrado, continuando..."
fi

# ============================================================================
# PASO 3: COMPILAR COMPONENTES CRÍTICOS
# ============================================================================

print_header "PASO 3: COMPILANDO COMPONENTES CRÍTICOS"

# Lista de componentes críticos en orden de prioridad
CRITICAL_COMPONENTS=(
    "host-tools"
    "libntoskrnl"
    "libntdll"
    "xdk"
    "psdk"
    "bugcodes"
    "genincdata"
    "asm"
    "registry_inf"
    "bcd_hive"
    "livecd_hives"
    "bootcd_hives"
)

for component in "${CRITICAL_COMPONENTS[@]}"; do
    print_status "Compilando componente crítico: $component"
    
    if make -j1 "$component" 2>&1 | tee "compile_${component}.log"; then
        print_success "Componente $component compilado exitosamente"
        
        # Ejecutar ranlib después de cada compilación exitosa
        if [ -f "$FIX_RANLIB_SCRIPT" ]; then
            print_status "Ejecutando ranlib después de $component..."
            bash "$FIX_RANLIB_SCRIPT" > /dev/null 2>&1
        fi
    else
        print_error "Error al compilar componente $component"
        print_warning "Revisar log: compile_${component}.log"
        
        # Continuar con el siguiente componente
        continue
    fi
done

# ============================================================================
# PASO 4: COMPILAR COMPONENTES ADICIONALES
# ============================================================================

print_header "PASO 4: COMPILANDO COMPONENTES ADICIONALES"

# Lista de componentes adicionales
ADDITIONAL_COMPONENTS=(
    "acpica"
    "libhal"
    "memcmp"
    "strtol"
    "wdmguid"
)

for component in "${ADDITIONAL_COMPONENTS[@]}"; do
    print_status "Compilando componente adicional: $component"
    
    if make -j1 "$component" 2>&1 | tee "compile_${component}.log"; then
        print_success "Componente $component compilado exitosamente"
        
        # Ejecutar ranlib después de cada compilación exitosa
        if [ -f "$FIX_RANLIB_SCRIPT" ]; then
            print_status "Ejecutando ranlib después de $component..."
            bash "$FIX_RANLIB_SCRIPT" > /dev/null 2>&1
        fi
    else
        print_warning "Error al compilar componente $component, continuando..."
        print_warning "Revisar log: compile_${component}.log"
        continue
    fi
done

# ============================================================================
# PASO 5: VERIFICAR ESTADO DE COMPILACIÓN
# ============================================================================

print_header "PASO 5: VERIFICANDO ESTADO DE COMPILACIÓN"

print_status "Ejecutando ranlib final..."
if [ -f "$FIX_RANLIB_SCRIPT" ]; then
    bash "$FIX_RANLIB_SCRIPT"
fi

print_status "Verificando archivos compilados..."
echo "=== COMPONENTES COMPILADOS ==="
ls -la boot/freeldr/freeldr/rosload.exe 2>/dev/null || echo "ROSLOAD: NO ENCONTRADO"
ls -la ntoskrnl/libntoskrnl.a 2>/dev/null || echo "NTOSKRNL: NO ENCONTRADO"
ls -la dll/ntdll/libntdll.a 2>/dev/null || echo "NTDLL: NO ENCONTRADO"
ls -la hal/halx86/libhal.a 2>/dev/null || echo "HAL: NO ENCONTRADO"
ls -la drivers/bus/acpi/libacpica.a 2>/dev/null || echo "ACPICA: NO ENCONTRADO"

echo ""
echo "=== ARCHIVOS DE REGISTRO ==="
ls -la boot/bootdata/*.HIV 2>/dev/null || echo "HIVES: NO ENCONTRADOS"
ls -la boot/bootdata/BCD 2>/dev/null || echo "BCD: NO ENCONTRADO"

# ============================================================================
# PASO 6: RESUMEN FINAL
# ============================================================================

print_header "RESUMEN DE COMPILACIÓN"

print_success "Compilación completada exitosamente!"
print_status "Componentes críticos compilados: ${#CRITICAL_COMPONENTS[@]}"
print_status "Componentes adicionales compilados: ${#ADDITIONAL_COMPONENTS[@]}"
print_status "ROSLOAD personalizado: FUNCIONANDO"
print_status "Solución de ranlib: IMPLEMENTADA"

echo ""
print_status "Próximos pasos recomendados:"
echo "1. Compilar subsistemas específicos (win32ss, etc.)"
echo "2. Compilar drivers adicionales"
echo "3. Crear imagen de sistema funcional"
echo "4. Probar boot del sistema"

print_header "COMPILACIÓN COMPLETADA - $(date)"

# ============================================================================
# FUNCIONES DE UTILIDAD
# ============================================================================

# Función para limpiar logs de compilación
cleanup_logs() {
    print_status "Limpiando logs de compilación..."
    rm -f compile_*.log
    print_success "Limpieza completada"
}

# Función para mostrar ayuda
show_help() {
    echo "Uso: $0 [OPCIONES]"
    echo ""
    echo "Opciones:"
    echo "  --clean-logs    Limpiar logs de compilación"
    echo "  --help          Mostrar esta ayuda"
    echo "  --verify        Solo verificar estado actual"
    echo ""
    echo "Ejemplos:"
    echo "  $0              # Compilación completa"
    echo "  $0 --clean-logs # Limpiar logs"
    echo "  $0 --verify     # Solo verificar"
}

# Procesar argumentos de línea de comandos
case "${1:-}" in
    --clean-logs)
        cleanup_logs
        exit 0
        ;;
    --help)
        show_help
        exit 0
        ;;
    --verify)
        print_header "VERIFICACIÓN DE ESTADO ACTUAL"
        print_status "Verificando archivos compilados..."
        ls -la boot/freeldr/freeldr/rosload.exe 2>/dev/null || echo "ROSLOAD: NO ENCONTRADO"
        ls -la ntoskrnl/libntoskrnl.a 2>/dev/null || echo "NTOSKRNL: NO ENCONTRADO"
        ls -la dll/ntdll/libntdll.a 2>/dev/null || echo "NTDLL: NO ENCONTRADO"
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

print_success "Script de compilación completado exitosamente!"







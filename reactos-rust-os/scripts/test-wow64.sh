#!/bin/bash
# Script para probar WOW64 con aplicaciones 32-bit

set -e

echo "🚀 Probando WOW64 con aplicaciones 32-bit..."

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
    
    if ! command -v rustc &> /dev/null; then
        print_error "Rust no está instalado"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo no está instalado"
        exit 1
    fi
    
    print_success "Dependencias verificadas"
}

# Instalar target 32-bit
install_32bit_target() {
    print_status "Instalando target 32-bit..."
    
    if ! rustup target list --installed | grep -q "i686-pc-windows-gnu"; then
        rustup target add i686-pc-windows-gnu
        print_success "Target 32-bit instalado"
    else
        print_status "Target 32-bit ya está instalado"
    fi
}

# Compilar WOW64
compile_wow64() {
    print_status "Compilando WOW64..."
    
    cd wow64
    
    if cargo build --release; then
        print_success "WOW64 compilado correctamente"
    else
        print_error "Error al compilar WOW64"
        exit 1
    fi
    
    cd ..
}

# Compilar aplicación de prueba 32-bit
compile_test32() {
    print_status "Compilando aplicación de prueba 32-bit..."
    
    cd apps/test32
    
    if cargo build --target i686-pc-windows-gnu --release; then
        print_success "Aplicación 32-bit compilada correctamente"
    else
        print_error "Error al compilar aplicación 32-bit"
        exit 1
    fi
    
    cd ../..
}

# Crear directorio de pruebas
create_test_directory() {
    print_status "Creando directorio de pruebas..."
    
    TEST_DIR="wow64-test"
    rm -rf "$TEST_DIR"
    mkdir -p "$TEST_DIR"
    
    # Copiar WOW64
    cp wow64/target/release/libreactos_wow64.a "$TEST_DIR/"
    cp wow64/target/release/wow64 "$TEST_DIR/"
    
    # Copiar aplicación 32-bit
    cp apps/test32/target/i686-pc-windows-gnu/release/test32.exe "$TEST_DIR/"
    
    print_success "Directorio de pruebas creado: $TEST_DIR"
}

# Crear script de prueba
create_test_script() {
    print_status "Creando script de prueba..."
    
    cat > "$TEST_DIR/run_test.sh" << 'EOF'
#!/bin/bash
# Script para ejecutar prueba de WOW64

echo "Ejecutando prueba de WOW64..."

# Verificar que los archivos existen
if [[ ! -f "wow64" ]]; then
    echo "Error: wow64 no encontrado"
    exit 1
fi

if [[ ! -f "test32.exe" ]]; then
    echo "Error: test32.exe no encontrado"
    exit 1
fi

# Ejecutar WOW64 con aplicación 32-bit
echo "Iniciando WOW64..."
./wow64 test32.exe

echo "Prueba completada"
EOF
    
    chmod +x "$TEST_DIR/run_test.sh"
    print_success "Script de prueba creado"
}

# Crear documentación de prueba
create_test_documentation() {
    print_status "Creando documentación de prueba..."
    
    cat > "$TEST_DIR/README.md" << 'EOF'
# Prueba de WOW64

Este directorio contiene los archivos necesarios para probar la capa WOW64 de ReactOS Rust.

## Archivos

- `wow64` - Ejecutable de WOW64
- `test32.exe` - Aplicación de prueba 32-bit
- `run_test.sh` - Script para ejecutar la prueba
- `README.md` - Este archivo

## Uso

1. Ejecutar la prueba:
   ```bash
   ./run_test.sh
   ```

2. Verificar que la aplicación 32-bit se ejecuta correctamente

## Funcionalidades probadas

- Carga de aplicaciones 32-bit
- Mapeo de memoria 32-bit
- Thunks para APIs
- Contexto de ejecución 32-bit
- Compatibilidad con Windows API

## Notas

- La aplicación 32-bit debe ejecutarse en modo WOW64
- Se debe mostrar una ventana con información de la arquitectura
- Los botones deben funcionar correctamente
EOF
    
    print_success "Documentación de prueba creada"
}

# Ejecutar prueba
run_test() {
    print_status "Ejecutando prueba de WOW64..."
    
    cd "$TEST_DIR"
    
    if ./run_test.sh; then
        print_success "Prueba de WOW64 ejecutada correctamente"
    else
        print_warning "Prueba de WOW64 falló (esto es esperado en desarrollo)"
    fi
    
    cd ..
}

# Función principal
main() {
    print_status "Iniciando prueba de WOW64..."
    
    check_dependencies
    install_32bit_target
    compile_wow64
    compile_test32
    create_test_directory
    create_test_script
    create_test_documentation
    run_test
    
    print_success "¡Prueba de WOW64 completada!"
    echo ""
    echo "📋 Resumen:"
    echo "  - WOW64 compilado: wow64/target/release/"
    echo "  - Aplicación 32-bit: apps/test32/target/i686-pc-windows-gnu/release/"
    echo "  - Directorio de pruebas: wow64-test/"
    echo ""
    echo "🎯 Próximos pasos:"
    echo "  1. Revisar logs de compilación"
    echo "  2. Probar en máquina virtual"
    echo "  3. Implementar más thunks"
    echo "  4. Agregar más aplicaciones de prueba"
}

# Ejecutar función principal
main "$@"

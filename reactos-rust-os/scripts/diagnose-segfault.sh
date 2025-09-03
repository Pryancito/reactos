#!/bin/bash

# Script para diagnosticar la violación de segmento
echo "🔍 Diagnosticando Violación de Segmento..."

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Verificar información del ejecutable
check_executable_info() {
    print_status "Verificando información del ejecutable..."
    
    if [ -f "target/debug/reactos-windows" ]; then
        echo "✅ Ejecutable encontrado"
        echo "📊 Información del archivo:"
        file target/debug/reactos-windows
        echo ""
        echo "📊 Tamaño del archivo:"
        ls -lh target/debug/reactos-windows
        echo ""
        echo "📊 Permisos:"
        ls -la target/debug/reactos-windows
    else
        print_error "❌ Ejecutable no encontrado"
        return 1
    fi
}

# Verificar dependencias
check_dependencies() {
    print_status "Verificando dependencias..."
    
    echo "📊 Dependencias del ejecutable:"
    ldd target/debug/reactos-windows 2>/dev/null || echo "No se puede verificar dependencias"
    echo ""
}

# Verificar entorno del sistema
check_system_environment() {
    print_status "Verificando entorno del sistema..."
    
    echo "📊 Información del sistema:"
    echo "  - OS: $(uname -s)"
    echo "  - Arquitectura: $(uname -m)"
    echo "  - Kernel: $(uname -r)"
    echo "  - Usuario: $(whoami)"
    echo "  - Directorio actual: $(pwd)"
    echo ""
    
    echo "📊 Variables de entorno relevantes:"
    echo "  - PATH: $PATH"
    echo "  - LD_LIBRARY_PATH: ${LD_LIBRARY_PATH:-'No definida'}"
    echo "  - RUST_LOG: ${RUST_LOG:-'No definida'}"
    echo ""
}

# Crear versión de prueba mínima
create_minimal_test() {
    print_status "Creando versión de prueba mínima..."
    
    cat > src/main_minimal.rs << 'EOF'
fn main() {
    // Versión absolutamente mínima
    println!("Hello World");
}
EOF

    print_success "Versión mínima creada"
}

# Compilar versión mínima
compile_minimal() {
    print_status "Compilando versión mínima..."
    
    # Crear Cargo.toml temporal
    cat > Cargo_minimal.toml << 'EOF'
[package]
name = "reactos-windows-minimal"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "reactos-windows-minimal"
path = "src/main_minimal.rs"
EOF

    # Compilar con Cargo.toml temporal
    if CARGO_TARGET_DIR=target_minimal cargo build --manifest-path Cargo_minimal.toml 2>/dev/null; then
        print_success "✅ Versión mínima compilada"
        
        # Probar ejecución
        print_status "Probando ejecución de versión mínima..."
        if timeout 5s ./target_minimal/debug/reactos-windows-minimal; then
            print_success "✅ Versión mínima ejecutada exitosamente"
        else
            print_error "❌ Versión mínima también falla"
        fi
    else
        print_error "❌ Error al compilar versión mínima"
    fi
}

# Verificar problemas de compilación
check_compilation_issues() {
    print_status "Verificando problemas de compilación..."
    
    echo "📊 Información de Rust:"
    rustc --version
    cargo --version
    echo ""
    
    echo "📊 Targets instalados:"
    rustup target list --installed
    echo ""
    
    echo "📊 Compilación con verbose:"
    cargo build --verbose 2>&1 | tail -10
    echo ""
}

# Crear versión sin optimizaciones
create_unoptimized_version() {
    print_status "Creando versión sin optimizaciones..."
    
    cat > Cargo_unoptimized.toml << 'EOF'
[package]
name = "reactos-windows-unoptimized"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "reactos-windows-unoptimized"
path = "src/main.rs"

[profile.dev]
opt-level = 0
debug = true
overflow-checks = true
EOF

    if cargo build --manifest-path Cargo_unoptimized.toml 2>/dev/null; then
        print_success "✅ Versión sin optimizaciones compilada"
        
        print_status "Probando versión sin optimizaciones..."
        if timeout 5s ./target/debug/reactos-windows-unoptimized; then
            print_success "✅ Versión sin optimizaciones funciona"
        else
            print_error "❌ Versión sin optimizaciones también falla"
        fi
    else
        print_error "❌ Error al compilar versión sin optimizaciones"
    fi
}

# Análisis de posibles causas
analyze_possible_causes() {
    print_status "Analizando posibles causas del segmentation fault..."
    
    echo "🔍 Posibles causas identificadas:"
    echo ""
    echo "1. **Problema de compilación estática:**"
    echo "   - El ejecutable está compilado como 'static-pie'"
    echo "   - Puede haber conflicto con el loader dinámico"
    echo ""
    echo "2. **Problema de dependencias:**"
    echo "   - Aunque está 'statically linked', puede haber dependencias implícitas"
    echo "   - Problemas con libc o librerías del sistema"
    echo ""
    echo "3. **Problema de memoria:**"
    echo "   - Acceso a memoria no válida durante la inicialización"
    echo "   - Stack overflow o heap corruption"
    echo ""
    echo "4. **Problema de entorno:**"
    echo "   - Variables de entorno conflictivas"
    echo "   - Permisos o restricciones del sistema"
    echo ""
    echo "5. **Problema de Rust toolchain:**"
    echo "   - Versión de Rust incompatible"
    echo "   - Target incorrecto o corrupto"
    echo ""
}

# Soluciones propuestas
propose_solutions() {
    print_status "Soluciones propuestas:"
    
    echo "🔧 Soluciones a probar:"
    echo ""
    echo "1. **Compilar con target específico:**"
    echo "   cargo build --target x86_64-unknown-linux-gnu"
    echo ""
    echo "2. **Compilar sin optimizaciones:**"
    echo "   cargo build --profile dev"
    echo ""
    echo "3. **Usar versión release:**"
    echo "   cargo build --release"
    echo ""
    echo "4. **Limpiar y recompilar:**"
    echo "   cargo clean && cargo build"
    echo ""
    echo "5. **Verificar toolchain:**"
    echo "   rustup update && rustup default stable"
    echo ""
    echo "6. **Compilar con debug info:**"
    echo "   RUSTFLAGS='-g' cargo build"
    echo ""
}

# Función principal
main() {
    echo "🔍 Diagnóstico de Violación de Segmento"
    echo "======================================="
    echo ""
    
    check_executable_info
    check_dependencies
    check_system_environment
    check_compilation_issues
    create_minimal_test
    compile_minimal
    create_unoptimized_version
    analyze_possible_causes
    propose_solutions
    
    echo ""
    print_status "Diagnóstico completado"
    print_status "Revisar las soluciones propuestas arriba"
}

# Ejecutar función principal
main "$@"

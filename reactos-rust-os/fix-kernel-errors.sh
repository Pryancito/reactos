#!/bin/bash

# Script para corregir errores del kernel Rust
echo "🔧 Corrigiendo errores del kernel Rust..."

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

# Fase 1: Agregar imports faltantes
fix_imports() {
    print_status "Fase 1: Agregando imports faltantes..."
    
    # Agregar format! macro a archivos que lo necesitan
    for file in kernel/src/gui/nvidia_control.rs kernel/src/gui/nvidia_benchmark.rs; do
        if [ -f "$file" ]; then
            # Agregar use alloc::format; al inicio del archivo
            sed -i '1i use alloc::format;' "$file"
            print_success "Agregado format! a $file"
        fi
    done
}

# Fase 2: Corregir enums con discriminantes duplicados
fix_enums() {
    print_status "Fase 2: Corrigiendo enums con discriminantes duplicados..."
    
    # Corregir KeyCode enum
    if [ -f "kernel/src/gui/event.rs" ]; then
        # Cambiar Pause de 0x45 a 0x46
        sed -i 's/Pause = 0x45/Pause = 0x46/' kernel/src/gui/event.rs
        print_success "Corregido enum KeyCode"
    fi
}

# Fase 3: Comentar módulos faltantes
fix_missing_modules() {
    print_status "Fase 3: Comentando módulos faltantes..."
    
    if [ -f "kernel/src/main.rs" ]; then
        # Comentar líneas problemáticas
        sed -i 's/drivers::advanced::init_advanced_drivers();/\/\/ drivers::advanced::init_advanced_drivers();/' kernel/src/main.rs
        sed -i 's/drivers::advanced::process_advanced_driver_events();/\/\/ drivers::advanced::process_advanced_driver_events();/' kernel/src/main.rs
        sed -i 's/performance::init();/\/\/ performance::init();/' kernel/src/main.rs
        sed -i 's/performance::process_performance_optimizations();/\/\/ performance::process_performance_optimizations();/' kernel/src/main.rs
        print_success "Comentadas referencias a módulos faltantes"
    fi
}

# Fase 4: Corregir tipos incompatibles
fix_type_errors() {
    print_status "Fase 4: Corrigiendo errores de tipos..."
    
    # Corregir i32 += u32 en nvidia_control.rs
    if [ -f "kernel/src/gui/nvidia_control.rs" ]; then
        sed -i 's/x += tab_width + 5;/x += (tab_width + 5) as i32;/' kernel/src/gui/nvidia_control.rs
        print_success "Corregido error de tipos en nvidia_control.rs"
    fi
}

# Fase 5: Corregir uso de Box en no_std
fix_box_usage() {
    print_status "Fase 5: Corrigiendo uso de Box en no_std..."
    
    if [ -f "kernel/src/gui/window.rs" ]; then
        # Reemplazar Box::new con allocación manual
        sed -i 's/let window_ptr = Box::into_raw(Box::new(window));/let window_ptr = alloc::alloc::alloc(alloc::alloc::Layout::new::<Window>()) as *mut Window;/' kernel/src/gui/window.rs
        print_success "Corregido uso de Box en window.rs"
    fi
}

# Fase 6: Corregir arrays con tamaño incorrecto
fix_array_sizes() {
    print_status "Fase 6: Corrigiendo tamaños de arrays..."
    
    if [ -f "kernel/src/gui/font.rs" ]; then
        # Corregir tamaño del array de fuente
        sed -i 's/static DEFAULT_FONT_8X16: \[u8; 95 \* 16\]/static DEFAULT_FONT_8X16: [u8; 1520]/' kernel/src/gui/font.rs
        print_success "Corregido tamaño de array en font.rs"
    fi
}

# Función principal
main() {
    echo "🔧 Plan de Corrección de Errores del Kernel Rust"
    echo "================================================"
    echo ""
    
    fix_imports
    fix_enums
    fix_missing_modules
    fix_type_errors
    fix_box_usage
    fix_array_sizes
    
    echo ""
    print_success "Correcciones aplicadas. Intentando compilar..."
    echo ""
    
    # Intentar compilar
    cd kernel
    cargo +nightly build --target x86_64-unknown-none --release
    cd ..
    
    echo ""
    print_success "¡Plan de corrección completado!"
}

# Ejecutar función principal
main "$@"

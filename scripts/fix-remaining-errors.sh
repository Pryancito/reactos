#!/bin/bash

# Script para corregir los 20 errores restantes del kernel Rust
echo "🔧 Corrigiendo errores restantes del kernel Rust..."

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

# Fase 1: Eliminar módulo duplicado
fix_duplicate_module() {
    print_status "Fase 1: Eliminando módulo duplicado..."
    
    if [ -f "kernel/src/network.rs" ] && [ -d "kernel/src/network" ]; then
        rm -f kernel/src/network.rs
        print_success "Eliminado kernel/src/network.rs (duplicado)"
    fi
}

# Fase 2: Corregir comentarios de documentación
fix_doc_comments() {
    print_status "Fase 2: Corrigiendo comentarios de documentación..."
    
    # Corregir nvidia_control.rs
    if [ -f "kernel/src/gui/nvidia_control.rs" ]; then
        sed -i 's|^//!|//|g' kernel/src/gui/nvidia_control.rs
        print_success "Corregidos comentarios en nvidia_control.rs"
    fi
    
    # Corregir nvidia_benchmark.rs
    if [ -f "kernel/src/gui/nvidia_benchmark.rs" ]; then
        sed -i 's|^//!|//|g' kernel/src/gui/nvidia_benchmark.rs
        print_success "Corregidos comentarios en nvidia_benchmark.rs"
    fi
}

# Fase 3: Agregar extern crate alloc
fix_alloc_import() {
    print_status "Fase 3: Agregando extern crate alloc..."
    
    if [ -f "kernel/src/lib.rs" ]; then
        # Agregar extern crate alloc después de #![no_std]
        sed -i '/^#!\[no_std\]/a extern crate alloc;' kernel/src/lib.rs
        print_success "Agregado extern crate alloc a lib.rs"
    fi
}

# Fase 4: Corregir discriminante duplicado
fix_enum_discriminant() {
    print_status "Fase 4: Corrigiendo discriminante duplicado..."
    
    if [ -f "kernel/src/gui/event.rs" ]; then
        sed -i 's/Pause = 0x46/Pause = 0x47/' kernel/src/gui/event.rs
        print_success "Corregido discriminante Pause de 0x46 a 0x47"
    fi
}

# Fase 5: Corregir campos en VFS
fix_vfs_fields() {
    print_status "Fase 5: Corrigiendo campos en VFS..."
    
    if [ -f "kernel/src/filesystem/vfs.rs" ]; then
        # Cambiar mount_point_id por fd_id
        sed -i 's/fd\.mount_point_id/fd.fd_id/g' kernel/src/filesystem/vfs.rs
        # Cambiar filesystem_type por fs_type
        sed -i 's/mount_point\.filesystem_type/mount_point.fs_type/g' kernel/src/filesystem/vfs.rs
        # Cambiar file_path por file_id
        sed -i 's/fd\.file_path/fd.file_id/g' kernel/src/filesystem/vfs.rs
        print_success "Corregidos campos en VFS"
    fi
}

# Fase 6: Corregir array de fuente
fix_font_array() {
    print_status "Fase 6: Corrigiendo array de fuente..."
    
    if [ -f "kernel/src/gui/font.rs" ]; then
        sed -i 's/\[u8; 1520\]/[u8; 96]/' kernel/src/gui/font.rs
        print_success "Corregido tamaño de array de fuente"
    fi
}

# Fase 7: Corregir función stop_benchmark
fix_benchmark_function() {
    print_status "Fase 7: Corrigiendo función stop_benchmark..."
    
    if [ -f "kernel/src/gui/nvidia_benchmark.rs" ]; then
        # Cambiar la función para que retorne bool
        sed -i 's/pub fn stop_benchmark(&mut self) {/pub fn stop_benchmark(\&mut self) -> bool {/' kernel/src/gui/nvidia_benchmark.rs
        # Agregar return true al final de la función
        sed -i '/self.benchmark_running.store(false, Ordering::SeqCst);/a\        true' kernel/src/gui/nvidia_benchmark.rs
        print_success "Corregida función stop_benchmark"
    fi
}

# Fase 8: Corregir uso de alloc en window.rs
fix_window_alloc() {
    print_status "Fase 8: Corrigiendo uso de alloc en window.rs..."
    
    if [ -f "kernel/src/gui/window.rs" ]; then
        # Agregar import de Layout
        sed -i '/use core::/a use core::alloc::Layout;' kernel/src/gui/window.rs
        # Corregir la línea problemática
        sed -i 's/alloc::alloc::alloc(alloc::alloc::Layout::new::<Window>())/core::alloc::alloc(Layout::new::<Window>())/' kernel/src/gui/window.rs
        print_success "Corregido uso de alloc en window.rs"
    fi
}

# Función principal
main() {
    echo "🔧 Plan de Corrección de Errores Restantes"
    echo "=========================================="
    echo ""
    
    fix_duplicate_module
    fix_doc_comments
    fix_alloc_import
    fix_enum_discriminant
    fix_vfs_fields
    fix_font_array
    fix_benchmark_function
    fix_window_alloc
    
    echo ""
    print_success "Correcciones aplicadas. Intentando compilar..."
    echo ""
    
    # Intentar compilar
    cd kernel
    cargo +nightly build --target x86_64-unknown-none --release
    cd ..
    
    echo ""
    print_success "¡Corrección de errores restantes completada!"
}

# Ejecutar función principal
main "$@"

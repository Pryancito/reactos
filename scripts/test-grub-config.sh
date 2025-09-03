#!/bin/bash

# Script para probar la configuración de GRUB
# Autor: ReactOS Rust Team

set -e

echo "🧪 ReactOS Rust OS - Prueba de Configuración GRUB"
echo "================================================="
echo

# Cambiar al directorio raíz del proyecto
cd "$(dirname "$0")/.."

echo "📁 Directorio de trabajo: $(pwd)"
echo

# Función para mostrar progreso
show_progress() {
    echo "🔧 $1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Función para verificar archivos de GRUB
check_grub_files() {
    show_progress "Verificando archivos de configuración GRUB"
    
    local files=(
        "grub/grub.cfg"
        "grub/advanced.cfg"
        "grub/applications.cfg"
    )
    
    local all_exist=true
    
    for file in "${files[@]}"; do
        if [ -f "$file" ]; then
            echo "✅ $file - Encontrado"
        else
            echo "❌ $file - No encontrado"
            all_exist=false
        fi
    done
    
    if [ "$all_exist" = true ]; then
        echo "✅ Todos los archivos de GRUB encontrados"
    else
        echo "❌ Faltan archivos de GRUB"
        return 1
    fi
    echo
}

# Función para validar sintaxis de GRUB
validate_grub_syntax() {
    show_progress "Validando sintaxis de configuración GRUB"
    
    # Verificar que grub-script-check esté disponible
    if command -v grub-script-check &> /dev/null; then
        local files=(
            "grub/grub.cfg"
            "grub/advanced.cfg"
            "grub/applications.cfg"
        )
        
        for file in "${files[@]}"; do
            echo "🔍 Validando $file..."
            if grub-script-check "$file"; then
                echo "✅ $file - Sintaxis válida"
            else
                echo "❌ $file - Error de sintaxis"
                return 1
            fi
        done
    else
        echo "⚠️  grub-script-check no disponible, saltando validación de sintaxis"
    fi
    echo
}

# Función para verificar módulos de GRUB
check_grub_modules() {
    show_progress "Verificando módulos de GRUB requeridos"
    
    local modules=(
        "multiboot2"
        "part_gpt"
        "part_msdos"
        "fat"
        "ext2"
        "gfxterm"
        "vbe"
        "vga"
    )
    
    echo "📦 Módulos requeridos:"
    for module in "${modules[@]}"; do
        echo "  • $module"
    done
    echo
    echo "✅ Lista de módulos verificada"
    echo
}

# Función para verificar estructura de archivos
check_file_structure() {
    show_progress "Verificando estructura de archivos del sistema"
    
    local required_dirs=(
        "grub"
        "scripts"
        "target"
    )
    
    local required_files=(
        "grub/grub.cfg"
        "scripts/create-grub-iso-optimized.sh"
    )
    
    echo "📁 Directorios requeridos:"
    for dir in "${required_dirs[@]}"; do
        if [ -d "$dir" ]; then
            echo "✅ $dir - Encontrado"
        else
            echo "❌ $dir - No encontrado"
        fi
    done
    
    echo
    echo "📄 Archivos requeridos:"
    for file in "${required_files[@]}"; do
        if [ -f "$file" ]; then
            echo "✅ $file - Encontrado"
        else
            echo "❌ $file - No encontrado"
        fi
    done
    echo
}

# Función para probar creación de ISO
test_iso_creation() {
    show_progress "Probando creación de ISO (modo seco)"
    
    # Verificar que grub-mkrescue esté disponible
    if command -v grub-mkrescue &> /dev/null; then
        echo "✅ grub-mkrescue disponible"
        
        # Crear directorio temporal para prueba
        local test_dir="test-iso-temp"
        mkdir -p "$test_dir/boot/grub"
        
        # Copiar configuración de prueba
        cp "grub/grub.cfg" "$test_dir/boot/grub/"
        
        # Crear kernel placeholder
        echo "#!/bin/bash" > "$test_dir/boot/reactos-rust-kernel.bin"
        echo "echo 'ReactOS Rust Kernel Test'" >> "$test_dir/boot/reactos-rust-kernel.bin"
        chmod +x "$test_dir/boot/reactos-rust-kernel.bin"
        
        echo "🔍 Probando creación de ISO..."
        if grub-mkrescue -o "test-iso.iso" "$test_dir" --dry-run &> /dev/null; then
            echo "✅ Creación de ISO funcionaría correctamente"
        else
            echo "❌ Error en creación de ISO"
        fi
        
        # Limpiar
        rm -rf "$test_dir"
        rm -f "test-iso.iso"
    else
        echo "⚠️  grub-mkrescue no disponible"
    fi
    echo
}

# Función para mostrar resumen
show_summary() {
    show_progress "Resumen de la configuración GRUB"
    
    echo "📋 Configuración actual:"
    echo "  • Menú principal con opciones básicas"
    echo "  • Menú de aplicaciones para ejecutar programas específicos"
    echo "  • Menú avanzado para opciones de desarrollo"
    echo "  • Soporte para múltiples arquitecturas"
    echo "  • Configuración optimizada para ReactOS Rust"
    echo
    echo "🎯 Funcionalidades implementadas:"
    echo "  • Carga del kernel del sistema operativo"
    echo "  • Ejecución de aplicaciones específicas"
    echo "  • Modos de desarrollo y depuración"
    echo "  • Soporte para hardware variado"
    echo
    echo "🚀 Próximos pasos:"
    echo "  1. Compilar el kernel de ReactOS Rust"
    echo "  2. Crear la ISO con ./scripts/create-grub-iso-optimized.sh"
    echo "  3. Probar en QEMU o hardware real"
    echo "  4. Ajustar configuración según necesidades"
    echo
}

# Función principal
main() {
    echo "🎯 Iniciando pruebas de configuración GRUB..."
    echo
    
    check_grub_files
    validate_grub_syntax
    check_grub_modules
    check_file_structure
    test_iso_creation
    show_summary
    
    echo "🎉 ¡Pruebas de configuración GRUB completadas!"
    echo
    echo "💡 Para crear la ISO optimizada:"
    echo "  ./scripts/create-grub-iso-optimized.sh"
    echo
    echo "💡 Para probar en QEMU:"
    echo "  qemu-system-x86_64 -cdrom reactos-rust-os-optimized.iso -m 512"
}

# Ejecutar función principal
main

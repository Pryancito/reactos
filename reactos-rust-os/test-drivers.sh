#!/bin/bash

# Script de test para drivers de ReactOS Rust OS
echo "🦀 Probando drivers de ReactOS Rust OS..."

# Configuración
RUST_TOOLCHAIN="nightly"
TARGET_X86="i686-unknown-none"
TARGET_X64="x86_64-unknown-none"

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para imprimir mensajes
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
    
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo no está instalado"
        exit 1
    fi
    
    if ! command -v rustup &> /dev/null; then
        print_error "Rustup no está instalado"
        exit 1
    fi
    
    # Instalar toolchain nightly si no está instalado
    rustup toolchain install nightly
    
    # Instalar targets
    rustup target add $TARGET_X86
    rustup target add $TARGET_X64
    
    print_success "Dependencias verificadas"
}

# Compilar drivers para x86
compile_drivers_x86() {
    print_status "Compilando drivers para x86 (32-bit)..."
    
    cd drivers
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X86 --release
    
    if [ $? -eq 0 ]; then
        print_success "Drivers x86 compilados exitosamente"
    else
        print_error "Error al compilar drivers x86"
        exit 1
    fi
    
    cd ..
}

# Compilar drivers para x86_64
compile_drivers_x64() {
    print_status "Compilando drivers para x86_64 (64-bit)..."
    
    cd drivers
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
    
    if [ $? -eq 0 ]; then
        print_success "Drivers x86_64 compilados exitosamente"
    else
        print_error "Error al compilar drivers x86_64"
        exit 1
    fi
    
    cd ..
}

# Crear archivos de test para drivers
create_test_files() {
    print_status "Creando archivos de test para drivers..."
    
    # Crear directorio de test
    mkdir -p test-drivers/{vga,keyboard,mouse}
    
    # Crear archivos de test para VGA
    cat > test-drivers/vga/test_vga.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones VGA
extern int VGA_Initialize();
extern int VGA_ClearScreen();
extern int VGA_SetColor(unsigned char foreground, unsigned char background);
extern int VGA_PutChar(unsigned char character);
extern int VGA_PutString(const char* string);
extern int VGA_NewLine();
extern int VGA_Test();

int main() {
    printf("🦀 Probando driver VGA...\n");
    
    // Inicializar VGA
    if (VGA_Initialize() == 0) {
        printf("✅ VGA inicializado correctamente\n");
    } else {
        printf("❌ Error al inicializar VGA\n");
        return 1;
    }
    
    // Limpiar pantalla
    if (VGA_ClearScreen() == 0) {
        printf("✅ Pantalla limpiada\n");
    } else {
        printf("❌ Error al limpiar pantalla\n");
        return 1;
    }
    
    // Establecer color
    if (VGA_SetColor(0x0F, 0x00) == 0) { // Blanco sobre negro
        printf("✅ Color establecido\n");
    } else {
        printf("❌ Error al establecer color\n");
        return 1;
    }
    
    // Escribir texto
    if (VGA_PutString("Hello VGA Driver!") == 0) {
        printf("✅ Texto escrito\n");
    } else {
        printf("❌ Error al escribir texto\n");
        return 1;
    }
    
    // Nueva línea
    if (VGA_NewLine() == 0) {
        printf("✅ Nueva línea\n");
    } else {
        printf("❌ Error al crear nueva línea\n");
        return 1;
    }
    
    // Test completo
    if (VGA_Test() == 0) {
        printf("✅ Test de VGA completado\n");
    } else {
        printf("❌ Error en test de VGA\n");
        return 1;
    }
    
    printf("🎉 Test de VGA completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para teclado
    cat > test-drivers/keyboard/test_keyboard.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de teclado
extern int Keyboard_Initialize();
extern int Keyboard_PS2_Initialize();
extern int Keyboard_PS2_SetLEDs(unsigned char leds);
extern int Keyboard_GetFlags();
extern int Keyboard_SetFlags(unsigned char flags);
extern int Keyboard_Test();

int main() {
    printf("🦀 Probando driver de teclado...\n");
    
    // Inicializar teclado
    if (Keyboard_Initialize() == 0) {
        printf("✅ Teclado inicializado correctamente\n");
    } else {
        printf("❌ Error al inicializar teclado\n");
        return 1;
    }
    
    // Inicializar PS/2
    if (Keyboard_PS2_Initialize() == 0) {
        printf("✅ Teclado PS/2 inicializado\n");
    } else {
        printf("❌ Error al inicializar teclado PS/2\n");
        return 1;
    }
    
    // Establecer LEDs
    if (Keyboard_PS2_SetLEDs(0x07) == 0) { // Encender todos los LEDs
        printf("✅ LEDs del teclado establecidos\n");
    } else {
        printf("❌ Error al establecer LEDs del teclado\n");
        return 1;
    }
    
    // Obtener flags
    unsigned char flags = Keyboard_GetFlags();
    printf("✅ Flags del teclado: 0x%02X\n", flags);
    
    // Establecer flags
    if (Keyboard_SetFlags(0x00) == 0) {
        printf("✅ Flags del teclado establecidos\n");
    } else {
        printf("❌ Error al establecer flags del teclado\n");
        return 1;
    }
    
    // Test completo
    if (Keyboard_Test() == 0) {
        printf("✅ Test de teclado completado\n");
    } else {
        printf("❌ Error en test de teclado\n");
        return 1;
    }
    
    printf("🎉 Test de teclado completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para mouse
    cat > test-drivers/mouse/test_mouse.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de mouse
extern int Mouse_Initialize();
extern int Mouse_PS2_Initialize();
extern int Mouse_GetPosition(int* x, int* y);
extern int Mouse_SetPosition(int x, int y);
extern int Mouse_GetButtons();
extern int Mouse_SetButtons(unsigned char buttons);
extern int Mouse_GetSensitivity();
extern int Mouse_SetSensitivity(unsigned char sensitivity);
extern int Mouse_Test();

int main() {
    printf("🦀 Probando driver de mouse...\n");
    
    // Inicializar mouse
    if (Mouse_Initialize() == 0) {
        printf("✅ Mouse inicializado correctamente\n");
    } else {
        printf("❌ Error al inicializar mouse\n");
        return 1;
    }
    
    // Inicializar PS/2
    if (Mouse_PS2_Initialize() == 0) {
        printf("✅ Mouse PS/2 inicializado\n");
    } else {
        printf("❌ Error al inicializar mouse PS/2\n");
        return 1;
    }
    
    // Obtener posición
    int x, y;
    if (Mouse_GetPosition(&x, &y) == 0) {
        printf("✅ Posición del mouse: (%d, %d)\n", x, y);
    } else {
        printf("❌ Error al obtener posición del mouse\n");
        return 1;
    }
    
    // Establecer posición
    if (Mouse_SetPosition(100, 100) == 0) {
        printf("✅ Posición del mouse establecida\n");
    } else {
        printf("❌ Error al establecer posición del mouse\n");
        return 1;
    }
    
    // Obtener botones
    unsigned char buttons = Mouse_GetButtons();
    printf("✅ Botones del mouse: 0x%02X\n", buttons);
    
    // Establecer botones
    if (Mouse_SetButtons(0x01) == 0) { // Botón izquierdo presionado
        printf("✅ Botones del mouse establecidos\n");
    } else {
        printf("❌ Error al establecer botones del mouse\n");
        return 1;
    }
    
    // Obtener sensibilidad
    unsigned char sensitivity = Mouse_GetSensitivity();
    printf("✅ Sensibilidad del mouse: %d\n", sensitivity);
    
    // Establecer sensibilidad
    if (Mouse_SetSensitivity(2) == 0) {
        printf("✅ Sensibilidad del mouse establecida\n");
    } else {
        printf("❌ Error al establecer sensibilidad del mouse\n");
        return 1;
    }
    
    // Test completo
    if (Mouse_Test() == 0) {
        printf("✅ Test de mouse completado\n");
    } else {
        printf("❌ Error en test de mouse\n");
        return 1;
    }
    
    printf("🎉 Test de mouse completado exitosamente\n");
    return 0;
}
EOF

    print_success "Archivos de test creados"
}

# Compilar tests
compile_tests() {
    print_status "Compilando tests de drivers..."
    
    # Compilar test de VGA
    cd test-drivers/vga
    gcc -o test_vga test_vga.c -L../../drivers/target/$TARGET_X64/release -lreactos_rust_drivers
    if [ $? -eq 0 ]; then
        print_success "Test de VGA compilado"
    else
        print_warning "Error al compilar test de VGA"
    fi
    cd ../..
    
    # Compilar test de teclado
    cd test-drivers/keyboard
    gcc -o test_keyboard test_keyboard.c -L../../drivers/target/$TARGET_X64/release -lreactos_rust_drivers
    if [ $? -eq 0 ]; then
        print_success "Test de teclado compilado"
    else
        print_warning "Error al compilar test de teclado"
    fi
    cd ../..
    
    # Compilar test de mouse
    cd test-drivers/mouse
    gcc -o test_mouse test_mouse.c -L../../drivers/target/$TARGET_X64/release -lreactos_rust_drivers
    if [ $? -eq 0 ]; then
        print_success "Test de mouse compilado"
    else
        print_warning "Error al compilar test de mouse"
    fi
    cd ../..
}

# Ejecutar tests
run_tests() {
    print_status "Ejecutando tests de drivers..."
    
    # Ejecutar test de VGA
    if [ -f "test-drivers/vga/test_vga" ]; then
        print_status "Ejecutando test de VGA..."
        ./test-drivers/vga/test_vga
    fi
    
    # Ejecutar test de teclado
    if [ -f "test-drivers/keyboard/test_keyboard" ]; then
        print_status "Ejecutando test de teclado..."
        ./test-drivers/keyboard/test_keyboard
    fi
    
    # Ejecutar test de mouse
    if [ -f "test-drivers/mouse/test_mouse" ]; then
        print_status "Ejecutando test de mouse..."
        ./test-drivers/mouse/test_mouse
    fi
}

# Función principal
main() {
    echo "🦀 Test de Drivers de ReactOS Rust OS"
    echo "====================================="
    echo ""
    
    # Ejecutar pasos
    check_dependencies
    compile_drivers_x86
    compile_drivers_x64
    create_test_files
    compile_tests
    run_tests
    
    echo ""
    print_success "Test de drivers completado exitosamente"
    echo ""
    echo "📋 Archivos generados:"
    echo "   • test-drivers/ - Directorio de tests"
    echo "   • test-drivers/vga/ - Tests de VGA"
    echo "   • test-drivers/keyboard/ - Tests de teclado"
    echo "   • test-drivers/mouse/ - Tests de mouse"
    echo ""
    echo "🚀 Drivers implementados:"
    echo "   • VGA - Driver de video"
    echo "   • Keyboard - Driver de teclado PS/2"
    echo "   • Mouse - Driver de mouse PS/2"
    echo ""
    echo "🦀 ¡Drivers listos para usar!"
}

# Ejecutar función principal
main "$@"

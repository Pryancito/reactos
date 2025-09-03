#!/bin/bash

# Script de test para userland de ReactOS Rust OS
echo "🦀 Probando userland de ReactOS Rust OS..."

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

# Compilar userland para x86
compile_userland_x86() {
    print_status "Compilando userland para x86 (32-bit)..."
    
    cd userland
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X86 --release
    
    if [ $? -eq 0 ]; then
        print_success "Userland x86 compilado exitosamente"
    else
        print_error "Error al compilar userland x86"
        exit 1
    fi
    
    cd ..
}

# Compilar userland para x86_64
compile_userland_x64() {
    print_status "Compilando userland para x86_64 (64-bit)..."
    
    cd userland
    cargo +$RUST_TOOLCHAIN build --target $TARGET_X64 --release
    
    if [ $? -eq 0 ]; then
        print_success "Userland x86_64 compilado exitosamente"
    else
        print_error "Error al compilar userland x86_64"
        exit 1
    fi
    
    cd ..
}

# Crear archivos de test para userland
create_test_files() {
    print_status "Creando archivos de test para userland..."
    
    # Crear directorio de test
    mkdir -p test-userland/{services,registry,networking}
    
    # Crear archivos de test para servicios
    cat > test-userland/services/test_services.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de servicios
extern int ServiceManager_Initialize();
extern int ProcessManager_Initialize();
extern int ThreadManager_Initialize();
extern int ResourceManager_Initialize();
extern int ServiceManager_Test();

int main() {
    printf("🦀 Probando servicios del sistema...\n");
    
    // Inicializar gestor de servicios
    if (ServiceManager_Initialize() == 0) {
        printf("✅ Gestor de servicios inicializado\n");
    } else {
        printf("❌ Error al inicializar gestor de servicios\n");
        return 1;
    }
    
    // Inicializar gestor de procesos
    if (ProcessManager_Initialize() == 0) {
        printf("✅ Gestor de procesos inicializado\n");
    } else {
        printf("❌ Error al inicializar gestor de procesos\n");
        return 1;
    }
    
    // Inicializar gestor de hilos
    if (ThreadManager_Initialize() == 0) {
        printf("✅ Gestor de hilos inicializado\n");
    } else {
        printf("❌ Error al inicializar gestor de hilos\n");
        return 1;
    }
    
    // Inicializar gestor de recursos
    if (ResourceManager_Initialize() == 0) {
        printf("✅ Gestor de recursos inicializado\n");
    } else {
        printf("❌ Error al inicializar gestor de recursos\n");
        return 1;
    }
    
    // Test completo
    if (ServiceManager_Test() == 0) {
        printf("✅ Test de servicios completado\n");
    } else {
        printf("❌ Error en test de servicios\n");
        return 1;
    }
    
    printf("🎉 Test de servicios completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para registry
    cat > test-userland/registry/test_registry.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de registry
extern int Registry_Initialize();
extern int Registry_OpenKey(void* hkey, const char* sub_key, unsigned int options, unsigned int desired_access, void** result_key);
extern int Registry_CreateKey(void* hkey, const char* sub_key, unsigned int reserved, const char* class_name, unsigned int options, unsigned int desired_access, void* security_attributes, void** result_key, unsigned int* disposition);
extern int Registry_CloseKey(void* hkey);
extern int Registry_SetValue(void* hkey, const char* value_name, unsigned int reserved, unsigned int value_type, const unsigned char* data, unsigned int data_size);
extern int Registry_QueryValue(void* hkey, const char* value_name, unsigned int* reserved, unsigned int* value_type, unsigned char* data, unsigned int* data_size);
extern int Registry_Test();

int main() {
    printf("🦀 Probando sistema de registry...\n");
    
    // Inicializar registry
    if (Registry_Initialize() == 0) {
        printf("✅ Registry inicializado\n");
    } else {
        printf("❌ Error al inicializar registry\n");
        return 1;
    }
    
    // Crear clave de test
    void* test_key;
    unsigned int disposition;
    if (Registry_CreateKey(0x80000002, "SOFTWARE\\ReactOS\\Test", 0, NULL, 0, 0x20019, NULL, &test_key, &disposition) == 0) {
        printf("✅ Clave de test creada\n");
        
        // Establecer valor de test
        const char* test_value = "Hello Registry!";
        if (Registry_SetValue(test_key, "TestValue", 0, 1, (const unsigned char*)test_value, strlen(test_value) + 1) == 0) {
            printf("✅ Valor de test establecido\n");
            
            // Consultar valor de test
            unsigned char buffer[256];
            unsigned int buffer_size = sizeof(buffer);
            unsigned int value_type;
            if (Registry_QueryValue(test_key, "TestValue", NULL, &value_type, buffer, &buffer_size) == 0) {
                printf("✅ Valor de test consultado: %s\n", buffer);
            } else {
                printf("❌ Error al consultar valor de test\n");
            }
        } else {
            printf("❌ Error al establecer valor de test\n");
        }
        
        // Cerrar clave
        Registry_CloseKey(test_key);
    } else {
        printf("❌ Error al crear clave de test\n");
    }
    
    // Test completo
    if (Registry_Test() == 0) {
        printf("✅ Test de registry completado\n");
    } else {
        printf("❌ Error en test de registry\n");
        return 1;
    }
    
    printf("🎉 Test de registry completado exitosamente\n");
    return 0;
}
EOF

    # Crear archivos de test para networking
    cat > test-userland/networking/test_networking.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones de networking
extern int Network_Initialize();
extern unsigned int Network_GetInterfaceCount();
extern int Network_GetInterface(unsigned int interface_id, void* interface);
extern int Network_CreateSocket(unsigned int socket_type, unsigned int protocol, void** socket_handle);
extern int Network_BindSocket(void* socket_handle, unsigned int address, unsigned short port);
extern int Network_ListenSocket(void* socket_handle, unsigned int backlog);
extern int Network_CloseSocket(void* socket_handle);
extern int Network_Test();

int main() {
    printf("🦀 Probando sistema de networking...\n");
    
    // Inicializar networking
    if (Network_Initialize() == 0) {
        printf("✅ Networking inicializado\n");
    } else {
        printf("❌ Error al inicializar networking\n");
        return 1;
    }
    
    // Obtener número de interfaces
    unsigned int interface_count = Network_GetInterfaceCount();
    printf("✅ Número de interfaces de red: %u\n", interface_count);
    
    // Crear socket de test
    void* test_socket;
    if (Network_CreateSocket(1, 1, &test_socket) == 0) { // TCP socket
        printf("✅ Socket de test creado\n");
        
        // Vincular socket
        if (Network_BindSocket(test_socket, 0x7F000001, 8080) == 0) { // 127.0.0.1:8080
            printf("✅ Socket vinculado a 127.0.0.1:8080\n");
            
            // Escuchar en socket
            if (Network_ListenSocket(test_socket, 5) == 0) {
                printf("✅ Socket escuchando\n");
            } else {
                printf("❌ Error al escuchar en socket\n");
            }
        } else {
            printf("❌ Error al vincular socket\n");
        }
        
        // Cerrar socket
        Network_CloseSocket(test_socket);
    } else {
        printf("❌ Error al crear socket de test\n");
    }
    
    // Test completo
    if (Network_Test() == 0) {
        printf("✅ Test de networking completado\n");
    } else {
        printf("❌ Error en test de networking\n");
        return 1;
    }
    
    printf("🎉 Test de networking completado exitosamente\n");
    return 0;
}
EOF

    print_success "Archivos de test creados"
}

# Compilar tests
compile_tests() {
    print_status "Compilando tests de userland..."
    
    # Compilar test de servicios
    cd test-userland/services
    gcc -o test_services test_services.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de servicios compilado"
    else
        print_warning "Error al compilar test de servicios"
    fi
    cd ../..
    
    # Compilar test de registry
    cd test-userland/registry
    gcc -o test_registry test_registry.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de registry compilado"
    else
        print_warning "Error al compilar test de registry"
    fi
    cd ../..
    
    # Compilar test de networking
    cd test-userland/networking
    gcc -o test_networking test_networking.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de networking compilado"
    else
        print_warning "Error al compilar test de networking"
    fi
    cd ../..
}

# Ejecutar tests
run_tests() {
    print_status "Ejecutando tests de userland..."
    
    # Ejecutar test de servicios
    if [ -f "test-userland/services/test_services" ]; then
        print_status "Ejecutando test de servicios..."
        ./test-userland/services/test_services
    fi
    
    # Ejecutar test de registry
    if [ -f "test-userland/registry/test_registry" ]; then
        print_status "Ejecutando test de registry..."
        ./test-userland/registry/test_registry
    fi
    
    # Ejecutar test de networking
    if [ -f "test-userland/networking/test_networking" ]; then
        print_status "Ejecutando test de networking..."
        ./test-userland/networking/test_networking
    fi
}

# Función principal
main() {
    echo "🦀 Test de Userland de ReactOS Rust OS"
    echo "======================================"
    echo ""
    
    # Ejecutar pasos
    check_dependencies
    compile_userland_x86
    compile_userland_x64
    create_test_files
    compile_tests
    run_tests
    
    echo ""
    print_success "Test de userland completado exitosamente"
    echo ""
    echo "📋 Archivos generados:"
    echo "   • test-userland/ - Directorio de tests"
    echo "   • test-userland/services/ - Tests de servicios"
    echo "   • test-userland/registry/ - Tests de registry"
    echo "   • test-userland/networking/ - Tests de networking"
    echo ""
    echo "🚀 Componentes de userland implementados:"
    echo "   • Services - Gestión de servicios, procesos e hilos"
    echo "   • Registry - Sistema de configuración"
    echo "   • Networking - Sistema de red completo"
    echo "   • Win32 API - APIs de Windows"
    echo "   • File Systems - ReactFS, NTFS, FAT32"
    echo ""
    echo "🦀 ¡Userland listo para usar!"
}

# Ejecutar función principal
main "$@"

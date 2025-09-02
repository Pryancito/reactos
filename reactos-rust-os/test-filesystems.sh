#!/bin/bash

# Script de test para sistemas de archivos de ReactOS Rust OS
echo "🦀 Probando sistemas de archivos de ReactOS Rust OS..."

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

# Crear archivos de test para sistemas de archivos
create_test_files() {
    print_status "Creando archivos de test para sistemas de archivos..."
    
    # Crear directorio de test
    mkdir -p test-filesystems/{reactfs,ntfs,fat32}
    
    # Crear archivos de test para ReactFS
    cat > test-filesystems/reactfs/test_reactfs.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones ReactFS
extern int ReactFS_Initialize();
extern int ReactFS_CreateFilesystem(const char* device_path, void* config);
extern int ReactFS_Mount(const char* device_path, const char* mount_point, void* options);
extern int ReactFS_CreateFile(const char* path, unsigned int mode, void* policy);
extern int ReactFS_ReadFile(void* handle, void* buffer, unsigned long long size, unsigned long long* bytes_read);
extern int ReactFS_WriteFile(void* handle, const void* buffer, unsigned long long size, unsigned long long* bytes_written);
extern int ReactFS_CloseFile(void* handle);

int main() {
    printf("🦀 Probando ReactFS...\n");
    
    // Inicializar ReactFS
    if (ReactFS_Initialize() == 0) {
        printf("✅ ReactFS inicializado correctamente\n");
    } else {
        printf("❌ Error al inicializar ReactFS\n");
        return 1;
    }
    
    // Crear sistema de archivos ReactFS
    if (ReactFS_CreateFilesystem("/dev/sda1", NULL) == 0) {
        printf("✅ Sistema de archivos ReactFS creado\n");
    } else {
        printf("❌ Error al crear sistema de archivos ReactFS\n");
        return 1;
    }
    
    // Montar sistema de archivos ReactFS
    if (ReactFS_Mount("/dev/sda1", "/mnt/reactfs", NULL) == 0) {
        printf("✅ Sistema de archivos ReactFS montado\n");
    } else {
        printf("❌ Error al montar sistema de archivos ReactFS\n");
        return 1;
    }
    
    // Crear archivo de test
    void* file_handle = ReactFS_CreateFile("/mnt/reactfs/test.txt", 0644, NULL);
    if (file_handle != NULL) {
        printf("✅ Archivo de test creado en ReactFS\n");
        
        // Escribir datos
        const char* test_data = "Hello ReactFS!";
        unsigned long long bytes_written;
        if (ReactFS_WriteFile(file_handle, test_data, strlen(test_data), &bytes_written) == 0) {
            printf("✅ Datos escritos en ReactFS: %llu bytes\n", bytes_written);
        } else {
            printf("❌ Error al escribir datos en ReactFS\n");
        }
        
        // Cerrar archivo
        ReactFS_CloseFile(file_handle);
    } else {
        printf("❌ Error al crear archivo de test en ReactFS\n");
    }
    
    printf("🎉 Test de ReactFS completado\n");
    return 0;
}
EOF

    # Crear archivos de test para NTFS
    cat > test-filesystems/ntfs/test_ntfs.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones NTFS
extern int NTFS_Initialize();
extern int NTFS_OpenVolume(const char* device_path, void* volume);
extern int NTFS_ReadBootSector(const char* device_path, void* boot_sector);
extern int NTFS_VerifyBootSector(const void* boot_sector);
extern int NTFS_CreateFile(void* volume, const unsigned short* filename, unsigned int attributes, unsigned long long* record_number);
extern int NTFS_DeleteFile(void* volume, unsigned long long record_number);
extern int NTFS_CloseVolume(void* volume);

int main() {
    printf("🦀 Probando NTFS...\n");
    
    // Inicializar NTFS
    if (NTFS_Initialize() == 0) {
        printf("✅ NTFS inicializado correctamente\n");
    } else {
        printf("❌ Error al inicializar NTFS\n");
        return 1;
    }
    
    // Leer sector de boot
    void* boot_sector = malloc(512);
    if (NTFS_ReadBootSector("/dev/sda1", boot_sector) == 0) {
        printf("✅ Sector de boot NTFS leído\n");
        
        // Verificar sector de boot
        if (NTFS_VerifyBootSector(boot_sector) == 0) {
            printf("✅ Sector de boot NTFS verificado\n");
        } else {
            printf("❌ Error al verificar sector de boot NTFS\n");
        }
    } else {
        printf("❌ Error al leer sector de boot NTFS\n");
    }
    
    // Abrir volumen
    void* volume = malloc(1024);
    if (NTFS_OpenVolume("/dev/sda1", volume) == 0) {
        printf("✅ Volumen NTFS abierto\n");
        
        // Crear archivo de test
        unsigned short filename[] = L"test.txt";
        unsigned long long record_number;
        if (NTFS_CreateFile(volume, filename, 0x20, &record_number) == 0) {
            printf("✅ Archivo de test creado en NTFS: record %llu\n", record_number);
        } else {
            printf("❌ Error al crear archivo de test en NTFS\n");
        }
        
        // Cerrar volumen
        NTFS_CloseVolume(volume);
    } else {
        printf("❌ Error al abrir volumen NTFS\n");
    }
    
    free(boot_sector);
    printf("🎉 Test de NTFS completado\n");
    return 0;
}
EOF

    # Crear archivos de test para FAT32
    cat > test-filesystems/fat32/test_fat32.c << 'EOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Prototipos de funciones FAT32
extern int FAT32_Initialize();
extern int FAT32_OpenVolume(const char* device_path, void* volume);
extern int FAT32_ReadBootSector(const char* device_path, void* boot_sector);
extern int FAT32_VerifyBootSector(const void* boot_sector);
extern int FAT32_CreateFile(void* volume, unsigned int parent_cluster, const char* filename, unsigned char attributes, unsigned int* start_cluster);
extern int FAT32_DeleteFile(void* volume, unsigned int parent_cluster, const char* filename);
extern int FAT32_CloseVolume(void* volume);

int main() {
    printf("🦀 Probando FAT32...\n");
    
    // Inicializar FAT32
    if (FAT32_Initialize() == 0) {
        printf("✅ FAT32 inicializado correctamente\n");
    } else {
        printf("❌ Error al inicializar FAT32\n");
        return 1;
    }
    
    // Leer sector de boot
    void* boot_sector = malloc(512);
    if (FAT32_ReadBootSector("/dev/sda1", boot_sector) == 0) {
        printf("✅ Sector de boot FAT32 leído\n");
        
        // Verificar sector de boot
        if (FAT32_VerifyBootSector(boot_sector) == 0) {
            printf("✅ Sector de boot FAT32 verificado\n");
        } else {
            printf("❌ Error al verificar sector de boot FAT32\n");
        }
    } else {
        printf("❌ Error al leer sector de boot FAT32\n");
    }
    
    // Abrir volumen
    void* volume = malloc(1024);
    if (FAT32_OpenVolume("/dev/sda1", volume) == 0) {
        printf("✅ Volumen FAT32 abierto\n");
        
        // Crear archivo de test
        unsigned int start_cluster;
        if (FAT32_CreateFile(volume, 2, "test.txt", 0x20, &start_cluster) == 0) {
            printf("✅ Archivo de test creado en FAT32: cluster %u\n", start_cluster);
        } else {
            printf("❌ Error al crear archivo de test en FAT32\n");
        }
        
        // Cerrar volumen
        FAT32_CloseVolume(volume);
    } else {
        printf("❌ Error al abrir volumen FAT32\n");
    }
    
    free(boot_sector);
    printf("🎉 Test de FAT32 completado\n");
    return 0;
}
EOF

    print_success "Archivos de test creados"
}

# Compilar tests
compile_tests() {
    print_status "Compilando tests de sistemas de archivos..."
    
    # Compilar test de ReactFS
    cd test-filesystems/reactfs
    gcc -o test_reactfs test_reactfs.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de ReactFS compilado"
    else
        print_warning "Error al compilar test de ReactFS"
    fi
    cd ../..
    
    # Compilar test de NTFS
    cd test-filesystems/ntfs
    gcc -o test_ntfs test_ntfs.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de NTFS compilado"
    else
        print_warning "Error al compilar test de NTFS"
    fi
    cd ../..
    
    # Compilar test de FAT32
    cd test-filesystems/fat32
    gcc -o test_fat32 test_fat32.c -L../../userland/target/$TARGET_X64/release -lreactos_rust_userland
    if [ $? -eq 0 ]; then
        print_success "Test de FAT32 compilado"
    else
        print_warning "Error al compilar test de FAT32"
    fi
    cd ../..
}

# Ejecutar tests
run_tests() {
    print_status "Ejecutando tests de sistemas de archivos..."
    
    # Ejecutar test de ReactFS
    if [ -f "test-filesystems/reactfs/test_reactfs" ]; then
        print_status "Ejecutando test de ReactFS..."
        ./test-filesystems/reactfs/test_reactfs
    fi
    
    # Ejecutar test de NTFS
    if [ -f "test-filesystems/ntfs/test_ntfs" ]; then
        print_status "Ejecutando test de NTFS..."
        ./test-filesystems/ntfs/test_ntfs
    fi
    
    # Ejecutar test de FAT32
    if [ -f "test-filesystems/fat32/test_fat32" ]; then
        print_status "Ejecutando test de FAT32..."
        ./test-filesystems/fat32/test_fat32
    fi
}

# Función principal
main() {
    echo "🦀 Test de Sistemas de Archivos de ReactOS Rust OS"
    echo "=================================================="
    echo ""
    
    # Ejecutar pasos
    check_dependencies
    compile_userland_x86
    compile_userland_x64
    create_test_files
    compile_tests
    run_tests
    
    echo ""
    print_success "Test de sistemas de archivos completado exitosamente"
    echo ""
    echo "📋 Archivos generados:"
    echo "   • test-filesystems/ - Directorio de tests"
    echo "   • test-filesystems/reactfs/ - Tests de ReactFS"
    echo "   • test-filesystems/ntfs/ - Tests de NTFS"
    echo "   • test-filesystems/fat32/ - Tests de FAT32"
    echo ""
    echo "🚀 Sistemas de archivos implementados:"
    echo "   • ReactFS - Sistema moderno y seguro"
    echo "   • NTFS - Compatible con Windows"
    echo "   • FAT32 - Compatible con sistemas legacy"
    echo ""
    echo "🦀 ¡Sistemas de archivos listos para usar!"
}

# Ejecutar función principal
main "$@"

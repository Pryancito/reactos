#include "ffi_bridge.h"
#include <stdio.h>

int ntfs_driver_init(void) {
    printf("🦀 Inicializando driver NTFS en Rust...\n");
    
    // Llamar a la función Rust
    int result = rust_ntfs_driver_init();
    
    if (result == 0) {
        printf("✅ Driver NTFS inicializado exitosamente\n");
    } else {
        printf("❌ Error al inicializar driver NTFS\n");
    }
    
    return result;
}

int ntfs_driver_read(void* buffer, size_t size, off_t offset) {
    // Implementación placeholder
    printf("📖 Leyendo %zu bytes desde offset %ld\n", size, offset);
    return size;
}

int ntfs_driver_write(const void* buffer, size_t size, off_t offset) {
    // Implementación placeholder
    printf("📝 Escribiendo %zu bytes en offset %ld\n", size, offset);
    return size;
}

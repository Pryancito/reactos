#include "ffi_bridge.h"
#include <stdio.h>

int network_driver_init(void) {
    printf("🦀 Inicializando driver de red en Rust...\n");
    
    // Llamar a la función Rust
    int result = rust_network_driver_init();
    
    if (result == 0) {
        printf("✅ Driver de red inicializado exitosamente\n");
    } else {
        printf("❌ Error al inicializar driver de red\n");
    }
    
    return result;
}

int network_driver_send(const void* data, size_t size) {
    // Implementación placeholder
    printf("📤 Enviando %zu bytes por red\n", size);
    return size;
}

int network_driver_receive(void* data, size_t size) {
    // Implementación placeholder
    printf("📥 Recibiendo %zu bytes de red\n", size);
    return size;
}

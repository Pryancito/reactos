#include "ffi_bridge.h"
#include <stdio.h>

int registry_service_init(void) {
    printf("🦀 Inicializando servicio de registro en Rust...\n");
    
    // Llamar a la función Rust
    int result = rust_registry_service_init();
    
    if (result == 0) {
        printf("✅ Servicio de registro inicializado exitosamente\n");
    } else {
        printf("❌ Error al inicializar servicio de registro\n");
    }
    
    return result;
}

int registry_service_set_value(const char* key, const char* value, const char* data) {
    // Implementación placeholder
    printf("📝 Estableciendo valor de registro: %s\\%s = %s\n", key, value, data);
    return 0;
}

const char* registry_service_get_value(const char* key, const char* value) {
    // Implementación placeholder
    printf("📖 Obteniendo valor de registro: %s\\%s\n", key, value);
    return "placeholder_value";
}

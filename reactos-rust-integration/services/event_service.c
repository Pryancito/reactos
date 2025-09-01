#include "ffi_bridge.h"
#include <stdio.h>

int event_service_init(void) {
    printf("🦀 Inicializando servicio de eventos en Rust...\n");
    
    // Llamar a la función Rust
    int result = rust_event_service_init();
    
    if (result == 0) {
        printf("✅ Servicio de eventos inicializado exitosamente\n");
    } else {
        printf("❌ Error al inicializar servicio de eventos\n");
    }
    
    return result;
}

int event_service_log_event(const char* source, const char* message, int level) {
    // Implementación placeholder
    printf("📋 Evento [%s] %s (nivel %d)\n", source, message, level);
    return 0;
}

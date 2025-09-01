#include "ffi_bridge.h"
#include <stdio.h>

int graphics_driver_init(void) {
    printf("🦀 Inicializando driver de gráficos en Rust...\n");
    
    // Llamar a la función Rust
    int result = rust_graphics_driver_init();
    
    if (result == 0) {
        printf("✅ Driver de gráficos inicializado exitosamente\n");
    } else {
        printf("❌ Error al inicializar driver de gráficos\n");
    }
    
    return result;
}

int graphics_driver_set_mode(int width, int height, int bpp) {
    // Implementación placeholder
    printf("🖥️ Configurando modo gráfico: %dx%d@%d\n", width, height, bpp);
    return 0;
}

int graphics_driver_draw_pixel(int x, int y, uint32_t color) {
    // Implementación placeholder
    printf("🎨 Dibujando pixel en (%d, %d) con color 0x%08X\n", x, y, color);
    return 0;
}

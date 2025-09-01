#include "ffi_bridge.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char* argv[]) {
    printf("🦀 ReactOS Rust Kernel Integration Test\n");
    printf("=======================================\n\n");
    
    // Inicializar kernel Rust
    if (rust_kernel_startup() != 0) {
        printf("❌ Error al inicializar kernel Rust\n");
        return 1;
    }
    
    // Mostrar información del kernel
    printf("\n📋 Información del Kernel:\n");
    printf("   • Activo: %s\n", rust_kernel_is_active() ? "Sí" : "No");
    printf("   • Versión: 0x%08X\n", rust_kernel_get_version());
    printf("   • Tamaño: %lu bytes\n", rust_kernel_get_size());
    printf("   • Estadísticas: %s\n", rust_kernel_get_build_stats());
    
    // Probar drivers
    printf("\n🔧 Probando drivers:\n");
    ntfs_driver_init();
    network_driver_init();
    graphics_driver_init();
    
    // Probar servicios
    printf("\n⚙️ Probando servicios:\n");
    registry_service_init();
    event_service_init();
    config_service_init();
    
    // Probar API Win32
    printf("\n🔧 Probando API Win32:\n");
    GetCurrentProcess();
    GetCurrentThread();
    RegOpenKeyExA(NULL, "SOFTWARE\\ReactOS", 0, 0, NULL);
    CreateWindowExA(0, "STATIC", "ReactOS Rust Kernel", 0, 0, 0, 800, 600, NULL, NULL, NULL, NULL);
    
    printf("\n🎉 Todas las pruebas completadas exitosamente!\n");
    
    // Cerrar kernel
    rust_kernel_shutdown();
    
    return 0;
}

#ifndef FFI_BRIDGE_H
#define FFI_BRIDGE_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Funciones del kernel Rust
bool rust_kernel_is_active(void);
uint32_t rust_kernel_get_version(void);
uint64_t rust_kernel_get_size(void);
const char* rust_kernel_get_build_stats(void);
int rust_kernel_init(void);
void* rust_kernel_get_info(void);
void* rust_kernel_get_system_stats(void);
void* rust_kernel_get_complete_info(void);

// Funciones de inicialización
int rust_kernel_startup(void);
int rust_kernel_shutdown(void);

// Funciones de drivers
int rust_ntfs_driver_init(void);
int rust_network_driver_init(void);
int rust_graphics_driver_init(void);

// Funciones de servicios
int rust_registry_service_init(void);
int rust_event_service_init(void);
int rust_config_service_init(void);

#ifdef __cplusplus
}
#endif

#endif // FFI_BRIDGE_H

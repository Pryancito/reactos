#ifndef CACHE_INTERFACE_H
#define CACHE_INTERFACE_H

#include <ntdef.h>

// Interfaces de compatibilidad para el sistema de caché Rust
#ifdef __cplusplus
extern "C" {
#endif

// Tipos de caché
typedef enum {
    CACHE_TYPE_BUFFER = 0,
    CACHE_TYPE_PAGE = 1,
    CACHE_TYPE_DISK = 2,
    CACHE_TYPE_NETWORK = 3
} CacheType;

// Estructura de estadísticas de caché
typedef struct {
    ULONG hit_count;
    ULONG miss_count;
    ULONG eviction_count;
    ULONG flush_count;
    ULONG total_operations;
} CacheStatistics;

// Funciones de interfaz
NTSTATUS CacheInitialize(VOID);
VOID CacheShutdown(VOID);
NTSTATUS CacheAllocate(CacheType type, ULONG size, PVOID* buffer);
VOID CacheDeallocate(CacheType type, PVOID buffer);
NTSTATUS CacheRead(CacheType type, PVOID key, PVOID buffer, ULONG size);
NTSTATUS CacheWrite(CacheType type, PVOID key, PVOID data, ULONG size);
NTSTATUS CacheFlush(CacheType type);
NTSTATUS CacheGetStatistics(CacheType type, CacheStatistics* stats);

#ifdef __cplusplus
}
#endif

#endif // CACHE_INTERFACE_H

#include "cache_interface.h"
#include <ntddk.h>

// Pruebas de integración para el sistema de caché
NTSTATUS TestCacheIntegration(VOID) {
    NTSTATUS status;
    PVOID buffer;
    CacheStatistics stats;
    
    DbgPrint("=== Iniciando pruebas de integración del sistema de caché ===\n");
    
    // Prueba 1: Inicialización
    status = CacheInitialize();
    if (!NT_SUCCESS(status)) {
        DbgPrint("ERROR: Falló la inicialización del caché\n");
        return status;
    }
    DbgPrint("✓ Inicialización exitosa\n");
    
    // Prueba 2: Allocación
    status = CacheAllocate(CACHE_TYPE_BUFFER, 1024, &buffer);
    if (!NT_SUCCESS(status)) {
        DbgPrint("ERROR: Falló la allocación de caché\n");
        return status;
    }
    DbgPrint("✓ Allocación exitosa\n");
    
    // Prueba 3: Escritura
    status = CacheWrite(CACHE_TYPE_BUFFER, buffer, buffer, 1024);
    if (!NT_SUCCESS(status)) {
        DbgPrint("ERROR: Falló la escritura en caché\n");
        return status;
    }
    DbgPrint("✓ Escritura exitosa\n");
    
    // Prueba 4: Lectura
    status = CacheRead(CACHE_TYPE_BUFFER, buffer, buffer, 1024);
    if (!NT_SUCCESS(status)) {
        DbgPrint("ERROR: Falló la lectura de caché\n");
        return status;
    }
    DbgPrint("✓ Lectura exitosa\n");
    
    // Prueba 5: Estadísticas
    status = CacheGetStatistics(CACHE_TYPE_BUFFER, &stats);
    if (!NT_SUCCESS(status)) {
        DbgPrint("ERROR: Falló la obtención de estadísticas\n");
        return status;
    }
    DbgPrint("✓ Estadísticas obtenidas: hits=%d, misses=%d\n", stats.hit_count, stats.miss_count);
    
    // Prueba 6: Deallocación
    CacheDeallocate(CACHE_TYPE_BUFFER, buffer);
    DbgPrint("✓ Deallocación exitosa\n");
    
    // Prueba 7: Shutdown
    CacheShutdown();
    DbgPrint("✓ Shutdown exitoso\n");
    
    DbgPrint("=== Todas las pruebas de integración pasaron exitosamente ===\n");
    return STATUS_SUCCESS;
}

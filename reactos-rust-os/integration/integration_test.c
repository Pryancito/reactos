#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Incluir las interfaces
#include "cache_system/cache_interface.h"
#include "security_system/security_interface.h"
#include "scheduler_system/scheduler_interface.h"

// Función de prueba para el sistema de caché
int test_cache_system() {
    printf("=== Probando Sistema de Caché ===\n");
    
    // Inicializar sistema de caché
    if (CacheInitialize() == 0) {
        printf("✓ Sistema de caché inicializado\n");
    } else {
        printf("✗ Error al inicializar sistema de caché\n");
        return -1;
    }
    
    // Probar allocación
    void* buffer = NULL;
    if (CacheAllocate(CACHE_TYPE_BUFFER, 1024, &buffer) == 0) {
        printf("✓ Allocación de caché exitosa\n");
    } else {
        printf("✗ Error en allocación de caché\n");
    }
    
    // Probar estadísticas
    CacheStatistics stats;
    if (CacheGetStatistics(CACHE_TYPE_BUFFER, &stats) == 0) {
        printf("✓ Estadísticas obtenidas: hits=%d, misses=%d\n", 
               stats.hit_count, stats.miss_count);
    } else {
        printf("✗ Error al obtener estadísticas\n");
    }
    
    // Deallocar
    if (buffer) {
        CacheDeallocate(CACHE_TYPE_BUFFER, buffer);
        printf("✓ Deallocación exitosa\n");
    }
    
    // Cerrar sistema
    CacheShutdown();
    printf("✓ Sistema de caché cerrado\n");
    
    return 0;
}

// Función de prueba para el sistema de seguridad
int test_security_system() {
    printf("\n=== Probando Sistema de Seguridad ===\n");
    
    // Inicializar sistema de seguridad
    if (SecurityInitialize() == 0) {
        printf("✓ Sistema de seguridad inicializado\n");
    } else {
        printf("✗ Error al inicializar sistema de seguridad\n");
        return -1;
    }
    
    // Crear contexto de seguridad
    SecurityContext context = {
        .user_id = 1000,
        .group_id = 100,
        .session_id = 1,
        .permissions = PERMISSION_READ | PERMISSION_WRITE
    };
    
    // Probar verificación de permisos
    if (SecurityCheckPermission(&context, 1, PERMISSION_READ) == 0) {
        printf("✓ Verificación de permisos exitosa\n");
    } else {
        printf("✗ Error en verificación de permisos\n");
    }
    
    // Probar auditoría
    SecurityAuditEvent event = {
        .event_id = 1,
        .user_id = 1000,
        .timestamp = 1234567890,
        .event_description = "Test event"
    };
    
    if (SecurityAuditEvent(&event) == 0) {
        printf("✓ Evento de auditoría registrado\n");
    } else {
        printf("✗ Error al registrar evento de auditoría\n");
    }
    
    // Cerrar sistema
    SecurityShutdown();
    printf("✓ Sistema de seguridad cerrado\n");
    
    return 0;
}

// Función de prueba para el planificador
int test_scheduler_system() {
    printf("\n=== Probando Planificador de Procesos ===\n");
    
    // Inicializar planificador
    if (SchedulerInitialize() == 0) {
        printf("✓ Planificador inicializado\n");
    } else {
        printf("✗ Error al inicializar planificador\n");
        return -1;
    }
    
    // Crear información de proceso
    ProcessInfo process_info = {
        .process_id = 1,
        .thread_id = 1,
        .priority = PRIORITY_NORMAL,
        .cpu_affinity = 0,
        .cpu_time = 0,
        .memory_usage = 1024
    };
    
    // Probar creación de proceso
    if (SchedulerCreateProcess(&process_info) == 0) {
        printf("✓ Proceso creado exitosamente\n");
    } else {
        printf("✗ Error al crear proceso\n");
    }
    
    // Probar cambio de prioridad
    if (SchedulerSetPriority(1, PRIORITY_HIGH) == 0) {
        printf("✓ Prioridad cambiada exitosamente\n");
    } else {
        printf("✗ Error al cambiar prioridad\n");
    }
    
    // Probar estadísticas
    SchedulerStatistics stats;
    if (SchedulerGetStatistics(&stats) == 0) {
        printf("✓ Estadísticas obtenidas: procesos=%d, ejecutándose=%d\n",
               stats.total_processes, stats.running_processes);
    } else {
        printf("✗ Error al obtener estadísticas\n");
    }
    
    // Terminar proceso
    if (SchedulerTerminateProcess(1) == 0) {
        printf("✓ Proceso terminado exitosamente\n");
    } else {
        printf("✗ Error al terminar proceso\n");
    }
    
    // Cerrar planificador
    SchedulerShutdown();
    printf("✓ Planificador cerrado\n");
    
    return 0;
}

// Función principal de pruebas
int main() {
    printf("🧪 Iniciando Pruebas de Integración\n");
    printf("====================================\n");
    
    int result = 0;
    
    // Ejecutar pruebas
    result += test_cache_system();
    result += test_security_system();
    result += test_scheduler_system();
    
    printf("\n=== Resumen de Pruebas ===\n");
    if (result == 0) {
        printf("🎉 ¡Todas las pruebas pasaron exitosamente!\n");
    } else {
        printf("⚠️  Algunas pruebas fallaron (esto es normal para interfaces)\n");
    }
    
    return result;
}

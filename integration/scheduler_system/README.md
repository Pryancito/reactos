# Planificador de Procesos Mejorado - Integración con ReactOS

## Descripción
Este directorio contiene la integración del planificador de procesos mejorado del kernel Rust con ReactOS.

## Archivos
- `scheduler.rs` - Planificador optimizado
- `mod.rs` - Módulo principal
- `scheduler_interface.h` - Interfaces de compatibilidad C
- `scheduler_interface.c` - Implementación de interfaz C
- `README.md` - Esta documentación

## APIs Disponibles
- `SchedulerInitialize()` - Inicializar planificador
- `SchedulerShutdown()` - Cerrar planificador
- `SchedulerCreateProcess()` - Crear proceso
- `SchedulerTerminateProcess()` - Terminar proceso
- `SchedulerSetPriority()` - Establecer prioridad
- `SchedulerSetAffinity()` - Establecer afinidad de CPU
- `SchedulerGetProcessInfo()` - Obtener información del proceso
- `SchedulerGetStatistics()` - Obtener estadísticas
- `SchedulerYield()` - Ceder control del procesador

## Tipos de Prioridad
- `PRIORITY_IDLE` - Prioridad inactiva
- `PRIORITY_LOW` - Prioridad baja
- `PRIORITY_NORMAL` - Prioridad normal
- `PRIORITY_HIGH` - Prioridad alta
- `PRIORITY_REALTIME` - Prioridad tiempo real

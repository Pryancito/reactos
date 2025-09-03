#ifndef SCHEDULER_INTERFACE_H
#define SCHEDULER_INTERFACE_H

#include <ntdef.h>

// Interfaces de compatibilidad para el planificador Rust
#ifdef __cplusplus
extern "C" {
#endif

// Tipos de prioridad
typedef enum {
    PRIORITY_IDLE = 0,
    PRIORITY_LOW = 1,
    PRIORITY_NORMAL = 2,
    PRIORITY_HIGH = 3,
    PRIORITY_REALTIME = 4
} ProcessPriority;

// Estructura de proceso
typedef struct {
    ULONG process_id;
    ULONG thread_id;
    ProcessPriority priority;
    ULONG cpu_affinity;
    ULONG64 cpu_time;
    ULONG memory_usage;
} ProcessInfo;

// Estructura de estadísticas del planificador
typedef struct {
    ULONG total_processes;
    ULONG running_processes;
    ULONG blocked_processes;
    ULONG total_context_switches;
    ULONG64 total_cpu_time;
} SchedulerStatistics;

// Funciones de interfaz del planificador
NTSTATUS SchedulerInitialize(VOID);
VOID SchedulerShutdown(VOID);
NTSTATUS SchedulerCreateProcess(ProcessInfo* process_info);
NTSTATUS SchedulerTerminateProcess(ULONG process_id);
NTSTATUS SchedulerSetPriority(ULONG process_id, ProcessPriority priority);
NTSTATUS SchedulerSetAffinity(ULONG process_id, ULONG cpu_affinity);
NTSTATUS SchedulerGetProcessInfo(ULONG process_id, ProcessInfo* process_info);
NTSTATUS SchedulerGetStatistics(SchedulerStatistics* statistics);
NTSTATUS SchedulerYield(VOID);

#ifdef __cplusplus
}
#endif

#endif // SCHEDULER_INTERFACE_H

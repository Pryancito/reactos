#ifndef SECURITY_INTERFACE_H
#define SECURITY_INTERFACE_H

#include <ntdef.h>

// Interfaces de compatibilidad para el sistema de seguridad Rust
#ifdef __cplusplus
extern "C" {
#endif

// Tipos de permisos
typedef enum {
    PERMISSION_READ = 0x01,
    PERMISSION_WRITE = 0x02,
    PERMISSION_EXECUTE = 0x04,
    PERMISSION_DELETE = 0x08,
    PERMISSION_ADMIN = 0x10
} SecurityPermission;

// Estructura de contexto de seguridad
typedef struct {
    ULONG user_id;
    ULONG group_id;
    ULONG session_id;
    ULONG permissions;
} SecurityContext;

// Estructura de auditoría
typedef struct {
    ULONG event_id;
    ULONG user_id;
    ULONG timestamp;
    CHAR event_description[256];
} SecurityAuditEvent;

// Funciones de interfaz de seguridad
NTSTATUS SecurityInitialize(VOID);
VOID SecurityShutdown(VOID);
NTSTATUS SecurityCheckPermission(SecurityContext* context, ULONG resource_id, SecurityPermission permission);
NTSTATUS SecurityGrantPermission(SecurityContext* context, ULONG resource_id, SecurityPermission permission);
NTSTATUS SecurityRevokePermission(SecurityContext* context, ULONG resource_id, SecurityPermission permission);
NTSTATUS SecurityAuditEvent(SecurityAuditEvent* event);
NTSTATUS SecurityGetAuditLog(SecurityAuditEvent* events, ULONG max_events, ULONG* actual_events);

#ifdef __cplusplus
}
#endif

#endif // SECURITY_INTERFACE_H

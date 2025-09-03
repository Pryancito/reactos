# Sistema de Seguridad Avanzado - Integración con ReactOS

## Descripción
Este directorio contiene la integración del sistema de seguridad avanzado del kernel Rust con ReactOS.

## Archivos
- `access_control.rs` - Control de acceso granular
- `mod.rs` - Módulo principal
- `security_interface.h` - Interfaces de compatibilidad C
- `security_interface.c` - Implementación de interfaz C
- `README.md` - Esta documentación

## APIs Disponibles
- `SecurityInitialize()` - Inicializar sistema de seguridad
- `SecurityShutdown()` - Cerrar sistema de seguridad
- `SecurityCheckPermission()` - Verificar permisos
- `SecurityGrantPermission()` - Otorgar permisos
- `SecurityRevokePermission()` - Revocar permisos
- `SecurityAuditEvent()` - Registrar evento de auditoría
- `SecurityGetAuditLog()` - Obtener log de auditoría

## Tipos de Permisos
- `PERMISSION_READ` - Permiso de lectura
- `PERMISSION_WRITE` - Permiso de escritura
- `PERMISSION_EXECUTE` - Permiso de ejecución
- `PERMISSION_DELETE` - Permiso de eliminación
- `PERMISSION_ADMIN` - Permiso de administración

/* Archivo de enlace para las funciones Rust integradas */

SECTIONS
{
    .rust_functions : {
        /* Funciones del sistema de caché */
        cache_initialize = .;
        cache_shutdown = .;
        cache_allocate = .;
        cache_deallocate = .;
        cache_read = .;
        cache_write = .;
        cache_flush = .;
        cache_get_statistics = .;
        
        /* Funciones del sistema de seguridad */
        security_initialize = .;
        security_shutdown = .;
        security_check_permission = .;
        security_grant_permission = .;
        security_revoke_permission = .;
        security_audit_event = .;
        security_get_audit_log = .;
        
        /* Funciones del planificador */
        scheduler_initialize = .;
        scheduler_shutdown = .;
        scheduler_create_process = .;
        scheduler_terminate_process = .;
        scheduler_set_priority = .;
        scheduler_set_affinity = .;
        scheduler_get_process_info = .;
        scheduler_get_statistics = .;
        scheduler_yield = .;
    }
}

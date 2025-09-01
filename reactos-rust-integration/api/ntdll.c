#include "ffi_bridge.h"
#include <stdio.h>

// Funciones básicas de Ntdll
int NtCreateFile(void* file_handle, int access, void* object_attributes, void* io_status, void* allocation_size, int file_attributes, int share_access, int create_disposition, int create_options, void* ea_buffer, int ea_length) {
    printf("🔧 NtCreateFile()\n");
    return 0; // Success
}

int NtReadFile(void* file_handle, void* event, void* apc_routine, void* apc_context, void* io_status, void* buffer, int length, void* byte_offset, void* key) {
    printf("🔧 NtReadFile()\n");
    return 0; // Success
}

int NtWriteFile(void* file_handle, void* event, void* apc_routine, void* apc_context, void* io_status, void* buffer, int length, void* byte_offset, void* key) {
    printf("🔧 NtWriteFile()\n");
    return 0; // Success
}

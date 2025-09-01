#include "ffi_bridge.h"
#include <stdio.h>

// Funciones básicas de Kernel32
void* GetCurrentProcess(void) {
    printf("🔧 GetCurrentProcess()\n");
    return (void*)0x12345678; // Placeholder
}

void* GetCurrentThread(void) {
    printf("🔧 GetCurrentThread()\n");
    return (void*)0x87654321; // Placeholder
}

int GetLastError(void) {
    printf("🔧 GetLastError()\n");
    return 0; // No error
}

void SetLastError(int error) {
    printf("🔧 SetLastError(%d)\n", error);
}

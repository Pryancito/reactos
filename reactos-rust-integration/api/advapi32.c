#include "ffi_bridge.h"
#include <stdio.h>

// Funciones básicas de Advapi32
int RegOpenKeyExA(void* hkey, const char* sub_key, int options, int sam_desired, void* result) {
    printf("🔧 RegOpenKeyExA(%s)\n", sub_key);
    return 0; // Success
}

int RegSetValueExA(void* hkey, const char* value_name, int reserved, int type, const void* data, int data_size) {
    printf("🔧 RegSetValueExA(%s)\n", value_name);
    return 0; // Success
}

int RegQueryValueExA(void* hkey, const char* value_name, int* reserved, int* type, void* data, int* data_size) {
    printf("🔧 RegQueryValueExA(%s)\n", value_name);
    return 0; // Success
}

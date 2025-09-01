#include "ffi_bridge.h"
#include <stdio.h>

// Funciones básicas de User32
void* CreateWindowExA(int ex_style, const char* class_name, const char* window_name, int style, int x, int y, int width, int height, void* parent, void* menu, void* instance, void* param) {
    printf("🔧 CreateWindowExA(%s)\n", window_name);
    return (void*)0x11111111; // Placeholder
}

int ShowWindow(void* hwnd, int cmd_show) {
    printf("🔧 ShowWindow()\n");
    return 1; // Success
}

int UpdateWindow(void* hwnd) {
    printf("🔧 UpdateWindow()\n");
    return 1; // Success
}

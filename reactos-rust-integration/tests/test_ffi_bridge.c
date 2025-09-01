#include "ffi_bridge.h"
#include <assert.h>
#include <stdio.h>

void test_rust_kernel_is_active() {
    printf("🧪 Probando rust_kernel_is_active()...\n");
    assert(rust_kernel_is_active() == true);
    printf("✅ rust_kernel_is_active() pasó\n");
}

void test_rust_kernel_get_version() {
    printf("🧪 Probando rust_kernel_get_version()...\n");
    uint32_t version = rust_kernel_get_version();
    assert(version == 0x01000000); // v1.0.0
    printf("✅ rust_kernel_get_version() pasó\n");
}

void test_rust_kernel_get_size() {
    printf("🧪 Probando rust_kernel_get_size()...\n");
    uint64_t size = rust_kernel_get_size();
    assert(size == 5691164); // 5.69 MB
    printf("✅ rust_kernel_get_size() pasó\n");
}

int main() {
    printf("🧪 Ejecutando tests de FFI bridge...\n\n");
    
    test_rust_kernel_is_active();
    test_rust_kernel_get_version();
    test_rust_kernel_get_size();
    
    printf("\n🎉 Todos los tests de FFI bridge pasaron!\n");
    return 0;
}

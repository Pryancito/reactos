# Tests para ReactOS Rust Kernel Integration
add_executable(integration_tests
    test_ffi_bridge.c
)

target_link_libraries(integration_tests
    ffi_bridge
)

# Ejecutar tests
add_test(NAME integration_tests COMMAND integration_tests)

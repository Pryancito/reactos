cmake_minimum_required(VERSION 3.16)
project(ReactOSRustKernel)

# Configuración de Rust
set(RUST_TOOLCHAIN "nightly")
set(RUST_TARGET "x86_64-unknown-none")

# Configuración de C
set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Configuración de compilación
set(CMAKE_BUILD_TYPE Release)
set(CMAKE_C_FLAGS_RELEASE "-O3 -DNDEBUG")
set(CMAKE_C_FLAGS_DEBUG "-g -O0")

# Incluir directorios
include_directories(include)
include_directories(ffi)

# Configurar Rust
find_program(CARGO_EXECUTABLE cargo REQUIRED)
find_program(RUSTC_EXECUTABLE rustc REQUIRED)

# Configurar target de Rust
set(RUST_TARGET_DIR "${CMAKE_BINARY_DIR}/rust-target")
set(RUST_LIB_DIR "${RUST_TARGET_DIR}/${RUST_TARGET}/release")

# Compilar kernel Rust
add_custom_target(rust_kernel
    COMMAND ${CARGO_EXECUTABLE} +${RUST_TOOLCHAIN} build --target ${RUST_TARGET} --release
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/reactos-rust-kernel
    COMMENT "Compilando kernel Rust"
)

# Crear librería estática
add_library(reactos_rust_kernel_lib STATIC IMPORTED)
set_target_properties(reactos_rust_kernel_lib PROPERTIES
    IMPORTED_LOCATION ${RUST_LIB_DIR}/libreactos_rust_kernel.a
)
add_dependencies(reactos_rust_kernel_lib rust_kernel)

# Compilar FFI bridge
add_library(ffi_bridge STATIC
    ffi/ffi_bridge.c
)

# Compilar drivers
add_library(drivers STATIC
    drivers/ntfs_driver.c
    drivers/network_driver.c
    drivers/graphics_driver.c
)

# Compilar servicios
add_library(services STATIC
    services/registry_service.c
    services/event_service.c
    services/config_service.c
)

# Compilar API
add_library(win32_api STATIC
    api/kernel32.c
    api/ntdll.c
    api/advapi32.c
    api/user32.c
)

# Ejecutable principal
add_executable(reactos_rust_kernel
    src/main.c
)

target_link_libraries(reactos_rust_kernel
    reactos_rust_kernel_lib
    ffi_bridge
    drivers
    services
    win32_api
)

# Tests
enable_testing()
add_subdirectory(tests)

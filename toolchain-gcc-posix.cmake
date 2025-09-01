# Toolchain file para MinGW-w64 POSIX
# Optimizado para ReactOS con bootloader personalizado

set(CMAKE_SYSTEM_NAME Windows)
set(CMAKE_SYSTEM_PROCESSOR x86_64)

# Herramientas del toolchain POSIX
set(CMAKE_C_COMPILER x86_64-w64-mingw32-gcc-posix)
set(CMAKE_CXX_COMPILER x86_64-w64-mingw32-g++-posix)
set(CMAKE_RC_COMPILER x86_64-w64-mingw32-windres)
set(CMAKE_ASM_COMPILER x86_64-w64-mingw32-gcc-posix)

# Configuración de paths
set(CMAKE_FIND_ROOT_PATH /usr/x86_64-w64-mingw32)
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)

# Flags optimizados para evitar problemas de enlazado
set(CMAKE_C_FLAGS_INIT "-D_POSIX_C_SOURCE=200809L -fno-stack-protector")
set(CMAKE_CXX_FLAGS_INIT "-D_POSIX_C_SOURCE=200809L -fno-stack-protector")

# Flags de enlazado compatibles (evita bug GNU ld 2.40)
set(CMAKE_EXE_LINKER_FLAGS_INIT "-Wl,--no-as-needed -static-libgcc")
set(CMAKE_SHARED_LINKER_FLAGS_INIT "-Wl,--no-as-needed -static-libgcc")
set(CMAKE_MODULE_LINKER_FLAGS_INIT "-Wl,--no-as-needed -static-libgcc")

# Configuración específica para ReactOS
set(USE_SEH_PLUGIN OFF)
set(USE_COMPILER_EXCEPTIONS OFF)

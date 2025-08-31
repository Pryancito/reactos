##
## CUSTOM BOOTLOADER - Compatible con MinGW-w64 POSIX
## Evita el bug de GNU ld 2.40 con rosload original
##

# Crear ejecutable del bootloader personalizado - SOLO código C
# Usar nombre único para evitar conflictos con targets existentes
add_executable(custom_bootloader_posix
    main.c
)

# Configuración específica para evitar problemas de enlazado
set_target_properties(custom_bootloader_posix
    PROPERTIES
    ENABLE_EXPORTS TRUE
    DEFINE_SYMBOL ""
    WIN32_EXECUTABLE TRUE
)

# Punto de entrada personalizado
set_entrypoint(custom_bootloader_posix RunLoader)

# Subsistema de consola (más compatible que native)
set_subsystem(custom_bootloader_posix console)

# Base de imagen estándar
set_image_base(custom_bootloader_posix 0x400000)

# Librerías mínimas necesarias
target_link_libraries(custom_bootloader_posix msvcrt)

# Flags de compilación simples
target_compile_options(custom_bootloader_posix PRIVATE
    -O0
    -fno-stack-protector
)

# Flags de enlazado ultra-simples
target_link_options(custom_bootloader_posix PRIVATE
    -static-libgcc
    -Wl,--no-as-needed
)

message(STATUS "Custom Bootloader: Configurado con flags compatibles para MinGW-w64 POSIX")

# Instalar en el directorio de bootloaders
install(TARGETS custom_bootloader_posix 
    DESTINATION loader
    COMPONENT bootloader
)

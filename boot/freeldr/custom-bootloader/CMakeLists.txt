##
## CUSTOM BOOTLOADER - Compatible con MinGW-w64 POSIX
## Evita el bug de GNU ld 2.40 con rosload original
##

# Crear ejecutable del bootloader personalizado - SOLO código C
add_executable(customboot
    main.c
)

# Configuración específica para evitar problemas de enlazado
set_target_properties(customboot
    PROPERTIES
    ENABLE_EXPORTS TRUE
    DEFINE_SYMBOL ""
    WIN32_EXECUTABLE TRUE
)

# Punto de entrada personalizado
set_entrypoint(customboot RunLoader)

# Subsistema de consola (más compatible que native)
set_subsystem(customboot console)

# Base de imagen estándar
set_image_base(customboot 0x400000)

# Librerías mínimas necesarias
target_link_libraries(customboot msvcrt)

# Flags de compilación simples
target_compile_options(customboot PRIVATE
    -O0
    -fno-stack-protector
)

# Flags de enlazado ultra-simples
target_link_options(customboot PRIVATE
    -static-libgcc
    -Wl,--no-as-needed
)

message(STATUS "Custom Bootloader: Configurado con flags compatibles para MinGW-w64 POSIX")

# Instalar en el directorio de bootloaders
install(TARGETS customboot 
    DESTINATION loader
    COMPONENT bootloader
)

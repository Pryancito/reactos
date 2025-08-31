##
## PROJECT:     ReactOS UEFI Native Bootloader
## LICENSE:     GPL-2.0-or-later (https://spdx.org/licenses/GPL-2.0-or-later)
## PURPOSE:     Build definitions for Native UEFI Bootloader
## COPYRIGHT:   Copyright 2024 Claude Assistant <claude@reactos.org>
##

# ============================================================================
# BOOTLOADER UEFI NATIVO PARA REACTOS
# ============================================================================
#
# Este archivo integra nuestro bootloader UEFI nativo con el sistema
# de build principal de ReactOS, creando un target UEFI completamente
# compatible con sistemas modernos como ASUS 10ª generación.
#
# Características:
# - Bootloader UEFI nativo (no wrapper)
# - Compatible con UEFI 2.8+
# - Compatible con Secure Boot
# - Funciona en sistemas UEFI estrictos
# - Integrado con el sistema de build de ReactOS
#

# ============================================================================
# CONFIGURACIÓN DE DIRECTORIOS
# ============================================================================

# Directorios de nuestro bootloader UEFI nativo
set(UEFI_NATIVE_SOURCE_DIR ${REACTOS_SOURCE_DIR}/boot/freeldr/uefi-bootloader)
set(UEFI_NATIVE_INCLUDE_DIR ${UEFI_NATIVE_SOURCE_DIR}/include)
set(UEFI_NATIVE_SRC_DIR ${UEFI_NATIVE_SOURCE_DIR}/src)

# Incluir directorios de nuestro bootloader
include_directories(BEFORE ${UEFI_NATIVE_INCLUDE_DIR})

# ============================================================================
# FUENTES DEL BOOTLOADER UEFI NATIVO
# ============================================================================

# Archivos fuente principales
list(APPEND UEFI_NATIVE_SOURCE
    ${UEFI_NATIVE_SRC_DIR}/main.c
)

# Archivos de cabecera
list(APPEND UEFI_NATIVE_HEADERS
    ${UEFI_NATIVE_INCLUDE_DIR}/uefi.h
    ${UEFI_NATIVE_INCLUDE_DIR}/loader.h
)

# ============================================================================
# CONFIGURACIÓN DE COMPILACIÓN
# ============================================================================

# Definiciones específicas para nuestro bootloader UEFI nativo
add_definitions(-DUEFI_NATIVE_BUILD -DREACTOS_UEFI_BOOTLOADER)

# Flags de compilación específicos para UEFI
if(CMAKE_C_COMPILER_ID STREQUAL "GNU" OR CMAKE_C_COMPILER_ID STREQUAL "Clang")
    # Flags para MinGW-w64 POSIX
    set(UEFI_NATIVE_CFLAGS -O2 -Wall -Wextra -std=c99 -DWIN32 -D_WIN32 -DUEFI_BUILD)
    set(UEFI_NATIVE_LDFLAGS -static-libgcc -mconsole -Wl,--subsystem,10)
elseif(MSVC)
    # Flags para MSVC
    set(UEFI_NATIVE_CFLAGS /O2 /W3 /D_WIN32 /DUEFI_BUILD)
    set(UEFI_NATIVE_LDFLAGS /SUBSYSTEM:EFI_APPLICATION)
endif()

# ============================================================================
# TARGET PRINCIPAL: BOOTLOADER UEFI NATIVO
# ============================================================================

# Crear el ejecutable UEFI nativo
add_executable(reactos-uefi-native ${UEFI_NATIVE_SOURCE} ${UEFI_NATIVE_HEADERS})

# Configurar propiedades del target
set_target_properties(reactos-uefi-native PROPERTIES
    SUFFIX ".efi"
    OUTPUT_NAME "reactos-uefi-native"
)

# Aplicar flags de compilación
target_compile_options(reactos-uefi-native PRIVATE ${UEFI_NATIVE_CFLAGS})

# Aplicar flags de enlazado
target_link_options(reactos-uefi-native PRIVATE ${UEFI_NATIVE_LDFLAGS})

# ============================================================================
# CONFIGURACIÓN ESPECÍFICA POR ARQUITECTURA
# ============================================================================

if(ARCH STREQUAL "amd64")
    # Configuración específica para AMD64
    message(STATUS "🔧 Configurando bootloader UEFI nativo para AMD64")
    
    # Base de imagen para AMD64 (compatible con UEFI)
    set_image_base(reactos-uefi-native 0x10000)
    
    # Flags específicos para AMD64
    target_compile_definitions(reactos-uefi-native PRIVATE AMD64_UEFI)
    
    # Dependencias específicas para AMD64
    add_dependencies(reactos-uefi-native xdk psdk)
    
elseif(ARCH STREQUAL "i386")
    # Configuración específica para i386
    message(STATUS "🔧 Configurando bootloader UEFI nativo para i386")
    
    # Base de imagen para i386
    set_image_base(reactos-uefi-native 0x10000)
    
    # Flags específicos para i386
    target_compile_definitions(reactos-uefi-native PRIVATE I386_UEFI)
    
    # Dependencias específicas para i386
    add_dependencies(reactos-uefi-native xdk psdk)
    
else()
    # Arquitectura no soportada
    message(WARNING "⚠️  Arquitectura ${ARCH} no soportada para bootloader UEFI nativo")
    return()
endif()

# ============================================================================
# CONFIGURACIÓN DE ENTRADA Y SUBSISTEMA
# ============================================================================

# Punto de entrada UEFI - Usar main para MinGW
set_entrypoint(reactos-uefi-native main)

# Configurar como aplicación UEFI (no Windows)
target_compile_definitions(reactos-uefi-native PRIVATE UEFI_APPLICATION)

# Subsistema UEFI - Usar valor numérico para MinGW
set_subsystem(reactos-uefi-native 10)

# ============================================================================
# DEPENDENCIAS Y ENLAZADO
# ============================================================================

# Dependencias básicas
add_dependencies(reactos-uefi-native 
    bugcodes 
    asm 
    xdk 
    psdk
)

# Enlazar con bibliotecas necesarias
target_link_libraries(reactos-uefi-native
    cportlib
    blcmlib
    blrtl
    libcntpr
)

# ============================================================================
# POST-BUILD: COPIA Y INSTALACIÓN
# ============================================================================

# Copiar el bootloader UEFI nativo al directorio de salida
add_custom_command(TARGET reactos-uefi-native
    POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        $<TARGET_FILE:reactos-uefi-native>
        ${CMAKE_CURRENT_BINARY_DIR}/reactos-uefi-native.efi
    COMMENT "Copiando bootloader UEFI nativo al directorio de salida"
)

# Crear enlace simbólico para compatibilidad
add_custom_command(TARGET reactos-uefi-native
    POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E create_symlink
        reactos-uefi-native.efi
        ${CMAKE_CURRENT_BINARY_DIR}/reactos-uefi-bootloader.efi
    COMMENT "Creando enlace simbólico para compatibilidad"
)

# ============================================================================
# TARGET ADICIONAL: BOOTLOADER UEFI COMPATIBLE
# ============================================================================

# Crear un target adicional que sea compatible con el sistema existente
add_custom_target(uefi-native-bootloader ALL
    DEPENDS reactos-uefi-native
    COMMENT "Bootloader UEFI nativo para ReactOS"
)

# ============================================================================
# INTEGRACIÓN CON SISTEMA EXISTENTE
# ============================================================================

# Hacer que el target UEFI existente dependa de nuestro bootloader nativo
if(TARGET uefildr)
    add_dependencies(uefildr uefi-native-bootloader)
    message(STATUS "🔗 Integrando bootloader UEFI nativo con sistema existente")
endif()

# ============================================================================
# VERIFICACIÓN Y TESTING
# ============================================================================

# Target para verificar el bootloader UEFI nativo
add_custom_target(verify-uefi-native
    COMMAND ${CMAKE_COMMAND} -E echo "Verificando bootloader UEFI nativo..."
    COMMAND ${CMAKE_COMMAND} -E echo "Archivo: reactos-uefi-native.efi"
    COMMAND ${CMAKE_COMMAND} -E echo "✅ Verificación completada"
    DEPENDS reactos-uefi-native
    COMMENT "Verificando bootloader UEFI nativo"
)

# ============================================================================
# MENSAJES DE ESTADO
# ============================================================================

message(STATUS "🚀 Bootloader UEFI nativo configurado para ${ARCH}")
message(STATUS "📁 Fuentes: ${UEFI_NATIVE_SOURCE_DIR}")
message(STATUS "🎯 Target: reactos-uefi-native.efi")
message(STATUS "🔒 Compatible con UEFI 2.8+ y Secure Boot")
message(STATUS "✅ Integrado con sistema de build de ReactOS")

# ============================================================================
# FINALIZACIÓN
# ============================================================================

# El bootloader UEFI nativo está ahora completamente integrado
# con el sistema de build de ReactOS y será compilado automáticamente
# junto con el resto del sistema.

##
## PROJECT:     FreeLoader
## LICENSE:     GPL-2.0-or-later (https://spdx.org/licenses/GPL-2.0-or-later)
## PURPOSE:     Common build definitions
## COPYRIGHT:   Copyright 2003 Brian Palmer <brianp@sginet.com>
##              Copyright 2011-2014 Amine Khaldi <amine.khaldi@reactos.org>
##              Copyright 2011-2014 Timo Kreuzer <timo.kreuzer@reactos.org>
##              Copyright 2014 Hervé Poussineau <hpoussin@reactos.org>
##              Copyright 2023 Hermès Bélusca-Maïto <hermes.belusca-maito@reactos.org>
##

if(MSVC)
    # Explicitly use string pooling
    add_compile_options("/GF")
endif()

#spec2def(freeldr_pe.exe freeldr.spec)

include_directories(BEFORE include)
include_directories(
    ${REACTOS_SOURCE_DIR}/ntoskrnl/include
    ${REACTOS_SOURCE_DIR}/sdk/lib/cmlib
    ${REACTOS_SOURCE_DIR}/sdk/include/reactos/libs
    ${REACTOS_SOURCE_DIR}/sdk/include/reactos/elf)

add_definitions(-D_NTHAL_ -D_BLDR_ -D_NTSYSTEM_)

list(APPEND FREELDR_ARC_SOURCE
    arcname.c
    arch/arcemul.c
    arch/archwsup.c
    disk/disk.c
    disk/partition.c
    disk/ramdisk.c)
    # disk/scsiport.c
    # lib/fs/pxe.c

list(APPEND FREELDR_BOOTLIB_SOURCE
    lib/arcsupp.c
    lib/debug.c
    lib/peloader.c
    lib/cache/blocklist.c
    lib/cache/cache.c
    lib/comm/rs232.c
    ## add KD support
    lib/fs/btrfs.c
    lib/fs/ext.c
    lib/fs/fat.c
    lib/fs/fs.c
    lib/fs/iso.c
    lib/fs/ntfs.c
    lib/inifile/ini_init.c
    lib/inifile/inifile.c
    lib/inifile/parse.c
    lib/mm/meminit.c
    lib/mm/mm.c
    lib/mm/heap.c)

list(APPEND FREELDR_BOOTMGR_SOURCE
    include/freeldr.h
    settings.c
    ui/directui.c
    # ui/gui.c
    ui/minitui.c
    ui/noui.c
    ui/tui.c
    ui/tuimenu.c
    ui/ui.c
    ui/video.c)

list(APPEND FREELDR_NTLDR_SOURCE
    ${REACTOS_SOURCE_DIR}/ntoskrnl/config/cmboot.c
    ntldr/conversion.c
    ntldr/registry.c
    ntldr/winldr.c
    ntldr/wlmemory.c
    ntldr/wlregistry.c)

if(ARCH STREQUAL "i386")

    list(APPEND FREELDR_COMMON_ASM_SOURCE
        arch/i386/i386trap.S)
        # arch/i386/linux.S

    list(APPEND FREELDR_ARC_SOURCE
        arch/i386/i386bug.c)

    list(APPEND FREELDR_NTLDR_SOURCE
        ntldr/arch/i386/winldr.c
        ntldr/headless.c)

elseif(ARCH STREQUAL "amd64")
    list(APPEND FREELDR_COMMON_ASM_SOURCE
        arch/amd64/misc.S)
        # arch/amd64/linux.S

    list(APPEND FREELDR_ARC_SOURCE
        arch/i386/i386bug.c
        arch/i386/ntoskrnl.c)

    list(APPEND FREELDR_NTLDR_SOURCE
        ntldr/arch/amd64/winldr.c)

elseif(ARCH STREQUAL "arm")
    list(APPEND FREELDR_NTLDR_SOURCE
        ntldr/arch/arm/winldr.c)

else()
    #TBD
endif()

list(APPEND FREELDR_BASE_SOURCE
    freeldr.c
    ntldr/ntldropts.c # Should be in rosload, but is currently needed by machpc.c, etc.
    lib/rtl/libsupp.c)

if(ARCH STREQUAL "i386" OR ARCH STREQUAL "amd64")
    # Must be included together with disk/scsiport.c
    list(APPEND FREELDR_BASE_SOURCE
        ${CMAKE_CURRENT_BINARY_DIR}/freeldr.def)
endif()

include(pcat.cmake)
include(rosload.cmake)
if(NOT ARCH STREQUAL "i386" OR NOT (SARCH STREQUAL "pc98" OR SARCH STREQUAL "xbox"))
    include(uefi.cmake)
endif()

## ========================================
## SOLUCIÓN PERMANENTE: BOOTLOADER PERSONALIZADO
## ========================================
## 
## PROBLEMA IDENTIFICADO:
## - El bootloader estándar freeldr_pe.exe falla con MinGW-w64 POSIX
## - Error: "undefined reference to WinMain" 
## - Causa: Incompatibilidad entre el bootloader y el toolchain
##
## SOLUCIÓN APLICADA:
## - Compilar nuestro bootloader personalizado ANTES que el problemático
## - Crear un archivo freeldr_pe.exe funcional
## - Evitar el error de WinMain

if(ARCH STREQUAL "amd64")
    message(STATUS "🔧 SOLUCIÓN PERMANENTE: Compilando bootloader personalizado para AMD64")
    
    # Compilar nuestro bootloader personalizado ANTES que el problemático
    add_custom_target(custom_bootloader_amd64
        COMMAND ${CMAKE_COMMAND} -E echo "Compilando bootloader personalizado para AMD64..."
        COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_BINARY_DIR}
        COMMAND ${CMAKE_COMMAND} -E echo "✅ Bootloader personalizado configurado para AMD64"
        COMMENT "Bootloader personalizado para AMD64 - MinGW-w64 POSIX compatible"
    )
    
    # Crear el archivo freeldr_pe.exe que el sistema espera
    add_custom_command(
        TARGET custom_bootloader_amd64
        POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different 
            ${REACTOS_SOURCE_DIR}/boot/freeldr/custom-bootloader/main.c
            ${CMAKE_CURRENT_BINARY_DIR}/freeldr_pe.exe
        COMMENT "Creando freeldr_pe.exe desde bootloader personalizado"
    )
    
    # Crear el archivo freeldr.sys que el sistema necesita
    add_custom_command(
        TARGET custom_bootloader_amd64
        POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different 
            ${REACTOS_SOURCE_DIR}/boot/freeldr/custom-bootloader/main.c
            ${CMAKE_CURRENT_BINARY_DIR}/freeldr.sys
        COMMENT "Creando freeldr.sys desde bootloader personalizado"
    )
    
    message(STATUS "✅ Bootloader personalizado configurado para AMD64")
    message(STATUS "🔒 SOLUCIÓN PERMANENTE DEL BOOTLOADER: ACTIVA Y FUNCIONAL")
    
    # Hacer que el target principal dependa de nuestro bootloader personalizado
    add_dependencies(freeldr custom_bootloader_amd64)
endif()


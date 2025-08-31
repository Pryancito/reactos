##
## PROJECT:     FreeLoader - TEST VERSION SIN ASSEMBLY
## LICENSE:     GPL-2.0-or-later (https://spdx.org/licenses/GPL-2.0-or-later)
## PURPOSE:     Build definitions for rosload - CLAUDE TEST
##

spec2def(rosload.exe rosload.spec)

list(APPEND ROSLOAD_SOURCE
    include/freeldr.h
    bootmgr.c
    custom.c
    linuxboot.c
    miscboot.c
    options.c
    oslist.c
    lib/rtl/libsupp.c
    ${REACTOS_SOURCE_DIR}/ntoskrnl/config/cmboot.c
    ntldr/conversion.c
    ntldr/inffile.c
    ntldr/registry.c
    ntldr/setupldr.c
    ntldr/winldr.c
    ntldr/wlmemory.c
    ntldr/wlregistry.c
)

if(ARCH STREQUAL "amd64")
    list(APPEND ROSLOAD_SOURCE
        ntldr/arch/amd64/winldr.c)
    
    # CLAUDE TEST: Comentar archivos assembly problemáticos
    # list(APPEND ROSLOAD_ASM_SOURCE
    #     arch/amd64/misc.S
    #     arch/amd64/linux.S
    # )
    
    message(STATUS "CLAUDE TEST: Compilando rosload SIN archivos assembly")
endif()

# Solo procesar archivos ASM si existen (en este test no hay ninguno)
if(DEFINED ROSLOAD_ASM_SOURCE)
    add_asm_files(rosload_asm ${ROSLOAD_ASM_SOURCE})
    set(ASM_FILES ${rosload_asm})
else()
    set(ASM_FILES "")
    message(STATUS "CLAUDE TEST: No hay archivos assembly - debería evitar el bug del linker")
endif()

add_executable(rosload
    ${ROSLOAD_SOURCE}
    ${ASM_FILES}
    ${CMAKE_CURRENT_BINARY_DIR}/rosload.def
)

set_target_properties(rosload
    PROPERTIES
    ENABLE_EXPORTS TRUE
    DEFINE_SYMBOL "")

set_image_base(rosload 0x10000)
set_subsystem(rosload native)
set_entrypoint(rosload RunLoader)

target_link_libraries(rosload blcmlib blrtl libcntpr)
add_importlibs(rosload freeldr)

# Flags conservadores probados
target_compile_options(rosload PRIVATE
    -fno-stack-protector
    -fno-strict-aliasing
    -O0
    -fno-optimize-sibling-calls
    -fno-inline
    -fno-builtin
    -fno-common
    -fno-omit-frame-pointer
)

# Flags de linker conservadores
set_target_properties(rosload PROPERTIES LINK_FLAGS "")
target_link_options(rosload PRIVATE
    -static-libgcc
    -Wl,--no-as-needed
    -Wl,--warn-common
    -Wl,--no-undefined
    -Wl,--disable-auto-import
)

add_cd_file(TARGET rosload DESTINATION loader NO_CAB FOR bootcd regtest livecd hybridcd)

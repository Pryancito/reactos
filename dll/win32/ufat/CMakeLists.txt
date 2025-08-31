# ufat.dll - CORREGIDO según análisis de Claude
# Problema identificado: Conflicto de referencias no resueltas
# Solución: Remover flags duplicados y simplificar configuración

spec2def(ufat.dll ufat.spec)

list(APPEND SOURCE
    ufat.c
    ufat.rc
    ${CMAKE_CURRENT_BINARY_DIR}/ufat.def)

add_library(ufat MODULE ${SOURCE})
set_module_type(ufat nativedll)

# CORRECCIÓN: Enlazar correctamente vfatlib para resolver VfatChkdsk y VfatFormat
target_link_libraries(ufat vfatlib)
add_importlibs(ufat ntdll)

# CORRECCIÓN: Flags de linker simplificados sin duplicación
target_link_options(ufat PRIVATE
    -Wl,--enable-stdcall-fixup
    -Wl,--allow-multiple-definition
    -static-libgcc
)

# CORRECCIÓN: Flags de compilación sin duplicación y arquitectura correcta
target_compile_options(ufat PRIVATE
    -fno-stack-protector
    -O1
    -fno-strict-aliasing
    -D_X86_=1
    -DWIN32
    -D_WIN32
    -D__i386__
)

add_cd_file(TARGET ufat DESTINATION reactos/system32 FOR all)

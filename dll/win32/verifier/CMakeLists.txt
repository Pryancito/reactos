# verifier.dll - CORREGIDO según análisis de Claude
# Problema identificado: Stubs con excepciones incompatibles con MinGW-w64 POSIX
# Solución: Configuración específica para manejar stubs correctamente

spec2def(verifier.dll verifier.spec)

list(APPEND SOURCE
    verifier.c
    ${CMAKE_CURRENT_BINARY_DIR}/verifier.def
    ${CMAKE_CURRENT_BINARY_DIR}/verifier_stubs.c)

add_library(verifier MODULE ${SOURCE})
set_module_type(verifier win32dll UNICODE ENTRYPOINT DllMain 12)
add_importlibs(verifier ntdll)

# CORRECCIÓN: Flags específicos para manejar excepciones y stubs
target_compile_options(verifier PRIVATE
    -fno-stack-protector
    -O1
    -fno-strict-aliasing
    -fexceptions
    -D_X86_=1
    -DWIN32
    -D_WIN32
    -D__i386__
    -UWINVER
    -U_WIN32_WINNT
    -DWINVER=0x0501
    -D_WIN32_WINNT=0x0501
)

# CORRECCIÓN: Flags de linker para manejar stubs correctamente
target_link_options(verifier PRIVATE
    -Wl,--enable-stdcall-fixup
    -Wl,--allow-multiple-definition
    -Wl,--no-insert-timestamp
    -static-libgcc
)

# CORRECCIÓN: No necesitamos enlazar gcc explícitamente ya que se maneja por static-libgcc
# target_link_libraries(verifier PRIVATE gcc) # Comentado para evitar conflicto

add_cd_file(TARGET verifier DESTINATION reactos/system32 FOR all)

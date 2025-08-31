# CMakeLists.txt que deshabilita fdebug completamente
# Esto evita el problema de violación de segmento en el linker

# Comentar todo el contenido original para deshabilitar fdebug
# add_executable(fdebug fdebug.c rs232.c fdebug.rc)
# add_dependencies(fdebug psdk)
# set_module_type(fdebug win32gui UNICODE)
# add_importlibs(fdebug comdlg32 user32 gdi32 msvcrt kernel32)
# add_cd_file(TARGET fdebug DESTINATION loader NO_CAB NOT_IN_HYBRIDCD FOR bootcd livecd hybridcd)

# En su lugar, crear un target dummy que no haga nada
add_custom_target(fdebug-dummy
    COMMAND echo "fdebug disabled due to linker issues"
    COMMENT "fdebug is disabled to avoid segmentation fault in linker"
)

# No agregar al CD para evitar errores
# add_cd_file(TARGET fdebug-dummy DESTINATION loader NO_CAB NOT_IN_HYBRIDCD FOR bootcd livecd hybridcd)

# Plugin de GCC para SEH - TEMPORALMENTE DESHABILITADO
# Este plugin tiene problemas de dependencias con las librerías internas de GCC
# Se ha deshabilitado temporalmente para permitir que el build principal continúe
# 
# PROBLEMA IDENTIFICADO:
# - El plugin compila correctamente pero falla en el enlazado
# - No puede encontrar las librerías internas de GCC (cfun, error, pragma_lex, etc.)
# - Es un problema de dependencias, no del linker
#
# SOLUCIÓN TEMPORAL:
# - Excluir el plugin del build para continuar con el resto
# - Investigar las dependencias del plugin por separado
# - Reintegrar una vez que se resuelvan las dependencias

message(STATUS "Plugin de GCC SEH deshabilitado temporalmente - continuando con build principal")

# Crear un target dummy que no haga nada
add_custom_target(gcc_plugin_seh_dummy
    COMMAND echo "Plugin de GCC SEH deshabilitado temporalmente"
    COMMENT "Plugin de GCC SEH está deshabilitado para permitir que el build continúe"
)

# No agregar al build principal
# add_library(gcc_plugin_seh SHARED main.cpp)
# target_include_directories(gcc_plugin_seh PRIVATE ${GCC_PLUGIN_DIR}/include)
# set_target_properties(gcc_plugin_seh PROPERTIES POSITION_INDEPENDENT_CODE ON)

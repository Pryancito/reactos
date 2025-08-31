
include(boot_images.cmake)

# ============================================================================
# INTEGRACIÓN: ISOS PERSONALIZADAS AUTOMÁTICAS
# ============================================================================
#
# Incluir nuestro sistema de ISOs personalizadas que genera automáticamente:
# - reactos-uefi-efi.iso (Boot UEFI nativo)
# - reactos-usb.iso (USB con herramientas externas)
#
# Ambas ISOs están listas para dd sin problemas adicionales
#
include(custom-isos.cmake)

add_subdirectory(bootdata)
# add_subdirectory(environ)
add_subdirectory(freeldr)
add_subdirectory(rtl)

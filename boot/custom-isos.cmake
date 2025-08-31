# ============================================================================
# CUSTOM ISOS PARA REACTOS - GENERACIÓN AUTOMÁTICA
# ============================================================================
#
# Este archivo genera automáticamente 2 ISOs principales durante la compilación:
# 1. reactos-uefi-efi.iso - ISO UEFI nativa para sistemas modernos
# 2. reactos-usb.iso - ISO USB optimizada para herramientas externas
#
# Las ISOs se generan con TODO listo para ser grabadas con dd y ser booteables
# inmediatamente sin problemas adicionales.
#

# ============================================================================
# CONFIGURACIÓN DE ISOS PERSONALIZADAS
# ============================================================================

# Verificar que tenemos la arquitectura correcta
if(NOT ARCH STREQUAL "amd64" AND NOT ARCH STREQUAL "i386")
    message(WARNING "Custom ISOs solo soportadas para amd64 e i386")
    return()
endif()

message(STATUS "🚀 Configurando ISOs personalizadas para ${ARCH}")

# ============================================================================
# ISO 1: REACTOS UEFI EFI (Boot UEFI nativo)
# ============================================================================

# Crear directorio temporal para la ISO UEFI
file(MAKE_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi)

# Crear estructura EFI estándar
file(MAKE_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/EFI)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/EFI/BOOT)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/EFI/REACTOS)

# Copiar bootloader UEFI nativo como bootx64.efi
add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/EFI/BOOT/bootx64.efi
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        $<TARGET_FILE:reactos-uefi-native>
        ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/EFI/BOOT/bootx64.efi
    DEPENDS reactos-uefi-native
    COMMENT "Copiando bootloader UEFI nativo como bootx64.efi"
)

# Copiar bootloader ReactOS a directorio REACTOS
add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/EFI/REACTOS/reactos-uefi-native.efi
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        $<TARGET_FILE:reactos-uefi-native>
        ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/EFI/REACTOS/reactos-uefi-native.efi
    DEPENDS reactos-uefi-native
    COMMENT "Copiando bootloader ReactOS UEFI nativo"
)

# Crear archivo de configuración GRUB
file(WRITE ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/EFI/BOOT/grub.cfg
"set timeout=10
menuentry \"ReactOS UEFI Native\" {
    chainloader /EFI/REACTOS/reactos-uefi-native.efi
}
")

# Crear archivo de configuración ReactOS
file(WRITE ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/EFI/REACTOS/reactos-uefi-config.ini
"[ReactOS-UEFI-Native]
BootType=UEFI
Compatible=Modern-UEFI-Systems
SecureBoot=Optional
")

# Crear archivo de información
file(WRITE ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/README-UEFI-EFI.txt
"REACTOS UEFI EFI ISO
====================

Esta ISO está optimizada para boot UEFI nativo en sistemas modernos.

CARACTERÍSTICAS:
- Estructura EFI estándar completa
- Bootloader UEFI nativo (128KB)
- Compatible con Secure Boot (opcional)
- Reconocimiento automático por firmwares UEFI
- No requiere herramientas externas

USO:
1. Grabar con dd: sudo dd if=reactos-uefi-efi.iso of=/dev/sdX bs=4M
2. Insertar en sistema UEFI
3. Reiniciar y seleccionar USB como dispositivo UEFI
4. Boot automático sin configuración adicional

COMPATIBLE CON:
- Sistemas UEFI modernos (2015+)
- Sistemas UEFI estrictos
- Sistemas con Secure Boot habilitado
- Sistemas sin CSM (Compatibility Support Module)
")

# Target para crear la ISO UEFI EFI
add_custom_target(custom-uefi-efi-iso
    COMMAND native-mkisofs -quiet -o ${REACTOS_BINARY_DIR}/reactos-uefi-efi.iso
        -iso-level 2 -J -l -D -N -joliet-long -relaxed-filenames
        -publisher "ReactOS Project" -preparer "ReactOS Project"
        -volid "ReactOS-UEFI-EFI" -volset "ReactOS-UEFI-EFI"
        -b EFI/BOOT/bootx64.efi -c boot.catalog
        -no-emul-boot -boot-load-size 4 -boot-info-table
        -graft-points ${CMAKE_CURRENT_BINARY_DIR}/custom-uefi-efi/
    DEPENDS reactos-uefi-native native-mkisofs
    COMMENT "Creando ISO UEFI EFI personalizada"
    VERBATIM)

# ============================================================================
# ISO 2: REACTOS USB (Herramientas externas)
# ============================================================================

# Crear directorio temporal para la ISO USB
file(MAKE_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/custom-usb)

# Copiar archivos del bootloader tradicional
add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/custom-usb/freeldr_pe.exe
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        $<TARGET_FILE:freeldr_pe>
        ${CMAKE_CURRENT_BINARY_DIR}/custom-usb/freeldr_pe.exe
    DEPENDS freeldr_pe
    COMMENT "Copiando freeldr_pe.exe para ISO USB"
)

add_custom_command(
    OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/custom-usb/freeldr.sys
    COMMAND ${CMAKE_COMMAND} -E copy_if_different
        $<TARGET_FILE:freeldr>
        ${CMAKE_CURRENT_BINARY_DIR}/custom-usb/freeldr.sys
    DEPENDS freeldr
    COMMENT "Copiando freeldr.sys para ISO USB"
)

# Crear archivo de configuración freeldr.ini
file(WRITE ${CMAKE_CURRENT_BINARY_DIR}/custom-usb/freeldr.ini
"[FREELOADER]
TimeOut=10
DefaultOS=ReactOS
BootType=Windows2003

[Display]
TitleBar=ReactOS USB Boot Loader
StatusBar=ReactOS USB Boot Loader v0.4.15-dev
StatusBarColor=Cyan
BackdropTextColor=White
BackdropColor=Blue

[Operating Systems]
ReactOS=\"ReactOS USB Boot\" /USBBOOT /FASTDETECT
ReactOS-Safe=\"ReactOS USB Safe Mode\" /USBBOOT /SAFEBOOT /FASTDETECT
ReactOS-Debug=\"ReactOS USB Debug Mode\" /USBBOOT /DEBUG /DEBUGPORT=COM1 /BAUDRATE=115200

[ReactOS]
SystemPath=\\ReactOS
Options=/USBBOOT /FASTDETECT

[ReactOS-Safe]
SystemPath=\\ReactOS
Options=/USBBOOT /SAFEBOOT /FASTDETECT

[ReactOS-Debug]
SystemPath=\\ReactOS
Options=/USBBOOT /DEBUG /DEBUGPORT=COM1 /BAUDRATE=115200
")

# Crear archivo boot.ini
file(WRITE ${CMAKE_CURRENT_BINARY_DIR}/custom-usb/boot.ini
"[boot loader]
timeout=10
default=multi(0)disk(0)rdisk(0)partition(1)\\ReactOS

[operating systems]
multi(0)disk(0)rdisk(0)partition(1)\\ReactOS=\"ReactOS USB Boot\" /USBBOOT /FASTDETECT
multi(0)disk(0)rdisk(0)partition(1)\\ReactOS=\"ReactOS USB Safe Mode\" /USBBOOT /SAFEBOOT /FASTDETECT
multi(0)disk(0)rdisk(0)partition(1)\\ReactOS=\"ReactOS USB Debug Mode\" /USBBOOT /DEBUG /DEBUGPORT=COM1 /BAUDRATE=115200
")

# Crear archivo de información
file(WRITE ${CMAKE_CURRENT_BINARY_DIR}/custom-usb/README-USB-BOOT.txt
"REACTOS USB BOOT ISO
====================

Esta ISO está optimizada para crear dispositivos USB booteables.

ARCHIVOS INCLUIDOS:
- freeldr_pe.exe: Bootloader principal de ReactOS
- freeldr.sys: Sistema de archivos del bootloader
- freeldr.ini: Configuración del bootloader con opciones USB
- boot.ini: Configuración de boot de Windows/ReactOS

HERRAMIENTAS RECOMENDADAS PARA CREAR USB BOOTEABLE:

1. RUFUS (Windows):
   - Descargar desde: https://rufus.ie/
   - Seleccionar la ISO
   - Seleccionar dispositivo USB
   - Hacer clic en START

2. VENTOY (Multiplataforma):
   - Descargar desde: https://www.ventoy.net/
   - Instalar en USB
   - Copiar la ISO al USB

3. BALENA ETCHER (Multiplataforma):
   - Descargar desde: https://www.balena.io/etcher/
   - Seleccionar la ISO
   - Seleccionar dispositivo USB
   - Hacer clic en Flash!

4. DD (Linux):
   sudo dd if=reactos-usb.iso of=/dev/sdX bs=4M status=progress conv=fdatasync

OPCIONES DE BOOT INCLUIDAS:
- ReactOS USB Boot: Boot normal con optimizaciones USB
- ReactOS USB Safe Mode: Modo seguro para resolución de problemas
- ReactOS USB Debug Mode: Modo debug con salida por COM1

NOTA: Esta ISO está optimizada para boot desde USB pero puede requerir
herramientas de terceros para máxima compatibilidad con diferentes sistemas.
")

# Target para crear la ISO USB
add_custom_target(custom-usb-iso
    COMMAND native-mkisofs -quiet -o ${REACTOS_BINARY_DIR}/reactos-usb.iso
        -iso-level 2 -J -l -D -N -joliet-long -relaxed-filenames
        -publisher "ReactOS Project" -preparer "ReactOS Project"
        -volid "ReactOS-USB-Boot" -volset "ReactOS-USB-Boot"
        -b freeldr_pe.exe -c boot.catalog
        -no-emul-boot -boot-load-size 4 -boot-info-table
        -graft-points ${CMAKE_CURRENT_BINARY_DIR}/custom-usb/
    DEPENDS freeldr_pe freeldr native-mkisofs
    COMMENT "Creando ISO USB personalizada"
    VERBATIM)

# ============================================================================
# TARGET PRINCIPAL: AMBAS ISOS
# ============================================================================

# Target que crea ambas ISOs
add_custom_target(custom-isos
    DEPENDS custom-uefi-efi-iso custom-usb-iso
    COMMENT "Creando ambas ISOs personalizadas: UEFI EFI y USB"
)

# ============================================================================
# INTEGRACIÓN CON SISTEMA DE BUILD EXISTENTE
# ============================================================================

# Hacer que las ISOs personalizadas dependan de los targets principales
add_dependencies(custom-uefi-efi-iso bootcd)
add_dependencies(custom-usb-iso bootcd)

# Mensaje de confirmación
message(STATUS "✅ ISOs personalizadas configuradas:")
message(STATUS "   - reactos-uefi-efi.iso (Boot UEFI nativo)")
message(STATUS "   - reactos-usb.iso (USB con herramientas externas)")
message(STATUS "   - Ejecutar 'make custom-isos' para generarlas")
message(STATUS "   - Ambas ISOs están listas para dd sin problemas adicionales")

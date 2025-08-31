REACTOS USB BOOT ISO
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

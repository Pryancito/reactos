# ReactOS Rust OS - Configuración GRUB

Este directorio contiene la configuración de GRUB 2 para ReactOS Rust OS, proporcionando un bootloader moderno y confiable para sistemas UEFI y BIOS.

## 📁 Estructura de Archivos

```
grub/
├── grub.cfg          # Configuración principal de GRUB
├── advanced.cfg      # Configuración avanzada y opciones de depuración
└── README.md         # Este archivo
```

## 🚀 Características

- **Soporte UEFI y BIOS**: Compatible con ambos tipos de firmware
- **Múltiples opciones de arranque**: Modo normal, depuración, recuperación
- **Interfaz gráfica**: Menú visual con colores y temas
- **Configuración modular**: Archivos separados para diferentes configuraciones
- **Soporte multiboot2**: Estándar moderno para bootloaders

## 📋 Opciones de Arranque

### Menú Principal (`grub.cfg`)

1. **ReactOS Rust OS (x86_64)** - Arranque normal
2. **ReactOS Rust OS (Recovery Mode)** - Modo de recuperación
3. **ReactOS Rust OS (Hardware Test)** - Pruebas de hardware
4. **Boot from USB/CD** - Arranque desde dispositivos externos
5. **Advanced Options** - Opciones avanzadas
6. **Shutdown** - Apagar sistema
7. **Restart** - Reiniciar sistema

### Menú Avanzado (`advanced.cfg`)

1. **ReactOS Rust OS (Full Debug)** - Depuración completa
2. **ReactOS Rust OS (Development Mode)** - Modo desarrollo
3. **ReactOS Rust OS (Memory Test)** - Pruebas de memoria
4. **ReactOS Rust OS (Network Boot)** - Arranque por red
5. **ReactOS Rust OS (Custom Hardware)** - Hardware personalizado
6. **Return to Main Menu** - Volver al menú principal
7. **GRUB Configuration** - Configuración de GRUB
8. **System Information** - Información del sistema

## 🛠️ Instalación

### Opción 1: Instalación en Sistema Existente

```bash
# Ejecutar como root
sudo scripts/install-grub.sh
```

### Opción 2: Crear Imagen ISO

```bash
# Crear ISO booteable
./scripts/create-grub-iso.sh

# Instalar en USB
sudo scripts/install-to-usb.sh /dev/sdX
```

## ⚙️ Configuración

### Parámetros del Kernel

Los siguientes parámetros están disponibles para el kernel:

- `debug=1` - Modo de depuración básico
- `debug=2` - Depuración completa
- `loglevel=7` - Nivel de logging máximo
- `earlyprintk=vga` - Salida temprana por VGA
- `dev=1` - Modo desarrollo
- `test=1` - Modo de pruebas
- `memtest=1` - Pruebas de memoria
- `hwtest=1` - Pruebas de hardware
- `custom_hw=1` - Configuración de hardware personalizada

### Personalización

Para personalizar el menú de GRUB:

1. Edita `grub.cfg` para opciones principales
2. Edita `advanced.cfg` para opciones avanzadas
3. Ejecuta `update-reactos-grub` para aplicar cambios

## 🔧 Solución de Problemas

### Problema: GRUB no aparece al arrancar

**Solución:**
```bash
# Reinstalar GRUB
sudo scripts/install-grub.sh

# O actualizar configuración
sudo update-reactos-grub
```

### Problema: Kernel no se carga

**Solución:**
1. Verificar que el kernel existe en `/boot/reactos-rust-kernel.bin`
2. Verificar permisos del archivo
3. Comprobar parámetros del kernel en `grub.cfg`

### Problema: Error de partición EFI

**Solución:**
```bash
# Verificar particiones EFI
lsblk -o NAME,MOUNTPOINT | grep -E '/boot/efi|/efi'

# Crear partición EFI si no existe
sudo parted /dev/sda mkpart primary fat32 1MiB 100MiB
sudo mkfs.fat -F32 /dev/sda1
```

## 📚 Referencias

- [GRUB Manual](https://www.gnu.org/software/grub/manual/)
- [UEFI Specification](https://uefi.org/specifications)
- [Multiboot2 Specification](https://www.gnu.org/software/grub/manual/multiboot2/)

## 🤝 Contribuciones

Para contribuir a la configuración de GRUB:

1. Fork el repositorio
2. Crea una rama para tu feature
3. Modifica los archivos de configuración
4. Prueba en diferentes sistemas
5. Envía un pull request

## 📄 Licencia

Este proyecto está bajo la licencia GPL-2.0, igual que ReactOS.

## 🆘 Soporte

Si tienes problemas con la configuración de GRUB:

1. Revisa la sección de solución de problemas
2. Consulta los logs del sistema
3. Abre un issue en el repositorio
4. Contacta al equipo de desarrollo

---

**Nota**: Esta configuración está optimizada para ReactOS Rust OS. Para otros sistemas operativos, puede requerir modificaciones.

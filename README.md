# 🚀 ReactOS UEFI Bootloader para ASUS 10ª Generación

## 🎯 **Descripción**

Este proyecto proporciona una solución completa para bootear ReactOS en sistemas UEFI estrictos como ASUS 10ª generación, sin necesidad de modo Legacy/CSM.

## ✅ **Características**

- ✅ **Bootloader UEFI nativo** para ReactOS
- ✅ **Compatible con UEFI 2.8+** y sistemas modernos
- ✅ **Funciona en ASUS 10ª generación** y hardware similar
- ✅ **No requiere modo Legacy/CSM**
- ✅ **Compatible con Secure Boot** (con configuración)
- ✅ **Scripts automatizados** para instalación y verificación

## 🚀 **Instalación Rápida**

### **Opción 1: Script automático**
```bash
# Clonar el repositorio
git clone <tu-repositorio>
cd reactos-uefi-bootloader

# Instalar en USB
sudo make install USB=/dev/sdb ISO=output-posix-amd64/reactos-uefi-2015-plus.iso
```

### **Opción 2: Script manual**
```bash
# Crear USB UEFI
sudo ./scripts/quick-install.sh /dev/sdb output-posix-amd64/reactos-uefi-2015-plus.iso

# Verificar configuración
./scripts/verify-uefi-usb.sh /dev/sdb

# Probar en QEMU (opcional)
./scripts/test-uefi-usb-qemu.sh /dev/sdb
```

## 📋 **Requisitos**

- **Sistema Linux** (Ubuntu/Debian recomendado)
- **USB de 4GB+** (formateado)
- **ISO de ReactOS UEFI** (`reactos-uefi-2015-plus.iso`)
- **Privilegios de root** para particionado

## 🔧 **Uso**

### **1. Preparar USB**
```bash
# Insertar USB y verificar dispositivo
lsblk

# Instalar ReactOS UEFI
sudo make install USB=/dev/sdb ISO=reactos-uefi-2015-plus.iso
```

### **2. Verificar instalación**
```bash
# Verificar configuración
make verify USB=/dev/sdb

# Probar en QEMU (opcional)
make test USB=/dev/sdb
```

### **3. Bootear en hardware**
1. Insertar USB en puerto USB 3.0
2. Reiniciar y acceder a BIOS (F2/Del)
3. Verificar configuración UEFI (no Legacy)
4. Presionar F8 para menú de arranque
5. Seleccionar "UEFI USB" o "ReactOS UEFI"

## 📁 **Estructura del Proyecto**

```
reactos-uefi-bootloader/
├── scripts/                    # Scripts de instalación
│   ├── quick-install.sh       # Instalación rápida
│   ├── verify-uefi-usb.sh     # Verificación USB
│   └── test-uefi-usb-qemu.sh  # Pruebas en QEMU
├── docs/                       # Documentación
│   ├── SOLUCION-USB-UEFI-ASUS-10GEN.md
│   ├── README-UEFI-BOOTLOADER.md
│   └── README-2-ISOS-PRINCIPALES.md
├── build/                      # Archivos de compilación
├── Makefile                    # Comandos principales
└── README.md                   # Este archivo
```

## 🎯 **Comandos Principales**

```bash
# Ver ayuda
make help

# Instalar en USB
make install USB=/dev/sdb ISO=reactos-uefi-2015-plus.iso

# Verificar configuración
make verify USB=/dev/sdb

# Probar en QEMU
make test USB=/dev/sdb

# Limpiar archivos temporales
make clean

# Ver documentación
make docs
```

## 🔍 **Solución de Problemas**

### **USB no reconocido por BIOS**
1. Verificar configuración UEFI (no Legacy/CSM)
2. Deshabilitar Secure Boot temporalmente
3. Usar puerto USB 3.0
4. Verificar estructura EFI: `make verify USB=/dev/sdb`

### **Error de instalación**
1. Verificar permisos de root
2. Verificar que el USB no esté montado
3. Verificar que el ISO existe
4. Revisar logs de error

## 📚 **Documentación Adicional**

- [Solución Completa ASUS 10ª Gen](docs/SOLUCION-USB-UEFI-ASUS-10GEN.md)
- [Bootloader UEFI Nativo](docs/README-UEFI-BOOTLOADER.md)
- [ISOs Principales](docs/README-2-ISOS-PRINCIPALES.md)

## 🤝 **Contribuir**

1. Fork el repositorio
2. Crear rama para feature (`git checkout -b feature/nueva-funcionalidad`)
3. Commit cambios (`git commit -am 'Agregar nueva funcionalidad'`)
4. Push a la rama (`git push origin feature/nueva-funcionalidad`)
5. Crear Pull Request

## 📄 **Licencia**

Este proyecto está bajo la licencia MIT. Ver [LICENSE](LICENSE) para más detalles.

## 🙏 **Agradecimientos**

- **ReactOS Project** por el sistema operativo
- **UEFI Forum** por las especificaciones UEFI
- **Comunidad Linux** por las herramientas de desarrollo

---

**¡ReactOS ahora funciona perfectamente en sistemas UEFI modernos!** 🎉

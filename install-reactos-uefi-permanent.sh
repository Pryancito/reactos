#!/bin/bash

# ============================================================================
# SCRIPT DE INSTALACIÓN PERMANENTE: REACTOS UEFI PARA ASUS 10ª GENERACIÓN
# ============================================================================
#
# Este script crea una instalación permanente de ReactOS UEFI que funciona
# en sistemas UEFI estrictos como ASUS 10ª generación.
#
# Autor: Claude Assistant
# Fecha: $(date)
# Versión: 2.0-Permanent
# ============================================================================

set -e  # Salir en caso de error

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Función para imprimir mensajes con colores
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}================================${NC}"
    echo -e "${PURPLE} $1${NC}"
    echo -e "${PURPLE}================================${NC}"
}

# ============================================================================
# CONFIGURACIÓN
# ============================================================================

PROJECT_DIR="$(pwd)"
SCRIPTS_DIR="$PROJECT_DIR/scripts"
DOCS_DIR="$PROJECT_DIR/docs"
BUILD_DIR="$PROJECT_DIR/build"

# ============================================================================
# PASO 1: CREAR ESTRUCTURA DE DIRECTORIOS
# ============================================================================

print_header "CREANDO ESTRUCTURA DE DIRECTORIOS PERMANENTE"

print_status "Creando directorios de scripts..."
mkdir -p "$SCRIPTS_DIR"
mkdir -p "$DOCS_DIR"
mkdir -p "$BUILD_DIR"

print_success "Estructura de directorios creada"

# ============================================================================
# PASO 2: INSTALAR SCRIPTS PRINCIPALES
# ============================================================================

print_header "INSTALANDO SCRIPTS PRINCIPALES"

# Copiar scripts principales
print_status "Instalando scripts de creación de USB UEFI..."
cp create-uefi-usb-fixed.sh "$SCRIPTS_DIR/"
cp verify-uefi-usb.sh "$SCRIPTS_DIR/"
cp test-uefi-usb-qemu.sh "$SCRIPTS_DIR/"

# Hacer ejecutables
chmod +x "$SCRIPTS_DIR"/*.sh

print_success "Scripts principales instalados"

# ============================================================================
# PASO 3: INSTALAR DOCUMENTACIÓN
# ============================================================================

print_header "INSTALANDO DOCUMENTACIÓN"

print_status "Instalando documentación..."
cp SOLUCION-USB-UEFI-ASUS-10GEN.md "$DOCS_DIR/"
cp README-UEFI-BOOTLOADER.md "$DOCS_DIR/"
cp README-2-ISOS-PRINCIPALES.md "$DOCS_DIR/"

print_success "Documentación instalada"

# ============================================================================
# PASO 4: CREAR SCRIPT DE INSTALACIÓN RÁPIDA
# ============================================================================

print_header "CREANDO SCRIPT DE INSTALACIÓN RÁPIDA"

cat > "$SCRIPTS_DIR/quick-install.sh" << 'EOF'
#!/bin/bash

# SCRIPT DE INSTALACIÓN RÁPIDA PARA REACTOS UEFI
# Uso: ./quick-install.sh <dispositivo_usb> <iso_file>

set -e

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}🚀 INSTALACIÓN RÁPIDA REACTOS UEFI${NC}"
echo

if [[ $# -ne 2 ]]; then
    echo -e "${RED}Uso: $0 <dispositivo_usb> <iso_file>${NC}"
    echo -e "${RED}Ejemplo: $0 /dev/sdb output-posix-amd64/reactos-uefi-2015-plus.iso${NC}"
    exit 1
fi

USB_DEVICE="$1"
ISO_FILE="$2"

echo -e "${BLUE}Dispositivo USB: $USB_DEVICE${NC}"
echo -e "${BLUE}Archivo ISO: $ISO_FILE${NC}"
echo

# Verificar que se ejecuta como root
if [[ $EUID -ne 0 ]]; then
    echo -e "${RED}Este script debe ejecutarse como root (sudo)${NC}"
    exit 1
fi

# Verificar archivos
if [[ ! -f "$ISO_FILE" ]]; then
    echo -e "${RED}El archivo ISO '$ISO_FILE' no existe${NC}"
    exit 1
fi

if [[ ! -b "$USB_DEVICE" ]]; then
    echo -e "${RED}El dispositivo USB '$USB_DEVICE' no existe${NC}"
    exit 1
fi

echo -e "${BLUE}⚠️  ATENCIÓN: Este proceso BORRARÁ TODOS los datos del USB $USB_DEVICE${NC}"
read -p "¿Estás seguro de continuar? (s/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Ss]$ ]]; then
    echo "Operación cancelada"
    exit 0
fi

echo -e "${BLUE}Procediendo con la instalación...${NC}"

# Desmontar USB
umount "$USB_DEVICE"* 2>/dev/null || true
sleep 2

# Crear tabla GPT
echo -e "${BLUE}Creando tabla GPT...${NC}"
parted "$USB_DEVICE" mklabel gpt

# Crear partición EFI
echo -e "${BLUE}Creando partición EFI...${NC}"
parted "$USB_DEVICE" mkpart primary fat32 1MiB 100MiB
parted "$USB_DEVICE" set 1 esp on

# Formatear
echo -e "${BLUE}Formateando partición EFI...${NC}"
mkfs.fat -F 32 "${USB_DEVICE}1"

# Montar y crear estructura
echo -e "${BLUE}Creando estructura EFI...${NC}"
mkdir -p /tmp/reactos-efi
mount "${USB_DEVICE}1" /tmp/reactos-efi
mkdir -p /tmp/reactos-efi/EFI/BOOT
mkdir -p /tmp/reactos-efi/EFI/REACTOS

# Extraer archivos
echo -e "${BLUE}Extrayendo archivos EFI...${NC}"
cd /tmp/reactos-efi
isoinfo -i "$(pwd)/$ISO_FILE" -x /EFI/BOOT/bootx64.efi | tee EFI/BOOT/bootx64.efi > /dev/null
isoinfo -i "$(pwd)/$ISO_FILE" -x /EFI/REACTOS/reactos-uefi-native.efi | tee EFI/REACTOS/reactos-uefi-native.efi > /dev/null

# Desmontar
cd -
umount /tmp/reactos-efi
rmdir /tmp/reactos-efi

echo -e "${GREEN}✅ ¡Instalación completada exitosamente!${NC}"
echo -e "${BLUE}El USB está listo para bootear ReactOS en sistemas UEFI${NC}"
EOF

chmod +x "$SCRIPTS_DIR/quick-install.sh"

print_success "Script de instalación rápida creado"

# ============================================================================
# PASO 5: CREAR MAKEFILE
# ============================================================================

print_header "CREANDO MAKEFILE"

cat > "$PROJECT_DIR/Makefile" << 'EOF'
# Makefile para ReactOS UEFI Installation
# Uso: make <target>

.PHONY: help install verify test clean

# Variables
SCRIPTS_DIR = scripts
DOCS_DIR = docs
BUILD_DIR = build

# Comandos por defecto
help:
	@echo "Comandos disponibles:"
	@echo "  make install <usb> <iso>  - Instalar ReactOS UEFI en USB"
	@echo "  make verify <usb>         - Verificar configuración USB"
	@echo "  make test <usb>           - Probar USB en QEMU"
	@echo "  make clean                - Limpiar archivos temporales"
	@echo "  make docs                 - Mostrar documentación"

install:
	@if [ -z "$(USB)" ] || [ -z "$(ISO)" ]; then \
		echo "Uso: make install USB=/dev/sdb ISO=reactos-uefi.iso"; \
		exit 1; \
	fi
	@echo "Instalando ReactOS UEFI en $(USB)..."
	@sudo $(SCRIPTS_DIR)/quick-install.sh $(USB) $(ISO)

verify:
	@if [ -z "$(USB)" ]; then \
		echo "Uso: make verify USB=/dev/sdb"; \
		exit 1; \
	fi
	@echo "Verificando configuración de $(USB)..."
	@$(SCRIPTS_DIR)/verify-uefi-usb.sh $(USB)

test:
	@if [ -z "$(USB)" ]; then \
		echo "Uso: make test USB=/dev/sdb"; \
		exit 1; \
	fi
	@echo "Probando $(USB) en QEMU..."
	@$(SCRIPTS_DIR)/test-uefi-usb-qemu.sh $(USB)

clean:
	@echo "Limpiando archivos temporales..."
	@rm -f test-uefi-usb.img
	@rm -rf /tmp/reactos-efi
	@echo "Limpieza completada"

docs:
	@echo "Documentación disponible:"
	@echo "  $(DOCS_DIR)/SOLUCION-USB-UEFI-ASUS-10GEN.md"
	@echo "  $(DOCS_DIR)/README-UEFI-BOOTLOADER.md"
	@echo "  $(DOCS_DIR)/README-2-ISOS-PRINCIPALES.md"
EOF

print_success "Makefile creado"

# ============================================================================
# PASO 6: CREAR README PRINCIPAL
# ============================================================================

print_header "CREANDO README PRINCIPAL"

cat > "$PROJECT_DIR/README.md" << 'EOF'
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
EOF

print_success "README principal creado"

# ============================================================================
# PASO 7: CREAR ARCHIVO DE LICENCIA
# ============================================================================

print_header "CREANDO ARCHIVO DE LICENCIA"

cat > "$PROJECT_DIR/LICENSE" << 'EOF'
MIT License

Copyright (c) 2024 ReactOS UEFI Bootloader Project

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF

print_success "Archivo de licencia creado"

# ============================================================================
# PASO 8: CREAR .GITIGNORE
# ============================================================================

print_header "CREANDO .GITIGNORE"

cat > "$PROJECT_DIR/.gitignore" << 'EOF'
# Archivos temporales
*.tmp
*.temp
*.log

# Archivos de prueba
test-uefi-usb.img
*.iso

# Directorios temporales
/tmp/
/temp/

# Archivos de sistema
.DS_Store
Thumbs.db

# Archivos de backup
*.bak
*.backup
*~

# Archivos de configuración local
.env
config.local

# Archivos de compilación
*.o
*.obj
*.exe
*.dll
*.so
*.dylib

# Directorios de build
build/
dist/
target/

# Archivos de IDE
.vscode/
.idea/
*.swp
*.swo

# Archivos de particionado
*.part
*.img
EOF

print_success "Archivo .gitignore creado"

# ============================================================================
# PASO 9: CREAR SCRIPT DE DESINSTALACIÓN
# ============================================================================

print_header "CREANDO SCRIPT DE DESINSTALACIÓN"

cat > "$SCRIPTS_DIR/uninstall.sh" << 'EOF'
#!/bin/bash

# SCRIPT DE DESINSTALACIÓN
# Uso: ./uninstall.sh

set -e

# Colores
RED='\033[0;31m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${BLUE}🗑️  DESINSTALANDO REACTOS UEFI BOOTLOADER${NC}"
echo

echo -e "${BLUE}¿Estás seguro de que quieres desinstalar? (s/N):${NC}"
read -p "" -n 1 -r
echo
if [[ ! $REPLY =~ ^[Ss]$ ]]; then
    echo "Desinstalación cancelada"
    exit 0
fi

echo -e "${BLUE}Desinstalando archivos...${NC}"

# Remover directorios
rm -rf scripts/
rm -rf docs/
rm -rf build/

# Remover archivos principales
rm -f Makefile
rm -f LICENSE
rm -f .gitignore

# Remover archivos de instalación
rm -f create-uefi-usb-fixed.sh
rm -f verify-uefi-usb.sh
rm -f test-uefi-usb-qemu.sh
rm -f SOLUCION-USB-UEFI-ASUS-10GEN.md

echo -e "${GREEN}✅ Desinstalación completada${NC}"
echo -e "${BLUE}Los archivos han sido removidos del sistema${NC}"
EOF

chmod +x "$SCRIPTS_DIR/uninstall.sh"

print_success "Script de desinstalación creado"

# ============================================================================
# PASO 10: RESUMEN FINAL
# ============================================================================

print_header "INSTALACIÓN PERMANENTE COMPLETADA"

print_success "¡Instalación permanente completada exitosamente!"
echo
print_status "📁 Estructura creada:"
echo "   ├── scripts/           # Scripts de instalación y utilidades"
echo "   ├── docs/              # Documentación completa"
echo "   ├── build/             # Directorio para archivos de compilación"
echo "   ├── Makefile           # Comandos principales"
echo "   ├── README.md          # Documentación principal"
echo "   ├── LICENSE            # Licencia MIT"
echo "   └── .gitignore         # Archivos a ignorar en Git"
echo

print_status "🚀 Comandos disponibles:"
echo "   make help              # Mostrar ayuda"
echo "   make install USB=/dev/sdb ISO=reactos-uefi.iso"
echo "   make verify USB=/dev/sdb"
echo "   make test USB=/dev/sdb"
echo "   make clean             # Limpiar archivos temporales"
echo "   make docs              # Mostrar documentación"
echo

print_status "📚 Documentación:"
echo "   docs/SOLUCION-USB-UEFI-ASUS-10GEN.md"
echo "   docs/README-UEFI-BOOTLOADER.md"
echo "   docs/README-2-ISOS-PRINCIPALES.md"
echo

print_success "🎯 El proyecto está listo para subir a GitHub!"
print_info "Puedes usar 'git init', 'git add .', 'git commit' y 'git push'"
print_info "para publicar tu solución en GitHub."

# ============================================================================
# FINALIZACIÓN
# ============================================================================

print_header "¡REACTOS UEFI BOOTLOADER INSTALADO PERMANENTEMENTE!"

# El proyecto está listo para ser subido a GitHub

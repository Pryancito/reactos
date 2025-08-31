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

.PHONY: help install verify test clean docs

# ... existing code ...

docs:
	@echo "Documentación disponible:"
	@echo "  docs/SOLUCION-USB-UEFI-ASUS-10GEN.md"
	@echo "  docs/README-UEFI-BOOTLOADER.md"
	@echo "  docs/README-2-ISOS-PRINCIPALES.md"

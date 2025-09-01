# Makefile para ReactOS UEFI Installation
# Uso: make <target>

.PHONY: help install verify test clean iso-uefi iso-legacy iso-both install-uefi install-legacy burn-uefi burn-legacy

# Variables
SCRIPTS_DIR = scripts
DOCS_DIR = docs
BUILD_DIR = build
REACTOS_BUILD_DIR = output-posix-amd64
TEMP_DIR = /tmp/reactos-iso-build

# Comandos por defecto
help:
	@echo "Comandos disponibles:"
	@echo ""
	@echo "🎯 GENERACIÓN DE ISOs:"
	@echo "  make iso-uefi              - Crear ISO UEFI (post-2015)"
	@echo "  make iso-legacy            - Crear ISO Legacy (pre-2015)"
	@echo "  make iso-both              - Crear ambas ISOs"
	@echo ""
	@echo "🚀 INSTALACIÓN EN USB:"
	@echo "  make install-uefi USB=/dev/sdb    - Instalar ISO UEFI en USB"
	@echo "  make install-legacy USB=/dev/sdb  - Instalar ISO Legacy en USB"
	@echo "  make install USB=/dev/sdb ISO=iso - Instalar ISO específica"
	@echo "  make install-complete USB=/dev/sdb - Instalar ReactOS completo en USB"
	@echo "  make burn-uefi USB=/dev/sdb       - Grabar ISO UEFI en USB (dd)"
	@echo "  make burn-legacy USB=/dev/sdb     - Grabar ISO Legacy en USB (dd)"
	@echo ""
	@echo "🔍 VERIFICACIÓN Y TESTING:"
	@echo "  make verify USB=/dev/sdb          - Verificar configuración USB"
	@echo "  make verify-complete USB=/dev/sdb - Verificar instalación completa"
	@echo "  make test USB=/dev/sdb            - Probar USB en QEMU"
	@echo "  make test-iso-uefi                - Probar ISO UEFI en QEMU"
	@echo "  make test-iso-legacy              - Probar ISO Legacy en QEMU"
	@echo ""
	@echo "🧹 MANTENIMIENTO:"
	@echo "  make clean                        - Limpiar archivos temporales"
	@echo "  make clean-isos                   - Limpiar ISOs generadas"
	@echo "  make docs                         - Mostrar documentación"

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

verify-complete:
	@if [ -z "$(USB)" ]; then \
		echo "Uso: make verify-complete USB=/dev/sdb"; \
		exit 1; \
	fi
	@echo "Verificando instalación completa de $(USB)..."
	@$(SCRIPTS_DIR)/verify-complete-usb.sh $(USB)

test:
	@if [ -z "$(USB)" ]; then \
		echo "Uso: make test USB=/dev/sdb"; \
		exit 1; \
	fi
	@echo "Probando $(USB) en QEMU..."
	@$(SCRIPTS_DIR)/test-uefi-usb-qemu.sh $(USB)

# ============================================================================
# GENERACIÓN DE ISOs
# ============================================================================

iso-uefi: check-dependencies
	@echo "🎯 Generando ISO UEFI (post-2015)..."
	@if [ -f "reactos-complete-uefi.iso" ]; then \
		cp reactos-complete-uefi.iso reactos-uefi-modern.iso; \
		echo "✅ ISO UEFI copiada: reactos-uefi-modern.iso"; \
	else \
		echo "❌ Error: reactos-complete-uefi.iso no encontrada"; \
		echo "💡 Ejecuta primero: ./create-complete-reactos-uefi-iso.sh"; \
		exit 1; \
	fi

iso-legacy: check-dependencies
	@echo "🎯 Generando ISO Legacy (pre-2015)..."
	@if [ -f "create-legacy-iso.sh" ]; then \
		./create-legacy-iso.sh; \
	else \
		echo "❌ Error: create-legacy-iso.sh no encontrado"; \
		exit 1; \
	fi

iso-both: iso-uefi iso-legacy
	@echo "🎉 Ambas ISOs generadas exitosamente!"
	@echo "📁 reactos-uefi-modern.iso - Para sistemas UEFI (post-2015)"
	@echo "📁 reactos-legacy-classic.iso - Para sistemas Legacy (pre-2015)"

# ============================================================================
# INSTALACIÓN EN USB
# ============================================================================

install-uefi:
	@if [ -z "$(USB)" ]; then \
		echo "Uso: make install-uefi USB=/dev/sdb"; \
		exit 1; \
	fi
	@echo "🚀 Instalando ISO UEFI en $(USB)..."
	@if [ ! -f "reactos-uefi-modern.iso" ]; then \
		echo "❌ ISO UEFI no encontrada. Ejecuta: make iso-uefi"; \
		exit 1; \
	fi
	@sudo dd if=reactos-uefi-modern.iso of=$(USB) bs=4M status=progress conv=fdatasync
	@echo "✅ ISO UEFI instalada en $(USB)"

install-complete:
	@if [ -z "$(USB)" ]; then \
		echo "Uso: make install-complete USB=/dev/sdb"; \
		exit 1; \
	fi
	@echo "🚀 Instalando ReactOS completo en $(USB)..."
	@if [ ! -f "reactos-uefi-modern.iso" ]; then \
		echo "❌ ISO UEFI no encontrada. Ejecuta: make iso-uefi"; \
		exit 1; \
	fi
	@sudo $(SCRIPTS_DIR)/complete-install.sh $(USB) $(PWD)/reactos-uefi-modern.iso
	@echo "✅ ReactOS completo instalado en $(USB)"

install-legacy:
	@if [ -z "$(USB)" ]; then \
		echo "Uso: make install-legacy USB=/dev/sdb"; \
		exit 1; \
	fi
	@echo "🚀 Instalando ISO Legacy en $(USB)..."
	@if [ ! -f "reactos-legacy-classic.iso" ]; then \
		echo "❌ ISO Legacy no encontrada. Ejecuta: make iso-legacy"; \
		exit 1; \
	fi
	@sudo dd if=reactos-legacy-classic.iso of=$(USB) bs=4M status=progress conv=fdatasync
	@echo "✅ ISO Legacy instalada en $(USB)"

# ============================================================================
# GRABACIÓN EN USB CON DIFERENTES MÉTODOS
# ============================================================================

burn-uefi:
	@if [ -z "$(USB)" ]; then \
		echo "Uso: make burn-uefi USB=/dev/sdb"; \
		exit 1; \
	fi
	@echo "🔥 Grabando ISO UEFI en $(USB)..."
	@if [ ! -f "reactos-uefi-modern.iso" ]; then \
		echo "❌ ISO UEFI no encontrada. Ejecuta: make iso-uefi"; \
		exit 1; \
	fi
	@./burn-iso-to-usb.sh reactos-uefi-modern.iso $(USB) dd

burn-legacy:
	@if [ -z "$(USB)" ]; then \
		echo "Uso: make burn-legacy USB=/dev/sdb"; \
		exit 1; \
	fi
	@echo "🔥 Grabando ISO Legacy en $(USB)..."
	@if [ ! -f "reactos-legacy-classic.iso" ]; then \
		echo "❌ ISO Legacy no encontrada. Ejecuta: make iso-legacy"; \
		exit 1; \
	fi
	@./burn-iso-to-usb.sh reactos-legacy-classic.iso $(USB) dd

# ============================================================================
# TESTING EN QEMU
# ============================================================================

test-iso-uefi:
	@if [ ! -f "reactos-uefi-modern.iso" ]; then \
		echo "❌ ISO UEFI no encontrada. Ejecuta: make iso-uefi"; \
		exit 1; \
	fi
	@echo "🧪 Probando ISO UEFI en QEMU..."
	@qemu-img create -f qcow2 test-uefi.img 10G
	@qemu-system-x86_64 -m 2G -enable-kvm -drive file=test-uefi.img,format=qcow2 -cdrom reactos-uefi-modern.iso -bios /usr/share/ovmf/OVMF.fd -display gtk -vga std
	@rm -f test-uefi.img

test-iso-legacy:
	@if [ ! -f "reactos-legacy-classic.iso" ]; then \
		echo "❌ ISO Legacy no encontrada. Ejecuta: make iso-legacy"; \
		exit 1; \
	fi
	@echo "🧪 Probando ISO Legacy en QEMU..."
	@qemu-img create -f qcow2 test-legacy.img 10G
	@qemu-system-x86_64 -m 2G -enable-kvm -drive file=test-legacy.img,format=qcow2 -cdrom reactos-legacy-classic.iso -display gtk -vga std
	@rm -f test-legacy.img

# ============================================================================
# FUNCIONES AUXILIARES
# ============================================================================

define create-iso
	@if command -v mkisofs >/dev/null 2>&1; then \
		mkisofs -o $(1) \
			-b boot/freeldr/freeldr/freeldr.sys \
			-no-emul-boot \
			-boot-load-size 4 \
			-boot-info-table \
			-eltorito-alt-boot \
			-e EFI/BOOT/bootx64.efi \
			-no-emul-boot \
			-iso-level 4 \
			-J -joliet-long \
			-D -N \
			-relaxed-filenames \
			-V "$(2)" \
			-A "$(3)" \
			-publisher "ReactOS Project" \
			-p "ReactOS Project" \
			.; \
	elif command -v genisoimage >/dev/null 2>&1; then \
		genisoimage -o $(1) \
			-b boot/freeldr/freeldr/freeldr.sys \
			-no-emul-boot \
			-boot-load-size 4 \
			-boot-info-table \
			-eltorito-alt-boot \
			-e EFI/BOOT/bootx64.efi \
			-no-emul-boot \
			-iso-level 4 \
			-J -joliet-long \
			-D -N \
			-relaxed-filenames \
			-V "$(2)" \
			-A "$(3)" \
			-publisher "ReactOS Project" \
			-p "ReactOS Project" \
			.; \
	else \
		echo "❌ Error: mkisofs o genisoimage no encontrados"; \
		exit 1; \
	fi
endef

define create-legacy-iso
	@if command -v mkisofs >/dev/null 2>&1; then \
		mkisofs -o $(1) \
			-b boot/freeldr/freeldr.sys \
			-no-emul-boot \
			-boot-load-size 4 \
			-boot-info-table \
			-iso-level 4 \
			-J -joliet-long \
			-D -N \
			-relaxed-filenames \
			-V "$(2)" \
			-A "$(3)" \
			-publisher "ReactOS Project" \
			-p "ReactOS Project" \
			.; \
	elif command -v genisoimage >/dev/null 2>&1; then \
		genisoimage -o $(1) \
			-b boot/freeldr/freeldr.sys \
			-no-emul-boot \
			-boot-load-size 4 \
			-boot-info-table \
			-iso-level 4 \
			-J -joliet-long \
			-D -N \
			-relaxed-filenames \
			-V "$(2)" \
			-A "$(3)" \
			-publisher "ReactOS Project" \
			-p "ReactOS Project" \
			.; \
	else \
		echo "❌ Error: mkisofs o genisoimage no encontrados"; \
		exit 1; \
	fi
endef

check-dependencies:
	@if [ ! -d "$(REACTOS_BUILD_DIR)" ]; then \
		echo "❌ Error: Directorio de build no encontrado: $(REACTOS_BUILD_DIR)"; \
		echo "💡 Ejecuta primero: ./compile-reactos-posix.sh"; \
		exit 1; \
	fi
	@if ! command -v mkisofs >/dev/null 2>&1 && ! command -v genisoimage >/dev/null 2>&1; then \
		echo "❌ Error: mkisofs o genisoimage no encontrados"; \
		echo "💡 Instala: sudo apt-get install genisoimage"; \
		exit 1; \
	fi

# ============================================================================
# LIMPIEZA
# ============================================================================

clean:
	@echo "🧹 Limpiando archivos temporales..."
	@rm -f test-uefi-usb.img test-uefi.img test-legacy.img
	@rm -rf /tmp/reactos-efi $(TEMP_DIR)
	@echo "✅ Limpieza completada"

clean-isos:
	@echo "🧹 Limpiando ISOs generadas..."
	@rm -f reactos-uefi-modern.iso reactos-legacy-classic.iso reactos-complete-uefi.iso
	@echo "✅ ISOs eliminadas"

# ============================================================================
# DOCUMENTACIÓN
# ============================================================================

docs:
	@echo "📚 Documentación disponible:"
	@echo "  docs/SOLUCION-USB-UEFI-ASUS-10GEN.md"
	@echo "  docs/README-UEFI-BOOTLOADER.md"
	@echo "  docs/README-2-ISOS-PRINCIPALES.md"
	@echo "  RESUMEN-ISO-COMPLETA.md"

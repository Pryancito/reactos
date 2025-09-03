# Makefile para compilar las interfaces Rust/C integradas

# Configuración
RUST_TOOLCHAIN = nightly
TARGET = x86_64-unknown-none
KERNEL_DIR = ../kernel
REACTOS_DIR = ../ntoskrnl

# Compilador Rust
RUSTC = rustc +$(RUST_TOOLCHAIN)

# Compilador C
CC = gcc
CFLAGS = -Wall -Wextra -std=c99 -fno-stack-protector -nostdlib -ffreestanding

# Archivos Rust
RUST_FILES = cache_system/cache_implementation.rs \
             security_system/security_implementation.rs \
             scheduler_system/scheduler_implementation.rs

# Archivos C
C_FILES = cache_system/cache_interface.c \
          security_system/security_interface.c \
          scheduler_system/scheduler_interface.c

# Archivos objeto
RUST_OBJECTS = $(RUST_FILES:.rs=.o)
C_OBJECTS = $(C_FILES:.c=.o)

# Objetivo principal
all: rust_objects c_objects

# Compilar archivos Rust
rust_objects: $(RUST_OBJECTS)

%.o: %.rs
	$(RUSTC) --target $(TARGET) --crate-type staticlib -o $@ $<

# Compilar archivos C
c_objects: $(C_OBJECTS)

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

# Limpiar archivos generados
clean:
	rm -f $(RUST_OBJECTS) $(C_OBJECTS)

# Instalar en ReactOS
install: all
	@echo "Instalando interfaces en ReactOS..."
	@echo "TODO: Implementar instalación en ReactOS"

# Probar interfaces
test: all
	@echo "Ejecutando pruebas de interfaces..."
	@echo "TODO: Implementar pruebas"

.PHONY: all clean install test

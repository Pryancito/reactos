# ReactOS UEFI Bootloader

Bootloader UEFI moderno en Rust para ReactOS Rust OS, compatible con sistemas UEFI 2.8+ y arquitectura x86_64.

## 🚀 Características

- **Compatible con UEFI 2.8+**: Soporte completo para sistemas UEFI modernos
- **Arquitectura x86_64**: Optimizado para procesadores de 64 bits
- **Escrito en Rust**: Seguridad de memoria y rendimiento optimizado
- **Detección automática**: Hardware y configuración del sistema
- **Carga del kernel**: Transferencia segura al kernel ReactOS Rust
- **Configuración avanzada**: Paginación, interrupciones y entorno del kernel

## 📋 Requisitos

### Sistema
- Sistema UEFI compatible (no Legacy BIOS)
- Procesador x86_64
- Mínimo 512MB RAM
- Partición EFI configurada

### Herramientas de desarrollo
- Rust 1.70+ con target UEFI
- Cargo
- xorriso (para crear ISO)
- QEMU (para pruebas)

## 🔧 Instalación

### 1. Instalar target UEFI
```bash
rustup target add x86_64-unknown-uefi
```

### 2. Compilar el bootloader
```bash
cd bootloader
cargo build --release --target x86_64-unknown-uefi
```

### 3. Usar script de build automático
```bash
./scripts/build-uefi-bootloader.sh
```

## 📁 Estructura del proyecto

```
bootloader/
├── src/
│   ├── uefi_bootloader.rs    # Bootloader UEFI principal
│   ├── main.rs              # Punto de entrada
│   └── main_simple.rs       # Versión simplificada
├── .cargo/
│   └── config.toml          # Configuración de Cargo
├── x86_64-unknown-uefi.json # Target UEFI personalizado
├── Cargo.toml               # Dependencias del proyecto
└── README.md                # Este archivo
```

## 🏗️ Arquitectura

### Componentes principales

1. **Inicialización UEFI**: Configuración del entorno UEFI
2. **Detección de hardware**: CPU, memoria, dispositivos
3. **Carga del kernel**: Lectura y validación del kernel
4. **Configuración del sistema**: Paginación, IDT, GDT
5. **Transferencia de control**: Salida de boot services y salto al kernel

### Flujo de ejecución

```
UEFI Firmware
     ↓
efi_main()
     ↓
bootloader_init()
     ↓
display_system_info()
     ↓
detect_hardware()
     ↓
load_kernel()
     ↓
setup_kernel_environment()
     ↓
exit_boot_services_and_jump_to_kernel()
     ↓
Kernel ReactOS Rust
```

## 🔨 Build y desarrollo

### Build manual
```bash
# Configurar variables de entorno
export TARGET="x86_64-unknown-uefi"
export RUSTFLAGS="-C target-feature=+crt-static"

# Compilar
cargo build --release --target $TARGET
```

### Build con script
```bash
# Script completo con ISO
./scripts/build-uefi-bootloader.sh
```

### Desarrollo
```bash
# Build de desarrollo
cargo build --target x86_64-unknown-uefi

# Tests (si están implementados)
cargo test --target x86_64-unknown-uefi
```

## 🧪 Pruebas

### QEMU con OVMF
```bash
# Instalar OVMF
sudo apt install ovmf

# Ejecutar en QEMU
qemu-system-x86_64 \
  -bios /usr/share/ovmf/OVMF.fd \
  -cdrom build/iso/reactos-rust-uefi.iso \
  -m 2G \
  -cpu qemu64
```

### Hardware real
1. Crear USB booteable con la ISO
2. Configurar firmware UEFI para bootear desde USB
3. Seleccionar ReactOS en el menú de boot

## 📊 Optimizaciones

### Tamaño del binario
- **LTO (Link Time Optimization)**: Optimización de enlazado
- **Codegen units = 1**: Mejor optimización
- **Strip symbols**: Eliminación de símbolos de debug
- **Opt-level = z**: Optimización de tamaño

### Rendimiento
- **Target features**: +crt-static para mejor rendimiento
- **Relocation model**: static para direcciones fijas
- **Code model**: kernel para código del kernel

## 🔧 Configuración

### Variables de entorno
```bash
# Target UEFI
export TARGET="x86_64-unknown-uefi"

# Flags de Rust
export RUSTFLAGS="-C target-feature=+crt-static -C opt-level=z"

# Configuración del linker
export RUST_LINKER="rust-lld"
```

### Configuración de Cargo
El archivo `.cargo/config.toml` contiene:
- Configuración del linker LLD
- Optimizaciones específicas para UEFI
- Configuración de perfiles de build

## 🐛 Debugging

### Logs del bootloader
El bootloader muestra información en la consola UEFI:
- Información del sistema
- Detección de hardware
- Progreso de carga del kernel
- Errores y warnings

### Debug con QEMU
```bash
# QEMU con debug
qemu-system-x86_64 \
  -bios /usr/share/ovmf/OVMF.fd \
  -cdrom build/iso/reactos-rust-uefi.iso \
  -serial stdio \
  -monitor stdio
```

## 📚 Referencias

- [UEFI Specification 2.8](https://uefi.org/specifications)
- [Rust UEFI Book](https://os.phil-opp.com/uefi-application/)
- [UEFI Programming Guide](https://edk2-docs.gitbook.io/edk-ii-uefi-driver-writer-s-guide/)
- [ReactOS Documentation](https://reactos.org/wiki/)

## 🤝 Contribución

1. Fork del repositorio
2. Crear rama de feature
3. Implementar cambios
4. Tests y validación
5. Pull request

## 📄 Licencia

GPL-2.0 - Ver archivo LICENSE para más detalles.

## 🆘 Soporte

- **Issues**: GitHub Issues
- **Discusiones**: GitHub Discussions
- **Documentación**: Wiki del proyecto
- **Chat**: Discord/Matrix del proyecto

---

**Nota**: Este bootloader está en desarrollo activo. Para uso en producción, consultar la documentación oficial y realizar pruebas exhaustivas.

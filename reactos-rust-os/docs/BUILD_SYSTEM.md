# Sistema de Build Optimizado para ReactOS Rust OS

Este documento describe el sistema de build optimizado que permite compilar automáticamente para múltiples arquitecturas.

## Características Principales

### ✅ **Soporte Multi-Arquitectura**
- **x86_64-unknown-linux-gnu**: Arquitectura nativa 64-bit (Linux)
- **i686-unknown-linux-gnu**: Arquitectura compatible 32-bit (Linux)  
- **x86_64-unknown-uefi**: Bootloader UEFI 64-bit

### ✅ **Optimizaciones de Build**
- Compilación con un solo job para evitar problemas de concurrencia
- Modo release optimizado por defecto
- Soporte para modo debug
- Configuración automática de targets

### ✅ **Herramientas Integradas**
- Instalación automática de targets
- Verificación de dependencias
- Compilación paralela para múltiples arquitecturas
- Sistema de pruebas integrado
- Limpieza automática de builds

## Uso del Sistema de Build

### Comandos Principales

```bash
# Mostrar información del sistema
./scripts/build-system.sh info

# Instalar targets necesarios
./scripts/build-system.sh install-targets

# Verificar targets instalados
./scripts/build-system.sh check-targets

# Compilar para todas las arquitecturas
./scripts/build-system.sh build-all

# Compilar para arquitectura específica
./scripts/build-system.sh build-native
./scripts/build-system.sh build-32bit
./scripts/build-system.sh build-uefi

# Ejecutar pruebas
./scripts/build-system.sh test-all

# Limpiar builds
./scripts/build-system.sh clean-all
```

### Opciones Avanzadas

```bash
# Usar modo debug
./scripts/build-system.sh build-all --debug

# Usar múltiples jobs paralelos
./scripts/build-system.sh build-all --jobs 4

# Combinar opciones
./scripts/build-system.sh build-native --debug --jobs 2
```

## Configuración del Sistema

### Archivo de Configuración Cargo

El archivo `.cargo/config.toml` contiene la configuración optimizada:

```toml
[build]
jobs = 1

[target.x86_64-unknown-linux-gnu]
linker = "gcc"
rustflags = [
    "-C", "target-cpu=native",
    "-C", "target-feature=+crt-static",
    "-C", "link-arg=-static",
    "-C", "link-arg=-nostdlib",
    "-C", "link-arg=-Wl,--build-id=none",
    "-C", "link-arg=-Wl,--strip-all"
]

[target.i686-unknown-linux-gnu]
linker = "gcc"
rustflags = [
    "-C", "target-cpu=generic",
    "-C", "target-feature=+crt-static",
    "-C", "link-arg=-static",
    "-C", "link-arg=-nostdlib",
    "-C", "link-arg=-Wl,--build-id=none",
    "-C", "link-arg=-Wl,--strip-all"
]

[target.x86_64-unknown-uefi]
linker = "rust-lld"
rustflags = [
    "-C", "link-arg=-T", "linker.ld",
    "-C", "link-arg=-nostdlib",
    "-C", "link-arg=-Wl,--build-id=none",
    "-C", "link-arg=-Wl,--strip-all"
]
```

### Alias de Cargo

El archivo `Cargo.toml` del workspace incluye alias útiles:

```toml
[alias]
build-native = "build --target x86_64-unknown-linux-gnu --release"
build-32bit = "build --target i686-unknown-linux-gnu --release"
build-uefi = "build --target x86_64-unknown-uefi --release"
build-all = "run --package reactos-rust-build --bin reactos-rust-build -- build-all"
test-all = "run --package reactos-rust-build --bin reactos-rust-build -- test-all"
clean-all = "run --package reactos-rust-build --bin reactos-rust-build -- clean-all"
install-targets = "run --package reactos-rust-build --bin reactos-rust-build -- install-targets"
```

## Flujo de Trabajo Recomendado

### 1. Configuración Inicial

```bash
# Instalar targets necesarios
./scripts/build-system.sh install-targets

# Verificar instalación
./scripts/build-system.sh check-targets
```

### 2. Desarrollo Diario

```bash
# Compilar para arquitectura nativa (más rápido)
./scripts/build-system.sh build-native

# Ejecutar pruebas
./scripts/build-system.sh test-all
```

### 3. Build Completo

```bash
# Compilar para todas las arquitecturas
./scripts/build-system.sh build-all

# Verificar resultados
./scripts/build-system.sh info
```

### 4. Limpieza

```bash
# Limpiar builds cuando sea necesario
./scripts/build-system.sh clean-all
```

## Resultados de Build

### Estructura de Directorios

```
target/
├── x86_64-unknown-linux-gnu/
│   └── release/
│       ├── reactos-rust-kernel
│       ├── reactos-rust-drivers
│       ├── reactos-wow64
│       └── ...
├── i686-unknown-linux-gnu/
│   └── release/
│       ├── reactos-rust-kernel
│       ├── reactos-rust-drivers
│       └── ...
└── x86_64-unknown-uefi/
    └── release/
        ├── reactos-uefi-bootloader.efi
        └── ...
```

### Métricas de Rendimiento

El sistema de build proporciona métricas detalladas:

```
📊 Resumen de compilación:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Total de targets: 3
✅ Exitosos: 3
❌ Fallidos: 0
📈 Tasa de éxito: 100.0%
🎉 ¡Todas las compilaciones exitosas!
```

## Troubleshooting

### Problemas Comunes

1. **Target no instalado**
   ```bash
   # Solución: Instalar targets
   ./scripts/build-system.sh install-targets
   ```

2. **Errores de compilación**
   ```bash
   # Solución: Limpiar y recompilar
   ./scripts/build-system.sh clean-all
   ./scripts/build-system.sh build-all
   ```

3. **Problemas de memoria**
   ```bash
   # Solución: Usar menos jobs paralelos
   ./scripts/build-system.sh build-all --jobs 1
   ```

### Logs y Debugging

- Los logs de compilación se muestran en tiempo real
- Usar `--debug` para compilaciones de desarrollo
- Verificar `target/` para archivos generados

## Integración con CI/CD

### GitHub Actions

```yaml
name: Build ReactOS Rust OS

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    - name: Install targets
      run: ./scripts/build-system.sh install-targets
    - name: Build all architectures
      run: ./scripts/build-system.sh build-all
    - name: Run tests
      run: ./scripts/build-system.sh test-all
```

### Scripts de Automatización

```bash
#!/bin/bash
# Script de CI/CD personalizado

set -e

echo "🚀 Iniciando build automatizado..."

# Instalar dependencias
./scripts/build-system.sh install-targets

# Compilar
./scripts/build-system.sh build-all

# Ejecutar pruebas
./scripts/build-system.sh test-all

# Crear artefactos
./scripts/create-grub-iso.sh

echo "✅ Build automatizado completado"
```

## Contribución

Para contribuir al sistema de build:

1. **Fork del repositorio**
2. **Crear rama para nueva funcionalidad**
3. **Implementar cambios en `scripts/build-system.sh`**
4. **Probar con múltiples arquitecturas**
5. **Crear pull request**

### Estándares de Código

- Usar bash con `set -e` para manejo de errores
- Incluir mensajes informativos con emojis
- Documentar nuevas funcionalidades
- Mantener compatibilidad con versiones anteriores

## Roadmap

### Próximas Características

- [ ] Soporte para targets de Windows (MSVC)
- [ ] Compilación cruzada para ARM
- [ ] Cache de dependencias optimizado
- [ ] Integración con Docker
- [ ] Métricas de rendimiento detalladas
- [ ] Soporte para compilación incremental

### Mejoras Planificadas

- [ ] Paralelización inteligente de builds
- [ ] Detección automática de cambios
- [ ] Optimización de tiempos de compilación
- [ ] Integración con herramientas de profiling

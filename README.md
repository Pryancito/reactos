# 🌙 Eclipse OS en Rust

Sistema operativo Windows-compatible implementado completamente en Rust con kernel nativo, GUI, y shell interactivo.

## ✨ Características

- **Kernel Rust nativo** - Compatible con Multiboot, sin dependencias de `std`
- **Shell interactivo completo** - Más de 50 comandos implementados
- **GUI nativa** - Interfaz gráfica con eframe/egui
- **APIs de Windows** - Compatibilidad con APIs nativas de Windows
- **Sistema de archivos** - Operaciones completas de archivos y directorios
- **Red** - Servicios de red, ping, HTTP, Echo
- **Autenticación** - Sistema de usuarios y grupos
- **Modular** - Arquitectura de plugins extensible

## 🚀 Instalación y Uso

### Requisitos

```bash
# Rust (última versión estable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dependencias del sistema
sudo apt update
sudo apt install grub-pc-bin genisoimage cpio gzip qemu-system-x86
```

### Compilación

```bash
# Clonar el repositorio
git clone https://github.com/tu-usuario/eclipse-os-rust.git
cd eclipse-os-rust

# Compilar y crear ISO
./build.sh
```

### Ejecución

```bash
# Con QEMU
qemu-system-x86_64 -cdrom eclipse-os.iso -m 512M -display gtk

# Con VirtualBox
# Crear nueva VM y seleccionar eclipse-os.iso como CD de arranque

# En hardware real
sudo dd if=eclipse-os.iso of=/dev/sdX bs=4M status=progress
```

## 📁 Estructura del Proyecto

```
eclipse-os-rust/
├── src/
│   ├── main.rs          # Sistema operativo principal
│   ├── kernel.rs        # Kernel Rust no_std
│   ├── gui/             # Interfaz gráfica
│   ├── auth/            # Sistema de autenticación
│   ├── network/         # Servicios de red
│   └── apps/            # Aplicaciones nativas
├── docs/                # Documentación
├── scripts/             # Scripts de utilidad
├── build.sh             # Script de construcción
└── Cargo.toml           # Configuración del proyecto
```

## 🎯 Comandos Disponibles

### Sistema
- `info` - Información del sistema
- `ver` - Versión del sistema
- `date` - Fecha actual
- `time` - Hora actual
- `whoami` - Usuario actual
- `hostname` - Nombre del equipo

### Archivos
- `dir` / `ls` - Listar directorios
- `cd` - Cambiar directorio
- `pwd` - Directorio actual
- `type` / `cat` - Mostrar archivo
- `copy` / `cp` - Copiar archivo
- `mkdir` / `md` - Crear directorio
- `del` / `rm` - Eliminar archivo
- `move` / `mv` - Mover archivo
- `ren` / `rename` - Renombrar archivo

### Red
- `ping` - Hacer ping
- `ipconfig` / `ifconfig` - Configuración de red
- `netstat` - Conexiones de red
- `http` - Servidor HTTP
- `echo` - Servidor Echo

### GUI
- `gui` / `desktop` - Abrir interfaz gráfica
- `notepad` / `edit` - Editor de texto
- `calculator` / `calc` - Calculadora
- `filemanager` / `explorer` - Explorador de archivos
- `taskmanager` / `tasks` - Administrador de tareas

### Windows API
- `getenv` - Variable de entorno
- `setenv` - Establecer variable
- `getpid` - ID del proceso
- `getsysteminfo` - Información del sistema
- `getcomputername` - Nombre del equipo
- `getusername` - Usuario actual
- `getcurrentdirectory` - Directorio actual
- `getsystemtime` - Tiempo del sistema
- `getmemoryinfo` - Información de memoria

### Autenticación
- `login` - Iniciar sesión
- `logout` - Cerrar sesión
- `adduser` / `useradd` - Agregar usuario
- `passwd` - Cambiar contraseña
- `listusers` / `users` - Listar usuarios
- `groups` - Listar grupos

## 🔧 Desarrollo

### Compilar solo el kernel

```bash
cargo build --release --bin eclipse-kernel
```

### Compilar solo el sistema

```bash
cargo build --release --bin eclipse-os
```

### Ejecutar en modo debug

```bash
cargo run --release --bin eclipse-os
```

## 📊 Arquitectura

### Kernel
- **Multiboot compatible** - Carga con GRUB
- **VGA driver** - Consola de texto
- **Serial driver** - Consola por puerto serie
- **VESA/VBE** - Soporte para hardware moderno
- **Fallback VGA** - Compatibilidad con hardware antiguo

### Sistema
- **Plugin architecture** - Sistema modular extensible
- **Windows API compatibility** - APIs nativas de Windows
- **Real file system** - Operaciones reales de archivos
- **Network services** - Servicios de red completos
- **GUI framework** - Interfaz gráfica nativa

## 🐛 Solución de Problemas

### Errores de Hardware Real

Si encuentras estos errores en hardware real:
- `WARNING: no console will be available to OS`
- `error: no suitable video mode found`

El kernel incluye:
- ✅ Driver VGA básico
- ✅ Driver Serial
- ✅ Detección VESA/VBE
- ✅ Fallback VGA

### Problemas de Compilación

```bash
# Limpiar compilaciones anteriores
cargo clean
rm -rf target/ iso/ *.iso

# Recompilar
./build.sh
```

## 🤝 Contribuir

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Ver `LICENSE` para más detalles.

## 🙏 Agradecimientos

- **Rust Community** - Por el excelente lenguaje de programación
- **GRUB** - Por el bootloader
- **QEMU** - Por la emulación
- **egui/eframe** - Por el framework GUI

## 📞 Contacto

- **Proyecto**: [Eclipse OS en Rust](https://github.com/tu-usuario/eclipse-os-rust)
- **Issues**: [GitHub Issues](https://github.com/tu-usuario/eclipse-os-rust/issues)
- **Discusiones**: [GitHub Discussions](https://github.com/tu-usuario/eclipse-os-rust/discussions)

---

**🌙 Eclipse OS en Rust** - Sistema operativo Windows-compatible implementado en Rust



🦀 ReactOS Windows en Rust v0.1.0
==================================

Sistema Operativo Windows completamente funcional implementado en Rust.

CARACTERÍSTICAS:
- Shell interactivo modular y ampliable
- APIs reales de Windows integradas
- Interfaz gráfica funcional
- Sistema de archivos real
- 6 plugins implementados
- 35+ comandos funcionales

PLUGINS DISPONIBLES:
- SystemPlugin: Comandos básicos del sistema
- NetworkPlugin: Comandos de red
- FilePlugin: Navegación de archivos
- WindowsApiPlugin: APIs nativas de Windows
- GuiPlugin: Interfaz gráfica
- FileSystemPlugin: Sistema de archivos real

COMANDOS PRINCIPALES:
- help: Mostrar ayuda
- info: Información del sistema
- gui: Interfaz gráfica
- mkdir: Crear directorio
- find: Buscar archivos
- tree: Estructura de directorios
- ping: Probar conectividad
- getpid: ID del proceso

ARQUITECTURA:
- Kernel: Rust
- GUI: eframe/egui
- APIs: Windows crate
- Sistema de archivos: walkdir
- Compilación: Cargo

INFORMACIÓN TÉCNICA:
- Arquitectura: x86_64
- Versión: 0.1.0
- Estado: Completamente funcional
- Plugins: 6 implementados
- Comandos: 35+ funcionales

¡Disfrute usando ReactOS Windows en Rust!

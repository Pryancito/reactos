# ReactOS WOW64 - Capa de Compatibilidad 32-bit

WOW64 (Windows 32-bit on Windows 64-bit) es una capa de compatibilidad que permite ejecutar aplicaciones 32-bit en un sistema operativo 64-bit nativo.

## 🚀 Características

- **Compatibilidad 32-bit**: Ejecuta aplicaciones 32-bit en sistema 64-bit
- **Mapeo de memoria**: Gestiona el espacio de memoria virtual 32-bit
- **Thunks de API**: Convierte llamadas 32-bit a 64-bit
- **Contexto de ejecución**: Maneja el contexto de CPU 32-bit
- **Carga de PE**: Carga y mapea archivos PE 32-bit

## 📁 Estructura del Proyecto

```
wow64/
├── src/
│   ├── lib.rs          # Biblioteca principal
│   ├── main.rs         # Punto de entrada
│   ├── memory/         # Módulo de memoria
│   ├── thunks/         # Módulo de thunks
│   ├── pe_loader/      # Módulo de carga PE
│   └── context/        # Módulo de contexto
├── Cargo.toml          # Configuración del proyecto
└── README.md           # Este archivo
```

## 🛠️ Módulos

### 1. Memory (`memory/mod.rs`)
- **VirtualSpace**: Espacio de memoria virtual 32-bit
- **Heap32**: Heap para aplicaciones 32-bit
- **Stack32**: Stack para aplicaciones 32-bit
- **Code32**: Espacio de código 32-bit
- **Data32**: Espacio de datos 32-bit

### 2. Thunks (`thunks/mod.rs`)
- **Kernel32Thunks**: Thunks para kernel32.dll
- **NtdllThunks**: Thunks para ntdll.dll
- **User32Thunks**: Thunks para user32.dll
- **Gdi32Thunks**: Thunks para gdi32.dll

### 3. PE Loader (`pe_loader/mod.rs`)
- **PeLoader**: Cargador de archivos PE 32-bit
- **Pe32File**: Estructura de archivo PE 32-bit
- **Section**: Sección de archivo PE
- **Import/Export**: Importaciones y exportaciones

### 4. Context (`context/mod.rs`)
- **Context32**: Contexto de ejecución 32-bit
- **Registros**: Manejo de registros 32-bit
- **Flags**: Manejo de flags del procesador
- **Excepciones**: Manejo de excepciones 32-bit

## 🔧 Uso

### Compilación

```bash
# Compilar WOW64
cargo build --release

# Compilar aplicación 32-bit de prueba
cd apps/test32
cargo build --target i686-pc-windows-gnu --release
```

### Ejecución

```bash
# Ejecutar WOW64 con aplicación 32-bit
./wow64 test32.exe
```

### Script de Prueba

```bash
# Ejecutar script de prueba completo
./scripts/test-wow64.sh
```

## 📋 API Principal

### Crear Instancia WOW64

```rust
use reactos_wow64::Wow64;

let mut wow64 = Wow64::new()?;
```

### Cargar Aplicación 32-bit

```rust
wow64.load_32bit_app("test32.exe")?;
```

### Ejecutar Aplicación 32-bit

```rust
wow64.run_32bit_app()?;
```

## 🎯 Funcionalidades Implementadas

### ✅ Completadas
- Estructura básica de WOW64
- Mapeo de memoria 32-bit
- Contexto de ejecución 32-bit
- Cargador de archivos PE 32-bit
- Thunks básicos para APIs
- Aplicación de prueba 32-bit

### 🚧 En Desarrollo
- Thunks completos para todas las APIs
- Manejo de excepciones avanzado
- Optimizaciones de rendimiento
- Soporte para DLLs 32-bit

### 📋 Pendientes
- Integración con el kernel
- Pruebas de compatibilidad
- Documentación de APIs
- Optimizaciones de memoria

## 🔍 Arquitectura

### Mapeo de Memoria

```
Espacio 32-bit (0x00000000 - 0x7FFFFFFF)
├── 0x00000000 - 0x0000FFFF: NULL
├── 0x00010000 - 0x3FFFFFFF: Código y datos
├── 0x40000000 - 0x7FFFFFFF: Heap y stack
└── 0x80000000 - 0xFFFFFFFF: Sistema (mapeado a 64-bit)
```

### Thunks

Los thunks convierten llamadas 32-bit a 64-bit:

```rust
// Llamada 32-bit
CreateFileA(filename, access, share, security, creation, flags, template)

// Se convierte a llamada 64-bit
CreateFileA_64bit(filename_64, access_64, share_64, security_64, creation_64, flags_64, template_64)
```

## 🧪 Pruebas

### Aplicación de Prueba

La aplicación `test32.exe` prueba:
- Creación de ventanas 32-bit
- Manejo de mensajes
- APIs de Windows 32-bit
- Compatibilidad con WOW64

### Script de Prueba

El script `test-wow64.sh`:
1. Verifica dependencias
2. Instala target 32-bit
3. Compila WOW64
4. Compila aplicación de prueba
5. Ejecuta pruebas
6. Genera reporte

## 📚 Referencias

- [WOW64 Architecture](https://docs.microsoft.com/en-us/windows/win32/winprog64/wow64-implementation-details)
- [PE Format](https://docs.microsoft.com/en-us/windows/win32/debug/pe-format)
- [x86-64 Architecture](https://en.wikipedia.org/wiki/X86-64)

## 🤝 Contribuciones

Para contribuir a WOW64:

1. Fork el repositorio
2. Crea una rama para tu feature
3. Implementa la funcionalidad
4. Agrega pruebas
5. Envía un pull request

## 📄 Licencia

Este proyecto está bajo la licencia GPL-2.0, igual que ReactOS.

## 🆘 Soporte

Si tienes problemas con WOW64:

1. Revisa los logs de compilación
2. Verifica que el target 32-bit esté instalado
3. Consulta la documentación
4. Abre un issue en el repositorio

---

**Nota**: WOW64 está en desarrollo activo. Algunas funcionalidades pueden no estar completamente implementadas.

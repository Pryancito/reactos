# Suite de Pruebas de WOW64

Esta suite de pruebas valida la compatibilidad de la capa WOW64 con aplicaciones Windows 32-bit en ReactOS Rust OS.

## Estructura de Pruebas

### Pruebas de Carga de Aplicaciones
- **Carga de archivos PE 32-bit**: Verifica que se pueden cargar archivos ejecutables de 32-bit
- **Mapeo de memoria 32-bit**: Valida el mapeo correcto del espacio de memoria virtual
- **Cambio de contexto 32-bit**: Verifica la configuración del contexto de ejecución

### Pruebas de APIs
- **APIs de kernel32.dll**: Valida thunks para funciones del kernel
- **APIs de ntdll.dll**: Verifica thunks para funciones nativas
- **APIs de user32.dll**: Valida thunks para funciones de interfaz de usuario
- **APIs de gdi32.dll**: Verifica thunks para funciones de gráficos

### Pruebas de Compatibilidad
- **Compatibilidad 32-bit**: Verifica la inicialización correcta de WOW64
- **Gestión de memoria**: Valida la configuración del heap y stack
- **Threading 32-bit**: Verifica la creación de múltiples contextos

### Pruebas de Rendimiento
- **Rendimiento WOW64**: Mide el tiempo de inicialización y operaciones básicas

## Ejecución de Pruebas

### Ejecutar todas las pruebas
```bash
# Desde el directorio raíz del proyecto
./target/release/test_runner
```

### Ejecutar con opciones
```bash
# Mostrar ayuda
./target/release/test_runner --help

# Mostrar versión
./target/release/test_runner --version

# Ejecutar solo pruebas de integración
./target/release/test_runner --integration
```

### Usar el script de pruebas
```bash
# Ejecutar todas las pruebas del proyecto
./scripts/run-all-tests.sh

# Ejecutar solo pruebas de WOW64
./scripts/run-wow64-tests.sh
```

## Resultados de Pruebas

### Formato de Salida
```
🧪 Iniciando suite de pruebas de integración WOW64...

✅ Carga de archivos PE 32-bit - Archivo PE cargado correctamente
✅ Mapeo de memoria 32-bit - Mapeo de memoria configurado correctamente
✅ Cambio de contexto 32-bit - Contexto 32-bit configurado correctamente
...

📊 Resumen de pruebas:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Total de pruebas: 11
✅ Pasaron: 11
❌ Fallaron: 0
⏭️  Omitidas: 0
🚨 Errores: 0
📈 Tasa de éxito: 100.0%
```

### Interpretación de Resultados
- **✅ Pasaron**: Pruebas que se ejecutaron exitosamente
- **❌ Fallaron**: Pruebas que encontraron problemas
- **⏭️ Omitidas**: Pruebas que no se ejecutaron (no implementadas)
- **🚨 Errores**: Errores críticos durante la ejecución

## Configuración de Pruebas

### Archivos de Prueba
- `test_data/simple32.exe`: Archivo PE 32-bit de prueba
- `integration_tests.rs`: Implementación de las pruebas
- `test_runner.rs`: Ejecutor principal de pruebas

### Dependencias
- `std`: Biblioteca estándar de Rust
- `reactos-wow64`: Módulo principal de WOW64

## Desarrollo de Nuevas Pruebas

### Agregar una nueva prueba
1. Crear función en `IntegrationTestSuite`
2. Llamar `self.run_test()` con nombre y función de prueba
3. Agregar llamada en `run_all_tests()`

### Ejemplo de nueva prueba
```rust
fn test_nueva_funcionalidad(&mut self) {
    self.run_test("Nueva funcionalidad", || {
        // Verificar que la funcionalidad funciona
        let resultado = verificar_funcionalidad();
        
        if resultado {
            Ok("Funcionalidad verificada correctamente".to_string())
        } else {
            Err("Funcionalidad no funciona".to_string())
        }
    });
}
```

## Troubleshooting

### Problemas Comunes

1. **Archivo de prueba no encontrado**
   - Verificar que `test_data/simple32.exe` existe
   - Ejecutar desde el directorio correcto

2. **Errores de compilación**
   - Verificar que todas las dependencias están instaladas
   - Limpiar y recompilar: `cargo clean && cargo build --release`

3. **Pruebas fallan**
   - Revisar los mensajes de error en la salida
   - Verificar la configuración del sistema

### Logs y Debugging
- Las pruebas muestran mensajes detallados de error
- Usar `RUST_LOG=debug` para información adicional
- Revisar el código fuente en `integration_tests.rs`

## Contribución

Para contribuir a las pruebas:

1. Fork del repositorio
2. Crear rama para nueva funcionalidad
3. Implementar pruebas
4. Ejecutar suite completa
5. Crear pull request

### Estándares de Código
- Usar nombres descriptivos para pruebas
- Incluir mensajes de error claros
- Documentar nuevas funcionalidades
- Mantener compatibilidad con versiones anteriores

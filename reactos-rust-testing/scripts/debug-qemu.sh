#!/bin/bash

# Script de debug para ReactOS Rust Kernel con QEMU
# Permite debug remoto via puerto serie

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Función para mostrar ayuda
show_help() {
    echo -e "${CYAN}🐛 ReactOS Rust Kernel - Debug con QEMU${NC}"
    echo "=============================================="
    echo ""
    echo "Uso: $0 [opciones]"
    echo ""
    echo "Opciones:"
    echo "  -h, --help          Mostrar esta ayuda"
    echo "  -d, --debug         Ejecutar con debug remoto"
    echo "  -s, --serial        Solo conectar al puerto serie"
    echo "  -g, --gdb           Ejecutar con GDB"
    echo "  -v, --verbose       Modo verbose"
    echo ""
    echo "Ejemplos:"
    echo "  $0                  # Ejecutar kernel normal"
    echo "  $0 -d               # Ejecutar con debug remoto"
    echo "  $0 -s               # Solo conectar al puerto serie"
    echo "  $0 -g               # Ejecutar con GDB"
    echo ""
}

# Variables por defecto
DEBUG_MODE=false
SERIAL_ONLY=false
GDB_MODE=false
VERBOSE=false
QEMU_PID=""

# Procesar argumentos
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -d|--debug)
            DEBUG_MODE=true
            shift
            ;;
        -s|--serial)
            SERIAL_ONLY=true
            shift
            ;;
        -g|--gdb)
            GDB_MODE=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        *)
            echo -e "${RED}Error: Opción desconocida '$1'${NC}"
            show_help
            exit 1
            ;;
    esac
done

# Función para logging
log() {
    if [ "$VERBOSE" = true ]; then
        echo -e "${BLUE}[DEBUG]${NC} $1"
    fi
}

# Función para limpiar al salir
cleanup() {
    if [ ! -z "$QEMU_PID" ]; then
        echo -e "\n${YELLOW}🛑 Deteniendo QEMU (PID: $QEMU_PID)...${NC}"
        kill $QEMU_PID 2>/dev/null || true
        wait $QEMU_PID 2>/dev/null || true
    fi
    echo -e "${GREEN}✅ Limpieza completada${NC}"
}

# Configurar trap para limpieza
trap cleanup EXIT INT TERM

# Verificar que estamos en el directorio correcto
if [ ! -f "reactos-rust-kernel-testing.iso" ]; then
    echo -e "${RED}❌ Error: No se encontró reactos-rust-kernel-testing.iso${NC}"
    echo "Ejecuta este script desde el directorio reactos-rust-testing"
    exit 1
fi

echo -e "${CYAN}🐛 ReactOS Rust Kernel - Debug con QEMU${NC}"
echo "=============================================="

# Verificar que QEMU está instalado
if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo -e "${RED}❌ Error: QEMU no está instalado${NC}"
    echo "Instala QEMU: sudo apt install qemu-system-x86"
    exit 1
fi

# Verificar que el kernel está compilado
if [ ! -f "../reactos-rust-kernel/target/x86_64-unknown-none/release/reactos-rust-kernel" ]; then
    echo -e "${YELLOW}⚠️  Kernel no encontrado, compilando...${NC}"
    cd ../reactos-rust-kernel
    cargo build --bin reactos-rust-kernel --release --target x86_64-unknown-none
    cd ../reactos-rust-testing
fi

# Crear ISO si no existe
if [ ! -f "reactos-rust-kernel-testing.iso" ]; then
    echo -e "${YELLOW}⚠️  ISO no encontrada, creando...${NC}"
    ./scripts/create-iso.sh
fi

# Función para ejecutar QEMU con debug
run_qemu_debug() {
    echo -e "${GREEN}🚀 Iniciando QEMU con debug remoto...${NC}"
    echo -e "${CYAN}📡 Puerto serie: /tmp/qemu-serial${NC}"
    echo -e "${CYAN}🔌 GDB: localhost:1234${NC}"
    echo -e "${YELLOW}💡 Presiona Ctrl+Alt+Q para salir${NC}"
    echo ""
    
    # Crear named pipe para puerto serie
    mkfifo /tmp/qemu-serial 2>/dev/null || true
    
    # Ejecutar QEMU en background
    qemu-system-x86_64 \
        -cdrom reactos-rust-kernel-testing.iso \
        -m 512M \
        -serial file:/tmp/qemu-serial \
        -s -S \
        -monitor stdio \
        -display gtk \
        -no-reboot \
        -no-shutdown &
    
    QEMU_PID=$!
    log "QEMU iniciado con PID: $QEMU_PID"
    
    # Esperar un poco para que QEMU se inicie
    sleep 2
    
    echo -e "${GREEN}✅ QEMU iniciado correctamente${NC}"
    echo -e "${BLUE}📋 Comandos disponibles:${NC}"
    echo "  - Conectar al puerto serie: screen /tmp/qemu-serial"
    echo "  - Conectar con GDB: gdb -ex 'target remote localhost:1234'"
    echo "  - Ver logs del kernel: tail -f /tmp/qemu-serial"
    echo ""
    
    # Mantener el script corriendo
    echo -e "${YELLOW}⏳ QEMU ejecutándose... Presiona Ctrl+C para detener${NC}"
    wait $QEMU_PID
}

# Función para conectar solo al puerto serie
connect_serial() {
    echo -e "${GREEN}📡 Conectando al puerto serie...${NC}"
    
    if [ ! -p "/tmp/qemu-serial" ]; then
        echo -e "${RED}❌ Error: No hay puerto serie disponible${NC}"
        echo "Ejecuta primero: $0 -d"
        exit 1
    fi
    
    echo -e "${CYAN}💡 Usando 'screen' para conectar al puerto serie${NC}"
    echo -e "${YELLOW}💡 Presiona Ctrl+A, K para salir de screen${NC}"
    echo ""
    
    screen /tmp/qemu-serial
}

# Función para ejecutar con GDB
run_gdb() {
    echo -e "${GREEN}🔧 Iniciando GDB...${NC}"
    
    # Verificar que GDB está instalado
    if ! command -v gdb &> /dev/null; then
        echo -e "${RED}❌ Error: GDB no está instalado${NC}"
        echo "Instala GDB: sudo apt install gdb"
        exit 1
    fi
    
    # Verificar que el kernel está compilado con símbolos de debug
    if [ ! -f "../reactos-rust-kernel/target/x86_64-unknown-none/debug/reactos-rust-kernel" ]; then
        echo -e "${YELLOW}⚠️  Compilando kernel con símbolos de debug...${NC}"
        cd ../reactos-rust-kernel
        cargo build --bin reactos-rust-kernel --target x86_64-unknown-none
        cd ../reactos-rust-testing
    fi
    
    echo -e "${CYAN}📋 Comandos GDB útiles:${NC}"
    echo "  - info registers    # Ver registros"
    echo "  - x/16x 0x100000    # Dump de memoria"
    echo "  - break _start      # Breakpoint en _start"
    echo "  - continue          # Continuar ejecución"
    echo "  - step              # Paso a paso"
    echo ""
    
    # Ejecutar GDB
    gdb -ex "target remote localhost:1234" \
        -ex "symbol-file ../reactos-rust-kernel/target/x86_64-unknown-none/debug/reactos-rust-kernel" \
        -ex "set architecture i386:x86-64" \
        -ex "break _start" \
        -ex "continue"
}

# Función para ejecutar QEMU normal
run_qemu_normal() {
    echo -e "${GREEN}🚀 Iniciando ReactOS Rust Kernel...${NC}"
    echo -e "${YELLOW}💡 Presiona Ctrl+Alt+Q para salir${NC}"
    echo ""
    
    qemu-system-x86_64 \
        -cdrom reactos-rust-kernel-testing.iso \
        -m 512M \
        -display gtk \
        -no-reboot \
        -no-shutdown
}

# Ejecutar según el modo seleccionado
if [ "$SERIAL_ONLY" = true ]; then
    connect_serial
elif [ "$GDB_MODE" = true ]; then
    run_gdb
elif [ "$DEBUG_MODE" = true ]; then
    run_qemu_debug
else
    run_qemu_normal
fi

echo -e "${GREEN}🎉 Debug completado${NC}"

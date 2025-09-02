#!/bin/bash

# Script para detectar dispositivos USB disponibles
# Uso: ./detect-usb-devices.sh

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Detectando Dispositivos USB Disponibles${NC}"
echo "=============================================="
echo ""

# Función para obtener tamaño en formato legible
get_human_size() {
    local size_bytes=$1
    if [ $size_bytes -ge 1073741824 ]; then
        echo "$((size_bytes / 1073741824))GB"
    elif [ $size_bytes -ge 1048576 ]; then
        echo "$((size_bytes / 1048576))MB"
    else
        echo "$((size_bytes / 1024))KB"
    fi
}

# Función para verificar si es USB
is_usb_device() {
    local device=$1
    # Verificar si el dispositivo es USB
    if [ -f "/sys/block/$(basename $device)/device/uevent" ]; then
        local uevent_file="/sys/block/$(basename $device)/device/uevent"
        if grep -q "DEVTYPE=usb_device" "$uevent_file" 2>/dev/null; then
            return 0
        fi
    fi
    
    # Verificar por el path del dispositivo
    if [[ "$device" =~ /dev/sd[b-z]$ ]] && [ ! -e "${device}1" ]; then
        return 0
    fi
    
    return 1
}

# Listar dispositivos de bloque
echo -e "${CYAN}📱 Dispositivos de Almacenamiento Detectados:${NC}"
echo ""

usb_devices=()
device_count=0

# Buscar dispositivos USB
for device in /dev/sd[b-z]; do
    if [ -b "$device" ]; then
        device_name=$(basename "$device")
        device_size=$(lsblk -b -d -o SIZE "$device" 2>/dev/null | tail -1)
        device_model=$(lsblk -d -o MODEL "$device" 2>/dev/null | tail -1)
        device_vendor=$(lsblk -d -o VENDOR "$device" 2>/dev/null | tail -1)
        
        if [ -n "$device_size" ] && [ "$device_size" -gt 0 ]; then
            device_count=$((device_count + 1))
            human_size=$(get_human_size "$device_size")
            
            # Verificar si tiene particiones
            has_partitions=false
            if [ -e "${device}1" ]; then
                has_partitions=true
            fi
            
            # Determinar si es USB
            is_usb=false
            if is_usb_device "$device"; then
                is_usb=true
                usb_devices+=("$device")
            fi
            
            echo -e "${YELLOW}📀 Dispositivo $device_count:${NC}"
            echo -e "   ${BLUE}Ruta:${NC} $device"
            echo -e "   ${BLUE}Tamaño:${NC} $human_size"
            echo -e "   ${BLUE}Modelo:${NC} $device_model"
            echo -e "   ${BLUE}Fabricante:${NC} $device_vendor"
            echo -e "   ${BLUE}Tipo:${NC} $([ "$is_usb" = true ] && echo -e "${GREEN}USB${NC}" || echo -e "${CYAN}Disco${NC}")"
            echo -e "   ${BLUE}Particiones:${NC} $([ "$has_partitions" = true ] && echo -e "${YELLOW}Sí${NC}" || echo -e "${GREEN}No${NC}")"
            
            if [ "$has_partitions" = true ]; then
                echo -e "   ${BLUE}Particiones encontradas:${NC}"
                for partition in ${device}[0-9]*; do
                    if [ -b "$partition" ]; then
                        partition_size=$(lsblk -b -o SIZE "$partition" 2>/dev/null | tail -1)
                        partition_fs=$(lsblk -o FSTYPE "$partition" 2>/dev/null | tail -1)
                        partition_label=$(lsblk -o LABEL "$partition" 2>/dev/null | tail -1)
                        human_part_size=$(get_human_size "$partition_size")
                        
                        echo -e "     • $partition: $human_part_size ($partition_fs) $([ -n "$partition_label" ] && echo "[$partition_label]")"
                    fi
                done
            fi
            echo ""
        fi
    fi
done

if [ $device_count -eq 0 ]; then
    echo -e "${RED}❌ No se encontraron dispositivos USB disponibles${NC}"
    echo ""
    echo -e "${YELLOW}💡 Sugerencias:${NC}"
    echo "• Conecta un dispositivo USB"
    echo "• Verifica que el USB esté funcionando"
    echo "• Ejecuta: sudo dmesg | tail -20 (para ver mensajes del kernel)"
    exit 1
fi

echo -e "${GREEN}✅ Se encontraron $device_count dispositivo(s)${NC}"
echo ""

# Mostrar dispositivos USB específicamente
if [ ${#usb_devices[@]} -gt 0 ]; then
    echo -e "${GREEN}🔌 Dispositivos USB Recomendados:${NC}"
    echo ""
    
    for i in "${!usb_devices[@]}"; do
        device="${usb_devices[$i]}"
        device_size=$(lsblk -b -d -o SIZE "$device" 2>/dev/null | tail -1)
        human_size=$(get_human_size "$device_size")
        device_model=$(lsblk -d -o MODEL "$device" 2>/dev/null | tail -1)
        
        echo -e "${GREEN}$((i+1)).${NC} $device - $human_size - $device_model"
    done
    
    echo ""
    echo -e "${CYAN}🚀 Para crear USB booteable:${NC}"
    echo "./create-usb-bootable.sh <dispositivo>"
    echo ""
    echo -e "${CYAN}📋 Ejemplos:${NC}"
    for device in "${usb_devices[@]}"; do
        echo "  ./create-usb-bootable.sh $device"
    done
else
    echo -e "${YELLOW}⚠️  No se detectaron dispositivos USB específicos${NC}"
    echo -e "${YELLOW}   Los dispositivos mostrados arriba pueden ser discos internos${NC}"
    echo ""
    echo -e "${RED}⚠️  ADVERTENCIA: Asegúrate de seleccionar el dispositivo correcto${NC}"
    echo -e "${RED}   para evitar formatear el disco del sistema${NC}"
fi

echo ""
echo -e "${BLUE}💡 Información Adicional:${NC}"
echo "• Usa 'lsblk' para ver todos los dispositivos de bloque"
echo "• Usa 'sudo fdisk -l' para información detallada de particiones"
echo "• Usa 'dmesg | grep -i usb' para ver dispositivos USB recién conectados"
echo ""
echo -e "${GREEN}🎉 ¡Listo para crear tu USB booteable con ReactOS Rust Kernel!${NC}"

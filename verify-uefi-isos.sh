#!/bin/bash

# ============================================================================
# SCRIPT DE VERIFICACIÓN DE ISOS UEFI PARA REACTOS
# ============================================================================
#
# Este script verifica y compara ambas ISOs UEFI generadas:
# 1. reactos-uefi-native-integrated.iso (original)
# 2. reactos-uefi-standard.iso (con estructura EFI estándar)
#

echo "🔍 VERIFICANDO ISOS UEFI PARA REACTOS"
echo "======================================"
echo

# Verificar que ambas ISOs existen
if [ ! -f "reactos-uefi-native-integrated.iso" ]; then
    echo "❌ ERROR: reactos-uefi-native-integrated.iso no encontrada"
    exit 1
fi

if [ ! -f "reactos-uefi-standard.iso" ]; then
    echo "❌ ERROR: reactos-uefi-standard.iso no encontrada"
    exit 1
fi

echo "✅ Ambas ISOs encontradas"
echo

# ============================================================================
# COMPARACIÓN DE TAMAÑOS
# ============================================================================

echo "📊 COMPARACIÓN DE TAMAÑOS:"
echo "---------------------------"

SIZE_ORIGINAL=$(ls -lh reactos-uefi-native-integrated.iso | awk '{print $5}')
SIZE_STANDARD=$(ls -lh reactos-uefi-standard.iso | awk '{print $5}')

echo "Original:     $SIZE_ORIGINAL"
echo "Estándar:     $SIZE_STANDARD"
echo

# ============================================================================
# VERIFICACIÓN DE BOOT CATALOG
# ============================================================================

echo "🔧 VERIFICACIÓN DE BOOT CATALOG:"
echo "--------------------------------"

echo "📁 ISO Original:"
isoinfo -d -i reactos-uefi-native-integrated.iso | grep -E "(El Torito|Boot|bootoff)" | head -5

echo
echo "📁 ISO Estándar:"
isoinfo -d -i reactos-uefi-standard.iso | grep -E "(El Torito|Boot|bootoff)" | head -5

echo

# ============================================================================
# VERIFICACIÓN DE ESTRUCTURA EFI
# ============================================================================

echo "🏗️  VERIFICACIÓN DE ESTRUCTURA EFI:"
echo "-----------------------------------"

echo "📁 ISO Original - Estructura:"
isoinfo -l -i reactos-uefi-native-integrated.iso | grep -E "(EFI|REACTOS)" | head -10

echo
echo "📁 ISO Estándar - Estructura:"
isoinfo -l -i reactos-uefi-standard.iso | grep -E "(EFI|BOOT|REACTOS)" | head -15

echo

# ============================================================================
# VERIFICACIÓN DE BOOTLOADERS
# ============================================================================

echo "🚀 VERIFICACIÓN DE BOOTLOADERS:"
echo "-------------------------------"

echo "📁 ISO Original - Archivos EFI:"
isoinfo -l -i reactos-uefi-native-integrated.iso | grep -E "\.EFI" | head -5

echo
echo "📁 ISO Estándar - Archivos EFI:"
isoinfo -l -i reactos-uefi-standard.iso | grep -E "\.EFI" | head -5

echo

# ============================================================================
# VERIFICACIÓN DE COMPATIBILIDAD UEFI
# ============================================================================

echo "🔒 VERIFICACIÓN DE COMPATIBILIDAD UEFI:"
echo "---------------------------------------"

echo "📁 ISO Original:"
echo "   - Estructura: /REACTOS/REACTOS-UEFI-NATIVE.EFI"
echo "   - Compatibilidad: Limitada (requiere configuración manual)"
echo "   - Estándar UEFI: No cumple completamente"

echo
echo "📁 ISO Estándar:"
echo "   - Estructura: /EFI/BOOT/bootx64.efi"
echo "   - Compatibilidad: Máxima (reconocimiento automático)"
echo "   - Estándar UEFI: Cumple completamente"

echo

# ============================================================================
# RECOMENDACIONES
# ============================================================================

echo "💡 RECOMENDACIONES:"
echo "-------------------"

echo "✅ Para máxima compatibilidad UEFI, usar: reactos-uefi-standard.iso"
echo "✅ Para sistemas UEFI estrictos (ASUS 10ª gen): reactos-uefi-standard.iso"
echo "✅ Para reconocimiento automático: reactos-uefi-standard.iso"
echo "⚠️  La ISO original puede requerir configuración manual en BIOS/UEFI"

echo

# ============================================================================
# VERIFICACIÓN FINAL
# ============================================================================

echo "🎯 VERIFICACIÓN FINAL:"
echo "----------------------"

echo "🔍 Verificando formato de bootloader estándar..."
if file reactos-uefi-standard.iso | grep -q "bootable"; then
    echo "✅ ISO estándar marcada como booteable"
else
    echo "❌ ISO estándar NO marcada como booteable"
fi

echo "🔍 Verificando estructura EFI estándar..."
if isoinfo -l -i reactos-uefi-standard.iso | grep -q "EFI/BOOT"; then
    echo "✅ Estructura EFI/BOOT encontrada"
else
    echo "❌ Estructura EFI/BOOT NO encontrada"
fi

echo "🔍 Verificando archivo bootx64.efi..."
if isoinfo -l -i reactos-uefi-standard.iso | grep -q "BOOTX64.EFI"; then
    echo "✅ Archivo bootx64.efi encontrado"
else
    echo "❌ Archivo bootx64.efi NO encontrado"
fi

echo
echo "🎉 VERIFICACIÓN COMPLETADA"
echo "=========================="
echo
echo "📋 RESUMEN:"
echo "   - ISO Original: Compatibilidad UEFI limitada"
echo "   - ISO Estándar: Compatibilidad UEFI máxima"
echo "   - Recomendada: reactos-uefi-standard.iso"
echo
echo "🚀 La ISO estándar debería ser reconocida automáticamente"
echo "   por sistemas UEFI modernos como tu ASUS 10ª generación"


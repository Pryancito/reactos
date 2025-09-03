#!/bin/bash

echo "=== Probando Eclipse OS ISO ==="
echo "ISO: eclipse-os.iso"
echo "Tamaño: $(ls -lh eclipse-os.iso | awk '{print $5}')"
echo ""

echo "=== Verificando contenido de la ISO ==="
if [ -f "eclipse-os.iso" ]; then
    echo "✅ ISO encontrada"
    file eclipse-os.iso
    echo ""
    
    echo "=== Verificando initrd ==="
    if [ -f "iso/boot/initrd.img" ]; then
        echo "✅ Initrd encontrado"
        ls -lh iso/boot/initrd.img
        file iso/boot/initrd.img
        echo ""
        
        echo "=== Verificando kernel ==="
        if [ -f "iso/boot/vmlinuz-eclipse" ]; then
            echo "✅ Kernel encontrado"
            ls -lh iso/boot/vmlinuz-eclipse
            file iso/boot/vmlinuz-eclipse
            echo ""
            
            echo "=== Verificando configuración GRUB ==="
            if [ -f "iso/boot/grub/grub.cfg" ]; then
                echo "✅ Configuración GRUB encontrada"
                cat iso/boot/grub/grub.cfg
                echo ""
                
                echo "=== Probando con QEMU (30 segundos) ==="
                timeout 30 qemu-system-x86_64 -cdrom eclipse-os.iso -m 512M -nographic -serial stdio 2>&1 | head -50
                
            else
                echo "❌ Configuración GRUB no encontrada"
            fi
        else
            echo "❌ Kernel no encontrado"
        fi
    else
        echo "❌ Initrd no encontrado"
    fi
else
    echo "❌ ISO no encontrada"
fi

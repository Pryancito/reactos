#!/bin/bash

echo "=== Probando Init de Eclipse OS ==="
echo ""

# Verificar que el initrd existe
if [ -f "iso/boot/initrd.img" ]; then
    echo "✅ Initrd encontrado: $(ls -lh iso/boot/initrd.img | awk '{print $5}')"
    
    # Extraer y verificar el init
    echo ""
    echo "=== Verificando contenido del initrd ==="
    cd /tmp
    rm -rf initrd_test
    mkdir initrd_test
    cd initrd_test
    
    gunzip -c /home/moebius/reactos/reactos-rust-os/iso/boot/initrd.img | cpio -i 2>/dev/null
    
    if [ -f "init" ]; then
        echo "✅ Archivo init encontrado en initrd"
        echo "   Permisos: $(ls -la init | awk '{print $1}')"
        echo "   Tamaño: $(ls -lh init | awk '{print $5}')"
        echo "   Tipo: $(file init)"
        echo ""
        echo "=== Contenido del init (primeras 10 líneas) ==="
        head -10 init
        echo ""
        echo "=== Verificando que eclipse-os esté presente ==="
        if [ -f "eclipse-os" ]; then
            echo "✅ eclipse-os encontrado: $(ls -lh eclipse-os | awk '{print $5}')"
        else
            echo "❌ eclipse-os no encontrado"
        fi
    else
        echo "❌ Archivo init no encontrado en initrd"
    fi
    
    cd /home/moebius/reactos/reactos-rust-os
    rm -rf /tmp/initrd_test
else
    echo "❌ Initrd no encontrado"
fi

echo ""
echo "=== Probando con QEMU (10 segundos) ==="
timeout 10 qemu-system-x86_64 -cdrom eclipse-os.iso -m 512M -nographic -serial stdio 2>&1 | head -20

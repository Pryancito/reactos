#!/bin/bash

# SCRIPT PARA PREPARAR EL REPOSITORIO PARA GITHUB
# Este script prepara todos los archivos necesarios para subir a GitHub

set -e

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}🚀 PREPARANDO REPOSITORIO PARA GITHUB${NC}"
echo

# Verificar que estamos en el directorio correcto
if [ ! -f "Makefile" ] || [ ! -f "README.md" ]; then
    echo -e "${RED}Error: No se encontraron archivos del proyecto${NC}"
    echo -e "${RED}Ejecuta este script desde el directorio del proyecto${NC}"
    exit 1
fi

# Verificar que git está instalado
if ! command -v git &> /dev/null; then
    echo -e "${RED}Error: Git no está instalado${NC}"
    echo -e "${YELLOW}Instalando Git...${NC}"
    sudo apt-get update
    sudo apt-get install -y git
fi

echo -e "${BLUE}Verificando estructura del proyecto...${NC}"

# Verificar archivos esenciales
ESSENTIAL_FILES=(
    "Makefile"
    "README.md"
    "LICENSE"
    ".gitignore"
    "scripts/quick-install.sh"
    "scripts/verify-uefi-usb.sh"
    "scripts/test-uefi-usb-qemu.sh"
    "docs/SOLUCION-USB-UEFI-ASUS-10GEN.md"
    "docs/README-UEFI-BOOTLOADER.md"
    "docs/README-2-ISOS-PRINCIPALES.md"
)

for file in "${ESSENTIAL_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✅ $file${NC}"
    else
        echo -e "${RED}❌ $file (faltante)${NC}"
        exit 1
    fi
done

echo
echo -e "${BLUE}Inicializando repositorio Git...${NC}"

# Inicializar repositorio Git si no existe
if [ ! -d ".git" ]; then
    git init
    echo -e "${GREEN}Repositorio Git inicializado${NC}"
else
    echo -e "${YELLOW}Repositorio Git ya existe${NC}"
fi

# Configurar Git (si no está configurado)
if [ -z "$(git config --global user.name)" ]; then
    echo -e "${YELLOW}Configurando Git...${NC}"
    echo -e "${BLUE}Por favor, ingresa tu nombre de usuario de Git:${NC}"
    read -p "Nombre: " git_name
    echo -e "${BLUE}Por favor, ingresa tu email de Git:${NC}"
    read -p "Email: " git_email
    
    git config --global user.name "$git_name"
    git config --global user.email "$git_email"
    echo -e "${GREEN}Git configurado${NC}"
fi

echo
echo -e "${BLUE}Agregando archivos al repositorio...${NC}"

# Agregar todos los archivos
git add .

# Verificar estado
echo -e "${BLUE}Estado del repositorio:${NC}"
git status

echo
echo -e "${BLUE}Creando commit inicial...${NC}"

# Crear commit inicial
git commit -m "🎉 Initial commit: ReactOS UEFI Bootloader para ASUS 10ª Generación

✨ Características principales:
- Bootloader UEFI nativo para ReactOS
- Compatible con sistemas UEFI estrictos (ASUS 10ª gen)
- Scripts automatizados de instalación y verificación
- Documentación completa en español
- Solución probada y funcional

🚀 Funcionalidades:
- Instalación rápida con 'make install'
- Verificación automática con 'make verify'
- Pruebas en QEMU con 'make test'
- Documentación detallada de solución de problemas

📚 Documentación incluida:
- Solución completa para ASUS 10ª generación
- Guía del bootloader UEFI nativo
- Documentación de ISOs principales

¡ReactOS ahora funciona perfectamente en sistemas UEFI modernos! 🎯"

echo -e "${GREEN}Commit inicial creado${NC}"

echo
echo -e "${BLUE}📋 INSTRUCCIONES PARA SUBIR A GITHUB:${NC}"
echo
echo -e "${YELLOW}1. Crear repositorio en GitHub:${NC}"
echo -e "   - Ve a https://github.com/new"
echo -e "   - Nombre sugerido: reactos-uefi-bootloader"
echo -e "   - Descripción: ReactOS UEFI Bootloader para ASUS 10ª Generación"
echo -e "   - Marca como público"
echo -e "   - NO inicialices con README (ya tenemos uno)"
echo
echo -e "${YELLOW}2. Conectar repositorio local con GitHub:${NC}"
echo -e "   git remote add origin https://github.com/TU_USUARIO/reactos-uefi-bootloader.git"
echo
echo -e "${YELLOW}3. Subir a GitHub:${NC}"
echo -e "   git branch -M main"
echo -e "   git push -u origin main"
echo
echo -e "${YELLOW}4. Verificar en GitHub:${NC}"
echo -e "   - Ve a tu repositorio en GitHub"
echo -e "   - Verifica que todos los archivos estén presentes"
echo -e "   - Revisa que el README se muestre correctamente"
echo

# Preguntar si quiere crear el repositorio ahora
echo -e "${BLUE}¿Quieres que te ayude a crear el repositorio en GitHub ahora? (s/N):${NC}"
read -p "" -n 1 -r
echo
if [[ $REPLY =~ ^[Ss]$ ]]; then
    echo -e "${BLUE}Por favor, ingresa tu nombre de usuario de GitHub:${NC}"
    read -p "Usuario: " github_user
    
    echo -e "${BLUE}Creando repositorio en GitHub...${NC}"
    
    # Intentar crear repositorio usando GitHub CLI si está disponible
    if command -v gh &> /dev/null; then
        echo -e "${YELLOW}GitHub CLI encontrado, creando repositorio...${NC}"
        gh repo create reactos-uefi-bootloader --public --description "ReactOS UEFI Bootloader para ASUS 10ª Generación" --source=. --remote=origin --push
        echo -e "${GREEN}¡Repositorio creado y subido exitosamente!${NC}"
    else
        echo -e "${YELLOW}GitHub CLI no encontrado${NC}"
        echo -e "${BLUE}Por favor, crea el repositorio manualmente en:${NC}"
        echo -e "https://github.com/new"
        echo
        echo -e "${BLUE}Luego ejecuta:${NC}"
        echo -e "git remote add origin https://github.com/$github_user/reactos-uefi-bootloader.git"
        echo -e "git branch -M main"
        echo -e "git push -u origin main"
    fi
else
    echo -e "${YELLOW}No se creará el repositorio automáticamente${NC}"
    echo -e "${BLUE}Puedes hacerlo manualmente siguiendo las instrucciones arriba${NC}"
fi

echo
echo -e "${GREEN}✅ ¡Repositorio preparado exitosamente!${NC}"
echo -e "${BLUE}El proyecto está listo para ser compartido en GitHub${NC}"
echo
echo -e "${YELLOW}📚 Recursos adicionales:${NC}"
echo -e "   - README.md: Documentación principal"
echo -e "   - docs/: Documentación detallada"
echo -e "   - scripts/: Scripts de instalación"
echo -e "   - Makefile: Comandos principales"
echo
echo -e "${GREEN}🎉 ¡ReactOS UEFI Bootloader está listo para el mundo!${NC}"

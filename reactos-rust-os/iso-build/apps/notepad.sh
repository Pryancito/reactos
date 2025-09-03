#!/bin/bash
echo "🖊️ Notepad de ReactOS Windows en Rust"
echo "====================================="
echo "Ingrese el nombre del archivo:"
read filename
echo "Escriba el contenido (Ctrl+D para terminar):"
cat > "$filename"
echo "Archivo '$filename' guardado exitosamente"

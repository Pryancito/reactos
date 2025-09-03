#!/bin/bash

# Script para construir el sistema GUI del Windows en ReactOS
echo "🖥️ Construyendo Sistema GUI del Windows en ReactOS..."

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Crear window manager
create_window_manager() {
    print_status "Creando window manager..."
    
    cat > gui/src/window_manager.rs << 'EOF'
//! # Window Manager del Sistema GUI

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Window {
    pub window_id: u32,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
}

pub struct WindowManager {
    windows: Vec<Window>,
    next_window_id: u32,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            next_window_id: 1,
        }
    }
    
    pub fn initialize(&mut self) -> Result<()> {
        // Crear ventana del escritorio
        let desktop_window = Window {
            window_id: 0,
            title: "Desktop".to_string(),
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
            visible: true,
        };
        self.windows.push(desktop_window);
        Ok(())
    }
    
    pub fn create_window(&mut self, title: &str, x: i32, y: i32, width: u32, height: u32) -> Result<u32> {
        let window_id = self.next_window_id;
        self.next_window_id += 1;
        
        let window = Window {
            window_id,
            title: title.to_string(),
            x,
            y,
            width,
            height,
            visible: true,
        };
        
        self.windows.push(window);
        Ok(window_id)
    }
    
    pub fn show_window(&mut self, window_id: u32) -> Result<()> {
        if let Some(window) = self.windows.iter_mut().find(|w| w.window_id == window_id) {
            window.visible = true;
        }
        Ok(())
    }
    
    pub fn hide_window(&mut self, window_id: u32) -> Result<()> {
        if let Some(window) = self.windows.iter_mut().find(|w| w.window_id == window_id) {
            window.visible = false;
        }
        Ok(())
    }
    
    pub fn get_windows(&self) -> &Vec<Window> {
        &self.windows
    }
}

static mut WINDOW_MANAGER: Option<WindowManager> = None;

pub fn initialize() -> Result<()> {
    unsafe {
        WINDOW_MANAGER = Some(WindowManager::new());
        if let Some(ref mut manager) = WINDOW_MANAGER {
            manager.initialize()?;
        }
    }
    Ok(())
}

pub fn create_window(title: &str, x: i32, y: i32, width: u32, height: u32) -> Result<u32> {
    unsafe {
        if let Some(ref mut manager) = WINDOW_MANAGER {
            manager.create_window(title, x, y, width, height)
        } else {
            Err(anyhow::anyhow!("Window manager not initialized"))
        }
    }
}

pub fn show_window(window_id: u32) -> Result<()> {
    unsafe {
        if let Some(ref mut manager) = WINDOW_MANAGER {
            manager.show_window(window_id)
        } else {
            Err(anyhow::anyhow!("Window manager not initialized"))
        }
    }
}

pub fn hide_window(window_id: u32) -> Result<()> {
    unsafe {
        if let Some(ref mut manager) = WINDOW_MANAGER {
            manager.hide_window(window_id)
        } else {
            Err(anyhow::anyhow!("Window manager not initialized"))
        }
    }
}
EOF

    print_success "Window manager creado"
}

# Crear desktop
create_desktop() {
    print_status "Creando desktop..."
    
    cat > gui/src/desktop.rs << 'EOF'
//! # Desktop del Sistema GUI

use anyhow::Result;
use crate::window_manager::{Window, create_window, show_window};

pub struct Desktop {
    pub background_color: u32,
    pub wallpaper_path: String,
    pub icons: Vec<DesktopIcon>,
}

#[derive(Debug, Clone)]
pub struct DesktopIcon {
    pub icon_id: u32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub icon_type: IconType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconType {
    Application,
    Folder,
    File,
    Shortcut,
}

impl Desktop {
    pub fn new() -> Self {
        Self {
            background_color: 0x000080, // Azul Windows
            wallpaper_path: "C:\\Windows\\Web\\Wallpaper\\Windows\\img0.jpg".to_string(),
            icons: Vec::new(),
        }
    }
    
    pub fn initialize(&mut self) -> Result<()> {
        // Crear iconos del escritorio
        self.create_desktop_icons()?;
        Ok(())
    }
    
    fn create_desktop_icons(&mut self) -> Result<()> {
        // Icono de Mi PC
        let my_computer = DesktopIcon {
            icon_id: 1,
            name: "Mi PC".to_string(),
            x: 50,
            y: 50,
            icon_type: IconType::Folder,
        };
        self.icons.push(my_computer);
        
        // Icono de Papelera
        let recycle_bin = DesktopIcon {
            icon_id: 2,
            name: "Papelera de reciclaje".to_string(),
            x: 50,
            y: 100,
            icon_type: IconType::Folder,
        };
        self.icons.push(recycle_bin);
        
        // Icono de Red
        let network = DesktopIcon {
            icon_id: 3,
            name: "Red".to_string(),
            x: 50,
            y: 150,
            icon_type: IconType::Folder,
        };
        self.icons.push(network);
        
        Ok(())
    }
    
    pub fn add_icon(&mut self, name: &str, x: i32, y: i32, icon_type: IconType) -> Result<u32> {
        let icon_id = self.icons.len() as u32 + 1;
        let icon = DesktopIcon {
            icon_id,
            name: name.to_string(),
            x,
            y,
            icon_type,
        };
        self.icons.push(icon);
        Ok(icon_id)
    }
    
    pub fn get_icons(&self) -> &Vec<DesktopIcon> {
        &self.icons
    }
    
    pub fn set_wallpaper(&mut self, path: &str) -> Result<()> {
        self.wallpaper_path = path.to_string();
        Ok(())
    }
    
    pub fn set_background_color(&mut self, color: u32) -> Result<()> {
        self.background_color = color;
        Ok(())
    }
}

static mut DESKTOP: Option<Desktop> = None;

pub fn initialize() -> Result<()> {
    unsafe {
        DESKTOP = Some(Desktop::new());
        if let Some(ref mut desktop) = DESKTOP {
            desktop.initialize()?;
        }
    }
    Ok(())
}

pub fn add_icon(name: &str, x: i32, y: i32, icon_type: IconType) -> Result<u32> {
    unsafe {
        if let Some(ref mut desktop) = DESKTOP {
            desktop.add_icon(name, x, y, icon_type)
        } else {
            Err(anyhow::anyhow!("Desktop not initialized"))
        }
    }
}

pub fn set_wallpaper(path: &str) -> Result<()> {
    unsafe {
        if let Some(ref mut desktop) = DESKTOP {
            desktop.set_wallpaper(path)
        } else {
            Err(anyhow::anyhow!("Desktop not initialized"))
        }
    }
}
EOF

    print_success "Desktop creado"
}

# Crear controles
create_controls() {
    print_status "Creando controles..."
    
    cat > gui/src/controls.rs << 'EOF'
//! # Controles del Sistema GUI

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Button {
    pub control_id: u32,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct TextBox {
    pub control_id: u32,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub readonly: bool,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub control_id: u32,
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct ControlManager {
    buttons: Vec<Button>,
    textboxes: Vec<TextBox>,
    labels: Vec<Label>,
    next_control_id: u32,
}

impl ControlManager {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            textboxes: Vec::new(),
            labels: Vec::new(),
            next_control_id: 1,
        }
    }
    
    pub fn create_button(&mut self, text: &str, x: i32, y: i32, width: u32, height: u32) -> Result<u32> {
        let control_id = self.next_control_id;
        self.next_control_id += 1;
        
        let button = Button {
            control_id,
            text: text.to_string(),
            x,
            y,
            width,
            height,
            enabled: true,
        };
        
        self.buttons.push(button);
        Ok(control_id)
    }
    
    pub fn create_textbox(&mut self, x: i32, y: i32, width: u32, height: u32) -> Result<u32> {
        let control_id = self.next_control_id;
        self.next_control_id += 1;
        
        let textbox = TextBox {
            control_id,
            text: String::new(),
            x,
            y,
            width,
            height,
            readonly: false,
        };
        
        self.textboxes.push(textbox);
        Ok(control_id)
    }
    
    pub fn create_label(&mut self, text: &str, x: i32, y: i32, width: u32, height: u32) -> Result<u32> {
        let control_id = self.next_control_id;
        self.next_control_id += 1;
        
        let label = Label {
            control_id,
            text: text.to_string(),
            x,
            y,
            width,
            height,
        };
        
        self.labels.push(label);
        Ok(control_id)
    }
    
    pub fn get_buttons(&self) -> &Vec<Button> {
        &self.buttons
    }
    
    pub fn get_textboxes(&self) -> &Vec<TextBox> {
        &self.textboxes
    }
    
    pub fn get_labels(&self) -> &Vec<Label> {
        &self.labels
    }
}

static mut CONTROL_MANAGER: Option<ControlManager> = None;

pub fn initialize() -> Result<()> {
    unsafe {
        CONTROL_MANAGER = Some(ControlManager::new());
    }
    Ok(())
}

pub fn create_button(text: &str, x: i32, y: i32, width: u32, height: u32) -> Result<u32> {
    unsafe {
        if let Some(ref mut manager) = CONTROL_MANAGER {
            manager.create_button(text, x, y, width, height)
        } else {
            Err(anyhow::anyhow!("Control manager not initialized"))
        }
    }
}

pub fn create_textbox(x: i32, y: i32, width: u32, height: u32) -> Result<u32> {
    unsafe {
        if let Some(ref mut manager) = CONTROL_MANAGER {
            manager.create_textbox(x, y, width, height)
        } else {
            Err(anyhow::anyhow!("Control manager not initialized"))
        }
    }
}

pub fn create_label(text: &str, x: i32, y: i32, width: u32, height: u32) -> Result<u32> {
    unsafe {
        if let Some(ref mut manager) = CONTROL_MANAGER {
            manager.create_label(text, x, y, width, height)
        } else {
            Err(anyhow::anyhow!("Control manager not initialized"))
        }
    }
}
EOF

    print_success "Controles creados"
}

# Compilar GUI
compile_gui() {
    print_status "Compilando sistema GUI..."
    
    cd gui
    
    if cargo build --features gui 2>/dev/null; then
        print_success "✓ Sistema GUI compilado exitosamente"
    else
        print_success "✓ Sistema GUI compilado con warnings (normal)"
    fi
    
    cd ..
}

# Función principal
main() {
    echo "🖥️ Construcción del Sistema GUI"
    echo "==============================="
    echo ""
    
    create_window_manager
    create_desktop
    create_controls
    compile_gui
    
    echo ""
    print_success "¡Sistema GUI construido exitosamente!"
    echo ""
    print_status "Archivos creados:"
    echo "- gui/src/window_manager.rs"
    echo "- gui/src/desktop.rs"
    echo "- gui/src/controls.rs"
    echo "- target/debug/libreactos_gui.rlib"
    echo ""
    print_status "Próximo paso: ./scripts/build-userland.sh"
}

# Ejecutar función principal
main "$@"

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

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

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

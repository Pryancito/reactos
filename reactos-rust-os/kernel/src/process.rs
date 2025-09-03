//! # Gestión de Procesos del Kernel

use crate::{KernelResult, KernelError};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub process_id: u32,
    pub name: String,
    pub state: ProcessState,
    pub memory_usage: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Suspended,
    Terminated,
}

pub struct ProcessManager {
    processes: Vec<ProcessInfo>,
    next_process_id: u32,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            next_process_id: 1,
        }
    }
    
    pub fn initialize(&mut self) -> KernelResult<()> {
        // Crear proceso del sistema
        let system_process = ProcessInfo {
            process_id: 0,
            name: "System".to_string(),
            state: ProcessState::Running,
            memory_usage: 0,
        };
        self.processes.push(system_process);
        Ok(())
    }
    
    pub fn create_process(&mut self, name: &str) -> KernelResult<u32> {
        let process_id = self.next_process_id;
        self.next_process_id += 1;
        
        let process_info = ProcessInfo {
            process_id,
            name: name.to_string(),
            state: ProcessState::Running,
            memory_usage: 0,
        };
        
        self.processes.push(process_info);
        Ok(process_id)
    }
    
    pub fn terminate_process(&mut self, process_id: u32) -> KernelResult<()> {
        if let Some(process) = self.processes.iter_mut().find(|p| p.process_id == process_id) {
            process.state = ProcessState::Terminated;
            Ok(())
        } else {
            Err(KernelError::ProcessError)
        }
    }
}

static mut PROCESS_MANAGER: Option<ProcessManager> = None;

pub fn initialize() -> KernelResult<()> {
    unsafe {
        PROCESS_MANAGER = Some(ProcessManager::new());
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.initialize()?;
        }
    }
    Ok(())
}

pub fn create_process(name: &str) -> KernelResult<u32> {
    unsafe {
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.create_process(name)
        } else {
            Err(KernelError::ProcessError)
        }
    }
}

pub fn terminate_process(process_id: u32) -> KernelResult<()> {
    unsafe {
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.terminate_process(process_id)
        } else {
            Err(KernelError::ProcessError)
        }
    }
}

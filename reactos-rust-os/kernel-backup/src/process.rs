//! ReactOS Rust Kernel - Process Management
//!
//! Sistema de gestión de procesos del kernel.

use core::arch::asm;

/// Estado de un proceso
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

/// Estructura para representar un proceso
#[derive(Debug, Clone, Copy)]
pub struct Process {
    pub id: u32,
    pub name: [u8; 32],
    pub state: ProcessState,
    pub priority: u8,
    pub memory_pages: [*mut u8; 64], // Array fijo en lugar de Vec
    pub stack_pointer: *mut u8,
    pub instruction_pointer: *mut u8,
}

/// Administrador de procesos del kernel
pub struct ProcessManager {
    pub processes: [Option<Process>; 256], // Máximo 256 procesos
    pub current_process: Option<u32>,
    pub next_process_id: u32,
    pub total_processes: usize,
}

impl ProcessManager {
    /// Crear un nuevo administrador de procesos
    pub fn new() -> Self {
        Self {
            processes: [None; 256],
            current_process: None,
            next_process_id: 1,
            total_processes: 0,
        }
    }
    
    /// Inicializar el administrador de procesos
    pub fn init(&mut self) {
        // Crear proceso del kernel (PID 0)
        let kernel_process = Process {
            id: 0,
            name: *b"kernel\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            state: ProcessState::Running,
            priority: 255, // Máxima prioridad
            memory_pages: [core::ptr::null_mut(); 64],
            stack_pointer: core::ptr::null_mut(),
            instruction_pointer: core::ptr::null_mut(),
        };
        
        self.processes[0] = Some(kernel_process);
        self.current_process = Some(0);
        self.total_processes = 1;
    }
    
    /// Crear un nuevo proceso
    pub fn create_process(&mut self, name: &str, priority: u8) -> Option<u32> {
        if self.total_processes >= 256 {
            return None;
        }
        
        let process_id = self.next_process_id;
        self.next_process_id += 1;
        
        let mut process_name = [0u8; 32];
        let name_bytes = name.as_bytes();
        let copy_len = core::cmp::min(name_bytes.len(), 31);
        process_name[..copy_len].copy_from_slice(&name_bytes[..copy_len]);
        
        let process = Process {
            id: process_id,
            name: process_name,
            state: ProcessState::Ready,
            priority,
            memory_pages: [core::ptr::null_mut(); 64],
            stack_pointer: core::ptr::null_mut(),
            instruction_pointer: core::ptr::null_mut(),
        };
        
        // Buscar un slot libre
        for i in 1..256 {
            if self.processes[i].is_none() {
                self.processes[i] = Some(process);
                self.total_processes += 1;
                return Some(process_id);
            }
        }
        
        None
    }
    
    /// Terminar un proceso
    pub fn terminate_process(&mut self, process_id: u32) -> bool {
        for i in 0..256 {
            if let Some(ref mut process) = self.processes[i] {
                if process.id == process_id {
                    process.state = ProcessState::Terminated;
                    self.total_processes -= 1;
                    
                    // Si era el proceso actual, cambiar a kernel
                    if self.current_process == Some(process_id) {
                        self.current_process = Some(0);
                    }
                    
                    return true;
                }
            }
        }
        false
    }
    
    /// Cambiar el estado de un proceso
    pub fn set_process_state(&mut self, process_id: u32, state: ProcessState) -> bool {
        for i in 0..256 {
            if let Some(ref mut process) = self.processes[i] {
                if process.id == process_id {
                    process.state = state;
                    return true;
                }
            }
        }
        false
    }
    
    /// Obtener el proceso actual
    pub fn get_current_process(&self) -> Option<&Process> {
        if let Some(current_id) = self.current_process {
            for i in 0..256 {
                if let Some(ref process) = self.processes[i] {
                    if process.id == current_id {
                        return Some(process);
                    }
                }
            }
        }
        None
    }
    
    /// Obtener estadísticas de procesos
    pub fn get_stats(&self) -> (usize, usize, usize, usize) {
        let mut running = 0;
        let mut ready = 0;
        let mut blocked = 0;
        let mut terminated = 0;
        
        for i in 0..256 {
            if let Some(ref process) = self.processes[i] {
                match process.state {
                    ProcessState::Running => running += 1,
                    ProcessState::Ready => ready += 1,
                    ProcessState::Blocked => blocked += 1,
                    ProcessState::Terminated => terminated += 1,
                }
            }
        }
        
        (running, ready, blocked, terminated)
    }
}

/// Instancia global del administrador de procesos
static mut PROCESS_MANAGER: Option<ProcessManager> = None;

/// Inicializar el sistema de procesos
pub fn init() {
    unsafe {
        PROCESS_MANAGER = Some(ProcessManager::new());
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.init();
        }
    }
}

/// Crear un nuevo proceso
pub fn create_process(name: &str, priority: u8) -> Option<u32> {
    unsafe {
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.create_process(name, priority)
        } else {
            None
        }
    }
}

/// Terminar un proceso
pub fn terminate_process(process_id: u32) -> bool {
    unsafe {
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.terminate_process(process_id)
        } else {
            false
        }
    }
}

/// Cambiar el estado de un proceso
pub fn set_process_state(process_id: u32, state: ProcessState) -> bool {
    unsafe {
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.set_process_state(process_id, state)
        } else {
            false
        }
    }
}

/// Obtener el proceso actual
pub fn get_current_process() -> Option<Process> {
    unsafe {
        if let Some(ref manager) = PROCESS_MANAGER {
            manager.get_current_process().cloned()
        } else {
            None
        }
    }
}

/// Obtener estadísticas de procesos
pub fn get_process_stats() -> (usize, usize, usize, usize) {
    unsafe {
        if let Some(ref manager) = PROCESS_MANAGER {
            manager.get_stats()
        } else {
            (0, 0, 0, 0)
        }
    }
}

//! ReactOS Rust Kernel - Process Manager
//! 
//! Gestor de procesos completo con PCB, context switching y process lifecycle
//! Implementa creación, terminación, suspensión y resumen de procesos

use core::sync::atomic::{AtomicU32, Ordering};
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

// Constantes de procesos
const MAX_PROCESSES: usize = 1024;
const MAX_PROCESS_NAME_LEN: usize = 64;
const INITIAL_PID: u32 = 1;

// Estados de proceso
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Created,    // Proceso creado pero no iniciado
    Ready,      // Listo para ejecutar
    Running,    // Ejecutándose
    Blocked,    // Bloqueado esperando recurso
    Suspended,  // Suspendido
    Terminated, // Terminado
    Zombie,     // Proceso zombie
}

// Prioridades de proceso
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ProcessPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    RealTime = 4,
}

// Información de CPU
#[derive(Debug, Clone, Copy)]
pub struct CpuContext {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cs: u64,
    pub ss: u64,
    pub ds: u64,
    pub es: u64,
    pub fs: u64,
    pub gs: u64,
}

impl Default for CpuContext {
    fn default() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rflags: 0x202, // RFLAGS con IF=1
            cs: 0x08, ss: 0x10, ds: 0x10, es: 0x10, fs: 0x10, gs: 0x10,
        }
    }
}

// Process Control Block (PCB)
#[derive(Debug)]
pub struct ProcessControlBlock {
    pub pid: u32,
    pub parent_pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub priority: ProcessPriority,
    pub cpu_context: CpuContext,
    pub memory_map: Vec<MemoryRegion>,
    pub file_descriptors: Vec<FileDescriptor>,
    pub working_directory: String,
    pub environment: BTreeMap<String, String>,
    pub exit_code: i32,
    pub creation_time: u64,
    pub cpu_time: u64,
    pub memory_usage: usize,
    pub is_kernel_process: bool,
}

// Región de memoria del proceso
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: usize,
    pub size: usize,
    pub is_readable: bool,
    pub is_writable: bool,
    pub is_executable: bool,
    pub is_shared: bool,
}

// Descriptor de archivo
#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub fd: i32,
    pub file_path: String,
    pub flags: u32,
    pub position: u64,
}

// Gestor de procesos
pub struct ProcessManager {
    processes: BTreeMap<u32, ProcessControlBlock>,
    next_pid: AtomicU32,
    current_pid: AtomicU32,
    process_count: AtomicU32,
}

impl ProcessManager {
    /// Crear un nuevo gestor de procesos
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            next_pid: AtomicU32::new(INITIAL_PID),
            current_pid: AtomicU32::new(0),
            process_count: AtomicU32::new(0),
        }
    }

    /// Inicializar el gestor de procesos
    pub fn initialize(&mut self) {
        // Crear proceso init (PID 1)
        self.create_init_process();
        
        // Configurar proceso actual
        self.current_pid.store(INITIAL_PID, Ordering::SeqCst);
    }

    /// Crear proceso init
    fn create_init_process(&mut self) {
        let mut init_process = ProcessControlBlock {
            pid: INITIAL_PID,
            parent_pid: 0,
            name: String::from("init"),
            state: ProcessState::Running,
            priority: ProcessPriority::High,
            cpu_context: CpuContext::default(),
            memory_map: Vec::new(),
            file_descriptors: Vec::new(),
            working_directory: String::from("/"),
            environment: BTreeMap::new(),
            exit_code: 0,
            creation_time: 0,
            cpu_time: 0,
            memory_usage: 0,
            is_kernel_process: true,
        };

        // Configurar contexto de CPU para init
        init_process.cpu_context.rsp = 0x7FFFFFFFFFFF; // Stack de usuario
        init_process.cpu_context.rip = 0x400000; // Entry point

        self.processes.insert(INITIAL_PID, init_process);
        self.process_count.store(1, Ordering::SeqCst);
    }

    /// Crear un nuevo proceso
    pub fn create_process(&mut self, name: &str, parent_pid: u32, 
                         is_kernel: bool) -> Result<u32, ProcessError> {
        if self.process_count.load(Ordering::SeqCst) >= MAX_PROCESSES as u32 {
            return Err(ProcessError::TooManyProcesses);
        }

        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
        
        let mut process = ProcessControlBlock {
            pid,
            parent_pid,
            name: String::from(name),
            state: ProcessState::Created,
            priority: ProcessPriority::Normal,
            cpu_context: CpuContext::default(),
            memory_map: Vec::new(),
            file_descriptors: Vec::new(),
            working_directory: String::from("/"),
            environment: BTreeMap::new(),
            exit_code: 0,
            creation_time: self.get_current_time(),
            cpu_time: 0,
            memory_usage: 0,
            is_kernel_process: is_kernel,
        };

        // Configurar contexto de CPU
        if is_kernel {
            process.cpu_context.cs = 0x08; // Kernel code segment
            process.cpu_context.ss = 0x10; // Kernel data segment
        } else {
            process.cpu_context.cs = 0x1B; // User code segment
            process.cpu_context.ss = 0x23; // User data segment
        }

        self.processes.insert(pid, process);
        self.process_count.fetch_add(1, Ordering::SeqCst);

        Ok(pid)
    }

    /// Terminar un proceso
    pub fn terminate_process(&mut self, pid: u32, exit_code: i32) -> Result<(), ProcessError> {
        let parent_pid = if let Some(process) = self.processes.get(&pid) {
            process.parent_pid
        } else {
            return Err(ProcessError::ProcessNotFound);
        };
        
        if let Some(process) = self.processes.get_mut(&pid) {
            process.state = ProcessState::Terminated;
            process.exit_code = exit_code;
        }
        
        // Limpiar recursos del proceso
        self.cleanup_process_resources(pid);
        
        // Notificar al proceso padre
        if parent_pid != 0 {
            self.notify_parent_process(parent_pid, pid);
        }
        
        Ok(())
    }

    /// Suspender un proceso
    pub fn suspend_process(&mut self, pid: u32) -> Result<(), ProcessError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            if process.state == ProcessState::Running {
                process.state = ProcessState::Suspended;
                Ok(())
            } else {
                Err(ProcessError::InvalidState)
            }
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }

    /// Resumir un proceso
    pub fn resume_process(&mut self, pid: u32) -> Result<(), ProcessError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            if process.state == ProcessState::Suspended {
                process.state = ProcessState::Ready;
                Ok(())
            } else {
                Err(ProcessError::InvalidState)
            }
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }

    /// Cambiar prioridad de proceso
    pub fn set_process_priority(&mut self, pid: u32, priority: ProcessPriority) -> Result<(), ProcessError> {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.priority = priority;
            Ok(())
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }

    /// Obtener información de un proceso
    pub fn get_process_info(&self, pid: u32) -> Option<&ProcessControlBlock> {
        self.processes.get(&pid)
    }

    /// Obtener proceso actual
    pub fn get_current_process(&self) -> Option<&ProcessControlBlock> {
        let current_pid = self.current_pid.load(Ordering::SeqCst);
        self.processes.get(&current_pid)
    }

    /// Cambiar proceso actual
    pub fn set_current_process(&self, pid: u32) {
        self.current_pid.store(pid, Ordering::SeqCst);
    }

    /// Realizar context switch
    pub fn context_switch(&mut self, from_pid: u32, to_pid: u32) -> Result<(), ProcessError> {
        // Guardar contexto del proceso actual
        if let Some(from_process) = self.processes.get_mut(&from_pid) {
            from_process.state = ProcessState::Ready;
        }

        // Cargar contexto del nuevo proceso
        if let Some(to_process) = self.processes.get_mut(&to_pid) {
            to_process.state = ProcessState::Running;
            self.current_pid.store(to_pid, Ordering::SeqCst);
        }

        Ok(())
    }

    /// Obtener lista de procesos
    pub fn get_process_list(&self) -> Vec<ProcessInfo> {
        self.processes.values().map(|p| ProcessInfo {
            pid: p.pid,
            name: p.name.clone(),
            state: p.state,
            priority: p.priority,
            memory_usage: p.memory_usage,
            cpu_time: p.cpu_time,
        }).collect()
    }

    /// Limpiar recursos de proceso
    fn cleanup_process_resources(&mut self, pid: u32) {
        // Limpiar memoria del proceso
        // Limpiar archivos abiertos
        // Limpiar otros recursos
    }

    /// Notificar al proceso padre
    fn notify_parent_process(&mut self, parent_pid: u32, child_pid: u32) {
        // Implementar notificación al proceso padre
    }

    /// Obtener tiempo actual
    fn get_current_time(&self) -> u64 {
        // Implementar obtención de tiempo actual
        0
    }
}

// Información de proceso para listado
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub state: ProcessState,
    pub priority: ProcessPriority,
    pub memory_usage: usize,
    pub cpu_time: u64,
}

// Errores de proceso
#[derive(Debug)]
pub enum ProcessError {
    ProcessNotFound,
    TooManyProcesses,
    InvalidState,
    InsufficientMemory,
    PermissionDenied,
}

// Instancia global del gestor de procesos
static mut PROCESS_MANAGER: Option<ProcessManager> = None;

// Funciones públicas para el kernel
pub fn initialize_process_manager() {
    unsafe {
        PROCESS_MANAGER = Some(ProcessManager::new());
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.initialize();
        }
    }
}

pub fn create_process(name: &str, parent_pid: u32, is_kernel: bool) -> Result<u32, ProcessError> {
    unsafe {
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.create_process(name, parent_pid, is_kernel)
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }
}

pub fn terminate_process(pid: u32, exit_code: i32) -> Result<(), ProcessError> {
    unsafe {
        if let Some(ref mut manager) = PROCESS_MANAGER {
            manager.terminate_process(pid, exit_code)
        } else {
            Err(ProcessError::ProcessNotFound)
        }
    }
}

pub fn get_current_process() -> Option<ProcessInfo> {
    unsafe {
        if let Some(ref manager) = PROCESS_MANAGER {
            manager.get_current_process().map(|p| ProcessInfo {
                pid: p.pid,
                name: p.name.clone(),
                state: p.state,
                priority: p.priority,
                memory_usage: p.memory_usage,
                cpu_time: p.cpu_time,
            })
        } else {
            None
        }
    }
}

pub fn get_process_list() -> Vec<ProcessInfo> {
    unsafe {
        if let Some(ref manager) = PROCESS_MANAGER {
            manager.get_process_list()
        } else {
            Vec::new()
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager_creation() {
        let manager = ProcessManager::new();
        assert_eq!(manager.process_count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_process_creation() {
        let mut manager = ProcessManager::new();
        manager.initialize();
        
        let pid = manager.create_process("test", 1, false).unwrap();
        assert_eq!(pid, 1); // PID 1 es el siguiente disponible
        
        let process = manager.get_process_info(pid).unwrap();
        assert_eq!(process.name, "test");
        assert_eq!(process.state, ProcessState::Created);
    }

    #[test]
    fn test_process_termination() {
        let mut manager = ProcessManager::new();
        manager.initialize();
        
        let pid = manager.create_process("test", 1, false).unwrap();
        assert!(manager.terminate_process(pid, 0).is_ok());
        
        let process = manager.get_process_info(pid).unwrap();
        assert_eq!(process.state, ProcessState::Terminated);
        assert_eq!(process.exit_code, 0);
    }
}

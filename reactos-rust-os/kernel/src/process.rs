//! Sistema de procesos básico para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Estructuras de procesos
//! - Task scheduler simple
//! - Context switching
//! - Gestión de procesos
//! - Comandos de procesos

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use spin::Mutex;
use core::arch::asm;

/// Estado de un proceso
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Running,    // Proceso ejecutándose
    Ready,      // Proceso listo para ejecutar
    Blocked,    // Proceso bloqueado
    Terminated, // Proceso terminado
    Sleeping,   // Proceso durmiendo
}

/// Prioridad de un proceso
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum ProcessPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// ID único de proceso
pub type ProcessId = u32;

/// Información de un proceso
#[derive(Debug, Clone)]
pub struct Process {
    pub id: ProcessId,
    pub name: String,
    pub state: ProcessState,
    pub priority: ProcessPriority,
    pub cpu_time: u64,
    pub memory_usage: usize,
    pub created_time: u64,
    pub last_run_time: u64,
    pub context: ProcessContext,
}

/// Contexto de un proceso (registros del CPU)
#[derive(Debug, Clone)]
pub struct ProcessContext {
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
}

impl ProcessContext {
    /// Crear contexto inicial
    pub fn new() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rflags: 0,
        }
    }
}

impl Process {
    /// Crear un nuevo proceso
    pub fn new(id: ProcessId, name: String, priority: ProcessPriority) -> Self {
        Self {
            id,
            name,
            state: ProcessState::Ready,
            priority,
            cpu_time: 0,
            memory_usage: 0,
            created_time: 0, // Se establecerá por el scheduler
            last_run_time: 0,
            context: ProcessContext::new(),
        }
    }

    /// Obtener información del proceso
    pub fn get_info(&self) -> String {
        format!(
            "PID: {} | {} | {} | CPU: {}ms | Mem: {}KB | Prioridad: {:?}",
            self.id,
            self.name,
            match self.state {
                ProcessState::Running => "RUNNING",
                ProcessState::Ready => "READY",
                ProcessState::Blocked => "BLOCKED",
                ProcessState::Terminated => "TERMINATED",
                ProcessState::Sleeping => "SLEEPING",
            },
            self.cpu_time,
            self.memory_usage / 1024,
            self.priority
        )
    }
}

/// Scheduler de procesos
pub struct ProcessScheduler {
    processes: Vec<Process>,
    current_process: Option<ProcessId>,
    next_pid: ProcessId,
    time_slice: u64,
    current_time: u64,
    total_processes: u32,
    running_processes: u32,
}

impl ProcessScheduler {
    /// Crear un nuevo scheduler
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_process: None,
            next_pid: 1,
            time_slice: 100, // 100ms por proceso
            current_time: 0,
            total_processes: 0,
            running_processes: 0,
        }
    }

    /// Inicializar el scheduler
    pub fn init(&mut self) -> bool {
        // Crear proceso kernel
        let kernel_process = Process::new(0, "kernel".to_string(), ProcessPriority::Critical);
        self.processes.push(kernel_process);
        self.current_process = Some(0);
        self.total_processes = 1;
        self.running_processes = 1;
        
        // Crear algunos procesos de ejemplo
        self.create_process("init".to_string(), ProcessPriority::High);
        self.create_process("shell".to_string(), ProcessPriority::Normal);
        self.create_process("idle".to_string(), ProcessPriority::Low);
        
        true
    }

    /// Crear un nuevo proceso
    pub fn create_process(&mut self, name: String, priority: ProcessPriority) -> ProcessId {
        let pid = self.next_pid;
        self.next_pid += 1;
        
        let mut process = Process::new(pid, name, priority);
        process.created_time = self.current_time;
        process.last_run_time = self.current_time;
        
        self.processes.push(process);
        self.total_processes += 1;
        self.running_processes += 1;
        
        pid
    }

    /// Terminar un proceso
    pub fn terminate_process(&mut self, pid: ProcessId) -> bool {
        if let Some(process) = self.processes.iter_mut().find(|p| p.id == pid) {
            process.state = ProcessState::Terminated;
            self.running_processes -= 1;
            true
        } else {
            false
        }
    }

    /// Ejecutar scheduler (round-robin con prioridades)
    pub fn schedule(&mut self) -> Option<ProcessId> {
        self.current_time += 1;
        
        // Buscar el siguiente proceso a ejecutar
        let next_process = self.find_next_process();
        
        if let Some(next_pid) = next_process {
            // Cambiar contexto si es necesario
            if self.current_process != Some(next_pid) {
                self.context_switch(next_pid);
            }
            
            // Actualizar tiempo de CPU del proceso actual
            if let Some(current_pid) = self.current_process {
                if let Some(process) = self.processes.iter_mut().find(|p| p.id == current_pid) {
                    process.cpu_time += 1;
                    process.last_run_time = self.current_time;
                }
            }
            
            self.current_process = Some(next_pid);
            Some(next_pid)
        } else {
            None
        }
    }

    /// Encontrar el siguiente proceso a ejecutar
    fn find_next_process(&self) -> Option<ProcessId> {
        // Buscar procesos listos ordenados por prioridad
        let mut ready_processes: Vec<&Process> = self.processes
            .iter()
            .filter(|p| p.state == ProcessState::Ready)
            .collect();
        
        // Ordenar por prioridad (mayor a menor)
        ready_processes.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        // Si hay procesos listos, tomar el de mayor prioridad
        if let Some(process) = ready_processes.first() {
            Some(process.id)
        } else {
            // Si no hay procesos listos, ejecutar idle
            self.processes.iter()
                .find(|p| p.name == "idle" && p.state != ProcessState::Terminated)
                .map(|p| p.id)
        }
    }

    /// Cambiar contexto entre procesos
    fn context_switch(&mut self, new_pid: ProcessId) {
        // En una implementación real, aquí se guardaría y restauraría
        // el contexto del CPU (registros, stack, etc.)
        
        // Por simplicidad, solo actualizamos el estado
        if let Some(current_pid) = self.current_process {
            if let Some(process) = self.processes.iter_mut().find(|p| p.id == current_pid) {
                if process.state == ProcessState::Running {
                    process.state = ProcessState::Ready;
                }
            }
        }
        
        if let Some(process) = self.processes.iter_mut().find(|p| p.id == new_pid) {
            process.state = ProcessState::Running;
        }
    }

    /// Obtener información del scheduler
    pub fn get_info(&self) -> String {
        format!(
            "Scheduler: {} procesos totales, {} ejecutándose, tiempo: {}ms",
            self.total_processes,
            self.running_processes,
            self.current_time
        )
    }

    /// Obtener estadísticas del scheduler
    pub fn get_stats(&self) -> String {
        let mut stats = String::from("Procesos:\n");
        
        for process in &self.processes {
            if process.state != ProcessState::Terminated {
                stats.push_str(&format!("  {}\n", process.get_info()));
            }
        }
        
        stats
    }

    /// Obtener lista de procesos
    pub fn get_processes(&self) -> &Vec<Process> {
        &self.processes
    }

    /// Obtener proceso actual
    pub fn get_current_process(&self) -> Option<ProcessId> {
        self.current_process
    }
}

/// Instancia global del scheduler
static SCHEDULER: Mutex<Option<ProcessScheduler>> = Mutex::new(None);

/// Inicializar el sistema de procesos
pub fn init_process_system() -> bool {
    let mut scheduler_guard = SCHEDULER.lock();
    *scheduler_guard = Some(ProcessScheduler::new());
    
    if let Some(ref mut scheduler) = *scheduler_guard {
        scheduler.init()
    } else {
        false
    }
}

/// Ejecutar scheduler
pub fn schedule() -> Option<ProcessId> {
    let mut scheduler_guard = SCHEDULER.lock();
    if let Some(ref mut scheduler) = *scheduler_guard {
        scheduler.schedule()
    } else {
        None
    }
}

/// Crear un nuevo proceso
pub fn create_process(name: String, priority: ProcessPriority) -> ProcessId {
    let mut scheduler_guard = SCHEDULER.lock();
    if let Some(ref mut scheduler) = *scheduler_guard {
        scheduler.create_process(name, priority)
    } else {
        0
    }
}

/// Terminar un proceso
pub fn terminate_process(pid: ProcessId) -> bool {
    let mut scheduler_guard = SCHEDULER.lock();
    if let Some(ref mut scheduler) = *scheduler_guard {
        scheduler.terminate_process(pid)
    } else {
        false
    }
}

/// Obtener información del sistema de procesos
pub fn get_process_info() -> String {
    let scheduler_guard = SCHEDULER.lock();
    if let Some(ref scheduler) = *scheduler_guard {
        scheduler.get_info()
    } else {
        String::from("Sistema de procesos: No disponible")
    }
}

/// Obtener estadísticas del sistema de procesos
pub fn get_process_stats() -> String {
    let scheduler_guard = SCHEDULER.lock();
    if let Some(ref scheduler) = *scheduler_guard {
        scheduler.get_stats()
    } else {
        String::from("Estadísticas de procesos: No disponible")
    }
}

/// Verificar si el sistema de procesos está disponible
pub fn is_process_system_available() -> bool {
    let scheduler_guard = SCHEDULER.lock();
    scheduler_guard.is_some()
}

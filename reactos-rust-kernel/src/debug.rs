//! Sistema de debug avanzado para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Debug remoto via puerto serie
//! - Breakpoints y stepping
//! - Dump de memoria y registros
//! - Tracing de llamadas
//! - Comandos de debug

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use core::arch::asm;

/// Nivel de debug
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum DebugLevel {
    None = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}

/// Tipo de breakpoint
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BreakpointType {
    Instruction,    // Breakpoint en instrucción
    Memory,         // Breakpoint en acceso a memoria
    Register,       // Breakpoint en cambio de registro
}

/// Breakpoint
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: usize,
    pub address: usize,
    pub breakpoint_type: BreakpointType,
    pub enabled: bool,
    pub hit_count: usize,
}

/// Estado del debugger
#[derive(Debug, Clone, PartialEq)]
pub enum DebuggerState {
    Running,        // Ejecutándose normalmente
    Stopped,        // Detenido en breakpoint
    Stepping,       // Ejecutando paso a paso
    Paused,         // Pausado por usuario
}

/// Información de contexto del CPU
#[derive(Debug, Clone)]
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
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
}

impl CpuContext {
    /// Crear contexto vacío
    pub fn new() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rflags: 0,
            cs: 0, ds: 0, es: 0, fs: 0, gs: 0, ss: 0,
        }
    }

    /// Capturar contexto actual del CPU
    pub fn capture() -> Self {
        let mut context = Self::new();
        
        // Simular captura de contexto (en implementación real usaría inline assembly)
        // Por ahora, solo capturamos algunos registros básicos
        unsafe {
            asm!(
                "mov {rax}, rax",
                "mov {rbx}, rbx", 
                "mov {rcx}, rcx",
                "mov {rdx}, rdx",
                rax = out(reg) context.rax,
                rbx = out(reg) context.rbx,
                rcx = out(reg) context.rcx,
                rdx = out(reg) context.rdx,
            );
        }
        
        // Simular otros registros con valores de ejemplo
        context.rsi = 0x123456789ABCDEF0;
        context.rdi = 0xFEDCBA9876543210;
        context.rbp = 0x1000;
        context.rsp = 0x2000;
        context.r8 = 0x1111;
        context.r9 = 0x2222;
        context.r10 = 0x3333;
        context.r11 = 0x4444;
        context.r12 = 0x5555;
        context.r13 = 0x6666;
        context.r14 = 0x7777;
        context.r15 = 0x8888;
        context.rip = 0x10000;
        context.rflags = 0x202; // IF flag set
        
        context
    }

    /// Convertir a string
    pub fn to_string(&self) -> String {
        format!(
            "RAX={:016X} RBX={:016X} RCX={:016X} RDX={:016X}\n\
             RSI={:016X} RDI={:016X} RBP={:016X} RSP={:016X}\n\
             R8 ={:016X} R9 ={:016X} R10={:016X} R11={:016X}\n\
             R12={:016X} R13={:016X} R14={:016X} R15={:016X}\n\
             RIP={:016X} RFLAGS={:016X}\n\
             CS={:04X} DS={:04X} ES={:04X} FS={:04X} GS={:04X} SS={:04X}",
            self.rax, self.rbx, self.rcx, self.rdx,
            self.rsi, self.rdi, self.rbp, self.rsp,
            self.r8, self.r9, self.r10, self.r11,
            self.r12, self.r13, self.r14, self.r15,
            self.rip, self.rflags,
            self.cs, self.ds, self.es, self.fs, self.gs, self.ss
        )
    }
}

/// Debugger principal
pub struct Debugger {
    pub state: DebuggerState,
    pub level: DebugLevel,
    pub breakpoints: Vec<Breakpoint>,
    pub current_context: Option<CpuContext>,
    pub step_count: AtomicUsize,
    pub breakpoint_hits: AtomicUsize,
    pub is_enabled: AtomicBool,
    pub serial_debug: bool,
    pub trace_calls: bool,
}

impl Debugger {
    /// Crear un nuevo debugger
    pub fn new() -> Self {
        Self {
            state: DebuggerState::Running,
            level: DebugLevel::Info,
            breakpoints: Vec::new(),
            current_context: None,
            step_count: AtomicUsize::new(0),
            breakpoint_hits: AtomicUsize::new(0),
            is_enabled: AtomicBool::new(false),
            serial_debug: false,
            trace_calls: false,
        }
    }

    /// Inicializar el debugger
    pub fn initialize(&mut self) -> bool {
        self.is_enabled.store(true, Ordering::SeqCst);
        self.state = DebuggerState::Running;
        
        // Log de inicialización
        crate::logging::info("debug", "Sistema de debug inicializado correctamente");
        
        true
    }

    /// Habilitar debug remoto via puerto serie
    pub fn enable_serial_debug(&mut self) {
        self.serial_debug = true;
        self.debug_print("Debug remoto habilitado via puerto serie");
    }

    /// Deshabilitar debug remoto
    pub fn disable_serial_debug(&mut self) {
        self.serial_debug = false;
        self.debug_print("Debug remoto deshabilitado");
    }

    /// Configurar nivel de debug
    pub fn set_level(&mut self, level: DebugLevel) {
        self.level = level;
        self.debug_print(&format!("Nivel de debug configurado a: {:?}", level));
    }

    /// Agregar breakpoint
    pub fn add_breakpoint(&mut self, address: usize, breakpoint_type: BreakpointType) -> usize {
        let id = self.breakpoints.len();
        let breakpoint = Breakpoint {
            id,
            address,
            breakpoint_type,
            enabled: true,
            hit_count: 0,
        };
        
        self.breakpoints.push(breakpoint);
        self.debug_print(&format!("Breakpoint {} agregado en 0x{:X}", id, address));
        id
    }

    /// Eliminar breakpoint
    pub fn remove_breakpoint(&mut self, id: usize) -> bool {
        if let Some(pos) = self.breakpoints.iter().position(|bp| bp.id == id) {
            self.breakpoints.remove(pos);
            self.debug_print(&format!("Breakpoint {} eliminado", id));
            true
        } else {
            false
        }
    }

    /// Verificar si hay breakpoint en dirección
    pub fn check_breakpoint(&mut self, address: usize) -> bool {
        for breakpoint in &mut self.breakpoints {
            if breakpoint.enabled && breakpoint.address == address {
                breakpoint.hit_count += 1;
                self.breakpoint_hits.fetch_add(1, Ordering::SeqCst);
                self.state = DebuggerState::Stopped;
                self.current_context = Some(CpuContext::capture());
                
                // Crear mensaje antes de llamar debug_print para evitar borrowing conflict
                let message = format!(
                    "Breakpoint {} hit en 0x{:X} (hit #{})",
                    breakpoint.id, address, breakpoint.hit_count
                );
                self.debug_print(&message);
                
                return true;
            }
        }
        false
    }

    /// Ejecutar paso a paso
    pub fn step(&mut self) {
        if self.state == DebuggerState::Stopped || self.state == DebuggerState::Paused {
            self.state = DebuggerState::Stepping;
            self.step_count.fetch_add(1, Ordering::SeqCst);
            self.debug_print("Ejecutando paso a paso");
        }
    }

    /// Continuar ejecución
    pub fn continue_execution(&mut self) {
        if self.state == DebuggerState::Stopped || self.state == DebuggerState::Paused {
            self.state = DebuggerState::Running;
            self.debug_print("Continuando ejecución");
        }
    }

    /// Pausar ejecución
    pub fn pause(&mut self) {
        if self.state == DebuggerState::Running {
            self.state = DebuggerState::Paused;
            self.current_context = Some(CpuContext::capture());
            self.debug_print("Ejecución pausada");
        }
    }

    /// Dump de memoria
    pub fn dump_memory(&self, address: usize, size: usize) -> String {
        let mut result = format!("Dump de memoria en 0x{:X} ({} bytes):\n", address, size);
        
        // Simular dump de memoria (en implementación real accedería a la memoria)
        for i in 0..(size / 16) {
            let offset = i * 16;
            let addr = address + offset;
            result.push_str(&format!("{:016X}: ", addr));
            
            // Simular datos de memoria
            for j in 0..16 {
                let byte = ((addr + j) & 0xFF) as u8;
                result.push_str(&format!("{:02X} ", byte));
            }
            
            result.push_str(" |");
            for j in 0..16 {
                let byte = ((addr + j) & 0xFF) as u8;
                let ch = if byte >= 32 && byte <= 126 { byte as char } else { '.' };
                result.push(ch);
            }
            result.push_str("|\n");
        }
        
        result
    }

    /// Dump de registros
    pub fn dump_registers(&self) -> String {
        if let Some(ref context) = self.current_context {
            context.to_string()
        } else {
            "No hay contexto de CPU disponible".to_string()
        }
    }

    /// Obtener información del debugger
    pub fn get_info(&self) -> String {
        format!(
            "Debugger: {} | Estado: {:?} | Nivel: {:?} | Breakpoints: {} | Steps: {} | Hits: {} | Serial: {} | Trace: {}",
            if self.is_enabled.load(Ordering::SeqCst) { "Activo" } else { "Inactivo" },
            self.state,
            self.level,
            self.breakpoints.len(),
            self.step_count.load(Ordering::SeqCst),
            self.breakpoint_hits.load(Ordering::SeqCst),
            if self.serial_debug { "Sí" } else { "No" },
            if self.trace_calls { "Sí" } else { "No" }
        )
    }

    /// Obtener estadísticas
    pub fn get_stats(&self) -> String {
        let enabled_breakpoints = self.breakpoints.iter().filter(|bp| bp.enabled).count();
        let total_hits: usize = self.breakpoints.iter().map(|bp| bp.hit_count).sum();
        
        format!(
            "Debug: {} | Estado: {:?} | Breakpoints: {}/{} | Steps: {} | Total hits: {} | Serial: {}",
            if self.is_enabled.load(Ordering::SeqCst) { "Activo" } else { "Inactivo" },
            self.state,
            enabled_breakpoints,
            self.breakpoints.len(),
            self.step_count.load(Ordering::SeqCst),
            total_hits,
            if self.serial_debug { "ON" } else { "OFF" }
        )
    }

    /// Imprimir mensaje de debug
    fn debug_print(&self, message: &str) {
        if self.serial_debug {
            // En implementación real, enviaría por puerto serie
            crate::logging::info("debug", message);
        }
    }
}

/// Instancia global del debugger
static DEBUGGER: Mutex<Option<Debugger>> = Mutex::new(None);

/// Inicializar el sistema de debug
pub fn init_debug() -> bool {
    let mut debugger_guard = DEBUGGER.lock();
    if debugger_guard.is_none() {
        let mut debugger = Debugger::new();
        if debugger.initialize() {
            *debugger_guard = Some(debugger);
            return true;
        }
    }
    false
}

/// Habilitar debug remoto
pub fn enable_serial_debug() {
    let mut debugger_guard = DEBUGGER.lock();
    if let Some(ref mut debugger) = *debugger_guard {
        debugger.enable_serial_debug();
    }
}

/// Configurar nivel de debug
pub fn set_debug_level(level: DebugLevel) {
    let mut debugger_guard = DEBUGGER.lock();
    if let Some(ref mut debugger) = *debugger_guard {
        debugger.set_level(level);
    }
}

/// Agregar breakpoint
pub fn add_breakpoint(address: usize, breakpoint_type: BreakpointType) -> Option<usize> {
    let mut debugger_guard = DEBUGGER.lock();
    if let Some(ref mut debugger) = *debugger_guard {
        Some(debugger.add_breakpoint(address, breakpoint_type))
    } else {
        None
    }
}

/// Eliminar breakpoint
pub fn remove_breakpoint(id: usize) -> bool {
    let mut debugger_guard = DEBUGGER.lock();
    if let Some(ref mut debugger) = *debugger_guard {
        debugger.remove_breakpoint(id)
    } else {
        false
    }
}

/// Verificar breakpoint
pub fn check_breakpoint(address: usize) -> bool {
    let mut debugger_guard = DEBUGGER.lock();
    if let Some(ref mut debugger) = *debugger_guard {
        debugger.check_breakpoint(address)
    } else {
        false
    }
}

/// Ejecutar paso a paso
pub fn step() {
    let mut debugger_guard = DEBUGGER.lock();
    if let Some(ref mut debugger) = *debugger_guard {
        debugger.step();
    }
}

/// Continuar ejecución
pub fn continue_execution() {
    let mut debugger_guard = DEBUGGER.lock();
    if let Some(ref mut debugger) = *debugger_guard {
        debugger.continue_execution();
    }
}

/// Pausar ejecución
pub fn pause() {
    let mut debugger_guard = DEBUGGER.lock();
    if let Some(ref mut debugger) = *debugger_guard {
        debugger.pause();
    }
}

/// Dump de memoria
pub fn dump_memory(address: usize, size: usize) -> String {
    let debugger_guard = DEBUGGER.lock();
    if let Some(ref debugger) = *debugger_guard {
        debugger.dump_memory(address, size)
    } else {
        String::from("Debugger no disponible")
    }
}

/// Dump de registros
pub fn dump_registers() -> String {
    let debugger_guard = DEBUGGER.lock();
    if let Some(ref debugger) = *debugger_guard {
        debugger.dump_registers()
    } else {
        String::from("Debugger no disponible")
    }
}

/// Obtener información del debugger
pub fn get_debug_info() -> String {
    let debugger_guard = DEBUGGER.lock();
    if let Some(ref debugger) = *debugger_guard {
        debugger.get_info()
    } else {
        String::from("Sistema de debug: No disponible")
    }
}

/// Obtener estadísticas del debugger
pub fn get_debug_stats() -> String {
    let debugger_guard = DEBUGGER.lock();
    if let Some(ref debugger) = *debugger_guard {
        debugger.get_stats()
    } else {
        String::from("Estadísticas de debug: No disponible")
    }
}

/// Verificar si el sistema de debug está disponible
pub fn is_debug_available() -> bool {
    let debugger_guard = DEBUGGER.lock();
    debugger_guard.is_some()
}

/// Macro para debug condicional
#[macro_export]
macro_rules! debug_print {
    ($level:expr, $($arg:tt)*) => {
        if crate::debug::is_debug_available() {
            let message = format!($($arg)*);
            crate::logging::info("debug", &message);
        }
    };
}

/// Macro para breakpoint
#[macro_export]
macro_rules! debug_breakpoint {
    () => {
        if crate::debug::is_debug_available() {
            // En implementación real, aquí se activaría el breakpoint
            crate::logging::info("debug", "Breakpoint hit");
        }
    };
}

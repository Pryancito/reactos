//! Sistema de Señales Completo
//!
//! Sistema de señales para comunicación entre procesos y manejo de eventos

use alloc::{vec::Vec, string::String, format, collections::BTreeMap};

/// Número de señal
pub type SignalNumber = u32;

/// Constantes de señales estándar
pub const SIGHUP: SignalNumber = 1;    // Hangup
pub const SIGINT: SignalNumber = 2;    // Interrupt (Ctrl+C)
pub const SIGQUIT: SignalNumber = 3;   // Quit
pub const SIGILL: SignalNumber = 4;    // Illegal instruction
pub const SIGTRAP: SignalNumber = 5;   // Trace/breakpoint trap
pub const SIGABRT: SignalNumber = 6;   // Abort
pub const SIGBUS: SignalNumber = 7;    // Bus error
pub const SIGFPE: SignalNumber = 8;    // Floating point exception
pub const SIGKILL: SignalNumber = 9;   // Kill (cannot be caught)
pub const SIGUSR1: SignalNumber = 10;  // User-defined signal 1
pub const SIGSEGV: SignalNumber = 11;  // Segmentation violation
pub const SIGUSR2: SignalNumber = 12;  // User-defined signal 2
pub const SIGPIPE: SignalNumber = 13;  // Broken pipe
pub const SIGALRM: SignalNumber = 14;  // Alarm clock
pub const SIGTERM: SignalNumber = 15;  // Termination
pub const SIGCHLD: SignalNumber = 17;  // Child status changed
pub const SIGCONT: SignalNumber = 18;  // Continue
pub const SIGSTOP: SignalNumber = 19;  // Stop (cannot be caught)
pub const SIGTSTP: SignalNumber = 20;  // Terminal stop
pub const SIGTTIN: SignalNumber = 21;  // Background read from tty
pub const SIGTTOU: SignalNumber = 22;  // Background write to tty
pub const SIGURG: SignalNumber = 23;   // Urgent condition
pub const SIGXCPU: SignalNumber = 24;  // CPU time limit exceeded
pub const SIGXFSZ: SignalNumber = 25;  // File size limit exceeded
pub const SIGVTALRM: SignalNumber = 26; // Virtual alarm clock
pub const SIGPROF: SignalNumber = 27;  // Profiling alarm clock
pub const SIGWINCH: SignalNumber = 28; // Window size change
pub const SIGIO: SignalNumber = 29;    // I/O now possible
pub const SIGPWR: SignalNumber = 30;   // Power failure
pub const SIGSYS: SignalNumber = 31;   // Bad system call

/// Acción de señal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalAction {
    Default,    // Acción por defecto del sistema
    Ignore,     // Ignorar la señal
    Handler,    // Llamar a un manejador personalizado
    Block,      // Bloquear la señal
}

/// Manejador de señal
#[derive(Debug, Clone)]
pub struct SignalHandler {
    pub action: SignalAction,
    pub handler_address: Option<usize>, // Dirección del manejador
    pub flags: SignalFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalFlags {
    pub sa_restart: bool,      // Reiniciar llamadas al sistema interrumpidas
    pub sa_nocldstop: bool,    // No generar SIGCHLD cuando los hijos se detienen
    pub sa_nocldwait: bool,    // No convertir hijos en zombies
    pub sa_nodefer: bool,      // No bloquear la señal durante la ejecución del manejador
    pub sa_nomask: bool,       // Alias para sa_nodefer
    pub sa_oneshot: bool,      // Restaurar acción por defecto después de la señal
    pub sa_onstack: bool,      // Usar stack alternativo
    pub sa_resethand: bool,    // Alias para sa_oneshot
    pub sa_siginfo: bool,      // Proporcionar información adicional
}

impl Default for SignalFlags {
    fn default() -> Self {
        Self {
            sa_restart: true,
            sa_nocldstop: false,
            sa_nocldwait: false,
            sa_nodefer: false,
            sa_nomask: false,
            sa_oneshot: false,
            sa_onstack: false,
            sa_resethand: false,
            sa_siginfo: false,
        }
    }
}

impl SignalHandler {
    pub fn new(action: SignalAction) -> Self {
        Self {
            action,
            handler_address: None,
            flags: SignalFlags::default(),
        }
    }

    pub fn with_handler(handler_address: usize, flags: SignalFlags) -> Self {
        Self {
            action: SignalAction::Handler,
            handler_address: Some(handler_address),
            flags,
        }
    }
}

/// Información de señal
#[derive(Debug, Clone)]
pub struct SignalInfo {
    pub signal: SignalNumber,
    pub pid: usize,           // PID del proceso que envió la señal
    pub uid: usize,           // UID del proceso que envió la señal
    pub code: i32,            // Código de la señal
    pub value: i32,           // Valor asociado
    pub timestamp: u64,       // Timestamp de la señal
    pub data: Vec<u8>,        // Datos adicionales
}

impl SignalInfo {
    pub fn new(signal: SignalNumber, pid: usize) -> Self {
        Self {
            signal,
            pid,
            uid: 0, // Por defecto
            code: 0,
            value: 0,
            timestamp: get_system_time(),
            data: Vec::new(),
        }
    }
}

/// Cola de señales pendientes
#[derive(Debug, Clone)]
pub struct SignalQueue {
    pub signals: Vec<SignalInfo>,
    pub max_size: usize,
}

impl SignalQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            signals: Vec::new(),
            max_size,
        }
    }

    pub fn add_signal(&mut self, signal_info: SignalInfo) -> bool {
        if self.signals.len() >= self.max_size {
            return false; // Cola llena
        }
        self.signals.push(signal_info);
        true
    }

    pub fn get_next_signal(&mut self) -> Option<SignalInfo> {
        if self.signals.is_empty() {
            None
        } else {
            Some(self.signals.remove(0))
        }
    }

    pub fn peek_signal(&self) -> Option<&SignalInfo> {
        self.signals.first()
    }

    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
    }

    pub fn len(&self) -> usize {
        self.signals.len()
    }

    pub fn clear(&mut self) {
        self.signals.clear();
    }
}

/// Máscara de señales
#[derive(Debug, Clone)]
pub struct SignalMask {
    pub blocked: u64,    // Señales bloqueadas (bitmap)
    pub pending: u64,    // Señales pendientes (bitmap)
    pub ignored: u64,    // Señales ignoradas (bitmap)
}

impl SignalMask {
    pub fn new() -> Self {
        Self {
            blocked: 0,
            pending: 0,
            ignored: 0,
        }
    }

    pub fn is_blocked(&self, signal: SignalNumber) -> bool {
        if signal > 64 { return false; }
        (self.blocked & (1 << (signal - 1))) != 0
    }

    pub fn is_pending(&self, signal: SignalNumber) -> bool {
        if signal > 64 { return false; }
        (self.pending & (1 << (signal - 1))) != 0
    }

    pub fn is_ignored(&self, signal: SignalNumber) -> bool {
        if signal > 64 { return false; }
        (self.ignored & (1 << (signal - 1))) != 0
    }

    pub fn block_signal(&mut self, signal: SignalNumber) {
        if signal <= 64 {
            self.blocked |= 1 << (signal - 1);
        }
    }

    pub fn unblock_signal(&mut self, signal: SignalNumber) {
        if signal <= 64 {
            self.blocked &= !(1 << (signal - 1));
        }
    }

    pub fn set_pending(&mut self, signal: SignalNumber) {
        if signal <= 64 {
            self.pending |= 1 << (signal - 1);
        }
    }

    pub fn clear_pending(&mut self, signal: SignalNumber) {
        if signal <= 64 {
            self.pending &= !(1 << (signal - 1));
        }
    }

    pub fn ignore_signal(&mut self, signal: SignalNumber) {
        if signal <= 64 {
            self.ignored |= 1 << (signal - 1);
        }
    }

    pub fn unignore_signal(&mut self, signal: SignalNumber) {
        if signal <= 64 {
            self.ignored &= !(1 << (signal - 1));
        }
    }
}

/// Contexto de señal
#[derive(Debug, Clone)]
pub struct SignalContext {
    pub pid: usize,
    pub handlers: BTreeMap<SignalNumber, SignalHandler>,
    pub mask: SignalMask,
    pub queue: SignalQueue,
    pub alt_stack: Option<usize>, // Stack alternativo para señales
    pub blocked_signals: u64,     // Señales temporalmente bloqueadas
}

impl SignalContext {
    pub fn new(pid: usize) -> Self {
        Self {
            pid,
            handlers: BTreeMap::new(),
            mask: SignalMask::new(),
            queue: SignalQueue::new(32), // Máximo 32 señales pendientes
            alt_stack: None,
            blocked_signals: 0,
        }
    }

    pub fn set_handler(&mut self, signal: SignalNumber, handler: SignalHandler) {
        self.handlers.insert(signal, handler);
    }

    pub fn get_handler(&self, signal: SignalNumber) -> Option<&SignalHandler> {
        self.handlers.get(&signal)
    }

    pub fn remove_handler(&mut self, signal: SignalNumber) {
        self.handlers.remove(&signal);
    }

    pub fn send_signal(&mut self, signal_info: SignalInfo) -> bool {
        // Verificar si la señal está bloqueada
        if self.mask.is_blocked(signal_info.signal) {
            self.mask.set_pending(signal_info.signal);
            return true;
        }

        // Verificar si la señal está siendo ignorada
        if self.mask.is_ignored(signal_info.signal) {
            return true;
        }

        // Agregar a la cola de señales
        self.queue.add_signal(signal_info)
    }

    pub fn process_pending_signals(&mut self) -> Vec<SignalInfo> {
        let mut processed = Vec::new();
        
        // Procesar señales en la cola
        while let Some(signal_info) = self.queue.get_next_signal() {
            processed.push(signal_info);
        }

        // Procesar señales pendientes en la máscara
        for signal_num in 1..=64 {
            if self.mask.is_pending(signal_num) && !self.mask.is_blocked(signal_num) {
                self.mask.clear_pending(signal_num);
                let signal_info = SignalInfo::new(signal_num, 0);
                processed.push(signal_info);
            }
        }

        processed
    }
}

/// Gestor de señales del sistema
#[derive(Debug, Clone)]
pub struct SignalManager {
    pub contexts: BTreeMap<usize, SignalContext>, // PID -> SignalContext
    pub global_handlers: BTreeMap<SignalNumber, SignalHandler>,
    pub signal_queue: Vec<(usize, SignalInfo)>, // (target_pid, signal_info)
    pub statistics: SignalStatistics,
}

#[derive(Debug, Clone)]
pub struct SignalStatistics {
    pub signals_sent: u64,
    pub signals_delivered: u64,
    pub signals_blocked: u64,
    pub signals_ignored: u64,
    pub handler_calls: u64,
    pub errors: u64,
}

impl Default for SignalStatistics {
    fn default() -> Self {
        Self {
            signals_sent: 0,
            signals_delivered: 0,
            signals_blocked: 0,
            signals_ignored: 0,
            handler_calls: 0,
            errors: 0,
        }
    }
}

impl SignalManager {
    pub fn new() -> Self {
        let mut manager = Self {
            contexts: BTreeMap::new(),
            global_handlers: BTreeMap::new(),
            signal_queue: Vec::new(),
            statistics: SignalStatistics::default(),
        };

        // Configurar manejadores por defecto
        manager.setup_default_handlers();
        manager
    }

    fn setup_default_handlers(&mut self) {
        // Señales que terminan el proceso por defecto
        let terminate_signals = [SIGTERM, SIGQUIT, SIGILL, SIGABRT, SIGFPE, SIGSEGV, SIGBUS, SIGSYS];
        for &signal in &terminate_signals {
            self.global_handlers.insert(signal, SignalHandler::new(SignalAction::Default));
        }

        // Señales que no se pueden atrapar
        let uncatchable_signals = [SIGKILL, SIGSTOP];
        for &signal in &uncatchable_signals {
            self.global_handlers.insert(signal, SignalHandler::new(SignalAction::Default));
        }

        // Señales que se ignoran por defecto
        let ignore_signals = [SIGCHLD, SIGURG, SIGWINCH];
        for &signal in &ignore_signals {
            self.global_handlers.insert(signal, SignalHandler::new(SignalAction::Ignore));
        }
    }

    pub fn create_context(&mut self, pid: usize) {
        let context = SignalContext::new(pid);
        self.contexts.insert(pid, context);
    }

    pub fn remove_context(&mut self, pid: usize) {
        self.contexts.remove(&pid);
    }

    pub fn send_signal(&mut self, target_pid: usize, signal: SignalNumber, sender_pid: usize) -> bool {
        self.statistics.signals_sent += 1;

        // Verificar si el proceso objetivo existe
        if let Some(context) = self.contexts.get_mut(&target_pid) {
            let signal_info = SignalInfo::new(signal, sender_pid);
            
            if context.send_signal(signal_info) {
                self.statistics.signals_delivered += 1;
                true
            } else {
                self.statistics.signals_blocked += 1;
                false
            }
        } else {
            self.statistics.errors += 1;
            false
        }
    }

    pub fn send_signal_to_group(&mut self, _group_id: usize, signal: SignalNumber, sender_pid: usize) -> usize {
        let mut sent_count = 0;
        
        for (pid, context) in &mut self.contexts {
            // Simular verificación de grupo (simplificado)
            if *pid != sender_pid {
                let signal_info = SignalInfo::new(signal, sender_pid);
                if context.send_signal(signal_info) {
                    sent_count += 1;
                }
            }
        }
        
        sent_count
    }

    pub fn send_signal_to_all(&mut self, signal: SignalNumber, sender_pid: usize) -> usize {
        self.send_signal_to_group(0, signal, sender_pid) // Grupo 0 = todos
    }

    pub fn set_signal_handler(&mut self, pid: usize, signal: SignalNumber, handler: SignalHandler) -> bool {
        if let Some(context) = self.contexts.get_mut(&pid) {
            context.set_handler(signal, handler);
            true
        } else {
            false
        }
    }

    pub fn block_signal(&mut self, pid: usize, signal: SignalNumber) -> bool {
        if let Some(context) = self.contexts.get_mut(&pid) {
            context.mask.block_signal(signal);
            true
        } else {
            false
        }
    }

    pub fn unblock_signal(&mut self, pid: usize, signal: SignalNumber) -> bool {
        if let Some(context) = self.contexts.get_mut(&pid) {
            context.mask.unblock_signal(signal);
            true
        } else {
            false
        }
    }

    pub fn ignore_signal(&mut self, pid: usize, signal: SignalNumber) -> bool {
        if let Some(context) = self.contexts.get_mut(&pid) {
            context.mask.ignore_signal(signal);
            true
        } else {
            false
        }
    }

    pub fn process_signals(&mut self, pid: usize) -> Vec<SignalInfo> {
        if let Some(context) = self.contexts.get_mut(&pid) {
            let processed = context.process_pending_signals();
            self.statistics.handler_calls += processed.len() as u64;
            processed
        } else {
            Vec::new()
        }
    }

    pub fn get_signal_name(&self, signal: SignalNumber) -> &'static str {
        match signal {
            SIGHUP => "SIGHUP",
            SIGINT => "SIGINT",
            SIGQUIT => "SIGQUIT",
            SIGILL => "SIGILL",
            SIGTRAP => "SIGTRAP",
            SIGABRT => "SIGABRT",
            SIGBUS => "SIGBUS",
            SIGFPE => "SIGFPE",
            SIGKILL => "SIGKILL",
            SIGUSR1 => "SIGUSR1",
            SIGSEGV => "SIGSEGV",
            SIGUSR2 => "SIGUSR2",
            SIGPIPE => "SIGPIPE",
            SIGALRM => "SIGALRM",
            SIGTERM => "SIGTERM",
            SIGCHLD => "SIGCHLD",
            SIGCONT => "SIGCONT",
            SIGSTOP => "SIGSTOP",
            SIGTSTP => "SIGTSTP",
            SIGTTIN => "SIGTTIN",
            SIGTTOU => "SIGTTOU",
            SIGURG => "SIGURG",
            SIGXCPU => "SIGXCPU",
            SIGXFSZ => "SIGXFSZ",
            SIGVTALRM => "SIGVTALRM",
            SIGPROF => "SIGPROF",
            SIGWINCH => "SIGWINCH",
            SIGIO => "SIGIO",
            SIGPWR => "SIGPWR",
            SIGSYS => "SIGSYS",
            _ => "UNKNOWN",
        }
    }

    pub fn get_context_info(&self, pid: usize) -> Option<String> {
        if let Some(context) = self.contexts.get(&pid) {
            Some(format!(
                "PID: {} | Señales pendientes: {} | Manejadores: {} | Bloqueadas: {}",
                pid,
                context.queue.len(),
                context.handlers.len(),
                count_bits(context.mask.blocked)
            ))
        } else {
            None
        }
    }

    pub fn get_statistics(&self) -> String {
        format!(
            "Enviadas: {} | Entregadas: {} | Bloqueadas: {} | Ignoradas: {} | Manejadores: {} | Errores: {}",
            self.statistics.signals_sent,
            self.statistics.signals_delivered,
            self.statistics.signals_blocked,
            self.statistics.signals_ignored,
            self.statistics.handler_calls,
            self.statistics.errors
        )
    }

    pub fn get_info(&self) -> String {
        format!(
            "Gestor de Señales - Contextos: {} | Cola: {} | Estadísticas: {}",
            self.contexts.len(),
            self.signal_queue.len(),
            self.statistics.signals_sent
        )
    }
}

// Funciones auxiliares
fn get_system_time() -> u64 {
    // Simulación simple de tiempo del sistema
    1234567890
}

fn count_bits(mut value: u64) -> usize {
    let mut count = 0;
    while value != 0 {
        count += (value & 1) as usize;
        value >>= 1;
    }
    count
}

// Gestor global de señales
use spin::Mutex;

pub static SIGNAL_MANAGER: Mutex<Option<SignalManager>> = Mutex::new(None);

/// Inicializar el gestor de señales
pub fn init_signal_manager() {
    let mut manager = SIGNAL_MANAGER.lock();
    *manager = Some(SignalManager::new());
    crate::logging::info("signals", "Gestor de señales inicializado");
}

/// Obtener información del gestor de señales
pub fn get_signal_manager_info() -> String {
    if let Some(ref manager) = *SIGNAL_MANAGER.lock() {
        manager.get_info()
    } else {
        String::from("Gestor de señales no inicializado")
    }
}

/// Obtener estadísticas del gestor de señales
pub fn get_signal_manager_stats() -> String {
    if let Some(ref manager) = *SIGNAL_MANAGER.lock() {
        manager.get_statistics()
    } else {
        String::from("Gestor de señales no inicializado")
    }
}

/// Enviar señal a un proceso
pub fn send_signal(target_pid: usize, signal: SignalNumber, sender_pid: usize) -> bool {
    let mut manager = SIGNAL_MANAGER.lock();
    if let Some(ref mut sm) = *manager {
        sm.send_signal(target_pid, signal, sender_pid)
    } else {
        false
    }
}

/// Enviar señal a un grupo de procesos
pub fn send_signal_to_group(group_id: usize, signal: SignalNumber, sender_pid: usize) -> usize {
    let mut manager = SIGNAL_MANAGER.lock();
    if let Some(ref mut sm) = *manager {
        sm.send_signal_to_group(group_id, signal, sender_pid)
    } else {
        0
    }
}

/// Crear contexto de señales para un proceso
pub fn create_signal_context(pid: usize) {
    let mut manager = SIGNAL_MANAGER.lock();
    if let Some(ref mut sm) = *manager {
        sm.create_context(pid);
    }
}

/// Eliminar contexto de señales de un proceso
pub fn remove_signal_context(pid: usize) {
    let mut manager = SIGNAL_MANAGER.lock();
    if let Some(ref mut sm) = *manager {
        sm.remove_context(pid);
    }
}

/// Procesar señales pendientes para un proceso
pub fn process_pending_signals(pid: usize) -> Vec<SignalInfo> {
    let mut manager = SIGNAL_MANAGER.lock();
    if let Some(ref mut sm) = *manager {
        sm.process_signals(pid)
    } else {
        Vec::new()
    }
}

/// Obtener nombre de señal
pub fn get_signal_name(signal: SignalNumber) -> &'static str {
    let manager = SIGNAL_MANAGER.lock();
    if let Some(ref sm) = *manager {
        sm.get_signal_name(signal)
    } else {
        "UNKNOWN"
    }
}

/// Verificar si el gestor de señales está disponible
pub fn is_signal_manager_available() -> bool {
    let manager = SIGNAL_MANAGER.lock();
    manager.is_some()
}

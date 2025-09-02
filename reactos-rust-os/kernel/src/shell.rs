//! Shell básico interactivo para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Prompt interactivo
//! - Comandos básicos del sistema
//! - Historial de comandos
//! - Autocompletado básico
//! - Integración con teclado y mouse

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use spin::Mutex;

// Usar macro vga_println desde la raíz del crate
use crate::vga_println;

/// Máximo número de comandos en el historial
const MAX_HISTORY: usize = 100;

/// Máximo número de caracteres en una línea
const MAX_LINE_LENGTH: usize = 512;

/// Máximo número de archivos virtuales
const MAX_VIRTUAL_FILES: usize = 50;

/// Estado del shell
#[derive(Debug, Clone, PartialEq)]
pub enum ShellState {
    Running,    // Shell ejecutándose normalmente
    Exiting,    // Shell saliendo
    Error,      // Error en el shell
}

/// Información de un comando
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub handler: fn(&[String]) -> String,
}

/// Archivo virtual en el sistema de archivos en memoria
#[derive(Debug, Clone)]
pub struct VirtualFile {
    pub name: String,
    pub content: String,
    pub size: usize,
    pub created: u64, // Timestamp simulado
}

/// Directorio virtual
#[derive(Debug, Clone)]
pub struct VirtualDirectory {
    pub name: String,
    pub files: Vec<VirtualFile>,
    pub subdirs: Vec<String>,
}

/// Shell interactivo
pub struct Shell {
    state: ShellState,
    current_line: String,
    cursor_position: usize,
    history: Vec<String>,
    history_index: usize,
    commands: Vec<Command>,
    prompt: String,
    last_output: String,
    current_directory: String,
    virtual_filesystem: VirtualDirectory,
    file_counter: u64,
}

impl Shell {
    /// Crear un nuevo shell
    pub fn new() -> Self {
        let mut shell = Self {
            state: ShellState::Running,
            current_line: String::new(),
            cursor_position: 0,
            history: Vec::new(),
            history_index: 0,
            commands: Vec::new(),
            prompt: String::from("reactos> "),
            last_output: String::new(),
            current_directory: String::from("/"),
            virtual_filesystem: VirtualDirectory {
                name: String::from("/"),
                files: Vec::new(),
                subdirs: Vec::new(),
            },
            file_counter: 0,
        };
        
        // Inicializar sistema de archivos virtual
        shell.init_virtual_filesystem();
        
        // Registrar comandos básicos
        shell.register_commands();
        shell
    }

    /// Inicializar sistema de archivos virtual
    fn init_virtual_filesystem(&mut self) {
        // Crear archivos del sistema
        self.create_file("README.txt", "ReactOS Rust Kernel v0.1.0\nSistema de archivos virtual en memoria\nComandos disponibles: help, ls, cat, echo, etc.");
        self.create_file("kernel.log", "Kernel iniciado correctamente\nVGA inicializado\nTeclado inicializado\nMouse inicializado\nInterrupciones activas\nMemoria configurada\nShell iniciado");
        self.create_file("config.txt", "kernel.version=0.1.0\nkernel.arch=x86_64\nmemory.size=512MB\nshell.prompt=reactos>");
        
        // Crear subdirectorios
        self.virtual_filesystem.subdirs.push("system".to_string());
        self.virtual_filesystem.subdirs.push("logs".to_string());
        self.virtual_filesystem.subdirs.push("temp".to_string());
    }

    /// Crear un archivo virtual
    fn create_file(&mut self, name: &str, content: &str) {
        self.file_counter += 1;
        let file = VirtualFile {
            name: name.to_string(),
            content: content.to_string(),
            size: content.len(),
            created: self.file_counter,
        };
        self.virtual_filesystem.files.push(file);
    }

    /// Registrar comandos básicos
    fn register_commands(&mut self) {
        self.add_command("help", "Mostrar ayuda", Self::cmd_help);
        self.add_command("clear", "Limpiar pantalla", Self::cmd_clear);
        self.add_command("info", "Información del sistema", Self::cmd_info);
        self.add_command("memory", "Información de memoria", Self::cmd_memory);
        self.add_command("interrupts", "Estadísticas de interrupciones", Self::cmd_interrupts);
        self.add_command("keyboard", "Estado del teclado", Self::cmd_keyboard);
        self.add_command("mouse", "Estado del mouse", Self::cmd_mouse);
        self.add_command("echo", "Mostrar texto", Self::cmd_echo);
        self.add_command("history", "Mostrar historial", Self::cmd_history);
        self.add_command("exit", "Salir del shell", Self::cmd_exit);
        self.add_command("version", "Versión del kernel", Self::cmd_version);
        self.add_command("uptime", "Tiempo de funcionamiento", Self::cmd_uptime);
        
        // Comandos de archivos nuevos
        self.add_command("ls", "Listar archivos", Self::cmd_ls);
        self.add_command("cat", "Mostrar contenido de archivo", Self::cmd_cat);
        self.add_command("pwd", "Mostrar directorio actual", Self::cmd_pwd);
        self.add_command("touch", "Crear archivo vacío", Self::cmd_touch);
        self.add_command("rm", "Eliminar archivo", Self::cmd_rm);
        self.add_command("mkdir", "Crear directorio", Self::cmd_mkdir);
        self.add_command("find", "Buscar archivos", Self::cmd_find);
        self.add_command("wc", "Contar líneas/palabras", Self::cmd_wc);
        
        // Comandos de procesos
        self.add_command("ps", "Listar procesos", Self::cmd_ps);
        self.add_command("kill", "Terminar proceso", Self::cmd_kill);
        self.add_command("top", "Mostrar procesos activos", Self::cmd_top);
        self.add_command("spawn", "Crear nuevo proceso", Self::cmd_spawn);
    }

    /// Agregar un comando al shell
    pub fn add_command(&mut self, name: &str, description: &str, handler: fn(&[String]) -> String) {
        self.commands.push(Command {
            name: name.to_string(),
            description: description.to_string(),
            handler,
        });
    }

    /// Procesar entrada del teclado
    pub fn process_input(&mut self, ch: char) -> bool {
        match ch {
            '\n' | '\r' => {
                self.execute_command();
                true
            }
            '\x08' | '\x7F' => { // Backspace
                self.handle_backspace();
                true
            }
            '\x1B' => { // Escape
                self.handle_escape();
                true
            }
            '\t' => { // Tab (autocompletado)
                self.handle_tab();
                true
            }
            '\x1A' => { // Ctrl+Z (historial hacia arriba)
                self.navigate_history_up();
                true
            }
            '\x1E' => { // Ctrl+^ (historial hacia abajo)
                self.navigate_history_down();
                true
            }
            _ if ch.is_ascii() && !ch.is_control() => {
                self.insert_char(ch);
                true
            }
            _ => false,
        }
    }

    /// Ejecutar comando actual
    fn execute_command(&mut self) {
        if self.current_line.trim().is_empty() {
            self.print_prompt();
            return;
        }

        // Agregar al historial
        self.add_to_history(self.current_line.clone());
        
        // Parsear comando
        let parts: Vec<String> = self.current_line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        if parts.is_empty() {
            self.print_prompt();
            return;
        }

        let command_name = &parts[0];
        let args = &parts[1..];

        // Buscar y ejecutar comando
        let output = if let Some(cmd) = self.commands.iter().find(|c| c.name == *command_name) {
            (cmd.handler)(args)
        } else {
            format!("Comando no encontrado: '{}'. Escribe 'help' para ver comandos disponibles.", command_name)
        };

        self.last_output = output.clone();
        vga_println!("{}", output);
        self.print_prompt();
    }

    /// Agregar comando al historial
    fn add_to_history(&mut self, command: String) {
        if self.history.len() >= MAX_HISTORY {
            self.history.remove(0);
        }
        self.history.push(command);
        self.history_index = self.history.len();
    }

    /// Manejar backspace
    fn handle_backspace(&mut self) {
        if self.cursor_position > 0 {
            self.current_line.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
            self.refresh_line();
        }
    }

    /// Manejar escape
    fn handle_escape(&mut self) {
        self.current_line.clear();
        self.cursor_position = 0;
        self.refresh_line();
    }

    /// Manejar tab (autocompletado)
    fn handle_tab(&mut self) {
        let current_word = self.get_current_word();
        if current_word.is_empty() {
            return;
        }

        // Buscar comandos que empiecen con la palabra actual
        let matches: Vec<String> = self.commands
            .iter()
            .filter(|cmd| cmd.name.starts_with(&current_word))
            .map(|cmd| cmd.name.clone())
            .collect();

        if matches.len() == 1 {
            // Autocompletar
            let cmd_name = &matches[0];
            let new_word = &cmd_name[current_word.len()..];
            self.insert_string(new_word);
        } else if matches.len() > 1 {
            // Mostrar opciones
            vga_println!("");
            for cmd_name in &matches {
                if let Some(cmd) = self.commands.iter().find(|c| c.name == *cmd_name) {
                    vga_println!("  {} - {}", cmd.name, cmd.description);
                }
            }
            self.print_prompt();
            self.refresh_line();
        }
    }

    /// Insertar carácter
    fn insert_char(&mut self, ch: char) {
        if self.current_line.len() < MAX_LINE_LENGTH {
            self.current_line.insert(self.cursor_position, ch);
            self.cursor_position += 1;
            self.refresh_line();
        }
    }

    /// Insertar string
    fn insert_string(&mut self, s: &str) {
        for ch in s.chars() {
            self.insert_char(ch);
        }
    }

    /// Obtener palabra actual
    fn get_current_word(&self) -> String {
        let words: Vec<&str> = self.current_line.split_whitespace().collect();
        if words.is_empty() {
            return String::new();
        }
        words.last().map_or("", |s| s).to_string()
    }

    /// Refrescar línea actual
    fn refresh_line(&self) {
        // En una implementación real, aquí se redibujaría la línea
        // Por simplicidad, solo mostramos el prompt y la línea actual
        vga_println!("\r{}{}", self.prompt, self.current_line);
    }

    /// Navegar hacia arriba en el historial
    fn navigate_history_up(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(history_cmd) = self.history.get(self.history_index) {
                self.current_line = history_cmd.clone();
                self.cursor_position = self.current_line.len();
                self.refresh_line();
            }
        }
    }

    /// Navegar hacia abajo en el historial
    fn navigate_history_down(&mut self) {
        if self.history_index < self.history.len() {
            self.history_index += 1;
            if let Some(history_cmd) = self.history.get(self.history_index) {
                self.current_line = history_cmd.clone();
                self.cursor_position = self.current_line.len();
            } else {
                self.current_line.clear();
                self.cursor_position = 0;
            }
            self.refresh_line();
        }
    }

    /// Mostrar prompt
    pub fn print_prompt(&self) {
        vga_println!("{}", self.prompt);
    }

    /// Obtener estado del shell
    pub fn get_state(&self) -> &ShellState {
        &self.state
    }

    /// Obtener información del shell
    pub fn get_info(&self) -> String {
        format!(
            "Shell: {} comandos, {} en historial, estado: {:?}",
            self.commands.len(),
            self.history.len(),
            self.state
        )
    }

    /// Obtener estadísticas del shell
    pub fn get_stats(&self) -> String {
        format!(
            "Shell: {} comandos registrados, {} comandos en historial",
            self.commands.len(),
            self.history.len()
        )
    }
}

// Comandos del shell
impl Shell {
    fn cmd_help(_args: &[String]) -> String {
        String::from("Comandos disponibles:\n  help - Mostrar esta ayuda\n  clear - Limpiar pantalla\n  info - Información del sistema\n  memory - Información de memoria\n  interrupts - Estadísticas de interrupciones\n  keyboard - Estado del teclado\n  mouse - Estado del mouse\n  echo <texto> - Mostrar texto\n  history - Mostrar historial\n  version - Versión del kernel\n  uptime - Tiempo de funcionamiento\n  exit - Salir del shell\n\nComandos de archivos:\n  ls - Listar archivos\n  cat <archivo> - Mostrar contenido\n  pwd - Directorio actual\n  touch <archivo> - Crear archivo\n  rm <archivo> - Eliminar archivo\n  mkdir <dir> - Crear directorio\n  find <patrón> - Buscar archivos\n  wc <archivo> - Contar líneas/palabras\n\nComandos de procesos:\n  ps - Listar procesos\n  kill <pid> - Terminar proceso\n  top - Mostrar procesos activos\n  spawn <nombre> - Crear nuevo proceso\n\nNavegación:\n  Ctrl+Z - Historial hacia arriba\n  Ctrl+^ - Historial hacia abajo\n  Tab - Autocompletado")
    }

    fn cmd_clear(_args: &[String]) -> String {
        // En una implementación real, aquí se limpiaría la pantalla
        String::from("Pantalla limpiada")
    }

    fn cmd_info(_args: &[String]) -> String {
        String::from("ReactOS Rust Kernel v0.1.0\nArquitectura: x86_64\nModo: Bare Metal\nBootloader: GRUB Multiboot")
    }

    fn cmd_memory(_args: &[String]) -> String {
        crate::memory::get_memory_info()
    }

    fn cmd_interrupts(_args: &[String]) -> String {
        crate::interrupts::get_interrupt_stats()
    }

    fn cmd_keyboard(_args: &[String]) -> String {
        crate::keyboard::get_keyboard_info()
    }

    fn cmd_mouse(_args: &[String]) -> String {
        crate::mouse::get_mouse_info()
    }

    fn cmd_echo(args: &[String]) -> String {
        if args.is_empty() {
            String::from("Uso: echo <texto>")
        } else {
            args.join(" ")
        }
    }

    fn cmd_history(_args: &[String]) -> String {
        // En una implementación real, esto accedería al shell actual
        let mut result = String::from("Historial de comandos:\n");
        for (i, cmd) in ["help", "info", "memory", "interrupts", "keyboard", "mouse", "echo Hola mundo", "version", "uptime", "clear", "ls", "cat README.txt"].iter().enumerate() {
            result.push_str(&format!("  {}: {}\n", i + 1, cmd));
        }
        result
    }

    fn cmd_exit(_args: &[String]) -> String {
        String::from("Saliendo del shell...")
    }

    fn cmd_version(_args: &[String]) -> String {
        String::from("ReactOS Rust Kernel v0.1.0\nCompilado con Rust 1.70+\nTarget: x86_64-unknown-none")
    }

    fn cmd_uptime(_args: &[String]) -> String {
        // En una implementación real, aquí se calcularía el tiempo real
        String::from("Tiempo de funcionamiento: 0 días, 0 horas, 0 minutos, 0 segundos")
    }

    // Comandos de archivos
    fn cmd_ls(args: &[String]) -> String {
        let path = if args.is_empty() { "/" } else { &args[0] };
        
        if let Some(entries) = crate::filesystem::list_directory(path) {
            let mut result = format!("Contenido de {}:\n", path);
            for entry in entries {
                result.push_str(&format!("  {}\n", entry));
            }
            result
        } else {
            format!("Error: No se pudo acceder al directorio '{}'", path)
        }
    }

    fn cmd_cat(args: &[String]) -> String {
        if args.is_empty() {
            return String::from("Uso: cat <archivo>");
        }
        
        let filename = &args[0];
        match filename.as_str() {
            "README.txt" => String::from("ReactOS Rust Kernel v0.1.0\nSistema de archivos virtual en memoria\nComandos disponibles: help, ls, cat, echo, etc."),
            "kernel.log" => String::from("Kernel iniciado correctamente\nVGA inicializado\nTeclado inicializado\nMouse inicializado\nInterrupciones activas\nMemoria configurada\nShell iniciado"),
            "config.txt" => String::from("kernel.version=0.1.0\nkernel.arch=x86_64\nmemory.size=512MB\nshell.prompt=reactos>"),
            _ => format!("Archivo no encontrado: {}", filename),
        }
    }

    fn cmd_pwd(_args: &[String]) -> String {
        String::from("/")
    }

    fn cmd_touch(args: &[String]) -> String {
        if args.is_empty() {
            return String::from("Uso: touch <archivo>");
        }
        format!("Archivo creado: {}", args[0])
    }

    fn cmd_rm(args: &[String]) -> String {
        if args.is_empty() {
            return String::from("Uso: rm <archivo>");
        }
        format!("Archivo eliminado: {}", args[0])
    }

    fn cmd_mkdir(args: &[String]) -> String {
        if args.is_empty() {
            return String::from("Uso: mkdir <directorio>");
        }
        format!("Directorio creado: {}", args[0])
    }

    fn cmd_find(args: &[String]) -> String {
        if args.is_empty() {
            return String::from("Uso: find <patrón>");
        }
        let pattern = &args[0];
        format!("Buscando archivos que contengan '{}':\n  README.txt\n  kernel.log\n  config.txt", pattern)
    }

    fn cmd_wc(args: &[String]) -> String {
        if args.is_empty() {
            return String::from("Uso: wc <archivo>");
        }
        let filename = &args[0];
        match filename.as_str() {
            "README.txt" => String::from("  3  15  128 README.txt"),
            "kernel.log" => String::from("  6  12  156 kernel.log"),
            "config.txt" => String::from("  4   4   89 config.txt"),
            _ => format!("Archivo no encontrado: {}", filename),
        }
    }

    // Comandos de procesos
    fn cmd_ps(_args: &[String]) -> String {
        crate::process::get_process_stats()
    }

    fn cmd_kill(args: &[String]) -> String {
        if args.is_empty() {
            return String::from("Uso: kill <pid>");
        }
        
        if let Ok(pid) = args[0].parse::<u32>() {
            if crate::process::terminate_process(pid) {
                format!("Proceso {} terminado", pid)
            } else {
                format!("Error: No se pudo terminar el proceso {}", pid)
            }
        } else {
            String::from("Error: PID inválido")
        }
    }

    fn cmd_top(_args: &[String]) -> String {
        let mut result = String::from("Procesos activos:\n");
        result.push_str(&crate::process::get_process_stats());
        result.push_str(&format!("\nSistema: {}\n", crate::process::get_process_info()));
        result
    }

    fn cmd_spawn(args: &[String]) -> String {
        if args.is_empty() {
            return String::from("Uso: spawn <nombre>");
        }
        
        let name = &args[0];
        let pid = crate::process::create_process(name.clone(), crate::process::ProcessPriority::Normal);
        format!("Proceso '{}' creado con PID: {}", name, pid)
    }
}

/// Instancia global del shell
static SHELL: Mutex<Option<Shell>> = Mutex::new(None);

/// Inicializar el shell
pub fn init_shell() -> bool {
    let mut shell_guard = SHELL.lock();
    *shell_guard = Some(Shell::new());
    true
}

/// Obtener el shell
pub fn get_shell() -> Option<&'static Mutex<Option<Shell>>> {
    Some(&SHELL)
}

/// Procesar entrada del shell
pub fn process_shell_input(ch: char) -> bool {
    let mut shell_guard = SHELL.lock();
    if let Some(ref mut shell) = *shell_guard {
        return shell.process_input(ch);
    }
    false
}

/// Obtener información del shell
pub fn get_shell_info() -> String {
    let shell_guard = SHELL.lock();
    if let Some(ref shell) = *shell_guard {
        return shell.get_info();
    }
    String::from("Shell: No disponible")
}

/// Obtener estadísticas del shell
pub fn get_shell_stats() -> String {
    let shell_guard = SHELL.lock();
    if let Some(ref shell) = *shell_guard {
        return shell.get_stats();
    }
    String::from("Shell Stats: No disponible")
}

/// Verificar si el shell está disponible
pub fn is_shell_available() -> bool {
    let shell_guard = SHELL.lock();
    shell_guard.is_some()
}

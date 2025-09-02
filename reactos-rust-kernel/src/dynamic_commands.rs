//! Sistema de Comandos Dinámico
//!
//! Genera comandos automáticamente basándose en los módulos del kernel disponibles

use alloc::{vec, vec::Vec, string::{String, ToString}, format, collections::BTreeMap};

/// Tipo de comando
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandType {
    System,     // Comando del sistema
    Module,     // Comando de módulo
    Application, // Comando de aplicación
    Utility,    // Utilidad del sistema
    Network,    // Comando de red
    File,       // Comando de archivos
    Audio,      // Comando de audio
    Gui,        // Comando de interfaz gráfica
}

/// Información de un comando
#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub command_type: CommandType,
    pub module_name: String,
    pub function_name: String,
    pub parameters: Vec<String>,
    pub examples: Vec<String>,
    pub is_available: bool,
}

impl CommandInfo {
    pub fn new(
        name: String,
        description: String,
        command_type: CommandType,
        module_name: String,
        function_name: String,
    ) -> Self {
        Self {
            name,
            description,
            command_type,
            module_name,
            function_name,
            parameters: Vec::new(),
            examples: Vec::new(),
            is_available: true,
        }
    }

    pub fn add_parameter(&mut self, param: String) {
        self.parameters.push(param);
    }

    pub fn add_example(&mut self, example: String) {
        self.examples.push(example);
    }

    pub fn get_help(&self) -> String {
        let mut help = format!("{} - {}\n", self.name, self.description);
        help.push_str(&format!("Tipo: {:?}\n", self.command_type));
        help.push_str(&format!("Módulo: {}\n", self.module_name));
        
        if !self.parameters.is_empty() {
            help.push_str("Parámetros:\n");
            for param in &self.parameters {
                help.push_str(&format!("  {}\n", param));
            }
        }
        
        if !self.examples.is_empty() {
            help.push_str("Ejemplos:\n");
            for example in &self.examples {
                help.push_str(&format!("  {}\n", example));
            }
        }
        
        help
    }
}

/// Generador de comandos dinámico
#[derive(Debug, Clone)]
pub struct DynamicCommandGenerator {
    pub commands: BTreeMap<String, CommandInfo>,
    pub module_commands: BTreeMap<String, Vec<String>>,
    pub is_initialized: bool,
    pub total_commands: u32,
    pub available_commands: u32,
}

impl DynamicCommandGenerator {
    pub fn new() -> Self {
        Self {
            commands: BTreeMap::new(),
            module_commands: BTreeMap::new(),
            is_initialized: false,
            total_commands: 0,
            available_commands: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.generate_system_commands();
        self.generate_module_commands();
        self.generate_application_commands();
        self.generate_utility_commands();
        self.generate_network_commands();
        self.generate_file_commands();
        self.generate_audio_commands();
        self.generate_gui_commands();
        
        self.is_initialized = true;
        self.total_commands = self.commands.len() as u32;
        self.available_commands = self.commands.values().filter(|c| c.is_available).count() as u32;
    }

    fn generate_system_commands(&mut self) {
        // Comandos básicos del sistema
        let mut ps_cmd = CommandInfo::new(
            "ps".to_string(),
            "Listar procesos en ejecución".to_string(),
            CommandType::System,
            "kernel".to_string(),
            "list_processes".to_string(),
        );
        ps_cmd.add_example("ps".to_string());
        ps_cmd.add_example("ps -a".to_string());
        self.add_command(ps_cmd);

        let mut kill_cmd = CommandInfo::new(
            "kill".to_string(),
            "Terminar proceso por PID".to_string(),
            CommandType::System,
            "kernel".to_string(),
            "kill_process".to_string(),
        );
        kill_cmd.add_parameter("<PID>".to_string());
        kill_cmd.add_example("kill 1234".to_string());
        self.add_command(kill_cmd);

        let mut uptime_cmd = CommandInfo::new(
            "uptime".to_string(),
            "Mostrar tiempo de actividad del sistema".to_string(),
            CommandType::System,
            "kernel".to_string(),
            "show_uptime".to_string(),
        );
        uptime_cmd.add_example("uptime".to_string());
        self.add_command(uptime_cmd);

        let mut date_cmd = CommandInfo::new(
            "date".to_string(),
            "Mostrar fecha y hora actual".to_string(),
            CommandType::System,
            "kernel".to_string(),
            "show_date".to_string(),
        );
        date_cmd.add_example("date".to_string());
        date_cmd.add_example("date +%Y-%m-%d".to_string());
        self.add_command(date_cmd);

        let mut whoami_cmd = CommandInfo::new(
            "whoami".to_string(),
            "Mostrar usuario actual".to_string(),
            CommandType::System,
            "kernel".to_string(),
            "show_user".to_string(),
        );
        whoami_cmd.add_example("whoami".to_string());
        self.add_command(whoami_cmd);
    }

    fn generate_module_commands(&mut self) {
        // Comandos de módulos del kernel
        let modules = vec![
            ("memory", "Gestión de memoria"),
            ("interrupts", "Manejo de interrupciones"),
            ("vga", "Control de video"),
            ("keyboard", "Control de teclado"),
            ("mouse", "Control de mouse"),
            ("filesystem", "Sistema de archivos"),
            ("network", "Protocolos de red"),
            ("audio", "Sistema de audio"),
            ("gui", "Interfaz gráfica"),
            ("apps", "Aplicaciones"),
            ("performance", "Rendimiento"),
            ("hardware", "Hardware"),
            ("debug", "Depuración"),
            ("logging", "Registro de eventos"),
            ("signals", "Sistema de señales"),
            ("file_operations", "Operaciones de archivos"),
            ("system_settings", "Configuración del sistema"),
            ("text_editor", "Editor de texto"),
            ("file_manager", "Gestor de archivos"),
            ("advanced_audio", "Audio avanzado"),
            ("network_protocols", "Protocolos de red"),
        ];

        for (module, description) in modules {
            let info_cmd = CommandInfo::new(
                format!("{}_info", module),
                format!("Información del módulo {}", description),
                CommandType::Module,
                module.to_string(),
                "get_info".to_string(),
            );
            self.add_command(info_cmd);

            let stats_cmd = CommandInfo::new(
                format!("{}_stats", module),
                format!("Estadísticas del módulo {}", description),
                CommandType::Module,
                module.to_string(),
                "get_stats".to_string(),
            );
            self.add_command(stats_cmd);

            // Agregar a la lista de comandos del módulo
            self.module_commands.entry(module.to_string())
                .or_insert_with(Vec::new)
                .push(format!("{}_info", module));
            self.module_commands.get_mut(module).unwrap()
                .push(format!("{}_stats", module));
        }
    }

    fn generate_application_commands(&mut self) {
        // Comandos de aplicaciones
        let apps = vec![
            ("editor", "Editor de texto", "open_text_editor"),
            ("calculator", "Calculadora", "open_calculator"),
            ("file_viewer", "Visor de archivos", "open_file_viewer"),
            ("system_monitor", "Monitor de sistema", "open_system_monitor"),
            ("snake_game", "Juego Snake", "open_snake_game"),
        ];

        for (app, description, function) in apps {
            let mut cmd = CommandInfo::new(
                app.to_string(),
                format!("Abrir {}", description),
                CommandType::Application,
                "apps".to_string(),
                function.to_string(),
            );
            cmd.add_example(format!("{}", app));
            self.add_command(cmd);
        }
    }

    fn generate_utility_commands(&mut self) {
        // Comandos de utilidades
        let utilities = vec![
            ("help", "Mostrar ayuda", "show_help"),
            ("clear", "Limpiar pantalla", "clear_screen"),
            ("history", "Mostrar historial", "show_history"),
            ("alias", "Crear alias", "create_alias"),
            ("unalias", "Eliminar alias", "remove_alias"),
            ("env", "Variables de entorno", "show_environment"),
            ("export", "Exportar variable", "export_variable"),
            ("unset", "Eliminar variable", "unset_variable"),
            ("echo", "Mostrar texto", "echo_text"),
            ("printf", "Formatear salida", "format_output"),
            ("sleep", "Pausar ejecución", "sleep_seconds"),
            ("time", "Medir tiempo", "measure_time"),
            ("which", "Localizar comando", "locate_command"),
            ("whereis", "Localizar archivo", "locate_file"),
            ("man", "Manual de comando", "show_manual"),
            ("info", "Información detallada", "show_info"),
            ("apropos", "Buscar comando", "search_command"),
            ("whatis", "Descripción breve", "show_description"),
        ];

        for (util, description, function) in utilities {
            let mut cmd = CommandInfo::new(
                util.to_string(),
                description.to_string(),
                CommandType::Utility,
                "shell".to_string(),
                function.to_string(),
            );
            cmd.add_example(format!("{}", util));
            self.add_command(cmd);
        }
    }

    fn generate_network_commands(&mut self) {
        // Comandos de red
        let network_cmds = vec![
            ("netproto", "Información de protocolos de red", "get_network_protocols_info"),
            ("netstats", "Estadísticas de red", "get_network_protocols_stats"),
            ("httpserver", "Información del servidor HTTP", "get_http_server_info"),
            ("ftpserver", "Información del servidor FTP", "get_ftp_server_info"),
            ("httpclient", "Información del cliente HTTP", "get_http_client_info"),
            ("httpreq", "Realizar solicitud HTTP", "make_http_request"),
            ("ftpsession", "Crear sesión FTP", "create_ftp_session"),
            ("ftpcmd", "Ejecutar comando FTP", "execute_ftp_command"),
            ("webpage", "Mostrar página web", "show_web_page"),
            ("netstatus", "Estado de servicios de red", "show_network_status"),
        ];

        for (cmd, description, function) in network_cmds {
            let mut command = CommandInfo::new(
                cmd.to_string(),
                description.to_string(),
                CommandType::Network,
                "network_protocols".to_string(),
                function.to_string(),
            );
            command.add_example(format!("{}", cmd));
            self.add_command(command);
        }
    }

    fn generate_file_commands(&mut self) {
        // Comandos de archivos
        let file_cmds = vec![
            ("ls", "Listar archivos", "list_files"),
            ("cat", "Mostrar contenido", "show_file_content"),
            ("touch", "Crear archivo", "create_file"),
            ("mkdir", "Crear directorio", "create_directory"),
            ("rm", "Eliminar archivo", "remove_file"),
            ("rmdir", "Eliminar directorio", "remove_directory"),
            ("cp", "Copiar archivo", "copy_file"),
            ("mv", "Mover archivo", "move_file"),
            ("find", "Buscar archivos", "find_files"),
            ("grep", "Buscar texto", "search_text"),
            ("wc", "Contar líneas", "count_lines"),
            ("head", "Mostrar primeras líneas", "show_head"),
            ("tail", "Mostrar últimas líneas", "show_tail"),
            ("sort", "Ordenar líneas", "sort_lines"),
            ("uniq", "Eliminar duplicados", "remove_duplicates"),
        ];

        for (cmd, description, function) in file_cmds {
            let mut command = CommandInfo::new(
                cmd.to_string(),
                description.to_string(),
                CommandType::File,
                "file_operations".to_string(),
                function.to_string(),
            );
            command.add_example(format!("{} archivo.txt", cmd));
            self.add_command(command);
        }
    }

    fn generate_audio_commands(&mut self) {
        // Comandos de audio
        let audio_cmds = vec![
            ("audio_info", "Información del sistema de audio", "get_advanced_audio_info"),
            ("audio_stats", "Estadísticas de audio", "get_advanced_audio_stats"),
            ("play", "Reproducir audio", "play_audio"),
            ("stop", "Detener reproducción", "stop_audio"),
            ("pause", "Pausar reproducción", "pause_audio"),
            ("resume", "Reanudar reproducción", "resume_audio"),
            ("volume", "Controlar volumen", "set_volume"),
            ("mute", "Silenciar", "mute_audio"),
            ("unmute", "Activar sonido", "unmute_audio"),
            ("record", "Grabar audio", "record_audio"),
            ("mixer", "Mezclador de audio", "open_mixer"),
            ("equalizer", "Ecualizador", "open_equalizer"),
        ];

        for (cmd, description, function) in audio_cmds {
            let mut command = CommandInfo::new(
                cmd.to_string(),
                description.to_string(),
                CommandType::Audio,
                "advanced_audio".to_string(),
                function.to_string(),
            );
            command.add_example(format!("{}", cmd));
            self.add_command(command);
        }
    }

    fn generate_gui_commands(&mut self) {
        // Comandos de interfaz gráfica
        let gui_cmds = vec![
            ("gui", "Abrir interfaz gráfica", "open_gui"),
            ("window", "Crear ventana", "create_window"),
            ("button", "Crear botón", "create_button"),
            ("menu", "Crear menú", "create_menu"),
            ("dialog", "Mostrar diálogo", "show_dialog"),
            ("theme", "Cambiar tema", "change_theme"),
            ("resolution", "Cambiar resolución", "change_resolution"),
            ("fullscreen", "Pantalla completa", "toggle_fullscreen"),
            ("minimize", "Minimizar ventana", "minimize_window"),
            ("maximize", "Maximizar ventana", "maximize_window"),
            ("close", "Cerrar ventana", "close_window"),
        ];

        for (cmd, description, function) in gui_cmds {
            let mut command = CommandInfo::new(
                cmd.to_string(),
                description.to_string(),
                CommandType::Gui,
                "gui".to_string(),
                function.to_string(),
            );
            command.add_example(format!("{}", cmd));
            self.add_command(command);
        }
    }

    fn add_command(&mut self, command: CommandInfo) {
        self.commands.insert(command.name.clone(), command);
    }

    pub fn get_command(&self, name: &str) -> Option<&CommandInfo> {
        self.commands.get(name)
    }

    pub fn get_commands_by_type(&self, command_type: CommandType) -> Vec<&CommandInfo> {
        self.commands.values()
            .filter(|cmd| cmd.command_type == command_type)
            .collect()
    }

    pub fn get_commands_by_module(&self, module: &str) -> Vec<&CommandInfo> {
        self.commands.values()
            .filter(|cmd| cmd.module_name == module)
            .collect()
    }

    pub fn search_commands(&self, query: &str) -> Vec<&CommandInfo> {
        self.commands.values()
            .filter(|cmd| {
                cmd.name.contains(query) || 
                cmd.description.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    pub fn get_all_commands(&self) -> Vec<&CommandInfo> {
        self.commands.values().collect()
    }

    pub fn get_available_commands(&self) -> Vec<&CommandInfo> {
        self.commands.values()
            .filter(|cmd| cmd.is_available)
            .collect()
    }

    pub fn get_command_count(&self) -> (u32, u32) {
        (self.total_commands, self.available_commands)
    }

    pub fn get_module_command_count(&self, module: &str) -> usize {
        self.module_commands.get(module)
            .map(|cmds| cmds.len())
            .unwrap_or(0)
    }

    pub fn get_statistics(&self) -> String {
        let mut stats = String::new();
        
        // Estadísticas generales
        stats.push_str(&format!("Comandos totales: {}\n", self.total_commands));
        stats.push_str(&format!("Comandos disponibles: {}\n", self.available_commands));
        stats.push_str(&format!("Comandos por tipo:\n"));
        
        // Contar por tipo
        let mut type_counts = BTreeMap::new();
        for cmd in self.commands.values() {
            *type_counts.entry(cmd.command_type).or_insert(0) += 1;
        }
        
        for (cmd_type, count) in type_counts {
            stats.push_str(&format!("  {:?}: {}\n", cmd_type, count));
        }
        
        // Comandos por módulo
        stats.push_str("Comandos por módulo:\n");
        for (module, cmds) in &self.module_commands {
            stats.push_str(&format!("  {}: {}\n", module, cmds.len()));
        }
        
        stats
    }

    pub fn generate_help_for_command(&self, command_name: &str) -> String {
        if let Some(cmd) = self.get_command(command_name) {
            cmd.get_help()
        } else {
            format!("Comando '{}' no encontrado", command_name)
        }
    }

    pub fn generate_help_for_module(&self, module_name: &str) -> String {
        let commands = self.get_commands_by_module(module_name);
        if commands.is_empty() {
            return format!("No se encontraron comandos para el módulo '{}'", module_name);
        }
        
        let mut help = format!("Comandos del módulo '{}':\n\n", module_name);
        for cmd in commands {
            help.push_str(&format!("{} - {}\n", cmd.name, cmd.description));
        }
        help
    }

    pub fn generate_complete_help(&self) -> String {
        let mut help = String::new();
        
        help.push_str("ReactOS Rust Kernel - Sistema de Comandos Dinámico\n");
        help.push_str("================================================\n\n");
        
        help.push_str(&self.get_statistics());
        help.push_str("\n");
        
        // Agrupar comandos por tipo
        for cmd_type in [CommandType::System, CommandType::Module, CommandType::Application, 
                        CommandType::Utility, CommandType::Network, CommandType::File, 
                        CommandType::Audio, CommandType::Gui] {
            let commands = self.get_commands_by_type(cmd_type);
            if !commands.is_empty() {
                help.push_str(&format!("{:?}:\n", cmd_type));
                for cmd in commands {
                    help.push_str(&format!("  {} - {}\n", cmd.name, cmd.description));
                }
                help.push_str("\n");
            }
        }
        
        help.push_str("Para obtener ayuda detallada de un comando específico:\n");
        help.push_str("  help <comando>\n");
        help.push_str("  man <comando>\n");
        help.push_str("  info <comando>\n");
        
        help
    }
}

// Gestor global de comandos dinámicos
use spin::Mutex;

pub static DYNAMIC_COMMAND_GENERATOR: Mutex<Option<DynamicCommandGenerator>> = Mutex::new(None);

/// Inicializar el generador de comandos dinámicos
pub fn init_dynamic_commands() {
    let mut generator = DYNAMIC_COMMAND_GENERATOR.lock();
    *generator = Some(DynamicCommandGenerator::new());
    if let Some(ref mut gen) = *generator {
        gen.initialize();
    }
    crate::logging::info("dynamic_commands", "Sistema de comandos dinámico inicializado");
}

/// Obtener información de un comando
pub fn get_command_info(command_name: &str) -> Option<CommandInfo> {
    let generator = DYNAMIC_COMMAND_GENERATOR.lock();
    generator.as_ref()?.get_command(command_name).cloned()
}

/// Buscar comandos
pub fn search_commands(query: &str) -> Vec<CommandInfo> {
    let generator = DYNAMIC_COMMAND_GENERATOR.lock();
    if let Some(ref gen) = *generator {
        gen.search_commands(query).into_iter().cloned().collect()
    } else {
        Vec::new()
    }
}

/// Obtener comandos por módulo
pub fn get_module_commands(module: &str) -> Vec<CommandInfo> {
    let generator = DYNAMIC_COMMAND_GENERATOR.lock();
    if let Some(ref gen) = *generator {
        gen.get_commands_by_module(module).into_iter().cloned().collect()
    } else {
        Vec::new()
    }
}

/// Obtener estadísticas del sistema de comandos
pub fn get_command_statistics() -> String {
    let generator = DYNAMIC_COMMAND_GENERATOR.lock();
    if let Some(ref gen) = *generator {
        gen.get_statistics()
    } else {
        String::from("Sistema de comandos no inicializado")
    }
}

/// Generar ayuda completa
pub fn generate_complete_help() -> String {
    let generator = DYNAMIC_COMMAND_GENERATOR.lock();
    if let Some(ref gen) = *generator {
        gen.generate_complete_help()
    } else {
        String::from("Sistema de comandos no inicializado")
    }
}

/// Verificar si el sistema de comandos está disponible
pub fn is_dynamic_commands_available() -> bool {
    let generator = DYNAMIC_COMMAND_GENERATOR.lock();
    generator.is_some()
}

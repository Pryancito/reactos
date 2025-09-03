//! Funciones comunes para aplicaciones nativas
//! 
//! Este módulo contiene funciones y estructuras comunes
//! utilizadas por todas las aplicaciones nativas.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Configuración de aplicación
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
    pub theme: AppTheme,
    pub language: String,
    pub auto_save: bool,
    pub auto_save_interval: Duration,
    pub debug_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            window_title: "ReactOS Rust App".to_string(),
            theme: AppTheme::Dark,
            language: "es".to_string(),
            auto_save: true,
            auto_save_interval: Duration::from_secs(30),
            debug_mode: false,
        }
    }
}

/// Tema de la aplicación
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppTheme {
    Light,
    Dark,
    Auto,
}

/// Estado de la ventana
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

/// Evento de la aplicación
#[derive(Debug, Clone)]
pub enum AppEvent {
    WindowResize { width: u32, height: u32 },
    WindowMove { x: i32, y: i32 },
    WindowStateChange { state: WindowState },
    KeyPress { key: String, modifiers: Vec<String> },
    MouseClick { x: i32, y: i32, button: MouseButton },
    MouseMove { x: i32, y: i32 },
    FileOpen { path: PathBuf },
    FileSave { path: PathBuf },
    FileClose,
    AppQuit,
    Custom { data: String },
}

/// Botón del mouse
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

/// Información del archivo
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub is_directory: bool,
    pub is_hidden: bool,
    pub is_readonly: bool,
    pub created: Option<Instant>,
    pub modified: Option<Instant>,
    pub accessed: Option<Instant>,
    pub permissions: FilePermissions,
}

/// Permisos de archivo
#[derive(Debug, Clone, Copy)]
pub struct FilePermissions {
    pub owner_read: bool,
    pub owner_write: bool,
    pub owner_execute: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,
}

impl Default for FilePermissions {
    fn default() -> Self {
        Self {
            owner_read: true,
            owner_write: true,
            owner_execute: false,
            group_read: true,
            group_write: false,
            group_execute: false,
            other_read: true,
            other_write: false,
            other_execute: false,
        }
    }
}

/// Gestor de archivos
pub struct FileManager {
    pub current_directory: PathBuf,
    pub file_cache: HashMap<PathBuf, FileInfo>,
    pub history: Vec<PathBuf>,
    pub history_index: usize,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            current_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            file_cache: HashMap::new(),
            history: Vec::new(),
            history_index: 0,
        }
    }
    
    /// Cambiar directorio
    pub fn change_directory(&mut self, path: PathBuf) -> Result<(), String> {
        if path.is_dir() {
            self.history.push(self.current_directory.clone());
            self.history_index = self.history.len() - 1;
            self.current_directory = path;
            self.refresh_cache();
            Ok(())
        } else {
            Err("Path is not a directory".to_string())
        }
    }
    
    /// Navegar hacia atrás
    pub fn go_back(&mut self) -> Result<(), String> {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_directory = self.history[self.history_index].clone();
            self.refresh_cache();
            Ok(())
        } else {
            Err("No history to go back to".to_string())
        }
    }
    
    /// Navegar hacia adelante
    pub fn go_forward(&mut self) -> Result<(), String> {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.current_directory = self.history[self.history_index].clone();
            self.refresh_cache();
            Ok(())
        } else {
            Err("No history to go forward to".to_string())
        }
    }
    
    /// Refrescar caché de archivos
    pub fn refresh_cache(&mut self) {
        self.file_cache.clear();
        
        if let Ok(entries) = std::fs::read_dir(&self.current_directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                let file_info = FileInfo {
                    path: path.clone(),
                    name: entry.file_name().to_string_lossy().to_string(),
                    size: entry.metadata().map(|m| m.len()).unwrap_or(0),
                    is_directory: path.is_dir(),
                    is_hidden: entry.file_name().to_string_lossy().starts_with('.'),
                    is_readonly: entry.metadata().map(|m| m.permissions().readonly()).unwrap_or(false),
                    created: entry.metadata().ok().and_then(|m| m.created().ok()).map(|t| Instant::now()),
                    modified: entry.metadata().ok().and_then(|m| m.modified().ok()).map(|t| Instant::now()),
                    accessed: entry.metadata().ok().and_then(|m| m.accessed().ok()).map(|t| Instant::now()),
                    permissions: FilePermissions::default(),
                };
                self.file_cache.insert(path, file_info);
            }
        }
    }
    
    /// Obtener archivos en el directorio actual
    pub fn get_files(&self) -> Vec<&FileInfo> {
        let mut files: Vec<&FileInfo> = self.file_cache.values().collect();
        files.sort_by(|a, b| {
            // Directorios primero, luego archivos
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });
        files
    }
    
    /// Buscar archivos
    pub fn search_files(&self, query: &str) -> Vec<&FileInfo> {
        self.file_cache
            .values()
            .filter(|file| file.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }
}

/// Utilidades de texto
pub struct TextUtils;

impl TextUtils {
    /// Contar líneas en texto
    pub fn count_lines(text: &str) -> usize {
        text.lines().count()
    }
    
    /// Contar palabras en texto
    pub fn count_words(text: &str) -> usize {
        text.split_whitespace().count()
    }
    
    /// Contar caracteres en texto
    pub fn count_characters(text: &str) -> usize {
        text.chars().count()
    }
    
    /// Contar caracteres sin espacios
    pub fn count_characters_no_spaces(text: &str) -> usize {
        text.chars().filter(|c| !c.is_whitespace()).count()
    }
    
    /// Detectar tipo de archivo por extensión
    pub fn detect_file_type(filename: &str) -> FileType {
        let extension = filename
            .split('.')
            .last()
            .unwrap_or("")
            .to_lowercase();
        
        match extension.as_str() {
            "txt" | "md" | "rst" | "log" => FileType::Text,
            "rs" | "c" | "cpp" | "h" | "hpp" => FileType::Code,
            "json" | "xml" | "yaml" | "yml" | "toml" => FileType::Data,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => FileType::Image,
            "mp3" | "wav" | "flac" | "ogg" | "aac" => FileType::Audio,
            "mp4" | "avi" | "mkv" | "mov" | "wmv" => FileType::Video,
            "pdf" | "doc" | "docx" | "odt" => FileType::Document,
            "zip" | "rar" | "7z" | "tar" | "gz" => FileType::Archive,
            _ => FileType::Unknown,
        }
    }
    
    /// Obtener resaltado de sintaxis para tipo de archivo
    pub fn get_syntax_highlighting(file_type: FileType) -> SyntaxHighlighting {
        match file_type {
            FileType::Code => SyntaxHighlighting::Rust,
            FileType::Text => SyntaxHighlighting::Plain,
            FileType::Data => SyntaxHighlighting::Json,
            _ => SyntaxHighlighting::Plain,
        }
    }
}

/// Tipo de archivo
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Text,
    Code,
    Data,
    Image,
    Audio,
    Video,
    Document,
    Archive,
    Unknown,
}

/// Resaltado de sintaxis
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SyntaxHighlighting {
    Plain,
    Rust,
    C,
    Cpp,
    Json,
    Xml,
    Markdown,
    Python,
    JavaScript,
    Html,
    Css,
}

/// Utilidades de matemáticas
pub struct MathUtils;

impl MathUtils {
    /// Evaluar expresión matemática
    pub fn evaluate_expression(expression: &str) -> Result<f64, String> {
        // Implementación básica de evaluador de expresiones
        // En una implementación real, se usaría una librería como `evalexpr`
        
        let expr = expression.trim();
        
        // Verificar si es un número simple
        if let Ok(num) = expr.parse::<f64>() {
            return Ok(num);
        }
        
        // Implementación básica para operaciones simples
        if expr.contains('+') {
            let parts: Vec<&str> = expr.split('+').collect();
            if parts.len() == 2 {
                let left = parts[0].trim().parse::<f64>().map_err(|_| "Invalid number")?;
                let right = parts[1].trim().parse::<f64>().map_err(|_| "Invalid number")?;
                return Ok(left + right);
            }
        }
        
        if expr.contains('-') {
            let parts: Vec<&str> = expr.split('-').collect();
            if parts.len() == 2 {
                let left = parts[0].trim().parse::<f64>().map_err(|_| "Invalid number")?;
                let right = parts[1].trim().parse::<f64>().map_err(|_| "Invalid number")?;
                return Ok(left - right);
            }
        }
        
        if expr.contains('*') {
            let parts: Vec<&str> = expr.split('*').collect();
            if parts.len() == 2 {
                let left = parts[0].trim().parse::<f64>().map_err(|_| "Invalid number")?;
                let right = parts[1].trim().parse::<f64>().map_err(|_| "Invalid number")?;
                return Ok(left * right);
            }
        }
        
        if expr.contains('/') {
            let parts: Vec<&str> = expr.split('/').collect();
            if parts.len() == 2 {
                let left = parts[0].trim().parse::<f64>().map_err(|_| "Invalid number")?;
                let right = parts[1].trim().parse::<f64>().map_err(|_| "Invalid number")?;
                if right == 0.0 {
                    return Err("Division by zero".to_string());
                }
                return Ok(left / right);
            }
        }
        
        Err("Unsupported expression".to_string())
    }
    
    /// Calcular funciones trigonométricas
    pub fn sin(angle: f64) -> f64 {
        angle.sin()
    }
    
    pub fn cos(angle: f64) -> f64 {
        angle.cos()
    }
    
    pub fn tan(angle: f64) -> f64 {
        angle.tan()
    }
    
    /// Calcular logaritmos
    pub fn ln(x: f64) -> Result<f64, String> {
        if x <= 0.0 {
            Err("Logarithm of non-positive number".to_string())
        } else {
            Ok(x.ln())
        }
    }
    
    pub fn log10(x: f64) -> Result<f64, String> {
        if x <= 0.0 {
            Err("Logarithm of non-positive number".to_string())
        } else {
            Ok(x.log10())
        }
    }
    
    /// Calcular raíz cuadrada
    pub fn sqrt(x: f64) -> Result<f64, String> {
        if x < 0.0 {
            Err("Square root of negative number".to_string())
        } else {
            Ok(x.sqrt())
        }
    }
    
    /// Calcular potencia
    pub fn pow(base: f64, exponent: f64) -> f64 {
        base.powf(exponent)
    }
}

/// Utilidades de sistema
pub struct SystemUtils;

impl SystemUtils {
    /// Obtener información del sistema
    pub fn get_system_info() -> SystemInfo {
        SystemInfo {
            os_name: "ReactOS Rust".to_string(),
            os_version: "1.0.0".to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            cpu_count: 4, // Simulado
            total_memory: 8 * 1024 * 1024 * 1024, // 8GB simulado
            available_memory: 4 * 1024 * 1024 * 1024, // 4GB simulado
            uptime: Duration::from_secs(3600), // 1 hora simulado
        }
    }
    
    /// Obtener uso de CPU
    pub fn get_cpu_usage() -> f64 {
        // Simulación de uso de CPU
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() % 100) as f64 / 100.0
    }
    
    /// Obtener uso de memoria
    pub fn get_memory_usage() -> f64 {
        // Simulación de uso de memoria
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() % 80) as f64 / 100.0
    }
    
    /// Obtener uso de disco
    pub fn get_disk_usage() -> f64 {
        // Simulación de uso de disco
        (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() % 60) as f64 / 100.0
    }
}

/// Información del sistema
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub architecture: String,
    pub cpu_count: usize,
    pub total_memory: u64,
    pub available_memory: u64,
    pub uptime: Duration,
}

/// Utilidades de audio
pub struct AudioUtils;

impl AudioUtils {
    /// Detectar formato de archivo de audio
    pub fn detect_audio_format(filename: &str) -> AudioFormat {
        let extension = filename
            .split('.')
            .last()
            .unwrap_or("")
            .to_lowercase();
        
        match extension.as_str() {
            "mp3" => AudioFormat::MP3,
            "wav" => AudioFormat::WAV,
            "flac" => AudioFormat::FLAC,
            "ogg" => AudioFormat::OGG,
            "aac" => AudioFormat::AAC,
            "m4a" => AudioFormat::M4A,
            _ => AudioFormat::Unknown,
        }
    }
    
    /// Obtener duración estimada del archivo de audio
    pub fn get_audio_duration(file_path: &str) -> Result<Duration, String> {
        // En una implementación real, se usaría una librería como `hound` o `symphonia`
        // Por ahora, simulamos la duración
        Ok(Duration::from_secs(180)) // 3 minutos simulado
    }
    
    /// Obtener información del archivo de audio
    pub fn get_audio_info(file_path: &str) -> Result<AudioInfo, String> {
        // Simulación de información de audio
        Ok(AudioInfo {
            format: AudioUtils::detect_audio_format(file_path),
            duration: AudioUtils::get_audio_duration(file_path)?,
            bitrate: 320000, // 320 kbps
            sample_rate: 44100, // 44.1 kHz
            channels: 2, // Estéreo
            title: "Unknown Title".to_string(),
            artist: "Unknown Artist".to_string(),
            album: "Unknown Album".to_string(),
        })
    }
}

/// Formato de audio
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioFormat {
    MP3,
    WAV,
    FLAC,
    OGG,
    AAC,
    M4A,
    Unknown,
}

/// Información de audio
#[derive(Debug, Clone)]
pub struct AudioInfo {
    pub format: AudioFormat,
    pub duration: Duration,
    pub bitrate: u32,
    pub sample_rate: u32,
    pub channels: u8,
    pub title: String,
    pub artist: String,
    pub album: String,
}

/// Utilidades de imagen
pub struct ImageUtils;

impl ImageUtils {
    /// Detectar formato de archivo de imagen
    pub fn detect_image_format(filename: &str) -> ImageFormat {
        let extension = filename
            .split('.')
            .last()
            .unwrap_or("")
            .to_lowercase();
        
        match extension.as_str() {
            "jpg" | "jpeg" => ImageFormat::JPEG,
            "png" => ImageFormat::PNG,
            "gif" => ImageFormat::GIF,
            "bmp" => ImageFormat::BMP,
            "svg" => ImageFormat::SVG,
            "webp" => ImageFormat::WEBP,
            "tiff" | "tif" => ImageFormat::TIFF,
            _ => ImageFormat::Unknown,
        }
    }
    
    /// Obtener información de la imagen
    pub fn get_image_info(file_path: &str) -> Result<ImageInfo, String> {
        // En una implementación real, se usaría la librería `image`
        // Por ahora, simulamos la información
        Ok(ImageInfo {
            format: ImageUtils::detect_image_format(file_path),
            width: 1920,
            height: 1080,
            color_depth: 24,
            has_transparency: false,
            file_size: 1024 * 1024, // 1MB simulado
        })
    }
}

/// Formato de imagen
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageFormat {
    JPEG,
    PNG,
    GIF,
    BMP,
    SVG,
    WEBP,
    TIFF,
    Unknown,
}

/// Información de imagen
#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
    pub color_depth: u8,
    pub has_transparency: bool,
    pub file_size: u64,
}

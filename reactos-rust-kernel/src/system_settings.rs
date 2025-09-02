//! Panel de Configuración del Sistema
//!
//! Sistema centralizado de configuración para todas las funcionalidades del kernel

use alloc::{vec, vec::Vec, string::{String, ToString}, format, collections::BTreeMap};

/// Tipo de configuración
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigType {
    Boolean,    // true/false
    Integer,    // Número entero
    String,     // Cadena de texto
    Float,      // Número decimal
    Enum,       // Valor de enumeración
    Color,      // Color RGB
    Path,       // Ruta de archivo/directorio
}

/// Categoría de configuración
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigCategory {
    System,     // Configuraciones del sistema
    Network,    // Configuraciones de red
    Audio,      // Configuraciones de audio
    Graphics,   // Configuraciones gráficas
    Security,   // Configuraciones de seguridad
    Performance, // Configuraciones de rendimiento
    Hardware,   // Configuraciones de hardware
    User,       // Configuraciones de usuario
}

/// Nivel de acceso a la configuración
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigAccess {
    ReadOnly,   // Solo lectura
    User,       // Usuario normal
    Admin,      // Administrador
    System,     // Solo sistema
}

/// Validación de configuración
#[derive(Debug, Clone)]
pub struct ConfigValidation {
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub allowed_values: Vec<String>,
    pub pattern: Option<String>, // Regex pattern
    pub required: bool,
}

impl ConfigValidation {
    pub fn new() -> Self {
        Self {
            min_value: None,
            max_value: None,
            allowed_values: Vec::new(),
            pattern: None,
            required: false,
        }
    }

    pub fn with_range(min: String, max: String) -> Self {
        Self {
            min_value: Some(min),
            max_value: Some(max),
            allowed_values: Vec::new(),
            pattern: None,
            required: false,
        }
    }

    pub fn with_allowed_values(values: Vec<String>) -> Self {
        Self {
            min_value: None,
            max_value: None,
            allowed_values: values,
            pattern: None,
            required: false,
        }
    }

    pub fn with_pattern(pattern: String) -> Self {
        Self {
            min_value: None,
            max_value: None,
            allowed_values: Vec::new(),
            pattern: Some(pattern),
            required: false,
        }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

/// Elemento de configuración
#[derive(Debug, Clone)]
pub struct ConfigItem {
    pub key: String,
    pub name: String,
    pub description: String,
    pub category: ConfigCategory,
    pub config_type: ConfigType,
    pub access_level: ConfigAccess,
    pub default_value: String,
    pub current_value: String,
    pub validation: ConfigValidation,
    pub is_modified: bool,
    pub last_modified: u64,
}

impl ConfigItem {
    pub fn new(
        key: String,
        name: String,
        description: String,
        category: ConfigCategory,
        config_type: ConfigType,
        default_value: String,
    ) -> Self {
        Self {
            key,
            name,
            description,
            category,
            config_type,
            access_level: ConfigAccess::User,
            default_value: default_value.clone(),
            current_value: default_value,
            validation: ConfigValidation::new(),
            is_modified: false,
            last_modified: get_system_time(),
        }
    }

    pub fn with_access_level(mut self, access: ConfigAccess) -> Self {
        self.access_level = access;
        self
    }

    pub fn with_validation(mut self, validation: ConfigValidation) -> Self {
        self.validation = validation;
        self
    }

    pub fn set_value(&mut self, value: String) -> bool {
        if self.validate_value(&value) {
            self.current_value = value;
            self.is_modified = true;
            self.last_modified = get_system_time();
            true
        } else {
            false
        }
    }

    pub fn validate_value(&self, value: &str) -> bool {
        // Validación básica
        if self.validation.required && value.is_empty() {
            return false;
        }

        // Validación por tipo
        match self.config_type {
            ConfigType::Boolean => {
                value == "true" || value == "false"
            },
            ConfigType::Integer => {
                if let Ok(_) = value.parse::<i64>() {
                    // Validar rango si está definido
                    if let (Some(min), Some(max)) = (&self.validation.min_value, &self.validation.max_value) {
                        if let (Ok(min_val), Ok(max_val), Ok(val)) = (min.parse::<i64>(), max.parse::<i64>(), value.parse::<i64>()) {
                            val >= min_val && val <= max_val
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                } else {
                    false
                }
            },
            ConfigType::Float => {
                if let Ok(_) = value.parse::<f64>() {
                    // Validar rango si está definido
                    if let (Some(min), Some(max)) = (&self.validation.min_value, &self.validation.max_value) {
                        if let (Ok(min_val), Ok(max_val), Ok(val)) = (min.parse::<f64>(), max.parse::<f64>(), value.parse::<f64>()) {
                            val >= min_val && val <= max_val
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                } else {
                    false
                }
            },
            ConfigType::Enum => {
                self.validation.allowed_values.contains(&value.to_string())
            },
            ConfigType::Color => {
                // Validar formato RGB: #RRGGBB
                value.len() == 7 && value.starts_with('#') && value[1..].chars().all(|c| c.is_ascii_hexdigit())
            },
            ConfigType::Path => {
                // Validación básica de ruta
                !value.is_empty() && !value.contains('\0')
            },
            ConfigType::String => {
                // Validación de patrón si está definida
                if let Some(pattern) = &self.validation.pattern {
                    // Implementación simplificada de validación de patrón
                    validate_pattern(value, pattern)
                } else {
                    true
                }
            },
        }
    }

    pub fn reset_to_default(&mut self) {
        self.current_value = self.default_value.clone();
        self.is_modified = false;
        self.last_modified = get_system_time();
    }

    pub fn get_info(&self) -> String {
        format!(
            "{}: {} ({}: {})",
            self.name,
            self.current_value,
            self.category_name(),
            self.config_type_name()
        )
    }

    pub fn category_name(&self) -> &'static str {
        match self.category {
            ConfigCategory::System => "Sistema",
            ConfigCategory::Network => "Red",
            ConfigCategory::Audio => "Audio",
            ConfigCategory::Graphics => "Gráficos",
            ConfigCategory::Security => "Seguridad",
            ConfigCategory::Performance => "Rendimiento",
            ConfigCategory::Hardware => "Hardware",
            ConfigCategory::User => "Usuario",
        }
    }

    pub fn config_type_name(&self) -> &'static str {
        match self.config_type {
            ConfigType::Boolean => "Boolean",
            ConfigType::Integer => "Entero",
            ConfigType::String => "Texto",
            ConfigType::Float => "Decimal",
            ConfigType::Enum => "Enumeración",
            ConfigType::Color => "Color",
            ConfigType::Path => "Ruta",
        }
    }
}

/// Grupo de configuraciones
#[derive(Debug, Clone)]
pub struct ConfigGroup {
    pub name: String,
    pub description: String,
    pub category: ConfigCategory,
    pub items: BTreeMap<String, ConfigItem>,
    pub is_expanded: bool,
}

impl ConfigGroup {
    pub fn new(name: String, description: String, category: ConfigCategory) -> Self {
        Self {
            name,
            description,
            category,
            items: BTreeMap::new(),
            is_expanded: false,
        }
    }

    pub fn add_item(&mut self, item: ConfigItem) {
        self.items.insert(item.key.clone(), item);
    }

    pub fn get_item(&self, key: &str) -> Option<&ConfigItem> {
        self.items.get(key)
    }

    pub fn get_item_mut(&mut self, key: &str) -> Option<&mut ConfigItem> {
        self.items.get_mut(key)
    }

    pub fn remove_item(&mut self, key: &str) -> Option<ConfigItem> {
        self.items.remove(key)
    }

    pub fn get_modified_count(&self) -> usize {
        self.items.values().filter(|item| item.is_modified).count()
    }

    pub fn reset_all_to_default(&mut self) {
        for item in self.items.values_mut() {
            item.reset_to_default();
        }
    }
}

/// Panel de configuración principal
#[derive(Debug, Clone)]
pub struct SystemSettings {
    pub groups: BTreeMap<String, ConfigGroup>,
    pub current_group: Option<String>,
    pub search_filter: String,
    pub show_modified_only: bool,
    pub auto_save: bool,
    pub backup_enabled: bool,
    pub last_backup: u64,
    pub statistics: SettingsStatistics,
}

#[derive(Debug, Clone)]
pub struct SettingsStatistics {
    pub total_items: usize,
    pub modified_items: usize,
    pub groups_count: usize,
    pub last_save: u64,
    pub save_count: u64,
    pub load_count: u64,
    pub error_count: u64,
}

impl Default for SettingsStatistics {
    fn default() -> Self {
        Self {
            total_items: 0,
            modified_items: 0,
            groups_count: 0,
            last_save: 0,
            save_count: 0,
            load_count: 0,
            error_count: 0,
        }
    }
}

impl SystemSettings {
    pub fn new() -> Self {
        let mut settings = Self {
            groups: BTreeMap::new(),
            current_group: None,
            search_filter: String::new(),
            show_modified_only: false,
            auto_save: true,
            backup_enabled: true,
            last_backup: 0,
            statistics: SettingsStatistics::default(),
        };

        // Inicializar configuraciones por defecto
        settings.initialize_default_settings();
        settings
    }

    fn initialize_default_settings(&mut self) {
        // Grupo de Sistema
        let mut system_group = ConfigGroup::new(
            String::from("Sistema"),
            String::from("Configuraciones generales del sistema"),
            ConfigCategory::System,
        );

        system_group.add_item(ConfigItem::new(
            String::from("system.hostname"),
            String::from("Nombre del Host"),
            String::from("Nombre del sistema"),
            ConfigCategory::System,
            ConfigType::String,
            String::from("reactos-kernel"),
        ).with_validation(ConfigValidation::with_pattern(String::from("^[a-zA-Z0-9-]+$")).required()));

        system_group.add_item(ConfigItem::new(
            String::from("system.timezone"),
            String::from("Zona Horaria"),
            String::from("Zona horaria del sistema"),
            ConfigCategory::System,
            ConfigType::String,
            String::from("UTC"),
        ));

        system_group.add_item(ConfigItem::new(
            String::from("system.language"),
            String::from("Idioma"),
            String::from("Idioma del sistema"),
            ConfigCategory::System,
            ConfigType::Enum,
            String::from("es"),
        ).with_validation(ConfigValidation::with_allowed_values(vec![
            String::from("es"), String::from("en"), String::from("fr"), String::from("de")
        ])));

        system_group.add_item(ConfigItem::new(
            String::from("system.debug_mode"),
            String::from("Modo Debug"),
            String::from("Habilitar modo de depuración"),
            ConfigCategory::System,
            ConfigType::Boolean,
            String::from("false"),
        ));

        self.groups.insert(String::from("system"), system_group);

        // Grupo de Red
        let mut network_group = ConfigGroup::new(
            String::from("Red"),
            String::from("Configuraciones de red"),
            ConfigCategory::Network,
        );

        network_group.add_item(ConfigItem::new(
            String::from("network.enable_dhcp"),
            String::from("Habilitar DHCP"),
            String::from("Configuración automática de IP"),
            ConfigCategory::Network,
            ConfigType::Boolean,
            String::from("true"),
        ));

        network_group.add_item(ConfigItem::new(
            String::from("network.ip_address"),
            String::from("Dirección IP"),
            String::from("Dirección IP estática"),
            ConfigCategory::Network,
            ConfigType::String,
            String::from("192.168.1.100"),
        ).with_validation(ConfigValidation::with_pattern(String::from("^\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}$"))));

        network_group.add_item(ConfigItem::new(
            String::from("network.port"),
            String::from("Puerto"),
            String::from("Puerto de red"),
            ConfigCategory::Network,
            ConfigType::Integer,
            String::from("8080"),
        ).with_validation(ConfigValidation::with_range(String::from("1"), String::from("65535"))));

        self.groups.insert(String::from("network"), network_group);

        // Grupo de Audio
        let mut audio_group = ConfigGroup::new(
            String::from("Audio"),
            String::from("Configuraciones de audio"),
            ConfigCategory::Audio,
        );

        audio_group.add_item(ConfigItem::new(
            String::from("audio.enabled"),
            String::from("Audio Habilitado"),
            String::from("Habilitar sistema de audio"),
            ConfigCategory::Audio,
            ConfigType::Boolean,
            String::from("true"),
        ));

        audio_group.add_item(ConfigItem::new(
            String::from("audio.volume"),
            String::from("Volumen"),
            String::from("Volumen del sistema"),
            ConfigCategory::Audio,
            ConfigType::Integer,
            String::from("50"),
        ).with_validation(ConfigValidation::with_range(String::from("0"), String::from("100"))));

        audio_group.add_item(ConfigItem::new(
            String::from("audio.sample_rate"),
            String::from("Frecuencia de Muestreo"),
            String::from("Frecuencia de muestreo de audio"),
            ConfigCategory::Audio,
            ConfigType::Integer,
            String::from("44100"),
        ).with_validation(ConfigValidation::with_allowed_values(vec![
            String::from("22050"), String::from("44100"), String::from("48000"), String::from("96000")
        ])));

        self.groups.insert(String::from("audio"), audio_group);

        // Grupo de Gráficos
        let mut graphics_group = ConfigGroup::new(
            String::from("Gráficos"),
            String::from("Configuraciones gráficas"),
            ConfigCategory::Graphics,
        );

        graphics_group.add_item(ConfigItem::new(
            String::from("graphics.resolution"),
            String::from("Resolución"),
            String::from("Resolución de pantalla"),
            ConfigCategory::Graphics,
            ConfigType::Enum,
            String::from("1024x768"),
        ).with_validation(ConfigValidation::with_allowed_values(vec![
            String::from("800x600"), String::from("1024x768"), String::from("1280x720"), String::from("1920x1080")
        ])));

        graphics_group.add_item(ConfigItem::new(
            String::from("graphics.color_depth"),
            String::from("Profundidad de Color"),
            String::from("Profundidad de color en bits"),
            ConfigCategory::Graphics,
            ConfigType::Integer,
            String::from("32"),
        ).with_validation(ConfigValidation::with_allowed_values(vec![
            String::from("16"), String::from("24"), String::from("32")
        ])));

        graphics_group.add_item(ConfigItem::new(
            String::from("graphics.theme"),
            String::from("Tema"),
            String::from("Tema visual del sistema"),
            ConfigCategory::Graphics,
            ConfigType::Enum,
            String::from("classic"),
        ).with_validation(ConfigValidation::with_allowed_values(vec![
            String::from("classic"), String::from("modern"), String::from("dark"), String::from("light")
        ])));

        self.groups.insert(String::from("graphics"), graphics_group);

        // Grupo de Rendimiento
        let mut performance_group = ConfigGroup::new(
            String::from("Rendimiento"),
            String::from("Configuraciones de rendimiento"),
            ConfigCategory::Performance,
        );

        performance_group.add_item(ConfigItem::new(
            String::from("performance.cpu_cores"),
            String::from("Núcleos de CPU"),
            String::from("Número de núcleos de CPU a utilizar"),
            ConfigCategory::Performance,
            ConfigType::Integer,
            String::from("1"),
        ).with_validation(ConfigValidation::with_range(String::from("1"), String::from("16"))));

        performance_group.add_item(ConfigItem::new(
            String::from("performance.memory_limit"),
            String::from("Límite de Memoria"),
            String::from("Límite de memoria en MB"),
            ConfigCategory::Performance,
            ConfigType::Integer,
            String::from("512"),
        ).with_validation(ConfigValidation::with_range(String::from("64"), String::from("4096"))));

        performance_group.add_item(ConfigItem::new(
            String::from("performance.cache_size"),
            String::from("Tamaño de Cache"),
            String::from("Tamaño de cache en MB"),
            ConfigCategory::Performance,
            ConfigType::Integer,
            String::from("64"),
        ).with_validation(ConfigValidation::with_range(String::from("16"), String::from("512"))));

        self.groups.insert(String::from("performance"), performance_group);

        // Actualizar estadísticas
        self.update_statistics();
    }

    pub fn get_item(&self, group_key: &str, item_key: &str) -> Option<&ConfigItem> {
        self.groups.get(group_key)?.get_item(item_key)
    }

    pub fn get_item_mut(&mut self, group_key: &str, item_key: &str) -> Option<&mut ConfigItem> {
        self.groups.get_mut(group_key)?.get_item_mut(item_key)
    }

    pub fn set_value(&mut self, group_key: &str, item_key: &str, value: String) -> bool {
        if let Some(item) = self.get_item_mut(group_key, item_key) {
            item.set_value(value)
        } else {
            false
        }
    }

    pub fn get_value(&self, group_key: &str, item_key: &str) -> Option<String> {
        self.get_item(group_key, item_key).map(|item| item.current_value.clone())
    }

    pub fn reset_item_to_default(&mut self, group_key: &str, item_key: &str) -> bool {
        if let Some(item) = self.get_item_mut(group_key, item_key) {
            item.reset_to_default();
            true
        } else {
            false
        }
    }

    pub fn reset_group_to_default(&mut self, group_key: &str) -> bool {
        if let Some(group) = self.groups.get_mut(group_key) {
            group.reset_all_to_default();
            true
        } else {
            false
        }
    }

    pub fn reset_all_to_default(&mut self) {
        for group in self.groups.values_mut() {
            group.reset_all_to_default();
        }
        self.update_statistics();
    }

    pub fn get_modified_items(&self) -> Vec<(&String, &String, &ConfigItem)> {
        let mut modified = Vec::new();
        for (group_key, group) in &self.groups {
            for (item_key, item) in &group.items {
                if item.is_modified {
                    modified.push((group_key, item_key, item));
                }
            }
        }
        modified
    }

    pub fn save_settings(&mut self) -> bool {
        // Simular guardado de configuraciones
        self.statistics.save_count += 1;
        self.statistics.last_save = get_system_time();
        
        // Marcar todos los elementos como no modificados
        for group in self.groups.values_mut() {
            for item in group.items.values_mut() {
                item.is_modified = false;
            }
        }
        
        self.update_statistics();
        true
    }

    pub fn load_settings(&mut self) -> bool {
        // Simular carga de configuraciones
        self.statistics.load_count += 1;
        self.update_statistics();
        true
    }

    pub fn create_backup(&mut self) -> bool {
        if self.backup_enabled {
            self.last_backup = get_system_time();
            true
        } else {
            false
        }
    }

    pub fn restore_from_backup(&mut self) -> bool {
        // Simular restauración desde backup
        self.load_settings();
        true
    }

    pub fn search_items(&self, query: &str) -> Vec<(&String, &String, &ConfigItem)> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        for (group_key, group) in &self.groups {
            for (item_key, item) in &group.items {
                if item.name.to_lowercase().contains(&query_lower) ||
                   item.description.to_lowercase().contains(&query_lower) ||
                   item.key.to_lowercase().contains(&query_lower) {
                    results.push((group_key, item_key, item));
                }
            }
        }
        
        results
    }

    pub fn get_groups_by_category(&self, category: ConfigCategory) -> Vec<&ConfigGroup> {
        self.groups.values().filter(|group| group.category == category).collect()
    }

    pub fn update_statistics(&mut self) {
        self.statistics.total_items = self.groups.values().map(|g| g.items.len()).sum();
        self.statistics.modified_items = self.get_modified_items().len();
        self.statistics.groups_count = self.groups.len();
    }

    pub fn get_info(&self) -> String {
        format!(
            "Panel de Configuración - Grupos: {} | Elementos: {} | Modificados: {}",
            self.statistics.groups_count,
            self.statistics.total_items,
            self.statistics.modified_items
        )
    }

    pub fn get_statistics(&self) -> String {
        format!(
            "Elementos: {} | Modificados: {} | Grupos: {} | Guardados: {} | Cargados: {} | Errores: {}",
            self.statistics.total_items,
            self.statistics.modified_items,
            self.statistics.groups_count,
            self.statistics.save_count,
            self.statistics.load_count,
            self.statistics.error_count
        )
    }
}

// Funciones auxiliares
fn get_system_time() -> u64 {
    // Simulación simple de tiempo del sistema
    1234567890
}

fn validate_pattern(value: &str, pattern: &str) -> bool {
    // Implementación simplificada de validación de patrón
    // En un sistema real, se usaría una librería de regex
    match pattern {
        "^[a-zA-Z0-9-]+$" => value.chars().all(|c| c.is_alphanumeric() || c == '-'),
        "^\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}$" => {
            let parts: Vec<&str> = value.split('.').collect();
            parts.len() == 4 && parts.iter().all(|part| {
                part.parse::<u8>().is_ok()
            })
        },
        _ => true, // Patrón no reconocido, permitir
    }
}

// Panel global de configuración
use spin::Mutex;

pub static SYSTEM_SETTINGS: Mutex<Option<SystemSettings>> = Mutex::new(None);

/// Inicializar el panel de configuración
pub fn init_system_settings() {
    let mut settings = SYSTEM_SETTINGS.lock();
    *settings = Some(SystemSettings::new());
    crate::logging::info("system_settings", "Panel de configuración inicializado");
}

/// Obtener información del panel de configuración
pub fn get_system_settings_info() -> String {
    if let Some(ref settings) = *SYSTEM_SETTINGS.lock() {
        settings.get_info()
    } else {
        String::from("Panel de configuración no inicializado")
    }
}

/// Obtener estadísticas del panel de configuración
pub fn get_system_settings_stats() -> String {
    if let Some(ref settings) = *SYSTEM_SETTINGS.lock() {
        settings.get_statistics()
    } else {
        String::from("Panel de configuración no inicializado")
    }
}

/// Obtener valor de configuración
pub fn get_config_value(group: &str, key: &str) -> Option<String> {
    let settings = SYSTEM_SETTINGS.lock();
    if let Some(ref s) = *settings {
        s.get_value(group, key)
    } else {
        None
    }
}

/// Establecer valor de configuración
pub fn set_config_value(group: &str, key: &str, value: String) -> bool {
    let mut settings = SYSTEM_SETTINGS.lock();
    if let Some(ref mut s) = *settings {
        s.set_value(group, key, value)
    } else {
        false
    }
}

/// Guardar configuraciones
pub fn save_configurations() -> bool {
    let mut settings = SYSTEM_SETTINGS.lock();
    if let Some(ref mut s) = *settings {
        s.save_settings()
    } else {
        false
    }
}

/// Cargar configuraciones
pub fn load_configurations() -> bool {
    let mut settings = SYSTEM_SETTINGS.lock();
    if let Some(ref mut s) = *settings {
        s.load_settings()
    } else {
        false
    }
}

/// Verificar si el panel de configuración está disponible
pub fn is_system_settings_available() -> bool {
    let settings = SYSTEM_SETTINGS.lock();
    settings.is_some()
}

//! Sistema de detección automática de hardware para ReactOS Rust Kernel
//! 
//! Este módulo implementa:
//! - Detección automática de dispositivos
//! - Gestión de recursos de hardware
//! - Drivers plug-and-play
//! - Monitoreo de temperatura y estado
//! - Configuración automática de dispositivos

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::format;
use spin::Mutex;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

/// Tipo de dispositivo
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceType {
    Processor,      // Procesador
    Memory,         // Memoria RAM
    Storage,        // Almacenamiento (HDD, SSD)
    Network,        // Tarjeta de red
    Audio,          // Tarjeta de audio
    Graphics,       // Tarjeta gráfica
    Input,          // Dispositivos de entrada (teclado, mouse)
    Serial,         // Puerto serie
    Parallel,       // Puerto paralelo
    USB,            // Dispositivo USB
    PCI,            // Dispositivo PCI
    Unknown,        // Desconocido
}

/// Estado del dispositivo
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    Detected,       // Detectado pero no inicializado
    Initialized,    // Inicializado y funcionando
    Error,          // Error en el dispositivo
    Disabled,       // Deshabilitado
    Unavailable,    // No disponible
}

/// Información del dispositivo
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: usize,
    pub name: String,
    pub device_type: DeviceType,
    pub vendor: String,
    pub model: String,
    pub version: String,
    pub state: DeviceState,
    pub resources: Vec<Resource>,
    pub temperature: i8,        // Temperatura en Celsius
    pub power_usage: u16,       // Consumo de energía en mW
    pub last_check: u64,
    pub error_count: u32,
}

/// Recurso de hardware
#[derive(Debug, Clone)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub start_address: u32,
    pub end_address: u32,
    pub size: u32,
    pub in_use: bool,
}

/// Tipo de recurso
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Memory,         // Memoria
    IO,             // Puerto I/O
    IRQ,            // Línea de interrupción
    DMA,            // Canal DMA
}

impl DeviceInfo {
    /// Crear nueva información de dispositivo
    pub fn new(id: usize, name: String, device_type: DeviceType) -> Self {
        Self {
            id,
            name,
            device_type,
            vendor: String::new(),
            model: String::new(),
            version: String::new(),
            state: DeviceState::Detected,
            resources: Vec::new(),
            temperature: 0,
            power_usage: 0,
            last_check: 0,
            error_count: 0,
        }
    }
    
    /// Agregar recurso
    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
    
    /// Obtener información del dispositivo
    pub fn get_info(&self) -> String {
        format!(
            "ID: {} | {} | {:?} | Estado: {:?} | Temp: {}°C | Poder: {}mW | Errores: {}",
            self.id,
            self.name,
            self.device_type,
            self.state,
            self.temperature,
            self.power_usage,
            self.error_count
        )
    }
    
    /// Verificar estado del dispositivo
    pub fn check_status(&mut self) -> bool {
        self.last_check = 1; // Simulado
        
        // Simular verificación de estado
        match self.device_type {
            DeviceType::Processor => {
                self.temperature = 45; // CPU a 45°C
                self.power_usage = 6500; // 6.5W (ajustado para u16)
                self.state = DeviceState::Initialized;
            },
            DeviceType::Memory => {
                self.temperature = 35; // RAM a 35°C
                self.power_usage = 5000; // 5W
                self.state = DeviceState::Initialized;
            },
            DeviceType::Storage => {
                self.temperature = 40; // Disco a 40°C
                self.power_usage = 8000; // 8W
                self.state = DeviceState::Initialized;
            },
            DeviceType::Network => {
                self.temperature = 30; // Red a 30°C
                self.power_usage = 2000; // 2W
                self.state = DeviceState::Initialized;
            },
            DeviceType::Audio => {
                self.temperature = 25; // Audio a 25°C
                self.power_usage = 1000; // 1W
                self.state = DeviceState::Initialized;
            },
            DeviceType::Graphics => {
                self.temperature = 55; // GPU a 55°C
                self.power_usage = 15000; // 15W (ajustado para u16)
                self.state = DeviceState::Initialized;
            },
            _ => {
                self.temperature = 20;
                self.power_usage = 500;
                self.state = DeviceState::Initialized;
            },
        }
        
        true
    }
}

/// Detector de hardware
pub struct HardwareDetector {
    pub devices: BTreeMap<usize, DeviceInfo>,
    pub next_device_id: AtomicUsize,
    pub detection_enabled: AtomicBool,
    pub auto_configure: AtomicBool,
    pub total_detected: AtomicUsize,
    pub total_initialized: AtomicUsize,
    pub total_errors: AtomicUsize,
}

impl HardwareDetector {
    /// Crear un nuevo detector de hardware
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            next_device_id: AtomicUsize::new(1),
            detection_enabled: AtomicBool::new(true),
            auto_configure: AtomicBool::new(true),
            total_detected: AtomicUsize::new(0),
            total_initialized: AtomicUsize::new(0),
            total_errors: AtomicUsize::new(0),
        }
    }
    
    /// Detectar dispositivos automáticamente
    pub fn detect_devices(&mut self) -> usize {
        if !self.detection_enabled.load(Ordering::SeqCst) {
            return 0;
        }
        
        let mut detected_count = 0;
        
        // Detectar procesador
        if self.detect_processor() {
            detected_count += 1;
        }
        
        // Detectar memoria
        if self.detect_memory() {
            detected_count += 1;
        }
        
        // Detectar almacenamiento
        if self.detect_storage() {
            detected_count += 1;
        }
        
        // Detectar red
        if self.detect_network() {
            detected_count += 1;
        }
        
        // Detectar audio
        if self.detect_audio() {
            detected_count += 1;
        }
        
        // Detectar gráficos
        if self.detect_graphics() {
            detected_count += 1;
        }
        
        // Detectar dispositivos de entrada
        if self.detect_input_devices() {
            detected_count += 1;
        }
        
        // Detectar puertos serie/paralelo
        if self.detect_ports() {
            detected_count += 1;
        }
        
        self.total_detected.fetch_add(detected_count, Ordering::SeqCst);
        detected_count
    }
    
    /// Detectar procesador
    fn detect_processor(&mut self) -> bool {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let mut device = DeviceInfo::new(id, "Intel x86_64 Processor".to_string(), DeviceType::Processor);
        
        device.vendor = "Intel".to_string();
        device.model = "x86_64".to_string();
        device.version = "1.0".to_string();
        
        // Agregar recursos del procesador
        device.add_resource(Resource {
            resource_type: ResourceType::Memory,
            start_address: 0x0,
            end_address: 0xFFFFFFFF,
            size: 0xFFFFFFFF,
            in_use: true,
        });
        
        device.check_status();
        self.devices.insert(id, device);
        
        crate::logging::info("hardware", "Procesador detectado e inicializado");
        true
    }
    
    /// Detectar memoria
    fn detect_memory(&mut self) -> bool {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let mut device = DeviceInfo::new(id, "System RAM".to_string(), DeviceType::Memory);
        
        device.vendor = "Generic".to_string();
        device.model = "DDR4".to_string();
        device.version = "1.0".to_string();
        
        // Agregar recursos de memoria
        device.add_resource(Resource {
            resource_type: ResourceType::Memory,
            start_address: 0x100000,
            end_address: 0x80000000,
            size: 0x7FF00000,
            in_use: true,
        });
        
        device.check_status();
        self.devices.insert(id, device);
        
        crate::logging::info("hardware", "Memoria RAM detectada e inicializada");
        true
    }
    
    /// Detectar almacenamiento
    fn detect_storage(&mut self) -> bool {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let mut device = DeviceInfo::new(id, "Virtual Disk".to_string(), DeviceType::Storage);
        
        device.vendor = "QEMU".to_string();
        device.model = "Virtual Disk".to_string();
        device.version = "1.0".to_string();
        
        // Agregar recursos de almacenamiento
        device.add_resource(Resource {
            resource_type: ResourceType::IO,
            start_address: 0x1F0,
            end_address: 0x1F7,
            size: 8,
            in_use: true,
        });
        
        device.check_status();
        self.devices.insert(id, device);
        
        crate::logging::info("hardware", "Dispositivo de almacenamiento detectado e inicializado");
        true
    }
    
    /// Detectar red
    fn detect_network(&mut self) -> bool {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let mut device = DeviceInfo::new(id, "Virtual Network Card".to_string(), DeviceType::Network);
        
        device.vendor = "QEMU".to_string();
        device.model = "E1000".to_string();
        device.version = "1.0".to_string();
        
        // Agregar recursos de red
        device.add_resource(Resource {
            resource_type: ResourceType::IO,
            start_address: 0xCF8,
            end_address: 0xCFF,
            size: 8,
            in_use: true,
        });
        
        device.add_resource(Resource {
            resource_type: ResourceType::IRQ,
            start_address: 11,
            end_address: 11,
            size: 1,
            in_use: true,
        });
        
        device.check_status();
        self.devices.insert(id, device);
        
        crate::logging::info("hardware", "Tarjeta de red detectada e inicializada");
        true
    }
    
    /// Detectar audio
    fn detect_audio(&mut self) -> bool {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let mut device = DeviceInfo::new(id, "Virtual Audio Device".to_string(), DeviceType::Audio);
        
        device.vendor = "QEMU".to_string();
        device.model = "AC97".to_string();
        device.version = "1.0".to_string();
        
        // Agregar recursos de audio
        device.add_resource(Resource {
            resource_type: ResourceType::IO,
            start_address: 0x220,
            end_address: 0x22F,
            size: 16,
            in_use: true,
        });
        
        device.add_resource(Resource {
            resource_type: ResourceType::IRQ,
            start_address: 5,
            end_address: 5,
            size: 1,
            in_use: true,
        });
        
        device.check_status();
        self.devices.insert(id, device);
        
        crate::logging::info("hardware", "Dispositivo de audio detectado e inicializado");
        true
    }
    
    /// Detectar gráficos
    fn detect_graphics(&mut self) -> bool {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let mut device = DeviceInfo::new(id, "VGA Graphics Card".to_string(), DeviceType::Graphics);
        
        device.vendor = "Generic".to_string();
        device.model = "VGA".to_string();
        device.version = "1.0".to_string();
        
        // Agregar recursos de gráficos
        device.add_resource(Resource {
            resource_type: ResourceType::Memory,
            start_address: 0xA0000,
            end_address: 0xBFFFF,
            size: 0x20000,
            in_use: true,
        });
        
        device.add_resource(Resource {
            resource_type: ResourceType::IO,
            start_address: 0x3C0,
            end_address: 0x3DF,
            size: 32,
            in_use: true,
        });
        
        device.check_status();
        self.devices.insert(id, device);
        
        crate::logging::info("hardware", "Tarjeta gráfica detectada e inicializada");
        true
    }
    
    /// Detectar dispositivos de entrada
    fn detect_input_devices(&mut self) -> bool {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let mut device = DeviceInfo::new(id, "PS/2 Input Devices".to_string(), DeviceType::Input);
        
        device.vendor = "Generic".to_string();
        device.model = "PS/2".to_string();
        device.version = "1.0".to_string();
        
        // Agregar recursos de entrada
        device.add_resource(Resource {
            resource_type: ResourceType::IO,
            start_address: 0x60,
            end_address: 0x64,
            size: 5,
            in_use: true,
        });
        
        device.add_resource(Resource {
            resource_type: ResourceType::IRQ,
            start_address: 1,
            end_address: 1,
            size: 1,
            in_use: true,
        });
        
        device.check_status();
        self.devices.insert(id, device);
        
        crate::logging::info("hardware", "Dispositivos de entrada detectados e inicializados");
        true
    }
    
    /// Detectar puertos
    fn detect_ports(&mut self) -> bool {
        let id = self.next_device_id.fetch_add(1, Ordering::SeqCst);
        let mut device = DeviceInfo::new(id, "Serial/Parallel Ports".to_string(), DeviceType::Serial);
        
        device.vendor = "Generic".to_string();
        device.model = "UART".to_string();
        device.version = "1.0".to_string();
        
        // Agregar recursos de puertos
        device.add_resource(Resource {
            resource_type: ResourceType::IO,
            start_address: 0x3F8,
            end_address: 0x3FF,
            size: 8,
            in_use: true,
        });
        
        device.check_status();
        self.devices.insert(id, device);
        
        crate::logging::info("hardware", "Puertos serie/paralelo detectados e inicializados");
        true
    }
    
    /// Inicializar dispositivo
    pub fn initialize_device(&mut self, device_id: usize) -> bool {
        if let Some(device) = self.devices.get_mut(&device_id) {
            if device.check_status() {
                device.state = DeviceState::Initialized;
                self.total_initialized.fetch_add(1, Ordering::SeqCst);
                crate::logging::info("hardware", &format!("Dispositivo {} inicializado", device.name));
                true
            } else {
                device.state = DeviceState::Error;
                device.error_count += 1;
                self.total_errors.fetch_add(1, Ordering::SeqCst);
                crate::logging::error("hardware", &format!("Error al inicializar dispositivo {}", device.name));
                false
            }
        } else {
            false
        }
    }
    
    /// Inicializar todos los dispositivos
    pub fn initialize_all_devices(&mut self) -> usize {
        let mut initialized_count = 0;
        
        for device_id in self.devices.keys().cloned().collect::<Vec<_>>() {
            if self.initialize_device(device_id) {
                initialized_count += 1;
            }
        }
        
        initialized_count
    }
    
    /// Obtener información del detector
    pub fn get_info(&self) -> String {
        let detected = self.total_detected.load(Ordering::SeqCst);
        let initialized = self.total_initialized.load(Ordering::SeqCst);
        let errors = self.total_errors.load(Ordering::SeqCst);
        
        format!(
            "Hardware: {} detectados | {} inicializados | {} errores | Auto-config: {}",
            detected,
            initialized,
            errors,
            if self.auto_configure.load(Ordering::SeqCst) { "Sí" } else { "No" }
        )
    }
    
    /// Obtener estadísticas detalladas
    pub fn get_detailed_stats(&self) -> String {
        let mut stats = format!(
            "=== ESTADÍSTICAS DE HARDWARE ===\nDispositivos detectados: {}\nDispositivos inicializados: {}\nErrores: {}\n\n",
            self.total_detected.load(Ordering::SeqCst),
            self.total_initialized.load(Ordering::SeqCst),
            self.total_errors.load(Ordering::SeqCst)
        );
        
        stats.push_str("=== DISPOSITIVOS ===\n");
        for device in self.devices.values() {
            stats.push_str(&format!("{}\n", device.get_info()));
        }
        
        stats
    }
    
    /// Monitorear temperatura del sistema
    pub fn monitor_temperature(&mut self) -> String {
        let mut max_temp = -100;
        let mut total_temp = 0;
        let mut device_count = 0;
        
        for device in self.devices.values_mut() {
            device.check_status();
            if device.temperature > max_temp {
                max_temp = device.temperature;
            }
            total_temp += device.temperature as i32;
            device_count += 1;
        }
        
        let avg_temp = if device_count > 0 { total_temp / device_count } else { 0 };
        
        format!(
            "Temperatura: Máxima {}°C | Promedio {}°C | Dispositivos: {}",
            max_temp,
            avg_temp,
            device_count
        )
    }
}

/// Instancia global del detector de hardware
static HARDWARE_DETECTOR: Mutex<Option<HardwareDetector>> = Mutex::new(None);

/// Inicializar el detector de hardware
pub fn init_hardware() -> bool {
    let mut detector_guard = HARDWARE_DETECTOR.lock();
    if detector_guard.is_none() {
        let mut detector = HardwareDetector::new();
        
        // Detectar dispositivos
        let detected = detector.detect_devices();
        crate::logging::info("hardware", &format!("{} dispositivos detectados", detected));
        
        // Inicializar dispositivos
        let initialized = detector.initialize_all_devices();
        crate::logging::info("hardware", &format!("{} dispositivos inicializados", initialized));
        
        *detector_guard = Some(detector);
        return true;
    }
    false
}

/// Detectar dispositivos
pub fn detect_devices() -> usize {
    let mut detector_guard = HARDWARE_DETECTOR.lock();
    if let Some(ref mut detector) = *detector_guard {
        detector.detect_devices()
    } else {
        0
    }
}

/// Inicializar dispositivo específico
pub fn initialize_device(device_id: usize) -> bool {
    let mut detector_guard = HARDWARE_DETECTOR.lock();
    if let Some(ref mut detector) = *detector_guard {
        detector.initialize_device(device_id)
    } else {
        false
    }
}

/// Obtener información del detector de hardware
pub fn get_hardware_info() -> String {
    let detector_guard = HARDWARE_DETECTOR.lock();
    if let Some(ref detector) = *detector_guard {
        detector.get_info()
    } else {
        String::from("Detector de hardware: No disponible")
    }
}

/// Obtener estadísticas detalladas de hardware
pub fn get_hardware_stats() -> String {
    let detector_guard = HARDWARE_DETECTOR.lock();
    if let Some(ref detector) = *detector_guard {
        detector.get_detailed_stats()
    } else {
        String::from("Estadísticas de hardware: No disponible")
    }
}

/// Monitorear temperatura del sistema
pub fn monitor_temperature() -> String {
    let mut detector_guard = HARDWARE_DETECTOR.lock();
    if let Some(ref mut detector) = *detector_guard {
        detector.monitor_temperature()
    } else {
        String::from("Monitoreo de temperatura: No disponible")
    }
}

/// Verificar si el detector de hardware está disponible
pub fn is_hardware_available() -> bool {
    let detector_guard = HARDWARE_DETECTOR.lock();
    detector_guard.is_some()
}

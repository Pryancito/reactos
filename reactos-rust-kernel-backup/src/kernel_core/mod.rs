//! # Core Kernel Components
//! 
//! Componentes principales del kernel en Rust

pub mod memory;
pub mod process;
pub mod interrupt;
pub mod io;
pub mod security;
pub mod power;
pub mod x86_64;
pub mod compatibility;
pub mod graphics;
pub mod audio;
pub mod usb;
pub mod virtualization;
pub mod monitoring;
pub mod advanced_security;
pub mod storage;
pub mod hal;
pub mod time;
pub mod services;
pub mod caching;
pub mod resource_management;
pub mod system_calls;
pub mod networking;

// Re-exportar tipos importantes
pub use memory::{MemoryManager, MemoryResult, MemoryError, MemoryInfo, MemoryFlags};
pub use process::{ProcessManager, Process, Thread, ProcessState, ThreadState, Priority, ProcessId, ThreadId, SystemInfo};
pub use process::scheduler::{Scheduler, SchedulingAlgorithm, SchedulingInfo};
pub use interrupt::{InterruptManager, InterruptResult, InterruptError, InterruptInfo, InterruptStats, InterruptNumber, InterruptType, InterruptPriority};
pub use io::{DeviceManager, FileSystemManager, NetworkManager, DriverManager, Device, FileSystem, NetworkInterface, Driver};
pub use security::{SecurityManager, SecurityLevel, ThreatType, ThreatInfo, SecurityStats};
pub use security::access_control::{AccessControlManager, User, Resource, Permission, AccessLevel, AccessStats};
pub use power::{PowerManager, PowerState, PowerLevel, PowerInfo, PowerStats};
pub use power::cpu::{CpuPowerManager, CpuState, CpuInfo, CpuStats};
pub use x86_64::{X86_64Manager, ExecutionMode, ApplicationType, ArchitectureInfo, X86_64Features, X86_64Stats};
pub use compatibility::{CompatibilityManager, CompatibilityType, CompatibilityLevel, CompatibilityInfo, CompatibilityStats};
pub use graphics::{GraphicsManager, ColorMode, Resolution, FramebufferInfo, GraphicsStats};
pub use audio::{AudioManager, AudioFormat, AudioConfig, AudioDeviceInfo, AudioDeviceType, AudioStats};
pub use usb::{UsbManager, UsbDeviceType, UsbSpeed, UsbDeviceState, UsbDeviceInfo, UsbControllerInfo, UsbControllerType, UsbStats};
pub use virtualization::{VirtualizationManager, VirtualizationType, VmState, VmInfo, VirtualizationStats};
pub use monitoring::{MonitoringManager, MetricType, MetricInfo, BenchmarkResult, MonitoringStats};
pub use advanced_security::{AdvancedSecurityManager, EncryptionAlgorithm, AuthenticationType, SecurityState, EncryptionInfo, SecurityScanResult, AdvancedSecurityStats};
pub use storage::{StorageManager, StorageDeviceType, StorageDeviceState, StorageDeviceInfo, StorageStats};
pub use storage::disk_manager::{DiskManager, PartitionTableType, PartitionType, PartitionState, PartitionInfo, DiskInfo, DiskStats};
pub use storage::raid::{RaidManager, RaidLevel, RaidArrayState, RaidArrayInfo, RaidStats};
pub use storage::ssd_optimization::{SsdOptimizationManager, SsdOptimizationType, SsdOptimizationState, SsdOptimizationInfo, SsdOptimizationStats};
pub use storage::encryption::{StorageEncryptionManager, EncryptionAlgorithm as StorageEncryptionAlgorithm, EncryptionState, EncryptionInfo as StorageEncryptionInfo, EncryptionStats};
pub use hal::{HalManager, HalDeviceType, HalDeviceState, HalDeviceInfo, HalStats};
pub use hal::acpi::{AcpiManager, AcpiTableType, AcpiState, AcpiTableInfo, AcpiStats};
pub use hal::pci::{PciManager, PciDeviceType, PciDeviceState, PciDeviceInfo, PciStats};
pub use hal::irq::{IrqManager, IrqType, IrqPriority, IrqState, IrqInfo, IrqStats};
pub use hal::dma::{DmaManager, DmaTransferType, DmaTransferState, DmaPriority, DmaTransferInfo, DmaStats};
pub use time::{TimeManager, ClockType, TimeResolution, TimeInfo, TimeStats};
pub use time::clock_source::{ClockSourceManager, ClockSourceType, ClockSourceState, ClockSourceInfo, ClockSourceStats};
pub use time::synchronization::{SynchronizationManager, SyncPrimitiveType, SyncPrimitiveState, SyncPrimitiveInfo, SynchronizationStats};
pub use time::timer_system::{TimerManager, TimerType, TimerState, TimerInfo, TimerStats};
pub use time::ntp_support::{NtpManager, NtpServerType, NtpServerState, NtpServerInfo, NtpStats};
pub use services::{ServiceManager, ServiceType, ServiceState, ServiceInfo, ServiceStats};
pub use services::event_system::{EventManager, EventType, EventPriority, EventState, EventInfo, EventStats};
pub use services::logging_system::{LoggingManager, LogLevel, LogType, LogDestination, LogEntry, LoggingStats};
pub use services::configuration_manager::{ConfigurationManager, ConfigType, ConfigValueType, ConfigInfo, ConfigurationStats};
pub use services::registry_manager::{RegistryManager, RegistryKeyType, RegistryValueType, RegistryKeyState, RegistryKeyInfo, RegistryValueInfo, RegistryStats};
pub use caching::{CacheManager, CacheType, CacheState, CacheInfo, CacheStats};
pub use caching::buffer_cache::{BufferCacheManager, BufferType, BufferState, BufferInfo, BufferCacheStats};
pub use caching::page_cache::{PageCacheManager, PageType, PageState, PageInfo, PageCacheStats};
pub use caching::disk_cache::{DiskCacheManager, DiskCacheType, DiskCacheState, DiskCacheInfo, DiskCacheStats};
pub use caching::network_cache::{NetworkCacheManager, NetworkCacheType, NetworkCacheState, NetworkCacheInfo, NetworkCacheStats};
pub use caching::memory_pool::{MemoryPoolManager, MemoryPoolType, MemoryPoolState, MemoryPoolInfo, MemoryPoolStats};
pub use resource_management::{ResourceManager, ResourceType, ResourceState, ResourceInfo, ResourceStats};
pub use resource_management::{MonitoringManager as ResourceMonitoringManager, MonitoringMetric, MonitoringState, MonitoringInfo, MonitoringStats as ResourceMonitoringStats};
pub use resource_management::{OptimizationManager, OptimizationType, OptimizationState, OptimizationInfo, OptimizationStats};
pub use system_calls::{SyscallHandler, SyscallTable, SyscallInterface, SyscallValidator, SyscallProfiler};
pub use system_calls::{SyscallInfo, SyscallFlags, SyscallCategory, SyscallHandlerStats, SyscallTableStats, SyscallInterfaceStats, ValidationStats, PerformanceStats};
pub use networking::{NetworkStack, ProtocolStack, NetworkInterface as NetworkInterfaceManager, NetworkDriver, NetworkSecurity, NetworkMonitoring};
pub use networking::{NetworkStackType, NetworkStackState, NetworkStackConfig, NetworkPacket, NetworkStackStats, ProtocolType, ProtocolInfo, ProtocolStats, ProtocolStackStats};
pub use networking::{InterfaceType, InterfaceStatus, InterfaceInfo, InterfaceStats, NetworkInterfaceStats, DriverType, DriverStatus, DriverInfo, DriverStats, NetworkDriverStats};
pub use networking::{SecurityRuleType, SecurityAction, SecurityPriority, SecurityRule, SecurityStats as NetworkSecurityStats, MonitorType, MonitorStatus, MonitorInfo, MonitorStats, NetworkMonitoringStats};

/// Inicializar todos los componentes del core
pub fn init() -> Result<(), &'static str> {
    // Inicializar memory manager
    memory::init().map_err(|_| "Failed to initialize memory manager")?;
    
    // Inicializar process manager
    process::init().map_err(|_| "Failed to initialize process manager")?;
    
    // Inicializar interrupt manager
    interrupt::init().map_err(|_| "Failed to initialize interrupt manager")?;
    
    // Inicializar sistema I/O
    io::init().map_err(|_| "Failed to initialize I/O system")?;
    
    // Inicializar sistema de seguridad
    security::init().map_err(|_| "Failed to initialize security system")?;
    
    // Inicializar sistema de energía
    power::init().map_err(|_| "Failed to initialize power system")?;
    
    // Inicializar soporte x86_64
    x86_64::init().map_err(|_| "Failed to initialize x86_64 support")?;
    
    // Inicializar capa de compatibilidad
    compatibility::init().map_err(|_| "Failed to initialize compatibility layer")?;
    
    // Inicializar sistema gráfico
    graphics::init().map_err(|_| "Failed to initialize graphics system")?;
    
    // Inicializar sistema de audio
    audio::init().map_err(|_| "Failed to initialize audio system")?;
    
    // Inicializar soporte USB
    usb::init().map_err(|_| "Failed to initialize USB support")?;
    
    // Inicializar soporte de virtualización
    virtualization::init().map_err(|_| "Failed to initialize virtualization support")?;
    
    // Inicializar monitoreo de rendimiento
    monitoring::init().map_err(|_| "Failed to initialize performance monitoring")?;
    
    // Inicializar seguridad avanzada
    advanced_security::init().map_err(|_| "Failed to initialize advanced security")?;
    
    // Inicializar sistema de almacenamiento
    storage::init().map_err(|_| "Failed to initialize storage system")?;
    
    // Inicializar Hardware Abstraction Layer
    hal::init().map_err(|_| "Failed to initialize HAL")?;
    
    // Inicializar Time & Synchronization
    time::init().map_err(|_| "Failed to initialize Time & Synchronization")?;
    
        // Inicializar System Services
    services::init().map_err(|_| "Failed to initialize System Services")?;
    // Inicializar Caching & Buffering
    caching::init().map_err(|_| "Failed to initialize Caching & Buffering")?;
    
    // Inicializar Resource Management
    resource_management::init().map_err(|_| "Failed to initialize Resource Management")?;
    
    // Inicializar System Calls
    system_calls::init();
    
    // Inicializar Networking
    networking::init();

    Ok(())
}
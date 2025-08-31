/**
 * ENCABEZADO DEL CARGADOR DE REACTOS PARA UEFI
 * 
 * Este archivo contiene las definiciones necesarias para cargar
 * el kernel de ReactOS desde un bootloader UEFI nativo.
 * 
 * Compatible con sistemas UEFI estrictos como ASUS 10ª generación
 */

#ifndef LOADER_H
#define LOADER_H

#include "uefi.h"

// ============================================================================
// DEFINICIONES DEL CARGADOR DE REACTOS
// ============================================================================

// Identificador del kernel de ReactOS
#define REACTOS_KERNEL_SIGNATURE     0x4E544F53  // "NTOS"
#define REACTOS_KERNEL_VERSION       0x0100      // 1.0

// Estructura de información del kernel
typedef struct {
    UINT32 Signature;
    UINT16 Version;
    UINT16 Revision;
    UINT32 EntryPoint;
    UINT32 ImageBase;
    UINT32 ImageSize;
    UINT32 Subsystem;
    UINT32 DllCharacteristics;
    UINT32 SizeOfStackReserve;
    UINT32 SizeOfStackCommit;
    UINT32 SizeOfHeapReserve;
    UINT32 SizeOfHeapCommit;
    UINT32 LoaderFlags;
    UINT32 NumberOfRvaAndSizes;
} REACTOS_KERNEL_HEADER;

// Estructura de opciones de arranque
typedef struct {
    UINT8 SafeMode;
    UINT8 DebugMode;
    UINT8 VGAMode;
    UINT8 NoGuiBoot;
    UINT8 FastDetect;
    UINT8 UsbBoot;
    CHAR8 KernelPath[256];
    CHAR8 SystemRoot[256];
    CHAR8 BootPath[256];
    UINT32 Timeout;
    UINT32 BootAttempts;
} REACTOS_BOOT_OPTIONS;

// Estructura de estado del sistema
typedef struct {
    UINT8 LastBootSucceeded;
    UINT8 LastBootShutdown;
    UINT32 BootAttemptCount;
    UINT32 LastBootCheckpoint;
    UINT64 TotalMemory;
    UINT64 AvailableMemory;
    UINT32 CpuCount;
    UINT32 CpuFeatures;
} REACTOS_SYSTEM_STATUS;

// ============================================================================
// FUNCIONES DEL CARGADOR
// ============================================================================

// Función principal de carga del kernel
UINT64 LoadReactOSKernel(
    VOID *ImageHandle,
    EFI_SYSTEM_TABLE *SystemTable,
    REACTOS_BOOT_OPTIONS *BootOptions
);

// Función para detectar ReactOS en el sistema
UINT64 DetectReactOS(
    VOID *ImageHandle,
    EFI_SYSTEM_TABLE *SystemTable,
    REACTOS_BOOT_OPTIONS *BootOptions
);

// Función para cargar el kernel desde archivo
UINT64 LoadKernelFromFile(
    CHAR16 *FilePath,
    VOID **KernelImage,
    UINT64 *KernelSize
);

// Función para verificar la integridad del kernel
UINT64 VerifyKernelIntegrity(
    VOID *KernelImage,
    UINT64 KernelSize
);

// Función para preparar la memoria para el kernel
UINT64 PrepareKernelMemory(
    EFI_SYSTEM_TABLE *SystemTable,
    VOID *KernelImage,
    UINT64 KernelSize
);

// Función para transferir control al kernel
UINT64 TransferToKernel(
    VOID *KernelEntryPoint,
    EFI_SYSTEM_TABLE *SystemTable,
    REACTOS_BOOT_OPTIONS *BootOptions
);

// ============================================================================
// FUNCIONES DE CONFIGURACIÓN
// ============================================================================

// Función para cargar configuración de arranque
UINT64 LoadBootConfiguration(
    VOID *ImageHandle,
    EFI_SYSTEM_TABLE *SystemTable,
    REACTOS_BOOT_OPTIONS *BootOptions
);

// Función para mostrar menú de arranque
UINT64 ShowBootMenu(
    EFI_SYSTEM_TABLE *SystemTable,
    REACTOS_BOOT_OPTIONS *BootOptions
);

// Función para procesar opciones de línea de comandos
UINT64 ProcessCommandLine(
    CHAR16 *CommandLine,
    REACTOS_BOOT_OPTIONS *BootOptions
);

// Función para guardar configuración de arranque
UINT64 SaveBootConfiguration(
    VOID *ImageHandle,
    EFI_SYSTEM_TABLE *SystemTable,
    REACTOS_BOOT_OPTIONS *BootOptions
);

// ============================================================================
// FUNCIONES DE DIAGNÓSTICO
// ============================================================================

// Función para mostrar información del sistema
UINT64 ShowSystemInfo(
    EFI_SYSTEM_TABLE *SystemTable
);

// Función para mostrar información de memoria
UINT64 ShowMemoryInfo(
    EFI_SYSTEM_TABLE *SystemTable
);

// Función para mostrar información de CPU
UINT64 ShowCpuInfo(
    EFI_SYSTEM_TABLE *SystemTable
);

// Función para mostrar información de dispositivos
UINT64 ShowDeviceInfo(
    EFI_SYSTEM_TABLE *SystemTable
);

// ============================================================================
// FUNCIONES DE RECUPERACIÓN
// ============================================================================

// Función para modo de recuperación
UINT64 EnterRecoveryMode(
    VOID *ImageHandle,
    EFI_SYSTEM_TABLE *SystemTable
);

// Función para reparar instalación
UINT64 RepairInstallation(
    VOID *ImageHandle,
    EFI_SYSTEM_TABLE *SystemTable
);

// Función para restaurar configuración
UINT64 RestoreConfiguration(
    VOID *ImageHandle,
    EFI_SYSTEM_TABLE *SystemTable
);

// ============================================================================
// CONSTANTES ESPECÍFICAS DE REACTOS
// ============================================================================

// Subsistemas de ReactOS
#define REACTOS_SUBSYSTEM_UNKNOWN        0
#define REACTOS_SUBSYSTEM_NATIVE         1
#define REACTOS_SUBSYSTEM_WINDOWS_GUI    2
#define REACTOS_SUBSYSTEM_WINDOWS_CUI    3
#define REACTOS_SUBSYSTEM_OS2_CUI        5
#define REACTOS_SUBSYSTEM_POSIX_CUI      7
#define REACTOS_SUBSYSTEM_NATIVE_WINDOWS 8
#define REACTOS_SUBSYSTEM_WINDOWS_CE_GUI 9

// Características de DLL de ReactOS
#define REACTOS_DLL_CHARACTERISTICS_DYNAMIC_BASE     0x0040
#define REACTOS_DLL_CHARACTERISTICS_FORCE_INTEGRITY  0x0080
#define REACTOS_DLL_CHARACTERISTICS_NX_COMPAT        0x0100
#define REACTOS_DLL_CHARACTERISTICS_NO_ISOLATION     0x0200
#define REACTOS_DLL_CHARACTERISTICS_NO_SEH           0x0400
#define REACTOS_DLL_CHARACTERISTICS_NO_BIND          0x0800
#define REACTOS_DLL_CHARACTERISTICS_WDM_DRIVER       0x2000
#define REACTOS_DLL_CHARACTERISTICS_TERMINAL_SERVER_AWARE 0x8000

// Flags del loader de ReactOS
#define REACTOS_LOADER_FLAGS_BREAK_ON_LOAD    0x00000001
#define REACTOS_LOADER_FLAGS_DEBUG_ON_LOAD    0x00000002
#define REACTOS_LOADER_FLAGS_SAFE_MODE        0x00000004
#define REACTOS_LOADER_FLAGS_NETWORK_BOOT    0x00000008
#define REACTOS_LOADER_FLAGS_MINIMAL_BOOT    0x00000010
#define REACTOS_LOADER_FLAGS_VERBOSE_BOOT    0x00000020

// ============================================================================
// MACROS ÚTILES
// ============================================================================

// Macro para verificar si el kernel es válido
#define IS_VALID_REACTOS_KERNEL(Header) \
    ((Header)->Signature == REACTOS_KERNEL_SIGNATURE)

// Macro para verificar versión del kernel
#define IS_COMPATIBLE_KERNEL_VERSION(Header) \
    ((Header)->Version >= REACTOS_KERNEL_VERSION)

// Macro para verificar si es modo seguro
#define IS_SAFE_MODE(BootOptions) \
    ((BootOptions)->SafeMode)

// Macro para verificar si es modo debug
#define IS_DEBUG_MODE(BootOptions) \
    ((BootOptions)->DebugMode)

// ============================================================================
// FINALIZACIÓN
// ============================================================================

#endif // LOADER_H

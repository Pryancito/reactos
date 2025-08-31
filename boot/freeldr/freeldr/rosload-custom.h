/**
 * ENCABEZADO MEJORADO PARA ROSLOAD PERSONALIZADO
 * Implementa funcionalidades principales del rosload oficial
 * Compatible con MinGW-w64 POSIX
 */

#ifndef ROSLOAD_CUSTOM_H
#define ROSLOAD_CUSTOM_H

// ============================================================================
// DEFINICIONES BÁSICAS
// ============================================================================

#ifndef BOOL
typedef int BOOL;
#endif

#ifndef TRUE
#define TRUE 1
#endif

#ifndef FALSE
#define FALSE 0
#endif

#ifndef VOID
#define VOID void
#endif

#ifndef ULONG
typedef unsigned long ULONG;
#endif

#ifndef PCHAR
typedef char* PCHAR;
#endif

#ifndef PWCHAR
typedef wchar_t* PWCHAR;
#endif

// ============================================================================
// ESTRUCTURAS PRINCIPALES
// ============================================================================

// Estructura de configuración del sistema
typedef struct _SYSTEM_CONFIG {
    BOOL DefaultOS;
    BOOL SafeMode;
    BOOL DebugMode;
    char BootPath[256];
    char ConfigPath[256];
    char SystemRoot[256];
    ULONG Timeout;
    ULONG BootAttempts;
} SYSTEM_CONFIG, *PSYSTEM_CONFIG;

// Estructura de opciones de arranque
typedef struct _BOOT_OPTIONS {
    char Description[128];
    char LoadIdentifier[128];
    BOOL Default;
    BOOL Enabled;
} BOOT_OPTIONS, *PBOOT_OPTIONS;

// Estructura de estado del sistema
typedef struct _SYSTEM_STATUS {
    BOOL LastBootSucceeded;
    BOOL LastBootShutdown;
    ULONG BootAttemptCount;
    ULONG LastBootCheckpoint;
} SYSTEM_STATUS, *PSYSTEM_STATUS;

// ============================================================================
// FUNCIONES PRINCIPALES
// ============================================================================

// Funciones de inicialización
BOOL InitializeSystem(void);
BOOL LoadSystemConfiguration(void);
BOOL InitializeUI(void);
BOOL InitializeDebugger(void);

// Funciones de configuración
BOOL LoadSettings(void);
BOOL ParseCommandLine(PCHAR CommandLine);
BOOL LoadBootDeviceDriver(void);

// Funciones de interfaz de usuario
void ShowBootMenu(void);
void ShowAdvancedOptions(void);
void ShowDebugOptions(void);
void DrawLogo(void);

// Funciones de arranque
BOOL BootReactOS(void);
BOOL BootSafeMode(void);
BOOL BootDebugMode(void);
BOOL LoadOperatingSystem(PBOOT_OPTIONS BootOption);

// Funciones de utilidad
void rosload_print(const char* message);
void rosload_print_w(const wchar_t* message);
void rosload_delay(int count);
void rosload_error(const char* error);
void rosload_warning(const char* warning);

// Función principal
VOID RunLoader(VOID);

// ============================================================================
// CONSTANTES Y CONFIGURACIONES
// ============================================================================

#define ROSLOAD_VERSION "2.0-POSIX"
#define ROSLOAD_BUILD_DATE __DATE__
#define ROSLOAD_BUILD_TIME __TIME__

#define MAX_BOOT_OPTIONS 10
#define DEFAULT_TIMEOUT 30
#define MAX_BOOT_ATTEMPTS 3

#define BOOT_PATH_DEFAULT "C:\\ReactOS"
#define CONFIG_PATH_DEFAULT "C:\\ReactOS\\freeldr.ini"
#define SYSTEM_ROOT_DEFAULT "C:\\ReactOS"

// ============================================================================
// CÓDIGOS DE ERROR
// ============================================================================

#define ROSLOAD_SUCCESS 0
#define ROSLOAD_ERROR_INIT -1
#define ROSLOAD_ERROR_CONFIG -2
#define ROSLOAD_ERROR_UI -3
#define ROSLOAD_ERROR_BOOT -4
#define ROSLOAD_ERROR_DRIVER -5

#endif // ROSLOAD_CUSTOM_H

/**
 * ENCABEZADO UEFI PARA BOOTLOADER NATIVO DE REACTOS
 * 
 * Este archivo contiene las definiciones UEFI necesarias para crear
 * un bootloader UEFI nativo que funcione en sistemas modernos
 * como ASUS 10ª generación.
 * 
 * Basado en UEFI Specification 2.8+
 * Compatible con Secure Boot y sistemas UEFI estrictos
 */

#ifndef UEFI_H
#define UEFI_H

// ============================================================================
// DEFINICIONES BÁSICAS UEFI
// ============================================================================

// Tipos de datos UEFI estándar
typedef unsigned char UINT8;
typedef unsigned short UINT16;
typedef unsigned int UINT32;
typedef unsigned long long UINT64;
typedef signed char INT8;
typedef signed short INT16;
typedef signed int INT32;
typedef signed long long INT64;
typedef char CHAR8;
typedef unsigned short CHAR16;
typedef void VOID;

// Punteros UEFI
typedef UINT8 *UINT8_PTR;
typedef UINT16 *UINT16_PTR;
typedef UINT32 *UINT32_PTR;
typedef UINT64 *UINT64_PTR;
typedef VOID *VOID_PTR;
typedef CHAR8 *CHAR8_PTR;
typedef CHAR16 *CHAR16_PTR;

// Constantes UEFI
#define EFI_SUCCESS                   0
#define EFI_LOAD_ERROR               (EFI_ERROR | 1)
#define EFI_INVALID_PARAMETER        (EFI_ERROR | 2)
#define EFI_UNSUPPORTED              (EFI_ERROR | 3)
#define EFI_BAD_BUFFER_SIZE          (EFI_ERROR | 4)
#define EFI_BUFFER_TOO_SMALL         (EFI_ERROR | 5)
#define EFI_NOT_READY                (EFI_ERROR | 6)
#define EFI_DEVICE_ERROR             (EFI_ERROR | 7)
#define EFI_WRITE_PROTECTED          (EFI_ERROR | 8)
#define EFI_OUT_OF_RESOURCES         (EFI_ERROR | 9)
#define EFI_VOLUME_CORRUPTED         (EFI_ERROR | 10)
#define EFI_VOLUME_FULL              (EFI_ERROR | 11)
#define EFI_NO_MEDIA                 (EFI_ERROR | 12)
#define EFI_MEDIA_CHANGED            (EFI_ERROR | 13)
#define EFI_NOT_FOUND                (EFI_ERROR | 14)
#define EFI_ACCESS_DENIED            (EFI_ERROR | 15)
#define EFI_NO_RESPONSE              (EFI_ERROR | 16)
#define EFI_NO_MAPPING               (EFI_ERROR | 17)
#define EFI_TIMEOUT                  (EFI_ERROR | 18)
#define EFI_NOT_STARTED              (EFI_ERROR | 19)
#define EFI_ALREADY_STARTED          (EFI_ERROR | 20)
#define EFI_ABORTED                  (EFI_ERROR | 21)
#define EFI_ICMP_ERROR               (EFI_ERROR | 22)
#define EFI_TFTP_ERROR               (EFI_ERROR | 23)
#define EFI_PROTOCOL_ERROR           (EFI_ERROR | 24)
#define EFI_INCOMPATIBLE_VERSION     (EFI_ERROR | 25)
#define EFI_SECURITY_VIOLATION       (EFI_ERROR | 26)
#define EFI_CRC_ERROR                (EFI_ERROR | 27)
#define EFI_END_OF_MEDIA             (EFI_ERROR | 28)
#define EFI_END_OF_FILE              (EFI_ERROR | 31)
#define EFI_ERROR                    (1ULL << 63)

// ============================================================================
// ESTRUCTURAS UEFI PRINCIPALES
// ============================================================================

// Estructura de tabla de memoria UEFI
typedef struct {
    UINT32 Type;
    UINT32 Pad;
    UINT64 PhysicalStart;
    UINT64 VirtualStart;
    UINT64 NumberOfPages;
    UINT64 Attribute;
} EFI_MEMORY_DESCRIPTOR;

// Estructura de información del sistema
typedef struct {
    UINT64 Signature;
    UINT32 Revision;
    UINT32 HeaderSize;
    UINT32 CRC32;
    UINT32 Reserved;
} EFI_TABLE_HEADER;

// Estructura de servicios del sistema
typedef struct {
    EFI_TABLE_HEADER Hdr;
    CHAR16 *FirmwareVendor;
    UINT32 FirmwareRevision;
    VOID *ConsoleInHandle;
    VOID *ConIn;
    VOID *ConsoleOutHandle;
    VOID *ConOut;
    VOID *StandardErrorHandle;
    VOID *StdErr;
    VOID *RuntimeServices;
    VOID *BootServices;
    UINT64 NumberOfTableEntries;
    VOID *ConfigurationTable;
} EFI_SYSTEM_TABLE;

// ============================================================================
// PROTOCOLOS UEFI PRINCIPALES
// ============================================================================

// Protocolo de entrada de texto simple
typedef struct {
    UINT64 QueryMode;
    UINT64 SetMode;
    UINT64 SetAttribute;
    UINT64 ClearScreen;
    UINT64 SetCursorPosition;
    UINT64 EnableCursor;
    UINT64 Mode;
} EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;

// Protocolo de entrada de texto simple
typedef struct {
    UINT64 Reset;
    UINT64 ReadKeyStroke;
    UINT64 WaitForKey;
} EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

// ============================================================================
// FUNCIONES UEFI PRINCIPALES
// ============================================================================

// Función de entrada principal UEFI
typedef UINT64 (*EFI_IMAGE_ENTRY_POINT)(
    VOID *ImageHandle,
    EFI_SYSTEM_TABLE *SystemTable
);

// Función de salida UEFI
typedef UINT64 (*EFI_IMAGE_UNLOAD)(
    VOID *ImageHandle
);

// ============================================================================
// CONSTANTES UEFI ESPECÍFICAS
// ============================================================================

// Tipos de memoria UEFI
#define EFI_RESERVED_MEMORY_TYPE          0
#define EFI_LOADER_CODE                   1
#define EFI_LOADER_DATA                   2
#define EFI_BOOT_SERVICES_CODE            3
#define EFI_BOOT_SERVICES_DATA            4
#define EFI_RUNTIME_SERVICES_CODE         5
#define EFI_RUNTIME_SERVICES_DATA         6
#define EFI_CONVENTIONAL_MEMORY           7
#define EFI_UNUSABLE_MEMORY              8
#define EFI_ACPI_RECLAIM_MEMORY          9
#define EFI_ACPI_MEMORY_NVS              10
#define EFI_MEMORY_MAPPED_IO             11
#define EFI_MEMORY_MAPPED_IO_PORT_SPACE  12
#define EFI_PAL_CODE                     13
#define EFI_PERSISTENT_MEMORY            14

// Atributos de memoria UEFI
#define EFI_MEMORY_UC                     0x0000000000000001
#define EFI_MEMORY_WC                     0x0000000000000002
#define EFI_MEMORY_WT                     0x0000000000000004
#define EFI_MEMORY_WB                     0x0000000000000008
#define EFI_MEMORY_UCE                    0x0000000000000010
#define EFI_MEMORY_WP                     0x0000000000001000
#define EFI_MEMORY_RP                     0x0000000000002000
#define EFI_MEMORY_XP                     0x0000000000004000
#define EFI_MEMORY_NV                     0x0000000000008000
#define EFI_MEMORY_MORE_RELIABLE         0x0000000000010000
#define EFI_MEMORY_RO                     0x0000000000020000
#define EFI_MEMORY_SP                    0x0000000000040000
#define EFI_MEMORY_CPU_CRYPTO            0x0000000000080000
#define EFI_MEMORY_RUNTIME               0x8000000000000000

// ============================================================================
// MACROS UEFI ÚTILES
// ============================================================================

// Macro para verificar si una función UEFI fue exitosa
#define EFI_ERROR_CHECK(Status) (((INT64)(Status)) < 0)

// Macro para obtener el código de error
#define EFI_ERROR_CODE(Status) ((Status) & ~(1ULL << 63))

// Macro para verificar si hay un error
#define EFI_ERROR_IS_ERROR(Status) (((INT64)(Status)) < 0)

// ============================================================================
// TIPOS DE DATOS ADICIONALES
// ============================================================================

// Handle UEFI
typedef VOID *EFI_HANDLE;

// Evento UEFI
typedef VOID *EFI_EVENT;

// Tarea UEFI
typedef VOID *EFI_TASK_PRIORITY_LEVEL;

// Protocolo UEFI
typedef VOID *EFI_PROTOCOL;

// ============================================================================
// FUNCIONES DE UTILIDAD UEFI
// ============================================================================

// Función para imprimir texto en consola UEFI
UINT64 UefiPrint(CHAR16 *String);

// Función para limpiar pantalla UEFI
UINT64 UefiClearScreen(VOID);

// Función para obtener entrada de teclado UEFI
UINT64 UefiReadKey(UINT16 *Key);

// Función para esperar entrada de teclado UEFI
UINT64 UefiWaitForKey(UINT16 *Key);

// ============================================================================
// DEFINICIONES ESPECÍFICAS PARA REACTOS
// ============================================================================

// Identificador único del bootloader ReactOS
#define REACTOS_UEFI_BOOTLOADER_SIGNATURE 0x524F5341  // "ROSA"

// Versión del bootloader UEFI
#define REACTOS_UEFI_BOOTLOADER_VERSION  0x0100      // 1.0

// Estructura de información del bootloader
typedef struct {
    UINT32 Signature;
    UINT16 Version;
    UINT16 Revision;
    UINT32 BuildDate;
    UINT32 BuildTime;
    CHAR8 BuildString[64];
} REACTOS_UEFI_BOOTLOADER_INFO;

// ============================================================================
// FINALIZACIÓN
// ============================================================================

#endif // UEFI_H

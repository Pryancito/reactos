/*
 * ReactOS Rust Kernel - C Header
 * 
 * Este header permite la interoperabilidad entre el kernel C de ReactOS
 * y el nuevo kernel Rust.
 */

#ifndef _REACTOS_RUST_KERNEL_H_
#define _REACTOS_RUST_KERNEL_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <ntdef.h>

/* Estructura de trap frame compatible con C */
typedef struct _CTRAP_FRAME {
    ULONG64 Rax;
    ULONG64 Rcx;
    ULONG64 Rdx;
    ULONG64 Rbx;
    ULONG64 Rsp;
    ULONG64 Rbp;
    ULONG64 Rsi;
    ULONG64 Rdi;
    ULONG64 R8;
    ULONG64 R9;
    ULONG64 R10;
    ULONG64 R11;
    ULONG64 R12;
    ULONG64 R13;
    ULONG64 R14;
    ULONG64 R15;
    USHORT Ds;
    USHORT Es;
    USHORT Fs;
    USHORT Gs;
    USHORT Ss;
    ULONG64 Rip;
    USHORT Cs;
    ULONG64 Rflags;
    ULONG64 ErrorCode;
    UCHAR PreviousMode;
    UCHAR Reserved[7];
} CTRAP_FRAME, *PCTRAP_FRAME;

/* Funciones del kernel Rust */

/**
 * Maneja una excepción de Invalid Opcode usando el kernel Rust
 * @param TrapFrame Puntero al trap frame
 * @return STATUS_SUCCESS si se manejó exitosamente, STATUS_UNSUCCESSFUL en caso contrario
 */
NTSTATUS
KiHandleInvalidOpcodeCompat(
    PCTRAP_FRAME TrapFrame
);

/**
 * Ejecuta un bug check usando el kernel Rust
 * @param Code Código del bug check
 * @param Param1 Primer parámetro
 * @param Param2 Segundo parámetro
 * @param Param3 Tercer parámetro
 * @param Param4 Cuarto parámetro
 */
VOID
KiBugCheck(
    ULONG Code,
    ULONG64 Param1,
    ULONG64 Param2,
    ULONG64 Param3,
    ULONG64 Param4
);

/**
 * Inicializa el kernel Rust
 * @return STATUS_SUCCESS si la inicialización fue exitosa
 */
NTSTATUS
KiInitializeRustKernel(
    VOID
);

/* Códigos de bug check compatibles */
#define RUST_BUGCHECK_SYSTEM_THREAD_EXCEPTION_NOT_HANDLED    0x7E
#define RUST_BUGCHECK_KERNEL_DATA_INPAGE_ERROR              0x7A
#define RUST_BUGCHECK_KERNEL_STACK_INPAGE_ERROR             0x77
#define RUST_BUGCHECK_KERNEL_SECURITY_CHECK_FAILURE         0x139
#define RUST_BUGCHECK_UNEXPECTED_KERNEL_MODE_TRAP           0x7F
#define RUST_BUGCHECK_DOUBLE_FAULT                          0x8
#define RUST_BUGCHECK_MEMORY_MANAGEMENT                     0x1A
#define RUST_BUGCHECK_PFN_LIST_CORRUPT                      0x4E
#define RUST_BUGCHECK_MACHINE_CHECK_EXCEPTION               0x9C
#define RUST_BUGCHECK_CLOCK_WATCHDOG_TIMEOUT                0x101
#define RUST_BUGCHECK_DRIVER_IRQL_NOT_LESS_OR_EQUAL         0xD1
#define RUST_BUGCHECK_SYSTEM_SERVICE_EXCEPTION              0x3B

#ifdef __cplusplus
}
#endif

#endif /* _REACTOS_RUST_KERNEL_H_ */

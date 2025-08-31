.macro StackAllocate size
    .if          \size ne 0
    sub         rsp, \size
    .seh_stackalloc \size
    .endif
.endm

.macro StackDeallocate size
    .if          \size ne 0
    add         rsp, \size
    .endif
.endm

.macro SaveReg reg64, offset
    mov         QWORD PTR [rsp+\offset], \reg64
    .seh_savereg    \reg64, \offset
.endm

.macro RestoreReg reg64, offset
    mov         \reg64, QWORD PTR [rsp+\offset]
.endm

.macro SaveXmm xmmreg, offset
    movdqa      XMMWORD PTR [\offset+rsp], \xmmreg
    .seh_savexmm \xmmreg, \offset
.endm

.macro RestoreXmm xmmreg, offset
    movdqa      \xmmreg, XMMWORD PTR [\offset+rsp]
.endm

.macro AVXSaveXmm xmmreg, offset
    vmovdqa     XMMWORD PTR [\offset+rsp], \xmmreg
    .seh_savexmm \xmmreg, \offset
.endm

.macro AVXRestoreXmm xmmreg, offset
    vmovdqa     \xmmreg, XMMWORD PTR [\offset+rsp]
.endm

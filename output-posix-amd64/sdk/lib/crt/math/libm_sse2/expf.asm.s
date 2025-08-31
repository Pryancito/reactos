//
// MIT License
// -----------
// 
// Copyright (c) 2002-2019 Advanced Micro Devices, Inc.
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this Software and associated documentaon files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//
// expf.asm
//
// An implementation of the expf libm function.
//
// Prototype:
//
//     float expf(float x);
//

//
//   Algorithm:
//       Similar to one presnted in exp.asm
//
// If FMA3 hardware is available, an FMA3 implementation of expf will be used.


.section .rdata
.align 16

__real_inf:                      .long 0x7f800000
                                .long 0
                                .quad 0

__real_ninf:                     .long 0x0ff800000
                                .long 0
                                .quad 0

__real_qnanbit:                  .long 0x00400000
                                .long 0
                                .quad 0

__real_zero:                     .long 0x00000000
                                .long 0
                                .quad 0

__real_p8192:                    .quad 0x40c0000000000000
                                .quad 0
__real_m9600:                    .quad 0x0c0c2c00000000000
                                .quad 0

__real_64_by_log2:               .quad 0x40571547652b82fe // 64/ln(2)
                                .quad 0
__real_log2_by_64:               .quad 0x3f862e42fefa39ef // log2_by_64
                                .quad 0

__real_1_by_6:                   .quad 0x3fc5555555555555 // 1/6
                                .quad 0
__real_1_by_2:                   .quad 0x3fe0000000000000 // 1/2
                                .quad 0

// these codes and the ones in the corresponding .c file have to match
__flag_x_nan:            .long 1
__flag_y_zero:           .long 2
__flag_y_inf:            .long 3

//EXTRN __two_to_jby64_table:QWORD
//EXTRN __use_fma3_lib:DWORD

#define fname expf
#define fname_special _expf_special

// define local variable storage offsets

// make room for fname_special to save things
#define dummy_space     0x020
#define stack_size     0x038

#include "fm.inc.h"

// external function
//EXTERN fname_special:PROC

.code64 .intel_syntax noprefix

.align 16
.global fname
.func fname
fname:
.seh_proc fname

    StackAllocate stack_size
    .seh_endprologue

    // Do this to avoid possible exceptions from a NaN argument.
    movd        edx, xmm0
    btr         edx,31
    cmp         edx, DWORD PTR __real_inf[rip]
    jge         Lexpf_x_is_inf_or_nan

    cmp          DWORD PTR __use_fma3_lib[rip], 0
    jne          Lexpf_fma3

Lexpf_sse2:

    cvtss2sd    xmm0, xmm0

    // x * (64/ln(2))
    movsd       xmm3, QWORD PTR __real_64_by_log2[rip]
    mulsd       xmm3, xmm0

    // x <= 128*ln(2), ( x * (64/ln(2)) ) <= 64*128
    // x > -150*ln(2), ( x * (64/ln(2)) ) > 64*(-150)
    comisd      xmm3, QWORD PTR __real_p8192[rip]
    jae         Lexpf_y_is_inf

    comisd      xmm3, QWORD PTR __real_m9600[rip]
    jb          Lexpf_y_is_zero

    // n = int( x * (64/ln(2)) )
    cvtpd2dq    xmm4, xmm3
    lea         r10, __two_to_jby64_table[rip]
    cvtdq2pd    xmm1, xmm4

    // r = x - n * ln(2)/64
    movsd       xmm2, QWORD PTR __real_log2_by_64[rip]
    mulsd       xmm2, xmm1
    movd        ecx, xmm4
    mov         rax, 0x3f
    and         eax, ecx
    subsd       xmm0, xmm2
    movsd       xmm1, xmm0

    // m = (n - j) / 64
    sub         ecx, eax
    sar         ecx, 6

    // q
    movsd       xmm3, QWORD PTR __real_1_by_6[rip]
    mulsd       xmm3, xmm0
    mulsd       xmm0, xmm0
    addsd       xmm3, QWORD PTR __real_1_by_2[rip]
    mulsd       xmm0, xmm3
    addsd       xmm0, xmm1

    add         rcx, 1023
    shl         rcx, 52

    // (f)*(1+q)
    movsd       xmm2, QWORD PTR [r10+rax*8]
    mulsd       xmm0, xmm2
    addsd       xmm0, xmm2

    movd        xmm1, rcx
    mulsd       xmm0, xmm1
    cvtsd2ss    xmm0, xmm0
 
Lexpf_final_check:
    StackDeallocate stack_size
    ret

.align 16
Lexpf_y_is_zero:

    movss       xmm1, DWORD PTR __real_zero[rip]
    movd        xmm0, edx
    mov         r8d, DWORD PTR __flag_y_zero[rip]

    call        fname_special
    jmp         Lexpf_finish          

.align 16
Lexpf_y_is_inf:

    movss       xmm1, DWORD PTR __real_inf[rip]
    movd        xmm0, edx
    mov         r8d, DWORD PTR __flag_y_inf[rip]

    call        fname_special
    jmp         Lexpf_finish      

.align 16
Lexpf_x_is_inf_or_nan:

    cmp         edx, DWORD PTR __real_inf[rip]
    je          Lexpf_finish

    cmp         edx, DWORD PTR __real_ninf[rip]
    je          Lexpf_process_zero

    or          edx, DWORD PTR __real_qnanbit[rip]
    movd        xmm1, edx
    mov         r8d, DWORD PTR __flag_x_nan[rip]
    call        fname_special
    jmp         Lexpf_finish    

.align 16
Lexpf_process_zero:
    movss       xmm0, DWORD PTR __real_zero[rip]
    jmp         Lexpf_final_check

.align 16
Lexpf_finish:
    StackDeallocate stack_size
    ret


.align 16
Lexpf_fma3:

    vcvtss2sd    xmm0, xmm0, xmm0

    // x * (64/ln(2))
    vmulsd      xmm3, xmm0, QWORD PTR __real_64_by_log2[rip]

    // x <= 128*ln(2), ( x * (64/ln(2)) ) <= 64*128
    // x > -150*ln(2), ( x * (64/ln(2)) ) > 64*(-150)
    vcomisd     xmm3, QWORD PTR __real_p8192[rip]
    jae         Lexpf_fma3_y_is_inf

    vucomisd    xmm3, QWORD PTR __real_m9600[rip]
    jb          Lexpf_fma3_y_is_zero

    // n = int( x * (64/ln(2)) )
    vcvtpd2dq   xmm4, xmm3
    lea         r10, __two_to_jby64_table[rip]
    vcvtdq2pd   xmm1, xmm4

    // r = x - n * ln(2)/64
    vfnmadd231sd xmm0, xmm1, QWORD PTR __real_log2_by_64[rip]
    vmovd        ecx, xmm4
    mov          rax, 0x3f
    and          eax, ecx
    vmovapd      xmm1, xmm0               // xmm1 <-- copy of r

    // m = (n - j) / 64
    sub          ecx, eax
    sar          ecx, 6

    // q
    vmovsd       xmm3, QWORD PTR __real_1_by_6[rip]
    vmulsd       xmm0, xmm0, xmm0         // xmm0 <-- r^2
    vfmadd213sd  xmm3, xmm1, QWORD PTR __real_1_by_2[rip] // xmm3 <-- r/6 + 1/2
    vfmadd213sd  xmm0, xmm3, xmm1         // xmm0 <-- q = r^2*(r/6 + 1/2) + r

    add         rcx, 1023
    shl         rcx, 52

    // (f)*(1+q)
    vmovsd       xmm2, QWORD PTR [r10+rax*8]
    vfmadd213sd  xmm0, xmm2, xmm2

    vmovq        xmm2,rcx
    vmulsd       xmm0, xmm0, xmm2
    vcvtsd2ss    xmm0, xmm0, xmm0
 
Lexpf_fma3_final_check:
    StackDeallocate stack_size
    ret

.align 16
Lexpf_fma3_y_is_zero:

    vmovss       xmm1, DWORD PTR __real_zero[rip]
    vmovd        xmm0, edx
    mov          r8d, DWORD PTR __flag_y_zero[rip]

    call         fname_special
    jmp          Lexpf_fma3_finish          

.align 16
Lexpf_fma3_y_is_inf:

    vmovss       xmm1, DWORD PTR __real_inf[rip]
    vmovd        xmm0, edx
    mov          r8d, DWORD PTR __flag_y_inf[rip]

    call         fname_special
    jmp          Lexpf_fma3_finish      

.align 16
Lexpf_fma3_process_zero:
    vmovss       xmm0, DWORD PTR __real_zero[rip]
    jmp          Lexpf_fma3_final_check

.align 16
Lexpf_fma3_finish:
    StackDeallocate stack_size
    ret

.seh_endproc
.endfunc

// END

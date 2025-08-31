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
// log.asm
//
// An implementation of the log libm function.
//
// Prototype:
//
//     double log(double x);
//

//
//   Algorithm:
//
//   Based on:
//   Ping-Tak Peter Tang
//   "Table-driven implementation of the logarithm function in IEEE
//   floating-point arithmetic"
//   ACM Transactions on Mathematical Software (TOMS)
//   Volume 16, Issue 4 (December 1990)
//
//
//   x very close to 1.0 is handled differently, for x everywhere else
//   a brief explanation is given below
//
//   x = (2^m)*A
//   x = (2^m)*(G+g) with (1 <= G < 2) and (g <= 2^(-9))
//   x = (2^m)*2*(G/2+g/2)
//   x = (2^m)*2*(F+f) with (0.5 <= F < 1) and (f <= 2^(-10))
//
//   Y = (2^(-1))*(2^(-m))*(2^m)*A
//   Now, range of Y is: 0.5 <= Y < 1
//
//   F = 0x100 + (first 8 mantissa bits) + (9th mantissa bit)
//   Now, range of F is: 256 <= F <= 512
//   F = F / 512
//   Now, range of F is: 0.5 <= F <= 1
//
//   f = -(Y-F), with (f <= 2^(-10))
//
//   log(x) = m*log(2) + log(2) + log(F-f)
//   log(x) = m*log(2) + log(2) + log(F) + log(1-(f/F))
//   log(x) = m*log(2) + log(2*F) + log(1-r)
//
//   r = (f/F), with (r <= 2^(-9))
//   r = f*(1/F) with (1/F) precomputed to avoid division
//
//   log(x) = m*log(2) + log(G) - poly
//
//   log(G) is precomputed
//   poly = (r + (r^2)/2 + (r^3)/3 + (r^4)/4) + (r^5)/5) + (r^6)/6))
//
//   log(2) and log(G) need to be maintained in extra precision
//   to avoid losing precision in the calculations
//

.section .rdata
.align 16

__real_ninf:         .quad 0x0fff0000000000000   // -inf
                    .quad 0x0000000000000000
__real_inf:          .quad 0x7ff0000000000000    // +inf
                    .quad 0x0000000000000000
__real_neg_qnan:     .quad 0x0fff8000000000000   // neg qNaN
                    .quad 0x0000000000000000
__real_qnanbit:      .quad 0x0008000000000000
                    .quad 0x0000000000000000
__real_min_norm:     .quad 0x0010000000000000
                    .quad 0x0000000000000000
__real_mant:         .quad 0x000FFFFFFFFFFFFF    // mantissa bits
                    .quad 0x0000000000000000
__mask_1023:         .quad 0x00000000000003ff
                    .quad 0x0000000000000000
__mask_001:          .quad 0x0000000000000001
                    .quad 0x0000000000000000

__mask_mant_all8:    .quad 0x000ff00000000000
                    .quad 0x0000000000000000
__mask_mant9:        .quad 0x0000080000000000
                    .quad 0x0000000000000000

__real_two:          .quad 0x4000000000000000 // 2
                    .quad 0x0000000000000000

__real_one:          .quad 0x3ff0000000000000 // 1
                    .quad 0x0000000000000000

__real_near_one_lt:  .quad 0x3fee000000000000 // .9375
                    .quad 0x0000000000000000

__real_near_one_gt:  .quad 0x3ff1000000000000 // 1.0625
                    .quad 0x0000000000000000

__real_half:         .quad 0x3fe0000000000000 // 1/2
                    .quad 0x0000000000000000

__mask_100:          .quad 0x0000000000000100
                    .quad 0x0000000000000000

__real_1_over_512:   .quad 0x3f60000000000000
                    .quad 0x0000000000000000

__real_1_over_2:     .quad 0x3fe0000000000000
                    .quad 0x0000000000000000
__real_1_over_3:     .quad 0x3fd5555555555555
                    .quad 0x0000000000000000
__real_1_over_4:     .quad 0x3fd0000000000000
                    .quad 0x0000000000000000
__real_1_over_5:     .quad 0x3fc999999999999a
                    .quad 0x0000000000000000
__real_1_over_6:     .quad 0x3fc5555555555555
                    .quad 0x0000000000000000

__mask_1023_f:       .quad 0x0c08ff80000000000
                    .quad 0x0000000000000000

__mask_2045:         .quad 0x00000000000007fd
                    .quad 0x0000000000000000

__real_threshold:    .quad 0x3fb0000000000000 // .0625
                    .quad 0x0000000000000000

__real_notsign:      .quad 0x7ffFFFFFFFFFFFFF // ^sign bit
                    .quad 0x0000000000000000

__real_ca1:          .quad 0x3fb55555555554e6 // 8.33333333333317923934e-02
                    .quad 0x0000000000000000
__real_ca2:          .quad 0x3f89999999bac6d4 // 1.25000000037717509602e-02
                    .quad 0x0000000000000000
__real_ca3:          .quad 0x3f62492307f1519f // 2.23213998791944806202e-03
                    .quad 0x0000000000000000
__real_ca4:          .quad 0x3f3c8034c85dfff0 // 4.34887777707614552256e-04
                    .quad 0x0000000000000000
__real_log2_lead:    .quad 0x03fe62e42e0000000 // 6.93147122859954833984e-01
                    .quad 0x00000000000000000
__real_log2_tail:    .quad 0x03e6efa39ef35793c // 5.76999904754328540596e-08
                    .quad 0x00000000000000000

// these codes and the ones in the corresponding .c file have to match
__flag_x_zero:          .long 1
__flag_x_neg:           .long 2
__flag_x_nan:           .long 3


//EXTRN __log_256_lead:QWORD
//EXTRN __log_256_tail:QWORD
//EXTRN __log_F_inv_qword:QWORD
//EXTRN __use_fma3_lib:DWORD


#define fname log
#define fname_special _log_special

// define local variable storage offsets

#define save_xmm6      0x20
#define dummy_space      0x40

#define stack_size      0x58

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
    SaveXmm      xmm6, save_xmm6
    .seh_endprologue

    cmp          DWORD PTR __use_fma3_lib[rip], 0
    jne          Llog_fma3

Llog_sse2:

    // compute exponent part
    movdqa      xmm3, xmm0
    movapd      xmm4, xmm0
    psrlq       xmm3, 52
    movd        rax, xmm0
    psubq       xmm3, XMMWORD PTR __mask_1023[rip]

    //  NaN or inf
    mov         rcx, rax
    btr         rcx, 63
    cmp         rcx, QWORD PTR __real_inf[rip]
    jae         __x_is_inf_or_nan

    movdqa      xmm2, xmm0
    cvtdq2pd    xmm6, xmm3 // xexp


    pand        xmm2, XMMWORD PTR __real_mant[rip]
    subsd       xmm4, QWORD PTR __real_one[rip]

    comisd      xmm6, QWORD PTR __mask_1023_f[rip]
    je          __denormal_adjust

__continue_common:    

    andpd       xmm4, XMMWORD PTR __real_notsign[rip]
    // compute index into the log tables
    mov         r9, rax
    and         rax, QWORD PTR __mask_mant_all8[rip]
    and         r9, QWORD PTR __mask_mant9[rip]
    shl         r9, 1
    add         rax, r9
    movd        xmm1, rax

    // near one codepath
    comisd      xmm4, QWORD PTR __real_threshold[rip]
    jb          __near_one

    // F, Y
    shr         rax, 44
    por         xmm2, XMMWORD PTR __real_half[rip]
    por         xmm1, XMMWORD PTR __real_half[rip]
    lea         r9, __log_F_inv_qword[rip]

    // check for negative numbers or zero
    xorpd       xmm5, xmm5
    comisd      xmm0, xmm5
    jbe         __x_is_zero_or_neg

    // f = F - Y, r = f * inv
    subsd       xmm1, xmm2                       // xmm1 <-- f = F - Y
    mulsd       xmm1, QWORD PTR [r9+rax*8]       // xmm1 <-- r = f * inv

    movapd      xmm2, xmm1                       // xmm2 <-- copy of r
    movapd      xmm0, xmm1                       // xmm0 <-- copy of r
    lea         r9, QWORD PTR __log_256_lead[rip]

    // poly
    movsd       xmm3, QWORD PTR __real_1_over_6[rip]
    movsd       xmm1, QWORD PTR __real_1_over_3[rip]
    mulsd       xmm3, xmm2                      // xmm3 <-- r/6
    mulsd       xmm1, xmm2                      // xmm1 <-- r/3
    mulsd       xmm0, xmm2                      // xmm0 <-- r*r
    movapd      xmm4, xmm0                      // xmm4 <-- copy of r*r
    addsd       xmm3, QWORD PTR __real_1_over_5[rip] // xmm3 <-- r/6 + 1/5
    addsd       xmm1, QWORD PTR __real_1_over_2[rip] // xmm1 <-- r/3 + 1/2
    mulsd       xmm4, xmm0                      // xmm4 <-- r^4
    mulsd       xmm3, xmm2                      // xmm3 <-- (r/6 + 1/5)*r
    mulsd       xmm1, xmm0                      // xmm1 <-- (r/3 + 1/2)*r^2
    addsd       xmm3, QWORD PTR __real_1_over_4[rip] // xmm3 <-- (r/6 + 1/5)*r + 1/4
    addsd       xmm1, xmm2                      // xmm1 <-- (r/3 + 1/2)*r^2 + r
    mulsd       xmm3, xmm4                      // xmm3 <-- ((r/6+1/5)*r+1/4)*r^4
    addsd       xmm1, xmm3                      // xmm1 <-- poly

    // m*log(2)_tail + log(G)_tail - poly
    movsd       xmm5, QWORD PTR __real_log2_tail[rip]
    mulsd       xmm5, xmm6                      // xmm5 <-- m*log2_tail
    subsd       xmm5, xmm1                      // xmm5 <-- m*log2_tail - poly

    movsd       xmm0, QWORD PTR [r9+rax*8]      // xmm0 <-- log(G)_lead
    lea         rdx, QWORD PTR __log_256_tail[rip]
    movsd       xmm2, QWORD PTR [rdx+rax*8]     // xmm2 <-- log(G)_tail
    addsd       xmm2, xmm5                      // xmm2 <-- (m*log2_tail - poly) + log(G)_tail

    movsd       xmm4, QWORD PTR __real_log2_lead[rip]
    mulsd       xmm4, xmm6                      // xmm4 <-- m*log2_lead
    addsd       xmm0, xmm4                      // xmm0 <-- m*log2_lead + log(G)_lead

    addsd       xmm0, xmm2        // xmm0 <-- m*log(2)_tail + log(G)_tail - poly

    RestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret

.align 16
__near_one:

    // r = x - 1.0
    movsd       xmm2, QWORD PTR __real_two[rip]
    subsd       xmm0, QWORD PTR __real_one[rip] // r

    addsd       xmm2, xmm0
    movsd       xmm1, xmm0
    divsd       xmm1, xmm2 // r/(2+r) = u/2

    movsd       xmm4, QWORD PTR __real_ca2[rip]
    movsd       xmm5, QWORD PTR __real_ca4[rip]

    movsd       xmm6, xmm0
    mulsd       xmm6, xmm1 // correction

    addsd       xmm1, xmm1 // u
    movsd       xmm2, xmm1

    mulsd       xmm2, xmm1 // u^2

    mulsd       xmm4, xmm2
    mulsd       xmm5, xmm2

    addsd       xmm4, __real_ca1[rip]
    addsd       xmm5, __real_ca3[rip]

    mulsd       xmm2, xmm1 // u^3
    mulsd       xmm4, xmm2

    mulsd       xmm2, xmm2
    mulsd       xmm2, xmm1 // u^7
    mulsd       xmm5, xmm2

    addsd       xmm4, xmm5
    subsd       xmm4, xmm6
    addsd       xmm0, xmm4

    RestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret

.align 16
__denormal_adjust:
    por         xmm2, XMMWORD PTR __real_one[rip]
    subsd       xmm2, QWORD PTR __real_one[rip]
    movsd       xmm5, xmm2
    pand        xmm2, XMMWORD PTR __real_mant[rip]
    movd        rax, xmm2
    psrlq       xmm5, 52
    psubd       xmm5, XMMWORD PTR __mask_2045[rip]
    cvtdq2pd    xmm6, xmm5
    jmp         __continue_common

.align 16
__x_is_zero_or_neg:
    jne         __x_is_neg

    movsd       xmm1, QWORD PTR __real_ninf[rip]
    mov         r8d, DWORD PTR __flag_x_zero[rip]
    call        fname_special
    jmp         __finish

.align 16
__x_is_neg:

    movsd       xmm1, QWORD PTR __real_neg_qnan[rip]
    mov         r8d, DWORD PTR __flag_x_neg[rip]
    call        fname_special
    jmp         __finish

.align 16
__x_is_inf_or_nan:

    cmp         rax, QWORD PTR __real_inf[rip]
    je          __finish

    cmp         rax, QWORD PTR __real_ninf[rip]
    je          __x_is_neg

    or          rax, QWORD PTR __real_qnanbit[rip]
    movd        xmm1, rax
    mov         r8d, DWORD PTR __flag_x_nan[rip]
    call        fname_special
    jmp         __finish

.align 16
__finish:
    RestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret

.align 16
Llog_fma3:
    // compute exponent part
    xor          rax,rax
    vpsrlq       xmm3,xmm0,52
    vmovq        rax,xmm0
    vpsubq       xmm3,xmm3,XMMWORD PTR __mask_1023[rip]
    vcvtdq2pd    xmm6,xmm3 // xexp

    //  NaN or inf
    vpand        xmm5,xmm0,XMMWORD PTR __real_inf[rip]
    vcomisd      xmm5,QWORD PTR __real_inf[rip]
    je           Llog_fma3_x_is_inf_or_nan

    // check for negative numbers or zero
    vpxor        xmm5,xmm5,xmm5
    vcomisd      xmm0,xmm5
    jbe          Llog_fma3_x_is_zero_or_neg

    vpand        xmm2,xmm0,XMMWORD PTR __real_mant[rip]
    vsubsd       xmm4,xmm0,QWORD PTR __real_one[rip]

    vcomisd      xmm6,QWORD PTR __mask_1023_f[rip]
    je           Llog_fma3_denormal_adjust

Llog_fma3_continue_common:
    // compute index into the log tables
    vpand        xmm1,xmm0,XMMWORD PTR __mask_mant_all8[rip]
    vpand        xmm3,xmm0,XMMWORD PTR __mask_mant9[rip]
    vpsllq       xmm3,xmm3,1
    vpaddq       xmm1,xmm3,xmm1
    vmovq        rax,xmm1

    // near one codepath
    vpand        xmm4,xmm4,XMMWORD PTR __real_notsign[rip]
    vcomisd      xmm4,QWORD PTR __real_threshold[rip]
    jb           Llog_fma3_near_one

    // F,Y
    shr          rax,44
    vpor         xmm2,xmm2,XMMWORD PTR __real_half[rip]
    vpor         xmm1,xmm1,XMMWORD PTR __real_half[rip]
    lea          r9,QWORD PTR __log_F_inv_qword[rip]

    // f = F - Y,r = f * inv
    vsubsd       xmm1,xmm1,xmm2
    vmulsd       xmm1,xmm1,QWORD PTR[r9 + rax * 8]

    lea          r9,QWORD PTR __log_256_lead[rip]

    // poly
    vmulsd       xmm0,xmm1,xmm1           // r*r
    vmovsd       xmm3,QWORD PTR __real_1_over_6[rip]
    vmovsd       xmm5,QWORD PTR __real_1_over_3[rip]
    vfmadd213sd  xmm3,xmm1,QWORD PTR __real_1_over_5[rip] // r*1/6 + 1/5
    vfmadd213sd  xmm5,xmm1,QWORD PTR __real_1_over_2[rip] // 1/2+r*1/3
    vmovsd       xmm4,xmm0,xmm0
    vfmadd213sd  xmm3,xmm1,QWORD PTR __real_1_over_4[rip] // 1/4+(1/5*r+r*r*1/6)

    vmulsd       xmm4,xmm0,xmm0           // r*r*r*r
    vfmadd231sd  xmm1,xmm5,xmm0           // r*r*(1/2+r*1/3) + r
    vfmadd231sd  xmm1,xmm3,xmm4

    // m*log(2) + log(G) - poly
    vmovsd       xmm5,QWORD PTR __real_log2_tail[rip]
    vfmsub213sd  xmm5,xmm6,xmm1

    vmovsd       xmm0,QWORD PTR[r9 + rax * 8]
    lea          rdx,QWORD PTR __log_256_tail[rip]
    vmovsd       xmm1,QWORD PTR[rdx + rax * 8]
    vaddsd       xmm1,xmm1,xmm5

    vfmadd231sd  xmm0,xmm6,QWORD PTR __real_log2_lead[rip]

    vaddsd       xmm0,xmm0,xmm1
    AVXRestoreXmm   xmm6, save_xmm6
    StackDeallocate stack_size
    ret


.align  16
Llog_fma3_near_one:

    // r = x - 1.0
    vmovsd       xmm3,QWORD PTR __real_two[rip]
    vsubsd       xmm0,xmm0,QWORD PTR __real_one[rip] // r

    vaddsd       xmm3,xmm3,xmm0
    vdivsd       xmm1,xmm0,xmm3           // r/(2+r) = u/2

    vmovsd       xmm4,QWORD PTR __real_ca2[rip]
    vmovsd       xmm5,QWORD PTR __real_ca4[rip]

    vmulsd       xmm3,xmm0,xmm1           // correction
    vaddsd       xmm1,xmm1,xmm1           // u

    vmulsd       xmm2,xmm1,xmm1           // u^2
    vfmadd213sd  xmm4,xmm2,QWORD PTR __real_ca1[rip]
    vfmadd213sd  xmm5,xmm2,QWORD PTR __real_ca3[rip]

    vmulsd       xmm2,xmm2,xmm1           // u^3
    vmulsd       xmm4,xmm4,xmm2

    vmulsd       xmm2,xmm2,xmm2
    vmulsd       xmm2,xmm2,xmm1           // u^7

    vfmadd231sd  xmm4,xmm5,xmm2
    vsubsd       xmm4,xmm4,xmm3
    vaddsd       xmm0,xmm0,xmm4

    AVXRestoreXmm   xmm6, save_xmm6
    StackDeallocate stack_size
    ret


Llog_fma3_denormal_adjust:
    vpor         xmm2,xmm2,XMMWORD PTR __real_one[rip]
    vsubsd       xmm2,xmm2,QWORD PTR __real_one[rip]
    vpsrlq       xmm5,xmm2,52
    vpand        xmm2,xmm2,XMMWORD PTR __real_mant[rip]
    vmovapd      xmm0,xmm2
    vpsubd       xmm5,xmm5,XMMWORD PTR __mask_2045[rip]
    vcvtdq2pd    xmm6,xmm5
    jmp          Llog_fma3_continue_common

.align  16
Llog_fma3_x_is_zero_or_neg:
    jne          Llog_fma3_x_is_neg
    vmovsd       xmm1,QWORD PTR __real_ninf[rip]
    mov          r8d,DWORD PTR __flag_x_zero[rip]
    call         fname_special

    AVXRestoreXmm   xmm6, save_xmm6
    StackDeallocate stack_size
    ret

.align  16
Llog_fma3_x_is_neg:

    vmovsd       xmm1,QWORD PTR __real_neg_qnan[rip]
    mov          r8d,DWORD PTR __flag_x_neg[rip]
    call         fname_special

    AVXRestoreXmm   xmm6, save_xmm6
    StackDeallocate stack_size
    ret

.align  16
Llog_fma3_x_is_inf_or_nan:

    cmp          rax,QWORD PTR __real_inf[rip]
    je           Llog_fma3_finish

    cmp          rax,QWORD PTR __real_ninf[rip]
    je           Llog_fma3_x_is_neg

    or           rax,QWORD PTR __real_qnanbit[rip]
    vmovq        xmm1,rax
    mov          r8d,DWORD PTR __flag_x_nan[rip]
    call         fname_special

.align  16
Llog_fma3_finish:
    AVXRestoreXmm   xmm6, save_xmm6
    StackDeallocate stack_size
    ret
.seh_endproc
.endfunc

// END

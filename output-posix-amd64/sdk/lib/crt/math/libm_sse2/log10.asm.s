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
// log10.asm
//
// An implementation of the log10 libm function.
//
// Prototype:
//
//     double log10(double x);
//

//
//   Algorithm:
//       Similar to one presnted in log.asm
//

.section .rdata

.align 16

__real_ninf:     .quad 0x0fff0000000000000   // -inf
                .quad 0x0000000000000000
__real_inf:      .quad 0x7ff0000000000000    // +inf
                .quad 0x0000000000000000
__real_neg_qnan: .quad 0x0fff8000000000000   // neg qNaN
                .quad 0x0000000000000000
__real_qnanbit:  .quad 0x0008000000000000
                .quad 0x0000000000000000
__int_1023:      .quad 0x00000000000003ff
                .quad 0x0000000000000000
__mask_001:      .quad 0x0000000000000001
                .quad 0x0000000000000000

__mask_mant:     .quad 0x000FFFFFFFFFFFFF    // mask for mantissa bits
                .quad 0x0000000000000000

__mask_mant_top8: .quad 0x000ff00000000000   // mask for top 8 mantissa bits
                .quad 0x0000000000000000

__mask_mant9:    .quad 0x0000080000000000    // mask for 9th mantissa bit
                .quad 0x0000000000000000

__real_log10_e:      .quad 0x3fdbcb7b1526e50e
                    .quad 0x0000000000000000

__real_log10_e_lead: .quad 0x3fdbcb7800000000 // log10e_lead 4.34293746948242187500e-01
                    .quad 0x0000000000000000
__real_log10_e_tail: .quad 0x3ea8a93728719535 // log10e_tail 7.3495500964015109100644e-7
                    .quad 0x0000000000000000

__real_log10_2_lead: .quad 0x3fd3441350000000
                    .quad 0x0000000000000000
__real_log10_2_tail: .quad 0x3e03ef3fde623e25
                    .quad 0x0000000000000000

__real_two:          .quad 0x4000000000000000 // 2
                    .quad 0x0000000000000000

__real_one:          .quad 0x3ff0000000000000 // 1
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

__real_neg_1023:     .quad 0x0c08ff80000000000
                    .quad 0x0000000000000000

__mask_2045:         .quad 0x00000000000007fd
                    .quad 0x0000000000000000

__real_threshold:    .quad 0x3fb0000000000000 // .0625
                    .quad 0x0000000000000000

__real_near_one_lt:  .quad 0x3fee000000000000 // .9375
                    .quad 0x0000000000000000

__real_near_one_gt:  .quad 0x3ff1000000000000 // 1.0625
                    .quad 0x0000000000000000

__real_min_norm:     .quad 0x0010000000000000
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

__mask_lower:        .quad 0x0ffffffff00000000
                    .quad 0x0000000000000000

// these codes and the ones in the corresponding .c file have to match
__flag_x_zero:           .long 1
__flag_x_neg:            .long 2
__flag_x_nan:            .long 3


//EXTRN __log10_256_lead:QWORD
//EXTRN __log10_256_tail:QWORD
//EXTRN __log_F_inv_qword:QWORD
//EXTRN __use_fma3_lib:DWORD


// local variable storage offsets
#define save_xmm6      0x20
#define dummy_space      0x30
#define stack_size      0x058

#include "fm.inc.h"

#define fname log10
#define fname_special _log10_special

//EXTERN fname_special:PROC

.code64 .intel_syntax noprefix
.align 16
.global fname
.func fname
fname:
.seh_proc fname

    StackAllocate stack_size
    SaveXmm xmm6, save_xmm6
    .seh_endprologue

    cmp          DWORD PTR __use_fma3_lib[rip], 0
    jne          Llog10_fma3

Llog10_sse2:

    // compute exponent part
    movapd      xmm3, xmm0
    movapd      xmm4, xmm0
    psrlq       xmm3, 52
    movd        rax, xmm0
    psubq       xmm3, XMMWORD PTR __int_1023[rip] // xmm3 <-- unbiased exponent

    //  NaN or inf
    movapd      xmm5, xmm0
    andpd       xmm5, XMMWORD PTR __real_inf[rip]
    comisd      xmm5, QWORD PTR __real_inf[rip]
    je          Llog10_sse2_x_is_inf_or_nan

    movapd      xmm2, xmm0
    cvtdq2pd    xmm6, xmm3                   // xmm6 <-- unbiased exp as double


    pand        xmm2, XMMWORD PTR __mask_mant[rip]
    subsd       xmm4, QWORD PTR __real_one[rip]

    comisd      xmm6, QWORD PTR __real_neg_1023[rip]
    je          Llog10_sse2_denormal_adjust

Llog10_sse2_continue_common:    

    andpd       xmm4, XMMWORD PTR __real_notsign[rip]
    // compute index into the log tables
    mov         r9, rax
    and         rax, QWORD PTR __mask_mant_top8[rip]
    and         r9, QWORD PTR __mask_mant9[rip]
    shl         r9, 1
    add         rax, r9
    movd        xmm1, rax

    // near one codepath
    comisd      xmm4, QWORD PTR __real_threshold[rip]
    jb          Llog10_sse2_near_one

    // F, Y
    shr         rax, 44
    por         xmm2, XMMWORD PTR __real_half[rip]
    por         xmm1, XMMWORD PTR __real_half[rip]
    lea         r9, QWORD PTR __log_F_inv_qword[rip]

    // check for negative numbers or zero
    xorpd       xmm5, xmm5
    comisd      xmm0, xmm5
    jbe         Llog10_sse2_x_is_zero_or_neg

    // f = F - Y, r = f * inv
    subsd       xmm1, xmm2
    mulsd       xmm1, QWORD PTR [r9+rax*8]

    movapd      xmm2, xmm1
    movapd      xmm0, xmm1
    lea         r9, QWORD PTR __log10_256_lead[rip]

    // poly
    movsd       xmm3, QWORD PTR __real_1_over_6[rip]
    movsd       xmm1, QWORD PTR __real_1_over_3[rip]
    mulsd       xmm3, xmm2                         
    mulsd       xmm1, xmm2                         
    mulsd       xmm0, xmm2                         
    movapd      xmm4, xmm0
    addsd       xmm3, QWORD PTR __real_1_over_5[rip]
    addsd       xmm1, QWORD PTR __real_1_over_2[rip]
    mulsd       xmm4, xmm0                         
    mulsd       xmm3, xmm2                         
    mulsd       xmm1, xmm0                         
    addsd       xmm3, QWORD PTR __real_1_over_4[rip]
    addsd       xmm1, xmm2                         
    mulsd       xmm3, xmm4                         
    addsd       xmm1, xmm3                         

    movsd       xmm5, QWORD PTR __real_log10_2_tail[rip]
    mulsd       xmm1, QWORD PTR __real_log10_e[rip]

    // m*log(10) + log10(G) - poly
    mulsd       xmm5, xmm6
    subsd       xmm5, xmm1

    movsd       xmm0, QWORD PTR [r9+rax*8]
    lea         rdx, QWORD PTR __log10_256_tail[rip]
    movsd       xmm2, QWORD PTR [rdx+rax*8]

    movsd       xmm4, QWORD PTR __real_log10_2_lead[rip]
    mulsd       xmm4, xmm6
    addsd       xmm0, xmm4
    addsd       xmm2, xmm5

    addsd       xmm0, xmm2

    RestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret

.align 16
Llog10_sse2_near_one:

    // r = x - 1.0
    movsd       xmm2, QWORD PTR __real_two[rip]
    subsd       xmm0, QWORD PTR __real_one[rip] // r

    addsd       xmm2, xmm0
    movapd      xmm1, xmm0
    divsd       xmm1, xmm2 // r/(2+r) = u/2

    movsd       xmm4, QWORD PTR __real_ca2[rip]
    movsd       xmm5, QWORD PTR __real_ca4[rip]

    movapd       xmm6, xmm0
    mulsd       xmm6, xmm1 // correction

    addsd       xmm1, xmm1 // u
    movapd      xmm2, xmm1

    mulsd       xmm2, xmm1 // u^2

    mulsd       xmm4, xmm2
    mulsd       xmm5, xmm2

    addsd       xmm4, QWORD PTR __real_ca1[rip]
    addsd       xmm5, QWORD PTR __real_ca3[rip]

    mulsd       xmm2, xmm1 // u^3
    mulsd       xmm4, xmm2

    mulsd       xmm2, xmm2
    mulsd       xmm2, xmm1 // u^7
    mulsd       xmm5, xmm2

    movsd       xmm2, QWORD PTR __real_log10_e_tail[rip]
    addsd       xmm4, xmm5
    subsd       xmm4, xmm6
    movsd       xmm6, QWORD PTR __real_log10_e_lead[rip]

    movapd      xmm3, xmm0
    pand        xmm3, XMMWORD PTR __mask_lower[rip]
    subsd       xmm0, xmm3
    addsd       xmm4, xmm0

    movapd      xmm0, xmm3
    movapd      xmm1, xmm4

    mulsd       xmm4, xmm2
    mulsd       xmm0, xmm2
    mulsd       xmm1, xmm6
    mulsd       xmm3, xmm6

    addsd       xmm0, xmm4
    addsd       xmm0, xmm1
    addsd       xmm0, xmm3

    RestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret

Llog10_sse2_denormal_adjust:
    por         xmm2, XMMWORD PTR __real_one[rip]
    subsd       xmm2, QWORD PTR __real_one[rip]
    movsd       xmm5, xmm2
    pand        xmm2, XMMWORD PTR __mask_mant[rip]
    movd        rax, xmm2
    psrlq       xmm5, 52
    psubd       xmm5, XMMWORD PTR __mask_2045[rip]
    cvtdq2pd    xmm6, xmm5
    jmp         Llog10_sse2_continue_common

.align 16
Llog10_sse2_x_is_zero_or_neg:
    jne         Llog10_sse2_x_is_neg

    movsd       xmm1, QWORD PTR __real_ninf[rip]
    mov         r8d, DWORD PTR __flag_x_zero[rip]
    call        fname_special
    jmp         Llog10_sse2_finish

.align 16
Llog10_sse2_x_is_neg:

    movsd       xmm1, QWORD PTR __real_neg_qnan[rip]
    mov         r8d, DWORD PTR __flag_x_neg[rip]
    call        fname_special
    jmp         Llog10_sse2_finish

.align 16
Llog10_sse2_x_is_inf_or_nan:

    cmp         rax, QWORD PTR __real_inf[rip]
    je          Llog10_sse2_finish

    cmp         rax, QWORD PTR __real_ninf[rip]
    je          Llog10_sse2_x_is_neg

    or          rax, QWORD PTR __real_qnanbit[rip]
    movd        xmm1, rax
    mov         r8d, DWORD PTR __flag_x_nan[rip]
    call        fname_special
    jmp         Llog10_sse2_finish    

.align 16
Llog10_sse2_finish:
    RestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret

.align 16
Llog10_fma3:
    // compute exponent part
    xor          rax,rax
    vpsrlq       xmm3,xmm0,52
    vmovq        rax,xmm0
    vpsubq       xmm3,xmm3,XMMWORD PTR __int_1023[rip]
    vcvtdq2pd    xmm6,xmm3                // xmm6 <-- (double)xexp

    //  NaN or Inf?
    vpand        xmm5,xmm0,__real_inf[rip]
    vcomisd      xmm5,QWORD PTR __real_inf[rip]
    je           Llog10_fma3_x_is_inf_or_nan

    // negative number or zero?
    vpxor        xmm5,xmm5,xmm5
    vcomisd      xmm0,xmm5
    jbe          Llog10_fma3_x_is_zero_or_neg

    vpand        xmm2,xmm0,__mask_mant[rip]
    vsubsd       xmm4,xmm0,QWORD PTR __real_one[rip]

    // Subnormal?
    vcomisd      xmm6,QWORD PTR __real_neg_1023[rip]
    je           Llog10_fma3_denormal_adjust

Llog10_fma3_continue_common:
    // compute index into the log tables
    vpand        xmm1,xmm0,XMMWORD PTR __mask_mant_top8[rip]
    vpand        xmm3,xmm0,XMMWORD PTR __mask_mant9[rip]
    vpsllq       xmm3,xmm3,1
    vpaddq       xmm1,xmm3,xmm1
    vmovq        rax,xmm1

    // near one codepath
    vpand        xmm4,xmm4,XMMWORD PTR __real_notsign[rip]
    vcomisd      xmm4,QWORD PTR __real_threshold[rip]
    jb           Llog10_fma3_near_one

    // F,Y
    shr          rax,44
    vpor         xmm2,xmm2,XMMWORD PTR __real_half[rip]
    vpor         xmm1,xmm1,XMMWORD PTR __real_half[rip]
    lea          r9,DWORD PTR __log_F_inv_qword[rip]

    // f = F - Y,r = f * inv
    vsubsd       xmm1,xmm1,xmm2
    vmulsd       xmm1,xmm1,QWORD PTR [r9 + rax * 8]

    lea          r9,DWORD PTR __log10_256_lead[rip]

    // poly
    vmulsd       xmm0,xmm1,xmm1 // r*r
    vmovsd       xmm3,QWORD PTR __real_1_over_6[rip]
    vmovsd       xmm5,QWORD PTR __real_1_over_3[rip]
    vfmadd213sd  xmm3,xmm1,QWORD PTR __real_1_over_5[rip]  // r*1/6 + 1/5
    vfmadd213sd  xmm5,xmm1,QWORD PTR __real_half[rip]      // 1/2+r*1/3
    movsd        xmm4,xmm0                             // r*r
    vfmadd213sd  xmm3 ,xmm1,QWORD PTR __real_1_over_4[rip] // 1/4+(1/5*r+r*r*1/6)

    vmulsd       xmm4,xmm0,xmm0                        // r*r*r*r
    vfmadd231sd  xmm1,xmm5,xmm0                        // r*r*(1/2+r*1/3) + r
    vfmadd231sd  xmm1,xmm3,xmm4

    vmulsd       xmm1,xmm1,QWORD PTR __real_log10_e[rip]
    // m*log(2) + log(G) - poly*log10_e
    vmovsd       xmm5,QWORD PTR __real_log10_2_tail[rip]
    vfmsub213sd  xmm5,xmm6,xmm1

    movsd        xmm0,QWORD PTR [r9 + rax * 8]
    lea          rdx,DWORD PTR __log10_256_tail[rip]
    movsd        xmm2,QWORD PTR [rdx + rax * 8]
    vaddsd       xmm2,xmm2,xmm5

    vfmadd231sd  xmm0,xmm6,QWORD PTR __real_log10_2_lead[rip]

    vaddsd       xmm0,xmm0,xmm2
    AVXRestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret


.align  16
Llog10_fma3_near_one:
    // r = x - 1.0
    vmovsd       xmm2,QWORD PTR __real_two[rip]
    vsubsd       xmm0,xmm0,QWORD PTR __real_one[rip] // r

    vaddsd       xmm2,xmm2,xmm0
    vdivsd       xmm1,xmm0,xmm2 // r/(2+r) = u/2

    vmovsd       xmm4,QWORD PTR __real_ca2[rip]
    vmovsd       xmm5,QWORD PTR __real_ca4[rip]

    vmulsd       xmm6,xmm0,xmm1 // correction
    vaddsd       xmm1,xmm1,xmm1 // u

    vmulsd       xmm2,xmm1,xmm1 // u^2
    vfmadd213sd  xmm4,xmm2,QWORD PTR __real_ca1[rip]
    vfmadd213sd  xmm5,xmm2,QWORD PTR __real_ca3[rip]

    vmulsd       xmm2,xmm2,xmm1 // u^3
    vmulsd       xmm4,xmm4,xmm2

    vmulsd       xmm2,xmm2,xmm2
    vmulsd       xmm2,xmm2,xmm1 // u^7

    vmulsd       xmm5,xmm5,xmm2
    vaddsd       xmm4,xmm4,xmm5
    vsubsd       xmm4,xmm4,xmm6
    vpand        xmm3,xmm0,XMMWORD PTR __mask_lower[rip]
    vsubsd       xmm0,xmm0,xmm3
    vaddsd       xmm4,xmm4,xmm0

    vmulsd       xmm1,xmm4,QWORD PTR __real_log10_e_lead[rip]
    vmulsd       xmm4,xmm4,QWORD PTR __real_log10_e_tail[rip]
    vmulsd       xmm0,xmm3,QWORD PTR __real_log10_e_tail[rip]
    vmulsd       xmm3,xmm3,QWORD PTR __real_log10_e_lead[rip]

    vaddsd       xmm0,xmm0,xmm4
    vaddsd       xmm0,xmm0,xmm1
    vaddsd       xmm0,xmm0,xmm3

    AVXRestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret


Llog10_fma3_denormal_adjust:
    vpor         xmm2,xmm2,XMMWORD PTR __real_one[rip]
    vsubsd       xmm2,xmm2,QWORD PTR __real_one[rip]
    vpsrlq       xmm5,xmm2,52
    vpand        xmm2,xmm2,XMMWORD PTR __mask_mant[rip]
    vmovapd      xmm0,xmm2
    vpsubd       xmm5,xmm5,XMMWORD PTR __mask_2045[rip]
    vcvtdq2pd    xmm6,xmm5
    jmp          Llog10_fma3_continue_common

.align  16
Llog10_fma3_x_is_zero_or_neg:
    jne          Llog10_fma3_x_is_neg
    vmovsd       xmm1,QWORD PTR __real_ninf[rip]
    mov          r8d,DWORD PTR __flag_x_zero[rip]
    call         fname_special

    AVXRestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret


.align  16
Llog10_fma3_x_is_neg:

    vmovsd       xmm1,QWORD PTR __real_neg_qnan[rip]
    mov          r8d,DWORD PTR __flag_x_neg[rip]
    call         fname_special

    AVXRestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret


.align  16
Llog10_fma3_x_is_inf_or_nan:

    cmp          rax,QWORD PTR __real_inf[rip]
    je           Llog10_fma3_finish

    cmp          rax,QWORD PTR __real_ninf[rip]
    je           Llog10_fma3_x_is_neg

    or           rax,QWORD PTR __real_qnanbit[rip]
    movd         xmm1,rax
    mov          r8d,DWORD PTR __flag_x_nan[rip]
    call         fname_special
    jmp          Llog10_fma3_finish

.align  16
Llog10_fma3_finish:

    AVXRestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret
.seh_endproc
.endfunc

// END

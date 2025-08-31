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
// exp.asm
//
// An implementation of the exp libm function.
//
// Prototype:
//
//     double exp(double x);
//

//
//   Algorithm:
//
//   e^x = 2^(x/ln(2)) = 2^(x*(64/ln(2))/64)
//
//   x*(64/ln(2)) = n + f, |f| <= 0.5, n is integer
//   n = 64*m + j,   0 <= j < 64
//
//   e^x = 2^((64*m + j + f)/64)
//       = (2^m) * (2^(j/64)) * 2^(f/64)
//       = (2^m) * (2^(j/64)) * e^(f*(ln(2)/64))
//
//   f = x*(64/ln(2)) - n
//   r = f*(ln(2)/64) = x - n*(ln(2)/64)
//
//   e^x = (2^m) * (2^(j/64)) * e^r
//
//   (2^(j/64)) is precomputed
//
//   e^r = 1 + r + (r^2)/2! + (r^3)/3! + (r^4)/4! + (r^5)/5! + (r^5)/5!
//   e^r = 1 + q
//
//   q = r + (r^2)/2! + (r^3)/3! + (r^4)/4! + (r^5)/5! + (r^5)/5!
//

.section .rdata
.align 16
// these codes and the ones in the corresponding .c file have to match
__flag_x_nan:            .long 1
__flag_y_zero:           .long 2
__flag_y_inf:            .long 3

.align 16

L__real_1_by_720:              .quad 0x03f56c16c16c16c17
                              .quad 0x03f56c16c16c16c17   // 1/720
L__real_1_by_120:              .quad 0x03f81111111111111
                              .quad 0x03f81111111111111   // 1/120
L__real_1_by_6:                .quad 0x03fc5555555555555
                              .quad 0x03fc5555555555555   // 1/6
L__real_1_by_2:                .quad 0x03fe0000000000000
                              .quad 0x03fe0000000000000   // 1/2
L__real_1_by_24:               .quad 0x03fa5555555555555
                              .quad 0x03fa5555555555555   // 1/24

.align 16
L__log2_by_64_mtail_mhead:     .quad 0x0bf862e42fefa0000, 0x0bd1cf79abc9e3b39
L__ln_of_smallest_normal:      .quad 0x0C086232BDD7ABCD2
L__zero:                       .quad 0x00000000000000000
L__max_exp_arg:                .quad 0x040862e42fefa39ef   //  709.78271289338397
L__denormal_tiny_threshold:    .quad 0x0c0874046dfefd9d0   // -744.03460681327306
L__min_exp_arg:                .quad 0x0c0874910d52d3051   // -745.13321910194111
L__real_64_by_log2:            .quad 0x040571547652b82fe   // 64/ln(2)
L__positive_infinity:          .quad 0x07ff0000000000000
L__negative_infinity:          .quad 0x0fff0000000000000
L__real_qnanbit:               .quad 0x0008000000000000    // qnan set bit
L__real_x_near0_threshold:     .quad 0x3c00000000000000
L__log2_by_64_mhead:           .quad 0x0bf862e42fefa0000
L__log2_by_64_mtail:           .quad 0x0bd1cf79abc9e3b39
L__real_smallest_denormal:     .quad 0x00000000000000001
L__real_one:                   .quad 0x03ff0000000000000
L__2_to_neg_26:                .quad 0x03E50000000000000   // 2^-26
L__min_normal:                 .quad 0x00010000000000000   // smallest normal


//EXTRN __two_to_jby64_table:QWORD
//EXTRN __two_to_jby64_head_table:QWORD
//EXTRN __two_to_jby64_tail_table:QWORD
//EXTRN __use_fma3_lib:DWORD

// make room for fname_special to save things
#define dummy_space     0x020
#define stack_size     0x038

#include "fm.inc.h"

#define fname exp
#define fname_special _exp_special

//Define name and any external functions being called
//EXTERN       fname_special      : PROC

.code64 .intel_syntax noprefix
.global fname
.func fname
fname:
.seh_proc fname

    StackAllocate stack_size
    .seh_endprologue

    // We need to avoid unwanted exceptions from a NaN argument.
    // It could be argued that a signaling NaN should raise an exception,
    // but the existing library doesn't.  At any rate, the comparison operations
    // don't seem to like quiet NaN either, so...
    movd         rdx, xmm0
    btr          rdx, 63
    cmp          rdx, L__positive_infinity[rip]
    jge          Lexp_x_is_nan_or_inf

    cmp          DWORD PTR __use_fma3_lib[rip], 0
    jne          Lexp_fma3

    movapd       xmm2, xmm0
    movapd       xmm3, xmm0

    // Some hardware has problems with too many branches in a single
    // 16- or 32-byte window, so let's peel off the common case into
    // a single branch.
    cmplesd      xmm2, L__max_exp_arg[rip]  // xmm2 <-- 0xFFFFFFFF is x is not too big positive
    cmpnltsd     xmm3, L__denormal_tiny_threshold[rip] // xmm3 <-- 0xFFFFFFFF if x is not too big negative
    andps        xmm2, xmm3     // xmm2 <-- 0xFFFFFFFF if x is in range, 0 otherwise
    ucomisd      xmm2, xmm2   // note that FFF... is NaN, so this comparison should set PF for in-range x
    jp           Lexp_y_is_finite

    ucomisd      xmm0,   L__max_exp_arg[rip]
    ja           Lexp_y_is_inf
    // Since we peeled off the cases with normal result,
    // there is only one possibility remaining:
    jmp          Lexp_y_is_denormal_or_zero

.align 16
Lexp_y_is_finite:
    // x * (64/ln(2))
    movapd       xmm1,   xmm0
    btr          rdx, 63                  // rdx <-- |x|
    cmp          rdx, L__2_to_neg_26[rip]
    jbe          Lexp_return_1_plus_x
    mulsd        xmm1,   L__real_64_by_log2[rip]

    // n = int( x * (64/ln(2)) )
    cvttpd2dq    xmm2, xmm1               // xmm2 = (int)n
    cvtdq2pd     xmm1, xmm2               // xmm1 = (double)n
    movd         ecx, xmm2
    movapd       xmm2, xmm1
    
    // r1 = x - n * ln(2)/64 head
    mulsd        xmm1, L__log2_by_64_mhead[rip]

    // j = n & 0x3f
    mov          rax, 0x03f
    and          eax, ecx                 // eax = j
    // m = (n - j) / 64
    sar          ecx,    6                // ecx = m


    // r2 = - n * ln(2)/64 tail
    mulsd        xmm2, L__log2_by_64_mtail[rip]
    addsd        xmm0, xmm1               // xmm0 = r1

    // r1+r2
    addsd        xmm2, xmm0               // xmm2 = r

    // q = r + r^2*1/2 + r^3*1/6 + r^4 *1/24 + r^5*1/120 + r^6*1/720
    // q = r + r*r*(1/2 + r*(1/6+ r*(1/24 + r*(1/120 + r*(1/720)))))
    movapd       xmm3, L__real_1_by_720[rip]   // xmm3 = 1/720
    mulsd        xmm3, xmm2               // xmm3 = r*1/720
    movapd       xmm0, L__real_1_by_6[rip]     // xmm0 = 1/6
    movapd       xmm1, xmm2               // xmm1 = r
    mulsd        xmm0, xmm2               // xmm0 = r*1/6
    addsd        xmm3, L__real_1_by_120[rip]   // xmm3 = 1/120 + (r*1/720)
    mulsd        xmm1, xmm2               // xmm1 = r*r
    addsd        xmm0, L__real_1_by_2[rip]     // xmm0 = 1/2 + (r*1/6)
    movapd       xmm4, xmm1               // xmm4 = r*r
    mulsd        xmm4, xmm1               // xmm4 = (r*r) * (r*r)
    mulsd        xmm3, xmm2               // xmm3 = r * (1/120 + (r*1/720))
    mulsd        xmm0, xmm1               // xmm0 = (r*r)*(1/2 + (r*1/6))
    addsd        xmm3, L__real_1_by_24[rip]    // xmm3 = 1/24 + (r * (1/120 + (r*1/720)))
    addsd        xmm0, xmm2               // xmm0 = r + ((r*r)*(1/2 + (r*1/6)))
    mulsd        xmm3, xmm4               // xmm3 = ((r*r) * (r*r)) * (1/24 + (r * (1/120 + (r*1/720))))
    addsd        xmm0, xmm3               // xmm0 = r + ((r*r)*(1/2 + (r*1/6))) + ((r*r) * (r*r)) * (1/24 + (r * (1/120 + (r*1/720))))

    //(f)*(q) + f2 + f1
    cmp          ecx, 0x0fffffc02          // -1022
    lea          rdx,  __two_to_jby64_table[rip]
    lea          r11,  __two_to_jby64_tail_table[rip]
    lea          r10,  __two_to_jby64_head_table[rip]
    mulsd        xmm0, QWORD PTR [rdx+rax * 8 ]
    addsd        xmm0, QWORD PTR [r11+rax * 8 ]
    addsd        xmm0, QWORD PTR [r10+rax * 8 ]

    jle          Lexp_process_denormal
Lexp_process_normal:
    shl          rcx,    52
    movd         xmm2,   rcx
    paddq        xmm0,   xmm2
    StackDeallocate stack_size
    ret

.align 16
Lexp_process_denormal:
    jl           Lexp_process_true_denormal
    ucomisd      xmm0,   L__real_one[rip]
    jae          Lexp_process_normal
Lexp_process_true_denormal:
    // here ( e^r < 1 and m = -1022 ) or m <= -1023
    add          ecx, 1074
    mov          rax, 1
    shl          rax, cl
    movd         xmm2, rax
    mulsd        xmm0, xmm2
    jmp          Lexp_finish

Lexp_y_is_one:
    movsd        xmm0, L__real_one[rip]
    jmp          Lexp_finish

.align 16
Lexp_x_is_nan_or_inf:
    movd         rax, xmm0
    cmp          rax, L__positive_infinity[rip]
    je           Lexp_finish
    cmp          rax, L__negative_infinity[rip]
    je           Lexp_return_zero_without_exception
    or           rax, L__real_qnanbit[rip]
    movd         xmm1, rax
    mov          r8d, __flag_x_nan[rip]
    call         fname_special
    jmp          Lexp_finish

.align 16
Lexp_y_is_inf:
    mov          rax, 0x07ff0000000000000
    movd         xmm1, rax
    mov          r8d, __flag_y_inf[rip]
    call         fname_special
    jmp          Lexp_finish

.align 16
Lexp_y_is_denormal_or_zero:
    ucomisd      xmm0, L__min_exp_arg[rip]
    jbe          Lexp_y_is_zero
    movapd       xmm0, L__real_smallest_denormal[rip]
    jmp          Lexp_finish

.align 16
Lexp_y_is_zero:
    pxor         xmm1, xmm1
    mov          r8d, __flag_y_zero[rip]
    call         fname_special
    jmp          Lexp_finish

.align 16
Lexp_return_1_plus_x:
    cmp          rdx, L__min_normal[rip]
    jbe          Lexp_return_1_plus_eps
    addsd        xmm0, L__real_one[rip]
    StackDeallocate stack_size
    ret          0

// Some hardware really does not like subnormals.  Try to avoid them.
.align 16
Lexp_return_1_plus_eps:
    movsd        xmm0, L__real_one[rip]
    addsd        xmm0, L__min_normal[rip]         // make sure inexact is set
    StackDeallocate stack_size
    ret          0

.align 16
Lexp_return_zero_without_exception:
    pxor         xmm0,xmm0
    StackDeallocate stack_size
    ret          0


.align 16
Lexp_finish:
    StackDeallocate stack_size
    ret          0

.align 16
Lexp_fma3:
    // Some hardware has problems with too many branches in a single
    // 16- or 32-byte window, so let's peel off the common case into
    // a single branch.
    vcmplesd     xmm2, xmm0, L__max_exp_arg[rip]  // xmm2 <-- 0xFFFFFFFF is x is not too big positive
    vcmpnltsd    xmm3, xmm0, L__denormal_tiny_threshold[rip] // xmm3 <-- 0xFFFFFFFF if x is not too big negative
    vandps       xmm2, xmm3, xmm2  // xmm2 <-- 0xFFFFFFFF if x is in range, 0 otherwise
    vucomisd     xmm2, xmm2   // note that FFF... is NaN, so this comparison should set PF for in-range x
    jp           Lexp_fma3_y_is_finite

    vucomisd     xmm0,L__max_exp_arg[rip]
    ja           Lexp_fma3_y_is_inf
    // Since we peeled off the cases with normal result,
    // there is only one possibility remaining:
    jmp          Lexp_fma3_y_is_zero

//   vpsllq       xmm1, xmm0, 1
//   vpsrlq       xmm1, xmm1, 1
//   vucomisd     xmm1, L__real_x_near0_threshold   ; 2^-63
//   jb           Lexp_fma3_y_is_one

.align 16
Lexp_fma3_y_is_finite:
    vmovq        rdx, xmm0
    btr          rdx, 63                  // rdx <-- |x|
    cmp          rdx, L__2_to_neg_26[rip]
    jbe          Lexp_fma3_return_1_plus_x

    // x * (64/ln(2))
    vmulsd       xmm1,xmm0,L__real_64_by_log2[rip]

    // n = int( x * (64/ln(2)) )
    vcvttpd2dq   xmm2,xmm1 //xmm2 = (int)n
    vcvtdq2pd    xmm1,xmm2 //xmm1 = (double)n ;can use round
    vmovd        ecx,xmm2

    // r1 = x - n * ln(2)/64 head
    // r2 = - n * ln(2)/64 tail
    // r = r1+r2
    vmovlhps     xmm1,xmm1,xmm1 //xmm1 = (double (double)n,)n
    vmovq        xmm0,xmm0 //xmm0 = 0,x ;zero out the upper part
    vfmadd132pd  xmm1,xmm0,L__log2_by_64_mtail_mhead[rip]
    vhaddpd      xmm2,xmm1,xmm1 //xmm2 = r,r

    //j = n & 03fh
    mov          rax,0x03f
    and          eax,ecx //eax = j
    // m = (n - j) / 64
    sar          ecx,6 //ecx = m

    // q = r + r^2*1/2 + r^3*1/6 + r^4 *1/24 + r^5*1/120 + r^6*1/720
    // q = r + r*r*(1/2 + r*(1/6+ r*(1/24 + r*(1/120 + r*(1/720)))))
    vmovapd      xmm3,L__real_1_by_720[rip]
    vfmadd213sd  xmm3,xmm2,L__real_1_by_120[rip]
    vfmadd213sd  xmm3,xmm2,L__real_1_by_24[rip]
    vfmadd213sd  xmm3,xmm2,L__real_1_by_6[rip]
    vfmadd213sd  xmm3,xmm2,L__real_1_by_2[rip]
    vmulsd       xmm0,xmm2,xmm2
    vfmadd213sd  xmm0,xmm3,xmm2

    // (f)*(q) + f2 + f1
    cmp          ecx,0x0fffffc02 // -1022
    lea          rdx,__two_to_jby64_table[rip]
    lea          r11,__two_to_jby64_tail_table[rip]
    lea          r10,__two_to_jby64_head_table[rip]
    vmulsd       xmm2,xmm0,QWORD PTR[rdx + rax * 8]
    vaddsd       xmm1,xmm2,QWORD PTR[r11 + rax * 8]
    vaddsd       xmm0,xmm1,QWORD PTR[r10 + rax * 8]

    jle          Lexp_fma3_process_denormal
Lexp_fma3_process_normal:
    shl          rcx,52
    vmovq        xmm2,rcx
    vpaddq       xmm0,xmm0,xmm2
    StackDeallocate stack_size
    ret

.align 16
Lexp_fma3_process_denormal:
    jl           Lexp_fma3_process_true_denormal
    vucomisd     xmm0,L__real_one[rip]
    jae          Lexp_fma3_process_normal
Lexp_fma3_process_true_denormal:
    // here ( e^r < 1 and m = -1022 ) or m <= -1023
    add          ecx,1074
    mov          rax,1
    shl          rax,cl
    vmovq        xmm2,rax
    vmulsd       xmm0,xmm0,xmm2
    jmp          Lexp_fma3_finish

Lexp_fma3_y_is_one:
    vmovsd       xmm0, L__real_one[rip]
    jmp          Lexp_fma3_finish


.align 16
Lexp_fma3_y_is_inf:
    mov          rax,0x07ff0000000000000
    vmovq        xmm1,rax
    mov          r8d,__flag_y_inf[rip]
    call         fname_special
    jmp          Lexp_fma3_finish

.align 16
Lexp_fma3_return_1_plus_x:
    cmp          rdx, L__min_normal[rip]
    jbe          Lexp_fma3_return_1_plus_eps
    vaddsd       xmm0, xmm0, L__real_one[rip]
    StackDeallocate stack_size
    ret          0

// Some hardware really does not like subnormals.  Try to avoid them.
.align 16
Lexp_fma3_return_1_plus_eps:
    vmovsd       xmm0, L__real_one[rip]
    vaddsd       xmm0, xmm0, L__min_normal[rip]         // make sure inexact is set
    StackDeallocate stack_size
    ret          0

.align 16
Lexp_fma3_y_is_zero:
    vpxor        xmm1,xmm1,xmm1
    mov          r8d,__flag_y_zero[rip]
    call         fname_special
    jmp          Lexp_fma3_finish

.align 16
Lexp_fma3_return_zero_without_exception:
    vpxor        xmm0,xmm0,xmm0

.align 16
Lexp_fma3_finish:
    StackDeallocate stack_size
    ret

.seh_endproc
.endfunc
// END

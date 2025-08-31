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
// An implementation of the cosf function.
//
// Prototype:
//
//     float cosf(float x);
//
//   Computes cosf(x).
//   Based on the NAG C implementation.
//   It will provide proper C99 return values,
//   but may not raise floating point status bits properly.
//   Original Author: Harsha Jagasia

.section .rdata
.align 16
L_real_one:                 .quad 0x03ff0000000000000      // 1.0
                           .quad 0                          // for alignment
L_one_half:                 .quad 0x03fe0000000000000      // 0.5
                           .quad 0
L_2bypi:                    .quad 0x03fe45f306dc9c883      // 2./pi
                           .quad 0
L_one_sixth:                .quad 0x03fc5555555555555      // 0.166666666666
                           .quad 0
L_piby2:                    .quad 0x03fe921fb54442d18
                           .quad 0
L_piby2_1:                  .quad 0x03ff921fb54400000     // piby2_1
                           .quad 0
L_piby2_1tail:              .quad 0x03dd0b4611a626331     // piby2_1tail
                           .quad 0
L_piby2_2:                  .quad 0x03dd0b4611a600000     // piby2_2
                           .quad 0
L_piby2_2tail:              .quad 0x03ba3198a2e037073     // piby2_2tail
                           .quad 0
L_large_x_sse2:             .quad 0x0411E848000000000     // 5e5
                           .quad 0
L_large_x_fma3:             .quad 0x041E921FB60000000     // 3.37325952e9
                           .quad 0
L_sign_mask:                .quad 0x07FFFFFFFFFFFFFFF
                           .quad 0x07FFFFFFFFFFFFFFF
L__int_three:               .quad 0x00000000000000003
                           .quad 0x00000000000000003
L__min_norm_double:         .quad 0x00010000000000000
                           .quad 0x00010000000000000
L_two_to_neg_7:             .quad 0x03f80000000000000
                           .quad 0
L_two_to_neg_13:            .quad 0x03f20000000000000
                           .quad 0
L_inf_mask_32:              .long 0x07F800000
                           .quad 0

#define fname cosf
#define fname_special _cosf_special

//Define name and any external functions being called
//EXTERN           __remainder_piby2d2f_forAsm : PROC    // NEAR
//EXTERN           __remainder_piby2_fma3_bdl  : PROC   // NEAR
//EXTERN           __remainder_piby2_fma3      : PROC   // NEAR
//EXTERN           fname_special      : PROC
//EXTERN           _set_statfp        : PROC


//EXTRN __Lcosfarray:QWORD
//EXTRN __Lsinfarray:QWORD
//EXTRN __use_fma3_lib:DWORD

// define local variable storage offsets
#define p_temp         0x020          // temporary for get/put bits operation
#define p_temp1         0x030          // temporary for get/put bits operation
#define dummy_space         0x040
#define stack_size         0x068

#include "fm.inc.h"

.code64 .intel_syntax noprefix

.align 16
.global fname
.func fname
fname:
.seh_proc fname

    StackAllocate stack_size
    .seh_endprologue
    cmp          DWORD PTR __use_fma3_lib[rip], 0
    jne          Lcosf_fma3

Lcosf_sse2:

    xorpd        xmm2, xmm2               // zeroed out for later use

//;  if NaN or inf
    movd         edx, xmm0
    mov          eax, 0x07f800000
    mov          r10d, eax
    and          r10d, edx
    cmp          r10d, eax
    jz           Lcosf_sse2_naninf

    cvtss2sd     xmm0, xmm0
    movd         rdx, xmm0

//  ax = (ux & ~SIGNBIT_DP64);
    mov          r10, rdx
    btr          r10, 63                  // r10 <-- |x|
    mov          r8d, 1                   // for determining region later on

    movapd       xmm1, xmm0               // xmm1 <-- copy of x


//;  if (ax <= 3fe921fb54442d18h) /* abs(x) <= pi/4 */
    mov          rax, 0x03fe921fb54442d18
    cmp          r10, rax
    jg           Lcosf_sse2_absx_gt_piby4

//          *c = cos_piby4(x, 0.0);
    movapd       xmm2, xmm0
    mulsd        xmm2, xmm2        //x^2
    xor          eax, eax
    mov          rdx, r10
    movsd        xmm5, QWORD PTR L_one_half[rip]
    jmp          Lcosf_sse2_calc_sincosf_piby4        // done


.align 16
Lcosf_sse2_absx_gt_piby4:
// reduce  the argument to be in a range from -pi/4 to +pi/4
// by subtracting multiples of pi/2
//  xneg = (ax != ux);
    movd         xmm0, r10                // xmm0 <-- |x|
    cmp          r10, QWORD PTR L_large_x_sse2[rip]
    jae          Lcosf_sse2_reduce_precise

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// xmm0=abs(x), xmm1=x
///* How many pi/2 is x a multiple of? */

    movapd       xmm2, xmm0
    movsd        xmm3, QWORD PTR L_2bypi[rip]
    movapd       xmm4, xmm0
    movsd        xmm5, QWORD PTR L_one_half[rip]
    mulsd        xmm2, xmm3

//   movsd        xmm5, QWORD PTR L_one_half
//   movapd       xmm2, xmm0
//   mulsd        xmm2, QWORD PTR L_2bypi
//   movapd       xmm4, xmm0

    mov     r9, r10
    shr     r9, 52                        // r9 <-- biased exponent of x

//        npi2  = (int)(x * twobypi + 0.5);
    addsd   xmm2, xmm5                          // npi2

    movsd        xmm3, QWORD PTR L_piby2_1[rip]      // piby2_1
    cvttpd2dq    xmm0, xmm2                     // xmm0 <-- npi2
    movsd        xmm1, QWORD PTR L_piby2_1tail[rip]  // piby2_1tail
    cvtdq2pd     xmm2, xmm0                     // xmm2 <-- (double)npi2

//    Subtract the multiple from x to get an extra-precision remainder
//      rhead  = x - npi2 * piby2_1;

    mulsd        xmm3, xmm2                     // use piby2_1
    subsd        xmm4, xmm3                     // rhead

//      rtail  = npi2 * piby2_1tail;
    mulsd        xmm1, xmm2                     // rtail
    movd         eax, xmm0

// GET_BITS_DP64(rhead-rtail, uy);
// originally only rhead
    movapd       xmm0, xmm4
    subsd        xmm0, xmm1

    movsd        xmm3, QWORD PTR L_piby2_2[rip]      // piby2_2
    movd         rcx, xmm0                      // rcx <-- rhead-rtail
    movsd        xmm5, QWORD PTR L_piby2_2tail[rip]  // piby2_2tail

//      region = npi2 & 3;
//    and        eax, 3
//      expdiff = xexp - ((uy & EXPBITS_DP64) >> EXPSHIFTBITS_DP64);
    shl          rcx, 1                         // strip any sign bit
    shr          rcx, 53                        // >> EXPSHIFTBITS_DP64 +1
    sub          r9, rcx                        // expdiff

//;      if (expdiff > 15)
    cmp          r9, 15
    jle          Lcosf_sse2_expdiff_le_15

// The remainder is pretty small compared with x, which
// implies that x is a near multiple of pi/2
// (x matches the multiple to at least 15 bits)
//          t  = rhead;
    movapd       xmm1, xmm4

//          rtail  = npi2 * piby2_2;
    mulsd        xmm3, xmm2

//          rhead  = t - rtail;
    mulsd        xmm5, xmm2                     // npi2 * piby2_2tail
    subsd        xmm4, xmm3                     // rhead

//          rtail  = npi2 * piby2_2tail - ((t - rhead) - rtail);
    subsd        xmm1, xmm4                     // t - rhead
    subsd        xmm1, xmm3                     // -rtail
    subsd        xmm5, xmm1                     // rtail

//      r = rhead - rtail;
    movapd       xmm0, xmm4

//HARSHA
//xmm1=rtail
    movapd       xmm1, xmm5
    subsd        xmm0, xmm5

//    xmm0=r, xmm4=rhead, xmm1=rtail

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
Lcosf_sse2_expdiff_le_15:
    cmp          rcx, 0x03f2                     // is r < 2^-13 ?
    jge          Lcosf_sse2_calc_sincosf_piby4  // use taylor series if not
    cmp          rcx, 0x03de                     // is r < 2^-33 ?
    jle          Lcosf_sse2_r_very_small        // then cosf(r) ~ 1 or r

    movapd       xmm2, xmm0
    mulsd        xmm2, xmm0                     // xmm2 <-- x^2

//;      if region is 1 or 3 do a sinf calc.
    and          r8d, eax
    jz           Lcosf_sse2_r_small_calc_sin

Lcosf_sse2_r_small_calc_cos:
// region 1 or 3
// use simply polynomial
//              *s = x - x*x*x*0.166666666666666666;
    movsd        xmm3, QWORD PTR L_one_sixth[rip]
    mulsd        xmm3, xmm0                     // * x
    mulsd        xmm3, xmm2                     // * x^2
    subsd        xmm0, xmm3                     // xs
    jmp          Lcosf_sse2_adjust_region

.align 16
Lcosf_sse2_r_small_calc_sin:
// region 0 or 2
//              cos = 1.0 - x*x*0.5;
    movsd        xmm0, QWORD PTR L_real_one[rip]     // 1.0
    mulsd        xmm2, QWORD PTR L_one_half[rip]     // 0.5 *x^2
    subsd        xmm0, xmm2
    jmp          Lcosf_sse2_adjust_region

.align 16
Lcosf_sse2_r_very_small:
// then sin(r) = r
// if region is 1 or 3    do a sin calc.
    and          r8d, eax
    jnz          Lcosf_sse2_adjust_region

    movsd        xmm0, QWORD PTR L_real_one[rip]  // cosf(r) is a 1
    // By this point, calculations should already have set inexact
    jmp          Lcosf_sse2_adjust_region

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
.align 16
Lcosf_sse2_reduce_precise:
//      Reduce abs(x) into range [-pi/4, pi/4]
//      remainder_piby2d2f(ax, &r, &region);
    mov          QWORD PTR p_temp[rsp], rdx     // save ux for use later
    mov          QWORD PTR p_temp1[rsp], r10    // save ax for use later

    call         __remainder_piby2d2f_forAsm
    mov          rdx, QWORD PTR p_temp[rsp]     // restore ux for use later
    mov          r10, QWORD PTR p_temp1[rsp]    // restore ax for use later
    mov          r8d, 1                         // for determining region later

    // Reduced argument is in xmm0.  No second word; after all, we started in
    // single precision.  Region is in rax.
    movapd       xmm1, xmm0
    movsd        xmm5, QWORD PTR L_one_half[rip]

    jmp          Lcosf_sse2_calc_sincosf_piby4


// done with reducing the argument.  Now perform the sin/cos calculations.
.align 16
Lcosf_sse2_calc_sincosf_piby4:
    movapd       xmm2, xmm0
    mulsd        xmm2, xmm0                     // x^2

//;       if region is 0 or 2, do a cosf calc
    and          r8d, eax
    jz           Lcosf_sse2_do_cosf_calc
//   region is 1 or 3: do a sinf calc.
Lcosf_sse2_do_sinf_calc:
    movsd   xmm1, QWORD PTR __Lsinfarray[rip]+0x18          // s4
    mulsd   xmm1, xmm2                                // s4x2
    movsd   xmm4, xmm2                                // move for x4    
    mulsd   xmm4, xmm2                                // x4
    movsd   xmm5, QWORD PTR __Lsinfarray[rip]+0x8           // s2
    mulsd   xmm5, xmm2                                // s2x2
    movsd   xmm3, xmm0                                // move for x3
    mulsd   xmm3, xmm2                                // x3        
    addsd   xmm1, QWORD PTR __Lsinfarray[rip]+0x10          // s3+s4x2
    mulsd   xmm1, xmm4                                // s3x4+s4x6     
    addsd   xmm5, QWORD PTR __Lsinfarray[rip]              // s1+s2x2
    addsd   xmm1, xmm5                                // s1+s2x2+s3x4+s4x6
    mulsd   xmm1, xmm3                                // x3(s1+s2x2+s3x4+s4x6)
    addsd   xmm0, xmm1                                // x + x3(s1+s2x2+s3x4+s4x6)
    jmp     Lcosf_sse2_adjust_region

.align 16
Lcosf_sse2_do_cosf_calc:
// region 0 or 2     - do a cos calculation
//  zc = 1-0.5*x2+ c1*x4 +c2*x6 +c3*x8;
//     zc = 1-0.5*x2+ c1*x4 +c2*x6 +c3*x8 + c4*x10 for a higher precision
    movsd   xmm1, QWORD PTR __Lcosfarray[rip]+0x20          // c4
    movsd   xmm4, xmm2                                // move for x4
    mulsd   xmm1, xmm2                                // c4x2
    movsd   xmm3, QWORD PTR __Lcosfarray[rip]+0x10          // c2
    mulsd   xmm4, xmm2                                // x4
    movsd   xmm0, QWORD PTR __Lcosfarray[rip]              // c0
    mulsd   xmm3, xmm2                                // c2x2
    mulsd   xmm0, xmm2                                // c0x2 (=-0.5x2)
    addsd   xmm1, QWORD PTR __Lcosfarray[rip]+0x18          // c3+c4x2
    mulsd   xmm1, xmm4                                // c3x4 + c4x6
    addsd   xmm3, QWORD PTR __Lcosfarray[rip]+0x8           // c1+c2x2
    addsd   xmm1, xmm3                                // c1 + c2x2 + c3x4 + c4x6
    mulsd   xmm1, xmm4                                // c1x4 + c2x6 + c3x8 + c4x10
    addsd   xmm0, QWORD PTR L_real_one[rip]                // 1 - 0.5x2
    addsd   xmm0, xmm1                                // 1 - 0.5x2 + c1x4 + c2x6 + c3x8 + c4x10

Lcosf_sse2_adjust_region:
// xmm1 is cos or sin, relies on previous sections to
//      switch (region)
    add          eax, 1
    and          eax, 2
    jz           Lcosf_sse2_cleanup
//; if region 1 or 2 then we negate the result.
    xorpd        xmm2, xmm2
    subsd        xmm2, xmm0
    movapd       xmm0, xmm2

.align 16
Lcosf_sse2_cleanup:
    cvtsd2ss     xmm0, xmm0
    StackDeallocate stack_size
    ret


Lcosf_sse2_naninf:
    call         fname_special
    StackDeallocate stack_size
    ret


.align 16
Lcosf_fma3:
    vmovd        eax,xmm0
    mov          r8d,L_inf_mask_32[rip]
    and          eax,r8d
    cmp          eax, r8d
    jz           Lcosf_fma3_naninf

    vcvtss2sd    xmm5,xmm0,xmm0
    vmovq        r9,xmm5
    btr          r9,63                    //clear sign

    cmp          r9,L_piby2[rip]
    jg           Lcosf_fma3_range_reduce
    cmp          r9,L_two_to_neg_7[rip]
    jge          Lcosf_fma3_compute_cosf_piby_4
    cmp          r9,L_two_to_neg_13[rip]
    jge          Lcosf_fma3_compute_1_xx_5

    vmovq        xmm0,QWORD PTR L_real_one[rip]
    // Here we need to set inexact
    vaddsd       xmm0,xmm0,L__min_norm_double[rip]  // this will set inexact
    jmp          Lcosf_fma3_return

.align 16
Lcosf_fma3_compute_1_xx_5:
    vmulsd       xmm0,xmm5,QWORD PTR L_one_half[rip]
    vfnmadd213sd xmm0,xmm5,L_real_one[rip]           // xmm9 1.0 - x*x*(double2)0.5
    jmp          Lcosf_fma3_return

.align 16
Lcosf_fma3_compute_cosf_piby_4:
    movsd        xmm0,xmm5
    vmovapd      xmm2,L_real_one[rip]
    vmulsd       xmm3,xmm0,xmm0
    vmulsd       xmm1,xmm3,L_one_half[rip]           // xmm1 <-- r
    vsubsd       xmm2,xmm2,xmm1
    vmovsd       xmm1,__Lcosfarray[rip]+0x018
    vfmadd231sd  xmm1,xmm3,__Lcosfarray[rip]+0x020
    vfmadd213sd  xmm1,xmm3,__Lcosfarray[rip]+0x010
    vfmadd213sd  xmm1,xmm3,__Lcosfarray[rip]+0x008
    vmulsd       xmm3,xmm3,xmm3                 // xmm3 <-- x^4
    vmovdqa      xmm0,xmm2
    vfmadd231sd  xmm0,xmm1,xmm3
    jmp          Lcosf_fma3_return

.align 16
Lcosf_fma3_range_reduce:
    vmovq        xmm0,r9                        // xmm0 <-- |x|
    cmp          r9,L_large_x_fma3[rip]
    jge          Lcosf_reduce_precise

//cosff_range_e_5_s:
    vandpd       xmm1,xmm0,L_sign_mask[rip]
    vmovapd      xmm2,L_2bypi[rip]
    vfmadd213sd  xmm2,xmm1,L_one_half[rip]
    vcvttpd2dq   xmm2,xmm2
    vpmovsxdq    xmm1,xmm2
    vandpd       xmm4,xmm1,L__int_three[rip]         // region xmm4
    vshufps      xmm1 ,xmm1,xmm1,8
    vcvtdq2pd    xmm1,xmm1
    vmovdqa      xmm2,xmm0
    vfnmadd231sd xmm2,xmm1,L_piby2_1[rip]            // xmm2 rhead
    vmulsd       xmm3,xmm1,L_piby2_1tail[rip]        // xmm3 rtail
    vsubsd       xmm0,xmm2,xmm3                 // r_1  xmm0
    vsubsd       xmm2,xmm2,xmm0
    vsubsd       xmm1,xmm2,xmm3
    vmovq        rax,xmm4
    jmp          Lcosf_exit_s

.align 16
Lcosf_reduce_precise:

    vmovq        xmm0,r9               // r9 <-- |x|
    cmp          r9,L_large_x_fma3[rip]
    jge          Lcos_remainder_piby2

    // __remainder_piby2_fma3 and __remainder_piby2_fma3_bdl
    // have the following conventions:
    // on input
    //   x is in xmm0
    // on output
    //   r is in xmm0
    //   rr is in xmm1
    //   region is in rax
    // The _bdl routine is guaranteed not to touch r10

Lcos_remainder_piby2_small: //; unused label
    // Boldo-Daumas-Li reduction for reasonably small |x|
    call         __remainder_piby2_fma3_bdl
    jmp          Lcosf_exit_s

.align 16
Lcos_remainder_piby2:
    // argument reduction for general x
    call         __remainder_piby2_fma3
Lcosf_exit_s:
    bt           rax,0
    jnc          Lcosf_piby4_compute

//sinf_piby4_compute:
//   vmovapd      xmm1,__Lsinfarray+010h
    vmovsd       xmm1,__Lsinfarray[rip]+0x010
    vmulsd       xmm3,xmm0,xmm0
    vfmadd231sd  xmm1,xmm3,__Lsinfarray[rip]+0x018
    vfmadd213sd  xmm1,xmm3,__Lsinfarray[rip]+0x008
    vfmadd213sd  xmm1,xmm3,__Lsinfarray[rip]
    vmulsd       xmm3,xmm0,xmm3                 // xmm3 <-- x^3
    vfmadd231sd  xmm0,xmm1,xmm3
    jmp          Lcosf_fma3_adjust_sign

.align 16
Lcosf_piby4_compute:
    vmovapd      xmm2,L_real_one[rip]
    vmulsd       xmm3,xmm0,xmm0
    vmulsd       xmm1,xmm3,L_one_half[rip]           // xmm1 <-- r
    vsubsd       xmm2,xmm2,xmm1
    vmovsd       xmm1,__Lcosfarray[rip]+0x018
    vfmadd231sd  xmm1 ,xmm3,__Lcosfarray[rip]+0x020
    vfmadd213sd  xmm1 ,xmm3,__Lcosfarray[rip]+0x010
    vfmadd213sd  xmm1 ,xmm3,__Lcosfarray[rip]+0x008
    vmulsd       xmm3,xmm3,xmm3                 // xmm3 <-- x^4
    vmovdqa      xmm0, xmm2
    vfmadd231sd  xmm0 ,xmm1,xmm3

Lcosf_fma3_adjust_sign:
    // assuming FMA3 ==> AVX ==> SSE4.1
//    vpcmpeqq     xmm1,xmm4,XMMWORD PTR L_int_one
//    vpcmpeqq     xmm2,xmm4,XMMWORD PTR L_int_two
//    vorpd        xmm3,xmm2,xmm1

//    vandpd       xmm3,xmm3,L_signbit

    add          rax,1                    // 1,2 --> 2,3
    shr          rax,1                    // 2,3 --> 1
    shl          rax,63                   // 1 --> sign bit
    vmovq        xmm3,rax

    vxorpd       xmm0,xmm0,xmm3

Lcosf_fma3_return:
    vcvtsd2ss    xmm0,xmm0,xmm0
    StackDeallocate stack_size
    ret

Lcosf_fma3_naninf:
    call         fname_special
    StackDeallocate stack_size
    ret

.seh_endproc
.endfunc
// END

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
//
// An implementation of the cos function.
//
// Prototype:
//
//     double cos(double x);
//
//   Computes cos(x).
//   It will provide proper C99 return values,
//   but may not raise floating point status bits properly.
//   Based on the NAG C implementation.
//
// If FMA3 hardware is available, an FMA3 implementation of cos will be used.

.section .rdata
.align 16
L_real_piby2_1:          .quad 0x03ff921fb54400000         // piby2_1
                        .quad 0
L_real_piby2_1tail:      .quad 0x03dd0b4611a626331         // piby2_1tail
                        .quad 0
L_real_piby2_2:          .quad 0x03dd0b4611a600000         // piby2_2
                        .quad 0
L_real_piby2_2tail:      .quad 0x03ba3198a2e037073         // piby2_2tail
                        .quad 0                 

.align 16
L_one:           .quad 0x03FF0000000000000, 0x03FF0000000000000
L_signbit:       .quad 0x08000000000000000, 0x00000000000000000
L_int_one:       .quad 0x00000000000000001, 0x00000000000000000
L_int_two:       .quad 0x00000000000000002, 0x00000000000000000

L_2_by_pi:       .quad 0x03fe45f306dc9c883     // 2/pi
L_one_half:      .quad 0x03FE0000000000000     // .5
L_neg_one_half:  .quad 0x0bfe0000000000000     // - 0.5
L_two_to_neg_27: .quad 0x03e40000000000000     // 2^-27
L_two_to_neg_13: .quad 0x03f20000000000000     // 2^-13
L_piby4:         .quad 0x03FE921FB54442D18     // pi/4
L_small_arg_cw:  .quad 0x0411E848000000000     // 5.e5, appropriate for CW
L_small_arg_bdl: .quad 0x0417312D000000000     // 2e7, works for BDL
L_sign_mask:     .quad 0x07FFFFFFFFFFFFFFF

L__inf_mask_64:  .quad 0x07FF0000000000000     // +Inf



//EXTRN __Lcosarray:QWORD
//EXTRN __Lsinarray:QWORD
//EXTRN __use_fma3_lib:DWORD

// local storage offsets
#define p_temp   0x020                     // temporary for get/put bits operation
#define p_temp1   0x030                     // temporary for get/put bits operation
#define dummy_space   0x040
#define stack_size   0x068

#include "fm.inc.h"

#define fname cos
#define fname_special _cos_special

//Define name and any external functions being called
//EXTERN           __remainder_piby2_forAsm   : PROC
//EXTERN           __remainder_piby2_fma3     : PROC
//EXTERN           __remainder_piby2_fma3_bdl : PROC
//EXTERN           fname_special              : PROC

.code64 .intel_syntax noprefix

.global fname
.func fname
fname:
.seh_proc fname

    StackAllocate stack_size
    .seh_endprologue

    cmp          DWORD PTR __use_fma3_lib[rip], 0
    jne          L_cos_fma3

Lcos_sse2:
    movd         rdx, xmm0
    xorpd        xmm2, xmm2               // zeroed out for later use

    mov          r10, rdx
    btr          r10, 63                  // r10 <-- |x|
    cmp          r10, L_piby4[rip]
    jb           Lcos_sse2_absx_lt_piby4

Lcos_absx_nlt_piby4:                      // common case

//  Here rdx has x, r10 has |x|
    movd    xmm0, r10                     // xmm0 <-- |x|

    cmp     r10, QWORD PTR L_small_arg_cw[rip]
    jae     Lcos_reduce_precise           // Note NaN/Inf will branch

// At this point we have |x| < L_small_arg_cw, which is currently 500000.
// Note that if |x| were too large, conversion of npi2 to integer would fail.
// We reduce  the argument to be in a range from -pi/4 to +pi/4
// by subtracting multiples of pi/2
    movapd  xmm2, xmm0
    mulsd   xmm2, L_2_by_pi[rip]
    movapd  xmm4, xmm0

//      xexp  = ax >> EXPSHIFTBITS_DP64;
    mov     r9, r10
    shr     r9, 52                        // >>EXPSHIFTBITS_DP64

// How many pi/2 is |x| a multiple of?
//      npi2  = (int)(x * twobypi + 0.5);
    addsd   xmm2, L_one_half[rip]              // npi2

    movsd   xmm3, L_real_piby2_1[rip]
    cvttpd2dq    xmm0, xmm2               // convert npi2 to integer
    movsd   xmm1, L_real_piby2_1tail[rip]
    cvtdq2pd    xmm2, xmm0                // and back to double.

//  Subtract the multiple from x to get an extra-precision remainder
//      rhead  = x - npi2 * piby2_1;
    mulsd   xmm3, xmm2
    subsd   xmm4, xmm3                    // rhead

//      rtail  = npi2 * piby2_1tail;
    mulsd   xmm1, xmm2                    // rtail
    movd    eax, xmm0                     // eax <-- npi2

//      GET_BITS_DP64(rhead-rtail, uy);
// originally only rhead
    movapd  xmm0, xmm4
    subsd   xmm0, xmm1

    movsd   xmm3, L_real_piby2_2[rip]
    movd    rcx, xmm0                     // rcx <-- rhead - rtail
    movsd   xmm5, L_real_piby2_2tail[rip]      // piby2_2tail

//    xmm0=r, xmm1=rtail, xmm2=npi2, xmm3=temp for calc,
//    xmm4=rhead xmm5= temp for calc
//      expdiff = xexp - ((uy & EXPBITS_DP64) >> EXPSHIFTBITS_DP64);
//   expdiff measures how close rhead - rtail is to |x|
//   (larger expdiff ==> more cancellation in |x| - (rhead-rtail) ==> closer)
    shl     rcx, 1                        // strip any sign bit
    shr     rcx, 53                       // >> EXPSHIFTBITS_DP64 +1
    sub     r9, rcx                       // expdiff

//;      if (expdiff > 15)
    cmp     r9, 15
    jle     Lcos_sse2_cw_reduction_done

//   Here the remainder is pretty small compared with x, which
//   implies that x is a near multiple of pi/2
//   (x matches the multiple to at least 15 bits)
//   So we do another stage of argument reduction.

//          t  = rhead;
    movapd  xmm1, xmm4

//          rtail  = npi2 * piby2_2;
    mulsd   xmm3, xmm2

//          rhead  = t - rtail;
    mulsd   xmm5, xmm2                    // npi2 * piby2_2tail
    subsd   xmm4, xmm3                    // rhead

//          rtail  = npi2 * piby2_2tail - ((t - rhead) - rtail);
    subsd   xmm1, xmm4                    // t - rhead
    subsd   xmm1, xmm3                    // -rtail
    subsd   xmm5, xmm1                    // rtail

//      r = rhead - rtail;
    movapd  xmm0, xmm4

//HARSHA
//xmm1=rtail
    movapd  xmm1, xmm5                    // xmm1 <-- copy of rtail
    subsd   xmm0, xmm5

//    xmm0=r, xmm4=rhead, xmm1=rtail
Lcos_sse2_cw_reduction_done:
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//; if the input was close to a pi/2 multiple
// The original NAG code missed this trick.
// If the input is very close to n*pi/2 after reduction, so  r < 2^-27,
// then the cos is either ~ 1.0 or ~r, to within 53 bits.

// NOTE: Unfortunately, this introduces two jcc instructions close to each
// other and to other branches.  As r < 2^-13 should be rather uncommon,
// the problems for branch prediction outweigh the computational savings. - WAT
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//      region = npi2 & 3;
    subsd   xmm4, xmm0                    // rhead-r
    subsd   xmm4, xmm1                    // rr = (rhead-r) - rtail

Lcos_piby4:
// perform taylor series to calc sinx or cosx
//  x2 = r * r;

//xmm4 = a part of rr for the sin path, xmm4 is overwritten in the cos path
//instead use xmm3 because that was freed up in the sin path, xmm3 is overwritten in sin path
    movapd  xmm3, xmm0
    movapd  xmm2, xmm0
    mulsd   xmm2, xmm0                                //x2

    bt      eax,0
    jnc     Lcos_sse2_calc_cos

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// region 1 or 3 do a sin calculation
    movsd   xmm3, __Lsinarray[rip]+0x50                     // s6
    mulsd   xmm3, xmm2                                // x2s6
    movsd   xmm5, __Lsinarray[rip]+0x20                     // s3
    movsd   QWORD PTR p_temp[rsp], xmm4               // store xx
    movapd  xmm1, xmm2                                // move for x4
    mulsd   xmm1, xmm2                                // x4
    movsd   QWORD PTR p_temp1[rsp], xmm0              // store x
    mulsd   xmm5, xmm2                                // x2s3
    movapd  xmm4, xmm0                                // move for x3
    addsd   xmm3, __Lsinarray[rip]+0x40                     // s5+x2s6
    mulsd   xmm1, xmm2                                // x6
    mulsd   xmm3, xmm2                                // x2(s5+x2s6)
    mulsd   xmm4, xmm2                                // x3
    addsd   xmm5, __Lsinarray[rip]+0x10                     // s2+x2s3
    mulsd   xmm5, xmm2                                // x2(s2+x2s3)
    addsd   xmm3, __Lsinarray[rip]+0x30                     // s4 + x2(s5+x2s6)
    mulsd   xmm2, L_one_half[rip]                              // 0.5 *x2
    movsd   xmm0, QWORD PTR p_temp[rsp]               // load xx
    mulsd   xmm3, xmm1                                // x6(s4 + x2(s5+x2s6))
    addsd   xmm5, __Lsinarray[rip]                         // s1+x2(s2+x2s3)
    mulsd   xmm2, xmm0                                // 0.5 * x2 *xx
    addsd   xmm3, xmm5                                // zs
    mulsd   xmm4, xmm3                                // *x3
    subsd   xmm4, xmm2                                // x3*zs - 0.5 * x2 *xx
    addsd   xmm0, xmm4                                // +xx
    addsd   xmm0, QWORD PTR p_temp1[rsp]              // +x
    
    jmp     Lcos_sse2_adjust_region

.align 16
Lcos_sse2_calc_cos:
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// region 0 or 2     - do a cos calculation
//  zc = (c2 + x2 * (c3 + x2 * (c4 + x2 * (c5 + x2 * c6))));
    mulsd   xmm4, xmm0                                // x*xx
    movsd   xmm5, L_one_half[rip]
    movsd   xmm1, __Lcosarray[rip]+0x50                     // c6
    movsd   xmm0, __Lcosarray[rip]+0x20                     // c3
    mulsd   xmm5, xmm2                                // r = 0.5 *x2
    movapd  xmm3, xmm2                                // copy of x2
    movsd   QWORD PTR p_temp[rsp], xmm4               // store x*xx
    mulsd   xmm1, xmm2                                // c6*x2
    mulsd   xmm0, xmm2                                // c3*x2
    subsd   xmm5, L_one[rip]                               // -t=r-1.0, trash r
    mulsd   xmm3, xmm2                                // x4
    addsd   xmm1, __Lcosarray[rip]+0x40                     // c5+x2c6
    addsd   xmm0, __Lcosarray[rip]+0x10                     // c2+x2C3
    addsd   xmm5, L_one[rip]                               // 1 + (-t), trash t
    mulsd   xmm3, xmm2                                // x6
    mulsd   xmm1, xmm2                                // x2(c5+x2c6)
    mulsd   xmm0, xmm2                                // x2(c2+x2C3)
    movapd  xmm4, xmm2                                // copy of x2
    mulsd   xmm4, L_one_half[rip]                              // r recalculate
    addsd   xmm1, __Lcosarray[rip]+0x30                     // c4 + x2(c5+x2c6)
    addsd   xmm0, __Lcosarray[rip]                         // c1+x2(c2+x2C3)
    mulsd   xmm2, xmm2                                // x4 recalculate
    subsd   xmm5, xmm4                                // (1 + (-t)) - r
    mulsd   xmm1, xmm3                                // x6(c4 + x2(c5+x2c6))
    addsd   xmm0, xmm1                                // zc
    subsd   xmm4, L_one[rip]                               // t relaculate
    subsd   xmm5, QWORD PTR p_temp[rsp]               // ((1 + (-t)) - r) - x*xx
    mulsd   xmm0, xmm2                                // x4 * zc
    addsd   xmm0, xmm5                                // x4 * zc + ((1 + (-t)) - r -x*xx)
    subsd   xmm0, xmm4                                // result - (-t)

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

Lcos_sse2_adjust_region:
//      switch (region)
    add     eax, 1
    and     eax, 2
    jz      Lcos_sse2_cleanup
    
//; if the original region 1 or 2 then we negate the result.
    movapd  xmm2, xmm0
    xorpd   xmm0, xmm0
    subsd   xmm0, xmm2

.align 16
Lcos_sse2_cleanup:
    StackDeallocate stack_size
    ret






.align 16
Lcos_sse2_absx_lt_piby4:
//          cos = cos_piby4(x, 0.0);

//  x2 = r * r;
    cmp     r10, L_two_to_neg_13[rip]
    jb      Lcos_sse2_x_small
    movapd  xmm2, xmm0
    mulsd   xmm2, xmm0                                // x2

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// region 0 - do a cos calculation
//  zc = (c2 + x2 * (c3 + x2 * (c4 + x2 * (c5 + x2 * c6))));
    movsd   xmm1, __Lcosarray[rip]+0x10                     // c2
    movapd  xmm4, xmm2                                // move for x4
    mulsd   xmm4, xmm2                                // x4
    movsd   xmm3, __Lcosarray[rip]+0x30                     // c4
    mulsd   xmm1, xmm2                                // c2x2
    movsd   xmm5, __Lcosarray[rip]+0x50                     // c6
    mulsd   xmm3, xmm2                                // c4x2
    movapd  xmm0, xmm4                                // move for x8
    mulsd   xmm5, xmm2                                // c6x2
    mulsd   xmm0, xmm4                                // x8
    addsd   xmm1, __Lcosarray[rip]                         // c1 + c2x2
    mulsd   xmm1, xmm4                                // c1x4 + c2x6
    addsd   xmm3, __Lcosarray[rip]+0x20                     // c3 + c4x2
    mulsd   xmm2, L_neg_one_half[rip]                      // -0.5x2, destroy xmm2
    addsd   xmm5, __Lcosarray[rip]+0x40                     // c5 + c6x2
    mulsd   xmm3, xmm0                                // c3x8 + c4x10    
    mulsd   xmm4, xmm0                                // x12    
    mulsd   xmm4, xmm5                                // c5x12 + c6x14

    movsd   xmm0, L_one[rip]
    addsd   xmm1, xmm3                                // c1x4 + c2x6 + c3x8 + c4x10
    movapd  xmm3, xmm2                                // preserve -0.5x2
    addsd   xmm2, xmm0                                // t = 1 - 0.5x2
    subsd   xmm0, xmm2                                // 1-t
    addsd   xmm0, xmm3                                // (1-t) - r
    addsd   xmm1, xmm4                                // c1x4 + c2x6 + c3x8 + c4x10 + c5x12 + c6x14
    addsd   xmm0, xmm1                                // (1-t) - r + c1x4 + c2x6 + c3x8 + c4x10 + c5x12 + c6x14
    addsd   xmm0, xmm2                                // 1 - 0.5x2 + above
        
    StackDeallocate stack_size
    ret

.align 16
Lcos_sse2_x_small:
    movsd   xmm2, xmm0
    movsd   xmm0, L_one[rip]
    cmp     r10, L_two_to_neg_27[rip]
    jb      Lcos_sse2_x_smaller
    mulsd   xmm2, xmm2
    mulsd   xmm2, L_one_half[rip]
    subsd   xmm0, xmm2
    StackDeallocate stack_size
    ret

.align 16
Lcos_sse2_x_smaller:
    movsd   xmm0, L_one[rip]
    addsd   xmm0, L_int_one[rip]     // really adding smallest subnormal; set inexact
    StackDeallocate stack_size
    ret

.align 16
Lcos_reduce_precise:
//   Reduce x into range [-pi/4, pi/4]
    cmp     r10, L__inf_mask_64[rip]
    jae     Lcos_x_naninf
    call    __remainder_piby2_forAsm

    // At this point xmm0 has r, xmm1 has rr, rax has region

    movapd  xmm4, xmm1                // xmm4 <-- rr
    jmp     Lcos_piby4

// xmm0 = x, xmm4 = xx, eax= region


.align 16
Lcos_x_naninf:
    call    fname_special
    StackDeallocate stack_size
    ret

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
// From this point we assume that FMA3 and AVX hardware are present.

.align 16
L_cos_fma3:
    vmovq        r9,xmm0
    mov          rax,r9
    and          r9,L_sign_mask[rip]           // clear sign

Lcos_early_exit_s_1:                   //; unused label
    cmp          r9,L_piby4[rip]
    jg           Lcos_early_exit_s     // Note that NaN will branch
    cmp          r9,L_two_to_neg_13[rip]
    jge          Lcompute_cos_pyby_4
    cmp          r9,L_two_to_neg_27[rip]
    jge          Lcompute_1_xx_5
    vmovq        xmm0,L_one[rip]               // for tiniest args, cos is 1
    jmp          Lreturn_no_restore

Lcompute_1_xx_5:
    vmulsd       xmm1,xmm0,L_one_half[rip]     // xmm1l <-- .5*x
    vfnmadd213sd xmm0,xmm1,L_one[rip]          // xmm0l <-- 1.0 - (.5*x)*x
    jmp          Lreturn_no_restore

Lcompute_cos_pyby_4:
    // make sure this is accurate enough
    // note that x^2 can't be all that close to 1 here
    vmulsd       xmm3,xmm0,xmm0           // xmm3 <-- xx = x*x
    vmovapd      xmm0,__Lcosarray[rip]+0x050    // xmm0 <-- c5   
    vfmadd213sd  xmm0,xmm3,__Lcosarray[rip]+0x040  // xmm0 <-- c5*xx + c4
    vfmadd213sd  xmm0,xmm3,__Lcosarray[rip]+0x030  // xmm0 <-- (c5*xx + c4)*xx + c3
    vfmadd213sd  xmm0,xmm3,__Lcosarray[rip]+0x020
    vfmadd213sd  xmm0,xmm3,__Lcosarray[rip]+0x010
    vfmadd213sd  xmm0,xmm3,__Lcosarray[rip]
    vfmsub213sd  xmm0,xmm3,L_one_half[rip]
    vfmadd213sd  xmm0,xmm3,L_one[rip]

    jmp          Lreturn_no_restore

Lcos_early_exit_s:
    mov          r8,L__inf_mask_64[rip]
    and          rax,r8
    cmp          rax, r8
    jz           Lcos_x_naninf    

Lrange_reduce:
    vmovq        xmm0,r9               // r9 <-- |x|
    cmp          r9,L_small_arg_bdl[rip]     
    jae          Lcos_remainder_piby2

    // For __remainder_piby2_fma3 and __remainder_piby2_fma3_bdl
    // on input
    //   x is in xmm0
    // on output
    //   r is in xmm0
    //   rr is in xmm1
    //   region is in rax

    // Boldo-Daumas-Li reduction for reasonably small |x|
    call         __remainder_piby2_fma3_bdl

//;      if region is 0 or 2    do a cos calc.
//;      if region is 1 or 3    do a sin calc.
Lcos_exit_s:
    bt           rax,0
    jc           Lsin_piby4_compute

Lcos_piby4_compute:                    //; unused label
    // compute the cosine of r+rr, where this sum is in [-pi/4,pi/4]
    vmovapd      xmm2,L_one[rip]
    vmulsd       xmm3,xmm0,xmm0        // xmm3 <-- x * x
    vmulsd       xmm5,xmm3,L_one_half[rip]      // xmm5 <-- x*x*.5 == r
    vsubsd       xmm4,xmm2,xmm5        // xmm4 <-- t = 1. - x*x*.5
    vsubsd       xmm2,xmm2,xmm4        // 1-t
    vsubsd       xmm2,xmm2,xmm5        // xmm2 <-- (1-t) - r
    vmovapd      xmm5,__Lcosarray[rip]+0x040
    vfnmadd231sd xmm2,xmm0,xmm1        // (1.0 - t) - r) - x * xx) xmm2
    vmulsd       xmm1,xmm3,xmm3           // x2 * x2 xmm1
    vfmadd231sd  xmm5,xmm3,__Lcosarray[rip]+0x050
    vfmadd213sd  xmm5,xmm3,__Lcosarray[rip]+0x030
    vfmadd213sd  xmm5,xmm3,__Lcosarray[rip]+0x020
    vfmadd213sd  xmm5,xmm3,__Lcosarray[rip]+0x010
    vfmadd213sd  xmm5,xmm3,__Lcosarray[rip]
    vfmadd213sd  xmm5,xmm1,xmm2
    vaddsd       xmm0,xmm5,xmm4

    jmp          Lcos_exit_s_1

.align 16
Lsin_piby4_compute:
    // compute the sine of r+rr, where this sum is in [-pi/4,pi/4]
    vmovapd      xmm5,__Lsinarray[rip]+0x040
    vmulsd       xmm3,xmm0,xmm0        // xmm3 <-- x2 = x * x
    vfmadd231sd  xmm5,xmm3,__Lsinarray[rip]+0x050 
    vfmadd213sd  xmm5,xmm3,__Lsinarray[rip]+0x030
    vfmadd213sd  xmm5,xmm3,__Lsinarray[rip]+0x020
    vfmadd213sd  xmm5,xmm3,__Lsinarray[rip]+0x010 // xmm5 <-- r
    
    vmulsd       xmm4,xmm0,xmm3        // xmm4 <-- x3 = x*x*x
    vmulsd       xmm2,xmm4,xmm5        // xmm2 <-- x*x*x * r
    vmulsd       xmm5,xmm1,L_one_half[rip]      // xmm5 <-- .5*x*x
    vsubsd       xmm2,xmm5,xmm2        // xmm2 <-- .5*x*x - x*x*x*r
    vmulsd       xmm2,xmm3,xmm2
    vsubsd       xmm2,xmm2,xmm1   
    vfnmadd231sd xmm2, xmm4,__Lsinarray[rip] 
    vsubsd       xmm0,xmm0,xmm2

Lcos_exit_s_1:
    xor          r8,r8
    add          eax, 1
    and          eax, 2
    cmovnz       r8, L_signbit[rip]
    vmovq        xmm3,r8
    vxorpd       xmm0,xmm0,xmm3

Lreturn_restore_regs:
    StackDeallocate stack_size
    ret

Lreturn_no_restore:
    StackDeallocate stack_size
    ret

.align 16
Lcos_remainder_piby2:
    // argument reduction for general x
    call         __remainder_piby2_fma3
    jmp          Lcos_exit_s


.seh_endproc
.endfunc
// END

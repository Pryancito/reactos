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
// An implementation of the remainder by pi/2 function
// This is a service routine for use by trig functions coded in C
//

#define fname __remainder_piby2d2f_forC

#define save_rdi       0x20
#define save_rsi       0x30
#define stack_size      0x088
#include "fm.inc.h"

.code64 .intel_syntax noprefix
.global fname
.func fname
fname:
.seh_proc fname

    StackAllocate stack_size
    SaveReg rdi,save_rdi
    SaveReg rsi,save_rsi 
    .seh_endprologue
 
 mov rdi, rcx
 mov rsi, rdx
 mov rdx, r8

 //get the unbiased exponent and the mantissa part of x
 movd    xmm0,rdi
 lea    r9,L__2_by_pi_bits[rip]
 
 //xexp = (x >> 52) - 1023
 movd   r11,xmm0
 mov    rcx,r11 
 shr    r11,52
 sub    r11,1023 //r11 = xexp = exponent of input x 

 //calculate the last byte from which to start multiplication
 //last = 134 - (xexp >> 3) 
 mov    r10,r11
 shr    r10,3
 sub    r10,134 //r10 = -last
 neg    r10 //r10 = last

 //load 64 bits of 2_by_pi
 mov    rax,[r9 + r10]
 mov    rdi,rdx // save address of region since mul modifies rdx
 
 //mantissa of x = ((x << 12) >> 12) | implied bit
 shl    rcx,12
 shr    rcx,12 //rcx = mantissa part of input x 
 bts    rcx,52 //add the implied bit as well 

 //load next 128 bits of 2_by_pi 
 add    r10,8 //increment to next 8 bytes of 2_by_pi
 movdqu xmm0,[r9 + r10] 

 //do three 64-bit multiplications with mant of x 
 mul rcx
 mov    r8,rax //r8 = last 64 bits of multiplication = res1[2] 
 mov    r10,rdx //r10 = carry
 movd   rax,xmm0
 mul rcx
 //resexp = xexp & 7 
 and    r11,7 //r11 = resexp = xexp & 7 = last 3 bits
 psrldq xmm0,8 
 add    rax,r10 // add the previous carry
 adc    rdx,0
 mov    r9,rax //r9 = next 64 bits of multiplication = res1[1]
 mov    r10,rdx //r10 = carry
 movd   rax,xmm0
 mul    rcx
 add    r10,rax //r10 = most significant 64 bits = res1[0]
 
 //find the region 
 //last three bits ltb = most sig bits >> (54 - resexp));  decimal point in last 18 bits == 8 lsb's in first 64 bits and 8 msb's in next 64 bits
 //point_five = ltb & 01h;
 //region = ((ltb >> 1) + point_five) & 3; 
 mov    rcx,54
 mov    rax,r10
 sub    rcx,r11
 xor    rdx,rdx //rdx = sign of x(i.e first part of x * 2bypi) 
 shr    rax,cl 
 jnc    L__no_point_five
 //;if there is carry.. then negate the result of multiplication
 not r10
 not r9
 not r8
 mov    rdx,0x08000000000000000

.align  16 
L__no_point_five:
 adc    rax,0
 and    rax,3
 mov    DWORD PTR[rdi],eax //store region to memory

 //calculate the number of integer bits and zero them out
 mov    rcx,r11 
 add    rcx,10 //rcx = no. of integer bits
 shl    r10,cl
 shr    r10,cl //r10 contains only mant bits
 sub    rcx,64 //form the exponent
 mov    r11,rcx
 
 //find the highest set bit
 bsr    rcx,r10
 jnz L__form_mantissa
 mov    r10,r9
 mov    r9,r8
 bsr    rcx,r10 //rcx = hsb
 sub    r11,64
 
 
.align  16 
L__form_mantissa:
 add    r11,rcx //for exp of x
 sub    rcx,52 //rcx = no. of bits to shift in r10 
 cmp    rcx,0
 jl L__hsb_below_52
 je L__form_numbers
 //hsb above 52
 mov    r8,r10 //previous contents of r8 not required
 shr    r10,cl //r10 = mantissa of x with hsb at 52
 jmp L__form_numbers
 
.align  16 
L__hsb_below_52:
 neg rcx
 mov    rax,r9
 shl    r10,cl
 shl    r9,cl
 sub    rcx,64
 neg rcx
 shr    rax,cl
 or    r10,rax
 
.align  16
L__form_numbers:
 add    r11,1023
 btr    r10,52 //remove the implied bit
 mov    rcx,r11
 or    r10,rdx //put the sign 
 shl    rcx,52
 or    r10,rcx //x is in r10
 
 movd    xmm0,r10 //xmm0 = x
 mulsd    xmm0,L__piby2[rip]
 movsd    QWORD PTR[rsi],xmm0
 RestoreReg rsi,save_rsi
 RestoreReg rdi,save_rdi
 StackDeallocate stack_size 
 ret 
 
.seh_endproc
.endfunc

.section .rdata
.align 16
L__piby2: .quad 0x03ff921fb54442d18

.align 16
L__2_by_pi_bits: .byte 224
  .byte 241
  .byte 27
  .byte 193
  .byte 12
  .byte 88
  .byte 33
  .byte 116
  .byte 53
  .byte 126
  .byte 196
  .byte 126
  .byte 237
  .byte 175
  .byte 169
  .byte 75
  .byte 74
  .byte 41
  .byte 222
  .byte 231
  .byte 28
  .byte 244
  .byte 236
  .byte 197
  .byte 151
  .byte 175
  .byte 31
  .byte 235
  .byte 158
  .byte 212
  .byte 181
  .byte 168
  .byte 127
  .byte 121
  .byte 154
  .byte 253
  .byte 24
  .byte 61
  .byte 221
  .byte 38
  .byte 44
  .byte 159
  .byte 60
  .byte 251
  .byte 217
  .byte 180
  .byte 125
  .byte 180
  .byte 41
  .byte 104
  .byte 45
  .byte 70
  .byte 188
  .byte 188
  .byte 63
  .byte 96
  .byte 22
  .byte 120
  .byte 255
  .byte 95
  .byte 226
  .byte 127
  .byte 236
  .byte 160
  .byte 228
  .byte 247
  .byte 46
  .byte 126
  .byte 17
  .byte 114
  .byte 210
  .byte 231
  .byte 76
  .byte 13
  .byte 230
  .byte 88
  .byte 71
  .byte 230
  .byte 4
  .byte 249
  .byte 125
  .byte 209
  .byte 154
  .byte 192
  .byte 113
  .byte 166
  .byte 19
  .byte 18
  .byte 237
  .byte 186
  .byte 212
  .byte 215
  .byte 8
  .byte 162
  .byte 251
  .byte 156
  .byte 166
  .byte 196
  .byte 114
  .byte 172
  .byte 119
  .byte 248
  .byte 115
  .byte 72
  .byte 70
  .byte 39
  .byte 168
  .byte 187
  .byte 36
  .byte 25
  .byte 128
  .byte 75
  .byte 55
  .byte 9
  .byte 233
  .byte 184
  .byte 145
  .byte 220
  .byte 134
  .byte 21
  .byte 239
  .byte 122
  .byte 175
  .byte 142
  .byte 69
  .byte 249
  .byte 7
  .byte 65
  .byte 14
  .byte 241
  .byte 100
  .byte 86
  .byte 138
  .byte 109
  .byte 3
  .byte 119
  .byte 211
  .byte 212
  .byte 71
  .byte 95
  .byte 157
  .byte 240
  .byte 167
  .byte 84
  .byte 16
  .byte 57
  .byte 185
  .byte 13
  .byte 230
  .byte 139
  .byte 2
  .byte 0
  .byte 0
  .byte 0
  .byte 0
  .byte 0
  .byte 0
  .byte 0

// END

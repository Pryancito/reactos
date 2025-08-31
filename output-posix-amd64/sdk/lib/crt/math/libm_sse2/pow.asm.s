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
// pow.asm
//
// An implementation of the pow libm function.
//
// Prototype:
//
//     double pow(double x, double y);
//

//
//   Algorithm:
//       x^y = e^(y*ln(x))
//
//       Look in exp, log for the respective algorithms
//

.section .rdata

.align 16

// these codes and the ones in the corresponding .c file have to match
__flag_x_one_y_snan:             .long 1
__flag_x_zero_z_inf:             .long 2
__flag_x_nan:                    .long 3
__flag_y_nan:                    .long 4
__flag_x_nan_y_nan:              .long 5
__flag_x_neg_y_notint:           .long 6
__flag_z_zero:                   .long 7
__flag_z_denormal:               .long 8
__flag_z_inf:                    .long 9

.align 16
   
__ay_max_bound:              .quad 0x43e0000000000000
__ay_min_bound:              .quad 0x3c00000000000000
__sign_mask:                 .quad 0x8000000000000000
__sign_and_exp_mask:         .quad 0x0fff0000000000000
__exp_mask:                  .quad 0x7ff0000000000000
__neg_inf:                   .quad 0x0fff0000000000000
__pos_inf:                   .quad 0x7ff0000000000000
__pos_one:                   .quad 0x3ff0000000000000
__pos_zero:                  .quad 0x0000000000000000
__exp_mant_mask:             .quad 0x7fffffffffffffff
__mant_mask:                 .quad 0x000fffffffffffff
__ind_pattern:               .quad 0x0fff8000000000000


__neg_qnan:                  .quad 0x0fff8000000000000
__qnan:                      .quad 0x7ff8000000000000
__qnan_set:                  .quad 0x0008000000000000

__neg_one:                   .quad 0x0bff0000000000000
__neg_zero:                  .quad 0x8000000000000000

__exp_shift:                 .quad 0x0000000000000034 // 52
__exp_bias:                  .quad 0x00000000000003ff // 1023
__exp_bias_m1:               .quad 0x00000000000003fe // 1022

__yexp_53:                   .quad 0x0000000000000035 // 53
__mant_full:                 .quad 0x000fffffffffffff
__1_before_mant:             .quad 0x0010000000000000

__mask_mant_all8:            .quad 0x000ff00000000000
__mask_mant9:                .quad 0x0000080000000000



.align 16
__real_fffffffff8000000:     .quad 0x0fffffffff8000000
                            .quad 0x0fffffffff8000000

__mask_8000000000000000:     .quad 0x8000000000000000
                            .quad 0x8000000000000000

__real_4090040000000000:     .quad 0x4090040000000000
                            .quad 0x4090040000000000

__real_C090C80000000000:     .quad 0x0C090C80000000000
                            .quad 0x0C090C80000000000

//---------------------
// log data
//---------------------

.align 16

__real_ninf:     .quad 0x0fff0000000000000   // -inf
                .quad 0x0000000000000000
__real_inf:      .quad 0x7ff0000000000000    // +inf
                .quad 0x0000000000000000
__real_nan:      .quad 0x7ff8000000000000    // NaN
                .quad 0x0000000000000000
__real_mant:     .quad 0x000FFFFFFFFFFFFF    // mantissa bits
                .quad 0x0000000000000000
__mask_1023:     .quad 0x00000000000003ff
                .quad 0x0000000000000000
__mask_001:      .quad 0x0000000000000001
                .quad 0x0000000000000000

__real_log2_lead:    .quad 0x3fe62e42e0000000 // log2_lead  6.93147122859954833984e-01
                    .quad 0x0000000000000000
__real_log2_tail:    .quad 0x3e6efa39ef35793c // log2_tail  5.76999904754328540596e-08
                    .quad 0x0000000000000000

__real_two:          .quad 0x4000000000000000 // 2
                    .quad 0x0000000000000000

__real_one:          .quad 0x3ff0000000000000 // 1
                    .quad 0x0000000000000000

__real_half:         .quad 0x3fe0000000000000 // 1/2
                    .quad 0x0000000000000000

__mask_100:          .quad 0x0000000000000100
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
__real_1_over_7:     .quad 0x3fc2492492492494
                    .quad 0x0000000000000000

__mask_1023_f:       .quad 0x0c08ff80000000000
                    .quad 0x0000000000000000

__mask_2045:         .quad 0x00000000000007fd
                    .quad 0x0000000000000000

__real_threshold:    .quad 0x3fc0000000000000 // 0.125
                    .quad 0x3fc0000000000000

__real_notsign:      .quad 0x7ffFFFFFFFFFFFFF // ^sign bit
                    .quad 0x0000000000000000


//EXTRN __log_256_lead:QWORD
//EXTRN __log_256_tail:QWORD
//EXTRN __use_fma3_lib:DWORD

// This table differs from the tables in log_256_lead_tail_table.asm:
// the heads have fewer significant bits (hence the tails also differ).
.align 16
__log_F_inv_head:    .quad 0x4000000000000000
                    .quad 0x3fffe00000000000
                    .quad 0x3fffc00000000000
                    .quad 0x3fffa00000000000
                    .quad 0x3fff800000000000
                    .quad 0x3fff600000000000
                    .quad 0x3fff400000000000
                    .quad 0x3fff200000000000
                    .quad 0x3fff000000000000
                    .quad 0x3ffee00000000000
                    .quad 0x3ffec00000000000
                    .quad 0x3ffea00000000000
                    .quad 0x3ffe900000000000
                    .quad 0x3ffe700000000000
                    .quad 0x3ffe500000000000
                    .quad 0x3ffe300000000000
                    .quad 0x3ffe100000000000
                    .quad 0x3ffe000000000000
                    .quad 0x3ffde00000000000
                    .quad 0x3ffdc00000000000
                    .quad 0x3ffda00000000000
                    .quad 0x3ffd900000000000
                    .quad 0x3ffd700000000000
                    .quad 0x3ffd500000000000
                    .quad 0x3ffd400000000000
                    .quad 0x3ffd200000000000
                    .quad 0x3ffd000000000000
                    .quad 0x3ffcf00000000000
                    .quad 0x3ffcd00000000000
                    .quad 0x3ffcb00000000000
                    .quad 0x3ffca00000000000
                    .quad 0x3ffc800000000000
                    .quad 0x3ffc700000000000
                    .quad 0x3ffc500000000000
                    .quad 0x3ffc300000000000
                    .quad 0x3ffc200000000000
                    .quad 0x3ffc000000000000
                    .quad 0x3ffbf00000000000
                    .quad 0x3ffbd00000000000
                    .quad 0x3ffbc00000000000
                    .quad 0x3ffba00000000000
                    .quad 0x3ffb900000000000
                    .quad 0x3ffb700000000000
                    .quad 0x3ffb600000000000
                    .quad 0x3ffb400000000000
                    .quad 0x3ffb300000000000
                    .quad 0x3ffb200000000000
                    .quad 0x3ffb000000000000
                    .quad 0x3ffaf00000000000
                    .quad 0x3ffad00000000000
                    .quad 0x3ffac00000000000
                    .quad 0x3ffaa00000000000
                    .quad 0x3ffa900000000000
                    .quad 0x3ffa800000000000
                    .quad 0x3ffa600000000000
                    .quad 0x3ffa500000000000
                    .quad 0x3ffa400000000000
                    .quad 0x3ffa200000000000
                    .quad 0x3ffa100000000000
                    .quad 0x3ffa000000000000
                    .quad 0x3ff9e00000000000
                    .quad 0x3ff9d00000000000
                    .quad 0x3ff9c00000000000
                    .quad 0x3ff9a00000000000
                    .quad 0x3ff9900000000000
                    .quad 0x3ff9800000000000
                    .quad 0x3ff9700000000000
                    .quad 0x3ff9500000000000
                    .quad 0x3ff9400000000000
                    .quad 0x3ff9300000000000
                    .quad 0x3ff9200000000000
                    .quad 0x3ff9000000000000
                    .quad 0x3ff8f00000000000
                    .quad 0x3ff8e00000000000
                    .quad 0x3ff8d00000000000
                    .quad 0x3ff8b00000000000
                    .quad 0x3ff8a00000000000
                    .quad 0x3ff8900000000000
                    .quad 0x3ff8800000000000
                    .quad 0x3ff8700000000000
                    .quad 0x3ff8600000000000
                    .quad 0x3ff8400000000000
                    .quad 0x3ff8300000000000
                    .quad 0x3ff8200000000000
                    .quad 0x3ff8100000000000
                    .quad 0x3ff8000000000000
                    .quad 0x3ff7f00000000000
                    .quad 0x3ff7e00000000000
                    .quad 0x3ff7d00000000000
                    .quad 0x3ff7b00000000000
                    .quad 0x3ff7a00000000000
                    .quad 0x3ff7900000000000
                    .quad 0x3ff7800000000000
                    .quad 0x3ff7700000000000
                    .quad 0x3ff7600000000000
                    .quad 0x3ff7500000000000
                    .quad 0x3ff7400000000000
                    .quad 0x3ff7300000000000
                    .quad 0x3ff7200000000000
                    .quad 0x3ff7100000000000
                    .quad 0x3ff7000000000000
                    .quad 0x3ff6f00000000000
                    .quad 0x3ff6e00000000000
                    .quad 0x3ff6d00000000000
                    .quad 0x3ff6c00000000000
                    .quad 0x3ff6b00000000000
                    .quad 0x3ff6a00000000000
                    .quad 0x3ff6900000000000
                    .quad 0x3ff6800000000000
                    .quad 0x3ff6700000000000
                    .quad 0x3ff6600000000000
                    .quad 0x3ff6500000000000
                    .quad 0x3ff6400000000000
                    .quad 0x3ff6300000000000
                    .quad 0x3ff6200000000000
                    .quad 0x3ff6100000000000
                    .quad 0x3ff6000000000000
                    .quad 0x3ff5f00000000000
                    .quad 0x3ff5e00000000000
                    .quad 0x3ff5d00000000000
                    .quad 0x3ff5c00000000000
                    .quad 0x3ff5b00000000000
                    .quad 0x3ff5a00000000000
                    .quad 0x3ff5900000000000
                    .quad 0x3ff5800000000000
                    .quad 0x3ff5800000000000
                    .quad 0x3ff5700000000000
                    .quad 0x3ff5600000000000
                    .quad 0x3ff5500000000000
                    .quad 0x3ff5400000000000
                    .quad 0x3ff5300000000000
                    .quad 0x3ff5200000000000
                    .quad 0x3ff5100000000000
                    .quad 0x3ff5000000000000
                    .quad 0x3ff5000000000000
                    .quad 0x3ff4f00000000000
                    .quad 0x3ff4e00000000000
                    .quad 0x3ff4d00000000000
                    .quad 0x3ff4c00000000000
                    .quad 0x3ff4b00000000000
                    .quad 0x3ff4a00000000000
                    .quad 0x3ff4a00000000000
                    .quad 0x3ff4900000000000
                    .quad 0x3ff4800000000000
                    .quad 0x3ff4700000000000
                    .quad 0x3ff4600000000000
                    .quad 0x3ff4600000000000
                    .quad 0x3ff4500000000000
                    .quad 0x3ff4400000000000
                    .quad 0x3ff4300000000000
                    .quad 0x3ff4200000000000
                    .quad 0x3ff4200000000000
                    .quad 0x3ff4100000000000
                    .quad 0x3ff4000000000000
                    .quad 0x3ff3f00000000000
                    .quad 0x3ff3e00000000000
                    .quad 0x3ff3e00000000000
                    .quad 0x3ff3d00000000000
                    .quad 0x3ff3c00000000000
                    .quad 0x3ff3b00000000000
                    .quad 0x3ff3b00000000000
                    .quad 0x3ff3a00000000000
                    .quad 0x3ff3900000000000
                    .quad 0x3ff3800000000000
                    .quad 0x3ff3800000000000
                    .quad 0x3ff3700000000000
                    .quad 0x3ff3600000000000
                    .quad 0x3ff3500000000000
                    .quad 0x3ff3500000000000
                    .quad 0x3ff3400000000000
                    .quad 0x3ff3300000000000
                    .quad 0x3ff3200000000000
                    .quad 0x3ff3200000000000
                    .quad 0x3ff3100000000000
                    .quad 0x3ff3000000000000
                    .quad 0x3ff3000000000000
                    .quad 0x3ff2f00000000000
                    .quad 0x3ff2e00000000000
                    .quad 0x3ff2e00000000000
                    .quad 0x3ff2d00000000000
                    .quad 0x3ff2c00000000000
                    .quad 0x3ff2b00000000000
                    .quad 0x3ff2b00000000000
                    .quad 0x3ff2a00000000000
                    .quad 0x3ff2900000000000
                    .quad 0x3ff2900000000000
                    .quad 0x3ff2800000000000
                    .quad 0x3ff2700000000000
                    .quad 0x3ff2700000000000
                    .quad 0x3ff2600000000000
                    .quad 0x3ff2500000000000
                    .quad 0x3ff2500000000000
                    .quad 0x3ff2400000000000
                    .quad 0x3ff2300000000000
                    .quad 0x3ff2300000000000
                    .quad 0x3ff2200000000000
                    .quad 0x3ff2100000000000
                    .quad 0x3ff2100000000000
                    .quad 0x3ff2000000000000
                    .quad 0x3ff2000000000000
                    .quad 0x3ff1f00000000000
                    .quad 0x3ff1e00000000000
                    .quad 0x3ff1e00000000000
                    .quad 0x3ff1d00000000000
                    .quad 0x3ff1c00000000000
                    .quad 0x3ff1c00000000000
                    .quad 0x3ff1b00000000000
                    .quad 0x3ff1b00000000000
                    .quad 0x3ff1a00000000000
                    .quad 0x3ff1900000000000
                    .quad 0x3ff1900000000000
                    .quad 0x3ff1800000000000
                    .quad 0x3ff1800000000000
                    .quad 0x3ff1700000000000
                    .quad 0x3ff1600000000000
                    .quad 0x3ff1600000000000
                    .quad 0x3ff1500000000000
                    .quad 0x3ff1500000000000
                    .quad 0x3ff1400000000000
                    .quad 0x3ff1300000000000
                    .quad 0x3ff1300000000000
                    .quad 0x3ff1200000000000
                    .quad 0x3ff1200000000000
                    .quad 0x3ff1100000000000
                    .quad 0x3ff1100000000000
                    .quad 0x3ff1000000000000
                    .quad 0x3ff0f00000000000
                    .quad 0x3ff0f00000000000
                    .quad 0x3ff0e00000000000
                    .quad 0x3ff0e00000000000
                    .quad 0x3ff0d00000000000
                    .quad 0x3ff0d00000000000
                    .quad 0x3ff0c00000000000
                    .quad 0x3ff0c00000000000
                    .quad 0x3ff0b00000000000
                    .quad 0x3ff0a00000000000
                    .quad 0x3ff0a00000000000
                    .quad 0x3ff0900000000000
                    .quad 0x3ff0900000000000
                    .quad 0x3ff0800000000000
                    .quad 0x3ff0800000000000
                    .quad 0x3ff0700000000000
                    .quad 0x3ff0700000000000
                    .quad 0x3ff0600000000000
                    .quad 0x3ff0600000000000
                    .quad 0x3ff0500000000000
                    .quad 0x3ff0500000000000
                    .quad 0x3ff0400000000000
                    .quad 0x3ff0400000000000
                    .quad 0x3ff0300000000000
                    .quad 0x3ff0300000000000
                    .quad 0x3ff0200000000000
                    .quad 0x3ff0200000000000
                    .quad 0x3ff0100000000000
                    .quad 0x3ff0100000000000
                    .quad 0x3ff0000000000000
                    .quad 0x3ff0000000000000

.align 16
__log_F_inv_tail:    .quad 0x0000000000000000
                    .quad 0x3effe01fe01fe020
                    .quad 0x3f1fc07f01fc07f0
                    .quad 0x3f31caa01fa11caa
                    .quad 0x3f3f81f81f81f820
                    .quad 0x3f48856506ddaba6
                    .quad 0x3f5196792909c560
                    .quad 0x3f57d9108c2ad433
                    .quad 0x3f5f07c1f07c1f08
                    .quad 0x3f638ff08b1c03dd
                    .quad 0x3f680f6603d980f6
                    .quad 0x3f6d00f57403d5d0
                    .quad 0x3f331abf0b7672a0
                    .quad 0x3f506a965d43919b
                    .quad 0x3f5ceb240795ceb2
                    .quad 0x3f6522f3b834e67f
                    .quad 0x3f6c3c3c3c3c3c3c
                    .quad 0x3f3e01e01e01e01e
                    .quad 0x3f575b8fe21a291c
                    .quad 0x3f6403b9403b9404
                    .quad 0x3f6cc0ed7303b5cc
                    .quad 0x3f479118f3fc4da2
                    .quad 0x3f5ed952e0b0ce46
                    .quad 0x3f695900eae56404
                    .quad 0x3f3d41d41d41d41d
                    .quad 0x3f5cb28ff16c69ae
                    .quad 0x3f696b1edd80e866
                    .quad 0x3f4372e225fe30d9
                    .quad 0x3f60ad12073615a2
                    .quad 0x3f6cdb2c0397cdb3
                    .quad 0x3f52cc157b864407
                    .quad 0x3f664cb5f7148404
                    .quad 0x3f3c71c71c71c71c
                    .quad 0x3f6129a21a930b84
                    .quad 0x3f6f1e0387f1e038
                    .quad 0x3f5ad4e4ba80709b
                    .quad 0x3f6c0e070381c0e0
                    .quad 0x3f560fba1a362bb0
                    .quad 0x3f6a5713280dee96
                    .quad 0x3f53f59620f9ece9
                    .quad 0x3f69f22983759f23
                    .quad 0x3f5478ac63fc8d5c
                    .quad 0x3f6ad87bb4671656
                    .quad 0x3f578b8efbb8148c
                    .quad 0x3f6d0369d0369d03
                    .quad 0x3f5d212b601b3748
                    .quad 0x3f0b2036406c80d9
                    .quad 0x3f629663b24547d1
                    .quad 0x3f4435e50d79435e
                    .quad 0x3f67d0ff2920bc03
                    .quad 0x3f55c06b15c06b16
                    .quad 0x3f6e3a5f0fd7f954
                    .quad 0x3f61dec0d4c77b03
                    .quad 0x3f473289870ac52e
                    .quad 0x3f6a034da034da03
                    .quad 0x3f5d041da2292856
                    .quad 0x3f3a41a41a41a41a
                    .quad 0x3f68550f8a39409d
                    .quad 0x3f5b4fe5e92c0686
                    .quad 0x3f3a01a01a01a01a
                    .quad 0x3f691d2a2067b23a
                    .quad 0x3f5e7c5dada0b4e5
                    .quad 0x3f468a7725080ce1
                    .quad 0x3f6c49d4aa21b490
                    .quad 0x3f63333333333333
                    .quad 0x3f54bc363b03fccf
                    .quad 0x3f2c9f01970e4f81
                    .quad 0x3f697617c6ef5b25
                    .quad 0x3f6161f9add3c0ca
                    .quad 0x3f5319fe6cb39806
                    .quad 0x3f2f693a1c451ab3
                    .quad 0x3f6a9e240321a9e2
                    .quad 0x3f63831f3831f383
                    .quad 0x3f5949ebc4dcfc1c
                    .quad 0x3f480c6980c6980c
                    .quad 0x3f6f9d00c5fe7403
                    .quad 0x3f69721ed7e75347
                    .quad 0x3f6381ec0313381f
                    .quad 0x3f5b97c2aec12653
                    .quad 0x3f509ef3024ae3ba
                    .quad 0x3f38618618618618
                    .quad 0x3f6e0184f00c2780
                    .quad 0x3f692ef5657dba52
                    .quad 0x3f64940305494030
                    .quad 0x3f60303030303030
                    .quad 0x3f58060180601806
                    .quad 0x3f5017f405fd017f
                    .quad 0x3f412a8ad278e8dd
                    .quad 0x3f17d05f417d05f4
                    .quad 0x3f6d67245c02f7d6
                    .quad 0x3f6a4411c1d986a9
                    .quad 0x3f6754d76c7316df
                    .quad 0x3f649902f149902f
                    .quad 0x3f621023358c1a68
                    .quad 0x3f5f7390d2a6c406
                    .quad 0x3f5b2b0805d5b2b1
                    .quad 0x3f5745d1745d1746
                    .quad 0x3f53c31507fa32c4
                    .quad 0x3f50a1fd1b7af017
                    .quad 0x3f4bc36ce3e0453a
                    .quad 0x3f4702e05c0b8170
                    .quad 0x3f4300b79300b793
                    .quad 0x3f3f76b4337c6cb1
                    .quad 0x3f3a62681c860fb0
                    .quad 0x3f36c16c16c16c17
                    .quad 0x3f3490aa31a3cfc7
                    .quad 0x3f33cd153729043e
                    .quad 0x3f3473a88d0bfd2e
                    .quad 0x3f36816816816817
                    .quad 0x3f39f36016719f36
                    .quad 0x3f3ec6a5122f9016
                    .quad 0x3f427c29da5519cf
                    .quad 0x3f4642c8590b2164
                    .quad 0x3f4ab5c45606f00b
                    .quad 0x3f4fd3b80b11fd3c
                    .quad 0x3f52cda0c6ba4eaa
                    .quad 0x3f56058160581606
                    .quad 0x3f5990d0a4b7ef87
                    .quad 0x3f5d6ee340579d6f
                    .quad 0x3f60cf87d9c54a69
                    .quad 0x3f6310572620ae4c
                    .quad 0x3f65798c8ff522a2
                    .quad 0x3f680ad602b580ad
                    .quad 0x3f6ac3e24799546f
                    .quad 0x3f6da46102b1da46
                    .quad 0x3f15805601580560
                    .quad 0x3f3ed3c506b39a23
                    .quad 0x3f4cbdd3e2970f60
                    .quad 0x3f55555555555555
                    .quad 0x3f5c979aee0bf805
                    .quad 0x3f621291e81fd58e
                    .quad 0x3f65fead500a9580
                    .quad 0x3f6a0fd5c5f02a3a
                    .quad 0x3f6e45c223898adc
                    .quad 0x3f35015015015015
                    .quad 0x3f4c7b16ea64d422
                    .quad 0x3f57829cbc14e5e1
                    .quad 0x3f60877db8589720
                    .quad 0x3f65710e4b5edcea
                    .quad 0x3f6a7dbb4d1fc1c8
                    .quad 0x3f6fad40a57eb503
                    .quad 0x3f43fd6bb00a5140
                    .quad 0x3f54e78ecb419ba9
                    .quad 0x3f600a44029100a4
                    .quad 0x3f65c28f5c28f5c3
                    .quad 0x3f6b9c68b2c0cc4a
                    .quad 0x3f2978feb9f34381
                    .quad 0x3f4ecf163bb6500a
                    .quad 0x3f5be1958b67ebb9
                    .quad 0x3f644e6157dc9a3b
                    .quad 0x3f6acc4baa3f0ddf
                    .quad 0x3f26a4cbcb2a247b
                    .quad 0x3f50505050505050
                    .quad 0x3f5e0b4439959819
                    .quad 0x3f66027f6027f602
                    .quad 0x3f6d1e854b5e0db4
                    .quad 0x3f4165e7254813e2
                    .quad 0x3f576646a9d716ef
                    .quad 0x3f632b48f757ce88
                    .quad 0x3f6ac1b24652a906
                    .quad 0x3f33b13b13b13b14
                    .quad 0x3f5490e1eb208984
                    .quad 0x3f62385830fec66e
                    .quad 0x3f6a45a6cc111b7e
                    .quad 0x3f33813813813814
                    .quad 0x3f556f472517b708
                    .quad 0x3f631be7bc0e8f2a
                    .quad 0x3f6b9cbf3e55f044
                    .quad 0x3f40e7d95bc609a9
                    .quad 0x3f59e6b3804d19e7
                    .quad 0x3f65c8b6af7963c2
                    .quad 0x3f6eb9dad43bf402
                    .quad 0x3f4f1a515885fb37
                    .quad 0x3f60eeb1d3d76c02
                    .quad 0x3f6a320261a32026
                    .quad 0x3f3c82ac40260390
                    .quad 0x3f5a12f684bda12f
                    .quad 0x3f669d43fda2962c
                    .quad 0x3f02e025c04b8097
                    .quad 0x3f542804b542804b
                    .quad 0x3f63f69b02593f6a
                    .quad 0x3f6df31cb46e21fa
                    .quad 0x3f5012b404ad012b
                    .quad 0x3f623925e7820a7f
                    .quad 0x3f6c8253c8253c82
                    .quad 0x3f4b92ddc02526e5
                    .quad 0x3f61602511602511
                    .quad 0x3f6bf471439c9adf
                    .quad 0x3f4a85c40939a85c
                    .quad 0x3f6166f9ac024d16
                    .quad 0x3f6c44e10125e227
                    .quad 0x3f4cebf48bbd90e5
                    .quad 0x3f62492492492492
                    .quad 0x3f6d6f2e2ec0b673
                    .quad 0x3f5159e26af37c05
                    .quad 0x3f64024540245402
                    .quad 0x3f6f6f0243f6f024
                    .quad 0x3f55e60121579805
                    .quad 0x3f668e18cf81b10f
                    .quad 0x3f32012012012012
                    .quad 0x3f5c11f7047dc11f
                    .quad 0x3f69e878ff70985e
                    .quad 0x3f4779d9fdc3a219
                    .quad 0x3f61eace5c957907
                    .quad 0x3f6e0d5b450239e1
                    .quad 0x3f548bf073816367
                    .quad 0x3f6694808dda5202
                    .quad 0x3f37c67f2bae2b21
                    .quad 0x3f5ee58469ee5847
                    .quad 0x3f6c0233c0233c02
                    .quad 0x3f514e02328a7012
                    .quad 0x3f6561072057b573
                    .quad 0x3f31811811811812
                    .quad 0x3f5e28646f5a1060
                    .quad 0x3f6c0d1284e6f1d7
                    .quad 0x3f523543f0c80459
                    .quad 0x3f663cbeea4e1a09
                    .quad 0x3f3b9a3fdd5c8cb8
                    .quad 0x3f60be1c159a76d2
                    .quad 0x3f6e1d1a688e4838
                    .quad 0x3f572044d72044d7
                    .quad 0x3f691713db81577b
                    .quad 0x3f4ac73ae9819b50
                    .quad 0x3f6460334e904cf6
                    .quad 0x3f31111111111111
                    .quad 0x3f5feef80441fef0
                    .quad 0x3f6de021fde021fe
                    .quad 0x3f57b7eacc9686a0
                    .quad 0x3f69ead7cd391fbc
                    .quad 0x3f50195609804390
                    .quad 0x3f6641511e8d2b32
                    .quad 0x3f4222b1acf1ce96
                    .quad 0x3f62e29f79b47582
                    .quad 0x3f24f0d1682e11cd
                    .quad 0x3f5f9bb096771e4d
                    .quad 0x3f6e5ee45dd96ae2
                    .quad 0x3f5a0429a0429a04
                    .quad 0x3f6bb74d5f06c021
                    .quad 0x3f54fce404254fce
                    .quad 0x3f695766eacbc402
                    .quad 0x3f50842108421084
                    .quad 0x3f673e5371d5c338
                    .quad 0x3f4930523fbe3368
                    .quad 0x3f656b38f225f6c4
                    .quad 0x3f426e978d4fdf3b
                    .quad 0x3f63dd40e4eb0cc6
                    .quad 0x3f397f7d73404146
                    .quad 0x3f6293982cc98af1
                    .quad 0x3f30410410410410
                    .quad 0x3f618d6f048ff7e4
                    .quad 0x3f2236a3ebc349de
                    .quad 0x3f60c9f8ee53d18c
                    .quad 0x3f10204081020408
                    .quad 0x3f60486ca2f46ea6
                    .quad 0x3ef0101010101010
                    .quad 0x3f60080402010080
                    .quad 0x0000000000000000

//---------------------
// exp data
//---------------------

.align 16

__denormal_threshold:            .long 0x0fffffc02 // -1022
                                .long 0
                                .quad 0

__enable_almost_inf:             .quad 0x7fe0000000000000
                                .quad 0

__real_zero:                     .quad 0x0000000000000000
                                .quad 0

__real_smallest_denormal:        .quad 0x0000000000000001
                                .quad 0
__denormal_tiny_threshold:       .quad 0x0c0874046dfefd9d0
                                .quad 0

__real_p65536:                   .quad 0x40f0000000000000    // 65536
                                .quad 0
__real_m68800:                   .quad 0x0c0f0cc0000000000   // -68800
                                .quad 0
__real_64_by_log2:               .quad 0x40571547652b82fe    // 64/ln(2)
                                .quad 0
__real_log2_by_64_head:          .quad 0x3f862e42f0000000    // log2_by_64_head
                                .quad 0
__real_log2_by_64_tail:          .quad 0x0bdfdf473de6af278   // -log2_by_64_tail
                                .quad 0
__real_1_by_720:                 .quad 0x3f56c16c16c16c17    // 1/720
                                .quad 0
__real_1_by_120:                 .quad 0x3f81111111111111    // 1/120
                                .quad 0
__real_1_by_24:                  .quad 0x3fa5555555555555    // 1/24
                                .quad 0
__real_1_by_6:                   .quad 0x3fc5555555555555    // 1/6
                                .quad 0
__real_1_by_2:                   .quad 0x3fe0000000000000    // 1/2
                                .quad 0


//EXTRN __two_to_jby64_head_table:QWORD
//EXTRN __two_to_jby64_tail_table:QWORD
//EXTRN __use_fma3_lib:DWORD

#define fname pow
#define fname_special _pow_special

// define local variable storage offsets

#define save_x      0x10
#define save_y      0x20
#define p_temp_exp      0x30
#define negate_result      0x40
#define save_ax      0x50
#define y_head      0x60
#define p_temp_log      0x70
#define save_xmm6      0x080
#define save_xmm7      0x090
#define dummy_space      0x0a0

#define stack_size      0x0c8

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
    SaveXmm xmm6, save_xmm6
    SaveXmm xmm7, save_xmm7
    .seh_endprologue
    cmp          DWORD PTR __use_fma3_lib[rip], 0
    jne          Lpow_fma3

.align 16
Lpow_sse2:
    movsd       QWORD PTR [save_x+rsp], xmm0
    movsd       QWORD PTR [save_y+rsp], xmm1

    mov         rdx, QWORD PTR [save_x+rsp]
    mov         r8, QWORD PTR [save_y+rsp]

    mov         r10, QWORD PTR __exp_mant_mask[rip]
    and         r10, r8
    jz          Lpow_sse2_y_is_zero

    cmp         r8, QWORD PTR __pos_one[rip]
    je          Lpow_sse2_y_is_one

    mov         r9, QWORD PTR __sign_mask[rip]
    and         r9, rdx
    mov         rax, QWORD PTR __pos_zero[rip]
    mov         QWORD PTR [negate_result+rsp], rax    
    cmp         r9, QWORD PTR __sign_mask[rip]
    je          Lpow_sse2_x_is_neg

    cmp         rdx, QWORD PTR __pos_one[rip]
    je          Lpow_sse2_x_is_pos_one

    cmp         rdx, QWORD PTR __pos_zero[rip]
    je          Lpow_sse2_x_is_zero

    mov         r9, QWORD PTR __exp_mask[rip]
    and         r9, rdx
    cmp         r9, QWORD PTR __exp_mask[rip]
    je          Lpow_sse2_x_is_inf_or_nan
   
    mov         r10, QWORD PTR __exp_mask[rip]
    and         r10, r8
    cmp         r10, QWORD PTR __ay_max_bound[rip]
    jg          Lpow_sse2_ay_is_very_large

    mov         r10, QWORD PTR __exp_mask[rip]
    and         r10, r8
    cmp         r10, QWORD PTR __ay_min_bound[rip]
    jl          Lpow_sse2_ay_is_very_small

    // -----------------------------
    // compute log(x) here
    // -----------------------------
Lpow_sse2_log_x:

    // compute exponent part
    xor         r8, r8
    movdqa      xmm3, xmm0
    psrlq       xmm3, 52
    movd        r8, xmm0
    psubq       xmm3, XMMWORD PTR __mask_1023[rip]
    movdqa      xmm2, xmm0
    cvtdq2pd    xmm6, xmm3 // xexp
    pand        xmm2, XMMWORD PTR __real_mant[rip]

    comisd      xmm6, QWORD PTR __mask_1023_f[rip]
    je          Lpow_sse2_denormal_adjust

Lpow_sse2_continue_common:

    // compute index into the log tables
    movsd       xmm7, xmm0
    mov         r9, r8
    and         r8, QWORD PTR __mask_mant_all8[rip]
    and         r9, QWORD PTR __mask_mant9[rip]
    subsd       xmm7, __real_one[rip]
    shl         r9, 1
    add         r8, r9
    mov         QWORD PTR [p_temp_log+rsp], r8
    andpd       xmm7, __real_notsign[rip]

    // F, Y, switch to near-one codepath
    movsd       xmm1, QWORD PTR [p_temp_log+rsp]
    shr         r8, 44
    por         xmm2, XMMWORD PTR __real_half[rip]
    por         xmm1, XMMWORD PTR __real_half[rip]
    lea         r9, QWORD PTR __log_F_inv_head[rip]
    lea         rdx, QWORD PTR __log_F_inv_tail[rip]
    comisd      xmm7, __real_threshold[rip]
    jb          Lpow_sse2_near_one

    // f = F - Y, r = f * inv
    subsd       xmm1, xmm2
    movsd       xmm4, xmm1
    mulsd       xmm1, QWORD PTR [r9+r8*8]
    movsd       xmm5, xmm1
    mulsd       xmm4, QWORD PTR [rdx+r8*8]
    movsd       xmm7, xmm4
    addsd       xmm1, xmm4

    movsd       xmm2, xmm1
    movsd       xmm0, xmm1
    lea         r9, __log_256_lead[rip]

    // poly
    movsd       xmm3, QWORD PTR __real_1_over_6[rip]
    movsd       xmm1, QWORD PTR __real_1_over_3[rip]
    mulsd       xmm3, xmm2                         
    mulsd       xmm1, xmm2                         
    mulsd       xmm0, xmm2                         
    subsd       xmm5, xmm2
    movsd       xmm4, xmm0
    addsd       xmm3, QWORD PTR __real_1_over_5[rip]
    addsd       xmm1, QWORD PTR __real_1_over_2[rip]
    mulsd       xmm4, xmm0                         
    mulsd       xmm3, xmm2                         
    mulsd       xmm1, xmm0                         
    addsd       xmm3, QWORD PTR __real_1_over_4[rip]
    addsd       xmm7, xmm5
    mulsd       xmm3, xmm4                         
    addsd       xmm1, xmm3                         
    addsd       xmm1, xmm7

    movsd       xmm5, QWORD PTR __real_log2_tail[rip]
    lea         rdx, __log_256_tail[rip]
    mulsd       xmm5, xmm6
    movsd       xmm0, QWORD PTR [r9+r8*8]
    subsd       xmm5, xmm1

    movsd       xmm3, QWORD PTR [rdx+r8*8]
    addsd       xmm3, xmm5
    movsd       xmm1, xmm3
    subsd       xmm3, xmm2

    movsd       xmm7, QWORD PTR __real_log2_lead[rip]
    mulsd       xmm7, xmm6
    addsd       xmm0, xmm7

    // result of ln(x) is computed from head and tail parts, resH and resT
    // res = ln(x) = resH + resT
    // resH and resT are in full precision 

    // resT is computed from head and tail parts, resT_h and resT_t
    // resT = resT_h + resT_t

    // now
    // xmm3 - resT
    // xmm0 - resH
    // xmm1 - (resT_t)
    // xmm2 - (-resT_h)

Lpow_sse2_log_x_continue:

    movsd       xmm7, xmm0
    addsd       xmm0, xmm3
    movsd       xmm5, xmm0
    andpd       xmm0, XMMWORD PTR __real_fffffffff8000000[rip]
   
    // xmm0 - H
    // xmm7 - resH
    // xmm5 - res

    mov         rax, QWORD PTR [save_y+rsp]
    and         rax, QWORD PTR __real_fffffffff8000000[rip]

    addsd       xmm2, xmm3
    subsd       xmm7, xmm5
    subsd       xmm1, xmm2
    addsd       xmm7, xmm3
    subsd       xmm5, xmm0

    mov         QWORD PTR [y_head+rsp], rax
    movsd       xmm4, QWORD PTR [save_y+rsp]
   
    addsd       xmm7, xmm1 
    addsd       xmm7, xmm5

    // res = H + T
    // H has leading 26 bits of precision
    // T has full precision

    // xmm0 - H
    // xmm7 - T

    movsd       xmm2, QWORD PTR [y_head+rsp] 
    subsd       xmm4, xmm2

    // y is split into head and tail
    // for y * ln(x) computation

    // xmm4 - Yt
    // xmm2 - Yh
    // xmm0 - H
    // xmm7 - T

    movsd   xmm3, xmm4
    movsd   xmm5, xmm7
    movsd   xmm6, xmm0
    mulsd   xmm3, xmm7 // YtRt
    mulsd   xmm4, xmm0 // YtRh
    mulsd   xmm5, xmm2 // YhRt
    mulsd   xmm6, xmm2 // YhRh 

    movsd   xmm1, xmm6
    addsd   xmm3, xmm4
    addsd   xmm3, xmm5

    addsd   xmm1, xmm3
    movsd   xmm0, xmm1

    subsd   xmm6, xmm1
    addsd   xmm6, xmm3 

    // y * ln(x) = v + vt
    // v and vt are in full precision 
 
    // xmm0 - v
    // xmm6 - vt

    // -----------------------------
    // compute exp( y * ln(x) ) here
    // -----------------------------

    // v * (64/ln(2))
    movsd       xmm7, QWORD PTR __real_64_by_log2[rip]
    movsd       QWORD PTR [p_temp_exp+rsp], xmm0
    mulsd       xmm7, xmm0
    mov         rdx, QWORD PTR [p_temp_exp+rsp]

    // v < 1024*ln(2), ( v * (64/ln(2)) ) < 64*1024
    // v >= -1075*ln(2), ( v * (64/ln(2)) ) >= 64*(-1075)
    comisd      xmm7, QWORD PTR __real_p65536[rip]
    ja          Lpow_sse2_process_result_inf

    comisd      xmm7, QWORD PTR __real_m68800[rip]
    jb          Lpow_sse2_process_result_zero

    // n = int( v * (64/ln(2)) )
    cvtpd2dq    xmm4, xmm7
    lea         r10, __two_to_jby64_head_table[rip]
    lea         r11, __two_to_jby64_tail_table[rip]
    cvtdq2pd    xmm1, xmm4

    // r1 = x - n * ln(2)/64 head
    movsd       xmm2, QWORD PTR __real_log2_by_64_head[rip]
    mulsd       xmm2, xmm1
    movd        ecx, xmm4
    mov         rax, 0x3f
    and         eax, ecx
    subsd       xmm0, xmm2

    // r2 = - n * ln(2)/64 tail
    mulsd       xmm1, QWORD PTR __real_log2_by_64_tail[rip]
    movsd       xmm2, xmm0

    // m = (n - j) / 64
    sub         ecx, eax
    sar         ecx, 6

    // r1+r2
    addsd       xmm2, xmm1
    addsd       xmm2, xmm6 // add vt here
    movsd       xmm1, xmm2

    // q
    movsd       xmm0, QWORD PTR __real_1_by_2[rip]
    movsd       xmm3, QWORD PTR __real_1_by_24[rip]
    movsd       xmm4, QWORD PTR __real_1_by_720[rip]
    mulsd       xmm1, xmm2
    mulsd       xmm0, xmm2
    mulsd       xmm3, xmm2
    mulsd       xmm4, xmm2

    movsd       xmm5, xmm1
    mulsd       xmm1, xmm2
    addsd       xmm0, QWORD PTR __real_one[rip]
    addsd       xmm3, QWORD PTR __real_1_by_6[rip]
    mulsd       xmm5, xmm1
    addsd       xmm4, QWORD PTR __real_1_by_120[rip]
    mulsd       xmm0, xmm2
    mulsd       xmm3, xmm1
 
    mulsd       xmm4, xmm5

    // deal with denormal results
    xor         r9d, r9d

    addsd       xmm3, xmm4
    addsd       xmm0, xmm3

    cmp         ecx, DWORD PTR __denormal_threshold[rip]
    cmovle      r9d, ecx
    add         rcx, 1023
    shl         rcx, 52

    // f1, f2
    movsd       xmm5, QWORD PTR [r11+rax*8]
    movsd       xmm1, QWORD PTR [r10+rax*8]
    mulsd       xmm5, xmm0
    mulsd       xmm1, xmm0


    // (f1+f2)*(1+q)
    addsd       xmm5, QWORD PTR [r11+rax*8]
    addsd       xmm1, xmm5
    addsd       xmm1, QWORD PTR [r10+rax*8]
    movsd       xmm0, xmm1

    cmp         rcx, QWORD PTR __real_inf[rip]
    je          Lpow_sse2_process_almost_inf

    mov         QWORD PTR [p_temp_exp+rsp], rcx
    test        r9d, r9d
    jnz         Lpow_sse2_process_denormal
    mulsd       xmm0, QWORD PTR [p_temp_exp+rsp]
    orpd        xmm0, XMMWORD PTR [negate_result+rsp]

Lpow_sse2_final_check:
    RestoreXmm   xmm7, save_xmm7
    RestoreXmm   xmm6, save_xmm6
    StackDeallocate stack_size
    ret

.align 16
Lpow_sse2_process_almost_inf:
    comisd      xmm0, QWORD PTR __real_one[rip]
    jae         Lpow_sse2_process_result_inf

    orpd        xmm0, XMMWORD PTR __enable_almost_inf[rip]
    orpd        xmm0, XMMWORD PTR [negate_result+rsp]
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_process_denormal:
    mov         ecx, r9d
    xor         r11d, r11d
    comisd      xmm0, QWORD PTR __real_one[rip]
    cmovae      r11d, ecx
    cmp         r11d, DWORD PTR __denormal_threshold[rip]
    jne         Lpow_sse2_process_true_denormal  

    mulsd       xmm0, QWORD PTR [p_temp_exp+rsp]
    orpd        xmm0, XMMWORD PTR [negate_result+rsp]
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_process_true_denormal:
    xor         r8, r8
    mov         r9, 1
    cmp         rdx, QWORD PTR __denormal_tiny_threshold[rip]
    jg          Lpow_sse2_process_denormal_tiny
    add         ecx, 1074
    cmovs       rcx, r8
    shl         r9, cl
    mov         rcx, r9

    mov         QWORD PTR [p_temp_exp+rsp], rcx
    mulsd       xmm0, QWORD PTR [p_temp_exp+rsp]
    orpd        xmm0, XMMWORD PTR [negate_result+rsp]
    jmp         Lpow_sse2_z_denormal        

.align 16
Lpow_sse2_process_denormal_tiny:
    movsd       xmm0, QWORD PTR __real_smallest_denormal[rip]
    orpd        xmm0, XMMWORD PTR [negate_result+rsp]
    jmp         Lpow_sse2_z_denormal

.align 16
Lpow_sse2_process_result_zero:
    mov         r11, QWORD PTR __real_zero[rip]
    or          r11, QWORD PTR [negate_result+rsp]
    jmp         Lpow_sse2_z_is_zero_or_inf
 
.align 16
Lpow_sse2_process_result_inf:
    mov         r11, QWORD PTR __real_inf[rip]
    or          r11, QWORD PTR [negate_result+rsp]
    jmp         Lpow_sse2_z_is_zero_or_inf

.align 16
Lpow_sse2_denormal_adjust:
    por         xmm2, XMMWORD PTR __real_one[rip]
    subsd       xmm2, QWORD PTR __real_one[rip]
    movsd       xmm5, xmm2
    pand        xmm2, XMMWORD PTR __real_mant[rip]
    movd        r8, xmm2
    psrlq       xmm5, 52
    psubd       xmm5, XMMWORD PTR __mask_2045[rip]
    cvtdq2pd    xmm6, xmm5
    jmp         Lpow_sse2_continue_common

.align 16
Lpow_sse2_x_is_neg:

    mov         r10, QWORD PTR __exp_mask[rip]
    and         r10, r8
    cmp         r10, QWORD PTR __ay_max_bound[rip]
    jg          Lpow_sse2_ay_is_very_large

    // determine if y is an integer
    mov         r10, QWORD PTR __exp_mant_mask[rip]
    and         r10, r8
    mov         r11, r10
    mov         rcx, QWORD PTR __exp_shift[rip]
    shr         r10, cl
    sub         r10, QWORD PTR __exp_bias[rip]
    js          Lpow_sse2_x_is_neg_y_is_not_int
   
    mov         rax, QWORD PTR __exp_mant_mask[rip]
    and         rax, rdx
    mov         QWORD PTR [save_ax+rsp], rax

    mov         rcx, r10
    cmp         r10, QWORD PTR __yexp_53[rip]
    jg          Lpow_sse2_continue_after_y_int_check

    mov         r9, QWORD PTR __mant_full[rip]
    shr         r9, cl
    and         r9, r11
    jnz         Lpow_sse2_x_is_neg_y_is_not_int

    mov         r9, QWORD PTR __1_before_mant[rip]
    shr         r9, cl
    and         r9, r11
    jz          Lpow_sse2_continue_after_y_int_check

    mov         rax, QWORD PTR __sign_mask[rip]
    mov         QWORD PTR [negate_result+rsp], rax    

Lpow_sse2_continue_after_y_int_check:

    cmp         rdx, QWORD PTR __neg_zero[rip]
    je          Lpow_sse2_x_is_zero

    cmp         rdx, QWORD PTR __neg_one[rip]
    je          Lpow_sse2_x_is_neg_one

    mov         r9, QWORD PTR __exp_mask[rip]
    and         r9, rdx
    cmp         r9, QWORD PTR __exp_mask[rip]
    je          Lpow_sse2_x_is_inf_or_nan
   
    movsd       xmm0, QWORD PTR [save_ax+rsp]
    jmp         Lpow_sse2_log_x


.align 16
Lpow_sse2_near_one:

    // f = F - Y, r = f * inv
    movsd       xmm0, xmm1
    subsd       xmm1, xmm2
    movsd       xmm4, xmm1

    movsd       xmm3, QWORD PTR [r9+r8*8]
    addsd       xmm3, QWORD PTR [rdx+r8*8]
    mulsd       xmm4, xmm3
    andpd       xmm4, XMMWORD PTR __real_fffffffff8000000[rip]
    movsd       xmm5, xmm4 // r1
    mulsd       xmm4, xmm0
    subsd       xmm1, xmm4
    mulsd       xmm1, xmm3
    movsd       xmm7, xmm1 // r2
    addsd       xmm1, xmm5

    movsd       xmm2, xmm1
    movsd       xmm0, xmm1

    lea         r9, __log_256_lead[rip]

    // poly
    movsd       xmm3, QWORD PTR __real_1_over_7[rip]
    movsd       xmm1, QWORD PTR __real_1_over_4[rip]
    mulsd       xmm3, xmm2
    mulsd       xmm1, xmm2
    mulsd       xmm0, xmm2
    movsd       xmm4, xmm0
    addsd       xmm3, QWORD PTR __real_1_over_6[rip]
    addsd       xmm1, QWORD PTR __real_1_over_3[rip]
    mulsd       xmm4, xmm0
    mulsd       xmm3, xmm2
    mulsd       xmm1, xmm2
    addsd       xmm3, QWORD PTR __real_1_over_5[rip]
    mulsd       xmm3, xmm2
    mulsd       xmm1, xmm0
    mulsd       xmm3, xmm4

    movsd       xmm2, xmm5
    movsd       xmm0, xmm7
    mulsd       xmm0, xmm0
    mulsd       xmm0, QWORD PTR __real_1_over_2[rip]
    mulsd       xmm5, xmm7
    addsd       xmm5, xmm0
    addsd       xmm5, xmm7

    movsd       xmm0, xmm2
    movsd       xmm7, xmm2
    mulsd       xmm0, xmm0
    mulsd       xmm0, QWORD PTR __real_1_over_2[rip]
    movsd       xmm4, xmm0
    addsd       xmm2, xmm0 // r1 + r1^2/2
    subsd       xmm7, xmm2
    addsd       xmm7, xmm4

    addsd       xmm3, xmm7
    movsd       xmm4, QWORD PTR __real_log2_tail[rip]
    addsd       xmm1, xmm3
    mulsd       xmm4, xmm6
    lea         rdx, __log_256_tail[rip]
    addsd       xmm1, xmm5
    addsd       xmm4, QWORD PTR [rdx+r8*8]
    subsd       xmm4, xmm1

    movsd       xmm3, xmm4
    movsd       xmm1, xmm4
    subsd       xmm3, xmm2

    movsd       xmm0, QWORD PTR [r9+r8*8]
    movsd       xmm7, QWORD PTR __real_log2_lead[rip]
    mulsd       xmm7, xmm6
    addsd       xmm0, xmm7

    jmp         Lpow_sse2_log_x_continue


.align 16
Lpow_sse2_x_is_pos_one:
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_y_is_zero:
    movsd       xmm0, QWORD PTR __real_one[rip]
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_y_is_one:
    xor         rax, rax
    mov         r11, rdx
    mov         r9, QWORD PTR __exp_mask[rip]
    //or          r11, QWORD PTR __qnan_set
    and         r9, rdx
    cmp         r9, QWORD PTR __exp_mask[rip]
    cmove       rax, rdx
    mov         r9, QWORD PTR __mant_mask[rip]
    and         r9, rax
    jnz         Lpow_sse2_x_is_nan

    movd        xmm0, rdx 
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_x_is_neg_one:
    mov         rdx, QWORD PTR __pos_one[rip]
    or          rdx, QWORD PTR [negate_result+rsp]
    xor         rax, rax
    mov         r11, r8
    mov         r10, QWORD PTR __exp_mask[rip]
    //or          r11, QWORD PTR __qnan_set
    and         r10, r8
    cmp         r10, QWORD PTR __exp_mask[rip]
    cmove       rax, r8
    mov         r10, QWORD PTR __mant_mask[rip]
    and         r10, rax
    jnz         Lpow_sse2_y_is_nan

    movd        xmm0, rdx
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_x_is_neg_y_is_not_int:
    mov         r9, QWORD PTR __exp_mask[rip]
    and         r9, rdx
    cmp         r9, QWORD PTR __exp_mask[rip]
    je          Lpow_sse2_x_is_inf_or_nan

    cmp         rdx, QWORD PTR __neg_zero[rip]
    je          Lpow_sse2_x_is_zero

    movsd       xmm0, QWORD PTR [save_x+rsp]
    movsd       xmm1, QWORD PTR [save_y+rsp]
    movsd       xmm2, QWORD PTR __neg_qnan[rip]
    mov         r9d, DWORD PTR __flag_x_neg_y_notint[rip]

    call        fname_special
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_ay_is_very_large:
    mov         r9, QWORD PTR __exp_mask[rip]
    and         r9, rdx
    cmp         r9, QWORD PTR __exp_mask[rip]
    je          Lpow_sse2_x_is_inf_or_nan

    mov         r9, QWORD PTR __exp_mant_mask[rip]
    and         r9, rdx
    jz          Lpow_sse2_x_is_zero 

    cmp         rdx, QWORD PTR __neg_one[rip]
    je          Lpow_sse2_x_is_neg_one

    mov         r9, rdx
    and         r9, QWORD PTR __exp_mant_mask[rip]
    cmp         r9, QWORD PTR __pos_one[rip]
    jl          Lpow_sse2_ax_lt1_y_is_large_or_inf_or_nan
  
    jmp         Lpow_sse2_ax_gt1_y_is_large_or_inf_or_nan

.align 16
Lpow_sse2_x_is_zero:
    mov         r10, QWORD PTR __exp_mask[rip]
    xor         rax, rax
    and         r10, r8
    cmp         r10, QWORD PTR __exp_mask[rip]
    je          Lpow_sse2_x_is_zero_y_is_inf_or_nan

    mov         r10, QWORD PTR __sign_mask[rip]
    and         r10, r8
    cmovnz      rax, QWORD PTR __pos_inf[rip]
    jnz         Lpow_sse2_x_is_zero_z_is_inf

    movd        xmm0, rax
    orpd        xmm0, XMMWORD PTR [negate_result+rsp]
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_x_is_zero_z_is_inf:

    movsd       xmm0, QWORD PTR [save_x+rsp]
    movsd       xmm1, QWORD PTR [save_y+rsp]
    movd        xmm2, rax
    orpd        xmm2, XMMWORD PTR [negate_result+rsp]
    mov         r9d, DWORD PTR __flag_x_zero_z_inf[rip]

    call        fname_special
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_x_is_zero_y_is_inf_or_nan:
    mov         r11, r8
    cmp         r8, QWORD PTR __neg_inf[rip]
    cmove       rax, QWORD PTR __pos_inf[rip]
    je          Lpow_sse2_x_is_zero_z_is_inf

    //or          r11, QWORD PTR __qnan_set
    mov         r10, QWORD PTR __mant_mask[rip]
    and         r10, r8
    jnz         Lpow_sse2_y_is_nan

    movd        xmm0, rax
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_x_is_inf_or_nan:
    xor         r11, r11
    mov         r10, QWORD PTR __sign_mask[rip]
    and         r10, r8
    cmovz       r11, QWORD PTR __pos_inf[rip]
    mov         rax, rdx
    mov         r9, QWORD PTR __mant_mask[rip]
    //or          rax, QWORD PTR __qnan_set
    and         r9, rdx
    cmovnz      r11, rax
    jnz         Lpow_sse2_x_is_nan

    xor         rax, rax
    mov         r9, r8
    mov         r10, QWORD PTR __exp_mask[rip]
    //or          r9, QWORD PTR __qnan_set
    and         r10, r8
    cmp         r10, QWORD PTR __exp_mask[rip]
    cmove       rax, r8
    mov         r10, QWORD PTR __mant_mask[rip]
    and         r10, rax
    cmovnz      r11, r9
    jnz         Lpow_sse2_y_is_nan

    movd        xmm0, r11
    orpd        xmm0, XMMWORD PTR [negate_result+rsp]
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_ay_is_very_small:
    movsd       xmm0, QWORD PTR __pos_one[rip]
    addsd       xmm0, xmm1
    jmp         Lpow_sse2_final_check


.align 16
Lpow_sse2_ax_lt1_y_is_large_or_inf_or_nan:
    xor         r11, r11
    mov         r10, QWORD PTR __sign_mask[rip]
    and         r10, r8
    cmovnz      r11, QWORD PTR __pos_inf[rip]
    jmp         Lpow_sse2_adjust_for_nan

.align 16
Lpow_sse2_ax_gt1_y_is_large_or_inf_or_nan:
    xor         r11, r11
    mov         r10, QWORD PTR __sign_mask[rip]
    and         r10, r8
    cmovz       r11, QWORD PTR __pos_inf[rip]

.align 16
Lpow_sse2_adjust_for_nan:

    xor         rax, rax
    mov         r9, r8
    mov         r10, QWORD PTR __exp_mask[rip]
    //or          r9, QWORD PTR __qnan_set
    and         r10, r8
    cmp         r10, QWORD PTR __exp_mask[rip]
    cmove       rax, r8
    mov         r10, QWORD PTR __mant_mask[rip]
    and         r10, rax
    cmovnz      r11, r9
    jnz         Lpow_sse2_y_is_nan

    test        rax, rax
    jnz         Lpow_sse2_y_is_inf

.align 16
Lpow_sse2_z_is_zero_or_inf:

    mov         r9d, DWORD PTR __flag_z_zero[rip]
    test        r11, QWORD PTR __exp_mant_mask[rip]
    cmovnz      r9d, DWORD PTR __flag_z_inf[rip]
    
    movsd       xmm0, QWORD PTR [save_x+rsp]
    movsd       xmm1, QWORD PTR [save_y+rsp]
    movd        xmm2, r11

    call        fname_special
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_y_is_inf:

    movd        xmm0, r11
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_x_is_nan:

    xor         rax, rax
    mov         r10, QWORD PTR __exp_mask[rip]
    and         r10, r8
    cmp         r10, QWORD PTR __exp_mask[rip]
    cmove       rax, r8
    mov         r10, QWORD PTR __mant_mask[rip]
    and         r10, rax
    jnz         Lpow_sse2_x_is_nan_y_is_nan

    movsd       xmm0, QWORD PTR [save_x+rsp]
    movsd       xmm1, QWORD PTR [save_y+rsp]
    movd        xmm2, r11
    mov         r9d, DWORD PTR __flag_x_nan[rip]

    call        fname_special
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_y_is_nan:

    movsd       xmm0, QWORD PTR [save_x+rsp]
    movsd       xmm1, QWORD PTR [save_y+rsp]
    movd        xmm2, r11
    mov         r9d, DWORD PTR __flag_y_nan[rip]

    call        fname_special
    jmp         Lpow_sse2_final_check

.align 16
Lpow_sse2_x_is_nan_y_is_nan:

    mov         r9, r8
    
    cmp         r11, QWORD PTR __ind_pattern[rip]
    cmove       r11, r9
    je          Lpow_sse2_continue_xy_nan

    cmp         r9, QWORD PTR __ind_pattern[rip]
    cmove       r9, r11

    mov         r10, r9
    and         r10, QWORD PTR __sign_mask[rip]
    cmovnz      r9, r11

    mov         r10, r11
    and         r10, QWORD PTR __sign_mask[rip]
    cmovnz      r11, r9
    
Lpow_sse2_continue_xy_nan:    
    //or          r11, QWORD PTR __qnan_set
    movsd       xmm0, QWORD PTR [save_x+rsp]
    movsd       xmm1, QWORD PTR [save_y+rsp]
    movd        xmm2, r11
    mov         r9d, DWORD PTR __flag_x_nan_y_nan[rip]

    call        fname_special
    jmp         Lpow_sse2_final_check  
    
.align 16
Lpow_sse2_z_denormal:
    
    movsd       xmm2, xmm0
    movsd       xmm0, QWORD PTR [save_x+rsp]
    movsd       xmm1, QWORD PTR [save_y+rsp]
    mov         r9d, DWORD PTR __flag_z_denormal[rip]

    call        fname_special
    jmp         Lpow_sse2_final_check  

Lpow_fma3:
    vmovsd       QWORD PTR [save_x+rsp], xmm0
    vmovsd       QWORD PTR [save_y+rsp], xmm1

    mov          rdx, QWORD PTR [save_x+rsp]
    mov          r8, QWORD PTR [save_y+rsp]

    mov          r10, QWORD PTR __exp_mant_mask[rip]
    and          r10, r8
    jz           Lpow_fma3_y_is_zero

    cmp          r8, QWORD PTR __pos_one[rip]
    je           Lpow_fma3_y_is_one

    mov          r9, QWORD PTR __sign_mask[rip]
    and          r9, rdx
    cmp          r9, QWORD PTR __sign_mask[rip]
    mov          rax, QWORD PTR __pos_zero[rip]
    mov          QWORD PTR [negate_result+rsp], rax
    je           Lpow_fma3_x_is_neg

    cmp          rdx, QWORD PTR __pos_one[rip]
    je           Lpow_fma3_x_is_pos_one

    cmp          rdx, QWORD PTR __pos_zero[rip]
    je           Lpow_fma3_x_is_zero

    mov          r9, QWORD PTR __exp_mask[rip]
    and          r9, rdx
    cmp          r9, QWORD PTR __exp_mask[rip]
    je           Lpow_fma3_x_is_inf_or_nan

    mov          r10, QWORD PTR __exp_mask[rip]
    and          r10, r8
    cmp          r10, QWORD PTR __ay_max_bound[rip]
    jg           Lpow_fma3_ay_is_very_large

    mov          r10, QWORD PTR __exp_mask[rip]
    and          r10, r8
    cmp          r10, QWORD PTR __ay_min_bound[rip]
    jl           Lpow_fma3_ay_is_very_small

    // -----------------------------
    // compute log(x) here
    // -----------------------------
Lpow_fma3_log_x:

    // compute exponent part
    vpsrlq       xmm3, xmm0, 52
    vmovq        r8, xmm0
    vpsubq       xmm3, xmm3, XMMWORD PTR __mask_1023[rip]
    vcvtdq2pd    xmm6, xmm3 // xexp
    vpand        xmm2, xmm0, XMMWORD PTR __real_mant[rip]

    vcomisd      xmm6, QWORD PTR __mask_1023_f[rip]
    je           Lpow_fma3_denormal_adjust

Lpow_fma3_continue_common:

    // compute index into the log tables
    mov          r9, r8
    and          r8, QWORD PTR __mask_mant_all8[rip]
    and          r9, QWORD PTR __mask_mant9[rip]
    vsubsd       xmm7, xmm0, __real_one[rip]
    shl          r9, 1
    add          r8, r9
    vmovq        xmm1, r8
    vandpd       xmm7, xmm7, __real_notsign[rip]

    // F, Y, switch to near-one codepath
    shr          r8, 44
    vpor         xmm2, xmm2, XMMWORD PTR __real_half[rip]
    vpor         xmm1, xmm1, XMMWORD PTR __real_half[rip]
    vcomisd      xmm7, __real_threshold[rip]
    lea          r9, QWORD PTR __log_F_inv_head[rip]
    lea          rdx, QWORD PTR __log_F_inv_tail[rip]
    jb           Lpow_fma3_near_one

    // f = F - Y, r = f * inv
    vsubsd       xmm4, xmm1, xmm2          // xmm4 <-- f = F - Y
    vmulsd       xmm1, xmm4, QWORD PTR [r9+r8*8] // xmm1 <-- rhead = f*inv_head
    vmovapd      xmm5, xmm1                // xmm5 <-- copy of rhead
    vmulsd       xmm4, xmm4, QWORD PTR [rdx+r8*8] // xmm4 <-- rtail = f*inv_tail
    vmovapd      xmm7, xmm4                // xmm7 <-- copy of rtail
    vaddsd       xmm1, xmm1, xmm4          // xmm1 <-- r = rhead + rtail

    vmovapd      xmm2, xmm1                // xmm2 <-- copy of r
    vmovapd      xmm0, xmm1                // xmm1 <-- copy of r
    lea          r9, __log_256_lead[rip]

    // poly
//    movsd       xmm3, QWORD PTR __real_1_over_6
//    movsd       xmm1, QWORD PTR __real_1_over_3
//    mulsd       xmm3, xmm2               ; r*1/6
//    mulsd       xmm1, xmm2               ; r*1/3
//    mulsd       xmm0, xmm2               ; r^2
//    subsd       xmm5, xmm2               ; xmm5 <-- rhead - r
//    movsd       xmm4, xmm0               ; xmm4 <-- copy of r^2
//    addsd       xmm3, QWORD PTR __real_1_over_5 ; xmm3 <-- r*1/6 + 1/5
//    addsd       xmm1, QWORD PTR __real_1_over_2 ; xmm1 <-- r*1/3 + 1/2
//    mulsd       xmm4, xmm0               ; xmm4 <-- r^4
//    mulsd       xmm3, xmm2               ; xmm3 <-- (r*1/6 + 1/5)*r
//    mulsd       xmm1, xmm0               ; xmm1 <-- (r*1/3 + 1/2)*r^2
//    addsd       xmm3, QWORD PTR __real_1_over_4 ; xmm3 <-- (r*1/6+1/5)*r + 1/4
//    addsd       xmm7, xmm5               ; xmm7 <-- rtail + (rhead - r)
//    mulsd       xmm3, xmm4               ; xmm3 <-- (r*1/6 + 1/5)*r^5 + r^4*1/4
//    addsd       xmm1, xmm3               ; xmm1 <-- poly down to r^2
//    addsd       xmm1, xmm7               ; xmm1 <-- poly + correction


    vsubsd       xmm3, xmm5, xmm2
    vmovsd       xmm1, QWORD PTR __real_1_over_6[rip]
    vmulsd       xmm0,xmm0,xmm0
    vaddsd       xmm3, xmm3, xmm7
    vfmadd213sd  xmm1, xmm2, QWORD PTR __real_1_over_5[rip]
    vfmadd213sd  xmm1, xmm2, QWORD PTR __real_1_over_4[rip]
    vfmadd213sd  xmm1, xmm2, QWORD PTR __real_1_over_3[rip]
    vfmadd213sd  xmm1, xmm2, QWORD PTR __real_1_over_2[rip]
    vfmadd213sd  xmm1, xmm0, xmm3

    vmovsd       xmm5, QWORD PTR __real_log2_tail[rip]
    lea          rdx, __log_256_tail[rip]
    vfmsub213sd  xmm5, xmm6, xmm1
    vmovsd       xmm0, QWORD PTR [r9+r8*8]

    vaddsd       xmm3, xmm5, QWORD PTR [rdx+r8*8]
    vmovapd      xmm1, xmm3
    vsubsd       xmm3, xmm3, xmm2

    vfmadd231sd  xmm0, xmm6, QWORD PTR __real_log2_lead[rip]

    // result of ln(x) is computed from head and tail parts, resH and resT
    // res = ln(x) = resH + resT
    // resH and resT are in full precision

    // resT is computed from head and tail parts, resT_h and resT_t
    // resT = resT_h + resT_t

    // now
    // xmm3 - resT
    // xmm0 - resH
    // xmm1 - (resT_t)
    // xmm2 - (-resT_h)

Lpow_fma3_log_x_continue:

    vmovapd      xmm7, xmm0
    vaddsd       xmm0, xmm0, xmm3
    vmovapd      xmm5, xmm0
    vandpd       xmm0, xmm0, XMMWORD PTR __real_fffffffff8000000[rip]

    // xmm0 - H
    // xmm7 - resH
    // xmm5 - res

    mov          rax, QWORD PTR [save_y+rsp]
    and          rax, QWORD PTR __real_fffffffff8000000[rip]

    vaddsd       xmm2, xmm2, xmm3
    vsubsd       xmm7, xmm7, xmm5
    vsubsd       xmm1, xmm1, xmm2
    vaddsd       xmm7, xmm7, xmm3
    vsubsd       xmm5, xmm5, xmm0

    mov          QWORD PTR [y_head+rsp], rax
    vmovsd       xmm4, QWORD PTR [save_y+rsp]

    vaddsd       xmm7, xmm7, xmm1
    vaddsd       xmm7, xmm7, xmm5

    // res = H + T
    // H has leading 26 bits of precision
    // T has full precision

    // xmm0 - H
    // xmm7 - T

    vmovsd       xmm2, QWORD PTR [y_head+rsp]
    vsubsd       xmm4, xmm4, xmm2

    // y is split into head and tail
    // for y * ln(x) computation

    // xmm4 - Yt
    // xmm2 - Yh
    // xmm0 - H
    // xmm7 - T

    vmulsd       xmm3, xmm4, xmm7 // YtRt
    vmulsd       xmm4, xmm4, xmm0 // YtRh
    vmulsd       xmm5, xmm7, xmm2 // YhRt
    vmulsd       xmm6, xmm0, xmm2 // YhRh

    vmovapd      xmm1, xmm6
    vaddsd       xmm3, xmm3, xmm4
    vaddsd       xmm3, xmm3, xmm5

    vaddsd       xmm1, xmm1, xmm3
    vmovapd      xmm0, xmm1

    vsubsd       xmm6, xmm6, xmm1
    vaddsd       xmm6, xmm6, xmm3

    // y * ln(x) = v + vt
    // v and vt are in full precision

    // xmm0 - v
    // xmm6 - vt

    // -----------------------------
    // compute exp( y * ln(x) ) here
    // -----------------------------

    // v * (64/ln(2))
    vmovsd       QWORD PTR [p_temp_exp+rsp], xmm0
    vmulsd       xmm7, xmm0, QWORD PTR __real_64_by_log2[rip]
    mov          rdx, QWORD PTR [p_temp_exp+rsp]

    // v < 1024*ln(2), ( v * (64/ln(2)) ) < 64*1024
    // v >= -1075*ln(2), ( v * (64/ln(2)) ) >= 64*(-1075)
    vcomisd      xmm7, QWORD PTR __real_p65536[rip]
    ja           Lpow_fma3_process_result_inf

    vcomisd      xmm7, QWORD PTR __real_m68800[rip]
    jb           Lpow_fma3_process_result_zero

    // n = int( v * (64/ln(2)) )
    vcvtpd2dq    xmm4, xmm7
    lea          r10, __two_to_jby64_head_table[rip]
    lea          r11, __two_to_jby64_tail_table[rip]
    vcvtdq2pd    xmm1, xmm4

    // r1 = x - n * ln(2)/64 head
    vfnmadd231sd xmm0, xmm1, QWORD PTR __real_log2_by_64_head[rip]
    vmovd        ecx, xmm4
    mov          rax, 0x3f
    and          eax, ecx

    // r2 = - n * ln(2)/64 tail
    vmulsd       xmm1, xmm1, QWORD PTR __real_log2_by_64_tail[rip]
    vmovapd      xmm2, xmm0

    // m = (n - j) / 64
    sub          ecx, eax
    sar          ecx, 6

    // r1+r2
    vaddsd       xmm2, xmm2, xmm1
    vaddsd       xmm2, xmm2, xmm6 // add vt here
    vmovapd      xmm1, xmm2

    // q
    vmovsd       xmm0, QWORD PTR __real_1_by_720[rip]
    xor         r9d, r9d
    vfmadd213sd  xmm0, xmm2,  QWORD PTR __real_1_by_120[rip]
    cmp         ecx, DWORD PTR __denormal_threshold[rip]
    vfmadd213sd  xmm0, xmm2,  QWORD PTR __real_1_by_24[rip]
    cmovle      r9d, ecx
    vfmadd213sd  xmm0, xmm2,  QWORD PTR __real_1_by_6[rip]
    add         rcx, 1023
    vfmadd213sd  xmm0, xmm2,  QWORD PTR __real_1_by_2[rip]
    shl         rcx, 52
    vfmadd213sd  xmm0, xmm2,  QWORD PTR __real_one[rip]
    vmulsd       xmm0, xmm0, xmm2         // xmm0 <-- q
//    movsd       xmm0, QWORD PTR __real_1_by_2
//    movsd       xmm3, QWORD PTR __real_1_by_24
//    movsd       xmm4, QWORD PTR __real_1_by_720
//    mulsd       xmm1, xmm2                ; xmm1 <-- r^2
//    mulsd       xmm0, xmm2                ; xmm0 <-- r/2
//    mulsd       xmm3, xmm2                ; xmm3 <-- r/24
//    mulsd       xmm4, xmm2                ; xmm4 <-- r/720

//    movsd       xmm5, xmm1                ; xmm5 <-- copy of r^2
//    mulsd       xmm1, xmm2                ; xmm1 <-- r^3
//    addsd       xmm0, QWORD PTR __real_one ; xmm0 <-- r/2 + 1
//    addsd       xmm3, QWORD PTR __real_1_by_6 ; xmm3 <-- r/24 + 1/6
//    mulsd       xmm5, xmm1                ; xmm5 <-- r^5
//    addsd       xmm4, QWORD PTR __real_1_by_120 ; xmm4 <-- r/720 + 1/120
//    mulsd       xmm0, xmm2                ; xmm0 <-- (r/2 + 1)*r
//    mulsd       xmm3, xmm1                ; xmm3 <-- (r/24 + 1/6)*r^3

//    mulsd       xmm4, xmm5                ; xmm4 <-- (r/720 + 1/120)*r^5

//   ; deal with denormal results
//   xor         r9d, r9d
//   cmp         ecx, DWORD PTR __denormal_threshold

//    addsd       xmm3, xmm4  ; xmm3 <-- (r/720 + 1/120)*r^5 + (r/24 + 1/6)*r^3
//    addsd       xmm0, xmm3  ; xmm0 <-- poly

//   cmovle      r9d, ecx
//   add         rcx, 1023
//   shl         rcx, 52

    // f1, f2
    vmulsd       xmm5, xmm0, QWORD PTR [r11+rax*8]
    vmulsd       xmm1, xmm0, QWORD PTR [r10+rax*8]

    cmp          rcx, QWORD PTR __real_inf[rip]

    // (f1+f2)*(1+q)
    vaddsd       xmm5, xmm5, QWORD PTR [r11+rax*8]
    vaddsd       xmm1, xmm1, xmm5
    vaddsd       xmm1, xmm1, QWORD PTR [r10+rax*8]
    vmovapd      xmm0, xmm1

    je           Lpow_fma3_process_almost_inf

    test         r9d, r9d
    mov          QWORD PTR [p_temp_exp+rsp], rcx
    jnz          Lpow_fma3_process_denormal
    vmulsd       xmm0, xmm0, QWORD PTR [p_temp_exp+rsp]
    vorpd        xmm0, xmm0, XMMWORD PTR [negate_result+rsp]

Lpow_fma3_final_check:
    AVXRestoreXmm  xmm7, save_xmm7
    AVXRestoreXmm  xmm6, save_xmm6
    StackDeallocate stack_size
    ret

.align 16
Lpow_fma3_process_almost_inf:
    vcomisd      xmm0, QWORD PTR __real_one[rip]
    jae          Lpow_fma3_process_result_inf

    vorpd        xmm0, xmm0, XMMWORD PTR __enable_almost_inf[rip]
    vorpd        xmm0, xmm0, XMMWORD PTR [negate_result+rsp]
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_process_denormal:
    mov          ecx, r9d
    xor          r11d, r11d
    vcomisd      xmm0, QWORD PTR __real_one[rip]
    cmovae       r11d, ecx
    cmp          r11d, DWORD PTR __denormal_threshold[rip]
    jne          Lpow_fma3_process_true_denormal

    vmulsd       xmm0, xmm0, QWORD PTR [p_temp_exp+rsp]
    vorpd        xmm0, xmm0, XMMWORD PTR [negate_result+rsp]
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_process_true_denormal:
    xor          r8, r8
    cmp          rdx, QWORD PTR __denormal_tiny_threshold[rip]
    mov          r9, 1
    jg           Lpow_fma3_process_denormal_tiny
    add          ecx, 1074
    cmovs        rcx, r8
    shl          r9, cl
    mov          rcx, r9

    mov          QWORD PTR [p_temp_exp+rsp], rcx
    vmulsd       xmm0, xmm0, QWORD PTR [p_temp_exp+rsp]
    vorpd        xmm0, xmm0, XMMWORD PTR [negate_result+rsp]
    jmp          Lpow_fma3_z_denormal

.align 16
Lpow_fma3_process_denormal_tiny:
    vmovsd       xmm0, QWORD PTR __real_smallest_denormal[rip]
    vorpd        xmm0, xmm0, XMMWORD PTR [negate_result+rsp]
    jmp          Lpow_fma3_z_denormal

.align 16
Lpow_fma3_process_result_zero:
    mov          r11, QWORD PTR __real_zero[rip]
    or           r11, QWORD PTR [negate_result+rsp]
    jmp          Lpow_fma3_z_is_zero_or_inf

.align 16
Lpow_fma3_process_result_inf:
    mov          r11, QWORD PTR __real_inf[rip]
    or           r11, QWORD PTR [negate_result+rsp]
    jmp          Lpow_fma3_z_is_zero_or_inf

.align 16
Lpow_fma3_denormal_adjust:
    vpor         xmm2, xmm2, XMMWORD PTR __real_one[rip]
    vsubsd       xmm2, xmm2, QWORD PTR __real_one[rip]
    vmovapd      xmm5, xmm2
    vpand        xmm2, xmm2, XMMWORD PTR __real_mant[rip]
    vmovq        r8, xmm2
    vpsrlq       xmm5, xmm5, 52
    vpsubd       xmm5, xmm5, XMMWORD PTR __mask_2045[rip]
    vcvtdq2pd    xmm6, xmm5
    jmp          Lpow_fma3_continue_common

.align 16
Lpow_fma3_x_is_neg:

    mov          r10, QWORD PTR __exp_mask[rip]
    and          r10, r8
    cmp          r10, QWORD PTR __ay_max_bound[rip]
    jg           Lpow_fma3_ay_is_very_large

    // determine if y is an integer
    mov          r10, QWORD PTR __exp_mant_mask[rip]
    and          r10, r8
    mov          r11, r10
    mov          rcx, QWORD PTR __exp_shift[rip]
    shr          r10, cl
    sub          r10, QWORD PTR __exp_bias[rip]
    js           Lpow_fma3_x_is_neg_y_is_not_int

    mov          rax, QWORD PTR __exp_mant_mask[rip]
    and          rax, rdx
    mov          QWORD PTR [save_ax+rsp], rax

    cmp          r10, QWORD PTR __yexp_53[rip]
    mov          rcx, r10
    jg           Lpow_fma3_continue_after_y_int_check

    mov          r9, QWORD PTR __mant_full[rip]
    shr          r9, cl
    and          r9, r11
    jnz          Lpow_fma3_x_is_neg_y_is_not_int

    mov          r9, QWORD PTR __1_before_mant[rip]
    shr          r9, cl
    and          r9, r11
    jz           Lpow_fma3_continue_after_y_int_check

    mov          rax, QWORD PTR __sign_mask[rip]
    mov          QWORD PTR [negate_result+rsp], rax

Lpow_fma3_continue_after_y_int_check:

    cmp          rdx, QWORD PTR __neg_zero[rip]
    je           Lpow_fma3_x_is_zero

    cmp          rdx, QWORD PTR __neg_one[rip]
    je           Lpow_fma3_x_is_neg_one

    mov          r9, QWORD PTR __exp_mask[rip]
    and          r9, rdx
    cmp          r9, QWORD PTR __exp_mask[rip]
    je           Lpow_fma3_x_is_inf_or_nan

    vmovsd       xmm0, QWORD PTR [save_ax+rsp]
    jmp          Lpow_fma3_log_x


.align 16
Lpow_fma3_near_one:

    // f = F - Y, r = f * inv
    vmovapd      xmm0, xmm1
    vsubsd       xmm1, xmm1, xmm2         // xmm1 <-- f
    vmovapd      xmm4, xmm1               // xmm4 <-- copy of f

    vmovsd       xmm3, QWORD PTR [r9+r8*8]
    vaddsd       xmm3, xmm3, QWORD PTR [rdx+r8*8]
    vmulsd       xmm4, xmm4, xmm3         // xmm4 <-- r = f*inv
    vandpd       xmm4, xmm4, XMMWORD PTR __real_fffffffff8000000[rip] // r1
    vmovapd      xmm5, xmm4               // xmm5 <-- copy of r1
//   mulsd        xmm4, xmm0               ; xmm4 <-- F*r1
//   subsd        xmm1, xmm4               ; xmm1 <-- f - F*r1
    vfnmadd231sd xmm1, xmm4, xmm0         // xmm1 <-- f - F*r1
    vmulsd       xmm1, xmm1, xmm3         // xmm1 <-- r2 = (f - F*r1)*inv
    vmovapd      xmm7, xmm1               // xmm7 <-- copy of r2
    vaddsd       xmm1, xmm1, xmm5         // xmm1 <-- r = r1 + r2

    vmovapd      xmm2, xmm1               // xmm2 <-- copy of r
    vmovapd      xmm0, xmm1               // xmm0 <-- copy of r

    lea          r9, __log_256_lead[rip]

    // poly
    // NOTE: Given the complicated corrections here,
    // I'm afraid to mess with it too much - WAT
    vmovsd       xmm3, QWORD PTR __real_1_over_7[rip]
    vmovsd       xmm1, QWORD PTR __real_1_over_4[rip]
    vmulsd       xmm0, xmm0, xmm2         // xmm0 <-- r^2
    vmovapd      xmm4, xmm0               // xmm4 <-- copy of r^2
    vfmadd213sd  xmm3, xmm2, QWORD PTR __real_1_over_6[rip] // xmm3 <-- r/7 + 1/6
    vfmadd213sd  xmm1, xmm2, QWORD PTR __real_1_over_3[rip] // xmm1 <-- r/4 + 1/3
    vmulsd       xmm4, xmm4, xmm0         // xmm4 <-- r^4
    vmulsd       xmm1, xmm1, xmm2         // xmm1 <-- (r/4 + 1/3)*r
    vfmadd213sd  xmm3, xmm2, QWORD PTR __real_1_over_5[rip] // xmm3 <-- ((r/7 + 1/6)*r) + 1/5
    vmulsd       xmm3, xmm3, xmm2         // xmm3 <-- (((r/7 + 1/6)*r) + 1/5)*r
    vmulsd       xmm1, xmm1, xmm0         // xmm1 <-- ((r/4 + 1/3)*r)*r^2
    vmulsd       xmm3, xmm3, xmm4         // xmm3 <-- ((((r/7 + 1/6)*r) + 1/5)*r)*r^4

    vmovapd      xmm2, xmm5               // xmm2 <-- copy of r1
    vmovapd      xmm0, xmm7               // xmm0 <-- copy of r2
    vmulsd       xmm0, xmm0, xmm0         // xmm0 <-- r2^2
    vmulsd       xmm0, xmm0, QWORD PTR __real_1_over_2[rip] // xmm0 <-- r2^2/2
//   mulsd        xmm5, xmm7               ; xmm5 <-- r1*r2
//   addsd        xmm5, xmm0               ; xmm5 <-- r1*r2 + r2^2^2
    vfmadd213sd  xmm5, xmm7, xmm0         // xmm5 <-- r1*r2 + r2^2^2
    vaddsd       xmm5, xmm5, xmm7         // xmm5 <-- r1*r2 + r2^2/2 + r2

    vmovapd      xmm0, xmm2               // xmm0 <-- copy of r1
    vmovapd      xmm7, xmm2               // xmm7 <-- copy of r1
    vmulsd       xmm0, xmm0, xmm0         // xmm0 <-- r1^2
    vmulsd       xmm0, xmm0, QWORD PTR __real_1_over_2[rip] // xmm0 <-- r1^2/2
    vmovapd      xmm4, xmm0               // xmm4 <-- copy of r1^2/2
    vaddsd       xmm2, xmm2, xmm0         // xmm2 <--  r1 + r1^2/2
    vsubsd       xmm7, xmm7, xmm2         // xmm7 <-- r1 - (r1 + r1^2/2)
    vaddsd       xmm7, xmm7, xmm4         // xmm7 <-- r1 - (r1 + r1^2/2) + r1^2/2
    // xmm3 <-- ((((r/7 + 1/6)*r) + 1/5)*r)*r^4 + r1 - (r1 + r1^2/2) + r1^2/2
    vaddsd       xmm3, xmm3, xmm7
    vmovsd       xmm4, QWORD PTR __real_log2_tail[rip]
    // xmm1 <-- (((((r/7 + 1/6)*r) + 1/5)*r)*r^4) +
    //   (r1 - (r1 + r1^2/2) + r1^2/2) + ((r/4 + 1/3)*r)*r^2)
    vaddsd       xmm1, xmm1, xmm3
    lea          rdx, __log_256_tail[rip]
    // xmm1 <-- ((((((r/7 + 1/6)*r) + 1/5)*r)*r^4) +
    //   (r1 - (r1 + r1^2/2) + r1^2/2) + ((r/4 + 1/3)*r)*r^2))
    //   +(r1*r2 + r2^2/2 + r2)
    vaddsd       xmm1, xmm1, xmm5
    // xmm4 <-- vt * log2_tail  + log256_tail
    vfmadd213sd  xmm4, xmm6, QWORD PTR [rdx+r8*8]
    // xmm4 <-- vt * log2_tail  + log2_tail - corrected poly
    vsubsd       xmm4, xmm4, xmm1

    vmovapd      xmm1, xmm4
    vsubsd       xmm3, xmm4, xmm2 // xmm3 <-- xmm4 - more correction???

    vmovsd       xmm0, QWORD PTR [r9+r8*8] // xmm0 <-- log256_lead
    // xmm0 <-- log256_lead + vt*log2_lead
    vfmadd231sd  xmm0, xmm6, QWORD PTR __real_log2_lead[rip]

    // at this point, xmm0, xmm1, xmm2, and xmm3 should matter
    jmp          Lpow_fma3_log_x_continue


.align 16
Lpow_fma3_x_is_pos_one:
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_y_is_zero:
    vmovsd       xmm0, QWORD PTR __real_one[rip]
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_y_is_one:
    xor          rax, rax
    mov          r11, rdx
    mov          r9, QWORD PTR __exp_mask[rip]
    //or          r11, QWORD PTR __qnan_set
    and          r9, rdx
    cmp          r9, QWORD PTR __exp_mask[rip]
    cmove        rax, rdx
    mov          r9, QWORD PTR __mant_mask[rip]
    and          r9, rax
    jnz          Lpow_fma3_x_is_nan

    vmovq        xmm0, rdx
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_x_is_neg_one:
    mov          rdx, QWORD PTR __pos_one[rip]
    or           rdx, QWORD PTR [negate_result+rsp]
    xor          rax, rax
    mov          r11, r8
    mov          r10, QWORD PTR __exp_mask[rip]
    //or          r11, QWORD PTR __qnan_set
    and          r10, r8
    cmp          r10, QWORD PTR __exp_mask[rip]
    cmove        rax, r8
    mov          r10, QWORD PTR __mant_mask[rip]
    and          r10, rax
    jnz          Lpow_fma3_y_is_nan

    vmovq        xmm0, rdx
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_x_is_neg_y_is_not_int:
    mov          r9, QWORD PTR __exp_mask[rip]
    and          r9, rdx
    cmp          r9, QWORD PTR __exp_mask[rip]
    je           Lpow_fma3_x_is_inf_or_nan

    cmp          rdx, QWORD PTR __neg_zero[rip]
    je           Lpow_fma3_x_is_zero

    vmovsd       xmm0, QWORD PTR [save_x+rsp]
    vmovsd       xmm1, QWORD PTR [save_y+rsp]
    vmovsd       xmm2, QWORD PTR __neg_qnan[rip]
    mov          r9d, DWORD PTR __flag_x_neg_y_notint[rip]

    call         fname_special
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_ay_is_very_large:
    mov          r9, QWORD PTR __exp_mask[rip]
    and          r9, rdx
    cmp          r9, QWORD PTR __exp_mask[rip]
    je           Lpow_fma3_x_is_inf_or_nan

    mov          r9, QWORD PTR __exp_mant_mask[rip]
    and          r9, rdx
    jz           Lpow_fma3_x_is_zero

    cmp          rdx, QWORD PTR __neg_one[rip]
    je           Lpow_fma3_x_is_neg_one

    mov          r9, rdx
    and          r9, QWORD PTR __exp_mant_mask[rip]
    cmp          r9, QWORD PTR __pos_one[rip]
    jl           Lpow_fma3_ax_lt1_y_is_large_or_inf_or_nan

    jmp          Lpow_fma3_ax_gt1_y_is_large_or_inf_or_nan

.align 16
Lpow_fma3_x_is_zero:
    mov          r10, QWORD PTR __exp_mask[rip]
    xor          rax, rax
    and          r10, r8
    cmp          r10, QWORD PTR __exp_mask[rip]
    je           Lpow_fma3_x_is_zero_y_is_inf_or_nan

    mov          r10, QWORD PTR __sign_mask[rip]
    and          r10, r8
    cmovnz       rax, QWORD PTR __pos_inf[rip]
    jnz          Lpow_fma3_x_is_zero_z_is_inf

    vmovq        xmm0, rax
    vorpd        xmm0, xmm0, XMMWORD PTR [negate_result+rsp]
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_x_is_zero_z_is_inf:

    vmovsd       xmm0, QWORD PTR [save_x+rsp]
    vmovsd       xmm1, QWORD PTR [save_y+rsp]
    vmovq        xmm2, rax
    vorpd        xmm2, xmm2, XMMWORD PTR [negate_result+rsp]
    mov          r9d, DWORD PTR __flag_x_zero_z_inf[rip]

    call         fname_special
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_x_is_zero_y_is_inf_or_nan:
    mov          r11, r8
    cmp          r8, QWORD PTR __neg_inf[rip]
//   The next two lines do not correspond to IEEE754-2008.
//   +-0 ^ -Inf should be +Inf with no exception
//   +-0 ^ +Inf should be +0 with no exception
//   cmove        rax, QWORD PTR __pos_inf
//   je           Lpow_fma3_x_is_zero_z_is_inf
//  begin replacement
    je           Lpow_fma3_x_is_zero_y_is_neg_inf
    cmp          r8, QWORD PTR __neg_inf[rip]
    je           Lpow_fma3_x_is_zero_y_is_pos_inf
//  end replacement

    //or          r11, QWORD PTR __qnan_set
    mov          r10, QWORD PTR __mant_mask[rip]
    and          r10, r8
    jnz          Lpow_fma3_y_is_nan

    vmovq        xmm0, rax
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_x_is_zero_y_is_neg_inf:
    // quietly return +Inf
    vmovsd       xmm0, __pos_inf[rip]
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_x_is_zero_y_is_pos_inf:
    // quietly return +0.
    vxorpd       xmm0, xmm0, xmm0
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_x_is_inf_or_nan:
    xor          r11, r11
    mov          r10, QWORD PTR __sign_mask[rip]
    and          r10, r8
    cmovz        r11, QWORD PTR __pos_inf[rip]
    mov          rax, rdx
    mov          r9, QWORD PTR __mant_mask[rip]
    //or          rax, QWORD PTR __qnan_set
    and          r9, rdx
    cmovnz       r11, rax
    jnz          Lpow_fma3_x_is_nan

    xor          rax, rax
    mov          r9, r8
    mov          r10, QWORD PTR __exp_mask[rip]
    //or          r9, QWORD PTR __qnan_set
    and          r10, r8
    cmp          r10, QWORD PTR __exp_mask[rip]
    cmove        rax, r8
    mov          r10, QWORD PTR __mant_mask[rip]
    and          r10, rax
    cmovnz       r11, r9
    jnz          Lpow_fma3_y_is_nan

    vmovq        xmm0, r11
    vorpd        xmm0, xmm0, XMMWORD PTR [negate_result+rsp]
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_ay_is_very_small:
    vaddsd       xmm0, xmm1, QWORD PTR __pos_one[rip]
    jmp          Lpow_fma3_final_check


.align 16
Lpow_fma3_ax_lt1_y_is_large_or_inf_or_nan:
    xor          r11, r11
    mov          r10, QWORD PTR __sign_mask[rip]
    and          r10, r8
    cmovnz       r11, QWORD PTR __pos_inf[rip]
    jmp          Lpow_fma3_adjust_for_nan

.align 16
Lpow_fma3_ax_gt1_y_is_large_or_inf_or_nan:
    xor          r11, r11
    mov          r10, QWORD PTR __sign_mask[rip]
    and          r10, r8
    cmovz        r11, QWORD PTR __pos_inf[rip]

.align 16
Lpow_fma3_adjust_for_nan:

    xor          rax, rax
    mov          r9, r8
    mov          r10, QWORD PTR __exp_mask[rip]
    //or          r9, QWORD PTR __qnan_set
    and          r10, r8
    cmp          r10, QWORD PTR __exp_mask[rip]
    cmove        rax, r8
    mov          r10, QWORD PTR __mant_mask[rip]
    and          r10, rax
    cmovnz       r11, r9
    jnz          Lpow_fma3_y_is_nan

    test         rax, rax
    jnz          Lpow_fma3_y_is_inf

.align 16
Lpow_fma3_z_is_zero_or_inf:

    mov          r9d, DWORD PTR __flag_z_zero[rip]
    test         r11, QWORD PTR __exp_mant_mask[rip]
    cmovnz       r9d, DWORD PTR __flag_z_inf[rip]

    vmovsd       xmm0, QWORD PTR [save_x+rsp]
    vmovsd       xmm1, QWORD PTR [save_y+rsp]
    vmovq        xmm2, r11

    call         fname_special
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_y_is_inf:

    vmovq        xmm0, r11
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_x_is_nan:

    xor          rax, rax
    mov          r10, QWORD PTR __exp_mask[rip]
    and          r10, r8
    cmp          r10, QWORD PTR __exp_mask[rip]
    cmove        rax, r8
    mov          r10, QWORD PTR __mant_mask[rip]
    and          r10, rax
    jnz          Lpow_fma3_x_is_nan_y_is_nan

    vmovsd       xmm0, QWORD PTR [save_x+rsp]
    vmovsd       xmm1, QWORD PTR [save_y+rsp]
    vmovq        xmm2, r11
    mov          r9d, DWORD PTR __flag_x_nan[rip]

    call         fname_special
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_y_is_nan:

    vmovsd       xmm0, QWORD PTR [save_x+rsp]
    vmovsd       xmm1, QWORD PTR [save_y+rsp]
    vmovq        xmm2, r11
    mov          r9d, DWORD PTR __flag_y_nan[rip]

    call         fname_special
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_x_is_nan_y_is_nan:

    mov          r9, r8

    cmp          r11, QWORD PTR __ind_pattern[rip]
    cmove        r11, r9
    je           Lpow_fma3_continue_xy_nan

    cmp          r9, QWORD PTR __ind_pattern[rip]
    cmove        r9, r11

    mov          r10, r9
    and          r10, QWORD PTR __sign_mask[rip]
    cmovnz       r9, r11

    mov          r10, r11
    and          r10, QWORD PTR __sign_mask[rip]
    cmovnz       r11, r9

Lpow_fma3_continue_xy_nan:
    //or          r11, QWORD PTR __qnan_set
    vmovsd       xmm0, QWORD PTR [save_x+rsp]
    vmovsd       xmm1, QWORD PTR [save_y+rsp]
    vmovq        xmm2, r11
    mov          r9d, DWORD PTR __flag_x_nan_y_nan[rip]

    call         fname_special
    jmp          Lpow_fma3_final_check

.align 16
Lpow_fma3_z_denormal:
    vmovapd      xmm2, xmm0
    vmovsd       xmm0, QWORD PTR [save_x+rsp]
    vmovsd       xmm1, QWORD PTR [save_y+rsp]
    mov          r9d, DWORD PTR __flag_z_denormal[rip]

    call         fname_special
    jmp          Lpow_fma3_final_check

.seh_endproc
.endfunc
// END

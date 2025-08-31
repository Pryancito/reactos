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
// $Workfile: fmod.asm $
// $Revision: 4 $
//     $Date: 9/15/04 16:43 $
//
//
// This is an optimized version of fmod.
//
// Define _CRTBLD_C9X to make it compliant with C90 and on.
//
// If building the OS CRTL (_NTSUBSET_ defined), abort.

//		.ERRDEF _NTSUBSET_, "x87 code cannot be used in kernel mode"

#define DOMAIN 	1			// _DOMAIN
#define EDOM 	33			// EDOM
#define FPCODEFMOD 	22			// _FpCodeFmod
#define INVALID 	8			// AMD_F_INVALID

#define FPIND 	0x0fff8000000000000	// indefinite
#define FPSNAN 	0x07ff7ffffffffffff	// SNAN
#define FPQNAN 	0x07fffffffffffffff	// QNAN

X87SW_B = 0
MASK_X87SW_B = 0x1
X87SW_C3 = 1
MASK_X87SW_C3 = 0x2
X87SW_TOP = 2
MASK_X87SW_TOP = 0x1c
X87SW_C = 5
MASK_X87SW_C = 0xe0
X87SW_ES = 8
MASK_X87SW_ES = 0x100
X87SW_SF = 9
MASK_X87SW_SF = 0x200
X87SW_PE = 10
MASK_X87SW_PE = 0x400
X87SW_E = 11
MASK_X87SW_E = 0xf800


#define X87XAM 	MASK_X87SW_C3 | MASK_X87SW_C & ! (1 << (X87SW_C + 1))
#define X87XAM_INF 	5 << X87SW_C
#define X87XAM_NAN 	1 << X87SW_C
#define X87XAM_BAD 	MASK_X87SW_E & ! 2

		//EXTRN	_handle_error: PROC	// float _handle_error (char *fname, int opcode, unsigned long long value, int type, int flags, int error, double arg1, double arg2, int nargs)

		.section .rdata

@fmodz:		.ascii "fmod"
.byte 0

		.code64 .intel_syntax noprefix

// double fmod [double, double] ----------------------------------

.global fmod
.func fmod
fmod:
.seh_proc fmod


		sub	rsp, 40 + 32

		.seh_stackalloc 40 + 32
		.seh_endprologue

		movsd	QWORD PTR 24 [rsp + 32], xmm1	// Y
		movsd	QWORD PTR 16 [rsp + 32], xmm0	// X

		.byte 0x0dd, 0x44, 0x24, 0x38	// fld	QWORD PTR 24 [rsp + 32]
		.byte 0x0dd, 0x44, 0x24, 0x30	// fld	QWORD PTR 16 [rsp + 32]

		.byte 0x0d9, 0x0e5		// fxam (X)
		.byte 0x09b, 0x0dd, 0x07c, 0x024, 0x010 // fstsw 16 [rsp]

		movzx	ecx, WORD PTR 16 [rsp]
		and	ecx, X87XAM

		fnclex			// clear exception flags
						// in preparation for fprem

@again:
		.byte 0x0d9, 0x0f8		// fprem

		.byte 0x09b, 0x0df, 0x0e0 	// fstsw	ax
		test	ax, 4 << X87SW_C
		jnz	@again	// do it again in case of partial result

		.byte 0x0dd, 0x01c, 0x024	// fstp	QWORD PTR [rsp]
		movlpd	xmm0, QWORD PTR [rsp]		// result

		.byte 0x0d9, 0x0e5		// fxam (Y)
		.byte 0x09b, 0x0dd, 0x07c, 0x024, 0x008 // fstsw 8 [rsp]

		movzx	edx, WORD PTR 8 [rsp]
		and	edx, X87XAM

		.byte 0x0dd, 0x0d8		// fstp	st(0)

		cmp	edx, X87XAM_NAN		// fmod (x, NAN) = QNAN
		je	@error

		cmp	ecx, X87XAM_NAN		// fmod (NAN, y) = QNAN
		je	@error

		and	eax, X87XAM_BAD
		jnz	@raise			// handle error

		.IFNDEF	_CRTBLD_C9X		// Not C90
		cmp	edx, X87XAM_INF		// fmod (x, infinity) = ???
		je	@raise
		.ELSE				// C90
						// fmod (x, infinity) = x (as x87 already does)
		.ENDIF

@exit:
		add	rsp, 40 + 32
		ret

		.align	16

@raise:
		mov	eax, INVALID		// raise exception
		mov	r8, FPIND
		jmp	@fail

@error:
		xor	eax, eax		// no exception
		movd	r8, xmm0
		jmp	@fail

@fail:
		lea	rcx, @fmodz[rip]		// fname
		mov	edx, FPCODEFMOD		// opcode
//		mov	r8, INDEF		; value
		mov	r9d, DOMAIN		// type
		mov	DWORD PTR  0 [rsp + 32], eax // flags
		mov	DWORD PTR  8 [rsp + 32], EDOM // error
		mov	DWORD PTR 32 [rsp + 32], 2 // nargs
		call	_handle_error		// (char *fname, int opcode, unsigned long long value, int type, int flags, int error, double arg1, double arg2, int nargs)

		.byte 0x09b, 0x0db, 0x0e2	// fclex
		jmp	@exit

.seh_endproc
.endfunc

// ---------------------------------------------------------------

		// END

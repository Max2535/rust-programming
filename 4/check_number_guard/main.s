	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"main.5f6bf0c8e9d0afce-cgu.0"
	.def	_ZN3std2rt10lang_start17h96f583715fa682afE;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start17h96f583715fa682afE
	.globl	_ZN3std2rt10lang_start17h96f583715fa682afE
	.p2align	4
_ZN3std2rt10lang_start17h96f583715fa682afE:
.seh_proc _ZN3std2rt10lang_start17h96f583715fa682afE
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movq	%r8, %rax
	movq	%rdx, %r8
	movq	%rcx, 48(%rsp)
	movb	%r9b, 32(%rsp)
	leaq	anon.e17bad63fec66305fa0f492970efde6a.0(%rip), %rdx
	leaq	48(%rsp), %rcx
	movq	%rax, %r9
	callq	_ZN3std2rt19lang_start_internal17h2abdc5cc45b16a8aE
	nop
	addq	$56, %rsp
	retq
	.seh_endproc

	.def	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfeb0e796775abb79E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfeb0e796775abb79E
	.p2align	4
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfeb0e796775abb79E:
.seh_proc _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfeb0e796775abb79E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hb99927b455bbd6e8E
	xorl	%eax, %eax
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hb99927b455bbd6e8E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hb99927b455bbd6e8E
	.p2align	4
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hb99927b455bbd6e8E:
.seh_proc _ZN3std3sys9backtrace28__rust_begin_short_backtrace17hb99927b455bbd6e8E
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	callq	*%rcx
	#APP
	#NO_APP
	nop
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hdb6e30601d8c187cE;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hdb6e30601d8c187cE
	.p2align	4
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hdb6e30601d8c187cE:
.seh_proc _ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hdb6e30601d8c187cE
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	movq	(%rcx), %rcx
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17hb99927b455bbd6e8E
	xorl	%eax, %eax
	addq	$40, %rsp
	retq
	.seh_endproc

	.def	_ZN4main4main17h5659c6c02e5cd445E;
	.scl	3;
	.type	32;
	.endef
	.section	.text,"xr",one_only,_ZN4main4main17h5659c6c02e5cd445E
	.p2align	4
_ZN4main4main17h5659c6c02e5cd445E:
.seh_proc _ZN4main4main17h5659c6c02e5cd445E
	pushq	%rsi
	.seh_pushreg %rsi
	subq	$96, %rsp
	.seh_stackalloc 96
	movaps	%xmm6, 80(%rsp)
	.seh_savexmm %xmm6, 80
	.seh_endprologue
	leaq	anon.e17bad63fec66305fa0f492970efde6a.6(%rip), %rax
	movq	%rax, 32(%rsp)
	movq	$1, 40(%rsp)
	movq	$8, 48(%rsp)
	xorps	%xmm6, %xmm6
	movups	%xmm6, 56(%rsp)
	leaq	32(%rsp), %rcx
	callq	_ZN3std2io5stdio6_print17hc35641ea0a8da346E
	leaq	anon.e17bad63fec66305fa0f492970efde6a.4(%rip), %rax
	movq	%rax, 32(%rsp)
	movq	$1, 40(%rsp)
	movq	$8, 48(%rsp)
	movups	%xmm6, 56(%rsp)
	leaq	32(%rsp), %rcx
	callq	_ZN3std2io5stdio6_print17hc35641ea0a8da346E
	leaq	anon.e17bad63fec66305fa0f492970efde6a.2(%rip), %rsi
	movq	%rsi, 32(%rsp)
	movq	$1, 40(%rsp)
	movq	$8, 48(%rsp)
	movups	%xmm6, 56(%rsp)
	leaq	32(%rsp), %rcx
	callq	_ZN3std2io5stdio6_print17hc35641ea0a8da346E
	movq	%rsi, 32(%rsp)
	movq	$1, 40(%rsp)
	movq	$8, 48(%rsp)
	movups	%xmm6, 56(%rsp)
	leaq	32(%rsp), %rcx
	callq	_ZN3std2io5stdio6_print17hc35641ea0a8da346E
	movaps	80(%rsp), %xmm6
	addq	$96, %rsp
	popq	%rsi
	retq
	.seh_endproc

	.def	main;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",one_only,main
	.globl	main
	.p2align	4
main:
.seh_proc main
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movq	%rdx, %r9
	movslq	%ecx, %r8
	leaq	_ZN4main4main17h5659c6c02e5cd445E(%rip), %rax
	movq	%rax, 48(%rsp)
	movb	$0, 32(%rsp)
	leaq	anon.e17bad63fec66305fa0f492970efde6a.0(%rip), %rdx
	leaq	48(%rsp), %rcx
	callq	_ZN3std2rt19lang_start_internal17h2abdc5cc45b16a8aE
	nop
	addq	$56, %rsp
	retq
	.seh_endproc

	.section	.rdata,"dr",one_only,anon.e17bad63fec66305fa0f492970efde6a.0
	.p2align	3, 0x0
anon.e17bad63fec66305fa0f492970efde6a.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hdb6e30601d8c187cE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfeb0e796775abb79E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hfeb0e796775abb79E

	.section	.rdata,"dr",one_only,anon.e17bad63fec66305fa0f492970efde6a.1
anon.e17bad63fec66305fa0f492970efde6a.1:
	.ascii	"Out of range\n"

	.section	.rdata,"dr",one_only,anon.e17bad63fec66305fa0f492970efde6a.2
	.p2align	3, 0x0
anon.e17bad63fec66305fa0f492970efde6a.2:
	.quad	anon.e17bad63fec66305fa0f492970efde6a.1
	.asciz	"\r\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.e17bad63fec66305fa0f492970efde6a.3
anon.e17bad63fec66305fa0f492970efde6a.3:
	.ascii	"Even\n"

	.section	.rdata,"dr",one_only,anon.e17bad63fec66305fa0f492970efde6a.4
	.p2align	3, 0x0
anon.e17bad63fec66305fa0f492970efde6a.4:
	.quad	anon.e17bad63fec66305fa0f492970efde6a.3
	.asciz	"\005\000\000\000\000\000\000"

	.section	.rdata,"dr",one_only,anon.e17bad63fec66305fa0f492970efde6a.5
anon.e17bad63fec66305fa0f492970efde6a.5:
	.ascii	"Odd\n"

	.section	.rdata,"dr",one_only,anon.e17bad63fec66305fa0f492970efde6a.6
	.p2align	3, 0x0
anon.e17bad63fec66305fa0f492970efde6a.6:
	.quad	anon.e17bad63fec66305fa0f492970efde6a.5
	.asciz	"\004\000\000\000\000\000\000"


.equ	RAMEND	= 0x08ff
.def	ZL	= r30
.def	ZH	= r31
.equ	SPL	= 0x3d
.equ	SPH	= 0x3e

	ldi	r16, LOW(RAMEND)		; load low byte of RAMEND into r16
	out	SPL, r16			; store r16 in stack pointer low
	ldi	r16, HIGH(RAMEND)	; load high byte of RAMEND into r16
	out	SPH, r16			; store r16 in stack pointer high
	push	r0			; push r0 to the stack
	pop	r0			; restore r0 from stack
	push	r0			; push contents of r0 to stack
	push	r1			; push contents of r1 to stack
	push	r2			; push contents of r2 to stack

	pop	r2			; restore contents of r2
	pop	r1			; restore contents of r1
	pop	r0			; restore contents of r0
    ldi	r16,0x01		; load r16 with 0x01
	ldi	r17,0x02		; load r17 with 0x02

	push	r16			; save r16 to the stack
	push	r17			; save r17 to the stack

;	pop	r16			; restore r16 (result = 0x02)
;	pop	r17			; restore r17 (result = 0x01)
;   ldi	r16,0x01		; load r16 with 0x01
;	ldi	r17,0x02		; load r17 with 0x02

;	call	addReg			; call subroutine

;loop:	rjmp	loop			; infinite loop

;addReg:
;	add	r16,r17			; add r16 and r17

;	ret				; return from subroutine
;    rcall	doSomething		; call subroutine

					; rest of program

;doSomething:
;	push	r16			; save r16 to the stack
;	push	r17			; save r17 to the stack

					; subroutine code

;	pop	r17			; restore r17 from the stack
;	pop	r16			; restore r16 from the stack

;	ret				; return from subroutine

;   .include "m328pdef.inc"
;	.include "delayMacro.inc"

;	.def	mask 	= r16		; mask register
;	.def	ledR 	= r17		; led register
;	.def	loopCt	= r18		; delay loop count

;	.equ	iVal 	= 39998		; inner loop value

;	.cseg
;	.org	0x00
;	ldi	r16,LOW(RAMEND)		; initialize
;	out	SPL,r16			; stack pointer
;	ldi	r16,HIGH(RAMEND)	; to RAMEND
;	out	SPH,r16			; "

;	clr	ledR			; clear led register
;	ldi	mask,(1<<PINB0)		; load 00000001 into mask register
;	out	DDRB,mask		; set PINB0 to output

;start:	eor	ledR,mask		; toggle PINB0 in led register
;	out	PORTB,ledR		; write led register to PORTB

;	delayms 500			; delay for 500ms

;	rjmp	start			; jump back to start

;	.include "delay10ms.asm"	; include delay subroutine

 ;   .macro	delayms
;	push	r18
;	push	r24
;	push	r25

;	ldi	r18,@0/10
;	rcall	delay10ms

;	pop	r25
;	pop	r24
;	pop	r18
;	.endmacro

;   .macro	setStack
;	.if @0>RAMEND
;	.error "Value greater than RAMEND used for setting stack"
;	.else
;	ldi	@1,LOW(@0)
;	out	SPL,@1
;	ldi	@1,LOW(@0)
;	out	SPL,@1
;	.endif
;	.endmacro
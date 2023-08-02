.cseg
.include "m328pdef.inc"

.org	0x00

.def	mask 	= r16

.equ	iVal 	= 39998		; inner loop value

; mask register

.include "m328pdef.inc"
.include "delayMacro.inc"

ret
rcall	doSomething		; call subroutine

label: ldi r16, r16
ldi r16, 1
ldi r16
ldi r16, LOW(RAMEND) ; mask register
ldi	r16, 0x01

ldi	mask,(1<<PINB0)		; load 00000001 into mask register

ldi	r18,@0/10

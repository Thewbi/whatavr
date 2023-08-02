.cseg
.include "m328pdef.inc"

.org	0x00

.def	mask 	= r16

.equ	iVal 	= 39998		; inner loop value

; mask register

label: ldi r16, r16
ldi r16, 1
ldi r16
ldi r16, LOW(RAMEND) ; mask register
ldi	r16, 0x01

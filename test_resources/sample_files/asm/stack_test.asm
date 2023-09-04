.include "m328Pdef.inc"

; fill RAM with test data

; X = 0x0708
ldi r26, 0x08 // low
ldi r27, 0x07 // high

ldi r16, 0x65
st X+, r16
ldi r16, 0x66
st X+, r16
ldi r16, 0x67
st X+, r16
ldi r16, 0x68
st X+, r16
ldi r16, 0x69
st X+, r16
ldi r16, 0x45
st X+, r16
ldi r16, 0x99
st X+, r16
ldi r16, 0x77
st X+, r16
ldi r16, 0x0F
st X+, r16
ldi r16, 0xFE
st X+, r16
ldi r16, 0xFD
st X+, r16
ldi r16, 0xFB
st X+, r16
ldi r16, 0xBC
st X+, r16
ldi r16, 0xB0
st X+, r16
ldi r16, 0x1A
st X+, r16
ldi r16, 0x1B
st X+, r16

; set stack pointer
;ldi r16, 0x0D ; LOW(RAMEND)
ldi r16, LOW(RAMEND)
out SPL, r16
;ldi r16, 0x07 ; HIGH(RAMEND)
ldi r16, HIGH(RAMEND)
out SPH, r16

; This instruction loads register Rd with a byte from the STACK. 
; The Stack Pointer is pre-incremented by 1
; PRE-INCREMENTED! before the POP.
push r17
push r16
pop r16
pop r17
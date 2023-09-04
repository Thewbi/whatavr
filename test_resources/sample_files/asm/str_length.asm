.include "m328Pdef.inc"

    .cseg

    .org 0x0000

    ldi r16, HIGH(RAMEND)
	out SPH, r16
	ldi r16, LOW(RAMEND)
	out SPL, r16

main:
    ldi ZH, HIGH(teststring*2)
    ldi ZL, LOW(teststring*2)

teststring: .db "Hello world",0
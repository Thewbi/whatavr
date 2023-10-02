.equ	RAMEND	= 0x08ff
.equ	SPL	= 0x3d
.equ	SPH	= 0x3e

        ; initialize the stack 
        ldi r16, LOW(RAMEND)
        out SPL, r16
        ldi r16, HIGH(RAMEND)
        out SPH, r16

        ldi r16, 4
        ldi r17, 3
start:  add r16, r17
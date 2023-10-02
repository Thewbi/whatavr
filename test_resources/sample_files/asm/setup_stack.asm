.equ	RAMEND	= 0x08ff
.equ	SPL	= 0x3d
.equ	SPH	= 0x3e

    ldi r16, LOW(RAMEND)
    out SPL, r16
    ldi r16, HIGH(RAMEND)
    out SPH, r16
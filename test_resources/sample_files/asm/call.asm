.equ	SPL	= 0x3d
.equ	SPH	= 0x3e
.def tmp_reg = r16

	ldi tmp_reg, HIGH(RAMEND) ; 0 1
	out SPH, tmp_reg          ; 2 3
	ldi tmp_reg, LOW(RAMEND)  ; 4 5
	out SPL, tmp_reg          ; 6 7

    ; call subroutine_label
    call 0x100
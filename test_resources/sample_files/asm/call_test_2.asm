.include "m328Pdef.inc"

.def tmp_reg = r16

	ldi tmp_reg, HIGH(RAMEND)
	out SPH, tmp_reg
	ldi tmp_reg, LOW(RAMEND)
	out SPL, tmp_reg

	ldi r17, 0xAA
	ldi r18, 0xBB

	call sub_routine_0

	rjmp end

sub_routine_0:
	push r17
	call sub_routine_1
	pop r17
	ret

sub_routine_1:
	push r18
	pop r18
	ret

end:
    nop
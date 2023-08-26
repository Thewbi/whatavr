.include "m328Pdef.inc"

.def tmp_reg = r16

	ldi tmp_reg, HIGH(RAMEND) ; 0 1
	out SPH, tmp_reg          ; 2 3
	ldi tmp_reg, LOW(RAMEND)  ; 4 5
	out SPL, tmp_reg          ; 6 7

	ldi r17, 0xAA             ; 8 9
	ldi r18, 0xBB             ; 10 11

	call sub_routine_0        ; 12 13 14 15

	rjmp end                  ; 16 17

sub_routine_0:
	push r17                  ; 18 19
	call sub_routine_1        ; 20 21 22 23
	pop r17                   ; 24 25
	ret                       ; 26 27

sub_routine_1:
	push r18                  ; 28 29
	pop r18                   ; 30 31
	ret                       ; 32 33

end:
    nop                     ; 34 35
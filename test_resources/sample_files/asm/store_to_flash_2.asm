    ldi r16, 5
	ldi r17, 0x55

	ldi r26, $40	; Set X to 0x0140
	ldi r27, $01

loop:
	st X+, r17
	dec r16
	brne loop

;here:				
;	rjmp here
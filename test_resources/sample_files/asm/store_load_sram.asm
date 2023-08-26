; source is the X register
; Set X to 0x0130
ldi r26, 0x30
ldi r27, 0x01

; target is the Y register
; set Y to 0x0200
ldi r28, 0x00
ldi r29, 0x02

; iterator
ldi r16, 5

loop:
	ld r17, X+
	st Y+, r17
	dec r16
	brne loop
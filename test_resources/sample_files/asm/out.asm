




out	0x3f, r1

.include "m328pdef.inc"

out	SPH, r16

out PORTC, PORTC_STATE 
out SREG, TMP_REG 

; out	SPL,@1
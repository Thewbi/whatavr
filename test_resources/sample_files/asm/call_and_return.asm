;
; CALL and RET
;

;.include "m328Pdef.inc"

.equ	RAMEND	= 0x08ff
.equ	SPL	= 0x3d
.equ	SPH	= 0x3e

    ; initialize the stack
    ldi r16, LOW(RAMEND)
    out SPL, r16
    ldi r16, HIGH(RAMEND)
    out SPH, r16

    ldi r16, 1
    ldi r17, 2

    call addReg

    ; this code will be executed after addReg because the call statement will jump to the addReg label
    ldi r16, 3
    push r16
    pop r0

    
end: 
    ;jmp end    ; endless loop
    break       ; halt CPU for debugger

addReg: 
    add r17, r16
    add r17, r16
    add r17, r16
    add r17, r16
    ret
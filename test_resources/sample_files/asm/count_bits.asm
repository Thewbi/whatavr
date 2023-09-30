    .include "m328Pdef.inc"

    .dseg
num_ones: .byte 1

    .cseg

    .org 0x0000

.include "setup_stack.asm"
;    ldi r16, HIGH(RAMEND)
;	out SPH, r16
;	ldi r16, LOW(RAMEND)
;	out SPL, r16

main:
    ldi r24, 12       ; 1100
    call count_bits
    sts num_ones, r24

end:
    rjmp end

count_bits:
    push r0
    clr r0

count_bits_loop:
    sbrc r24, 0
    inc r0
    lsr r24
    brne count_bits_loop 
    mov r24, r0
    pop r0
    ret

    
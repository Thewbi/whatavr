; https://www.youtube.com/watch?v=uBHvHNJ1A7I
;.global timer1

.include "m328Pdef.inc"

stack_init:
    ldi     r16, LOW(RAMEND)
    out     SPL, r16
    ldi     r16, HIGH(RAMEND)
    out     SPH, r16

timer1:
    LDI     R16, 0b00100000         ; to toggle PB5 (D13)
    LDI     R17, 0b00000000

    SBI     DDRB, 5                 ; set PB5 for o/p (output pin)
    OUT     PORTB, R17              ; PB5 = 0

label_1:
    RCALL   delay_timer1            ; 0.5 sec delay timer 1
    EOR     R17, R16                ; R17 = R17 XOR R16
    OUT     PORTB, R17              ; toggle PB5
    LDI     R18, 61                 ; reset loop counter
    RJMP    label_1                 ; go back & repeat toggle

delay_timer1:                       ; 1 sec delay via timer 1 

.EQU value = 57724                  ; value to give 0.5 sec delay
    LDI     R20, hi8(value)
    STS     TCNT1H, R20
    LDI     R20, lo8(value)
    STS     TCNT1L, R20             ; initialize counter TCNT1 = value

    LDI     R20, 0b00000000
    STS     TCCR1A, R20
    LDI     R20, 0b00000101
    STS     TCCR1B, R20             ; normal mode prescaler = 1024

label_2: 
    IN      R20, TIFR1              ; get the TIFR1 byte and 
    SBRS    R20, TOV1               ; check if TOV=1, skip next instruction
    RJMP    label_2                 ; else loop back and check TOV1 flag 
    ;-------------------------------------------------------
;    LDI   R20, 1<<TOV1
;    OUT   TIFR1, R20                ; clear TOV1 flag
    ;-------------------------------------------------------
;    LDI   R20, 0b00000000
;    STS   TCCR1B, R20               ; stop timer0
;    RET
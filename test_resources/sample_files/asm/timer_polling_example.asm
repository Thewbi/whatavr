; --------------------------
; ------ Delay 250ms ------
;
; Source: https://www.arxterra.com/9-atmega328p-timers/
;
; This function will make the CPU wait for 250ms when called.
; It uses the normal mode of the ATmega328p timer 1.
;
; The normal timer 1 counts upwards, it increments every tick.
; To store the tick value it uses the TCNT1 register (16bit)
; (consisting of the two 8bit registers TCNT1H and TCNT1L).
;
; The timer ticks up until it overflows the 16bit TCNT1 register.
; Once the overflow from 11111111 111111111 to 00000000 00000000
; takes place, the TOV1 bit of the TIFR1 register is set.
;
; To achieve a 250 ms delay, the tick speed has to be adjusted.
; Once the tick duration is known, the TCNT1 register will be
; initialized with a start-value. The start-value is selected so
; that when the timer 1 overflows the TCNT1 register, then exactly
; 250 ms have passed and the TOV1 bit is set.
;
; When the TOV1 bit is set, the function first clears the TOV1 flag and
; inserts the start value into the TCNT1 register again and returns.
; 
;
; Called from main program
; Input: none Output: none
; no registers are modified by this subroutine
Delay:

    push r15          ; save register r15 because it is used by this function
    in   r15, SREG    ; store SREG into r15 because ???

    push r16          ; save register r16 because it is used by this function

wait:
    ; sbis - SBIS – Skip if Bit in I/O Register is Set
    sbis TIFR1, TOV1 ; when the TOV1 bit is set (sbis) 
    rjmp wait

    sbi TIFR1, TOV1 ; clear flag bit by writing a one (1)

    ; TCNT1H and TCNT1L
    ; TCNT1H -- T(imer) C(ou)NT(er) 1 H(igh) -- Timer/Counter 1 High
    ; TCNT1L -- T(imer) C(ou)NT(er) 1 L(ow) -- Timer/Counter 1 Low
    ldi r16, 0x0B    ; load value high byte 0x0B
    sts TCNT1H, r16
    ldi r16, 0xDC    ; load value low byte 0xDC
    sts TCNT1L, r16

    pop r16           ; restore r16

    out SREG, r15     ; write r16 into SREG
    pop r15           ; restore r15

    ret
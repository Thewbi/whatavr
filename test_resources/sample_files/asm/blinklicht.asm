; ATmega328p – Einfaches Blinklicht (1Hz) realisiert in AVR-Assembler
; Zielplattform: ATMEL ATmega328p, Arduino Uno R3 SMD Edition
; by Dr. C. Jakob, fbeit, h_da, Januar 2015, christian.jakob@h-da.de

.include "m328Pdef.inc"

;.equ LOOP_CNT = 50000           ; 1 Hz
.equ LOOP_CNT = 5000            ; 10 Hz

.def PORTB_STATE = r17
.def PORTC_STATE = r18
.def BIT_MASK_5 = r19

.cseg                           ; code segment starts here ...

.org 0x0000

    ldi r16, (1 << DDB5)        ; Pin PB5 als Ausgang konfigurieren ...
    out DDRB, r16               ; DDRB 

    ldi PORTB_STATE, 0x00       ; Programmvariablen 
    ldi BIT_MASK_5, 0x20        ; initialisieren ...

wait:                           ; outer loop: 16 times. 16 x 0.03125 s = 0,5 s (Alle 0,5 s wird die LED umgeschaltet)
                                ; ==> Periode = 1s. f = 1/T Hz = 1/1 Hz = 1 Hz
                                ;
                                ; Ziel: 10 Hz. Der Ablauf muss 10 mal so schnell ablaufen: alle 0,05 s wird die LED umgeschaltet
                                ; Führe die innere Schleife 10 mal weniger oft aus. Setze LOOP_COUNT von 50000 auf 5000
                                ; 10 Takte pro iteration. 5000 Iterationen x 10 Takte = 50000 Takte. => 50000 Takte / 16*10^6 Takte / sek = 0.003125 s
                                ; outer loop: 16 times. 16 x 0.003125 s = 0,05 s (Alle 0,05 s wird die LED umgeschaltet)
                                ; ==> Periode = 0.1s. f = 1/T Hz = 1/0.1 Hz = 10 Hz

    ldi r16, 16                 ; 1 Takt

loop_0:
    ldi r24, LOW(LOOP_CNT)      ; 1 Takt - LOOP_COUNT = 50000
    ldi r25, HIGH(LOOP_CNT)     ; 1 Takt


loop_1:                         ; 10 Takte pro iteration. 50000 Iterationen x 10 Takte = 500000 Takte. => 500000 Takte / 16*10^6 Takte / sek = 0.03125 s
    sbiw r24, 1                 ; 2 Takt - 102. SBIW – Subtract Immediate from Word 
    nop                         ; 1 Takt
    nop                         ; 1 Takt
    nop                         ; 1 Takt
    nop                         ; 1 Takt
    nop                         ; 1 Takt
    nop                         ; 1 Takt
    brne loop_1                 ; 2 Takte wenn ungleich Null, 1 Takt 
                                ; wenn gleich Null ...
    dec r16                     ; 1 Takt
    brne loop_0                 ; 2 Takte wenn ungleich Null, 1 Takt 
                                ; wenn gleich Null ...

    eor PORTB_STATE, BIT_MASK_5 ; toggle LED on Pin PB50 ...
    out PORTB, PORTB_STATE      ; ...
    jmp wait

end:
    rjmp end                    ; Total Delay of ~0.5s @ 16MHz

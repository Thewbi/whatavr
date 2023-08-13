;.cseg bzw. .text
.include "m328Pdef.inc" ; useful definitions ...

    ldi r16, LOW(RAMEND)
    out SPL, r16
    ldi r16, HIGH(RAMEND)
    out SPH, r16

    call UP             ; 0082 0E94 0000 
    break               ; 0086 9895 
UP:
    push ZH             ; 0120 FF93 
    push ZL             ; 0122 EF93 
    push r16            ; 0124 0F93
    push r17            ; 0124 0F93
    clr r17             ; 0128 1127 
    add ZL, r16         ; 012a E00F 
    adc ZH, r17         ; 012c F11F
    ldi r17, 0x30       ; ASCII Null - 012e 10E3 
    ldi r16, 2          ; 0130 02E0
dowhile:
    st Z+, r17          ; 0132 1193
    dec r16             ; 0134 0A95 
    brne dowhile        ; 0136 01F4
ende:
    clr r16             ; 0138 0027
    st Z, r16           ; 013a 0083
    pop r17             ; 013c 1F91
    pop r16             ; 013e 0F91
    pop ZL              ; 0140 EF91
    pop ZH              ; 0142 FF91
    ret                 ; 0144 0895

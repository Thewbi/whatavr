
;.include "m328Pdef.inc" ; useful definitions ...

.equ	RAMEND	= 0x08ff
.equ	SPL	= 0x3d
.equ	SPH	= 0x3e

.def	ZL	= r30
.def	ZH	= r31

.cseg                   ; .cseg bzw. .text

    ldi r16, LOW(RAMEND)
    out SPL, r16
    ldi r16, HIGH(RAMEND)
    out SPH, r16

; between the OUT instruction and 0x0082, the CPU will execute a lot of NOP instructions
    
.org 0x0082

    call UP             ; 0082 0E94 0000 
    break               ; 0086 9895

.org 0x0120

UP:
    push ZH             ; 0120 FF93 
    push ZL             ; 0122 EF93 
    push r16            ; 0124 0F93
    push r17            ; 0124 0F93

    ; clear 
    clr r16             ; this statement is not part of the original code
    clr r17             ; 0128 1127

    ; initialize Z
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


.dseg						; data segment == data memory (sram): 0x0000 - 0x08FF
BUFFER: .db $32, $33, $34, $00, $ff, $ff, $ff, $ff, $ff, $23, $66, $b2, $20, $40, $30, $90


;.org 0x0120					; 0x0120
;BUFFER: .db $33, $20, $31, $42, $43, $00, $66, $b2, $c2, $e4, $d3, $09, $22, $56, $91, $3e
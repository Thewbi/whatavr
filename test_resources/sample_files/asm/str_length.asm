;.include "m328Pdef.inc"             ; useful definitions ...

.equ	RAMEND	= 0x08ff
.def	ZL	= r30
.def	ZH	= r31
.equ	SPL	= 0x3d
.equ	SPH	= 0x3e

.cseg                               ; code segments starts here ...

.org 0x0000                         ; Einsprung nach Reset ... 
                                    ; CPU starts processing at 0x0000 after power up / reset
                                    ; Keine interrupts werden enabled, keine interrupt vector tabelle wird bereitgestellt
                                    ; Es wird einfach direkt der Applications Code ausgeführt.

    ;teststring: .db "Hello world", 0

    ldi r16, HIGH(RAMEND)           ; Stackpointer wird initialsiert ...
	out SPH, r16
	ldi r16, LOW(RAMEND)
	out SPL, r16

main:
    ldi ZH, HIGH(teststring * 2)    ; Laden des Übergabewertes ins Z-Reg 
    ldi ZL, LOW(teststring * 2)     ; (2x) -> Flash ist wort-adressiert!
    call str_length                 ; Funktion ´str_length´ wird aufgerufen

end:                                ; Endlosschleife ...
    rjmp end

str_length:
    push ZH                         ; Register Z auf dem Stack retten …
    push ZL                         ; ...
    clr r25                         ; Registerpaar r25:r24 (Rückgabewert) 
    clr r24                         ; löschen 
while:
    lpm r0, z+                      ; Über Z adressiertes Byte aus dem Flash
    tst r0                          ; laden und auf '\0' prüfen ...
    breq endwhile                   ; Nulltemninierung gefunden?
    adiw r24, 1                     ; Nein, also r24 inkrementieren ...
    rjmp while                      ; ... und nächstes Byte prüfen
endwhile:                           ; Null-Byte gefunden!
    pop ZL                          ; Registerinhalte wiederherstellen 
    pop ZH                          ; ...
    ret                             ; Rücksprung ins Hauptprogramm

teststring: .db "Hello world", 0
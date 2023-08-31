;ldi r21, 0xF5 ; lade die Konstante 0xF5 ins Register r21
;ldi r22, 0x0B ; lade die Konstante 0x0B ins Register r22
;add r21, r22 ; Addiere den Inhalt von Register r22 zum
; Inhalt von Register r21 ...


;ldi r16, 0x03	    ; Register r16 wird mit dem Wert 3 initialisiert
;loop:
;	inc r16			; Registerinhalt inkrementieren
;	brne loop		; Solange das Resultat der vorangegangenen Operation ungleich Null ist, wird zum Label ‘loop’ gesprungen
;
;here:
;	rjmp here


    ldi r16, 5
	ldi r17, 0x55

	ldi r26, $40	; Set X to 0x0140
	ldi r27, $01

loop:
	st X+, r17
	dec r16
	brne loop

here:				
	rjmp here
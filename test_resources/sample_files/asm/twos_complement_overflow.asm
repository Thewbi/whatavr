;
; test_status_register.asm
;
; Created: 22.08.2023 16:30:15
; Author : U5353
;


	ldi r16, 0x00	; Register r16 wird mit dem Wert Null initialisiert
loop:
	inc r16			; Registerinhalt inkrementieren
	brne loop		; Solange das Resultat der vorangegangenen Operation
here:				; ungleich Null ist, wird zum Label ‘loop’ gesprungen
	rjmp here

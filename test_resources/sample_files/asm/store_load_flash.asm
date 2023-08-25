
/*	ldi r16, 0x00	; Register r16 wird mit dem Wert Null initialisiert
loop:
	inc r16			; Registerinhalt inkrementieren
	brne loop		; Solange das Resultat der vorangegangenen Operation
here:				; ungleich Null ist, wird zum Label ‘loop’ gesprungen
	rjmp here*/


/*ldi r21, 0xF5 ; lade die Konstante 0xF5 ins Register r21
ldi r22, 0x0B ; lade die Konstante 0x0B ins Register r22
add r21, r22 ; Addiere den Inhalt von Register r22 zum
; Inhalt von Register r21 ...*/

/*ldi r16, 48
ror r16
ror r16
ror r16*/
/*
Schreiben Sie ein Assemblerprogramm, das den Wert 0x55 an die Speicherstellen 0x0140 bis 0x0144 kopiert.
Benutzen Sie fuer ihre Umsetung die Befehle st und brne
*/
/*	
	ldi r16, 5
	ldi r17, 0x55

	ldi r26, $40	; Set X to 0x0140
	ldi r27, $01

loop:
	st X+, r17
	dec r16
	brne loop

here:				
	rjmp here*/

/* 
Schreiben Sie ein Assemblerprogramm, das die Inhalte der Speicherstellen 0x0240 bis 0x0243 ausliest und aufaddiert. 
Gehen Sie von beliebigen Inhalten aus. 
Das Ergebnis soll anschließend an den Adressen 0x0220 und 0x0221 abgespeichert werden. 
Benutzen Sie für ihre Umsetzung die Befehl st, ld, brne und brcc.
*/

;	clr r0

	ldi r16, 5
	ldi r17, 0x55

	ldi r26, $40	; Set X to 0x0240
	ldi r27, $02

write_loop:
	st X+, r17
	dec r16
	brne write_loop

	ldi r16, 4

	ldi r26, $40	; Set X to 0x0240
	ldi r27, $02

	/*; var_1
	clr r16 ; lo
	clr r17 ; hi

	; var_2
	clr r18 ; lo
	clr r19 ; hi

	ldi r20, 4
loop:
	ld r16, X+
	add r18, r16 ; lo
	adc r19, r17 ; hi
	dec r20
	brne loop*/

	; var_1
	clr r0 ; lo
	clr r1 ; hi

	; var_2
	clr r2 ; lo
	clr r3 ; hi

	ldi r20, 4
loop:
	ld r0, X+
	add r2, r0 ; lo
	adc r3, r1 ; hi
	dec r20
	brne loop

	; Set X to 0x0220
	ldi r26, $20	
	ldi r27, $02

	st X+, r2 ; lo
	st X+, r3 ; lo

	nop
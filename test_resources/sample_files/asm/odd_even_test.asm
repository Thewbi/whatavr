/*
10. Schreiben Sie ein Assemblerprogramm, das die Speicherstelle 0x0137 ausliest und prüft, ob der dort gespeicherte Wert eine gerade Zahl darstellt. 

Ist dies der Fall (gerade Zahl), so soll der Wert 0x55 an der Speicherstelle 0x0200 abgespeichert werden. 

Handelt es sich hingegen um eine ungerade Zahl, so soll der Wert 0x63 an die Speicherlokation 0x0200 geschrieben werden.
*/

	; X = 0x0137
	ldi r26, 0x37
	ldi r27, 0x01

	; Y = 0x0200
	ldi r28, 0x00
	ldi r29, 0x02

	; load from memory
	ld r16, X

	; check lsb to find if even or odd
	; the z-flag is set if the result is zero (= even number)
	; the z-flag is cleared if the result is non-zero (= odd number)
	andi r16, 0x01

	; Tests the Zero Flag (Z) and branches relatively to PC if Z is set. 
	breq is_even

is_odd:
	; constant to write into memory
	ldi r17, 0x63
	st Y, r17
	jmp end

is_even:
	; constant to write into memory
	ldi r17, 0x55
	st Y, r17

end:
	nop
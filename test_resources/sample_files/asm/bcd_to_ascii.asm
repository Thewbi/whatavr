; http://www.avr-asm-download.de/beginner_de.pdf
; In Ascii-Zeichenkette umwandeln
ToAsc:
 ldi ZH,High(sDecAsc) ; Zeiger auf ASCII Zeichenkette, MSB
 ldi ZL,Low(sDecAsc) ; dto., LSB
 ldi XH,High(cMantissa) ; Mantissenvorzeichen lesen
 ldi rmp,' ' ; Positiv
 lsl XH ; Vorzeichen in Carry
 brcc ToAsc1 ; Nicht negativ
 ldi rmp,'-' ; Vorzeichen negativ
ToAsc1:
 st Z+,rmp ; Vorzeichen in Zeichenkette
 ldi XH,High(sMant+1) ; X als Zeiger auf Dezimalmantisse, MSB
 ldi XL,Low(sMant+1) ; dto., LSB
 ld rmp,X+ ; Lese erste Mantissenziffer
 subi rmp,-'0' ; in ASCII
 st Z+,rmp ; und schreibe in Zeichenkette
 ldi rmp,',' ; Dezimalkomma
 st Z+,rmp ; in Zeichenkette
 ldi rCnt,MantLength-5
ToAsc2:
 ld rmp,X+ ; Lese Ziffer
 subi rmp,-'0' ; in ASCII
 st Z+,rmp ; in Zeichenkette
 dec rCnt ; Abwaerts zaehlen
 brne ToAsc2 ; Weitere Ziffern kopieren
 tst rDecExp ; Dezimalexponent = 0?
 breq ToAscExpBlk ; Vier Leerzeichen ausgeben
 ldi rmp,'E' ; Exponentenzeichen
 st Z+,rmp ; in Zeichenkette
 ldi rmp,'+' ; Vorzeichen Exponent
 sbrs rDecExp,7 ; Bei negativ ueberspringen
 rjmp ToAsc3 ; Exponent positiv
 neg rDecExp ; 0 minus Exponent
 ldi rmp,'-' ; Minuszeichen
ToAsc3:
st Z+,rmp ; Vorzeichen ausgeben
 ldi rmp,'0'-1 ; Exponentenzehner
ToAsc4:
 inc rmp ; Naechster Zehner
 subi rDecExp,10 ; 10 abziehen
 brcc ToAsc4
 st Z+,rmp ; Zehner ausgeben
 ldi rmp,'0'+10 ; Einer
 add rmp,rDecExp ; in ASCII
 st Z,rmp ; und in Zeichenkette
 ret
ToAscExpBlk:
 ldi rmp,' ' ; Leerzeichen
 ldi rCnt,4 ; Vier Stueck
ToAscExpBlk1:
 st Z+,rmp ; in Zeichenkette
 dec rCnt ; Abwaerts zaehlen
 brne ToAscExpBlk1 ; Weitere Zeichen
 ret

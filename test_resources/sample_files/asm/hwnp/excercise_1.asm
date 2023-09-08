; 1. Schreiben Sie ein Assemblerprogramm, dass zuerst den Inhalt von Register r20 auf
; Null zurücksetzt. Danach soll der Wert ´3´ zehnmal nacheinander zu diesem Register
; hinzu addiert werden, bevor anschließend das Ergebnis im Register r16 abspeichert
; wird. Benutzen Sie für ihre Umsetzung das Zero-bit sowie den Befehl brne.

start:
    ldi r20, 0x00 ; r20 auf 0 setzen
	ldi r21, 0x0A ; 0x0A == 10, also zehn Schleifendurchläufe
	ldi r22, 0x03 ; Wert 3 vorbereiten
loop:
	add r20, r22  ; add ist nur zwischen zwei Registern möglich
	dec r21       ; eine iteration weniger
	brne loop     ; wenn r21 den Wert 0 angenommen hat, dann spring brne nicht mehr

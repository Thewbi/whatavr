; test
ldi r16, LOW(RAMEND)
out SPL, r16
ldi r16, HIGH(RAMEND)
out SPH, r16
push r16
ldi r16, 2
push r16
ldi r16, 3
push r16
pop r0
pop r0
pop r0
ldi r16, 7
push r16
ldi r16, 8
push r16
ldi r16, 9
push r16
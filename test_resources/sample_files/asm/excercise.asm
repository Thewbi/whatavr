                .include "m328Pdef.inc"

				.dseg						; data segment == data memory: 0x0000 - 0x08FF
				.org 0x0160					; 0x0160
BUFFER: .db $33, $20, $31, $42, $43, $00, $66, $b2, $c2, $e4, $d3, $09, $22, $56, $91, $3e

				.cseg						; code segment == programm memory: 0x0000 - 0x3FFF
				.org 0x007e

                ; set stack pointer
set_stack_pointer:
                ldi r16, LOW(RAMEND)
                out SPL, r16
                ldi r16, HIGH(RAMEND)
                out SPH, r16

main:
 				ldi XH, HIGH(BUFFER)		; 007e B1E0
 				ldi XL, LOW(BUFFER)			; 0080 A0E6
 				call UP						; 0082 0E94
 				break						; 0086 9895
UP:
 				push r25					; 0100 9F93
 				clr r24						; 0102 8827
loop:
 				ld r25, X+					; 0104 9D91
 				inc r24						; 0106 8395
 				cpi r25, ' ' 				; 0108 9032 ; Leerzeichen = 0x20 !
 				brne loop					; 010a 01F4
 				ldi r25, '\n' 				; 010c 9AE0 ; Zeilenende = 0x0A !
 				st X+, r25					; 010e 9D93
 				clr r25						; 0110 9927
 				st X+, r25					; 0112 9D93
 				subi r24, -2				; 0114 8E5F
 				pop r25						; 0116 9F91
 				ret							; 0118 0895



ldi r21, 0x01
ldi r22, 0x02
add r21, r22

;				.dseg						; data segment == data memory (sram): 0x0000 - 0x08FF
;				.org 0x0160					; 0x0160
;BUFFER: .db $33, $20, $31, $42, $43, $00, $66, $b2, $c2, $e4, $d3, $09, $22, $56, $91, $3e

;                .cseg
;                .org 0x0000
;ldi XH, HIGH(BUFFER)
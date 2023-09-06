.include "m328Pdef.inc"
.cseg
.org 0x0000

teststring: .db "Hello world", 0

ldi ZH, HIGH(teststring * 2)
;teststring: .db "Hello world", 0
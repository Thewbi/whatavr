;ldi r25, 3 * (17 + 18)
;ldi r25, (1 + 2) * (17 + 18)
;ldi r25, (1 + (7 * 8)) * (17 + 18)
;ldi r25, 3 * (17 + +18)
;ldi r25, 3 * (17 + -18)
;ldi r25, 3 * (17 ++18)
;ldi r25, 3 * (17 +-18)

ldi ZH, HIGH(teststring * 2)

teststring: .db "Hello world", 0
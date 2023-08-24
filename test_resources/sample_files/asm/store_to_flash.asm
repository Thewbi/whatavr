; some data in flash memory

        .org 0x0004
data_0: .db 1, 8, 5, 35
data_2: .dw 286
data_4: .dw 0x398

        .equ mem_loc = 0x0220

    lds r30, mem_loc    ; liest die Speicherzelle mem_loc und
    tst r30             ; prüft auf Null ..

    brne here
    ldi r30, 0x55
    sts mem_loc, r30

here:
    rjmp here 

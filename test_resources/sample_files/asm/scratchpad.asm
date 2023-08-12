.include "m328Pdef.inc"

//.org 0x0000                         ; interrupt vector table ...
    jmp reset_handler
//.org PCI0addr                       ; @address 0x0006 in FLASH
    jmp pcint0_irq_handler

reset_handler:
    ldi r16, LOW(RAMEND)            ; init stack pointer ...
    out SPL, r16                    ;
    ldi r16, HIGH(RAMEND)           ;
    out SPH, r16                    ;

pcint0_irq_handler:
    out SPL, r16 
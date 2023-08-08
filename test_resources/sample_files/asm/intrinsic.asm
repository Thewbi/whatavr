.include "m328Pdef.inc"             ; useful definitions ...

.def PORTC_STATE = r18              ; some defines to improve the
.def BIT_MASK_0 = r19               ; readability ;-)
.def TMP_REG = r20

.cseg                               ; code segment starts here ...

.org 0x0000                         ; interrupt vector table ...
    jmp reset_handler
.org PCI0addr                       ; @address 0x0006 in FLASH
    jmp pcint0_irq_handler
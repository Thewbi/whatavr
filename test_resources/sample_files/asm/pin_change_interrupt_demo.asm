; ATmega328p ,Pin Change Interrupt' ASM Demo - Beliebige Pegeländerung an 
; den Pins PB0 und PB1 (beide in PCINT0 IRQ Gruppe) führt zum Ausläsen
; eines IRQ und zu einem Toggeln der an Pin PC0 angeschlossenen PED
; Zielplattform: ATMEL ATmega328p,  Arduino Uno R3 SMD Edition
; by Dr. C. Jakob, fbeit, h_da, Januar 2015, christian.jakob@h-da.de

.include "m328Pdef.inc"             ; useful definitions ...

.def PORTC_STATE = r18              ; some defines to improve the
.def BIT_MASK_0 = r19               ; readability ;-)
.def TMP_REG = r20

.cseg                               ; code segment starts here ...

.org 0x0000                         ; interrupt vector table ...
    jmp reset_handler
.org PCI0addr                       ; @address 0x0006 in FLASH
    jmp pcint0_irq_handler

reset_handler:
    ldi r16, LOW(RAMEND)            ; init stack pointer ...
    out SPL, r16                    ;
    ldi r16, HIGH(RAMEND)           ;
    out SPH, r16                    ;

    ldi r16, (1 << DDC0)            ;
    out DDRC, r16                   ;

    cbi DDRB, DDB0                  ;
    sbi PORTB, PORTBß               ;
    cbi DDRB, DDB1                  ;
    sbi PORTB, PORTB1               ;

    lds r24, PCICR                  ;
    ori r24, (1 << PCIE0)           ;
    sts PCICR, r24                  ; PCICR is Memory Mapped, therefore
                                    ; lds / sts ...
    lds r24, PCMSK0                 ;
    ori r24, (1 << PCINT0)          ;
    ori r24, (1 << PCINT1)          ;
    sts PCMSK0, r24                 ;
    ldi PORTC_STATE, 0x00           ;
    ldi BIT_MASK_0, 0x01            ;
    sei                             ; enable global interrupts
loop:                               ; endless loop, we simply do nothing
    rjmp loop                       ; in the main loop, just waiting for
                                    ; an IRQ to occur ...

pcint0_irq_handler:                 ; the famous interrupt service routine ...
    in TMP_REG, SREG                ; prolog..., we just store the SREG
    push TMP_REG                    ; (actually not necessary right here)
    eor PORTC_STATE, BIT_MASK_0     ; toggle Pin PC0 on every
    out PORTC, PORTC_STATE          ; interrupt
    pop TMP_REG                     ; epilog ...
    out SREG, TMP_REG
    reti
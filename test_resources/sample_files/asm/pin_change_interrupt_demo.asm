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

    ; configure Pin PC0 as an output for blinking an LED
    ldi r16, (1 << DDC0)            ; configure Pin PC0 as an output ...
    out DDRC, r16                   ; to drive the LED connected to that pin
                                    ; you could also use the sbi instructions
                                    ; DDRC = Data Direction Register for Port C

    ; Step 1 - make pins inputs
    ;
    ; configure pins PB0 and PB1 as inputs with pullups.
    ; This is required for external interrupts when the signal on the pins change
    cbi DDRB, DDB0                  ; configure Pin PB0 as an input ...
    sbi PORTB, PORTB0               ; and activate the pull-up resistor
    cbi DDRB, DDB1                  ; configure Pin PB1 as an input ...
    sbi PORTB, PORTB1               ; and activate the pull-up resistor

    ; Step 2 - set alternate function to PIN Change Interrupt
    ; Some pins can have many alternate functions. This step selects one of the alternate functions.
    ;
    ; activate pin change interrupts for all Pins in group PCINT[7:0]
    ; PCICR is Memory Mapped, therefore lds / sts ...
    ;
    ; PCICR = [P]in [C]hange [I]nterrupt [C]ontrol [R]egister
    lds r24, PCICR                  ; read PCICR into r24
    ori r24, (1 << PCIE0)           ; set bit PCIE0 in r24. PCIE0 is bit 0 in PCICR.
                                    ; When PCIE0 is 1 and global interrupts are enabled, then any change on 
                                    ; enabled pins in PCINT7..0 will cause an interrupt
                                    ; enabling pins PCINT0 and PCINT1 is done in the next section
    sts PCICR, r24                  ; write new value back into PCICR

    ; Step 3 - set a mask to define which pins cause the interrupt
    ;
    ; Set Pin Change Mask
    ; activate pin interrupts on PB0 and PB1
    ; could also be packed in a single instruction ...
    ; 
    ; PCMSK0 = [P]in [C]hange [M]a[sk] Register 0
    ; PCMSK0
    lds r24, PCMSK0                 ; read value from PCMSK0 into r24 so the value can be modified
    ori r24, (1 << PCINT0)          ; PCINT0 is enabled. Now any change on PB0 causes an interrupt (see Table 13-3)
    ori r24, (1 << PCINT1)          
    sts PCMSK0, r24                 ; Write back 
                                    ; PCMASK0 is also 'Memory Mapped' ...

    ldi PORTC_STATE, 0x00           ; initialization of some variables used
    ldi BIT_MASK_0, 0x01            ; in the ISR ...

    sei                             ; enable global interrupts

loop:                               ; endless loop, we simply do nothing
    rjmp loop                       ; in the main loop, just waiting for
                                    ; an IRQ to occur ...

pcint0_irq_handler:

    ; the famous interrupt service routine prolog
    ; we just store the SREG on the stack (actually not necessary right here)
    in TMP_REG, SREG
    push TMP_REG

    eor PORTC_STATE, BIT_MASK_0     ; toggle Pin PC0 on every
    out PORTC, PORTC_STATE          ; interrupt

    ; epilog ...
    pop TMP_REG                     ; pop initial SREG value from the stack into TMP_REG
    out SREG, TMP_REG               ; write from TMP_REG into SREG

    reti
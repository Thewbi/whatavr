
/*timebase.elf:     file format elf32-avr

Sections:
Idx Name          Size      VMA       LMA       File off  Algn
  0 .data         00000000  00800100  00800100  00000170  2**0
                  CONTENTS, ALLOC, LOAD, DATA
  1 .text         000000fc  00000000  00000000  00000074  2**1
                  CONTENTS, ALLOC, LOAD, READONLY, CODE
  2 .bss          00000001  00800100  00800100  00000170  2**0
                  ALLOC
  3 .comment      00000030  00000000  00000000  00000170  2**0
                  CONTENTS, READONLY
  4 .note.gnu.avr.deviceinfo 00000040  00000000  00000000  000001a0  2**2
                  CONTENTS, READONLY
  5 .debug_aranges 00000028  00000000  00000000  000001e0  2**0
                  CONTENTS, READONLY, DEBUGGING
  6 .debug_info   000006af  00000000  00000000  00000208  2**0
                  CONTENTS, READONLY, DEBUGGING
  7 .debug_abbrev 00000621  00000000  00000000  000008b7  2**0
                  CONTENTS, READONLY, DEBUGGING
  8 .debug_line   00000232  00000000  00000000  00000ed8  2**0
                  CONTENTS, READONLY, DEBUGGING
  9 .debug_frame  00000044  00000000  00000000  0000110c  2**2
                  CONTENTS, READONLY, DEBUGGING
 10 .debug_str    00000360  00000000  00000000  00001150  2**0
                  CONTENTS, READONLY, DEBUGGING
 11 .debug_loc    0000003c  00000000  00000000  000014b0  2**0
                  CONTENTS, READONLY, DEBUGGING
 12 .debug_ranges 00000018  00000000  00000000  000014ec  2**0
                  CONTENTS, READONLY, DEBUGGING
*/

; Disassembly of section .text:

; 00000000 <__vectors>:
   /* 0:	0c 94 34 00 */	jmp	0x68	; 0x68 <__ctors_end>
   /* 4:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
   /* 8:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
   /* c:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 10:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 14:	0c 94 46 00 */	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 18:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 1c:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 20:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 24:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 28:	0c 94 46 00 */	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 2c:	0c 94 6d 00 */ 	jmp	0xda	; 0xda <__vector_11>
  /* 30:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 34:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 38:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 3c:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 40:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 44:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 48:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 4c:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 50:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 54:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 58:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 5c:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 60:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>
  /* 64:	0c 94 46 00 */ 	jmp	0x8c	; 0x8c <__bad_interrupt>

; 00000068 <__ctors_end>:
  /* 68:	11 24 */       	eor	r1, r1
  /* 6a:	1f be */       	out	0x3f, r1	; 63
  /* 6c:	cf ef */       	ldi	r28, 0xFF	; 255
  /* 6e:	d8 e0 */       	ldi	r29, 0x08	; 8
  /* 70:	de bf */       	out	0x3e, r29	; 62
  /* 72:	cd bf */       	out	0x3d, r28	; 61

/* ; 00000074 <__do_clear_bss>:
  74:	21 e0       	ldi	r18, 0x01	; 1
  76:	a0 e0       	ldi	r26, 0x00	; 0
  78:	b1 e0       	ldi	r27, 0x01	; 1
  7a:	01 c0       	rjmp	.+2      	; 0x7e <.do_clear_bss_start>

; 0000007c <.do_clear_bss_loop>:
  7c:	1d 92       	st	X+, r1

; 0000007e <.do_clear_bss_start>:
  7e:	a1 30       	cpi	r26, 0x01	; 1
  80:	b2 07       	cpc	r27, r18
  82:	e1 f7       	brne	.-8      	; 0x7c <.do_clear_bss_loop>
  84:	0e 94 48 00 	call	0x90	; 0x90 <main>
  88:	0c 94 7c 00 	jmp	0xf8	; 0xf8 <_exit>

; 0000008c <__bad_interrupt>:
  8c:	0c 94 00 00 	jmp	0	; 0x0 <__vectors>

; 00000090 <main>:
; unsigned char handshake = 0;

; int main(void)
; {
; 	// Configuration of Status-LED
; 	DDRB |= (1 << PB5);
  90:	84 b1       	in	r24, 0x04	; 4
  92:	80 62       	ori	r24, 0x20	; 32
  94:	84 b9       	out	0x04, r24	; 4
; 	PORTB &= ~(1 << PB5);
  96:	85 b1       	in	r24, 0x05	; 5
  98:	8f 7d       	andi	r24, 0xDF	; 223
  9a:	85 b9       	out	0x05, r24	; 5
	// Configuration of Timer T/C1
	
	// OCR1A - Output Compare Register 1 A, wird mit TCNT1 Register verglichen. 
	// Hier wird ein Wert gesetzt. Der Interrupt wird ausgelöst, wenn TIMSK1{OCIE1A} && (OCR1A == TCNT1)
	// also wenn das Aktivierungs-Bit TIMSK1{OCIE1A} gesetzt ist und wenn dann nach einiger Zeit der Timer den Wert OCR1A erreicht hat
; 	OCR1A = 0x3D08;				
  9c:	88 e0       	ldi	r24, 0x08	; 8
  9e:	9d e3       	ldi	r25, 0x3D	; 61
  a0:	90 93 89 00 	sts	0x0089, r25	; 0x800089 <__TEXT_REGION_LENGTH__+0x7f8089>
  a4:	80 93 88 00 	sts	0x0088, r24	; 0x800088 <__TEXT_REGION_LENGTH__+0x7f8088>
	// Diese bits sind über TCCR1A und TCCR1B verteilt.
	// Es gibt eine Tabelle mit 16 Einträgen, die zeigt, welche 4-Bit Kombination, welchen der 16 Modi aktiviert.
	// der Modus {WGM13, WGM12, WGM11, WGM10} == 0100 aktiviert den Modus CTC. 
	// CTC verwendet OCR1A als TOP register.
	// CTC steht für (15.9.2 Clear Timer on Compare Match (CTC) Mode)
; 	TCCR1B |= (1 << WGM12);	
  a8:	e1 e8       	ldi	r30, 0x81	; 129
  aa:	f0 e0       	ldi	r31, 0x00	; 0
  ac:	80 81       	ld	r24, Z
  ae:	88 60       	ori	r24, 0x08	; 8
  b0:	80 83       	st	Z, r24
	
	// TIMSK1 - Timer/Counter1 Interrupt Mask Register TIMSK1. 
	// Das Bit TIMSK1{OCIE1A} aktiviert den interrupt: TCNT1 gleich dem Vergleichswert OCR1A	
; 	TIMSK1 |= (1 << OCIE1A);
  b2:	af e6       	ldi	r26, 0x6F	; 111
  b4:	b0 e0       	ldi	r27, 0x00	; 0
  b6:	8c 91       	ld	r24, X
  b8:	82 60       	ori	r24, 0x02	; 2
  ba:	8c 93       	st	X, r24
	// Prescale-Wert konfigurieren:
	//
	//TCCR1B |= (1 << CS12) | (1 << CS11) | (1 >> CS10); // directly from system clock
	//TCCR1B |= (0 << CS12) | (0 << CS11) | (1 >> CS10); // kein Prescale
	//TCCR1B |= (0 << CS12) | (1 << CS11) | (0 >> CS10); // Prescale auf 8
; 	TCCR1B |= (0 << CS12) | (1 << CS11) | (1 >> CS10); // Prescale auf 64
  bc:	80 81       	ld	r24, Z
  be:	83 60       	ori	r24, 0x03	; 3
  c0:	80 83       	st	Z, r24
	//TCCR1B |= (1 << CS12) | (0 << CS11) | (1 >> CS10); // Prescale auf 1024
	
	// CSX2, wobei X hier 1 ist, also Timer 1. CS12 == 1, CS11 == 0 und CS10 == 1 bedeutet: Prescale auf 1024
	//TCCR1B |= (1 << CS12) | (0 << CS11) | (1 >> CS10);
	
; 	sei();
  c2:	78 94       	sei

; 	while(1)
; 	{
; 		while(handshake == 0);
  c4:	80 91 00 01 	lds	r24, 0x0100	; 0x800100 <__DATA_REGION_ORIGIN__>
  c8:	88 23       	and	r24, r24
  ca:	e1 f3       	breq	.-8      	; 0xc4 <main+0x34>
		
; 		PORTB ^= (1 << PB5);
  cc:	95 b1       	in	r25, 0x05	; 5
  ce:	80 e2       	ldi	r24, 0x20	; 32
  d0:	89 27       	eor	r24, r25
  d2:	85 b9       	out	0x05, r24	; 5
		
		// reset the handshake variable
; 		handshake = 0;
  d4:	10 92 00 01 	sts	0x0100, r1	; 0x800100 <__DATA_REGION_ORIGIN__>
; 	}
  d8:	f5 cf       	rjmp	.-22     	; 0xc4 <main+0x34>

; 000000da <__vector_11>:
; }

; ISR (TIMER1_COMPA_vect){
  da:	1f 92       	push	r1
  dc:	0f 92       	push	r0
  de:	0f b6       	in	r0, 0x3f	; 63
  e0:	0f 92       	push	r0
  e2:	11 24       	eor	r1, r1
  e4:	8f 93       	push	r24
; 	handshake = 1;
  e6:	81 e0       	ldi	r24, 0x01	; 1
  e8:	80 93 00 01 	sts	0x0100, r24	; 0x800100 <__DATA_REGION_ORIGIN__>
; }
  ec:	8f 91       	pop	r24
  ee:	0f 90       	pop	r0
  f0:	0f be       	out	0x3f, r0	; 63
  f2:	0f 90       	pop	r0
  f4:	1f 90       	pop	r1
  f6:	18 95       	reti

; 000000f8 <_exit>:
  f8:	f8 94       	cli

; 000000fa <__stop_program>:
  fa:	ff cf       	rjmp	.-2      	; 0xfa <__stop_program> 
  */

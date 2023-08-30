;
; CALL and RET
;

.include "m328Pdef.inc"

; initialize the stack
; encode_ldi(&mut assembler_segment, &mut idx, 16u16, LOW!(RAMEND));
; encode_out(&mut assembler_segment, &mut idx, IO_Destination::SPL, 16u16);
; encode_ldi(&mut assembler_segment, &mut idx, 16u16, HIGH!(RAMEND));
; encode_out(&mut assembler_segment, &mut idx, IO_Destination::SPH, 16u16);
ldi r16, LOW(RAMEND)
out SPL, r16
ldi r16, HIGH(RAMEND)
out SPH, r16

; encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x01);
; encode_ldi(&mut assembler_segment, &mut idx, 17u16, 0x02);
ldi r16, 1
ldi r17, 2

; this label should be on the same line as the first add command but
; currently there is no assembler that can detect labels that lie ahead
;create_label(&mut labels, String::from("addReg"), idx + 4); ; call is a 4 byte instruction
;encode_call(&mut assembler_segment, &mut idx, &labels, &String::from("addReg"));
call addReg
;;create_label(&mut labels, String::from("addReg"), idx);
;encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
;encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
;encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
;encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
;encode_ret(&mut assembler_segment, &mut idx);

; this code will not be executed because the call statement will jump to the addReg label
ldi r16, 3
push r16
pop r0

end: jmp end

addReg: add r17, r16
add r17, r16
add r17, r16
add r17, r16
ret
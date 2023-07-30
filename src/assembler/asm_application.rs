use crate::{instructions::instruction_type::InstructionType, assembler::io_destination::IoDestination};

use super::asm_record::AsmRecord;

// let mut asm_record:AsmRecord = AsmRecord::new(String::new(), 
//     InstructionType::LDI, 
//     16u16, 
//     0, 
//     LOW!(RAMEND),
//     String::new(),
//     IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record);

//
// BRNE
//

//     ldi r16, 7
// loop:
//     -- Block of code
//     dec r16
//     brne loop

//       ldi r16, 7
// loop: dec r16
//       brne loop (in this case, k is -1)

// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 7u16);
// create_label(&mut labels, String::from("loop"), idx);
// encode_dec(&mut assembler_segment, &mut idx, 16u16);
// encode_brne(&mut assembler_segment, &mut idx, &labels, &String::from("loop"));

//
// ADD
//

//          ldi r16, 1
//          ldi r17, 1
//          add r16, r17
// loop:    jmp loop

// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 1u16);
// encode_ldi(&mut assembler_segment, &mut idx, 17u16, 1u16);
// encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
// create_label(&mut labels, String::from("loop"), idx);
// encode_jmp(&mut assembler_segment, &mut idx, &labels, &String::from("loop"));

//
// JMP
//

// // will overflow the u8 datatype, on the real hardware, this would be an endless loop
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 1u16);
// encode_ldi(&mut assembler_segment, &mut idx, 17u16, 1u16);
// create_label(&mut labels, String::from("loop"), idx);
// encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
// encode_jmp(&mut assembler_segment, &mut idx, &labels, &String::from("loop"));

//
// PUSH and POP
//

// // initialize the stack
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, LOW!(RAMEND));
// encode_out(&mut assembler_segment, &mut idx, IO_Destination::SPL, 16u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, HIGH!(RAMEND));
// encode_out(&mut assembler_segment, &mut idx, IO_Destination::SPH, 16u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x01u16);
// encode_push(&mut assembler_segment, &mut idx, 16u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x02u16);
// encode_push(&mut assembler_segment, &mut idx, 16u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x03u16);
// encode_push(&mut assembler_segment, &mut idx, 16u16);
// encode_pop(&mut assembler_segment, &mut idx, 0u16);
// encode_pop(&mut assembler_segment, &mut idx, 0u16);
// encode_pop(&mut assembler_segment, &mut idx, 0u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x07u16);
// encode_push(&mut assembler_segment, &mut idx, 16u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x08u16);
// encode_push(&mut assembler_segment, &mut idx, 16u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x09u16);
// encode_push(&mut assembler_segment, &mut idx, 16u16);

//
// CALL and RET
//

// // initialize the stack
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, LOW!(RAMEND));
// encode_out(&mut assembler_segment, &mut idx, IO_Destination::SPL, 16u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, HIGH!(RAMEND));
// encode_out(&mut assembler_segment, &mut idx, IO_Destination::SPH, 16u16);

// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x01);
// encode_ldi(&mut assembler_segment, &mut idx, 17u16, 0x02);

// // this label should be on the same line as the first add command but
// // currently there is no assembler that can detect labels that lie ahead
// create_label(&mut labels, String::from("addReg"), idx + 4); // call is a 4 byte instruction
// encode_call(&mut assembler_segment, &mut idx, &labels, &String::from("addReg"));
// //create_label(&mut labels, String::from("addReg"), idx);
// encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
// encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
// encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
// encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
// encode_ret(&mut assembler_segment, &mut idx);

// let mut asm_record_1:AsmRecord = AsmRecord::new(String::new(), InstructionType::LDI, 16u16, 0, LOW!(RAMEND), String::new(), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_1);
// let mut asm_record_2:AsmRecord = AsmRecord::new(String::new(), InstructionType::OUT, 16u16, 0, 0, String::new(), IoDestination::SPL);
// asm_records.push(&mut asm_record_2);
// let mut asm_record_3:AsmRecord = AsmRecord::new(String::new(), InstructionType::LDI, 16u16, 0, HIGH!(RAMEND), String::new(), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_3);
// let mut asm_record_4:AsmRecord = AsmRecord::new(String::new(), InstructionType::OUT, 16u16, 0, 0, String::new(), IoDestination::SPH);
// asm_records.push(&mut asm_record_4);

// let mut asm_record_5:AsmRecord = AsmRecord::new(String::new(), InstructionType::LDI, 16u16, 0, 0x01, String::new(), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_5);
// let mut asm_record_6:AsmRecord = AsmRecord::new(String::new(), InstructionType::LDI, 17u16, 0, 0x02, String::new(), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_6);

// let mut asm_record_7:AsmRecord = AsmRecord::new(String::new(), InstructionType::CALL, 16u16, 0, 0, String::from("addReg"), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_7);

// let mut asm_record_8:AsmRecord = AsmRecord::new(String::from("addReg"), InstructionType::ADD, 16u16, 17u16, 0, String::new(), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_8);
// let mut asm_record_9:AsmRecord = AsmRecord::new(String::new(), InstructionType::ADD, 16u16, 17u16, 0, String::new(), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_9);
// let mut asm_record_10:AsmRecord = AsmRecord::new(String::new(), InstructionType::ADD, 16u16, 17u16, 0, String::new(), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_10);
// let mut asm_record_11:AsmRecord = AsmRecord::new(String::new(), InstructionType::ADD, 16u16, 17u16, 0, String::new(), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_11);

// let mut asm_record_12:AsmRecord = AsmRecord::new(String::new(), InstructionType::RET, 0, 0, 0, String::new(), IoDestination::UNKNOWN);
// asm_records.push(&mut asm_record_12);

//
// RJMP
//

// create_label(&mut labels, String::from("target"), idx);
// encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
// encode_rjmp(&mut assembler_segment, &mut idx, &labels, &String::from("target"));

//
// RCALL
//

// // initialize the stack
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, LOW!(RAMEND));
// encode_out(&mut assembler_segment, &mut idx, IO_Destination::SPL, 16u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, HIGH!(RAMEND));
// encode_out(&mut assembler_segment, &mut idx, IO_Destination::SPH, 16u16);

// create_label(&mut labels, String::from("main"), idx);
// encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
// encode_rcall(&mut assembler_segment, &mut idx, &labels, &String::from("main"));

//
// MOV
//

// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x01);
// encode_ldi(&mut assembler_segment, &mut idx, 17u16, 0x02);
// encode_mov(&mut assembler_segment, &mut idx, 16u16, 17u16);

//
// Example
//

// initialize the stack
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, LOW!(RAMEND));
// encode_out(&mut assembler_segment, &mut idx, IoDestination::SPL, 16u16);
// encode_ldi(&mut assembler_segment, &mut idx, 16u16, HIGH!(RAMEND));
// encode_out(&mut assembler_segment, &mut idx, IoDestination::SPH, 16u16);

// create_label(&mut labels, String::from("main"), idx); // main:
// encode_rjmp(&mut assembler_segment, &mut idx, &labels, &String::from("reset"));  // rjmp reset

// create_label(&mut labels, String::from("swap"), idx); // swap:
// encode_push(&mut assembler_segment, &mut idx, 16u16); // push r18
// encode_mov(&mut assembler_segment, &mut idx, 18u16, 16u16); // mov r18, r16
// encode_mov(&mut assembler_segment, &mut idx, 16u16, 17u16); // mov r16, r17
// encode_mov(&mut assembler_segment, &mut idx, 17u16, 18u16); // mov r17, r18
// encode_pop(&mut assembler_segment, &mut idx, 18u16); // pop r18
// encode_ret(&mut assembler_segment, &mut idx); // ret

// create_label(&mut labels, String::from("reset"), idx);  // reset:
// encode_ldi(&mut assembler_segment, &mut idx, 18u16, 0x21); // ldi r18, 33d == 0x21

// encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x0B); // ldi r16, 11
// encode_ldi(&mut assembler_segment, &mut idx, 17u16, 0x16); // ldi r17, 22
// encode_rcall(&mut assembler_segment, &mut idx, &labels, &String::from("swap"));
// encode_rjmp(&mut assembler_segment, &mut idx, &labels, &String::from("main"));

//
// Example app
//

//pub fn application_instruction_source(asm_records: &mut Vec<&mut AsmRecord>) {
pub fn application_instruction_source(asm_records: &mut Vec<AsmRecord>) {

    // ldi r16, 0xFF
    let asm_record_1:AsmRecord = AsmRecord::new(String::new(), InstructionType::LDI, 16u16, 0, 0xFF, String::new(), IoDestination::UNKNOWN);
    asm_records.push(asm_record_1.clone());

    // out DDRB, r16
    const DDRB: u8 = 0x24;
    let asm_record_2:AsmRecord = AsmRecord::new(String::new(), InstructionType::OUT, 16u16, 0, DDRB as u16, String::new(), IoDestination::DDRB);
    asm_records.push(asm_record_2.clone());

    // read the Data Direction Register for Port B into r0 using the in instruction
    // in	r1, DDRB
    let asm_record_3:AsmRecord = AsmRecord::new(String::new(), InstructionType::IN, 1u16, 0, DDRB as u16, String::new(), IoDestination::DDRB);
    asm_records.push(asm_record_3.clone());
}
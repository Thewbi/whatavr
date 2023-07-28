mod ihex_mgmt;
mod file_mgmt;
mod instructions;
mod cpu;
mod assembler;

use std::io;
use std::io::Write;
use std::io::Cursor;
use std::collections::HashMap;

use env_logger::{Builder, Target};
use instructions::instruction_definition::InstructionDefinition;
use log::LevelFilter;

use crate::assembler::asm_encoder::AsmEncoder;
use crate::assembler::asm_record::AsmRecord;
use crate::assembler::io_destination::IoDestination;
use crate::cpu::cpu::CPU;
use crate::cpu::cpu::RAMEND;
use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;
use crate::instructions::decode::decode_instruction;
use crate::instructions::instruction_type::InstructionType;
use crate::instructions::instructions::instructions;
use crate::instructions::instructions::unknown;
use crate::instructions::process::*;

use byteorder::{LittleEndian, ReadBytesExt};

// the value is placed at the stackpointer then, after that, the stack pointer is decremented
pub fn push_to_stack_u8(cpu: &mut CPU, data: u8) 
{
    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = data;

    decrement_stack_pointer(cpu);
}

pub fn push_to_stack_u16(cpu: &mut CPU, data: &u16) 
{
    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);
}

pub fn push_to_stack_i16(cpu: &mut CPU, data: i16) 
{
    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH_I16!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW_I16!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);
}

pub fn push_to_stack_i32(cpu: &mut CPU, data: i32) 
{
    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH_HIGH_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW_LOW_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);
}

pub fn pop_from_stack_u8(cpu: &mut CPU) -> u8 {

    increment_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    let data: u8 = cpu.sram[(stack_pointer - 1) as usize];

    data
}

fn increment_stack_pointer(cpu: &mut CPU)
{
    let mut stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    stack_pointer += 1u16;
    cpu.sph = HIGH!(stack_pointer).try_into().unwrap();
    cpu.spl = LOW!(stack_pointer).try_into().unwrap();
}

fn decrement_stack_pointer(cpu: &mut CPU)
{
    let mut stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    stack_pointer -= 1u16;
    cpu.sph = HIGH!(stack_pointer).try_into().unwrap();
    cpu.spl = LOW!(stack_pointer).try_into().unwrap();
}

pub fn create_label(labels:&mut HashMap<String, usize>, label: String, idx: usize)
{
    labels.insert(label, idx);
}

fn main() -> io::Result<()> {

    println!("whatavr starting ...");

    // logging setup
    init_logging();
    log_start();

    let hex: bool = false;
    if hex {
        // load hex file
        let mut hex_file_path:String = String::new();
        //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank1.hex");
        //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank2.hex") {
        //hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication1/GccApplication1.hex");
        hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication2/GccApplication1.hex");
        //hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/arduboy/Ardynia/ardynia.hex");

        // split into segments
        // each segment has to have a segment_start and a segment_size
        let mut segments: Vec<Segment> = Vec::new();
        match parse_hex_file(&mut segments, &hex_file_path) {
            Ok(_name) => log::info!("File read"),
            Err(err) => {
                log::error!("An error occured while retrieving the peername: {:?}", err);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error at load hex file!"));
            }
        }

        // // DEBUG dump parsed segments
        // for seg in segments.iter_mut() {
        //     log::info!("Segment: {}", seg);
        // }

        // process the first segment only
        let ref segment_0: &Segment = &segments[0];
        log::info!("Segment: {}", segment_0);

        let mut index: usize = 0;

        // TODO output disassembly so that a comparison to the .lss is possible
        // TODO build records from lines so that parsing a .lss file produces the exact same output
        // TODO build an executor for records
        //
        // Set PC to 0
        // loop:
        //  Fetch instruction from PC
        //  increment PC by 2
        //  if the current instruction is a jump, set instruction pointer to jump destination
        // goto loop

        //
        // disassenble the entire segment
        //

        const DISSASSEMBLE: bool = false;
        if DISSASSEMBLE {

            let mut rdr = Cursor::new(&segment_0.data);
            while index < segment_0.data.len()
            {
                let word:u16 = rdr.read_u16::<LittleEndian>().unwrap().into();
                index += 2;

                log::trace!("word: {:#06x} {:b}", word, word);

                let mut value_storage:HashMap<char, u16> = HashMap::new();
                let instruction: &InstructionDefinition = decode_instruction(word, instructions, &unknown, &mut value_storage);

                log::info!("instruction {:?}", instruction.instruction_type);
                if instruction.instruction_type == InstructionType::EOR || instruction.instruction_type == InstructionType::CLR {
                    log::info!("EOR and CLR similar. CLI is implemented by EOR the register with itself!");
                }

                match_instruction(&instruction, &mut rdr, &word, &mut index, &mut value_storage);
            }

        }
    }

    let mut cpu: CPU = CPU {
        z: false,
        sph: 0x00u8,
        spl: 0x00u8,
        pc: 0x02i32,
        register_file: [0; 32usize],
        sram: [0; RAMEND as usize],
    };

    const EXECUTE: bool = true;
    if EXECUTE {

        // 1. enter all commands into a list
        // 2. resolve all macros and insert new entries (created from the resolved macros) into the list
        // 3. go through the list of all commands when a label is found, insert the label into a map along with the current idx
        //    but do not encode any command in this phase
        // 4. got through the list of commands and call encode for each command using the table of resolved labels
        //    but this time ignore the creation of labels and do not insert the labels int the map any more since they are already resolved in phase 1

        // 1. Add a cycle counter

        let mut assembler_segment: Segment = Segment::new();
        assembler_segment.address = 0u16;
        assembler_segment.size = 0u32;

        let mut asm_records: Vec<&mut AsmRecord> = Vec::new();

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
        // Example app
        //

        // ldi r16, 0xFF
        let mut asm_record_1:AsmRecord = AsmRecord::new(String::new(), InstructionType::LDI, 16u16, 0, 0xFF, String::new(), IoDestination::UNKNOWN);
        asm_records.push(&mut asm_record_1);

        // out DDRB, r16
        const DDRB: u8 = 0x24;
        let mut asm_record_2:AsmRecord = AsmRecord::new(String::new(), InstructionType::OUT, 16u16, 0, DDRB as u16, String::new(), IoDestination::DDRB);
        asm_records.push(&mut asm_record_2);

        // read the Data Direction Register for Port B into r0 using the in instruction
        // in	r1, DDRB
        let mut asm_record_3:AsmRecord = AsmRecord::new(String::new(), InstructionType::IN, 1u16, 0, DDRB as u16, String::new(), IoDestination::DDRB);
        asm_records.push(&mut asm_record_3);
        

        

        // let mut asm_record:AsmRecord = AsmRecord::new(String::new(), 
        //     InstructionType::LDI, 
        //     16u16, 
        //     0, 
        //     LOW!(RAMEND),
        //     String::new(),
        //     IoDestination::UNKNOWN);
        // asm_records.push(&mut asm_record);

        let mut asm_encoder:AsmEncoder = AsmEncoder::new();

        //
        // phase 1 - scan for labels
        //

        let mut idx: usize = 0usize;
        for rec in asm_records.iter_mut() {

            // assign the current address to the record
            rec.idx = idx;

            // if a label was specified for the current address,
            // manage the mapping of the label to the current address
            if rec.label != "" {
                create_label(&mut asm_encoder.labels, rec.label.clone(), idx);
            }

            // advance the address by the actual length of the instruction.
            // Some instructions are 1 word (2 byte) whereas others are 2 word (4 byte)
            idx += InstructionType::words(&rec.instruction_type);
        }

        //
        // phase 2 - encode (with addresses resolved to labels)
        //
        
        for rec in asm_records.iter() {
            asm_encoder.encode(&mut assembler_segment, rec);
        }

        // // initialize the stack
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

        let done: bool = false;
        while !done {

            // get the current instruction
            let temp_pc:i32 = cpu.pc - 0x02;

            // check for end of code
            if assembler_segment.size <= temp_pc as u32 {
                log::info!("End of Code reached! Application Finished!");
                log_end();
                return Ok(());
            }

            let word_hi:u16 = assembler_segment.data[(temp_pc + 1i32) as usize].into();
            let word_lo:u16 = assembler_segment.data[temp_pc as usize].into();
            let word:u16 = (word_hi << 8u8) + word_lo;
            
            // decode the current instruction
            let mut value_storage:HashMap<char, u16> = HashMap::new();
            let instruction: &InstructionDefinition = decode_instruction(word, 
                instructions, &unknown, &mut value_storage);

            // execute the instruction
            match instruction.instruction_type {

                /*   6 */ 
                InstructionType::ADD => { 
                    log::info!("[ADD]");

                    let r_valu16 = value_storage[&'r'];
                    let d_valu16 = value_storage[&'d'];

                    cpu.register_file[d_valu16 as usize] += cpu.register_file[r_valu16 as usize];

                    log::info!("[ADD] result value: {}", cpu.register_file[d_valu16 as usize]);

                    cpu.pc += 2i32;
                },

                /*  13 */ 
                InstructionType::BRBC => { 
                    log::info!("[BRBC]");

                    //process_brbc(rdr, &word, index, value_storage);

                    let k_val:i8 = value_storage[&'k'] as i8;
                    //log::info!("K: {k_val:#b} {k_val:#x}");
                    let s_val = value_storage[&'s'];
                    //log::info!("d: {d_val:#b} {d_val:#x}");

                    // twos-complement
                    let offset:i8 = (k_val as i8 * -1i8) as i8;

                    // todo find a better way to deal with the SREG array of status bits
                    if s_val == 1 {
                        // zero flag 

                        // same code as BRNE

                        // check the Z-flag
                        if cpu.z {
                            
                            cpu.pc += 2i32;

                        } else {
                            
                            cpu.pc = (cpu.pc as i16 + offset as i16) as i32;

                        }

                    }
                },

                /*  27 */ 
                InstructionType::BRNE => { 
                    log::info!("[BRNE]");

                    let offset:i32 = value_storage[&'k'] as i32;

                    // check the Z-flag
                    if cpu.z {
                        cpu.pc += offset;
                    } else {
                        cpu.pc += 2i32;
                    }
                },

                /* 36 */ 
                InstructionType::CALL => {

                    log::info!("[CALL]");

                    // get the first 16 bit
                    let k_hi:u32 = value_storage[&'k'].into();

                    // get the next 16 stored at the pc since the JMP command is encoded using 32 bit (4 byte)
                    let temp_pc:i32 = cpu.pc;

                    let word_hi:u16 = assembler_segment.data[(temp_pc + 1i32) as usize] as u16;
                    //log::info!("READ: {:#02x}", word_hi as u8);

                    let word_lo:u16 = assembler_segment.data[temp_pc as usize] as u16;
                    //log::info!("READ: {:#02x}", word_lo as u8);

                    let k_lo:u32 = ((word_hi << 8u8) + word_lo).into();

                    // assemble the parameter k
                    let k_val:i16 = ((k_hi << 16u8) + k_lo) as i16;

                    // sign extend to i32
                    let k_val_i32:i32 = k_val as i32;

                    // push return address onto the stack 
                    let data = cpu.pc;
                    push_to_stack_i16(&mut cpu, data as i16);

                    log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                    
                    // jump to address
                    cpu.pc += k_val_i32;
                }

                // /*  41 */ 
                // InstructionType::CLI => { 
                //     let k_val = value_storage[&'K'];
                //     //log::info!("K: {k_val:#b} {k_val:#x}");
                //     let d_val = value_storage[&'d'];
                //     //log::info!("d: {d_val:#b} {d_val:#x}");

                //     // "Loads an 8-bit constant directly to register 16 to 31."
                //     // To compute the register to use, add the offset 16 to the parsed value
                //     //let register = d_val + 16;
                //     let register = d_val;
                    
                //     //log::info!("[LDI] Using register: r{}", register);

                //     log::info!("{temp_pc:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}");

                //     pc += 2u32;
                // },

                /*  43 */ 
                InstructionType::CLR => { 
                    log::info!("[CLR]");

                    let d:u16 = value_storage[&'d'];
                    
                    //log::info!("Clearing register d: {:#06x}", d);
                    cpu.register_file[d as usize] = 0x00;

                    cpu.pc += 2i32;
                },

                /*  53 */ 
                InstructionType::DEC => { 
                    log::info!("[DEC]");

                    let d:u16 = value_storage[&'d'];
                    
                    // perform the decrement
                    cpu.register_file[d as usize] -= 0x01u8;

                    // set the z flag
                    if cpu.register_file[d as usize] == 0x00u8 {
                        cpu.z = true;
                    }

                    log::info!("[DEC] Register r{}: value at reg:{:#06x}", d, cpu.register_file[d as usize]);

                    cpu.pc += 2i32;
                },

                /*  64 */ 
                InstructionType::IN => { 
                    log::info!("[IN]");

                    let register_d:u16 = value_storage[&'d'];
                    let address:u16 = value_storage[&'A'];
                    
                    let val: u8 = cpu.sram[address as usize];

                    log::info!("[IN] value from read-operation: {:#06x}", val);

                    cpu.register_file[register_d as usize] = val as u8;

                    log::info!("[IN] Register r{}: value at reg:{:#06x}", register_d, cpu.register_file[register_d as usize]);

                    cpu.pc += 2i32;
                },
            
                /*  66 */ 
                InstructionType::JMP => {

                    log::info!("[JMP]");

                    // get the first 16 bit
                    let k_hi:u32 = value_storage[&'k'].into();

                    // get the next 16 stored at the pc since the JMP command is encoded using 32 bit (4 byte)
                    let temp_pc:i32 = cpu.pc;

                    let word_hi:u16 = assembler_segment.data[(temp_pc + 1i32) as usize] as u16;
                    //log::info!("READ: {:#02x}", word_hi as u8);

                    let word_lo:u16 = assembler_segment.data[temp_pc as usize] as u16;
                    //log::info!("READ: {:#02x}", word_lo as u8);

                    let k_lo:u32 = ((word_hi << 8u8) + word_lo).into();

                    // assemble the parameter k
                    let k_val:i16 = ((k_hi << 16u8) + k_lo) as i16;

                    // sign extend to i32
                    let k_val_i32:i32 = k_val as i32;
                    
                    cpu.pc += k_val_i32;
                },

                /*  73 */ 
                InstructionType::LDI => { 
                    log::info!("[LDI]");

                    let k_val = value_storage[&'K'];
                    //log::info!("K: {k_val:#b} {k_val:#x}");
                    let d_val = value_storage[&'d'];
                    //log::info!("d: {d_val:#b} {d_val:#x}");

                    // "Loads an 8-bit constant directly to register 16 to 31."
                    // To compute the register to use, add the offset 16 to the parsed value
                    let register = d_val + 16;
                    //log::info!("[LDI] Using register: r{}", register);

                    log::info!("{temp_pc:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}");

                    // execute
                    cpu.register_file[register as usize] = k_val as u8;

                    cpu.pc += 2i32;
                },

                /*  79 */
                InstructionType::MOV => {
                    log::info!("[MOV]");

                    let r_register = value_storage[&'r'];
                    //log::info!("K: {r_register:#b} {r_register:#x}");
                    let d_register = value_storage[&'d'];
                    //log::info!("d: {d_register:#b} {d_register:#x}");

                    let k_val: u8 = cpu.register_file[r_register as usize];

                    cpu.register_file[d_register as usize] = k_val as u8;

                    cpu.pc += 2i32;
                }

                /*  88 */ 
                InstructionType::OUT => {
                    log::info!("[OUT]");

                    // Stores data from register Rr in the Register File to I/O Space (Ports, Timers, Configuration Registers, etc.).
                    let a_val: u16 = value_storage[&'A'];
                    log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
                    let r_val = value_storage[&'r'];
                    log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

                    // TODO output the value stored in register r_val into memory to the address a_val
                    let dest: IoDestination = IoDestination::from_code(a_val);
                    log::info!("dest: {:?}", dest);

                    match dest {

                        IoDestination::SPH => {
                            log::info!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                            let val:u8 = cpu.register_file[r_val as usize];
                            log::info!("val: {val:#b} {val:#x} {val}");
                            cpu.sph = val;

                            log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                        }

                        IoDestination::SPL => {
                            log::info!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                            let val:u8 = cpu.register_file[r_val as usize];
                            log::info!("val: {val:#b} {val:#x} {val}");
                            cpu.spl = val;

                            log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                        }

                        IoDestination::DDRB | IoDestination::PORTB | IoDestination::PINB => {
                            log::info!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                            let val:u8 = cpu.register_file[r_val as usize];
                            log::info!("val: {val:#b} {val:#x} {val}");

                            // write the value into SRAM
                            cpu.sram[a_val as usize] = val;
                        }

                        IoDestination::UNKNOWN => {
                            panic!("UNKNOWN enum value!")
                        }

                        // _ => panic!("Unknown enum value!")
                    }

                    cpu.pc += 2i32;
                },

                /*  89 */ 
                InstructionType::POP => { 
                    log::info!("[POP]");

                    let d_val: u16 = value_storage[&'d'];
                    log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                    cpu.pc += 2i32;

                    pop_from_stack_u8(&mut cpu);

                    log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                },

                /*  90 */ 
                InstructionType::PUSH => { 
                    log::info!("[PUSH]");

                    let d_val: u16 = value_storage[&'d'];
                    log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                    cpu.pc += 2i32;

                    let data = cpu.register_file[d_val as usize];
                    push_to_stack_u8(&mut cpu, data);

                    log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                },

                /* 91 */ 
                InstructionType::RCALL => { 
                    log::info!("[RCALL]");

                    // get the first 16 bit
                    let mut k: u16 = value_storage[&'k'] as u16;

                    log::info!("k: {:04x} {:016b}", k, k);

                    // sign extend
                    k |= 0xF000;

                    log::info!("k: {:04x} {:016b} {}", k as i16, k as i16, k as i16);

                    let kk: i16 = k as i16;

                    log::info!("kk: {:04x} {:016b} {}", kk, kk, kk);
                    
                    //cpu.pc += kk as i32;

                    // push return address onto the stack 
                    let data: i32 = cpu.pc;
                    push_to_stack_i16(&mut cpu, data as i16);

                    log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                    
                    // jump to address
                    cpu.pc += kk as i32;
                }

                /* 92 */ 
                InstructionType::RET => { 
                    log::info!("[RET]");

                    // let d_val: u16 = value_storage[&'d'];
                    // log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                    // cpu.pc += 2i32;

                    // let data = cpu.register_file[d_val as usize];
                    let k_hi: u16 = pop_from_stack_u8(&mut cpu) as u16;
                    let k_lo: u16 = pop_from_stack_u8(&mut cpu) as u16;

                    let k_val:i16 = ((k_hi << 8i16) + k_lo) as i16;

                    cpu.pc = k_val as i32;

                    log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                },

                /*  94 */ 
                InstructionType::RJMP => {

                    log::info!("[RJMP]");

                    // get the first 16 bit
                    let mut k: u16 = value_storage[&'k'] as u16;

                    log::info!("k: {:04x} {:016b}", k, k);

                    // sign extend
                    k |= 0xF000;

                    log::info!("k: {:04x} {:016b} {}", k as i16, k as i16, k as i16);

                    let kk: i16 = k as i16;

                    log::info!("kk: {:04x} {:016b} {}", kk, kk, kk);
                    
                    cpu.pc += kk as i32;
                },

                InstructionType::Unknown => { panic!("Unknown instruction!"); },
        
                _ => { panic!("Unknown instruction!"); }

            }
        }
    }

    log_end();

    Ok(())

}

fn init_logging() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        // https://stackoverflow.com/questions/61810740/log-source-file-and-line-numbers
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

fn log_start() {
    log::trace!("Application starts ...");
    log::debug!("Application starts ...");
    log::info!("Application starts ...");
    log::warn!("Application starts ...");
    log::error!("Application starts ...");
}

fn log_end() {
    log::trace!("Application terminates.");
    log::debug!("Application terminates.");
    log::info!("Application terminates.");
    log::warn!("Application terminates.");
    log::error!("Application terminates.");
}

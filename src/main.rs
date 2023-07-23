mod ihex_mgmt;
mod file_mgmt;
mod instructions;

use std::io;
use std::io::Write;
use std::io::Cursor;
use std::collections::HashMap;

use env_logger::{Builder, Target};
use instructions::instruction_definition::InstructionDefinition;
use log::LevelFilter;

use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;
use crate::instructions::decode::decode_instruction;
use crate::instructions::instruction_type::InstructionType;
use crate::instructions::instructions::instructions;
use crate::instructions::instructions::unknown;
use crate::instructions::process::*;

use byteorder::{LittleEndian, ReadBytesExt};

fn main() -> io::Result<()> {

    println!("whatavr starting ...");

    // logging setup
    init_logging();
    log_start();

    let HEX: bool = false;
    if HEX {
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

        const DISSASSEMBLE:bool = false;
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

    

    const EXECUTE: bool = true;
    if EXECUTE {

        let mut idx: usize = 0usize;

        let mut assembler_segment: Segment = Segment::new();
        assembler_segment.address = 0u16;
        assembler_segment.size = 0u32;
        //assembler_segment.data[0usize] = 0x00u8;

        let mut labels:HashMap<String, usize> = HashMap::new();

        let mut z:bool = false;

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

        //          ldi r16, 1
        //          ldi r17, 1
        //          add r16, r17
        // loop:    jmp loop

        // encode_ldi(&mut assembler_segment, &mut idx, 16u16, 1u16);
        // encode_ldi(&mut assembler_segment, &mut idx, 17u16, 1u16);
        // encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
        // create_label(&mut labels, String::from("loop"), idx);
        // encode_jmp(&mut assembler_segment, &mut idx, &labels, &String::from("loop"));


        encode_ldi(&mut assembler_segment, &mut idx, 16u16, 1u16);
        encode_ldi(&mut assembler_segment, &mut idx, 17u16, 1u16);
        create_label(&mut labels, String::from("loop"), idx);
        encode_add(&mut assembler_segment, &mut idx, 16u16, 17u16);
        encode_jmp(&mut assembler_segment, &mut idx, &labels, &String::from("loop"));
        
        //encode_jmp(&mut assembler_segment, &mut idx, &String::from("loop"));

        // pc always points to the instruction after the current instruction so it does not start at 0x00 but at 0x02
        let mut pc: i32 = 0x02;

        // register file, 32 8bit registers
        let mut register_file: [u8; 32] = [0; 32];

        let mut done: bool = false;
        while !done {

            // get the current instruction
            let temp_pc:i32 = pc - 0x02;

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
            let instruction: &InstructionDefinition = decode_instruction(word, instructions, &unknown, &mut value_storage);

            // execute the instruction
            match instruction.instruction_type {

                /*   6 */ 
                InstructionType::ADD => { 
                    log::info!("[ADD]");

                    let r_valu16 = value_storage[&'r'];
                    let d_valu16 = value_storage[&'d'];

                    register_file[d_valu16 as usize] += register_file[r_valu16 as usize];

                    log::info!("[ADD] result value: {}", register_file[d_valu16 as usize]);

                    pc += 2i32;
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
                        if z {
                            
                            pc += 2i32;

                        } else {
                            
                            pc = (pc as i16 + offset as i16) as i32;

                        }

                    }
                },

                /*  27 */ 
                InstructionType::BRNE => { 
                    log::info!("[BRNE]");

                    let offset:i32 = value_storage[&'k'] as i32;

                    // check the Z-flag
                    if z {
                        pc += offset;
                    } else {
                        pc += 2i32;
                    }
                },

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

                    let mut d:u16 = value_storage[&'d'];
                    
                    //log::info!("Clearing register d: {:#06x}", d);
                    register_file[d as usize] = 0x00;

                    pc += 2i32;
                },

                /*  53 */ 
                InstructionType::DEC => { 
                    log::info!("[DEC]");

                    let mut d:u16 = value_storage[&'d'];
                    
                    // perform the decrement
                    register_file[d as usize] -= 0x01u8;

                    // set the z flag
                    if register_file[d as usize] == 0x00u8 {
                        z = true;
                    }

                    log::info!("[DEC] Register r{}: {:#06x}", d, register_file[d as usize]);

                    pc += 2i32;
                },
            
                /*  66 */ 
                InstructionType::JMP => {

                    log::info!("[JMP]");

                    // get the first 16 bit
                    let k_hi:u32 = value_storage[&'k'].into();

                    //log::info!("READ 1: {:#02x}", assembler_segment.data[pc as usize] as u8);
                    //log::info!("READ 2: {:#02x}", assembler_segment.data[(pc + 1) as usize] as u8);

                    //pc += 2u32;
                    //pc += 1u32;

                    //log::info!("READ 3: {:#02x}", assembler_segment.data[(pc + 2) as usize] as u8);
                    //log::info!("READ 4: {:#02x}", assembler_segment.data[(pc + 3) as usize] as u8);

                    // get the next 16 stored at the pc since the JMP command is encoded using 32 bit (4 byte)
                    let temp_pc:i32 = pc;

                    let word_hi:u16 = assembler_segment.data[(temp_pc + 1i32) as usize] as u16;
                    //log::info!("READ: {:#02x}", word_hi as u8);

                    let word_lo:u16 = assembler_segment.data[temp_pc as usize] as u16;
                    //log::info!("READ: {:#02x}", word_lo as u8);

                    let k_lo:u32 = ((word_hi << 8u8) + word_lo).into();

                    // assemble the parameter k
                    let mut k_val:i16 = ((k_hi << 16u8) + k_lo) as i16;

                    //log::info!("k_val: {k_val:#02x}");

                    // sign extend to i32
                    //let k_val_i32:i32 = 0b11111111110000000000000000000000 | k_val;
                    let k_val_i32:i32 = k_val as i32;
                    //log::info!("k_val_i32: {k_val_i32:#02x}");
                    //log::info!("offset: {k_val_i32:#02x}");

                    // log::info!("k_val_i32: {k_val_i32:#02x}");

                    // twos-complement
                    //let offset:i32 = (k_val as i8 * -1i8) as i32;

                    //log::info!("offset: {offset:#02x}");

                    // since the amount of elements to jump are words, to find the address, multiply by two
                    //k *= 2u32;

                    // DEBUG
                    //log::info!("k: {:#06x}", k);

                    // perform the jump
                    //pc += (offset - 2u32);

                    //let p: i32 = offset as i32 - 2i32;

                    //pc -= 2;
                    //pc += offset;
                    pc += k_val_i32;

                    //pc += p;

                    //log::info!("done");
                    //panic!("LUL");




                    //pc = (pc as i16 + offset as i16) as i32;
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

                    log::trace!("{temp_pc:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}");


                    // execute
                    register_file[register as usize] = k_val as u8;

                    pc += 2i32;
                },

                /*  88 */ 
                InstructionType::OUT => {
                    log::info!("[OUT]");

                    // Stores data from register Rr in the Register File to I/O Space (Ports, Timers, Configuration Registers, etc.).
                    let a_val = value_storage[&'A'];
                    // log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
                    let r_val = value_storage[&'r'];
                    // log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

                    // TODO output the value stored in register r_val into memory to the address a_val

                    pc += 2i32;
                },

                InstructionType::Unknown => { panic!("Unknown instruction!"); },
        
                _ => { panic!("Unknown instruction!"); }

            }
        }
    }

    log_end();

    Ok(())

}

fn create_label(labels:&mut HashMap<String, usize>, label: String, idx: usize)
{
    labels.insert(label, idx);
}

/// add - Add without carry (Rd ← Rd + Rr)
/// 0000 11rd dddd rrrr
fn encode_add(assembler_segment:&mut Segment, idx:&mut usize, register_d: u16, register_r: u16)
{
    // register is increased by 16 to arrive at the register id
    let register_d_offset: u16 = register_d; // - 16u16;
    let register_r_offset: u16 = register_r; // - 16u16;

    let r_mask: u16 = ((register_r_offset >> 4u16) << 9u16) | (register_r_offset & 0x0Fu16);
    //let k_mask: u16 = ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);

    let result: u16 = 0x0C00u16 | (r_mask | register_d_offset << 4u16);

    log::info!("ENC ADD: {:#02x}", (result >> 0u16) as u8);
    assembler_segment.data.push((result >> 0u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;

    log::info!("ENC ADD: {:#02x}", (result >> 0u16) as u8);
    assembler_segment.data.push((result >> 8u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;
}

/// ldi - load immediate
/// 1110 KKKK dddd KKKK
/// 
/// NOTE: LDI is only allowed for registers in the range from [r16, r31]
/// The full 32 regsters cannot be used since there are only 4 bit of space to store the target register.
/// The register parameter is diminished by 16 so that it fits into the 4 bit space in the instruction word.
fn encode_ldi(assembler_segment:&mut Segment, idx:&mut usize, register_d: u16, imm_value_k: u16)
{
    if register_d < 15 || register_d > 31 {
        panic!("Invalid register for LDI! Only registers [r16, r31] are allowed")
    }
    // register is increased by 16 to arrive at the register id
    let register: u16 = register_d - 16u16;

    let k_mask: u16 = 0xE000u16 | ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);
    //let k_mask: u16 = ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);

    let result: u16 = 0xEFFFu16 & (k_mask | (register << 4u16));

    //log::info!("result: {:#018b}", result);
    
    log::info!("ENC LDI: {:#02x}", (result >> 0u16) as u8);
    assembler_segment.data.push((result >> 0u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;
    
    log::info!("ENC LDI: {:#02x}", (result >> 8u16) as u8);
    assembler_segment.data.push((result >> 8u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;
}

/// dec - decrement
/// 1001 010d dddd 1010
fn encode_dec(assembler_segment:&mut Segment, idx:&mut usize, register_d: u16)
{
    // register is increased by 16 to arrive at the register id
    let register: u16 = register_d; // - 16u16;

    let result: u16 = 0x940Au16 | (register << 4u16);
    //let k_mask: u16 = ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);

    // let result: u16 = 0xEFFFu16 & (k_mask | (register << 4u16));
    
    log::info!("ENC DEC: {:#02x}", (result >> 0u16) as u8);
    assembler_segment.data.push((result >> 0u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;

    log::info!("ENC DEC: {:#02x}", (result >> 8u16) as u8);
    assembler_segment.data.push((result >> 8u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;
}

/// BRNE – Branch if Not Equal
/// 1111 01kk kkkk k001
fn encode_brne(assembler_segment:&mut Segment, idx:&mut usize, labels:&HashMap<String, usize>, label: &String)
{
    // register is increased by 16 to arrive at the register id
    let offset_k: u16 = labels[label] as u16;

    let result: u16 = 0xF401u16 | (offset_k << 3u16);
    //let k_mask: u16 = ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);

    // let result: u16 = 0xEFFFu16 & (k_mask | (register << 4u16));
    
    log::info!("ENC BRNE: {:#02x}", (result >> 0u16) as u8);
    assembler_segment.data.push((result >> 0u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;

    log::info!("ENC BRNE: {:#02x}", (result >> 8u16) as u8);
    assembler_segment.data.push((result >> 8u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;
}

/// 66. JMP – Jump
/// 1001 010k kkkk 110k
/// kkkk kkkk kkkk kkkk
fn encode_jmp(assembler_segment:&mut Segment, idx:&mut usize, labels:&HashMap<String, usize>, label: &String)
{
    // register is increased by 16 to arrive at the register id
    let label_address: i32 = labels[label] as i32;
    let mut offset_k: i32 = label_address - (*idx as i32);

    log::info!("offset_k: {:#06x}", offset_k);
    log::info!("offset_k: {:#06x}", offset_k as u32);

    offset_k &= 0b00000000001111111111111111111111i32;
    //offset_k &= 0b 0000 0000 0011 1111 1111 1111 1111 1111 i32;

    log::info!("offset_k: {:#06x}", offset_k);
    log::info!("offset_k: {:#06x}", offset_k as u32);

    //let offset_k: i32 = 0;
    //let result: u32 = (0b1001010u32 << 25) | ((offset_k as u32 >> 16) << 20) | (0b110u32 << 17) | (offset_k as u32 & 0b11111111111111111u32);
    let result: u32 = (0b1001010u32 << 25) | ((offset_k as u32 >> 17) << 20) | (0b110u32 << 17) | (offset_k as u32 & 0b11111111111111111u32);


    log::info!("result: {:#32b}", result);


    //panic!("t");


    // let label_address: u32 = labels[label] as u32;
    // let offset_k: u32 = label_address - (*idx as u32);
    // let result: u32 = (0b1001010u32 << 25) | ((offset_k as u32 >> 16) << 20) | (0b110u32 << 17) | (offset_k as u32 & 0b11111111111111111u32);

    


    
    
    log::info!("ENC JMP: {:#02x}", (result >> 16u16) as u8);
    assembler_segment.data.push((result >> 16u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;

    log::info!("ENC JMP: {:#02x}", (result >> 24u16) as u8);
    assembler_segment.data.push((result >> 24u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;

    log::info!("ENC JMP: {:#02x}", (result >> 0u16) as u8);
    assembler_segment.data.push((result >> 0u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;

    log::info!("ENC JMP: {:#02x}", (result >> 8u16) as u8);
    assembler_segment.data.push((result >> 8u16) as u8);
    assembler_segment.size += 1u32;
    *idx += 1usize;

    

    //log::info!("result: {:#026b}", result);

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

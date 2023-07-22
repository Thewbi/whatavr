mod ihex_mgmt;
mod file_mgmt;
mod instructions;

use std::io;
use std::io::Write;
use std::io::Cursor;
use std::collections::HashMap;

use env_logger::{Builder, Target};
use instructions::bit_pattern_matching::bit_match;
use instructions::bit_pattern_matching::bit_pattern_match;
use instructions::instruction_definition::InstructionDefinition;
use log::LevelFilter;

use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;
use crate::instructions::instruction_type::InstructionType;
use crate::instructions::instructions::instructions;
use crate::instructions::instructions::unknown;
use crate::instructions::process::*;

use byteorder::{LittleEndian, ReadBytesExt};




/// finds a command that matches data and returns that instruction definition
/// If no matching instruction is found, returns the instruction definition 'unknown'
//fn decode<'a>(data:u16, instructions_list: &'a Vec<InstructionDefinition>, unknown_instruction: &'a InstructionDefinition, value_storage:&mut HashMap<char, u16>) -> &'a InstructionDefinition {
fn decode<'a>(data:u16, instructions_list: &'static [InstructionDefinition], unknown_instruction: &'a InstructionDefinition, value_storage:&mut HashMap<char, u16>) -> &'a InstructionDefinition {

    for instruction in instructions_list 
    {
        if bit_match(data, &instruction.bit_pattern)
        {
            bit_pattern_match(data, &instruction.bit_pattern, value_storage);

            return &instruction;
        }
    }
     
    return unknown_instruction;
}

fn main() -> io::Result<()> {

    println!("whatavr starting ...");

    // logging setup
    init_logging();
    log_start();

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

    const DECODE:bool = false;
    if DECODE {

        let mut rdr = Cursor::new(&segment_0.data);
        while index < segment_0.data.len()
        {
            let word:u16 = rdr.read_u16::<LittleEndian>().unwrap().into();
            index += 2;

            log::trace!("word: {:#06x} {:b}", word, word);

            let mut value_storage:HashMap<char, u16> = HashMap::new();
            let instruction: &InstructionDefinition = decode(word, instructions, &unknown, &mut value_storage);

            log::info!("instruction {:?}", instruction.instruction_type);
            if instruction.instruction_type == InstructionType::EOR || instruction.instruction_type == InstructionType::CLR {
                log::info!("EOR and CLR similar. CLI is implemented by EOR the register with itself!");
            }

            match_instruction(&instruction, &mut rdr, &word, &mut index, &mut value_storage);
        }

    }

    const EXECUTE: bool = true;
    if EXECUTE {

        //let mut pc: usize = 0x00;
        let mut pc: u32 = 0x02;

        // register file, 32 8bit registers
        let mut register_file: [u8; 32] = [0; 32];

        let mut done: bool = false;
        while !done {

            let temp_pc:u32 = pc - 0x02;
            let word_hi:u16 = segment_0.data[(temp_pc+1u32) as usize].into();
            let word_lo:u16 = segment_0.data[temp_pc as usize].into();
            //pc += 2u32;

            let word:u16 = (word_hi << 8u8) + word_lo;
            
            let mut value_storage:HashMap<char, u16> = HashMap::new();
            let instruction: &InstructionDefinition = decode(word, instructions, &unknown, &mut value_storage);

            match instruction.instruction_type {

                /*  43 */ InstructionType::CLR          => { 

                    let mut d:u16 = value_storage[&'d'].into();
                    d -= 16u16;

                    log::info!("Clearing register d: {:#06x}", d);

                    register_file[d as usize] = 0x00;

                    pc += 2u32;

                },
            
                /*  66 */ InstructionType::JMP          => {

                    //let k_hi:u32 = word.into();
                    let k_hi:u32 = value_storage[&'k'].into();

                    //let temp_pc:u32 = pc - 0x02;
                    let temp_pc:u32 = pc;
                    let word_hi:u16 = segment_0.data[(temp_pc+1u32) as usize].into();
                    let word_lo:u16 = segment_0.data[temp_pc as usize].into();
                    let k_lo:u32 = ((word_hi << 8u8) + word_lo).into();

                    let mut k:u32 = ((k_hi << 16u8) + k_lo).into();

                    // since the amount of elements to jump are words, to find the address, multiply by two
                    k *= 2u32;
                    log::info!("k: {:#06x}", k);

                    // perform the jump
                    pc += k;
                },

                /*  88 */ InstructionType::OUT          => {

                    // Stores data from register Rr in the Register File to I/O Space (Ports, Timers, Configuration Registers, etc.).

                    let a_val = value_storage[&'A'];
                    log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
                    let r_val = value_storage[&'r'];
                    log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

                    // TODO output r1 to address

                    pc += 2u32;
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

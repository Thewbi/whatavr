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

use crate::assembler::asm_application::application_instruction_source;
use crate::assembler::asm_encoder::AsmEncoder;
use crate::assembler::asm_record::AsmRecord;
use crate::cpu::cpu::CPU;
use crate::cpu::cpu::RAMEND;
use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;
use crate::instructions::decode::decode_instruction;
use crate::instructions::instruction_type::InstructionType;
use crate::instructions::instructions::INSTRUCTIONS;
use crate::instructions::instructions::UNKNOWN;
use crate::instructions::process::*;

use byteorder::{LittleEndian, ReadBytesExt};

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
                let instruction: &InstructionDefinition = decode_instruction(word, INSTRUCTIONS, &UNKNOWN, &mut value_storage);

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

        // create an application as a vector of instructions
        let mut asm_records: Vec<AsmRecord> = Vec::new();
        application_instruction_source(&mut asm_records);

        let mut asm_encoder: AsmEncoder = AsmEncoder::new();

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

        // the ihex segment which is filled with source code bytes by the assembler
        let mut assembler_segment: Segment = Segment::new();
        assembler_segment.address = 0u16;
        assembler_segment.size = 0u32;
        
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

            cpu.execute_instruction(&assembler_segment);
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

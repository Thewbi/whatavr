mod ihex_mgmt;
mod file_mgmt;
mod instructions;
mod cpu;
mod assembler;
mod parser;

use std::io;
use std::io::Write;
use std::io::Cursor;
use std::collections::HashMap;

use antlr_rust::InputStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::ParseTreeListener;
use env_logger::{Builder, Target};
use instructions::instruction_definition::InstructionDefinition;
use log::LevelFilter;

use crate::parser::assemblerlexer;
use crate::assembler::application_file_source::application_file_source;
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
use crate::parser::assemblerparser::assemblerParserContextType;

use byteorder::{LittleEndian, ReadBytesExt};

// First, build the grammar
// cargo run --bin build_parser
// the generated files are placed into src/parser
fn main() -> io::Result<()> {

    println!("whatavr starting ...");

    // logging setup
    init_logging();
    log_start();

    struct Listener {}

    impl<'input> ParseTreeListener<'input, assemblerParserContextType> for Listener {
        // fn enter_every_rule(&mut self, ctx: &dyn assemblerParserContextType<'input>) {
        //     println!(
        //         "rule entered {}",
        //         csvparser::ruleNames
        //             .get(ctx.get_rule_index())
        //             .unwrap_or(&"error")
        //     )
        // }
    }

    //impl<'input> CSVListener<'input> for Listener {}

    println!("test started");
    let tf = ArenaCommonFactory::default();
    let mut _lexer =
        parser::assemblerlexer::assemblerLexer::new_with_token_factory(InputStream::new("V123,V2\nd1,d2\n".into()), &tf);
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = parser::assemblerparser::assemblerParser::new(token_source);
    //parser.add_parse_listener(Box::new(parser::assemblerlistenerimpl::assemblerListenerImpl{}));
    
    parser.add_parse_listener(Box::new(parser::assemblerlistenerimpl::assemblerListenerImpl{}));

    println!("\nstart parsing parser_test_csv");
    let result = parser.csvFile();
    assert!(result.is_ok());

    // assert_eq!(
    //     result.unwrap().to_string_tree(&*parser),
    //     "(csvFile (hdr (row (field V123) , (field V2) \\n)) (row (field d1) , (field d2) \\n))"
    // );



    //dissassemble();

    const EXECUTE: bool = true;
    if EXECUTE {

        // vector of instructions
        let mut asm_application: Vec<AsmRecord> = Vec::new();

        // create an application as a vector of instructions (mnemonics)
        //application_instruction_source(&mut asm_application);

        let mut asm_file_path:String = String::new();
        //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank1.hex");
        //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank2.hex") {
        //hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication1/GccApplication1.hex");
        asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_1.asm");

        application_file_source(&mut asm_application);

        // the ihex segment which is filled with source code bytes by the assembler
        let mut assembler_segment: Segment = Segment::new();
        assembler_segment.address = 0u16;
        assembler_segment.size = 0u32;

        // convert the mnemonic instructions into bytes to store into the ihex segment
        let mut asm_encoder: AsmEncoder = AsmEncoder::new();
        asm_encoder.assemble(&mut asm_application, &mut assembler_segment);
        
        // ATmega328p cpu
        let mut cpu: CPU = CPU {
            z: false,
            sph: 0x00u8,
            spl: 0x00u8,
            pc: 0x02i32,
            register_file: [0; 32usize],
            sram: [0; RAMEND as usize],
        };

        // main loop that executes the instruction
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

            // execute the next instruction
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

#[allow(dead_code, unused)]
fn dissassemble() -> io::Result<()> {

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

    Ok(())
}

mod assembler;
mod cpu;
mod file_mgmt;
mod ihex_mgmt;
mod instructions;
mod parser;
mod common;

use std::collections::HashMap;
use std::io;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::InputStream;
use antlr_rust::tree::ParseTreeVisitorCompat;
use common::common_constants::RAMEND;
use env_logger::{Builder, Target};
use instructions::instruction_definition::InstructionDefinition;
use log::LevelFilter;

use std::fs;
use std::rc::Rc;

use crate::assembler::asm_encoder::AsmEncoder;
use crate::assembler::asm_visitor_new::NewAssemblerVisitor;
use crate::common::listing_parser::is_code_offset_c_listing;
use crate::cpu::cpu::CPU;
use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;
use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::instructions::decode::decode_instruction;
use crate::instructions::instruction_type::InstructionType;
use crate::instructions::instructions::INSTRUCTIONS;
use crate::instructions::instructions::UNKNOWN;
use crate::instructions::process::match_instruction;
use crate::parser::assemblerparser::assemblerParserContextType;
use crate::parser::assemblerparser::Asm_fileContextAll;

use crate::fs::File;

use std::io::BufRead;
use crate::io::BufReader;

use byteorder::{LittleEndian, ReadBytesExt};

// https://stackoverflow.com/questions/34832583/global-mutable-hashmap-in-a-library
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;
lazy_static! {

    static ref CSEG_HASHMAP: Mutex<HashMap<String, String>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };

    static ref DSEG_HASHMAP: Mutex<HashMap<String, u16>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };

}

// rustup default stable
//
// First, build the grammar
// cargo run --bin build_parser
// the generated files are placed into src/parser
//
// antlr lab
//
// cargo build
// cargo run
// cargo run --bin build_parser
// cargo run --bin whatavr
//
// cargo fmt
fn main() -> io::Result<()> {

    log::info!("whatavr starting ...\n");

    //
    // logging setup
    //

    init_logging();
    log_start();

    // this is cseg the code segment
    let mut segments: Vec<Segment> = Vec::new();

    // asm source code
    let sram_content: [u8; RAMEND as usize] = load_segment_from_asm_source_code(&mut segments);

    // // hex
    // load_segment_from_hex_file(&mut segments);

    // // listing (lss) file (from C-SourceCode)
    // load_segment_from_listing_file(&mut segments);

    //
    // Phase - Program Execution
    //

    log::info!("*************************************************\n");
    log::info!("Phase - Program Execution\n");
    log::info!("*************************************************\n");

    let mut cpu: CPU = CPU::default();
    cpu.sram = sram_content;

    // main loop that executes the instructions
    let mut done: bool = false;
    while !done {

        log::trace!("\n");

        // check for end of code
        let temp_pc: i32 = cpu.pc;
        if segments[0].size <= temp_pc as u32 {
            log::info!("End of Code reached! Application Finished!\n");
            done = true;

            continue;
        }

        // execute the next instruction
        cpu.execute_instruction(&segments[0]);

        // DEBUG - output the CPU state
        log::trace!("{}\n", cpu);
    }

    log_end();

    Ok(())

}

fn load_segment_from_listing_file(segments: &mut Vec<Segment>) -> [u8; RAMEND as usize] //-> io::Result<()>
{
    let mut lss_file_path: String = String::new();
    //lss_file_path.push_str("test_resources/sample_files/lss/ADC_C.lss");
    lss_file_path.push_str("test_resources/sample_files/lss/Vorlesung_HNP_Beispiel_1.lss");

    let srcdir = PathBuf::from(&lss_file_path);
    log::trace!("absolute path: {:?}", fs::canonicalize(&srcdir));

    let data = fs::read_to_string(&lss_file_path).expect("Unable to read file");
    log::trace!("\n{}\n", data);

    // let input_stream: InputStream<&str> = InputStream::new(data.as_str());

    // open the file in read-only mode (ignoring errors).
    let file = File::open(lss_file_path).unwrap();
    let reader = BufReader::new(file);

    let mut string_buffer = String::new();

    // read the file line by line using the lines() iterator from std::io::BufRead.
    let mut idx: u32 = 1u32;
    for (index, line) in reader.lines().enumerate() {

        // ignore errors
        let line = line.unwrap();

        if !is_code_offset_c_listing(&line) {
            continue;
        }

        // DEBUG show the line and its number
        log::info!("{}. {}\n", idx, line);
        idx += 1u32;

        let colon_split = line.split(":");
        let colon_collection: Vec<&str> = colon_split.collect::<Vec<_>>();

        //log::info!("{:?}\n", parts);

        // for part in parts {
        //     log::info!("{:?}\n", part);
        // }

        log::trace!("{:?}\n", &colon_collection[1]);

        let tab_split = colon_collection[1].split("\t");

        log::trace!("{:?}\n", &tab_split);

        let tab_collection: Vec<&str> = tab_split.collect::<Vec<_>>();

        log::trace!("{:?}\n", &tab_collection);

        let start = 2;
        let end = tab_collection.len();
        let source_code_row = tab_collection[start .. end].join(" ");

        log::trace!("scr: {}\n", source_code_row);

        string_buffer.push_str(&source_code_row);
        string_buffer.push_str("\n");

        log::trace!("done\n");
    }

    log::trace!("{}", string_buffer);

    let input_stream: InputStream<&str> = InputStream::new(string_buffer.as_str());

    parse(segments, input_stream)

    //Ok(())
}

fn load_segment_from_asm_source_code(segments: &mut Vec<Segment>) -> [u8; RAMEND as usize] 
{
    //
    // Phase - load token into a hashmap
    //

    log::info!("**********************************************\n");
    log::info!("Phase - load token into a hashmap\n");
    log::info!("**********************************************\n");

    let mut token_storage: HashMap<isize, String> = HashMap::new();
    let mut token_value_storage: HashMap<String, isize> = HashMap::new();

    let mut token_file_path: String = String::new();
    //token_file_path.push_str("src/parser/assembler.tokens");
    token_file_path.push_str("src/parser/assembler.tokens");

    // open the file in read-only mode (ignoring errors).
    let file = File::open(token_file_path).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {

        // ignore errors.
        let line = line.unwrap();

        // DEBUG show the line and its number.
        log::trace!("{}. {}\n", index + 1, line);

        // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
        let collection: Vec<&str> = line.split('=').collect::<Vec<_>>();

        let token:&str = collection[0];
        let token_idx:i16 = collection[1].parse::<i16>().unwrap();
        let token_idx_as_usize:isize = token_idx.into();

        // at the end of the token file, the individual characters are repeated but
        // the purpose of the token map is to just contain the token labels/names and not
        // the individual characters so break on the first duplicate
        if token_storage.contains_key(&token_idx_as_usize) {
            break;
        }
        token_storage.insert(token_idx_as_usize, token.to_string());

        if token_value_storage.contains_key(&token.to_string()) {
            break;
        }
        token_value_storage.insert(token.to_string(), token_idx_as_usize);
    }

    //
    // Phase - read the .asm file
    //

    log::info!("**********************************************\n");
    log::info!("Phase - Phase - read the .asm file\n");
    log::info!("**********************************************\n");

    //
    // read the .asm file which will be the input to the assembler
    //
    // check files here: http://lab.antlr.org/
    // (erase the entire content in the lexer tab, paste the grammar into the parser tab,
    // use 'asm_file' as a start symbol)
    //

    let mut asm_file_path: String = String::new();
    //asm_file_path.push_str("test_resources/sample_files/asm/asm_1.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/asm_2.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/asm_3.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/asm_4.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/call_and_return.asm"); // regression test
    //asm_file_path.push_str("test_resources/sample_files/asm/call_test.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/call_test_2.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/count_bits.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/def_assembler_directive.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/dseg.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/excercise.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/expression.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/inc.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/intrinsic.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/jmp_instruction.asm"); // problem
    //asm_file_path.push_str("test_resources/sample_files/asm/jmp.asm"); // good for regression test (will increment r17 until overflow)
    //asm_file_path.push_str("test_resources/sample_files/asm/preprocessor.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/push_pop.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/ret_test.asm");
    asm_file_path.push_str("test_resources/sample_files/asm/str_length.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/scratchpad.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/scratchpad_2.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/setup_stack.asm"); // regression test
    //asm_file_path.push_str("test_resources/sample_files/asm/timer_polling_example.asm");
    //asm_file_path.push_str("C:/Program Files (x86)/Atmel/Studio/7.0/Packs/atmel/ATmega_DFP/1.7.374/avrasm/inc/m328Pdef.inc");
    //asm_file_path.push_str("test_resources/sample_files/asm/hwnp_excercise_1.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/st_std_test.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/stack_test.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/pin_change_interrupt_demo.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/pin_change_interrupt_demo.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/rjh_coding_avr-asm-add-16.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/include_test.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/timer1.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/twos_complement_overflow.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/sts.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/timebase.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/out.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/store_to_flash.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/store_to_flash_2.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/store_load_flash.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/store_load_sram.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/odd_even_test.asm");

    // ld, st, call, ret, push, pop, mov, movw, and, inc, dec, andi, add, adc, adiw, ldi, lsr,
    // lsl, brne, brbc, breq, brsh, brge, brlt, rol, ror, sbi, cbi, sbc, subi

    //asm_file_path.push_str("test_resources/sample_files/asm/andi.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/add.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/adc.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/adiw.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/and.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/brbc.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/brne.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/breq.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/brsh.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/brge.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/brlt.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/call.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/cbi.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/cpi.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/dec.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/inc.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/ld.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/ld_z.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/ldi.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/lpm.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/lsr.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/lsl.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/mov.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/movw.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/push.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/pop.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/ret.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/rjmp.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/rol.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/ror.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/sbi.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/sbc.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/subi.asm");
    //asm_file_path.push_str("test_resources/sample_files/asm/st.asm");

    let srcdir = PathBuf::from(&asm_file_path);
    println!("absolute path: {:?}", fs::canonicalize(&srcdir));

    let data = fs::read_to_string(&asm_file_path).expect("Unable to read file");
    log::info!("\n{}", data);

    let input_stream: InputStream<&str> = InputStream::new(data.as_str());

    parse(segments, input_stream)

}

fn load_segment_from_hex_file(segments: &mut Vec<Segment>) -> io::Result<()>
{
    // load hex file
    let mut hex_file_path: String = String::new();
    //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank1.hex");
    //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank2.hex") {
    //hex_file_path.push_str("test_resources/sample_files/GccApplication1/GccApplication1.hex");
    //hex_file_path.push_str("test_resources/sample_files/GccApplication2/GccApplication1.hex");
    //hex_file_path.push_str("test_resources/sample_files/arduboy/Ardynia/ardynia.hex");
    //hex_file_path.push_str("test_resources/sample_files/GccApplication2/GccApplication1.hex");
    //hex_file_path.push_str("test_resources/sample_files/hex/ExcerciseSheet2.hex");
    hex_file_path.push_str("test_resources/sample_files/hex/SegmentByteTest.hex");

    // split into segments
    // each segment has to have a segment_start and a segment_size

    match parse_hex_file(segments, &hex_file_path) {
        Ok(_name) => log::info!("File read"),
        Err(err) => {
            log::error!("An error occured while retrieving the peername: {:?}\n", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error at load hex file!",
            ));
        }
    }

    Ok(())
}

fn parse(segments: &mut Vec<Segment>, input_stream: InputStream<&str>) -> [u8; RAMEND as usize] 
{
    //
    // Phase - AST Creation (Grammar Lexing and Parsing)
    //

    log::info!("*************************************************\n");
    log::info!("Phase - AST Creation (Grammar Lexing and Parsing)\n");
    log::info!("*************************************************\n");

    let token_factory: antlr_rust::token_factory::ArenaFactory<'_, antlr_rust::token_factory::CommonTokenFactory, antlr_rust::token::GenericToken<_>> = ArenaCommonFactory::default();
    let mut _lexer: parser::assemblerlexer::assemblerLexer<'_, InputStream<&str>> = parser::assemblerlexer::assemblerLexer::new_with_token_factory(
        input_stream,
        &token_factory,
    );
    let token_source: CommonTokenStream<'_, parser::assemblerlexer::assemblerLexer<'_, InputStream<&str>>> = CommonTokenStream::new(_lexer);
    let mut parser: parser::assemblerparser::assemblerParser<'_, CommonTokenStream<'_, parser::assemblerlexer::assemblerLexer<'_, InputStream<&str>>>, antlr_rust::DefaultErrorStrategy<'_, assemblerParserContextType>> = parser::assemblerparser::assemblerParser::new(token_source);

    let result = parser.asm_file();
    assert!(result.is_ok());
    let root: Rc<Asm_fileContextAll> = result.unwrap();

    //next steps
        // // check the interrupt queue for a interrupt
        // // but only if the CPU has not executed an interrupt right before.
        // // The CPU has to execute at least one normal instruction before
        // // going into the next interrupt
        // if !force_normal_instruction_execution && is_global_interrupt_flag_enabled {

        //     // get next interrupt from the interrupt queue
        //     // the entry contains the type of interrupt and the address in the global interrupt vector table

        //     // check if this particular interrupt is enabled in the particular periphery register
        //     // if the specific interrupt is disabled, skip the event and check the next interrupt in the queue

        //     // turn of global interrupt enable flag

        //     // push the current PC onto the stack

        //     // disable global interrupt flag

        //     // disable the flag of the periphery specific interrupt in the specific interrupt register

        //     // execute the interrupt handler stored in the global interrupt vector at the address of the interrupt

        //     // enable the specific interrupt

        //     // enable the global interrupt flag

        // }

    //
    // Phase - AST Visiting
    //

    log::info!("*************************************************\n");
    log::info!("Phase - AST Visiting\n");
    log::info!("*************************************************\n");

    // // the visitor traverses the AST (Abstract Syntax Tree) and creates
    // // AsmRecords. It will insert these ARMRecords into the records parameter
    // let mut visitor = DefaultAssemblerVisitor {
    //     result_value: String::default(),
    //     ident: 0u16,
    //     records: Vec::new(),
    //     record: AsmRecord::default(),
    //     text: String::default(),
    //     last_terminal: String::default(),
    //     intrinsic_usage: String::default(),
    //     mnemonic: String::default(),
    //     reg_1: String::default(),
    //     reg_2: String::default(),
    //     data: String::default(),
    //     label: String::default(),
    //     target_label: String::default(),
    //     return_val: Vec::new(),
    //     preprocessor_directive: bool::default(),
    //     debug_output: true,
    // };
    // visitor.record.clear();

    // new visitor
    // let mut visitor: NewAssemblerVisitor = NewAssemblerVisitor {
    //     records: Vec::new(),
    //     record: AsmRecord::default(),

    //     ident: 0u16,
    //     debug_output: true,

    //     return_val: Vec::new(),

    //     label: String::default(),
    // };
    let mut visitor: NewAssemblerVisitor = NewAssemblerVisitor::default();
    visitor.record.clear();

    let visitor_result = visitor.visit(&*root);
    log::trace!("{:?}\n", visitor_result);

    //
    // Phase - Encoding
    //

    log::info!("*************************************************\n");
    log::info!("Phase - Encoding\n");
    log::info!("*************************************************\n");

    // the ihex segment which is filled with source code bytes by the assembler
    let mut assembler_segment: Segment = Segment::new();
    assembler_segment.address = 0u16;
    assembler_segment.size = 0u32;

    // convert the mnemonic instructions into bytes to store into the ihex segment
    let mut asm_encoder: AsmEncoder = AsmEncoder::new();
    asm_encoder.assemble(&mut visitor.records, &mut assembler_segment);

    segments.push(assembler_segment);

    if !asm_encoder.encoding_success {
        panic!("Encoding failed!");
    }

    visitor.sram
}

fn init_logging() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        // https://stackoverflow.com/questions/61810740/log-source-file-and-line-numbers
        .format(|buf, record| {
            //writeln!(
            write!(
                buf,
                //"{}:{} {} [{}] - {}",
                "{}:{} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                //chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

fn log_start() {
    log::trace!("Application starts ...\n");
    log::debug!("Application starts ...\n");
    log::info!("Application starts ...\n");
    log::warn!("Application starts ...\n");
    log::error!("Application starts ...\n");
}

fn log_end() {
    log::trace!("Application terminates.\n");
    log::debug!("Application terminates.\n");
    log::info!("Application terminates.\n");
    log::warn!("Application terminates.\n");
    log::error!("Application terminates.\n");
}

#[allow(dead_code, unused)]
fn dissassemble() -> io::Result<()> {

    //let hex: bool = false;
    //if hex {

        let mut segments: Vec<Segment> = Vec::new();
    load_segment_from_hex_file(&mut segments);

        // // DEBUG dump parsed segments
        // for seg in segments.iter_mut() {
        //     log::info!("Segment: {}", seg);
        // }

    const DISSASSEMBLE: bool = false;
    if DISSASSEMBLE {

        // process the first segment only
        let ref segment_0: &Segment = &segments[0];
        log::info!("Segment: {}\n", segment_0);

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

            let mut rdr = Cursor::new(&segment_0.data);
            while index < segment_0.data.len() {

                let word: u16 = rdr.read_u16::<LittleEndian>().unwrap().into();
                index += 2;

                log::trace!("word: {:#06x} {:b}\n", word, word);

                let mut value_storage: HashMap<char, u16> = HashMap::new();
                let instruction: &InstructionDefinition =
                    decode_instruction(word, INSTRUCTIONS, &UNKNOWN, &mut value_storage);

                log::info!("instruction {:?}\n", instruction.instruction_type);
                if instruction.instruction_type == InstructionType::EOR
                    || instruction.instruction_type == InstructionType::CLR
                {
                    log::info!(
                        "EOR and CLR similar. CLI is implemented by EOR the register with itself!\n"
                    );
                }

            // produce output of the disassembly process
                match_instruction(
                    &instruction,
                    &mut rdr,
                    &word,
                    &mut index,
                    &mut value_storage,
                );
            }
        }

    // }

    Ok(())
}

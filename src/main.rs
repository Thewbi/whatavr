mod assembler;
mod cpu;
mod file_mgmt;
mod ihex_mgmt;
mod instructions;
mod parser;

use std::collections::HashMap;
use std::io;
use std::io::Cursor;
use std::io::Write;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::token::Token;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::ParseTreeListener;
use antlr_rust::InputStream;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::tree::Tree;
use antlr_rust::tree::VisitChildren;
use env_logger::{Builder, Target};
use instructions::instruction_definition::InstructionDefinition;
use log::LevelFilter;

use std::fs;
use std::rc::Rc;

use crate::assembler::application_file_source::application_file_source;
use crate::assembler::asm_encoder::AsmEncoder;
use crate::assembler::asm_record;
use crate::assembler::asm_record::AsmRecord;
use crate::assembler::io_destination::IoDestination;
use crate::cpu::cpu::CPU;
use crate::cpu::cpu::RAMEND;
use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;
use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::instructions::decode::decode_instruction;
use crate::instructions::instruction_type::InstructionType;
use crate::instructions::instructions::INSTRUCTIONS;
use crate::instructions::instructions::UNKNOWN;
use crate::instructions::process::*;
use crate::parser::assemblerparser::InstructionContext;
use crate::parser::assemblerparser::assemblerParser;
use crate::parser::assemblerparser::assemblerParserContextType;
use crate::parser::assemblerparser::Asm_fileContextAll;
use crate::parser::assemblervisitor::assemblerVisitor;
use crate::parser::assemblervisitor::assemblerVisitorCompat;
use antlr_rust::tree::ParseTree;
use crate::parser::assemblerparser::ParamContext;

use crate::fs::File;
//use std::fs::File;

use std::io::BufRead;
use crate::io::BufReader;
//use std::io::BufReader;

use byteorder::{LittleEndian, ReadBytesExt};

// rustup default stable
//
// First, build the grammar
// cargo run --bin build_parser
// the generated files are placed into src/parser
//
// cargo build
// cargo run
// cargo run --bin whatavr
// cargo run --bin build_parser
//
// cargo fmt
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

    //
    // load token into a hashmap
    //

    let mut token_storage: HashMap<isize, String> = HashMap::new();

    let mut token_file_path: String = String::new();
    token_file_path.push_str("C:/aaa_se/rust/whatavr/src/parser/assembler.tokens");

    // open the file in read-only mode (ignoring errors).
    let file = File::open(token_file_path).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {

        // ignore errors.
        let line = line.unwrap();

        // show the line and its number.
        println!("{}. {}", index + 1, line);

        // https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust
        let collection: Vec<&str> = line.split('=').collect::<Vec<_>>();
        
        let token:&str = collection[0];
        let token_idx:i16 = collection[1].parse::<i16>().unwrap();
        let token_idx_as_usize:isize = token_idx.into();

        // at the end of the token file, the individual characters are repeasted but
        // the purpose of the token map is to just contain the token labels/names and not
        // the individual characters so break on the first duplicate
        if token_storage.contains_key(&token_idx_as_usize) {
            break;
        }

        token_storage.insert(token_idx_as_usize, token.to_string());
    }

    //
    // read the .asm file which will be the input to the assembler
    // 

    let mut asm_file_path: String = String::new();
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_1.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_2.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_3.asm");
    asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_4.asm");

    let data = fs::read_to_string(asm_file_path).expect("Unable to read file");
    println!("{}", data);

    let input_stream: InputStream<&str> = InputStream::new(data.as_str());

    println!("test started");
    let tf = ArenaCommonFactory::default();
    let mut _lexer = parser::assemblerlexer::assemblerLexer::new_with_token_factory(
        //InputStream::new("V123,V2\nd1,d2\n".into()),
        input_stream,
        &tf,
    );
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = parser::assemblerparser::assemblerParser::new(token_source);
    //parser.add_parse_listener(Box::new(parser::assemblerlistenerimpl::assemblerListenerImpl{}));

    parser.add_parse_listener(Box::new(
        parser::assemblerlistenerimpl::assemblerListenerImpl {},
    ));

    //println!("start parsing");

    let result = parser.asm_file();
    assert!(result.is_ok());

    let root: Rc<Asm_fileContextAll> = result.unwrap();

    log::info!("string tree: {}", root.to_string_tree(&*parser));


    
    //let res = Vec<& str>();

    //let asm_record: AsmRecord;

    //struct DefaultAssemblerVisitor<'i>(Vec<&'i str>);

    // structs with parethesis are tuple structs (https://stackoverflow.com/questions/49716865/what-are-structs-with-round-brackets-in-rust-for)
    struct DefaultAssemblerVisitor<'i>(Vec<&'i str>, AsmRecord, HashMap<isize, String>, Vec<AsmRecord>);

    impl<'i> ParseTreeVisitorCompat<'i> for DefaultAssemblerVisitor<'i> {

        type Node = assemblerParserContextType;
        type Return = Vec<&'i str>;
        //type Return = Vec<&'i String>;
        //type Return = Vec<String>;

        

        fn temp_result(&mut self) -> &mut Self::Return {
            
            &mut self.0
            //&mut vec![&String::from(self.0[0])]

            //return &mut vec![&String::from("ADD")];

            //self.

            //todo!()

        }

        // fn visit(&mut self, node: &<Self::Node as antlr_rust::parser::ParserNodeType<'i>>::Type) -> Self::Return {
            
        //     //log::info!("visit(): {:?} child_count: {:?}", node, node.get_child_count());
            
        //     self.visit_node(node);
            
        //     self.temp_result().to_vec()
        // }

        fn visit_terminal(&mut self, node: &antlr_rust::tree::TerminalNode<'i, Self::Node>) -> Self::Return {

            let token_type:isize = node.symbol.get_token_type();

            if token_type < 0 {
                return vec![&node.symbol.text];
            }

            let token_name:&String = self.2.get(&token_type).unwrap();

            let instruction_type: InstructionType = InstructionType::from_string(token_name);

            if InstructionType::Unknown == instruction_type {
                return vec![&node.symbol.text];
            }

            self.1.instruction_type = instruction_type;

            //return vec![token_name.as_str()];

            let ssval:String = self.1.instruction_type.to_string_string();
            //return vec![&ssval.as_str()];

            //return vec![&self.1.instruction_type.to_string_string().clone()];
            return vec![&node.symbol.text];
            
            // if node.symbol.get_token_type() == parser::assemblerparser::ADD {

            //     self.1.instruction_type = InstructionType::ADD;

            //     return vec!["ADD"];
            // }

            // if node.symbol.get_token_type() == parser::assemblerparser::LDI {

            //     self.1.instruction_type = InstructionType::LDI;

            //     return vec!["LDI"];
            // }
            

            //return vec![&node.symbol.text];

        }

        fn visit_error_node(&mut self, _node: &antlr_rust::tree::ErrorNode<'i, Self::Node>) -> Self::Return {
            Self::Return::default()
        }

        fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
            //next
            //self.aggregate_results(aggregate, next)
            //aggregate[1] = next[0];
            //aggregate

            // https://stackoverflow.com/questions/40792801/what-is-the-best-way-to-concatenate-vectors-in-rust
            let c: Vec<&'i str> = aggregate.iter().cloned().chain(next.iter().cloned()).collect(); // Cloned
            c

            // let c: Vec<String> = aggregate.iter().cloned().chain(next.iter().cloned()).collect(); // Cloned
            // c
        }

        fn should_visit_next_child(
            &self,
            node: &<Self::Node as antlr_rust::parser::ParserNodeType<'i>>::Type,
            current: &Self::Return,
        ) -> bool {
            true
        }

    }

    

    impl<'i> assemblerVisitorCompat<'i> for DefaultAssemblerVisitor<'i> {

        fn visit_asm_file(&mut self, ctx: &parser::assemblerparser::Asm_fileContext<'i>) -> Self::Return {
            //log::info!("visit_asm_file()");
            self.visit_children(ctx)
        }

        fn visit_row(&mut self, ctx: &parser::assemblerparser::RowContext<'i>) -> Self::Return {

            //log::info!("visit_row()");

            //self.visit_children(ctx)

            // if 1usize <= ctx.get_child_count() {
                
            //     let first_child = ctx.get_child(0usize);

            //     log::info!("{:?}", first_child);
            // }

            // let children_result = self.visit_children(ctx);

            // log::info!("{:?}", children_result);

            // children_result




            let children_result = self.visit_children(ctx);

            // log::info!("{:?}", children_result);
            log::info!("{:?}", self.1);

            
            self.3.push(self.1.clone());

            // clear for the next record
            self.1.clear();


            children_result

        }

        fn visit_instruction(&mut self, ctx: &InstructionContext<'i>) -> Self::Return {
			//self.visit_children(ctx)

            //log::info!("visit_instruction()");

            // for child in ctx.get_children() {
            //     let child_result = self.visit(child);
            // }
            



            let children_result = self.visit_children(ctx);

            log::info!("visit_instruction() - {:?}", children_result);

            children_result
		}

        fn visit_macro_usage(&mut self, ctx: &parser::assemblerparser::Macro_usageContext<'i>) -> Self::Return {
            log::info!("visit_macro_usage()");
            //self.visit_children(ctx)

            let children_result = self.visit_children(ctx);

            log::info!("visit_macro_usage() - {:?}", children_result);

            children_result
        }

        fn visit_label_definition(&mut self, ctx: &parser::assemblerparser::Label_definitionContext<'i>) -> Self::Return {
            //log::info!("visit_label_definition()");
            //self.visit_children(ctx)

            let children_result = self.visit_children(ctx);

            log::info!("visit_label_definition() - {:?}", children_result);

            children_result
        }

        // parameter inside an instruction
        fn visit_param(&mut self, ctx: &ParamContext<'i>) -> Self::Return {
			//self.visit_children(ctx)

            let children_result = self.visit_children(ctx);

            log::info!("{:?}", children_result);

            let text = children_result[0];

            

            // log::info!("{}", node.symbol.text);

            if 0xFF == self.1.reg_1 {

                if text.starts_with("r")
                {
                    self.1.reg_1 = text[1..].parse::<u16>().unwrap();
                }
                else if "" != text && "," != text && "\r\n" != text 
                {
                    self.1.data = text.parse::<u16>().unwrap();
                }

            } else if 0xFF == self.1.reg_2 {

                if text.starts_with("r")
                {
                    self.1.reg_2 = text[1..].parse::<u16>().unwrap();
                } 
                else if "" != text && "," != text && "\r\n" != text 
                {
                    self.1.data = text.parse::<u16>().unwrap();
                }

            }



            children_result
		}

        fn visit_parameter(&mut self, ctx: &parser::assemblerparser::ParameterContext<'i>) -> Self::Return {
            //log::info!("visit_parameter()");

            

            self.visit_children(ctx)
        }

        fn visit_macro_placeholder(&mut self, ctx: &parser::assemblerparser::Macro_placeholderContext<'i>) -> Self::Return {
            //log::info!("visit_macro_placeholder()");
            self.visit_children(ctx)
        }

        fn visit_expression(&mut self, ctx: &parser::assemblerparser::ExpressionContext<'i>) -> Self::Return {
            //log::info!("visit_expression()");
            self.visit_children(ctx)
        }

        fn visit_asm_instrinsic_instruction(&mut self, ctx: &parser::assemblerparser::Asm_instrinsic_instructionContext<'i>) -> Self::Return {
            //log::info!("visit_asm_instrinsic_instruction()");
            //self.visit_children(ctx)

            let children_result = self.visit_children(ctx);

            log::info!("visit_asm_instrinsic_instruction() - {:?}", children_result);

            children_result
        }

        fn visit_asm_intrinsic_usage(&mut self, ctx: &parser::assemblerparser::Asm_intrinsic_usageContext<'i>) -> Self::Return {
            
            //log::info!("visit_asm_intrinsic_usage()");
            
            self.visit_children(ctx)

            // working
            // let children_result = self.visit_children(ctx);
            // log::info!("visit_asm_intrinsic_usage() - {:?}", children_result);
            // children_result

            // todo either resolve the instrinsic right here and return the result or just return the string

            // for str in &children_result {
            //     log::info!("{}", str);
            // }

            // // https://stackoverflow.com/questions/28311868/what-is-the-equivalent-of-the-join-operator-over-a-vector-of-strings
            // let children_result = self.visit_children(ctx);
            // //let joined = children_result.iter().copied().collect::<Vec<_>>();

            // let joined = children_result.iter().collect::<Vec<String>>();

            // return joined;
            //let joined = String::from(children_result.join("-"));

            //return vec![joined];

            //return vec![&children_result.join("-")];
        }

        /// is the rule that directly selects the TOKEN of an instruction (ADD; CALL, EOR; LDI; ...)
        fn visit_mnemonic(&mut self, ctx: &parser::assemblerparser::MnemonicContext<'i>) -> Self::Return {

            //log::info!("visit_mnemonic()");

            let result = self.visit_children(ctx);

            // for str in &result {
            //     log::info!("{}", str);
            // }

            result
        }
    }

    // vector of instructions
    let mut asm_application: Vec<AsmRecord> = Vec::new();

    let mut visitor = DefaultAssemblerVisitor(Vec::new(), 
        AsmRecord::new(String::from(""), InstructionType::Unknown, 0xFF, 0xFF, 0, String::from(""), IoDestination::UNKNOWN),
        token_storage,
        asm_application);
    
    let visitor_result = visitor.visit(&*root);

    log::info!("{:?}", visitor_result);

    // assert_eq!(
    //     result.unwrap().to_string_tree(&*parser),
    //     "(csvFile (hdr (row (field V123) , (field V2) \\n)) (row (field d1) , (field d2) \\n))"
    // );

    //dissassemble();

    // fn lul(&self) {
        //     // for asm_record in asm_application {
        //     //     log::info!("{:?}", asm_record);
        //     // }
        // }

    // for asm_record in asm_application {
    //     log::info!("{:?}", asm_record);
    // }

    const EXECUTE: bool = true;
    if EXECUTE {
        // vector of instructions
        let mut asm_application: Vec<AsmRecord> = Vec::new();

        // create an application as a vector of instructions (mnemonics)
        //application_instruction_source(&mut asm_application);

        let mut asm_file_path: String = String::new();
        //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank1.hex");
        //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank2.hex") {
        //hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication1/GccApplication1.hex");
        asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_1.asm");

        // this function uses a parser to retrieve source code from a .asm file and
        // it will produce AsmRecord out of the source code. The AsmRecords can then
        // be put into an assembler that will fill a segment with machine code assembled
        // from the instructions
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
            let temp_pc: i32 = cpu.pc - 0x02;

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
        let mut hex_file_path: String = String::new();
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
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Error at load hex file!",
                ));
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
            while index < segment_0.data.len() {
                let word: u16 = rdr.read_u16::<LittleEndian>().unwrap().into();
                index += 2;

                log::trace!("word: {:#06x} {:b}", word, word);

                let mut value_storage: HashMap<char, u16> = HashMap::new();
                let instruction: &InstructionDefinition =
                    decode_instruction(word, INSTRUCTIONS, &UNKNOWN, &mut value_storage);

                log::info!("instruction {:?}", instruction.instruction_type);
                if instruction.instruction_type == InstructionType::EOR
                    || instruction.instruction_type == InstructionType::CLR
                {
                    log::info!(
                        "EOR and CLR similar. CLI is implemented by EOR the register with itself!"
                    );
                }

                match_instruction(
                    &instruction,
                    &mut rdr,
                    &word,
                    &mut index,
                    &mut value_storage,
                );
            }
        }
    }

    Ok(())
}

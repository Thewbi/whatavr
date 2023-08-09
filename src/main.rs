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
use antlr_rust::lazy_static;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::ParseTreeListener;
use antlr_rust::InputStream;
use antlr_rust::tree::ParseTreeVisitorCompat;
use env_logger::{Builder, Target};
use instructions::instruction_definition::InstructionDefinition;
use log::LevelFilter;

use std::fs;
use std::rc::Rc;

use crate::assembler::asm_encoder::AsmEncoder;
use crate::assembler::asm_record::AsmRecord;
use crate::assembler::io_destination::IoDestination;
use crate::cpu::cpu::CPU;
use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;
use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::instructions::decode::decode_instruction;
use crate::instructions::instruction_type::InstructionType;
use crate::instructions::instructions::INSTRUCTIONS;
use crate::instructions::instructions::UNKNOWN;
use crate::instructions::process::*;
use crate::parser::assemblerparser::InstructionContext;
use crate::parser::assemblerparser::ParamContext;
use crate::parser::assemblerparser::assemblerParserContextType;
use crate::parser::assemblerparser::Asm_fileContextAll;
use crate::parser::assemblervisitor::assemblerVisitorCompat;
use crate::parser::assemblerparser::Preprocessor_directiveContext;
use antlr_rust::tree::ParseTree;

use crate::fs::File;

use std::io::BufRead;
use crate::io::BufReader;

use byteorder::{LittleEndian, ReadBytesExt};

use regex::Regex;

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

    pub const RAMEND: u16 = 0x08ff;

    log::info!("whatavr starting ...");

    //
    // logging setup
    //

    init_logging();
    log_start();

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
        log::trace!("{}. {}", index + 1, line);

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
    // check files here: http://lab.antlr.org/ 
    // (erase the entire content in the lexer tab, paste the grammar into the parser tab,
    // use 'asm_file' as a start symbol)
    // 

    let mut asm_file_path: String = String::new();
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_1.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_2.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_3.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_4.asm");
    asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/call_and_return.asm"); // regression test
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/def_assembler_directive.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/expression.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/inc.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/intrinsic.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/jmp_instruction.asm"); // problem
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/jmp.asm"); // good for regression test (will increment r17 until overflow)
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/pin_change_interrupt_demo.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/preprocessor.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/push_pop.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/scratchpad.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/setup_stack.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/timer_polling_example.asm");
    //asm_file_path.push_str("C:/Program Files (x86)/Atmel/Studio/7.0/Packs/atmel/ATmega_DFP/1.7.374/avrasm/inc/m328Pdef.inc");

    let data = fs::read_to_string(asm_file_path).expect("Unable to read file");
    log::info!("\n{}", data);

    let input_stream: InputStream<&str> = InputStream::new(data.as_str());

    log::info!("test started");

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

    struct DefaultAssemblerVisitor {
        
        pub result_value: String,

        pub ident: u16,

        pub records: Vec<AsmRecord>,

        pub record: AsmRecord,

        pub text: String,

        pub last_terminal: String,

        pub intrinsic_usage: String,

        pub mnemonic: String,

        pub reg_1: String,

        pub reg_2: String,

        pub data: String,

        pub label: String,

        pub target_label: String,

        pub return_val: Vec<String>,

        pub constant_storage: HashMap<String, String>,

        pub preprocessor_directive: bool,

    }

    impl DefaultAssemblerVisitor {

        pub fn ascend_ident(&mut self) {
            self.ident = self.ident - 1;
        }

        pub fn descend_ident(&mut self, label: &str) {
            self.ident = self.ident + 1;
            for n in 0..self.ident {
                print!("  ");
            }
            println!("{}", label);
        }

        pub fn reset_self(&mut self) {
            self.result_value = String::default();
            self.ident = 0u16;
            self.text = String::default();
            self.last_terminal = String::default();
            self.intrinsic_usage = String::default();
            self.mnemonic = String::default();
            self.reg_1 = String::default();
            self.reg_2 = String::default();
            self.data = String::default();
            //self.label = String::default();
            self.record.clear();
            self.preprocessor_directive = bool::default();
        }

        // https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
        pub fn is_register_name(&self, input: &str) -> bool {
            //let re = Regex::new(r"r\d+").unwrap();

            //let re = Regex::new(r"^r(?:\\d|[12]\\d|3[01])$").unwrap();
            //let re = Regex::new(r"r(?:\\d|[12]\\d|3[01])").unwrap();
            //let re = Regex::new(r"r(\\d|[12]\\d|3[01])").unwrap();
            let re = Regex::new("^r(\\d|[12]\\d|3[01])$").unwrap();
            re.is_match(input)
        }

        pub fn parse_assembler_directive(&mut self, assembler_directive: &Vec<String>) {
            println!("parse_assembler_directive");

            if "cseg".eq(&assembler_directive[1]) {
                // ignored
                return;
            }

            // Set a symbolic name on a register.
            // The DEF directive allows the registers to be referred to through symbols. A defined symbol can be used
            // in the rest of the program to refer to the register it is assigned to. A register can have several symbolic
            // names attached to it. A symbol can be redefined later in the program.
            if "def".eq(&assembler_directive[1]) {
                self.constant_storage.insert(assembler_directive[2].to_string(), assembler_directive[4].to_string());
                return;
            }

            if "device".eq(&assembler_directive[1]) {
                // ignored
                return;
            }

            // Set a symbol equal to an expression.
            // The EQU directive assigns a value to a label. This label can then be used in later expressions. A label
            // assigned to a value by the EQU directive is a constant and can not be changed or redefined.
            if "equ".eq(&assembler_directive[1]) {
                self.constant_storage.insert(assembler_directive[2].to_string(), assembler_directive[4].to_string());
                return;
            }

            // C:/Program Files (x86)\Atmel\Studio\7.0\Packs\atmel\ATmega_DFP\1.7.374\avrasm\inc\m328Pdef.inc
            if "include".eq(&assembler_directive[1]) {

                let unwrapped_name = &assembler_directive[2].replace("\"", "");

                let mut asm_file_path: String = String::new();
                asm_file_path.push_str("C:/Program Files (x86)/Atmel/Studio/7.0/Packs/atmel/ATmega_DFP/1.7.374/avrasm/inc/");
                asm_file_path.push_str(unwrapped_name);

                log::info!("Including \"{}\"", &asm_file_path.clone());

                let data = fs::read_to_string(asm_file_path).expect("Unable to read file");
                log::info!("\n{}", data);

                let input_stream: InputStream<&str> = InputStream::new(data.as_str());

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

                let mut visitor = DefaultAssemblerVisitor { 
                    result_value: String::default(), 
                    ident: 0u16,
                    records: Vec::new(),
                    record: AsmRecord::default(),
                    text: String::default(), 
                    last_terminal: String::default(),
                    intrinsic_usage: String::default(),
                    mnemonic: String::default(),
                    reg_1: String::default(),
                    reg_2: String::default(),
                    data: String::default(),
                    label: String::default(),
                    target_label: String::default(),
                    return_val: Vec::new(),
                    constant_storage: HashMap::new(),
                    preprocessor_directive: bool::default(),
                };
                visitor.record.clear();
                
                let visitor_result = visitor.visit(&*root);
            
                log::info!("{:?}", visitor_result);

                // copy the constant storage into the parent visitor
                // https://users.rust-lang.org/t/is-there-a-nice-way-to-copy-the-contents-of-an-entire-hashmap-to-another/52647/4
                self.constant_storage.extend(visitor.constant_storage);

                return;
            }

            if "org".eq(&assembler_directive[1]) {
                // ignored
                return;
            }

            panic!();
        }

    }

    impl<'i> ParseTreeVisitorCompat<'i> for DefaultAssemblerVisitor {

        type Node = assemblerParserContextType;
        type Return = Vec<String>;

        fn temp_result(&mut self) -> &mut Self::Return {
            &mut self.return_val
        }

        fn visit_terminal(&mut self, node: &antlr_rust::tree::TerminalNode<'i, Self::Node>) -> Self::Return {
            let terminal_text = node.get_text();
            log::trace!("'{}'", terminal_text);
            if terminal_text != ":" && terminal_text != "," && terminal_text != "\r\n" {
                if self.last_terminal != terminal_text {
                    self.last_terminal.push_str(terminal_text.as_str());
                }
            }
            return vec![terminal_text];
        }

        fn visit_error_node(&mut self, _node: &antlr_rust::tree::ErrorNode<'i, Self::Node>) -> Self::Return {
            Self::Return::default()
        }

        fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
            // https://stackoverflow.com/questions/40792801/what-is-the-best-way-to-concatenate-vectors-in-rust
            //let c: Vec<&'i str> = aggregate.iter().cloned().chain(next.iter().cloned()).collect(); // Cloned
            //c
            let c: Vec<String> = aggregate.iter().cloned().chain(next.iter().cloned()).collect(); // Cloned
            c
        }

        fn should_visit_next_child(
            &self,
            node: &<Self::Node as antlr_rust::parser::ParserNodeType<'i>>::Type,
            current: &Self::Return,
        ) -> bool {
            true
        }

    }

    impl<'i> assemblerVisitorCompat<'i> for DefaultAssemblerVisitor {

        fn visit_asm_file(&mut self, ctx: &parser::assemblerparser::Asm_fileContext<'i>) -> Self::Return {
            let mut children_result = self.visit_children(ctx);
            // the very last line contained a label definition
            // force a NOP operation and place the label onto that NOP operation
            if self.label != "" {
                let asm_record: AsmRecord = AsmRecord::new(
                    self.label.clone(), 
                    InstructionType::NOP, 
                    0xFF, 
                    0xFF, 
                    0, 
                    String::from(""), 
                    IoDestination::UNKNOWN);
                self.records.push(asm_record);
            }
            children_result
        }

        fn visit_row(&mut self, ctx: &parser::assemblerparser::RowContext<'i>) -> Self::Return {
            self.descend_ident("visit_row");
            let children_result = self.visit_children(ctx);
            log::trace!("[exit_row] ...");
            // ignore preprocessor statements
            if self.preprocessor_directive {
                self.preprocessor_directive = false;
                self.ascend_ident();
                self.reset_self();
                return vec![];
            }
            // look for assembler directives
            // assembler directives are identified via a dot character
            let assembler_directive = ".".eq(&children_result[0]);
            if assembler_directive {
                self.parse_assembler_directive(&children_result);
                self.ascend_ident();
                self.reset_self();
                return vec![];
            }
            // do not deal with records that only consists of labels
            // instead insert the label into the next instruction
            if self.label != "" && self.mnemonic == "" {
                self.ascend_ident();
                self.reset_self();
                return vec![];
            }
            if self.reg_1 != "" {
                if self.reg_1.starts_with("r")
                {
                    self.record.reg_1 = self.reg_1[1..].parse::<u16>().unwrap();
                }
            }
            if self.reg_2 != "" {
                if self.reg_2.starts_with("r")
                {
                    self.record.reg_2 = self.reg_2[1..].parse::<u16>().unwrap();
                }
            }
            if self.data != "" {
                self.record.io_dest = IoDestination::from_string(self.data.as_str());
                if self.record.io_dest == IoDestination::UNKNOWN
                {
                    let parse_result = self.data.parse::<u16>();
                    if parse_result.is_ok() {
                        self.record.data = parse_result.unwrap();
                    } else  {
                        self.target_label = self.data.clone();
                    }
                }
            }
            // create an AsmRecord so it can be added to the application code
            let rec = AsmRecord::new(
                self.label.clone(), 
                InstructionType::from_string(&self.mnemonic.as_str()), 
                self.record.reg_1, 
                self.record.reg_2, 
                self.record.data, 
                self.target_label.clone(), 
                self.record.io_dest);
            self.records.push(rec);
            self.ascend_ident();
            self.reset_self();
            self.label = String::default();
            return vec![];
        }

        fn visit_instruction(&mut self, ctx: &InstructionContext<'i>) -> Self::Return {
            self.descend_ident("visit_instruction");
            let children_result = self.visit_children(ctx);
            self.ascend_ident();
            children_result
		}

        fn visit_macro_usage(&mut self, ctx: &parser::assemblerparser::Macro_usageContext<'i>) -> Self::Return {
            self.descend_ident("visit_macro_usage");
            let children_result = self.visit_children(ctx);
            self.ascend_ident();
            children_result
        }

        fn visit_label_definition(&mut self, ctx: &parser::assemblerparser::Label_definitionContext<'i>) -> Self::Return {
            self.descend_ident("visit_label_definition");
            let children_result = self.visit_children(ctx);
            self.label = self.last_terminal.clone();
            self.last_terminal = String::default();
            self.ascend_ident();
            children_result
        }

        fn visit_param(&mut self, ctx: &ParamContext<'i>) -> Self::Return {
            self.descend_ident("visit_param");
            let children_result = self.visit_children(ctx);
            let result_join = children_result.join("");
            // try to resolve constants
            if self.constant_storage.contains_key(&result_join) {
                let constant_value = self.constant_storage.get(&result_join).unwrap();
                if self.reg_1 == "" {
                    self.reg_1 = constant_value.to_string();
                } else if self.reg_2 == "" {
                    self.reg_2 = constant_value.to_string();
                }
                self.last_terminal = String::default();
                self.ascend_ident();
                return vec![];
            }

            // if self.reg_1 == "" && self.is_register_name(&self.last_terminal) {
            //     self.reg_1 = self.last_terminal.clone();
            // }

            if self.reg_1 == "" && self.is_register_name(&self.last_terminal) {
                self.reg_1 = self.last_terminal.clone();
            } else if self.reg_2 == "" && self.is_register_name(&self.last_terminal) {
                self.reg_2 = self.last_terminal.clone();
            } else {
                self.data = children_result.join("");
            }

            // if self.reg_1 == "" && self.last_terminal.starts_with("r") {
            //     self.reg_1 = self.last_terminal.clone();
            // } else if self.reg_2 == "" && self.last_terminal.starts_with("r") {
            //     self.reg_2 = self.last_terminal.clone();
            // } else {
            //     self.data = children_result.join("");
            // }

            self.last_terminal = String::default();
            self.ascend_ident();
            return vec![];
		}

        fn visit_parameter(&mut self, ctx: &parser::assemblerparser::ParameterContext<'i>) -> Self::Return {
            self.descend_ident("visit_parameter");
            let children_result = self.visit_children(ctx);

            // if self.reg_1 == "" && self.is_register_name(&self.last_terminal) {
            //     self.reg_1 = self.last_terminal.clone();
            // }

            if self.reg_1 == "" && self.is_register_name(&self.last_terminal) {
                self.reg_1 = self.last_terminal.clone();
            } else if self.reg_2 == "" && self.is_register_name(&self.last_terminal) {
                self.reg_2 = self.last_terminal.clone();
            } else {
                self.data = self.last_terminal.clone();
            }

            // if self.reg_1 == "" && self.last_terminal.starts_with("r") {
            //     self.reg_1 = self.last_terminal.clone();
            // } else if self.reg_2 == "" && self.last_terminal.starts_with("r") {
            //     self.reg_2 = self.last_terminal.clone();
            // } else {
            //     self.data = self.last_terminal.clone();
            // }

            self.last_terminal = String::default();
            self.ascend_ident();
            children_result
        }

        fn visit_macro_placeholder(&mut self, ctx: &parser::assemblerparser::Macro_placeholderContext<'i>) -> Self::Return {
            self.descend_ident("visit_macro_placeholder");
            let children_result = self.visit_children(ctx);
            self.ascend_ident();
            children_result
        }

        fn visit_expression(&mut self, ctx: &parser::assemblerparser::ExpressionContext<'i>) -> Self::Return {
            self.descend_ident("visit_expression");
            self.last_terminal.clear();
            let mut children_result = self.visit_children(ctx);
            if children_result.len() == 3usize {

                if "<<".eq(&children_result[1]) {

                    // lhs << rhs
                    let lhs_as_string = &children_result[0];
                    let op_as_string = &children_result[1];
                    let rhs_as_string = &children_result[2];

                    println!("lhs: {} op: {} rhs: {}", lhs_as_string, op_as_string, rhs_as_string);

                    let mut lhs: i16 = 0i16;
                    let lhs_parse_result = lhs_as_string.parse::<i16>();
                    if lhs_parse_result.is_ok() {
                        lhs = lhs_parse_result.unwrap();
                    }

                    let mut rhs: i16 = 0i16;
                    let rhs_parse_result = rhs_as_string.parse::<i16>();
                    if rhs_parse_result.is_ok() {
                        rhs = rhs_parse_result.unwrap();
                    }

                    let result: i16 = lhs << rhs;

                    self.ascend_ident();
                    return vec![result.to_string()];
                }
                else if "(".eq(&children_result[0]) && ")".eq(&children_result[2])
                {
                    children_result.remove(2);
                    children_result.remove(0);

                    self.ascend_ident();
                    return children_result;
                }
            }

            if children_result.len() == 1usize {

                // if the value is a number, return it
                let parse_result = children_result[0].parse::<u16>();
                if parse_result.is_ok() {
                    self.ascend_ident();
                    return children_result;
                }
                
                // DDC0
                if "DDC0".eq(&children_result[0]) {
                    children_result[0] = "1".to_string();

                    self.ascend_ident();
                    return children_result;
                }

            }

            self.ascend_ident();
            children_result
        }

        fn visit_asm_instrinsic_instruction(&mut self, ctx: &parser::assemblerparser::Asm_instrinsic_instructionContext<'i>) -> Self::Return {
            self.descend_ident("visit_asm_instrinsic_instruction");
            let children_result = self.visit_children(ctx);
            self.ascend_ident();
            children_result
        }

        fn visit_asm_intrinsic_usage(&mut self, ctx: &parser::assemblerparser::Asm_intrinsic_usageContext<'i>) -> Self::Return {
            self.descend_ident("visit_asm_intrinsic_usage");
            let children_result = self.visit_children(ctx);
            self.intrinsic_usage = self.last_terminal.clone();
            if "LOW(RAMEND)" == self.intrinsic_usage {
                let low_ramend: u16 = LOW!(RAMEND);
                self.last_terminal = low_ramend.to_string();
                return vec![low_ramend.to_string().clone()];
            }
            if "HIGH(RAMEND)" == self.intrinsic_usage {
                let high_ramend: u16 = HIGH!(RAMEND);
                self.last_terminal = high_ramend.to_string();
                return vec![high_ramend.to_string().clone()];
            }
            self.ascend_ident();
            children_result
        }

        /// is the rule that directly selects the TOKEN of an instruction (ADD; CALL, EOR; LDI; ...)
        fn visit_mnemonic(&mut self, ctx: &parser::assemblerparser::MnemonicContext<'i>) -> Self::Return {
            self.descend_ident("visit_mnemonic");

            let result = self.visit_children(ctx);

            self.mnemonic = self.last_terminal.clone();
            self.last_terminal = String::default();

            self.ascend_ident();

            result
        }

        fn visit_preprocessor_directive(&mut self, ctx: &Preprocessor_directiveContext<'i>) -> Self::Return {
            self.descend_ident("visit_preprocessor_directive");
            self.preprocessor_directive = true;
			let result = self.visit_children(ctx);
            self.ascend_ident();
            result
		}
    }

    let mut visitor = DefaultAssemblerVisitor { 
        result_value: String::default(), 
        ident: 0u16,
        records: Vec::new(),
        record: AsmRecord::default(),
        text: String::default(), 
        last_terminal: String::default(),
        intrinsic_usage: String::default(),
        mnemonic: String::default(),
        reg_1: String::default(),
        reg_2: String::default(),
        data: String::default(),
        label: String::default(),
        target_label: String::default(),
        return_val: Vec::new(),
        constant_storage: HashMap::new(),
        preprocessor_directive: bool::default(),
    };
    visitor.record.clear();
    
    let visitor_result = visitor.visit(&*root);

    log::info!("{:?}", visitor_result);

    // ATmega328p cpu
    let mut cpu: CPU = CPU {
        z: false,
        sph: 0x00u8,
        spl: 0x00u8,
        pc: 0x02i32,
        register_file: [0; 32usize],
        sram: [0; RAMEND as usize],
    };

    // the ihex segment which is filled with source code bytes by the assembler
    let mut assembler_segment: Segment = Segment::new();
    assembler_segment.address = 0u16;
    assembler_segment.size = 0u32;

    // convert the mnemonic instructions into bytes to store into the ihex segment
    let mut asm_encoder: AsmEncoder = AsmEncoder::new();
    //asm_encoder.labels.extend(visitor_result.labels);
    asm_encoder.assemble(&mut visitor.records, &mut assembler_segment);

    log::info!(">>> CPU program execution ...");

    // main loop that executes the instruction
    let mut done: bool = false;
    while !done {

        // get the current instruction
        let temp_pc: i32 = cpu.pc - 0x02;

        // check for end of code
        if assembler_segment.size <= temp_pc as u32 {
            log::info!("End of Code reached! Application Finished!");
            // log_end();
            // return Ok(());

            done = true;
            continue;
        }

        // execute the next instruction
        cpu.execute_instruction(&assembler_segment);
    }

    log::info!("<<< CPU program execution done.");

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

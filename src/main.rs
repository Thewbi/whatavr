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
use antlr_rust::tree::ParseTree;

use crate::fs::File;

use std::io::BufRead;
use crate::io::BufReader;

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

    pub const RAMEND: u16 = 0x08ff;

    log::info!("whatavr starting ...");

    // logging setup
    init_logging();
    log_start();

    struct Listener {}

    impl<'input> ParseTreeListener<'input, assemblerParserContextType> for Listener {
        // fn enter_every_rule(&mut self, ctx: &dyn assemblerParserContextType<'input>) {
        //     log::info!(
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

    let mut asm_file_path: String = String::new();
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_1.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_2.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_3.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_4.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/jmp.asm");
    asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/push_pop.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/call_and_return.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/inc.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/expression.asm");
    //asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/scratchpad.asm");

    let data = fs::read_to_string(asm_file_path).expect("Unable to read file");
    log::info!("\n{}", data);

    let input_stream: InputStream<&str> = InputStream::new(data.as_str());

    log::info!("test started");
    // vector of instructions
    //let mut asm_application: Vec<&AsmRecord/*<&str>*/> = Vec::new();
    //let mut asa: Vec<&AsmRecord/*<&str>*/> = Vec::new();

    //let rec1 = AsmRecord{ label: String::from("val"), reg_1: 0x00, reg_2: 0x00, instruction_type: InstructionType::Unknown, data: 0x00, target_label: "".to_string(), io_dest: IoDestination::UNKNOWN, idx: 0, text: "", reffff: &asm_application };
    //asm_application.push(&rec1);

    let token_factory = ArenaCommonFactory::default();
    let mut _lexer = parser::assemblerlexer::assemblerLexer::new_with_token_factory(
        //InputStream::new("V123,V2\nd1,d2\n".into()),
        input_stream,
        &token_factory,
    );
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = parser::assemblerparser::assemblerParser::new(token_source);
    //parser.add_parse_listener(Box::new(parser::assemblerlistenerimpl::assemblerListenerImpl{}));

    // let mut asm_records: Vec<AsmRecord> = Vec::new();
    // //let mut asm_apps: Rc<Vec<AsmRecord>> = asm_application.into();
    // //asm_application.push(AsmRecord::new(String::from(""), InstructionType::Unknown, 0xFF, 0xFF, 0, String::from(""), IoDestination::UNKNOWN));

    // //let mut asm_application: Vec<Rc<AsmRecord>> = Vec::new();
    // //let mut asm_application: Vec<AsmRecord> = Vec::new();
    // //let bbox = Box::new(asm_application);

    // let mut listener_impl = parser::assemblerlistenerimpl::assemblerListenerImpl {
    //     reg_1: String::default(),
    //     reg_2: String::default(),
    //     data: String::default(),
    //     instruction: String::default(),
    //     last_terminal: String::default(),
    //     mnemonic: String::default(),
    //     asm_records: asm_records,
    //     //asm_records: bbox,
    //     //asm_apps: asm_apps,
    // };

    // // https://dhghomon.github.io/easy_rust/Chapter_53.html
    // // put the listener onto the heap
    // let listener_box = Box::new(listener_impl);

    // parser.add_parse_listener(listener_box);

    // //let test = (*listener_box).get_clone();

    // log::info!("start parsing");

    let result = parser.asm_file();
    assert!(result.is_ok());

    

    let root: Rc<Asm_fileContextAll> = result.unwrap();

    //recurse_node(root);

    // for child in root.get_children() {
    //     log::info!("{:?}", child);
    // }

    //assemblerTreeWalker parse_tree_walker = assemblerTreeWalker::new();

    //let ptl: assemblerListenerImpl;

    //let parse_tree_walker: assemblerTreeWalker<'input, 'a> = assemblerTreeWalker::default();
    //let parse_tree_walker: assemblerTreeWalker;
    //parse_tree_walker.walk(ptl, &root);

    //assemblerTreeWalker::walk(Box::new(ptl), result);
    //assemblerTreeWalker::walk(Box::new(ptl), &result);
    //assemblerTreeWalker::walk(Box::new(ptl), root);
    //assemblerTreeWalker::walk(Box::new(ptl), &root);

    //assemblerTreeWalker::walk(Box::new(ptl), root.);

    //log::info!("string tree: {}", root.to_string_tree(&*parser));

    // fn recurse<'a>(cc: &'a Rc<dyn assemblerParserContext>) {
    //     log::info!("cc: {:?}", cc)
    // }

    //unsafe {
        // log::info!("{:?}", root);
        // //let child_0 = root.get_child(0);
        // //recurse(child_0.unwrap());
        // for child in root.get_children() {
        //     //log::info!("child {:?}", child);

        //     let cc: Rc<dyn assemblerParserContext> = child;

        //     //log::info!("cc {:?}", cc.get_text());

        //     //found struct `Rc<dyn assemblerParserContext<'_, TF = ArenaFactory<'_, CommonTokenFactory, GenericToken<Cow<'_, str>>>, Ctx

        //     //recurse_child(child);

        //     recurse(&cc);

        //     //let r = Rc::clone(&cc);
        //     //recurse(r);

        //     // for cchild in cc.get_children() {
        //     //     log::info!("cchild {:?}", cchild);
        //     // }
        // }
    //}

    //recurse_node(root)

    //root.get_rule_names();


/* */ 
    //let res = Vec<& str>();

    //let asm_record: AsmRecord;

    //struct DefaultAssemblerVisitor<'i>(Vec<&'i str>);

    // structs with parethesis are tuple structs (https://stackoverflow.com/questions/49716865/what-are-structs-with-round-brackets-in-rust-for)
    //struct DefaultAssemblerVisitor<'i>(Vec<&'i str>, AsmRecord<&'i str>, HashMap<isize, String>, &'i mut Vec<&'i AsmRecord<&'i str>>);
    //struct DefaultAssemblerVisitor<'i>(Vec<&'i str>, AsmRecord<'i, &'i str>, HashMap<isize, String>, Vec<&'i AsmRecord<'i, &'i str>>);

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
        //pub data: u16,

        pub label: String,

        pub target_label: String,

    }

    impl DefaultAssemblerVisitor {

        pub fn ascend_ident(&mut self) {
            //return;

            self.ident = self.ident - 1;
        }

        pub fn descend_ident(&mut self, label: &str) {

            //return;

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
            //self.data = 0u16;
            self.label = String::default();

            self.record.clear();
        }

    }

    impl<'i> ParseTreeVisitorCompat<'i> for DefaultAssemblerVisitor {

        type Node = assemblerParserContextType;
        //type Return = Vec<&'i str>;
        //type Return = Vec<&'i String>;
        //type Return = Vec<String>;

        //type Return = Vec<&'i AsmRecord<'i, &'i str>>;
        //type Return = Vec<&'i AsmRecord>;
        type Return = String;

        //let temp: &mut Vec<&'i AsmRecord<&'i str>>;

        fn temp_result(&mut self) -> &mut Self::Return {
        //fn temp_result(&mut self) -> Self::Return {
            
            //&mut self.3

            //&mut vec![&String::from(self.0[0])]

            //return &mut vec![&String::from("ADD")];

            //self.

            //todo!()

            //return vec![];
            //return &mut Self::Return::default();
            //()

            //return self.3;

            //&mut vec![AsmRecord<&str>>]
            //&mut Vec<&AsmRecord<&str>>

            //return self.3;
            //&mut self.3

            &mut self.result_value

        }

        // fn visit(&mut self, node: &<Self::Node as antlr_rust::parser::ParserNodeType<'i>>::Type) -> Self::Return {
            
        //     //log::info!("visit(): {:?} child_count: {:?}", node, node.get_child_count());
            
        //     self.visit_node(node);
            
        //     self.temp_result().to_vec()
        // }

        fn visit_terminal(&mut self, node: &antlr_rust::tree::TerminalNode<'i, Self::Node>) -> Self::Return {

            let terminal_text = node.get_text();

            //log::info!("'{}'", terminal_text);

            if terminal_text != ":" && terminal_text != "," && terminal_text != "\r\n" {

                if self.last_terminal != terminal_text {
                    self.last_terminal.push_str(terminal_text.as_str());
                }
            }

            String::default()

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
            //let c: Vec<&'i str> = aggregate.iter().cloned().chain(next.iter().cloned()).collect(); // Cloned
            //c

            // let c: Vec<String> = aggregate.iter().cloned().chain(next.iter().cloned()).collect(); // Cloned
            // c

            //return vec![];
            String::default()
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
            //log::info!("visit_asm_file()");
            //println!("visit_asm_file");

            let mut children_result = self.visit_children(ctx);


            // the very last line contained a label definition
            // force a NOP operation and place the label onto that NOP operation
            if self.label != "" {
                let asm_record: AsmRecord = AsmRecord::new(self.label.clone(), InstructionType::NOP, 0xFF, 0xFF, 0, String::from(""), IoDestination::UNKNOWN);

                self.records.push(asm_record);
            }

            children_result
        }

        fn visit_row(&mut self, ctx: &parser::assemblerparser::RowContext<'i>) -> Self::Return {

            self.descend_ident("visit_row");

            //log::info!("visit_row()");

            //self.visit_children(ctx)

            // if 1usize <= ctx.get_child_count() {
                
            //     let first_child = ctx.get_child(0usize);

            //     log::info!("{:?}", first_child);
            // }

            // let children_result = self.visit_children(ctx);

            // log::info!("{:?}", children_result);

            // children_result



            // type Return = Vec<&'i AsmRecord<&'i str>>;



            let mut children_result = self.visit_children(ctx);

            





            log::trace!("[exit_row] ...");

            // do not deal with records that only consists of labels
            // instead insert the label into the next instruction
            if self.label != "" && self.mnemonic == "" {
                return String::default();
            }

            // if 0xFF == self.record.reg_1 {

            //     if self.last_terminal.starts_with("r")
            //     {
            //         self.record.reg_1 = self.last_terminal[1..].parse::<u16>().unwrap();
            //     }
            //     else if "" != self.last_terminal && "," != self.last_terminal && "\r\n" != self.last_terminal 
            //     {
            //         self.record.data = self.last_terminal.parse::<u16>().unwrap();
            //     }

            // } else if 0xFF == self.record.reg_2 {

            //     if self.last_terminal.starts_with("r")
            //     {
            //         self.record.reg_2 = self.last_terminal[1..].parse::<u16>().unwrap();
            //     } 
            //     else if "" != self.last_terminal && "," != self.last_terminal && "\r\n" != self.last_terminal 
            //     {
            //         self.record.data = self.last_terminal.parse::<u16>().unwrap();
            //     }

            // }

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
                        self.record.data = self.data.parse::<u16>().unwrap();
                        //self.data = self.data.parse::<u16>().unwrap();
                    } else  {
                        //self.record.target_label = self.data.clone();
                        self.target_label= self.data.clone();
                    }
                }
            }





            //let rec = AsmRecord::new(String::from(""), InstructionType::UNKNOWN, 0xFF, 0xFF, 0, String::from(""), IoDestination::UNKNOWN);
            let rec = AsmRecord::new(
                self.label.clone(), 
                InstructionType::from_string(&self.mnemonic.as_str()), 
                self.record.reg_1, 
                self.record.reg_2, 
                self.record.data, 
                self.target_label.clone(), 
                self.record.io_dest);
            
            //rec.label.push(self.label.clone());

            self.records.push(rec);


            self.ascend_ident();

            self.reset_self();


            // log::info!("{:?}", children_result);
            //log::info!("{:?}", self.1);

            //self.1.emit();

            //children_result.push(self.1);
            //children_result.push(&mut self.1);
            //children_result.push(&AsmRecord{ label: String::from("val"), reg_1: 0x00, reg_2: 0x00, instruction_type: InstructionType::Unknown, data: 0x00, target_label: "".to_string(), io_dest: IoDestination::UNKNOWN, idx: 0, text: "" });

           // let ar = AsmRecord{ label: String::from("val"), reg_1: 0x00, reg_2: 0x00, instruction_type: InstructionType::Unknown, data: 0x00, target_label: "".to_string(), io_dest: IoDestination::UNKNOWN, idx: 0, text: "" };

            
            
            //self.3.push(&self.1);
            //self.3.push(&self.1);

            //log::info!("00: {:?}", self.3.get(0));

            //self.temp_result().push(&self.1);
            //self.temp_result().push(&ar);
            
            //log::info!("00: {:?}", self.temp_result().get(0));

           // let testttt = self.temp_result().get(0);
            //log::info!("00: {:?}", testttt);

            //self.aggregate_results(children_result, vec![]);
            //self.aggregate_results(children_result, self.temp_result());

            // clear for the next record
            //self.1.clear();


            //children_result
            //return vec![&self.1];
            //return vec![];

            // works
            //return children_result;

            // works
            //return vec![];
            String::default()

            //self.3

            //return vec![&AsmRecord{ label: String::from("val"), reg_1: 0x00, reg_2: 0x00, instruction_type: InstructionType::Unknown, data: 0x00, target_label: "".to_string(), io_dest: IoDestination::UNKNOWN, idx: 0, text: "" }];

        }

        fn visit_instruction(&mut self, ctx: &InstructionContext<'i>) -> Self::Return {
            self.descend_ident("visit_instruction");
			//self.visit_children(ctx)

            //log::info!("visit_instruction()");

            // for child in ctx.get_children() {
            //     let child_result = self.visit(child);
            // }
            



            let children_result = self.visit_children(ctx);

            self.ascend_ident();

            //log::info!("visit_instruction() - {:?}", children_result);

            children_result
		}

        fn visit_macro_usage(&mut self, ctx: &parser::assemblerparser::Macro_usageContext<'i>) -> Self::Return {
            self.descend_ident("visit_macro_usage");
            //log::info!("visit_macro_usage()");
            //self.visit_children(ctx)

            let children_result = self.visit_children(ctx);

            //log::info!("visit_macro_usage() - {:?}", children_result);

            self.ascend_ident();

            children_result
        }

        fn visit_label_definition(&mut self, ctx: &parser::assemblerparser::Label_definitionContext<'i>) -> Self::Return {
            self.descend_ident("visit_label_definition");
            //log::info!("visit_label_definition()");
            //self.visit_children(ctx)

            let children_result = self.visit_children(ctx);

            //log::info!("visit_label_definition() - {:?}", children_result);

            self.label = self.last_terminal.clone();
            self.last_terminal = String::default();

            self.ascend_ident();

            children_result
        }

        // parameter inside an instruction
        fn visit_param(&mut self, ctx: &ParamContext<'i>) -> Self::Return {

            self.descend_ident("visit_param");

			//self.visit_children(ctx)

            let children_result = self.visit_children(ctx);

            

            //log::info!("{:?}", children_result);

            // let text = children_result[0];

            //self.text = children_result[0].to_owned();
            self.text = children_result;

            //log::info!("{}", node.symbol.text);

            // if 0xFF == self.record.reg_1 {

            //     if self.text.starts_with("r")
            //     {
            //         self.record.reg_1 = self.text[1..].parse::<u16>().unwrap();
            //     }
            //     else if "" != self.text && "," != self.text && "\r\n" != self.text 
            //     {
            //         self.record.data = self.text.parse::<u16>().unwrap();
            //     }

            // } else if 0xFF == self.record.reg_2 {

            //     if self.text.starts_with("r")
            //     {
            //         self.record.reg_2 = self.text[1..].parse::<u16>().unwrap();
            //     } 
            //     else if "" != self.text && "," != self.text && "\r\n" != self.text 
            //     {
            //         self.record.data = self.text.parse::<u16>().unwrap();
            //     }

            // }

            //let children_result = self.visit_children(ctx);


            if self.reg_1 == "" && self.last_terminal.starts_with("r") {
                self.reg_1 = self.last_terminal.clone();
            } else if self.reg_2 == "" && self.last_terminal.starts_with("r") {
                self.reg_2 = self.last_terminal.clone();
            } else {
                self.data = self.last_terminal.clone();
            }
    
            self.last_terminal = String::default();
            

            self.ascend_ident();



            //children_result
            //self.text
            String::default()
		}

        fn visit_parameter(&mut self, ctx: &parser::assemblerparser::ParameterContext<'i>) -> Self::Return {
            self.descend_ident("visit_parameter");
            //log::info!("visit_parameter()");

            

            let children_result = self.visit_children(ctx);


            if self.reg_1 == "" && self.last_terminal.starts_with("r") {
                self.reg_1 = self.last_terminal.clone();
            } else if self.reg_2 == "" && self.last_terminal.starts_with("r") {
                self.reg_2 = self.last_terminal.clone();
            } else {
                self.data = self.last_terminal.clone();
            }
    
            self.last_terminal = String::default();
            

            self.ascend_ident();

            children_result
        }

        fn visit_macro_placeholder(&mut self, ctx: &parser::assemblerparser::Macro_placeholderContext<'i>) -> Self::Return {
            self.descend_ident("visit_macro_placeholder");
            //log::info!("visit_macro_placeholder()");
            let children_result = self.visit_children(ctx);

            self.ascend_ident();

            children_result
        }

        fn visit_expression(&mut self, ctx: &parser::assemblerparser::ExpressionContext<'i>) -> Self::Return {
            self.descend_ident("visit_expression");
            //log::info!("visit_expression()");
            let children_result = self.visit_children(ctx);

            self.ascend_ident();

            children_result
        }

        fn visit_asm_instrinsic_instruction(&mut self, ctx: &parser::assemblerparser::Asm_instrinsic_instructionContext<'i>) -> Self::Return {
            
            self.descend_ident("visit_asm_instrinsic_instruction");
            //log::info!("visit_asm_instrinsic_instruction()");
            //self.visit_children(ctx)

            let children_result = self.visit_children(ctx);

            // self.intrinsic_usage = self.last_terminal.clone();

            // if "LOW(RAMEND)" == self.intrinsic_usage {
            //     let low_ramend: u16 = LOW!(RAMEND);
            //     self.last_terminal = low_ramend.to_string();
            // }

            // if "HIGH(RAMEND)" == self.intrinsic_usage {
            //     let high_ramend: u16 = HIGH!(RAMEND);
            //     self.last_terminal = high_ramend.to_string();
            // }

            //log::info!("visit_asm_instrinsic_instruction() - {:?}", children_result);

            self.ascend_ident();

            children_result
        }

        fn visit_asm_intrinsic_usage(&mut self, ctx: &parser::assemblerparser::Asm_intrinsic_usageContext<'i>) -> Self::Return {
            
            self.descend_ident("visit_asm_intrinsic_usage");
            //log::info!("visit_asm_intrinsic_usage()");
            
            let children_result = self.visit_children(ctx);

            self.intrinsic_usage = self.last_terminal.clone();

            if "LOW(RAMEND)" == self.intrinsic_usage {
                let low_ramend: u16 = LOW!(RAMEND);
                self.last_terminal = low_ramend.to_string();
                //self.data = low_ramend.to_string();
            }

            if "HIGH(RAMEND)" == self.intrinsic_usage {
                let high_ramend: u16 = HIGH!(RAMEND);
                self.last_terminal = high_ramend.to_string();
                //self.data = high_ramend.to_string();
            }

            self.ascend_ident();

            children_result

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
            self.descend_ident("visit_mnemonic");
            //log::info!("visit_mnemonic()");

            let result = self.visit_children(ctx);

            self.mnemonic = self.last_terminal.clone();
            self.last_terminal = String::default();

            self.ascend_ident();

            // for str in &result {
            //     log::info!("{}", str);
            // }

            result
        }
    }

    
    //let tempAsmRecord = AsmRecord::new(String::from(""), InstructionType::Unknown, 0xFF, 0xFF, 0, String::from(""), IoDestination::UNKNOWN, "", &asa);
        

    // let mut visitor = DefaultAssemblerVisitor(Vec::new(),
    //     tempAsmRecord,
    //     token_storage,
    //     //&mut asm_application);
    //     asm_application);

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
   
    // const EXECUTE: bool = true;
    // if EXECUTE {
    //     // vector of instructions
    //     //let mut asm_application: Vec<AsmRecord> = Vec::new();

    //     // create an application as a vector of instructions (mnemonics)
    //     //application_instruction_source(&mut asm_application);

    //     // let mut asm_file_path: String = String::new();
    //     // //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank1.hex");
    //     // //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank2.hex") {
    //     // //hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication1/GccApplication1.hex");
    //     // asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/asm_1.asm");

    //     // this function uses a parser to retrieve source code from a .asm file and
    //     // it will produce AsmRecord out of the source code. The AsmRecords can then
    //     // be put into an assembler that will fill a segment with machine code assembled
    //     // from the instructions
    //     //application_file_source(&mut asm_application);

    //     // // the ihex segment which is filled with source code bytes by the assembler
    //     // let mut assembler_segment: Segment = Segment::new();
    //     // assembler_segment.address = 0u16;
    //     // assembler_segment.size = 0u32;

    //     // // convert the mnemonic instructions into bytes to store into the ihex segment
    //     let mut asm_encoder: AsmEncoder = AsmEncoder::new();
    //     // //asm_encoder.assemble(&mut asm_application, &mut assembler_segment);

    //     //let asm_apps_clone = listener_impl.get_asm_records();
    //     //let asm_apps_clone = asm_application.clone();

    //     // let asm_record: &mut AsmRecord = listener_impl.get(0);
    //     // log::info!("{:?}", asm_record);

    //     //asm_encoder.assemble(&mut asm_application, &mut assembler_segment);
    //     //asm_encoder.assemble(&bbox, &mut assembler_segment);

    //     let mut asm_records: Vec<AsmRecord> = Vec::new();
    //     //let mut asm_apps: Rc<Vec<AsmRecord>> = asm_application.into();
    //     //asm_application.push(AsmRecord::new(String::from(""), InstructionType::Unknown, 0xFF, 0xFF, 0, String::from(""), IoDestination::UNKNOWN));

    //     //let mut asm_application: Vec<Rc<AsmRecord>> = Vec::new();
    //     //let mut asm_application: Vec<AsmRecord> = Vec::new();
    //     //let bbox = Box::new(asm_application);

    //     let mut listener_impl = parser::assemblerlistenerimpl::assemblerListenerImpl {
    //         reg_1: String::default(),
    //         reg_2: String::default(),
    //         data: String::default(),
    //         instruction: String::default(),
    //         last_terminal: String::default(),
    //         mnemonic: String::default(),
    //         label: String::default(),
    //         asm_records: asm_records,
    //         //asm_records: bbox,
    //         //asm_apps: asm_apps,
    //         asm_encoder: asm_encoder,
    //         intrinsic_usage: String::default(),
    //     };

    //     // https://dhghomon.github.io/easy_rust/Chapter_53.html
    //     // put the listener onto the heap
    //     let listener_box = Box::new(listener_impl);

    //     parser.add_parse_listener(listener_box);

    //     //let test = (*listener_box).get_clone();

    //     log::info!("start parsing");

    //     let result = parser.asm_file();
    //     assert!(result.is_ok());

    //     let root: Rc<Asm_fileContextAll> = result.unwrap();
    //     log::info!("string tree: {}", root.to_string_tree(&*parser));

    //     // // ATmega328p cpu
    //     // let mut cpu: CPU = CPU {
    //     //     z: false,
    //     //     sph: 0x00u8,
    //     //     spl: 0x00u8,
    //     //     pc: 0x02i32,
    //     //     register_file: [0; 32usize],
    //     //     sram: [0; RAMEND as usize],
    //     // };

    //     // // main loop that executes the instruction
    //     // let done: bool = false;
    //     // while !done {
    //     //     // get the current instruction
    //     //     let temp_pc: i32 = cpu.pc - 0x02;

    //     //     // check for end of code
    //     //     if assembler_segment.size <= temp_pc as u32 {
    //     //         log::info!("End of Code reached! Application Finished!");
    //     //         log_end();
    //     //         return Ok(());
    //     //     }

    //     //     // execute the next instruction
    //     //     cpu.execute_instruction(&assembler_segment);
    //     // }
    // }

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

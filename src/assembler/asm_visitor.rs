use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::InputStream;
use antlr_rust::tree::ParseTreeVisitorCompat;

use std::fs;
use std::rc::Rc;

use crate::HASHMAP;
use crate::HIGH;
use crate::assembler::asm_record::AsmRecord;
use crate::assembler::io_destination::IoDestination;
use crate::common::common_constants::RAMEND;
use crate::common::register_parser::is_register_name;
use crate::instructions::instruction_type::InstructionType;
use crate::parser;
use crate::parser::assemblerparser::InstructionContext;
use crate::parser::assemblerparser::ParamContext;
use crate::parser::assemblerparser::assemblerParserContextType;
use crate::parser::assemblerparser::Asm_fileContextAll;
use crate::parser::assemblervisitor::assemblerVisitorCompat;
use crate::parser::assemblerparser::Preprocessor_directiveContext;
use antlr_rust::tree::ParseTree;



pub struct DefaultAssemblerVisitor {
        
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

    pub preprocessor_directive: bool,

    pub debug_output: bool,

}

impl DefaultAssemblerVisitor {

    pub fn ascend_ident(&mut self) {
        if !self.debug_output {
            return;
        }
        self.ident = self.ident - 1;
    }

    pub fn descend_ident(&mut self, label: &str) {
        if !self.debug_output {
            return;
        }
        self.ident = self.ident + 1;
        for _n in 0..self.ident {
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
        self.record.clear();
        self.preprocessor_directive = bool::default();
    }

    pub fn parse_assembler_directive(&mut self, assembler_directive: &Vec<String>) {
        log::trace!("parse_assembler_directive");

        let asm_directive = assembler_directive[1].to_lowercase();

        if "cseg".eq(&asm_directive) {

            // ignored

        } else if "device".eq(&asm_directive) {

            // ignored

        } else if "def".eq(&asm_directive) {

            // Set a symbolic name on a register.
            // The DEF directive allows the registers to be referred to through symbols. A defined symbol can be used
            // in the rest of the program to refer to the register it is assigned to. A register can have several symbolic
            // names attached to it. A symbol can be redefined later in the program.
            
            let mut map = HASHMAP.lock().unwrap();
            map.insert(assembler_directive[2].to_string(), assembler_directive[4].to_string());
            
        } else if "equ".eq(&asm_directive) {

            // Set a symbol equal to an expression.
            // The EQU directive assigns a value to a label. This label can then be used in later expressions. A label
            // assigned to a value by the EQU directive is a constant and can not be changed or redefined.
            
            let mut map = HASHMAP.lock().unwrap();
            map.insert(assembler_directive[2].to_string(), assembler_directive[4].to_string());

        } else if "include".eq(&asm_directive) {

            // C:/Program Files (x86)\Atmel\Studio\7.0\Packs\atmel\ATmega_DFP\1.7.374\avrasm\inc\m328Pdef.inc

            let unwrapped_name: &String = &assembler_directive[2].replace("\"", "");

            let mut asm_file_path: String = String::new();
            // .inc files are resolved from the system include folder
            // .asm files are included from the current folder
            if unwrapped_name.ends_with(".inc") {
                asm_file_path.push_str("C:/Program Files (x86)/Atmel/Studio/7.0/Packs/atmel/ATmega_DFP/1.7.374/avrasm/inc/");
            } else {
                asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/");
            }
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
                preprocessor_directive: bool::default(),
                debug_output: false,
            };
            visitor.record.clear();
            
            let visitor_result = visitor.visit(&*root);
        
            log::trace!("{:?}", visitor_result);

            // insert all parsed AsmRecords into the parent
            if visitor.records.len() > 0 {
                self.records.append(&mut visitor.records);
            }

        } else if "org".eq(&asm_directive) {

            // ignored

        } else {

            panic!();

        }
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
        let c: Vec<String> = aggregate.iter().cloned().chain(next.iter().cloned()).collect(); // Cloned
        c
    }

    #[allow(dead_code, unused)]
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

        let children_result = self.visit_children(ctx);

        // the very last line contained a label definition
        // force a NOP operation and place the label onto that NOP operation
        if self.label != "" {

            // create a new record
            let asm_record: AsmRecord = AsmRecord::new(
                self.label.clone(), 
                InstructionType::NOP, 
                0xFF, 
                0xFF, 
                0, 
                String::from(""), 
                0x00i16,
                IoDestination::UNKNOWN);

            // store the record into the result
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
            if self.reg_1.to_lowercase().starts_with("r")
            {
                self.record.reg_1 = self.reg_1[1..].parse::<u16>().unwrap();
            } 
            else
            {
                let without_prefix = self.reg_1.trim_start_matches("0x");
                self.record.reg_1 = u16::from_str_radix(without_prefix, 16).unwrap();
            }
        }
        if self.reg_2 != "" {
            if self.reg_2.to_lowercase().starts_with("r")
            {
                self.record.reg_2 = self.reg_2[1..].parse::<u16>().unwrap();
            }
            else
            {
                let without_prefix = self.reg_2.trim_start_matches("0x");
                self.record.reg_2 = u16::from_str_radix(without_prefix, 16).unwrap();
            }
        }
        if self.data != "" {
            self.record.io_dest = IoDestination::from_string(self.data.as_str());
            if self.record.io_dest == IoDestination::UNKNOWN
            {

                // todo somehow find a way to use common::number_literal_to_u16() instead of this if else

                if self.data.to_lowercase().starts_with("0b")
                {
                    // parse binary
                    let without_prefix = self.data.trim_start_matches("0b");
                    self.record.data = u16::from_str_radix(without_prefix, 2).unwrap();
                } 
                else if self.data.to_lowercase().starts_with("0x")
                {
                    // parse hex
                    let without_prefix = self.data.trim_start_matches("0x");
                    self.record.data = u16::from_str_radix(without_prefix, 16).unwrap();
                } 
                else if self.data.starts_with("$")
                {
                    // parse hex
                    let without_prefix = self.data.trim_start_matches("$");
                    self.record.data = u16::from_str_radix(without_prefix, 16).unwrap();
                }
                else if self.data.starts_with("0")
                {
                    // parse octal
                    let without_prefix = self.data.trim_start_matches("0");
                    self.record.data = u16::from_str_radix(without_prefix, 8).unwrap();
                }
                else 
                {
                    // parse decimal
                    let parse_result = self.data.parse::<u16>();
                    if parse_result.is_ok() {
                        self.record.data = parse_result.unwrap();
                    } else  {
                        self.target_label = self.data.clone();
                    }
                }
            }
        }
        
        let mnemonic_upper_case = self.mnemonic.to_uppercase();
        let mnemonic_upper_case_as_string: &str = mnemonic_upper_case.as_str();

        let base_upper_case = self.data.to_uppercase();
        let base_upper_case_as_string: &str = base_upper_case.as_str();

        if mnemonic_upper_case_as_string == "LDS" {

            // perform special handling for the LDS command which has 2 variants

            let instruction_type: InstructionType;

            let without_prefix = self.reg_2.trim_start_matches("0x");
            let reg_2_as_u16: u16 = u16::from_str_radix(without_prefix, 16).unwrap();

            // if the second parameter is a value that fits into 7 bit, choose 
            // the 16 bit variant of LDS. Otherwise choose the 32 bit variant.
            if reg_2_as_u16 <= 0x7F {
                instruction_type = InstructionType::LDS_16bit;

                // // the 16 bit version only uses 4 bit to encode the registers 
                // // r0 to r31. Since 32 does not fit into 4 bit, an offset of
                // // 16 is used and only half the registers can be used in the
                // // 16 bit version
                // register_offset = 16u16;
            } else {
                instruction_type = InstructionType::LDS;
            }

            log::trace!("{:?}", instruction_type);

            // create an AsmRecord so it can be added to the application code
            let rec = AsmRecord::new(
                self.label.clone(), 
                instruction_type, 
                self.record.reg_1, 
                self.record.reg_2, 
                self.record.data, 
                self.target_label.clone(), 
                0x00i16,
                self.record.io_dest);
            self.records.push(rec);

        } else if mnemonic_upper_case_as_string == "STS" {

            // perform special handling for the STS command which has 2 variants

            let instruction_type: InstructionType;

            let without_prefix = self.reg_1.trim_start_matches("0x");
            let reg_1_as_u16: u16 = u16::from_str_radix(without_prefix, 16).unwrap();

            // if the second parameter is a value that fits into 7 bit, choose 
            // the 16 bit variant of LDS. Otherwise choose the 32 bit variant.
            if reg_1_as_u16 <= 0x7F {
                // 16 bit version
                instruction_type = InstructionType::STS_16bit;
            } else {
                // 32 bit version
                instruction_type = InstructionType::STS;
            }

            log::trace!("{:?}", instruction_type);

            // create an AsmRecord so it can be added to the application code
            let rec = AsmRecord::new(
                self.label.clone(), 
                instruction_type, 
                self.record.reg_1, 
                self.record.reg_2, 
                self.record.data, 
                self.target_label.clone(), 
                0x00i16,
                self.record.io_dest);
            self.records.push(rec);

        } else if mnemonic_upper_case_as_string == "ST" {

            // perform special handling for the ST command which has 11 variants

            let instruction_type: InstructionType;

            // expectation: 
            // self.record.data contains the pointer base (X, Y or Z)
            // self.record.reg_1 contains the register that holds the data

            if base_upper_case_as_string == "X" {
                instruction_type = InstructionType::ST_STD_X_1;
            } else if base_upper_case_as_string == "X+" {
                instruction_type = InstructionType::ST_STD_X_2;
            } else if base_upper_case_as_string == "-X" {
                instruction_type = InstructionType::ST_STD_X_3;
            } else if base_upper_case_as_string == "Y" {
                instruction_type = InstructionType::ST_STD_Y_1;
            } else if base_upper_case_as_string == "Y+" {
                instruction_type = InstructionType::ST_STD_Y_2;
            } else if base_upper_case_as_string == "-Y" {
                instruction_type = InstructionType::ST_STD_Y_3;
            } else if base_upper_case_as_string.starts_with("Y+") {
                instruction_type = InstructionType::ST_STD_Y_4;
            } else if base_upper_case_as_string == "Z" {
                instruction_type = InstructionType::ST_STD_Z_1;
            } else if base_upper_case_as_string == "Z+" {
                instruction_type = InstructionType::ST_STD_Z_2;
            } else if base_upper_case_as_string == "-Z" {
                instruction_type = InstructionType::ST_STD_Z_3;
            } else if base_upper_case_as_string.starts_with("Z+") {
                instruction_type = InstructionType::ST_STD_Z_4;
            } else {
                panic!("Unknown option \"{}\"", base_upper_case_as_string);
            }

            log::trace!("{:?}", instruction_type);

            // create an AsmRecord so it can be added to the application code
            let rec = AsmRecord::new(
                self.label.clone(), 
                instruction_type, 
                self.record.reg_1, 
                self.record.reg_2, 
                self.record.data, 
                self.target_label.clone(),
                0x00i16,
                self.record.io_dest);
            self.records.push(rec);

        } else {

            // create an AsmRecord so it can be added to the application code
            let rec = AsmRecord::new(
                self.label.clone(), 
                InstructionType::from_string(&self.mnemonic.as_str()), 
                self.record.reg_1, 
                self.record.reg_2, 
                self.record.data, 
                self.target_label.clone(),
                0x00i16,
                self.record.io_dest);
                
            self.records.push(rec);
        }
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

        if self.reg_1 == "" && is_register_name(&self.last_terminal) {
            self.reg_1 = self.last_terminal.clone();
        } else if self.reg_2 == "" && is_register_name(&self.last_terminal) {
            self.reg_2 = self.last_terminal.clone();
        } else {

            // https://users.rust-lang.org/t/check-if-string-is-numeric/16988/3
            // check if a string contains a number 
            let test = result_join.trim().parse::<f64>();
            match test {
                Ok(ok) => {
                    println!("it is a decimal ({})", ok);

                    self.data = children_result.join("");
                },
                Err(e) => {
                    println!("not a decimal ({})", e);

                    // try to resolve constants
                    let map = HASHMAP.lock().unwrap();
                    if map.contains_key(&result_join) {
                        let constant_value = map.get(&result_join).unwrap();
                        if self.reg_1 == "" {
                            self.reg_1 = constant_value.to_string();
                        } else if self.reg_2 == "" {
                            self.reg_2 = constant_value.to_string();
                        }
                        self.last_terminal = String::default();
                        self.ascend_ident();
                        return vec![];
                    }

                    //panic!("Cannot decode: \"{:?}\"! Define this constant or include C:\\Program Files (x86)\\Atmel\\Studio\\7.0\\packs\\atmel\\ATmega_DFP\\1.7.374\\avrasm\\inc\\m328def.inc", result_join);

                    self.target_label = self.last_terminal.clone();

                }, 
            }
        }

        self.last_terminal = String::default();
        self.ascend_ident();
        return vec![];
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

                if !self.debug_output {
                    println!("lhs: {} op: {} rhs: {}", lhs_as_string, op_as_string, rhs_as_string);
                }

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
            let low_ramend: u16 = crate::LOW!(RAMEND);
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
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::InputStream;

use std::fs;
use std::rc::Rc;

use crate::assembler::asm_record::AsmRecord;
use crate::common::common_constants::RAMEND;
use crate::common::number_literal_parser::char_literal_to_u16;
use crate::common::number_literal_parser::is_char_literal;
use crate::common::number_literal_parser::is_number_literal_i16;
use crate::common::number_literal_parser::is_number_literal_u16;
use crate::common::number_literal_parser::number_literal_to_i16;
use crate::common::number_literal_parser::number_literal_to_u16;
use crate::common::number_literal_parser::number_literal_to_u8;
use crate::common::register_parser::is_register_name;
use crate::common::register_parser::register_name_to_u16;
use crate::instructions::instruction_type::InstructionType;
use crate::parser;
use crate::parser::assemblerparser::assemblerParserContextType;
use crate::parser::assemblerparser::Asm_fileContextAll;
use crate::parser::assemblerparser::InstructionContext;
use crate::parser::assemblerparser::ParamContext;
use crate::parser::assemblervisitor::assemblerVisitorCompat;
use crate::DSEG_HASHMAP;
use crate::CSEG_HASHMAP;
use crate::HIGH;
use antlr_rust::tree::ParseTree;

use super::segment_mode;
use super::segment_mode::SegmentMode;

pub struct NewAssemblerVisitor {

    // result
    pub records: Vec<AsmRecord>,
    pub record: AsmRecord,

    // debug
    pub debug_output: bool,
    pub indent: u16,

    // traversal
    pub return_val: Vec<String>,

    // label
    pub label: String,
    pub labels: Vec<String>,

    // data segment as opposed to the code segment which is filled from the AsmRecords
    pub sram: [u8; RAMEND as usize],

    pub segment_mode: SegmentMode,

    pub cseg_org_pointer: u16,
    pub dseg_org_pointer: u16,
}

impl Default for NewAssemblerVisitor {
    fn default() -> Self {
        Self {
            records: Vec::new(),
            record: AsmRecord::default(),

            indent: 0u16,
            debug_output: false,

            return_val: Vec::new(),

            label: String::default(),
            labels: Vec::new(),

            sram: [0; RAMEND as usize],

            segment_mode: SegmentMode::CodeSegment,

            cseg_org_pointer: 0x00u16,
            dseg_org_pointer: 0x00u16,
        }
    }
}

impl<'i> NewAssemblerVisitor {

    pub fn ascend_ident(&mut self) {
        if !self.debug_output {
            return;
        }
        self.indent = self.indent - 1;
    }

    pub fn descend_ident(&mut self, label: &str) {
        if !self.debug_output {
            return;
        }
        self.indent = self.indent + 1;
        for _n in 0..self.indent {
            print!("  ");
        }
        println!("{}", label);
    }

    pub fn reset_self(&mut self) {
        self.record.clear();
    }

    // cr: ["st", "X", "+", "r17"]
    fn process_st(&mut self, /*ctx: &InstructionContext<'i>,*/
        visit_children_result: &<NewAssemblerVisitor as ParseTreeVisitorCompat>::Return,
        asm_record: &mut AsmRecord)
    {
        let mut idx: usize = 1usize;

        let val_1: &String = &visit_children_result[1];
        let val_2: &String = &visit_children_result[2];

        // can touples and matching be used here somehow?

        let mut base_upper_case_as_string: String = String::default();
        if val_1.eq("-")
        {
            base_upper_case_as_string.push_str("-");
            base_upper_case_as_string.push_str(val_2);
            idx += 2usize;
        }
        else if val_2.eq("+")
        {
            base_upper_case_as_string.push_str(val_1);
            base_upper_case_as_string.push_str("+");
            idx += 2usize;
        }
        else
        {
            base_upper_case_as_string.push_str(val_1);
            idx += 1usize;
        }

        if base_upper_case_as_string == "X" {
            asm_record.instruction_type = InstructionType::ST_STD_X_1;
        } else if base_upper_case_as_string == "X+" {
            asm_record.instruction_type = InstructionType::ST_STD_X_2;
        } else if base_upper_case_as_string == "-X" {
            asm_record.instruction_type = InstructionType::ST_STD_X_3;
        } else if base_upper_case_as_string == "Y" {
            asm_record.instruction_type = InstructionType::ST_STD_Y_1;
        } else if base_upper_case_as_string == "Y+" {
            asm_record.instruction_type = InstructionType::ST_STD_Y_2;
        } else if base_upper_case_as_string == "-Y" {
            asm_record.instruction_type = InstructionType::ST_STD_Y_3;
        } else if base_upper_case_as_string.starts_with("Y+") {
            asm_record.instruction_type = InstructionType::ST_STD_Y_4;
        } else if base_upper_case_as_string == "Z" {
            asm_record.instruction_type = InstructionType::ST_STD_Z_1;
        } else if base_upper_case_as_string == "Z+" {
            asm_record.instruction_type = InstructionType::ST_STD_Z_2;
        } else if base_upper_case_as_string == "-Z" {
            asm_record.instruction_type = InstructionType::ST_STD_Z_3;
        } else if base_upper_case_as_string.starts_with("Z+") {
            asm_record.instruction_type = InstructionType::ST_STD_Z_4;
        } else {
            panic!("Unknown option \"{}\"", base_upper_case_as_string);
        }

        let param_2: &String = &visit_children_result[idx];
        let param_2_as_number: u16;
        if is_register_name(param_2) {
            param_2_as_number = register_name_to_u16(param_2);
            asm_record.reg_1 = param_2_as_number;
        } else {
            param_2_as_number = number_literal_to_u16(&param_2);
            asm_record.data = param_2_as_number;
        }
        idx += 1usize;
    }

    // cr: ["ld", "r0", "X"]
    // cr: ["ld", "r25", "Z"]
    fn process_ld(&mut self, /*_ctx: &InstructionContext<'i>,*/
        visit_children_result: &<NewAssemblerVisitor as ParseTreeVisitorCompat>::Return,
        asm_record: &mut AsmRecord)
    {
        let mut idx: usize = 1usize;

        let param_1: &String = &visit_children_result[idx];
        let param_1_as_number: u16;
        if is_register_name(param_1) {
            param_1_as_number = register_name_to_u16(param_1);
            asm_record.reg_1 = param_1_as_number;
        } else {
            param_1_as_number = number_literal_to_u16(&param_1);
            asm_record.data = param_1_as_number;
        }

        let val_1: &String = &visit_children_result[2];
        let mut val_2: String = String::default();
        if visit_children_result.len() > 3
        {
            val_2 = visit_children_result[3].clone();
        }

        // can touples and matching be used here somehow?

        let mut base_upper_case_as_string: String = String::default();
        if val_1.eq("-")
        {
            base_upper_case_as_string.push_str("-");
            base_upper_case_as_string.push_str(&val_2);
            idx += 2usize;
        }
        else if val_2.eq("+")
        {
            base_upper_case_as_string.push_str(val_1);
            base_upper_case_as_string.push_str("+");
            idx += 2usize;
        }
        else
        {
            base_upper_case_as_string.push_str(val_1);
            idx += 1usize;
        }

        if base_upper_case_as_string == "X" {
            asm_record.instruction_type = InstructionType::LD_LDD_X_1;
        } else if base_upper_case_as_string == "X+" {
            asm_record.instruction_type = InstructionType::LD_LDD_X_2;
        } else if base_upper_case_as_string == "-X" {
            asm_record.instruction_type = InstructionType::LD_LDD_X_3;
        } else if base_upper_case_as_string == "Y" {
            asm_record.instruction_type = InstructionType::LD_LDD_Y_1;
        } else if base_upper_case_as_string == "Y+" {
            asm_record.instruction_type = InstructionType::LD_LDD_Y_2;
        } else if base_upper_case_as_string == "-Y" {
            asm_record.instruction_type = InstructionType::LD_LDD_Y_3;
        } else if base_upper_case_as_string.starts_with("Y+") {
            asm_record.instruction_type = InstructionType::LD_LDD_Y_4;
        } else if base_upper_case_as_string == "Z" {
            asm_record.instruction_type = InstructionType::LD_LDD_Z_1;
        } else if base_upper_case_as_string == "Z+" {
            asm_record.instruction_type = InstructionType::LD_LDD_Z_2;
        } else if base_upper_case_as_string == "-Z" {
            asm_record.instruction_type = InstructionType::LD_LDD_Z_3;
        } else if base_upper_case_as_string.starts_with("Z+") {
            asm_record.instruction_type = InstructionType::LD_LDD_Z_4;
        } else {
            panic!("Unknown option \"{}\"", base_upper_case_as_string);
        }
    }

    /// cr: ["lpm"]
    /// cr: ["lpm", "r16", "Z"]
    /// cr: ["lpm", "r24", "Z", "+"]
    fn process_lpm(&mut self, /*_ctx: &InstructionContext<'i>,*/
        visit_children_result: &<NewAssemblerVisitor as ParseTreeVisitorCompat>::Return,
        asm_record: &mut AsmRecord)
    {
        if 1 == visit_children_result.len()
        {
            asm_record.instruction_type = InstructionType::LPM_1;
        }
        else if 3 == visit_children_result.len()
        {
            asm_record.instruction_type = InstructionType::LPM_2;

            let param_1: &String = &visit_children_result[1usize];
            let param_1_as_number: u16;
            if is_register_name(param_1) {
                param_1_as_number = register_name_to_u16(param_1);
                asm_record.reg_1 = param_1_as_number;
            } else {
                panic!("invalid register");
            }
        }
        else if 4 == visit_children_result.len()
        {
            asm_record.instruction_type = InstructionType::LPM_3;

            let param_1: &String = &visit_children_result[1usize];
            let param_1_as_number: u16;
            if is_register_name(param_1) {
                param_1_as_number = register_name_to_u16(param_1);
                asm_record.reg_1 = param_1_as_number;
            } else {
                panic!("invalid register");
            }
        } 
        else 
        {
            panic!("Unknown option");
        }
    }

    fn process_adiw_sbiw(&mut self,
        instruction_type: InstructionType,
        visit_children_result: &<NewAssemblerVisitor as ParseTreeVisitorCompat>::Return,
        asm_record: &mut AsmRecord) {
        log::info!("cr: {:?}\n", visit_children_result);

        if 3usize == visit_children_result.len()
        {
            if is_register_name(&visit_children_result[1usize])
            {
                asm_record.reg_2 = register_name_to_u16(&visit_children_result[1usize]);
                asm_record.reg_1 = asm_record.reg_2 + 1;
            }
            asm_record.data = number_literal_to_u16(&visit_children_result[2usize]);
        }
        else if 5usize == visit_children_result.len() 
        {
            if is_register_name(&visit_children_result[1usize])
            {
                asm_record.reg_1 = register_name_to_u16(&visit_children_result[1usize]);
            }
            if is_register_name(&visit_children_result[3usize])
            {
                asm_record.reg_2 = register_name_to_u16(&visit_children_result[3usize]);
            }
            asm_record.data = number_literal_to_u16(&visit_children_result[4usize]);
        }
        else 
        {
            panic!("Invalid input for ADIW/SBIW instruction! {:?}", visit_children_result);
        }

        //asm_record.instruction_type = InstructionType::ADIW;
        asm_record.instruction_type = instruction_type;
    }

    fn parse_assembler_directive(&mut self, assembler_directive: &Vec<String>) {
        log::trace!("parse_assembler_directive");

        if assembler_directive.len() == 1
        {
            log::info!("test: {:?}\n", assembler_directive);
        }

        let asm_directive = assembler_directive[1].to_lowercase();

        if "cseg".eq(&asm_directive) {

            self.segment_mode = SegmentMode::CodeSegment;

        } else if "dseg".eq(&asm_directive) {

            self.segment_mode = SegmentMode::DataSegment;

        } else if "db".eq(&asm_directive) {

            if SegmentMode::DataSegment == self.segment_mode
            {
                // store the label in the sram (data_segment) hashmap
                let mut map = DSEG_HASHMAP.lock().unwrap();
                map.insert(self.label.clone(), self.dseg_org_pointer);

                log::info!("DSEG_HASHMAP {:?}\n", map);

                // copy the bytes into sram (data_segment)
                for i in 2..assembler_directive.len() 
                {    
                    let temp_data: u8 = number_literal_to_u8(&assembler_directive[i]);

                    log::info!("Storing value {:#04x} into SRAM at address {:#06x}\n", temp_data, self.dseg_org_pointer);

                    self.sram[self.dseg_org_pointer as usize] = temp_data;
                    self.dseg_org_pointer += 1u16;
                }

                self.label = String::default();
            }
            else 
            {
                log::info!("cseg_org_pointer: {}\n", self.cseg_org_pointer);

                let mut asm_record: AsmRecord = AsmRecord::default();

                for i in 2..assembler_directive.len()
                {
                    let temp = assembler_directive[i].as_bytes();

                    if temp[0] == b'\"'
                    {
                        asm_record.direct_data = [asm_record.direct_data.as_slice(), &temp[1..(temp.len() - 1usize)]].concat();
                    }
                    else 
                    {
                        asm_record.direct_data = [asm_record.direct_data.as_slice(), &temp].concat();
                    }
                }

                asm_record.label = self.label.clone();
                self.label = String::default();

                self.records.push(asm_record);
            }

        } else if "byte".eq(&asm_directive) {

            log::info!("byte\n");

            if SegmentMode::DataSegment == self.segment_mode
            {
                // store the label in the sram (data_segment) hashmap
                let mut map = DSEG_HASHMAP.lock().unwrap();
                map.insert(self.label.clone(), self.dseg_org_pointer);

                log::info!("DSEG LABEL: label: {} addr: {:#06x}\n", self.label, self.dseg_org_pointer);

                // log::info!("DSEG_HASHMAP {:?}\n", map);

                // // copy the bytes into sram (data_segment)
                // for i in 2..assembler_directive.len() 
                // {    
                //     let temp_data: u8 = number_literal_to_u8(&assembler_directive[i]);

                //     log::info!("Storing value {:#04x} into SRAM at address {:#06x}\n", temp_data, self.dseg_org_pointer);

                //     self.sram[self.dseg_org_pointer as usize] = temp_data;
                //     self.dseg_org_pointer += 1u16;
                // }

                self.label = String::default();
            }
            else 
            {
                panic!("BYTE in mode CodeSegment! Not implemented yet!");
            }
        
        } else if "device".eq(&asm_directive) {

            // ignored

        } else if "def".eq(&asm_directive) {

            // Set a symbolic name on a register.
            // The DEF directive allows the registers to be referred to through symbols. A defined symbol can be used
            // in the rest of the program to refer to the register it is assigned to. A register can have several symbolic
            // names attached to it. A symbol can be redefined later in the program.

            let mut map = CSEG_HASHMAP.lock().unwrap();
            map.insert(assembler_directive[2].to_string(), assembler_directive[4].to_string());

            log::info!("CSEG setting {} to {}\n", assembler_directive[2], assembler_directive[4]);

        } else if "equ".eq(&asm_directive) {

            // Set a symbol equal to an expression.
            // The EQU directive assigns a value to a label. This label can then be used in later expressions. A label
            // assigned to a value by the EQU directive is a constant and can not be changed or redefined.

            let mut map = CSEG_HASHMAP.lock().unwrap();
            map.insert(assembler_directive[2].to_string(), assembler_directive[4].to_string());

        } else if "include".eq(&asm_directive) {

            // C:/Program Files (x86)\Atmel\Studio\7.0\Packs\atmel\ATmega_DFP\1.7.374\avrasm\inc\m328Pdef.inc

            let unwrapped_name: &String = &assembler_directive[2].replace("\"", "");

            let mut asm_file_path: String = String::new();
            // .inc files are resolved from the system include folder
            // .asm files are included from the current folder
            // if unwrapped_name.ends_with(".inc") {
            //     asm_file_path.push_str("C:/Program Files (x86)/Atmel/Studio/7.0/Packs/atmel/ATmega_DFP/1.7.374/avrasm/inc/");
            // } else {
            //     asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/");
            // }
            //asm_file_path.push_str("/Users/bischowg/dev/rust/whatavr/test_resources/sample_files/asm/");
            asm_file_path.push_str("test_resources/sample_files/asm/");
            asm_file_path.push_str(unwrapped_name);

            log::info!("Including \"{}\"\n", &asm_file_path.clone());

            let data = fs::read_to_string(asm_file_path).expect("Unable to read file");
            log::trace!("\n{}\n", data);

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

            // new visitor
            let mut visitor: NewAssemblerVisitor = NewAssemblerVisitor::default();
            visitor.record.clear();

            let visitor_result = visitor.visit(&*root);

            log::trace!("{:?}", visitor_result);

            // insert all parsed AsmRecords into the parent
            if visitor.records.len() > 0 {
                self.records.append(&mut visitor.records);
            }

        } else if "org".eq(&asm_directive) {

            match self.segment_mode
            {
                SegmentMode::CodeSegment => {
                    self.cseg_org_pointer = number_literal_to_u16(&assembler_directive[2]);
                }
                SegmentMode::DataSegment => {
                    self.dseg_org_pointer = number_literal_to_u16(&assembler_directive[2]);
                }

                SegmentMode::UNKNOWN => panic!("Unknown segment mode!")
            }
            

        } else {

            panic!();

        }
    }

    fn process_asm_intrinsic_usage(&mut self, visit_children_result: Vec<String>) -> Vec<String>
    {
        log::info!("cr: {:?}\n", visit_children_result);

        let var_name = &visit_children_result[2];

        let mut cseg_map = CSEG_HASHMAP.lock().unwrap();
        if cseg_map.contains_key(var_name)
        {
            let var_value = cseg_map.get(var_name).unwrap();

            let var_as_u16:u16 = number_literal_to_u16(var_value);

            match visit_children_result[0].as_str() {
                "LOW" => {
                    let low_value: u16 = crate::LOW!(var_as_u16);
                    return vec![low_value.to_string().clone()];
                }
                "HIGH" => {
                    let high_value: u16 = crate::HIGH!(var_as_u16);
                    return vec![high_value.to_string().clone()];
                }
                _ => panic!("Unknown!"),
            }
        }

        let mut dseg_map = DSEG_HASHMAP.lock().unwrap();
        if dseg_map.contains_key(var_name)
        {
            let var_value = dseg_map.get(var_name).unwrap();
            match visit_children_result[0].as_str() {
                "LOW" => {
                    let low_value: u16 = crate::LOW!(var_value);
                    return vec![low_value.to_string().clone()];
                }
                "HIGH" => {
                    let high_value: u16 = crate::HIGH!(var_value);
                    return vec![high_value.to_string().clone()];
                }
                _ => panic!("Unknown!"),
            }
        }

        panic!("Unresolved symbol in AST visiting phase: {:?}", visit_children_result);

        visit_children_result
    }

    pub fn process_instruction(&mut self, visit_children_result: &Vec<String>) //-> Vec<String>
    {
        log::info!("cr: {:?}\n", visit_children_result);

        let mut asm_record: AsmRecord = AsmRecord::default();

        let mnemonic: &String = &visit_children_result[0];

        if mnemonic.to_uppercase().eq("ST")
        {
            self.process_st(&visit_children_result, &mut asm_record);
        }
        else if mnemonic.to_uppercase().eq("LD")
        {
            self.process_ld(&visit_children_result, &mut asm_record);
        }
        else if mnemonic.to_uppercase().eq("LPM")
        {
            self.process_lpm(&visit_children_result, &mut asm_record);
        }
        else if mnemonic.to_uppercase().eq("ADIW")
        {
            self.process_adiw_sbiw(InstructionType::ADIW, &visit_children_result, &mut asm_record);
        }
        else if mnemonic.to_uppercase().eq("SBIW")
        {
            self.process_adiw_sbiw(InstructionType::SBIW, &visit_children_result, &mut asm_record);
        }
        else if (
                mnemonic.to_uppercase().eq("BREQ") ||
                mnemonic.to_uppercase().eq("CALL") || 
                mnemonic.to_uppercase().eq("RJMP") ||
                mnemonic.to_uppercase().eq("JMP") 
            ) && is_number_literal_i16(&visit_children_result[1])
        {
            log::trace!("{:?}", visit_children_result);
            asm_record.target_address = number_literal_to_i16(&visit_children_result[1]) as i16;
            asm_record.instruction_type = InstructionType::from_string(mnemonic.as_str());
        }
        else
        {
            asm_record.instruction_type = InstructionType::from_string(mnemonic.as_str());

            if visit_children_result.len() > 1 {

                let param_1: &String = &visit_children_result[1];
                let param_1_as_number: u16;

                if is_register_name(param_1)
                {
                    param_1_as_number = register_name_to_u16(param_1);
                    asm_record.reg_1 = param_1_as_number;
                }
                else if is_number_literal_u16(param_1)
                {
                    param_1_as_number = number_literal_to_u16(&param_1);
                    asm_record.reg_1 = param_1_as_number;
                }
                else
                {
                    let param_as_string = param_1.to_string();

                    let mut label_resolved: bool = false;
                    
                    // try to resolve constants
                    let cseg_map = CSEG_HASHMAP.lock().unwrap();
                    log::trace!("CSEG_HASHMAP: {:?}\n", cseg_map);
                    if cseg_map.contains_key(&param_as_string)
                    {
                        let constant_value = cseg_map.get(&param_as_string).unwrap();
                        if is_number_literal_u16(constant_value)
                        {
                            asm_record.reg_1 = number_literal_to_u16(constant_value);
                            label_resolved = true;
                        }
                        else if is_register_name(constant_value)
                        {
                            asm_record.reg_1 = register_name_to_u16(constant_value);
                            label_resolved = true;
                        } 
                    }

                    let dseg_map = DSEG_HASHMAP.lock().unwrap();
                    log::trace!("DSEG_HASHMAP: {:?}\n", dseg_map);
                    if dseg_map.contains_key(&param_as_string)
                    {
                        let constant_value: &u16 = dseg_map.get(&param_as_string).unwrap();
                        asm_record.reg_1 = *constant_value;
                        label_resolved = true;
                        // if is_number_literal_u16(constant_value)
                        // {
                        //     asm_record.reg_1 = number_literal_to_u16(constant_value);
                        //     label_resolved = true;
                        // }
                        // else if is_register_name(constant_value)
                        // {
                        //     asm_record.reg_1 = register_name_to_u16(constant_value);
                        //     label_resolved = true;
                        // } 
                    }

                    if self.labels.contains(&param_as_string)
                    {
                        asm_record.target_label = param_as_string.clone();
                        label_resolved = true;
                    }

                    if !label_resolved
                    {
                        // log::warn!("Could not resolve label: {}\n", param_as_string);
                        //asm_record.target_label = param_as_string;
                        panic!("Could not resolve label: {}\n", param_as_string.clone());
                    }
                }
            }

            if visit_children_result.len() > 2 {

                let param_2: &String = &visit_children_result[2];
                let param_2_as_number: u16;

                if is_register_name(param_2)
                {
                    param_2_as_number = register_name_to_u16(param_2);
                    asm_record.reg_2 = param_2_as_number;
                }
                else if is_number_literal_u16(param_2)
                {
                    param_2_as_number = number_literal_to_u16(&param_2);
                    asm_record.data = param_2_as_number;
                }
                else if is_char_literal(param_2)
                {
                    param_2_as_number = char_literal_to_u16(&param_2);
                    asm_record.data = param_2_as_number;
                }
                else
                {
                    let param_as_string = param_2.to_string();

                    log::trace!("param_2: {}", param_as_string);

                    // try to resolve constants
                    let cseg_map = CSEG_HASHMAP.lock().unwrap();
                    let dseg_map = DSEG_HASHMAP.lock().unwrap();

                    log::trace!("dseg_map: {:?}", dseg_map);

                    if cseg_map.contains_key(&param_as_string)
                    {
                        let constant_value = cseg_map.get(&param_as_string).unwrap();

                        if is_number_literal_u16(constant_value)
                        {
                            asm_record.reg_2 = number_literal_to_u16(constant_value);
                        }
                        else if is_register_name(constant_value)
                        {
                            asm_record.reg_2 = register_name_to_u16(constant_value);
                        }
                    }
                    else if dseg_map.contains_key(&param_as_string)
                    {
                        let constant_value = dseg_map.get(&param_as_string).unwrap();
                        asm_record.reg_2 = constant_value.clone();
                    }
                    else
                    {
                        asm_record.target_label = param_as_string;
                    }
                }
            }
        }

        if !self.label.is_empty()
        {
            asm_record.label = self.label.clone();
            self.label = String::default();
        }

        self.records.push(asm_record);

    }
}

impl<'i> ParseTreeVisitorCompat<'i> for NewAssemblerVisitor {
    type Node = assemblerParserContextType;
    type Return = Vec<String>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.return_val
    }

    fn visit_terminal(
        &mut self,
        node: &antlr_rust::tree::TerminalNode<'i, Self::Node>,
    ) -> Self::Return {
        let terminal_text = node.get_text();
        log::trace!("'{}'", terminal_text);

        if terminal_text.eq(",") {
            return vec![];
        }

        return vec![terminal_text];
    }

    fn visit_error_node(
        &mut self,
        _node: &antlr_rust::tree::ErrorNode<'i, Self::Node>,
    ) -> Self::Return {
        Self::Return::default()
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        // https://stackoverflow.com/questions/40792801/what-is-the-best-way-to-concatenate-vectors-in-rust
        let c: Vec<String> = aggregate
            .iter()
            .cloned()
            .chain(next.iter().cloned())
            .collect(); // Cloned
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

impl<'i> assemblerVisitorCompat<'i> for NewAssemblerVisitor {

    fn visit_row(&mut self, ctx: &parser::assemblerparser::RowContext<'i>) -> Self::Return {
        self.descend_ident("visit_row");
        let children_result = self.visit_children(ctx);
        self.ascend_ident();

        log::trace!("cr: {:?}", children_result);

        if children_result.len() == 2usize && children_result[1].eq(":")
        {
            self.label = children_result[0usize].clone();
        }

        children_result
    }

    fn visit_instruction(&mut self, ctx: &InstructionContext<'i>) -> Self::Return {
        self.descend_ident("visit_instruction");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        self.process_instruction(&visit_children_result);

        visit_children_result
    }

    fn visit_mnemonic(&mut self, ctx: &parser::assemblerparser::MnemonicContext<'i>) -> Self::Return {
        self.descend_ident("visit_mnemonic");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        visit_children_result
    }

    fn visit_param(&mut self, ctx: &ParamContext<'i>) -> Self::Return {
        self.descend_ident("visit_param");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        visit_children_result
    }

    fn visit_asm_instrinsic_instruction(&mut self, ctx: &parser::assemblerparser::Asm_instrinsic_instructionContext<'i>) -> Self::Return {
        self.descend_ident("visit_asm_instrinsic_instruction");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        log::trace!("cr: {:?}\n", visit_children_result);

        // look for assembler directives
        // assembler directives are identified via a dot character
        let assembler_directive: bool = ".".eq(&visit_children_result[0]);
        if assembler_directive {
            self.parse_assembler_directive(&visit_children_result);

            self.reset_self();
        }

        visit_children_result
    }

    fn visit_asm_intrinsic_usage(&mut self, ctx: &parser::assemblerparser::Asm_intrinsic_usageContext<'i>) -> Self::Return {
        self.descend_ident("visit_asm_intrinsic_usage");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        log::info!("cr: {:?}\n", visit_children_result);

        self.process_asm_intrinsic_usage(visit_children_result)
    }

    fn visit_expression(&mut self, ctx: &parser::assemblerparser::ExpressionContext<'i>) -> Self::Return {
        self.descend_ident("visit_expression");
        let mut visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        if visit_children_result.len() == 1usize {

            // if the value is a number, return it
            let parse_result = visit_children_result[0].parse::<u16>();
            if parse_result.is_ok() {
                return visit_children_result;
            }
        }

        if visit_children_result.len() == 3usize {

            if "<<".eq(&visit_children_result[1]) {

                // lhs << rhs
                let lhs_as_string = &visit_children_result[0];
                let op_as_string = &visit_children_result[1];
                let rhs_as_string = &visit_children_result[2];

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

                return vec![result.to_string()];
            }
            else if "(".eq(&visit_children_result[0]) && ")".eq(&visit_children_result[2])
            {
                visit_children_result.remove(2);
                visit_children_result.remove(0);

                return visit_children_result;
            }
            else if ".".eq(&visit_children_result[0])
            {
                let sign: &str = visit_children_result[1].as_str();
                let mut offset: i16 = number_literal_to_u16(&visit_children_result[2]) as i16;
                if sign == "-"
                {
                    offset *= -1i16;
                }

                log::trace!("sign: {}, offset: {}", sign, offset);

                return vec![offset.to_string()];
            }
        }

        visit_children_result
    }

    fn visit_label_definition(&mut self, ctx: &parser::assemblerparser::Label_definitionContext<'i>) -> Self::Return {
        self.descend_ident("visit_label_definition");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        log::info!("cr: {:?}\n", visit_children_result);

        self.label = visit_children_result[0].clone();

        self.labels.push(self.label.clone());

        visit_children_result
    }

}



#[cfg(test)]
mod tests {

    // use std::marker::PhantomData;

    // use crate::parser::assemblerparser::{InstructionContextExt, Asm_intrinsic_usageContext, Asm_intrinsic_usageContextExt};

    use super::*;

    #[test]
    fn process_ld_ld_ldd_z_1_test() {

        // Arrange

        let mut new_assembler_visitor: NewAssemblerVisitor = NewAssemblerVisitor::default();
        // let parent_ctx = None;
        // let invoking_state: isize = 0isize;
        // let instr_ctx: InstructionContextExt<'_> = InstructionContextExt{
        //     ph:PhantomData
        // };
        // let ctx: InstructionContext = InstructionContext::new_parser_ctx(parent_ctx, invoking_state, instr_ctx);
        let mut asm_record: AsmRecord = AsmRecord::default();
        let visit_children_result = vec!["ld".to_string(), "r25".to_string(), "Z".to_string()];

        // Act

        new_assembler_visitor.process_ld(/*&ctx,*/ &visit_children_result, &mut asm_record);

        // Assert

        assert_eq!(InstructionType::LD_LDD_Z_1, asm_record.instruction_type);
        assert_eq!(25, asm_record.reg_1);

    }

    #[test]
    fn process_ldi_xh_high_buffer_test() {

        // Arrange

        {
            let mut dseg_map = DSEG_HASHMAP.lock().unwrap();
            dseg_map.insert("BUFFER".to_string(), 0x0160);
        }

        let mut new_assembler_visitor: NewAssemblerVisitor = NewAssemblerVisitor::default();

        // ldi XH, HIGH(BUFFER)     
        //let visit_children_result = vec!["ldi".to_string(), "XH".to_string(), "HIGH".to_string(), "(".to_string(), "BUFFER".to_string(), ")".to_string()];
        
        // ldi r16, LOW(RAMEND)
        let visit_children_result = vec!["ldi".to_string(), "r16".to_string(), "LOW".to_string(), "(".to_string(), "RAMEND".to_string(), ")".to_string()];
        
        // Act

        new_assembler_visitor.process_instruction(&visit_children_result);

        // Assert

        let asm_record: &AsmRecord = &new_assembler_visitor.records[0];

        assert_eq!(InstructionType::LDI, asm_record.instruction_type);
        assert_eq!(16, asm_record.reg_1);

    }

    #[test]
    fn visit_asm_intrinsic_usage_low_ramend_test() {

        // Arrange

        {
            let mut cseg_map = CSEG_HASHMAP.lock().unwrap();
            cseg_map.insert("RAMEND".to_string(), "0x08ff".to_string());
        }

        let mut new_assembler_visitor: NewAssemblerVisitor = NewAssemblerVisitor::default();
        let visit_children_result = vec!["LOW".to_string(), "(".to_string(), "RAMEND".to_string(), ")".to_string()];

        // Act

        let result = new_assembler_visitor.process_asm_intrinsic_usage(visit_children_result);

        // Assert

        assert_eq!("255", result[0]);
    }

    #[test]
    fn visit_asm_intrinsic_usage_high_ramend_test() {

        // Arrange

        {
            let mut cseg_map = CSEG_HASHMAP.lock().unwrap();
            cseg_map.insert("RAMEND".to_string(), "0x08ff".to_string());
        }

        let mut new_assembler_visitor: NewAssemblerVisitor = NewAssemblerVisitor::default();
        let visit_children_result = vec!["HIGH".to_string(), "(".to_string(), "RAMEND".to_string(), ")".to_string()];

        // Act

        let result = new_assembler_visitor.process_asm_intrinsic_usage(visit_children_result);

        // Assert

        assert_eq!("8", result[0]);
    }

}

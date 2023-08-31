use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::parser_rule_context::BaseParserRuleContext;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::InputStream;

use std::fs;
use std::rc::Rc;

use crate::assembler::asm_record::AsmRecord;
use crate::common::common_constants::RAMEND;
use crate::common::number_literal_parser::is_number_literal_i16;
use crate::common::number_literal_parser::is_number_literal_u16;
use crate::common::number_literal_parser::number_literal_to_i16;
use crate::common::number_literal_parser::number_literal_to_u16;
use crate::common::register_parser::is_register_name;
use crate::common::register_parser::register_name_to_u16;
use crate::instructions::instruction_type::InstructionType;
use crate::parser;
use crate::parser::assemblerparser::assemblerParserContextType;
use crate::parser::assemblerparser::Asm_fileContextAll;
use crate::parser::assemblerparser::InstructionContext;
use crate::parser::assemblerparser::ParamContext;
use crate::parser::assemblervisitor::assemblerVisitorCompat;
use crate::HASHMAP;
use crate::HIGH;
use antlr_rust::tree::ParseTree;

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
    fn process_st(&mut self, ctx: &InstructionContext<'i>,
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
    fn process_ld(&mut self, _ctx: &InstructionContext<'i>,
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

    fn parse_assembler_directive(&mut self, assembler_directive: &Vec<String>) {
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
            // if unwrapped_name.ends_with(".inc") {
            //     asm_file_path.push_str("C:/Program Files (x86)/Atmel/Studio/7.0/Packs/atmel/ATmega_DFP/1.7.374/avrasm/inc/");
            // } else {
            //     asm_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/asm/");
            // }
            //asm_file_path.push_str("/Users/bischowg/dev/rust/whatavr/test_resources/sample_files/asm/");
            asm_file_path.push_str("test_resources/sample_files/asm/");
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

            // ignored

        } else {

            panic!();

        }
    }

    fn process_asm_intrinsic_usage(&mut self, visit_children_result: Vec<String>) -> Vec<String>
    {
        log::trace!("cr: {:?}", visit_children_result);

        let joined = visit_children_result.join("");

        if "LOW(RAMEND)" == joined {
            let low_ramend: u16 = crate::LOW!(RAMEND);
            return vec![low_ramend.to_string().clone()];
        }
        if "HIGH(RAMEND)" == joined {
            let high_ramend: u16 = HIGH!(RAMEND);
            return vec![high_ramend.to_string().clone()];
        }

        visit_children_result
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
        // if terminal_text != ":" && terminal_text != "," && terminal_text != "\r\n" {
        //     if self.last_terminal != terminal_text {
        //         self.last_terminal.push_str(terminal_text.as_str());
        //     }
        // }

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

        log::trace!("cr: {:?}", visit_children_result);

        let mut asm_record = AsmRecord::default();

        let mnemonic: &String = &visit_children_result[0];

        if mnemonic.to_uppercase().eq("ST")
        {
            self.process_st(ctx, &visit_children_result, &mut asm_record);
        }
        else if mnemonic.to_uppercase().eq("LD")
        {
            self.process_ld(ctx, &visit_children_result, &mut asm_record);
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

                    // try to resolve constants
                    let map = HASHMAP.lock().unwrap();
                    if map.contains_key(&param_as_string)
                    {
                        let constant_value = map.get(&param_as_string).unwrap();

                        if is_number_literal_u16(constant_value)
                        {
                            asm_record.reg_1 = number_literal_to_u16(constant_value);
                        }
                        else if is_register_name(constant_value)
                        {
                            asm_record.reg_1 = register_name_to_u16(constant_value);
                        }
                    }
                    else
                    {
                        asm_record.target_label = param_as_string;
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
                else
                {
                    let param_as_string = param_2.to_string();

                    // try to resolve constants
                    let map = HASHMAP.lock().unwrap();
                    if map.contains_key(&param_as_string)
                    {
                        let constant_value = map.get(&param_as_string).unwrap();

                        if is_number_literal_u16(constant_value)
                        {
                            asm_record.reg_2 = number_literal_to_u16(constant_value);
                        }
                        else if is_register_name(constant_value)
                        {
                            asm_record.reg_2 = register_name_to_u16(constant_value);
                        }
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

        visit_children_result
    }

    /// is the rule that directly selects the TOKEN of an instruction (ADD; CALL, EOR; LDI; ...)
    fn visit_mnemonic(
        &mut self,
        ctx: &parser::assemblerparser::MnemonicContext<'i>,
    ) -> Self::Return {
        self.descend_ident("visit_mnemonic");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        // self.mnemonic = self.last_terminal.clone();
        // self.last_terminal = String::default();

        visit_children_result
    }

    fn visit_param(&mut self, ctx: &ParamContext<'i>) -> Self::Return {
        self.descend_ident("visit_param");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        //let result_join = children_result.join("");
        //return vec![result_join];

        visit_children_result
    }

    fn visit_asm_instrinsic_instruction(&mut self, ctx: &parser::assemblerparser::Asm_instrinsic_instructionContext<'i>) -> Self::Return {
        self.descend_ident("visit_asm_instrinsic_instruction");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        log::trace!("cr: {:?}", visit_children_result);

        // look for assembler directives
        // assembler directives are identified via a dot character
        let assembler_directive = ".".eq(&visit_children_result[0]);
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

}



#[cfg(test)]
mod tests {

    use std::marker::PhantomData;

    use crate::parser::assemblerparser::{InstructionContextExt, Asm_intrinsic_usageContext, Asm_intrinsic_usageContextExt};

    use super::*;

    #[test]
    fn process_ld_ld_ldd_z_1_test() {

        // Arrange

        let mut new_assembler_visitor: NewAssemblerVisitor = NewAssemblerVisitor::default();
        let parent_ctx = None;
        let invoking_state: isize = 0isize;
        let instr_ctx: InstructionContextExt<'_> = InstructionContextExt{
            ph:PhantomData
        };
        let ctx: InstructionContext = InstructionContext::new_parser_ctx(parent_ctx, invoking_state, instr_ctx);
        let mut asm_record: AsmRecord = AsmRecord::default();
        let visit_children_result = vec!["ld".to_string(), "r25".to_string(), "Z".to_string()];

        // Act

        new_assembler_visitor.process_ld(&ctx, &visit_children_result, &mut asm_record);

        // Assert

        assert_eq!(InstructionType::LD_LDD_Z_1, asm_record.instruction_type);
        assert_eq!(25, asm_record.reg_1);

    }

    #[test]
    fn visit_asm_intrinsic_usage_low_ramend_test() {

        // Arrange

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

        let mut new_assembler_visitor: NewAssemblerVisitor = NewAssemblerVisitor::default();
        let visit_children_result = vec!["HIGH".to_string(), "(".to_string(), "RAMEND".to_string(), ")".to_string()];

        // Act

        let result = new_assembler_visitor.process_asm_intrinsic_usage(visit_children_result);

        // Assert

        assert_eq!("8", result[0]);
    }

}

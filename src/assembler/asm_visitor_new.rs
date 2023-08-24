use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::parser_rule_context::BaseParserRuleContext;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::InputStream;

use std::fs;
use std::rc::Rc;

use crate::assembler::asm_record::AsmRecord;
use crate::assembler::io_destination::IoDestination;
use crate::common::common_constants::RAMEND;
use crate::common::number_literal_parser::number_literal_to_u16;
use crate::common::register_parser::is_register_name;
use crate::common::register_parser::register_name_to_u16;
use crate::instructions::instruction_type::InstructionType;
use crate::parser;
use crate::parser::assemblerparser::assemblerParserContextType;
use crate::parser::assemblerparser::Asm_fileContextAll;
use crate::parser::assemblerparser::InstructionContext;
use crate::parser::assemblerparser::MnemonicContextExt;
use crate::parser::assemblerparser::ParamContext;
use crate::parser::assemblerparser::Preprocessor_directiveContext;
use crate::parser::assemblervisitor::assemblerVisitorCompat;
use crate::HASHMAP;
use crate::HIGH;
use antlr_rust::tree::ParseTree;
use antlr_rust::tree::Tree;

use regex::Regex;

pub struct NewAssemblerVisitor {
    // result
    pub records: Vec<AsmRecord>,
    pub record: AsmRecord,

    // debug
    pub debug_output: bool,
    pub ident: u16,

    // traversal
    pub return_val: Vec<String>,
}

impl NewAssemblerVisitor {
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
        self.record.clear();
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
    fn visit_instruction(&mut self, ctx: &InstructionContext<'i>) -> Self::Return {

        self.descend_ident("visit_instruction");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_ident();

        println!("cr: {:?}", visit_children_result);

        let mut asm_record = AsmRecord::default();

        let mnemonic: &String = &visit_children_result[0];
        asm_record.instruction_type = InstructionType::from_string(mnemonic.as_str());

        if visit_children_result.len() > 1 {
            let param_1: &String = &visit_children_result[1];
            let param_1_as_number: u16;
            if is_register_name(param_1) {
                param_1_as_number = register_name_to_u16(param_1);
                asm_record.reg_1 = param_1_as_number;
            } else {
                param_1_as_number = number_literal_to_u16(&param_1);
                asm_record.reg_1 = param_1_as_number;
            }
        }

        if visit_children_result.len() > 2 {
            let param_2: &String = &visit_children_result[2];
            let param_2_as_number: u16;
            if is_register_name(param_2) {
                param_2_as_number = register_name_to_u16(param_2);
                asm_record.reg_2 = param_2_as_number;
            } else {
                param_2_as_number = number_literal_to_u16(&param_2);
                asm_record.data = param_2_as_number; // test: ldi
            }
        }

        // // create an AsmRecord so it can be added to the application code
        // let asm_record = AsmRecord::new(
        //     String::default(),
        //     InstructionType::from_string(mnemonic.as_str()),
        //     param_1_as_number,
        //     param_2_as_number,
        //     0u16,
        //     String::default(),
        //     IoDestination::UNKNOWN,
        // );

        self.records.push(asm_record);

        // // create an AsmRecord so it can be added to the application code
        // let rec = AsmRecord::new(
        //     self.label.clone(),
        //     InstructionType::from_string(&self.mnemonic.as_str()),
        //     self.record.reg_1,
        //     self.record.reg_2,
        //     self.record.data,
        //     self.target_label.clone(),
        //     self.record.io_dest);

        // self.records.push(rec);

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
}

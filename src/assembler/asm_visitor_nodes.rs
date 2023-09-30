use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::ArenaCommonFactory;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::InputStream;

use std::fs;
use std::rc::Rc;

use crate::assembler::asm_record::AsmRecord;
use crate::assembler::asm_record_type::AsmRecordType;
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
use antlr_rust::tree::ParseTree;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token::GenericToken;
use crate::lazy_static::__Deref;
use antlr_rust::rule_context::CustomRuleContext;
use std::cell::Ref;
use std::borrow::Cow;
use crate::assembler::binary_tree::BinaryTree;

use super::node::Node;
use super::segment_mode::SegmentMode;

pub struct NodeAssemblerVisitor {

    // result
    pub records: Vec<AsmRecord>,
    pub record: AsmRecord,

    // debug
    pub debug_output: bool,
    pub indent: u16,

    // traversal
    pub return_val: Vec<Node<String>>,

    // label
    pub label: String,
    pub labels: Vec<String>,

    // data segment as opposed to the code segment which is filled from the AsmRecords
    pub sram: [u8; RAMEND as usize],

    pub segment_mode: SegmentMode,

    pub cseg_org_pointer: u16,
    pub dseg_org_pointer: u16,

    // the source file that this visitor visits
    pub source_file: String,
}

impl Default for NodeAssemblerVisitor {
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

            source_file: String::default(),
        }
    }
}

impl<'i> NodeAssemblerVisitor {
    pub fn ascend_indent(&mut self) {
        if !self.debug_output {
            return;
        }
        self.indent = self.indent - 1;
    }

    pub fn descend_indent(&mut self, label: &str) {
        if !self.debug_output {
            return;
        }
        self.indent = self.indent + 1;
        for _n in 0..self.indent {
            print!("  ");
        }
        println!("{}", label);
    }
}

impl<'i> ParseTreeVisitorCompat<'i> for NodeAssemblerVisitor {
    type Node = assemblerParserContextType;
    type Return = Vec<Node<String>>;

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

        let result_node = Node::<String>::new(terminal_text);

        return vec![result_node];
    }

    fn visit_error_node(
        &mut self,
        _node: &antlr_rust::tree::ErrorNode<'i, Self::Node>,
    ) -> Self::Return {
        Self::Return::default()
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        // https://stackoverflow.com/questions/40792801/what-is-the-best-way-to-concatenate-vectors-in-rust
        // let c: Vec<String> = aggregate
        //     .iter()
        //     .cloned()
        //     .chain(next.iter().cloned())
        //     .collect(); // Cloned


        // let c: Vec<Node<String>> = aggregate
        //     .iter()
        //     .cloned()
        //     .chain(next.iter().cloned())
        //     .collect(); // Cloned

        // c

        let mut c: Vec<Node<String>> = Vec::new();
        c.append(&mut aggregate.clone());
        c.append(&mut next.clone());

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

impl<'i> assemblerVisitorCompat<'i> for NodeAssemblerVisitor {

    fn visit_row(&mut self, ctx: &parser::assemblerparser::RowContext<'i>) -> Self::Return {
        self.descend_indent("visit_row");
        let children_result = self.visit_children(ctx);
        self.ascend_indent();

        //log::trace!("cr: {:?}", children_result);

        if children_result.len() == 2usize && children_result[1usize].value.eq(":")
        {
            self.label = children_result[0usize].value.clone();
        }

        children_result
    }

    fn visit_instruction(&mut self, ctx: &InstructionContext<'i>) -> Self::Return {
        self.descend_indent("visit_instruction");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        log::info!("{:?}\n", visit_children_result[2]);

        let tok: Ref<'_, GenericToken<Cow<'_, str>>> = ctx.start();

        //self.process_instruction(&visit_children_result, tok.line, tok.column);

        visit_children_result
    }

    fn visit_mnemonic(&mut self, ctx: &parser::assemblerparser::MnemonicContext<'i>) -> Self::Return {
        self.descend_indent("visit_mnemonic");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        visit_children_result
    }

    fn visit_param(&mut self, ctx: &ParamContext<'i>) -> Self::Return {
        self.descend_indent("visit_param");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        visit_children_result
    }

    fn visit_asm_instrinsic_instruction(&mut self, 
        ctx: &parser::assemblerparser::Asm_instrinsic_instructionContext<'i>) -> Self::Return 
    {
        let tok: Ref<'_, GenericToken<Cow<'_, str>>> = ctx.start();
        log::info!("TOK {:?}\n", tok);
        log::info!("TOK.line {:?}\n", tok.line);
        log::info!("TOK.column {:?}\n", tok.column);

        //let b = Box::new(ctx.start());

        //let generic_token: dyn Token = ctx.start() as Token;
        //let generic_token = ctx.start() as ParserRuleContext;
        //generic_token.
        //ctx.

        //let generic_token =  ctx.start().deref() as GenericToken<dyn CustomRuleContext>;
        //let generic_token =  ctx.start().deref();
        //generic_token.

        //ctx.file();
        log::info!("visit_asm_instrinsic_instruction CTX {:?} {:?} {:?}\n", self.source_file, ctx.start(), ctx.start().get_token_index());

        self.descend_indent("visit_asm_instrinsic_instruction");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        //log::trace!("cr: {:?}\n", visit_children_result);

        // look for assembler directives
        // assembler directives are identified via a dot character
        let assembler_directive: bool = ".".eq(&visit_children_result[0].value);
        if assembler_directive {
//            self.parse_assembler_directive(&visit_children_result, tok.line, tok.column);

//            self.reset_self();
        }

        visit_children_result
    }

    /*
    fn visit_asm_intrinsic_usage(&mut self, ctx: &parser::assemblerparser::Asm_intrinsic_usageContext<'i>) -> Self::Return {
        self.descend_indent("visit_asm_intrinsic_usage");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        log::info!("cr: {:?}\n", visit_children_result);

        self.process_asm_intrinsic_usage(visit_children_result)
    } */

    fn visit_expression(&mut self, ctx: &parser::assemblerparser::ExpressionContext<'i>) -> Self::Return 
    {
        self.descend_indent("visit_expression");
        let mut visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        log::info!("{:?}\n", visit_children_result);

        if visit_children_result.len() == 1usize {

            // if the value is a number, return it
            let parse_result = visit_children_result[0].value.parse::<u16>();
            if parse_result.is_ok() {
                return visit_children_result;
            }
        }

        if visit_children_result.len() == 3usize {

            if "(".eq(&visit_children_result[0].value) && ")".eq(&visit_children_result[2].value)
            {
                visit_children_result.remove(2);
                visit_children_result.remove(0);

                return visit_children_result;

                // let op_as_string: &String = &visit_children_result[2].value;

                // let op_node: Node<String> = Node::new(op_as_string.clone());

                // return vec![op_node.left(visit_children_result[1].clone()).right(visit_children_result[3].clone())];
            }
            else if ".".eq(&visit_children_result[0].value)
            {
                let sign: &str = visit_children_result[1].value.as_str();
                let mut offset: i16 = number_literal_to_u16(&visit_children_result[2].value) as i16;
                if sign == "-"
                {
                    offset *= -1i16;
                }

                log::trace!("sign: {}, offset: {}", sign, offset);

                let binary_tree: Node<String> = Node::new(offset.to_string());
                return vec![binary_tree];
            } 
            else 
            {
                //if "<<".eq(&visit_children_result[1].value) {

                //let operator: String = visit_children_result[1].value;

                // lhs << rhs
                let lhs_as_string: &String = &visit_children_result[0].value;
                let op_as_string: &String = &visit_children_result[1].value;
                let rhs_as_string: &String = &visit_children_result[2].value;

                if !self.debug_output {
                    println!("lhs: {} op: {} rhs: {}", lhs_as_string, op_as_string, rhs_as_string);
                }

                // let mut lhs: i16 = 0i16;
                // let lhs_parse_result = lhs_as_string.parse::<i16>();
                // if lhs_parse_result.is_ok() {
                //     lhs = lhs_parse_result.unwrap();
                // }

                // let mut rhs: i16 = 0i16;
                // let rhs_parse_result = rhs_as_string.parse::<i16>();
                // if rhs_parse_result.is_ok() {
                //     rhs = rhs_parse_result.unwrap();
                // }

                // let result: i16 = lhs << rhs;

                //let lhs_node: Node<String> = Node::new(lhs_as_string.clone());
                let op_node: Node<String> = Node::new(op_as_string.clone());
                //let rhs_node: Node<String> = Node::new(rhs_as_string.clone());

                //return vec![op_node.left(lhs_node).right(rhs_node)];

                return vec![op_node.left(visit_children_result[0].clone()).right(visit_children_result[2].clone())];
            }
        }

        //let binary_tree: BinaryTree<i32> = BinaryTree::new(1);

        //self.record

        visit_children_result
        //binary_tree
    }

    /* 
    fn visit_expression(&mut self, ctx: &parser::assemblerparser::ExpressionContext<'i>) -> Self::Return {
        self.descend_indent("visit_expression");
        let mut visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

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
    }*/

    fn visit_label_definition(&mut self, ctx: &parser::assemblerparser::Label_definitionContext<'i>) -> Self::Return {
        self.descend_indent("visit_label_definition");
        let visit_children_result = self.visit_children(ctx);
        self.ascend_indent();

        //log::info!("cr: {:?}\n", visit_children_result);

        self.label = visit_children_result[0usize].value.clone();

        self.labels.push(self.label.clone());

        visit_children_result
    }

}
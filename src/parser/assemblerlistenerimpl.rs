use antlr_rust::tree::ParseTreeListener;

use super::{assemblerparser::{assemblerParserContextType, CsvFileContext, HdrContext, RowContext, FieldContext}, assemblerlistener::assemblerListener};

pub struct assemblerListenerImpl {

}

impl<'input> assemblerListener<'input> for assemblerListenerImpl {
     /**
 * Enter a parse tree produced by {@link assemblerParser#csvFile}.
 * @param ctx the parse tree
 */
fn enter_csvFile(&mut self, _ctx: &CsvFileContext<'input>) { println!("enter_csvFile()"); }
/**
 * Exit a parse tree produced by {@link assemblerParser#csvFile}.
 * @param ctx the parse tree
 */
fn exit_csvFile(&mut self, _ctx: &CsvFileContext<'input>) {println!("exit_csvFile()"); }
/**
 * Enter a parse tree produced by {@link assemblerParser#hdr}.
 * @param ctx the parse tree
 */
fn enter_hdr(&mut self, _ctx: &HdrContext<'input>) { println!("enter_hdr()");}
/**
 * Exit a parse tree produced by {@link assemblerParser#hdr}.
 * @param ctx the parse tree
 */
fn exit_hdr(&mut self, _ctx: &HdrContext<'input>) { println!("exit_hdr()");}
/**
 * Enter a parse tree produced by {@link assemblerParser#row}.
 * @param ctx the parse tree
 */
fn enter_row(&mut self, _ctx: &RowContext<'input>) { println!("enter_row()");}
/**
 * Exit a parse tree produced by {@link assemblerParser#row}.
 * @param ctx the parse tree
 */
fn exit_row(&mut self, _ctx: &RowContext<'input>) {println!("exit_row()"); }
/**
 * Enter a parse tree produced by {@link assemblerParser#field}.
 * @param ctx the parse tree
 */
fn enter_field(&mut self, _ctx: &FieldContext<'input>) { println!("enter_field()");}
/**
 * Exit a parse tree produced by {@link assemblerParser#field}.
 * @param ctx the parse tree
 */
fn exit_field(&mut self, _ctx: &FieldContext<'input>) {println!("exit_field()"); }
}

impl<'input> ParseTreeListener<'input, assemblerParserContextType> for assemblerListenerImpl {

    // fn enter_csvFile(&mut self, _ctx: &super::assemblerparser::CsvFileContext<'input>) { }

    // fn exit_csvFile(&mut self, _ctx: &super::assemblerparser::CsvFileContext<'input>) { }

    // fn enter_hdr(&mut self, _ctx: &super::assemblerparser::HdrContext<'input>) { }

    // fn exit_hdr(&mut self, _ctx: &super::assemblerparser::HdrContext<'input>) { }

    // fn enter_row(&mut self, _ctx: &super::assemblerparser::RowContext<'input>) { }

    // fn exit_row(&mut self, _ctx: &super::assemblerparser::RowContext<'input>) { }

    // fn enter_field(&mut self, _ctx: &super::assemblerparser::FieldContext<'input>) { }

    // fn exit_field(&mut self, _ctx: &super::assemblerparser::FieldContext<'input>) { }

    fn visit_terminal(&mut self, _node: &antlr_rust::tree::TerminalNode<'input, assemblerParserContextType>) {println!("visit_terminal()");}

    fn visit_error_node(&mut self, _node: &antlr_rust::tree::ErrorNode<'input, assemblerParserContextType>) {println!("visit_error_node()");}

    fn enter_every_rule(&mut self, _ctx: &<assemblerParserContextType as antlr_rust::parser::ParserNodeType>::Type) {println!("enter_every_rule()");}

    fn exit_every_rule(&mut self, _ctx: &<assemblerParserContextType as antlr_rust::parser::ParserNodeType>::Type) {println!("exit_every_rule()");}

    // fn enter_csvFile(&mut self, _ctx: &super::assemblerparser::CsvFileContext<'input>) { }

    // fn exit_csvFile(&mut self, _ctx: &super::assemblerparser::CsvFileContext<'input>) { }

    // fn enter_hdr(&mut self, _ctx: &super::assemblerparser::HdrContext<'input>) { }

    // fn exit_hdr(&mut self, _ctx: &super::assemblerparser::HdrContext<'input>) { }

    // fn enter_row(&mut self, _ctx: &super::assemblerparser::RowContext<'input>) { }

    // fn exit_row(&mut self, _ctx: &super::assemblerparser::RowContext<'input>) { }

    // fn enter_field(&mut self, _ctx: &super::assemblerparser::FieldContext<'input>) { }

    // fn exit_field(&mut self, _ctx: &super::assemblerparser::FieldContext<'input>) { }

}
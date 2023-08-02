use antlr_rust::tree::ParseTreeListener;

use super::{
    assemblerlistener::assemblerListener,
    assemblerparser::{assemblerParserContextType, RowContext, Asm_fileContext},
};
use crate::parser::assemblerparser::InstructionContext;
use crate::parser::assemblerparser::Label_definitionContext;
use crate::parser::assemblerparser::ParameterContext;
//use crate::parser::assemblerparser::Macro_usageContext;
//use crate::parser::assemblerparser::Macro_usageContextAttrs;

pub struct assemblerListenerImpl {}

impl<'input> assemblerListener<'input> for assemblerListenerImpl {
    /**
     * Enter a parse tree produced by {@link assemblerParser#asmFile}.
     * @param ctx the parse tree
     */
    fn enter_asm_file(&mut self, _ctx: &Asm_fileContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link assemblerParser#asmFile}.
     * @param ctx the parse tree
     */
    fn exit_asm_file(&mut self, _ctx: &Asm_fileContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link assemblerParser#row}.
     * @param ctx the parse tree
     */
    fn enter_row(&mut self, _ctx: &RowContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link assemblerParser#row}.
     * @param ctx the parse tree
     */
    fn exit_row(&mut self, _ctx: &RowContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link assemblerParser#label_definition}.
     * @param ctx the parse tree
     */
    fn enter_label_definition(&mut self, _ctx: &Label_definitionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link assemblerParser#label_definition}.
     * @param ctx the parse tree
     */
    fn exit_label_definition(&mut self, _ctx: &Label_definitionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link assemblerParser#parameter}.
     * @param ctx the parse tree
     */
    fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link assemblerParser#parameter}.
     * @param ctx the parse tree
     */
    fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link assemblerParser#instruction}.
     * @param ctx the parse tree
     */
    fn enter_instruction(&mut self, _ctx: &InstructionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link assemblerParser#instruction}.
     * @param ctx the parse tree
     */
    fn exit_instruction(&mut self, _ctx: &InstructionContext<'input>) {}

    // /**
    //  * Enter a parse tree produced by {@link assemblerParser#macro_usage}.
    //  * @param ctx the parse tree
    //  */
    // fn enter_macro_usage(&mut self, _ctx: &Macro_usageContext<'input>) { 
    //     println!("enter_macro_usage()");
    //     println!("Macro_usageContext ident 1: {:?} ident 2: {:?}", _ctx.IDENTIFIER(0), _ctx.IDENTIFIER(1));
    // }


}

impl<'input> ParseTreeListener<'input, assemblerParserContextType> for assemblerListenerImpl {
    fn visit_terminal(
        &mut self,
        _node: &antlr_rust::tree::TerminalNode<'input, assemblerParserContextType>,
    ) {
        //println!("visit_terminal()");
    }

    fn visit_error_node(
        &mut self,
        _node: &antlr_rust::tree::ErrorNode<'input, assemblerParserContextType>,
    ) {
        //println!("visit_error_node()");
    }

    fn enter_every_rule(
        &mut self,
        _ctx: &<assemblerParserContextType as antlr_rust::parser::ParserNodeType>::Type,
    ) {
        //println!("enter_every_rule()");
    }

    fn exit_every_rule(
        &mut self,
        _ctx: &<assemblerParserContextType as antlr_rust::parser::ParserNodeType>::Type,
    ) {
        //println!("exit_every_rule()");
    }
}

#![allow(nonstandard_style)]
// Generated from ../../src/parser/assembler.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::assemblerparser::*;

pub trait assemblerListener<'input> : ParseTreeListener<'input,assemblerParserContextType>{
/**
 * Enter a parse tree produced by {@link assemblerParser#asmFile}.
 * @param ctx the parse tree
 */
fn enter_asmFile(&mut self, _ctx: &AsmFileContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#asmFile}.
 * @param ctx the parse tree
 */
fn exit_asmFile(&mut self, _ctx: &AsmFileContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#row}.
 * @param ctx the parse tree
 */
fn enter_row(&mut self, _ctx: &RowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#row}.
 * @param ctx the parse tree
 */
fn exit_row(&mut self, _ctx: &RowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#label_definition}.
 * @param ctx the parse tree
 */
fn enter_label_definition(&mut self, _ctx: &Label_definitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#label_definition}.
 * @param ctx the parse tree
 */
fn exit_label_definition(&mut self, _ctx: &Label_definitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#parameter}.
 * @param ctx the parse tree
 */
fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#parameter}.
 * @param ctx the parse tree
 */
fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#macro_usage}.
 * @param ctx the parse tree
 */
fn enter_macro_usage(&mut self, _ctx: &Macro_usageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#macro_usage}.
 * @param ctx the parse tree
 */
fn exit_macro_usage(&mut self, _ctx: &Macro_usageContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#instruction}.
 * @param ctx the parse tree
 */
fn enter_instruction(&mut self, _ctx: &InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#instruction}.
 * @param ctx the parse tree
 */
fn exit_instruction(&mut self, _ctx: &InstructionContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : assemblerListener<'input> }



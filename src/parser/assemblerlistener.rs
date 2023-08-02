#![allow(nonstandard_style)]
// Generated from ../../src/parser/assembler.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::assemblerparser::*;

pub trait assemblerListener<'input> : ParseTreeListener<'input,assemblerParserContextType>{
/**
 * Enter a parse tree produced by {@link assemblerParser#asm_file}.
 * @param ctx the parse tree
 */
fn enter_asm_file(&mut self, _ctx: &Asm_fileContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#asm_file}.
 * @param ctx the parse tree
 */
fn exit_asm_file(&mut self, _ctx: &Asm_fileContext<'input>) { }
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
 * Enter a parse tree produced by {@link assemblerParser#instruction}.
 * @param ctx the parse tree
 */
fn enter_instruction(&mut self, _ctx: &InstructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#instruction}.
 * @param ctx the parse tree
 */
fn exit_instruction(&mut self, _ctx: &InstructionContext<'input>) { }
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
 * Enter a parse tree produced by {@link assemblerParser#macro_placeholder}.
 * @param ctx the parse tree
 */
fn enter_macro_placeholder(&mut self, _ctx: &Macro_placeholderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#macro_placeholder}.
 * @param ctx the parse tree
 */
fn exit_macro_placeholder(&mut self, _ctx: &Macro_placeholderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#asm_instrinsic_instruction}.
 * @param ctx the parse tree
 */
fn enter_asm_instrinsic_instruction(&mut self, _ctx: &Asm_instrinsic_instructionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#asm_instrinsic_instruction}.
 * @param ctx the parse tree
 */
fn exit_asm_instrinsic_instruction(&mut self, _ctx: &Asm_instrinsic_instructionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#asm_intrinsic_usage}.
 * @param ctx the parse tree
 */
fn enter_asm_intrinsic_usage(&mut self, _ctx: &Asm_intrinsic_usageContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#asm_intrinsic_usage}.
 * @param ctx the parse tree
 */
fn exit_asm_intrinsic_usage(&mut self, _ctx: &Asm_intrinsic_usageContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic}.
 * @param ctx the parse tree
 */
fn enter_mnemonic(&mut self, _ctx: &MnemonicContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic}.
 * @param ctx the parse tree
 */
fn exit_mnemonic(&mut self, _ctx: &MnemonicContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : assemblerListener<'input> }



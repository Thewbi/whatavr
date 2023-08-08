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
 * Enter a parse tree produced by {@link assemblerParser#param}.
 * @param ctx the parse tree
 */
fn enter_param(&mut self, _ctx: &ParamContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#param}.
 * @param ctx the parse tree
 */
fn exit_param(&mut self, _ctx: &ParamContext<'input>) { }
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
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_a}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_a(&mut self, _ctx: &Mnemonic_aContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_a}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_a(&mut self, _ctx: &Mnemonic_aContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_b}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_b(&mut self, _ctx: &Mnemonic_bContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_b}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_b(&mut self, _ctx: &Mnemonic_bContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_c}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_c(&mut self, _ctx: &Mnemonic_cContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_c}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_c(&mut self, _ctx: &Mnemonic_cContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_d}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_d(&mut self, _ctx: &Mnemonic_dContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_d}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_d(&mut self, _ctx: &Mnemonic_dContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_e}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_e(&mut self, _ctx: &Mnemonic_eContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_e}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_e(&mut self, _ctx: &Mnemonic_eContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_f}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_f(&mut self, _ctx: &Mnemonic_fContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_f}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_f(&mut self, _ctx: &Mnemonic_fContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_i}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_i(&mut self, _ctx: &Mnemonic_iContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_i}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_i(&mut self, _ctx: &Mnemonic_iContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_j}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_j(&mut self, _ctx: &Mnemonic_jContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_j}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_j(&mut self, _ctx: &Mnemonic_jContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_l}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_l(&mut self, _ctx: &Mnemonic_lContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_l}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_l(&mut self, _ctx: &Mnemonic_lContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_m}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_m(&mut self, _ctx: &Mnemonic_mContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_m}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_m(&mut self, _ctx: &Mnemonic_mContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_n}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_n(&mut self, _ctx: &Mnemonic_nContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_n}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_n(&mut self, _ctx: &Mnemonic_nContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_o}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_o(&mut self, _ctx: &Mnemonic_oContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_o}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_o(&mut self, _ctx: &Mnemonic_oContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_p}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_p(&mut self, _ctx: &Mnemonic_pContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_p}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_p(&mut self, _ctx: &Mnemonic_pContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_r}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_r(&mut self, _ctx: &Mnemonic_rContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_r}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_r(&mut self, _ctx: &Mnemonic_rContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_s}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_s(&mut self, _ctx: &Mnemonic_sContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_s}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_s(&mut self, _ctx: &Mnemonic_sContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_t}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_t(&mut self, _ctx: &Mnemonic_tContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_t}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_t(&mut self, _ctx: &Mnemonic_tContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_w}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_w(&mut self, _ctx: &Mnemonic_wContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_w}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_w(&mut self, _ctx: &Mnemonic_wContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#mnemonic_x}.
 * @param ctx the parse tree
 */
fn enter_mnemonic_x(&mut self, _ctx: &Mnemonic_xContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#mnemonic_x}.
 * @param ctx the parse tree
 */
fn exit_mnemonic_x(&mut self, _ctx: &Mnemonic_xContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : assemblerListener<'input> }



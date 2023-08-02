#![allow(nonstandard_style)]
// Generated from ../../src/parser/assembler.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::assemblerparser::*;

pub trait assemblerListener<'input> : ParseTreeListener<'input,assemblerParserContextType>{
/**
 * Enter a parse tree produced by {@link assemblerParser#csvFile}.
 * @param ctx the parse tree
 */
fn enter_csvFile(&mut self, _ctx: &CsvFileContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#csvFile}.
 * @param ctx the parse tree
 */
fn exit_csvFile(&mut self, _ctx: &CsvFileContext<'input>) { }
/**
 * Enter a parse tree produced by {@link assemblerParser#hdr}.
 * @param ctx the parse tree
 */
fn enter_hdr(&mut self, _ctx: &HdrContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#hdr}.
 * @param ctx the parse tree
 */
fn exit_hdr(&mut self, _ctx: &HdrContext<'input>) { }
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
 * Enter a parse tree produced by {@link assemblerParser#field}.
 * @param ctx the parse tree
 */
fn enter_field(&mut self, _ctx: &FieldContext<'input>) { }
/**
 * Exit a parse tree produced by {@link assemblerParser#field}.
 * @param ctx the parse tree
 */
fn exit_field(&mut self, _ctx: &FieldContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : assemblerListener<'input> }



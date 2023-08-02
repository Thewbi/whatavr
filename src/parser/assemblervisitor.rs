#![allow(nonstandard_style)]
// Generated from ../../src/parser/assembler.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::assemblerparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link assemblerParser}.
 */
pub trait assemblerVisitor<'input>: ParseTreeVisitor<'input,assemblerParserContextType>{
	/**
	 * Visit a parse tree produced by {@link assemblerParser#csvFile}.
	 * @param ctx the parse tree
	 */
	fn visit_csvFile(&mut self, ctx: &CsvFileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#hdr}.
	 * @param ctx the parse tree
	 */
	fn visit_hdr(&mut self, ctx: &HdrContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#row}.
	 * @param ctx the parse tree
	 */
	fn visit_row(&mut self, ctx: &RowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#field}.
	 * @param ctx the parse tree
	 */
	fn visit_field(&mut self, ctx: &FieldContext<'input>) { self.visit_children(ctx) }

}

pub trait assemblerVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= assemblerParserContextType>{
	/**
	 * Visit a parse tree produced by {@link assemblerParser#csvFile}.
	 * @param ctx the parse tree
	 */
		fn visit_csvFile(&mut self, ctx: &CsvFileContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#hdr}.
	 * @param ctx the parse tree
	 */
		fn visit_hdr(&mut self, ctx: &HdrContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#row}.
	 * @param ctx the parse tree
	 */
		fn visit_row(&mut self, ctx: &RowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#field}.
	 * @param ctx the parse tree
	 */
		fn visit_field(&mut self, ctx: &FieldContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> assemblerVisitor<'input> for T
where
	T: assemblerVisitorCompat<'input>
{
	fn visit_csvFile(&mut self, ctx: &CsvFileContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_csvFile(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hdr(&mut self, ctx: &HdrContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_hdr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_row(&mut self, ctx: &RowContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_row(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_field(&mut self, ctx: &FieldContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_field(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}
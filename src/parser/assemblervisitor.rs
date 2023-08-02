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
	 * Visit a parse tree produced by {@link assemblerParser#asmFile}.
	 * @param ctx the parse tree
	 */
	fn visit_asmFile(&mut self, ctx: &AsmFileContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#row}.
	 * @param ctx the parse tree
	 */
	fn visit_row(&mut self, ctx: &RowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#label_definition}.
	 * @param ctx the parse tree
	 */
	fn visit_label_definition(&mut self, ctx: &Label_definitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#parameter}.
	 * @param ctx the parse tree
	 */
	fn visit_parameter(&mut self, ctx: &ParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#macro_usage}.
	 * @param ctx the parse tree
	 */
	fn visit_macro_usage(&mut self, ctx: &Macro_usageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#instruction}.
	 * @param ctx the parse tree
	 */
	fn visit_instruction(&mut self, ctx: &InstructionContext<'input>) { self.visit_children(ctx) }

}

pub trait assemblerVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= assemblerParserContextType>{
	/**
	 * Visit a parse tree produced by {@link assemblerParser#asmFile}.
	 * @param ctx the parse tree
	 */
		fn visit_asmFile(&mut self, ctx: &AsmFileContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link assemblerParser#label_definition}.
	 * @param ctx the parse tree
	 */
		fn visit_label_definition(&mut self, ctx: &Label_definitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#parameter}.
	 * @param ctx the parse tree
	 */
		fn visit_parameter(&mut self, ctx: &ParameterContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#macro_usage}.
	 * @param ctx the parse tree
	 */
		fn visit_macro_usage(&mut self, ctx: &Macro_usageContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#instruction}.
	 * @param ctx the parse tree
	 */
		fn visit_instruction(&mut self, ctx: &InstructionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> assemblerVisitor<'input> for T
where
	T: assemblerVisitorCompat<'input>
{
	fn visit_asmFile(&mut self, ctx: &AsmFileContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_asmFile(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_row(&mut self, ctx: &RowContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_row(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_label_definition(&mut self, ctx: &Label_definitionContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_label_definition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parameter(&mut self, ctx: &ParameterContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_parameter(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_macro_usage(&mut self, ctx: &Macro_usageContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_macro_usage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_instruction(&mut self, ctx: &InstructionContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_instruction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}
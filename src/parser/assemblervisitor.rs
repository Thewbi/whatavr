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
	 * Visit a parse tree produced by {@link assemblerParser#asm_file}.
	 * @param ctx the parse tree
	 */
	fn visit_asm_file(&mut self, ctx: &Asm_fileContext<'input>) { self.visit_children(ctx) }

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
	 * Visit a parse tree produced by {@link assemblerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#asm_intrinsic_usage}.
	 * @param ctx the parse tree
	 */
	fn visit_asm_intrinsic_usage(&mut self, ctx: &Asm_intrinsic_usageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#instruction}.
	 * @param ctx the parse tree
	 */
	fn visit_instruction(&mut self, ctx: &InstructionContext<'input>) { self.visit_children(ctx) }

}

pub trait assemblerVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= assemblerParserContextType>{
	/**
	 * Visit a parse tree produced by {@link assemblerParser#asm_file}.
	 * @param ctx the parse tree
	 */
		fn visit_asm_file(&mut self, ctx: &Asm_fileContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link assemblerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#asm_intrinsic_usage}.
	 * @param ctx the parse tree
	 */
		fn visit_asm_intrinsic_usage(&mut self, ctx: &Asm_intrinsic_usageContext<'input>) -> Self::Return {
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
	fn visit_asm_file(&mut self, ctx: &Asm_fileContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_asm_file(self, ctx);
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

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_asm_intrinsic_usage(&mut self, ctx: &Asm_intrinsic_usageContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_asm_intrinsic_usage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_instruction(&mut self, ctx: &InstructionContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_instruction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}
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
	 * Visit a parse tree produced by {@link assemblerParser#instruction}.
	 * @param ctx the parse tree
	 */
	fn visit_instruction(&mut self, ctx: &InstructionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#param}.
	 * @param ctx the parse tree
	 */
	fn visit_param(&mut self, ctx: &ParamContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#macro_usage}.
	 * @param ctx the parse tree
	 */
	fn visit_macro_usage(&mut self, ctx: &Macro_usageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#label_definition}.
	 * @param ctx the parse tree
	 */
	fn visit_label_definition(&mut self, ctx: &Label_definitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#macro_placeholder}.
	 * @param ctx the parse tree
	 */
	fn visit_macro_placeholder(&mut self, ctx: &Macro_placeholderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#asm_instrinsic_instruction}.
	 * @param ctx the parse tree
	 */
	fn visit_asm_instrinsic_instruction(&mut self, ctx: &Asm_instrinsic_instructionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#asm_intrinsic_usage}.
	 * @param ctx the parse tree
	 */
	fn visit_asm_intrinsic_usage(&mut self, ctx: &Asm_intrinsic_usageContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#preprocessor_directive}.
	 * @param ctx the parse tree
	 */
	fn visit_preprocessor_directive(&mut self, ctx: &Preprocessor_directiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic(&mut self, ctx: &MnemonicContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_a}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_a(&mut self, ctx: &Mnemonic_aContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_b}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_b(&mut self, ctx: &Mnemonic_bContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_c}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_c(&mut self, ctx: &Mnemonic_cContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_d}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_d(&mut self, ctx: &Mnemonic_dContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_e}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_e(&mut self, ctx: &Mnemonic_eContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_f}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_f(&mut self, ctx: &Mnemonic_fContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_i}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_i(&mut self, ctx: &Mnemonic_iContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_j}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_j(&mut self, ctx: &Mnemonic_jContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_l}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_l(&mut self, ctx: &Mnemonic_lContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_m}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_m(&mut self, ctx: &Mnemonic_mContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_n}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_n(&mut self, ctx: &Mnemonic_nContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_o}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_o(&mut self, ctx: &Mnemonic_oContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_p}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_p(&mut self, ctx: &Mnemonic_pContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_r}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_r(&mut self, ctx: &Mnemonic_rContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_s}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_s(&mut self, ctx: &Mnemonic_sContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_t}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_t(&mut self, ctx: &Mnemonic_tContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_w}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_w(&mut self, ctx: &Mnemonic_wContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_x}.
	 * @param ctx the parse tree
	 */
	fn visit_mnemonic_x(&mut self, ctx: &Mnemonic_xContext<'input>) { self.visit_children(ctx) }

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
	 * Visit a parse tree produced by {@link assemblerParser#instruction}.
	 * @param ctx the parse tree
	 */
		fn visit_instruction(&mut self, ctx: &InstructionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#param}.
	 * @param ctx the parse tree
	 */
		fn visit_param(&mut self, ctx: &ParamContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link assemblerParser#label_definition}.
	 * @param ctx the parse tree
	 */
		fn visit_label_definition(&mut self, ctx: &Label_definitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#macro_placeholder}.
	 * @param ctx the parse tree
	 */
		fn visit_macro_placeholder(&mut self, ctx: &Macro_placeholderContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link assemblerParser#asm_instrinsic_instruction}.
	 * @param ctx the parse tree
	 */
		fn visit_asm_instrinsic_instruction(&mut self, ctx: &Asm_instrinsic_instructionContext<'input>) -> Self::Return {
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
	 * Visit a parse tree produced by {@link assemblerParser#preprocessor_directive}.
	 * @param ctx the parse tree
	 */
		fn visit_preprocessor_directive(&mut self, ctx: &Preprocessor_directiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic(&mut self, ctx: &MnemonicContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_a}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_a(&mut self, ctx: &Mnemonic_aContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_b}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_b(&mut self, ctx: &Mnemonic_bContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_c}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_c(&mut self, ctx: &Mnemonic_cContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_d}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_d(&mut self, ctx: &Mnemonic_dContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_e}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_e(&mut self, ctx: &Mnemonic_eContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_f}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_f(&mut self, ctx: &Mnemonic_fContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_i}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_i(&mut self, ctx: &Mnemonic_iContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_j}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_j(&mut self, ctx: &Mnemonic_jContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_l}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_l(&mut self, ctx: &Mnemonic_lContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_m}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_m(&mut self, ctx: &Mnemonic_mContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_n}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_n(&mut self, ctx: &Mnemonic_nContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_o}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_o(&mut self, ctx: &Mnemonic_oContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_p}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_p(&mut self, ctx: &Mnemonic_pContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_r}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_r(&mut self, ctx: &Mnemonic_rContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_s}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_s(&mut self, ctx: &Mnemonic_sContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_t}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_t(&mut self, ctx: &Mnemonic_tContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_w}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_w(&mut self, ctx: &Mnemonic_wContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link assemblerParser#mnemonic_x}.
	 * @param ctx the parse tree
	 */
		fn visit_mnemonic_x(&mut self, ctx: &Mnemonic_xContext<'input>) -> Self::Return {
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

	fn visit_instruction(&mut self, ctx: &InstructionContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_instruction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_param(&mut self, ctx: &ParamContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_param(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_macro_usage(&mut self, ctx: &Macro_usageContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_macro_usage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_label_definition(&mut self, ctx: &Label_definitionContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_label_definition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_macro_placeholder(&mut self, ctx: &Macro_placeholderContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_macro_placeholder(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_asm_instrinsic_instruction(&mut self, ctx: &Asm_instrinsic_instructionContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_asm_instrinsic_instruction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_asm_intrinsic_usage(&mut self, ctx: &Asm_intrinsic_usageContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_asm_intrinsic_usage(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_preprocessor_directive(&mut self, ctx: &Preprocessor_directiveContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_preprocessor_directive(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic(&mut self, ctx: &MnemonicContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_a(&mut self, ctx: &Mnemonic_aContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_a(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_b(&mut self, ctx: &Mnemonic_bContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_b(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_c(&mut self, ctx: &Mnemonic_cContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_c(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_d(&mut self, ctx: &Mnemonic_dContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_d(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_e(&mut self, ctx: &Mnemonic_eContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_e(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_f(&mut self, ctx: &Mnemonic_fContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_f(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_i(&mut self, ctx: &Mnemonic_iContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_i(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_j(&mut self, ctx: &Mnemonic_jContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_j(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_l(&mut self, ctx: &Mnemonic_lContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_l(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_m(&mut self, ctx: &Mnemonic_mContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_m(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_n(&mut self, ctx: &Mnemonic_nContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_n(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_o(&mut self, ctx: &Mnemonic_oContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_o(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_p(&mut self, ctx: &Mnemonic_pContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_p(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_r(&mut self, ctx: &Mnemonic_rContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_r(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_s(&mut self, ctx: &Mnemonic_sContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_s(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_t(&mut self, ctx: &Mnemonic_tContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_t(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_w(&mut self, ctx: &Mnemonic_wContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_w(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mnemonic_x(&mut self, ctx: &Mnemonic_xContext<'input>){
		let result = <Self as assemblerVisitorCompat>::visit_mnemonic_x(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}
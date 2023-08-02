// Generated from ../../src/parser/assembler.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::assemblerlistener::*;
use super::assemblervisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const LDI:isize=1; 
		pub const LINE_COMMENT:isize=2; 
		pub const STRING:isize=3; 
		pub const ASTERISK:isize=4; 
		pub const CLOSEING_BRACKET:isize=5; 
		pub const COLON:isize=6; 
		pub const COMMA:isize=7; 
		pub const CSEG:isize=8; 
		pub const DEF:isize=9; 
		pub const DOT:isize=10; 
		pub const ELSE:isize=11; 
		pub const END_MACRO:isize=12; 
		pub const ENDIF:isize=13; 
		pub const EQUALS:isize=14; 
		pub const EQU:isize=15; 
		pub const ERROR:isize=16; 
		pub const IF:isize=17; 
		pub const INCLUDE:isize=18; 
		pub const MACRO:isize=19; 
		pub const MINUS:isize=20; 
		pub const OPENING_BRACKET:isize=21; 
		pub const ORG:isize=22; 
		pub const PLUS:isize=23; 
		pub const SLASH:isize=24; 
		pub const NEWLINE:isize=25; 
		pub const WS:isize=26; 
		pub const NUMBER:isize=27; 
		pub const HEX_NUMBER:isize=28; 
		pub const IDENTIFIER:isize=29;
	pub const RULE_asm_file:usize = 0; 
	pub const RULE_row:usize = 1; 
	pub const RULE_label_definition:usize = 2; 
	pub const RULE_parameter:usize = 3; 
	pub const RULE_expression:usize = 4; 
	pub const RULE_asm_instrinsic_instruction:usize = 5; 
	pub const RULE_asm_intrinsic_usage:usize = 6; 
	pub const RULE_instruction:usize = 7;
	pub const ruleNames: [&'static str; 8] =  [
		"asm_file", "row", "label_definition", "parameter", "expression", "asm_instrinsic_instruction", 
		"asm_intrinsic_usage", "instruction"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;25] = [
		None, None, None, None, Some("'*'"), Some("')'"), Some("':'"), Some("','"), 
		Some("'cseg'"), Some("'def'"), Some("'.'"), Some("'else'"), Some("'endmacro'"), 
		Some("'endif'"), Some("'='"), Some("'equ'"), Some("'error'"), Some("'if'"), 
		Some("'include'"), Some("'macro'"), Some("'-'"), Some("'('"), Some("'org'"), 
		Some("'+'"), Some("'/'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;30]  = [
		None, Some("LDI"), Some("LINE_COMMENT"), Some("STRING"), Some("ASTERISK"), 
		Some("CLOSEING_BRACKET"), Some("COLON"), Some("COMMA"), Some("CSEG"), 
		Some("DEF"), Some("DOT"), Some("ELSE"), Some("END_MACRO"), Some("ENDIF"), 
		Some("EQUALS"), Some("EQU"), Some("ERROR"), Some("IF"), Some("INCLUDE"), 
		Some("MACRO"), Some("MINUS"), Some("OPENING_BRACKET"), Some("ORG"), Some("PLUS"), 
		Some("SLASH"), Some("NEWLINE"), Some("WS"), Some("NUMBER"), Some("HEX_NUMBER"), 
		Some("IDENTIFIER")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,assemblerParserExt<'input>, I, assemblerParserContextType , dyn assemblerListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;

pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;

pub type assemblerTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, assemblerParserContextType , dyn assemblerListener<'input> + 'a>;

/// Parser for assembler grammar
pub struct assemblerParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				assemblerParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> assemblerParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> assemblerParser<'input, I, DefaultErrorStrategy<'input,assemblerParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for assemblerParser
pub trait assemblerParserContext<'input>:
	for<'x> Listenable<dyn assemblerListener<'input> + 'x > + 
	for<'x> Visitable<dyn assemblerVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=assemblerParserContextType>
{}

antlr_rust::coerce_from!{ 'input : assemblerParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn assemblerParserContext<'input> + 'input
where
    T: assemblerVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn assemblerVisitor<'input> + 'x))
    }
}

impl<'input> assemblerParserContext<'input> for TerminalNode<'input,assemblerParserContextType> {}
impl<'input> assemblerParserContext<'input> for ErrorNode<'input,assemblerParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn assemblerParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn assemblerListener<'input> + 'input }

pub struct assemblerParserContextType;
antlr_rust::tid!{assemblerParserContextType}

impl<'input> ParserNodeType<'input> for assemblerParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn assemblerParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct assemblerParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> assemblerParserExt<'input>{
}
antlr_rust::tid! { assemblerParserExt<'a> }

impl<'input> TokenAware<'input> for assemblerParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for assemblerParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for assemblerParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "assembler.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- asm_file ----------------
pub type Asm_fileContextAll<'input> = Asm_fileContext<'input>;


pub type Asm_fileContext<'input> = BaseParserRuleContext<'input,Asm_fileContextExt<'input>>;

#[derive(Clone)]
pub struct Asm_fileContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for Asm_fileContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for Asm_fileContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_asm_file(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_asm_file(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for Asm_fileContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_asm_file(self);
	}
}

impl<'input> CustomRuleContext<'input> for Asm_fileContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_asm_file }
	//fn type_rule_index() -> usize where Self: Sized { RULE_asm_file }
}
antlr_rust::tid!{Asm_fileContextExt<'a>}

impl<'input> Asm_fileContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Asm_fileContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Asm_fileContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Asm_fileContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<Asm_fileContextExt<'input>>{

fn row_all(&self) ->  Vec<Rc<RowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn row(&self, i: usize) -> Option<Rc<RowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,assemblerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> Asm_fileContextAttrs<'input> for Asm_fileContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn asm_file(&mut self,)
	-> Result<Rc<Asm_fileContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Asm_fileContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_asm_file);
        let mut _localctx: Rc<Asm_fileContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(19);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(16);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(21);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule row*/
			recog.base.set_state(22);
			recog.row()?;

			recog.base.set_state(32);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(26);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==NEWLINE {
						{
						{
						recog.base.set_state(23);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(28);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule row*/
					recog.base.set_state(29);
					recog.row()?;

					}
					} 
				}
				recog.base.set_state(34);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			}
			recog.base.set_state(38);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(35);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(40);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(41);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- row ----------------
pub type RowContextAll<'input> = RowContext<'input>;


pub type RowContext<'input> = BaseParserRuleContext<'input,RowContextExt<'input>>;

#[derive(Clone)]
pub struct RowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for RowContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for RowContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_row(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_row(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for RowContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_row(self);
	}
}

impl<'input> CustomRuleContext<'input> for RowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_row }
	//fn type_rule_index() -> usize where Self: Sized { RULE_row }
}
antlr_rust::tid!{RowContextExt<'a>}

impl<'input> RowContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RowContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RowContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<RowContextExt<'input>>{

fn instruction(&self) -> Option<Rc<InstructionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn label_definition(&self) -> Option<Rc<Label_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,assemblerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn asm_intrinsic_usage_all(&self) ->  Vec<Rc<Asm_intrinsic_usageContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn asm_intrinsic_usage(&self, i: usize) -> Option<Rc<Asm_intrinsic_usageContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
fn asm_instrinsic_instruction(&self) -> Option<Rc<Asm_instrinsic_instructionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RowContextAttrs<'input> for RowContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn row(&mut self,)
	-> Result<Rc<RowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_row);
        let mut _localctx: Rc<RowContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(61);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LDI | IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					{
					recog.base.set_state(44);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==IDENTIFIER {
						{
						/*InvokeRule label_definition*/
						recog.base.set_state(43);
						recog.label_definition()?;

						}
					}

					{
					/*InvokeRule instruction*/
					recog.base.set_state(46);
					recog.instruction()?;

					{
					recog.base.set_state(50);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(47);
							recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule expression*/
							recog.base.set_state(48);
							recog.expression()?;

							}
						}
					,
						3 =>{
							{
							/*InvokeRule asm_intrinsic_usage*/
							recog.base.set_state(49);
							recog.asm_intrinsic_usage()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(58);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(52);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						recog.base.set_state(56);
						recog.err_handler.sync(&mut recog.base)?;
						match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
							1 =>{
								{
								recog.base.set_state(53);
								recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

								}
							}
						,
							2 =>{
								{
								/*InvokeRule expression*/
								recog.base.set_state(54);
								recog.expression()?;

								}
							}
						,
							3 =>{
								{
								/*InvokeRule asm_intrinsic_usage*/
								recog.base.set_state(55);
								recog.asm_intrinsic_usage()?;

								}
							}

							_ => {}
						}
						}
					}

					}
					}
					}
					}
				}

			 DOT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule asm_instrinsic_instruction*/
					recog.base.set_state(60);
					recog.asm_instrinsic_instruction()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- label_definition ----------------
pub type Label_definitionContextAll<'input> = Label_definitionContext<'input>;


pub type Label_definitionContext<'input> = BaseParserRuleContext<'input,Label_definitionContextExt<'input>>;

#[derive(Clone)]
pub struct Label_definitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for Label_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for Label_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_label_definition(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_label_definition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for Label_definitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_label_definition(self);
	}
}

impl<'input> CustomRuleContext<'input> for Label_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_label_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_label_definition }
}
antlr_rust::tid!{Label_definitionContextExt<'a>}

impl<'input> Label_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Label_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Label_definitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Label_definitionContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<Label_definitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> Label_definitionContextAttrs<'input> for Label_definitionContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn label_definition(&mut self,)
	-> Result<Rc<Label_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Label_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_label_definition);
        let mut _localctx: Rc<Label_definitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(63);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(64);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameter ----------------
pub type ParameterContextAll<'input> = ParameterContext<'input>;


pub type ParameterContext<'input> = BaseParserRuleContext<'input,ParameterContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for ParameterContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for ParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_parameter(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_parameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for ParameterContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_parameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameter }
}
antlr_rust::tid!{ParameterContextExt<'a>}

impl<'input> ParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<ParameterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ParameterContextAttrs<'input> for ParameterContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameter(&mut self,)
	-> Result<Rc<ParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_parameter);
        let mut _localctx: Rc<ParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(66);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX_NUMBER
/// Returns `None` if there is no child corresponding to token HEX_NUMBER
fn HEX_NUMBER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(HEX_NUMBER, 0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(68);
			_la = recog.base.input.la(1);
			if { !(_la==NUMBER || _la==HEX_NUMBER) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- asm_instrinsic_instruction ----------------
pub type Asm_instrinsic_instructionContextAll<'input> = Asm_instrinsic_instructionContext<'input>;


pub type Asm_instrinsic_instructionContext<'input> = BaseParserRuleContext<'input,Asm_instrinsic_instructionContextExt<'input>>;

#[derive(Clone)]
pub struct Asm_instrinsic_instructionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for Asm_instrinsic_instructionContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for Asm_instrinsic_instructionContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_asm_instrinsic_instruction(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_asm_instrinsic_instruction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for Asm_instrinsic_instructionContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_asm_instrinsic_instruction(self);
	}
}

impl<'input> CustomRuleContext<'input> for Asm_instrinsic_instructionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_asm_instrinsic_instruction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_asm_instrinsic_instruction }
}
antlr_rust::tid!{Asm_instrinsic_instructionContextExt<'a>}

impl<'input> Asm_instrinsic_instructionContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Asm_instrinsic_instructionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Asm_instrinsic_instructionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Asm_instrinsic_instructionContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<Asm_instrinsic_instructionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token DEF
/// Returns `None` if there is no child corresponding to token DEF
fn DEF(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(DEF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,assemblerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
/// Retrieves first TerminalNode corresponding to token EQU
/// Returns `None` if there is no child corresponding to token EQU
fn EQU(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(EQU, 0)
}
/// Retrieves first TerminalNode corresponding to token CSEG
/// Returns `None` if there is no child corresponding to token CSEG
fn CSEG(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CSEG, 0)
}
/// Retrieves first TerminalNode corresponding to token ORG
/// Returns `None` if there is no child corresponding to token ORG
fn ORG(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ORG, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX_NUMBER
/// Returns `None` if there is no child corresponding to token HEX_NUMBER
fn HEX_NUMBER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(HEX_NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token MACRO
/// Returns `None` if there is no child corresponding to token MACRO
fn MACRO(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(MACRO, 0)
}
/// Retrieves first TerminalNode corresponding to token END_MACRO
/// Returns `None` if there is no child corresponding to token END_MACRO
fn END_MACRO(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(END_MACRO, 0)
}
/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ELSE, 0)
}
/// Retrieves first TerminalNode corresponding to token ENDIF
/// Returns `None` if there is no child corresponding to token ENDIF
fn ENDIF(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ENDIF, 0)
}
/// Retrieves first TerminalNode corresponding to token ERROR
/// Returns `None` if there is no child corresponding to token ERROR
fn ERROR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ERROR, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token INCLUDE
/// Returns `None` if there is no child corresponding to token INCLUDE
fn INCLUDE(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(INCLUDE, 0)
}

}

impl<'input> Asm_instrinsic_instructionContextAttrs<'input> for Asm_instrinsic_instructionContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn asm_instrinsic_instruction(&mut self,)
	-> Result<Rc<Asm_instrinsic_instructionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Asm_instrinsic_instructionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_asm_instrinsic_instruction);
        let mut _localctx: Rc<Asm_instrinsic_instructionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(70);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(99);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INCLUDE 
				=> {
					{
					{
					recog.base.set_state(71);
					recog.base.match_token(INCLUDE,&mut recog.err_handler)?;

					recog.base.set_state(72);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
					}
				}

			 DEF 
				=> {
					{
					recog.base.set_state(73);
					recog.base.match_token(DEF,&mut recog.err_handler)?;

					recog.base.set_state(74);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(75);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					recog.base.set_state(78);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 NUMBER | HEX_NUMBER 
						=> {
							{
							/*InvokeRule expression*/
							recog.base.set_state(76);
							recog.expression()?;

							}
						}

					 IDENTIFIER 
						=> {
							{
							recog.base.set_state(77);
							recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

			 EQU 
				=> {
					{
					recog.base.set_state(80);
					recog.base.match_token(EQU,&mut recog.err_handler)?;

					recog.base.set_state(81);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(82);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					recog.base.set_state(85);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 NUMBER | HEX_NUMBER 
						=> {
							{
							/*InvokeRule expression*/
							recog.base.set_state(83);
							recog.expression()?;

							}
						}

					 IDENTIFIER 
						=> {
							{
							recog.base.set_state(84);
							recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					}
				}

			 CSEG 
				=> {
					{
					recog.base.set_state(87);
					recog.base.match_token(CSEG,&mut recog.err_handler)?;

					}
				}

			 ORG 
				=> {
					{
					recog.base.set_state(88);
					recog.base.match_token(ORG,&mut recog.err_handler)?;

					recog.base.set_state(89);
					recog.base.match_token(HEX_NUMBER,&mut recog.err_handler)?;

					}
				}

			 MACRO 
				=> {
					{
					recog.base.set_state(90);
					recog.base.match_token(MACRO,&mut recog.err_handler)?;

					recog.base.set_state(91);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 END_MACRO 
				=> {
					{
					recog.base.set_state(92);
					recog.base.match_token(END_MACRO,&mut recog.err_handler)?;

					}
				}

			 IF 
				=> {
					{
					recog.base.set_state(93);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(94);
					recog.expression()?;

					}
				}

			 ELSE 
				=> {
					{
					recog.base.set_state(95);
					recog.base.match_token(ELSE,&mut recog.err_handler)?;

					}
				}

			 ENDIF 
				=> {
					{
					recog.base.set_state(96);
					recog.base.match_token(ENDIF,&mut recog.err_handler)?;

					}
				}

			 ERROR 
				=> {
					{
					recog.base.set_state(97);
					recog.base.match_token(ERROR,&mut recog.err_handler)?;

					recog.base.set_state(98);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- asm_intrinsic_usage ----------------
pub type Asm_intrinsic_usageContextAll<'input> = Asm_intrinsic_usageContext<'input>;


pub type Asm_intrinsic_usageContext<'input> = BaseParserRuleContext<'input,Asm_intrinsic_usageContextExt<'input>>;

#[derive(Clone)]
pub struct Asm_intrinsic_usageContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for Asm_intrinsic_usageContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for Asm_intrinsic_usageContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_asm_intrinsic_usage(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_asm_intrinsic_usage(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for Asm_intrinsic_usageContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_asm_intrinsic_usage(self);
	}
}

impl<'input> CustomRuleContext<'input> for Asm_intrinsic_usageContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_asm_intrinsic_usage }
	//fn type_rule_index() -> usize where Self: Sized { RULE_asm_intrinsic_usage }
}
antlr_rust::tid!{Asm_intrinsic_usageContextExt<'a>}

impl<'input> Asm_intrinsic_usageContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Asm_intrinsic_usageContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Asm_intrinsic_usageContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Asm_intrinsic_usageContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<Asm_intrinsic_usageContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,assemblerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token OPENING_BRACKET
/// Returns `None` if there is no child corresponding to token OPENING_BRACKET
fn OPENING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(OPENING_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSEING_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSEING_BRACKET
fn CLOSEING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLOSEING_BRACKET, 0)
}

}

impl<'input> Asm_intrinsic_usageContextAttrs<'input> for Asm_intrinsic_usageContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn asm_intrinsic_usage(&mut self,)
	-> Result<Rc<Asm_intrinsic_usageContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Asm_intrinsic_usageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_asm_intrinsic_usage);
        let mut _localctx: Rc<Asm_intrinsic_usageContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(101);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(102);
			recog.base.match_token(OPENING_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(103);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(104);
			recog.base.match_token(CLOSEING_BRACKET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- instruction ----------------
pub type InstructionContextAll<'input> = InstructionContext<'input>;


pub type InstructionContext<'input> = BaseParserRuleContext<'input,InstructionContextExt<'input>>;

#[derive(Clone)]
pub struct InstructionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for InstructionContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for InstructionContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_instruction(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_instruction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for InstructionContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_instruction(self);
	}
}

impl<'input> CustomRuleContext<'input> for InstructionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_instruction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_instruction }
}
antlr_rust::tid!{InstructionContextExt<'a>}

impl<'input> InstructionContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InstructionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InstructionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InstructionContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<InstructionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LDI
/// Returns `None` if there is no child corresponding to token LDI
fn LDI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LDI, 0)
}

}

impl<'input> InstructionContextAttrs<'input> for InstructionContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn instruction(&mut self,)
	-> Result<Rc<InstructionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InstructionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_instruction);
        let mut _localctx: Rc<InstructionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(106);
			recog.base.match_token(LDI,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x1f\x6f\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x03\x02\
	\x07\x02\x14\x0a\x02\x0c\x02\x0e\x02\x17\x0b\x02\x03\x02\x03\x02\x07\x02\
	\x1b\x0a\x02\x0c\x02\x0e\x02\x1e\x0b\x02\x03\x02\x07\x02\x21\x0a\x02\x0c\
	\x02\x0e\x02\x24\x0b\x02\x03\x02\x07\x02\x27\x0a\x02\x0c\x02\x0e\x02\x2a\
	\x0b\x02\x03\x02\x03\x02\x03\x03\x05\x03\x2f\x0a\x03\x03\x03\x03\x03\x03\
	\x03\x03\x03\x05\x03\x35\x0a\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\
	\x3b\x0a\x03\x05\x03\x3d\x0a\x03\x03\x03\x05\x03\x40\x0a\x03\x03\x04\x03\
	\x04\x03\x04\x03\x05\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\
	\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\x51\x0a\x07\x03\x07\x03\x07\
	\x03\x07\x03\x07\x03\x07\x05\x07\x58\x0a\x07\x03\x07\x03\x07\x03\x07\x03\
	\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\
	\x07\x66\x0a\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x09\x03\x09\
	\x03\x09\x02\x02\x0a\x02\x04\x06\x08\x0a\x0c\x0e\x10\x02\x03\x03\x02\x1d\
	\x1e\x02\x7d\x02\x15\x03\x02\x02\x02\x04\x3f\x03\x02\x02\x02\x06\x41\x03\
	\x02\x02\x02\x08\x44\x03\x02\x02\x02\x0a\x46\x03\x02\x02\x02\x0c\x48\x03\
	\x02\x02\x02\x0e\x67\x03\x02\x02\x02\x10\x6c\x03\x02\x02\x02\x12\x14\x07\
	\x1b\x02\x02\x13\x12\x03\x02\x02\x02\x14\x17\x03\x02\x02\x02\x15\x13\x03\
	\x02\x02\x02\x15\x16\x03\x02\x02\x02\x16\x18\x03\x02\x02\x02\x17\x15\x03\
	\x02\x02\x02\x18\x22\x05\x04\x03\x02\x19\x1b\x07\x1b\x02\x02\x1a\x19\x03\
	\x02\x02\x02\x1b\x1e\x03\x02\x02\x02\x1c\x1a\x03\x02\x02\x02\x1c\x1d\x03\
	\x02\x02\x02\x1d\x1f\x03\x02\x02\x02\x1e\x1c\x03\x02\x02\x02\x1f\x21\x05\
	\x04\x03\x02\x20\x1c\x03\x02\x02\x02\x21\x24\x03\x02\x02\x02\x22\x20\x03\
	\x02\x02\x02\x22\x23\x03\x02\x02\x02\x23\x28\x03\x02\x02\x02\x24\x22\x03\
	\x02\x02\x02\x25\x27\x07\x1b\x02\x02\x26\x25\x03\x02\x02\x02\x27\x2a\x03\
	\x02\x02\x02\x28\x26\x03\x02\x02\x02\x28\x29\x03\x02\x02\x02\x29\x2b\x03\
	\x02\x02\x02\x2a\x28\x03\x02\x02\x02\x2b\x2c\x07\x02\x02\x03\x2c\x03\x03\
	\x02\x02\x02\x2d\x2f\x05\x06\x04\x02\x2e\x2d\x03\x02\x02\x02\x2e\x2f\x03\
	\x02\x02\x02\x2f\x30\x03\x02\x02\x02\x30\x34\x05\x10\x09\x02\x31\x35\x07\
	\x1f\x02\x02\x32\x35\x05\x0a\x06\x02\x33\x35\x05\x0e\x08\x02\x34\x31\x03\
	\x02\x02\x02\x34\x32\x03\x02\x02\x02\x34\x33\x03\x02\x02\x02\x35\x3c\x03\
	\x02\x02\x02\x36\x3a\x07\x09\x02\x02\x37\x3b\x07\x1f\x02\x02\x38\x3b\x05\
	\x0a\x06\x02\x39\x3b\x05\x0e\x08\x02\x3a\x37\x03\x02\x02\x02\x3a\x38\x03\
	\x02\x02\x02\x3a\x39\x03\x02\x02\x02\x3b\x3d\x03\x02\x02\x02\x3c\x36\x03\
	\x02\x02\x02\x3c\x3d\x03\x02\x02\x02\x3d\x40\x03\x02\x02\x02\x3e\x40\x05\
	\x0c\x07\x02\x3f\x2e\x03\x02\x02\x02\x3f\x3e\x03\x02\x02\x02\x40\x05\x03\
	\x02\x02\x02\x41\x42\x07\x1f\x02\x02\x42\x43\x07\x08\x02\x02\x43\x07\x03\
	\x02\x02\x02\x44\x45\x07\x1f\x02\x02\x45\x09\x03\x02\x02\x02\x46\x47\x09\
	\x02\x02\x02\x47\x0b\x03\x02\x02\x02\x48\x65\x07\x0c\x02\x02\x49\x4a\x07\
	\x14\x02\x02\x4a\x66\x07\x05\x02\x02\x4b\x4c\x07\x0b\x02\x02\x4c\x4d\x07\
	\x1f\x02\x02\x4d\x50\x07\x10\x02\x02\x4e\x51\x05\x0a\x06\x02\x4f\x51\x07\
	\x1f\x02\x02\x50\x4e\x03\x02\x02\x02\x50\x4f\x03\x02\x02\x02\x51\x66\x03\
	\x02\x02\x02\x52\x53\x07\x11\x02\x02\x53\x54\x07\x1f\x02\x02\x54\x57\x07\
	\x10\x02\x02\x55\x58\x05\x0a\x06\x02\x56\x58\x07\x1f\x02\x02\x57\x55\x03\
	\x02\x02\x02\x57\x56\x03\x02\x02\x02\x58\x66\x03\x02\x02\x02\x59\x66\x07\
	\x0a\x02\x02\x5a\x5b\x07\x18\x02\x02\x5b\x66\x07\x1e\x02\x02\x5c\x5d\x07\
	\x15\x02\x02\x5d\x66\x07\x1f\x02\x02\x5e\x66\x07\x0e\x02\x02\x5f\x60\x07\
	\x13\x02\x02\x60\x66\x05\x0a\x06\x02\x61\x66\x07\x0d\x02\x02\x62\x66\x07\
	\x0f\x02\x02\x63\x64\x07\x12\x02\x02\x64\x66\x07\x05\x02\x02\x65\x49\x03\
	\x02\x02\x02\x65\x4b\x03\x02\x02\x02\x65\x52\x03\x02\x02\x02\x65\x59\x03\
	\x02\x02\x02\x65\x5a\x03\x02\x02\x02\x65\x5c\x03\x02\x02\x02\x65\x5e\x03\
	\x02\x02\x02\x65\x5f\x03\x02\x02\x02\x65\x61\x03\x02\x02\x02\x65\x62\x03\
	\x02\x02\x02\x65\x63\x03\x02\x02\x02\x66\x0d\x03\x02\x02\x02\x67\x68\x07\
	\x1f\x02\x02\x68\x69\x07\x17\x02\x02\x69\x6a\x07\x1f\x02\x02\x6a\x6b\x07\
	\x07\x02\x02\x6b\x0f\x03\x02\x02\x02\x6c\x6d\x07\x03\x02\x02\x6d\x11\x03\
	\x02\x02\x02\x0e\x15\x1c\x22\x28\x2e\x34\x3a\x3c\x3f\x50\x57\x65";


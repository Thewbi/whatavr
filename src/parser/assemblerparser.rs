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

		pub const ADD:isize=1; 
		pub const ADIW:isize=2; 
		pub const AND:isize=3; 
		pub const ANDI:isize=4; 
		pub const ASR:isize=5; 
		pub const BCLR:isize=6; 
		pub const BLD:isize=7; 
		pub const BRBC:isize=8; 
		pub const BRBS:isize=9; 
		pub const BRCC:isize=10; 
		pub const BRCS:isize=11; 
		pub const BREAK:isize=12; 
		pub const BREQ:isize=13; 
		pub const BRGE:isize=14; 
		pub const BRHC:isize=15; 
		pub const BRHS:isize=16; 
		pub const BRID:isize=17; 
		pub const BRIE:isize=18; 
		pub const BRLO:isize=19; 
		pub const BRLT:isize=20; 
		pub const BRMI:isize=21; 
		pub const BRNE:isize=22; 
		pub const BRPL:isize=23; 
		pub const BRSH:isize=24; 
		pub const BRTC:isize=25; 
		pub const BRTS:isize=26; 
		pub const BRVC:isize=27; 
		pub const BRVS:isize=28; 
		pub const BSET:isize=29; 
		pub const BST:isize=30; 
		pub const CALL:isize=31; 
		pub const CBI:isize=32; 
		pub const CBR:isize=33; 
		pub const CLC:isize=34; 
		pub const CLH:isize=35; 
		pub const CLI:isize=36; 
		pub const CLN:isize=37; 
		pub const CLR:isize=38; 
		pub const CLS:isize=39; 
		pub const CLT:isize=40; 
		pub const CLV:isize=41; 
		pub const CLZ:isize=42; 
		pub const COM:isize=43; 
		pub const CP:isize=44; 
		pub const CPC:isize=45; 
		pub const CPI:isize=46; 
		pub const CPSE:isize=47; 
		pub const DEC:isize=48; 
		pub const DES:isize=49; 
		pub const EICALL:isize=50; 
		pub const EIJMP:isize=51; 
		pub const ELPM:isize=52; 
		pub const EOR:isize=53; 
		pub const FMUL:isize=54; 
		pub const FMULS:isize=55; 
		pub const FMULSU:isize=56; 
		pub const ICALL:isize=57; 
		pub const IJMP:isize=58; 
		pub const IN:isize=59; 
		pub const INC:isize=60; 
		pub const JMP:isize=61; 
		pub const LAC:isize=62; 
		pub const LAS:isize=63; 
		pub const LAT:isize=64; 
		pub const LD:isize=65; 
		pub const LDI:isize=66; 
		pub const LDS:isize=67; 
		pub const LPM:isize=68; 
		pub const LSL:isize=69; 
		pub const LSR:isize=70; 
		pub const MOV:isize=71; 
		pub const MOVW:isize=72; 
		pub const MUL:isize=73; 
		pub const MULS:isize=74; 
		pub const MULSU:isize=75; 
		pub const NEG:isize=76; 
		pub const NOP:isize=77; 
		pub const OR:isize=78; 
		pub const ORI:isize=79; 
		pub const OUT:isize=80; 
		pub const POP:isize=81; 
		pub const PUSH:isize=82; 
		pub const RCALL:isize=83; 
		pub const RET:isize=84; 
		pub const RETI:isize=85; 
		pub const RJMP:isize=86; 
		pub const ROL:isize=87; 
		pub const ROR:isize=88; 
		pub const SBC:isize=89; 
		pub const SBCI:isize=90; 
		pub const SBI:isize=91; 
		pub const SBIC:isize=92; 
		pub const SBIS:isize=93; 
		pub const SBIW:isize=94; 
		pub const SBR:isize=95; 
		pub const SBRC:isize=96; 
		pub const SBRS:isize=97; 
		pub const SEC:isize=98; 
		pub const SEH:isize=99; 
		pub const SEI:isize=100; 
		pub const SEN:isize=101; 
		pub const SER:isize=102; 
		pub const SES:isize=103; 
		pub const SET:isize=104; 
		pub const SEV:isize=105; 
		pub const SEZ:isize=106; 
		pub const SLEEP:isize=107; 
		pub const SPM:isize=108; 
		pub const ST:isize=109; 
		pub const STS:isize=110; 
		pub const SUB:isize=111; 
		pub const SUBI:isize=112; 
		pub const SWAP:isize=113; 
		pub const TST:isize=114; 
		pub const WDR:isize=115; 
		pub const XCH:isize=116; 
		pub const NEWLINE:isize=117; 
		pub const WS:isize=118; 
		pub const LINE_COMMENT:isize=119; 
		pub const STRING:isize=120; 
		pub const ASTERISK:isize=121; 
		pub const AT:isize=122; 
		pub const CLOSEING_BRACKET:isize=123; 
		pub const COLON:isize=124; 
		pub const COMMA:isize=125; 
		pub const CSEG:isize=126; 
		pub const DEF:isize=127; 
		pub const DOT:isize=128; 
		pub const ELSE:isize=129; 
		pub const END_MACRO:isize=130; 
		pub const ENDIF:isize=131; 
		pub const EQUALS:isize=132; 
		pub const EQU:isize=133; 
		pub const ERROR:isize=134; 
		pub const GT:isize=135; 
		pub const IF:isize=136; 
		pub const INCLUDE:isize=137; 
		pub const LEFT_SHIFT:isize=138; 
		pub const LT:isize=139; 
		pub const MACRO:isize=140; 
		pub const MINUS:isize=141; 
		pub const OPENING_BRACKET:isize=142; 
		pub const ORG:isize=143; 
		pub const PLUS:isize=144; 
		pub const RIGHT_SHIFT:isize=145; 
		pub const SLASH:isize=146; 
		pub const NUMBER:isize=147; 
		pub const HEX_NUMBER:isize=148; 
		pub const IDENTIFIER:isize=149;
	pub const RULE_asm_file:usize = 0; 
	pub const RULE_row:usize = 1; 
	pub const RULE_instruction:usize = 2; 
	pub const RULE_param:usize = 3; 
	pub const RULE_macro_usage:usize = 4; 
	pub const RULE_label_definition:usize = 5; 
	pub const RULE_parameter:usize = 6; 
	pub const RULE_macro_placeholder:usize = 7; 
	pub const RULE_expression:usize = 8; 
	pub const RULE_asm_instrinsic_instruction:usize = 9; 
	pub const RULE_asm_intrinsic_usage:usize = 10; 
	pub const RULE_mnemonic:usize = 11;
	pub const ruleNames: [&'static str; 12] =  [
		"asm_file", "row", "instruction", "param", "macro_usage", "label_definition", 
		"parameter", "macro_placeholder", "expression", "asm_instrinsic_instruction", 
		"asm_intrinsic_usage", "mnemonic"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;147] = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, Some("'*'"), Some("'@'"), Some("')'"), Some("':'"), Some("','"), 
		Some("'cseg'"), Some("'def'"), Some("'.'"), Some("'else'"), Some("'endmacro'"), 
		Some("'endif'"), Some("'='"), Some("'equ'"), Some("'error'"), Some("'>'"), 
		Some("'if'"), Some("'include'"), Some("'<<'"), Some("'<'"), Some("'macro'"), 
		Some("'-'"), Some("'('"), Some("'org'"), Some("'+'"), Some("'>>'"), Some("'/'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;150]  = [
		None, Some("ADD"), Some("ADIW"), Some("AND"), Some("ANDI"), Some("ASR"), 
		Some("BCLR"), Some("BLD"), Some("BRBC"), Some("BRBS"), Some("BRCC"), Some("BRCS"), 
		Some("BREAK"), Some("BREQ"), Some("BRGE"), Some("BRHC"), Some("BRHS"), 
		Some("BRID"), Some("BRIE"), Some("BRLO"), Some("BRLT"), Some("BRMI"), 
		Some("BRNE"), Some("BRPL"), Some("BRSH"), Some("BRTC"), Some("BRTS"), 
		Some("BRVC"), Some("BRVS"), Some("BSET"), Some("BST"), Some("CALL"), Some("CBI"), 
		Some("CBR"), Some("CLC"), Some("CLH"), Some("CLI"), Some("CLN"), Some("CLR"), 
		Some("CLS"), Some("CLT"), Some("CLV"), Some("CLZ"), Some("COM"), Some("CP"), 
		Some("CPC"), Some("CPI"), Some("CPSE"), Some("DEC"), Some("DES"), Some("EICALL"), 
		Some("EIJMP"), Some("ELPM"), Some("EOR"), Some("FMUL"), Some("FMULS"), 
		Some("FMULSU"), Some("ICALL"), Some("IJMP"), Some("IN"), Some("INC"), 
		Some("JMP"), Some("LAC"), Some("LAS"), Some("LAT"), Some("LD"), Some("LDI"), 
		Some("LDS"), Some("LPM"), Some("LSL"), Some("LSR"), Some("MOV"), Some("MOVW"), 
		Some("MUL"), Some("MULS"), Some("MULSU"), Some("NEG"), Some("NOP"), Some("OR"), 
		Some("ORI"), Some("OUT"), Some("POP"), Some("PUSH"), Some("RCALL"), Some("RET"), 
		Some("RETI"), Some("RJMP"), Some("ROL"), Some("ROR"), Some("SBC"), Some("SBCI"), 
		Some("SBI"), Some("SBIC"), Some("SBIS"), Some("SBIW"), Some("SBR"), Some("SBRC"), 
		Some("SBRS"), Some("SEC"), Some("SEH"), Some("SEI"), Some("SEN"), Some("SER"), 
		Some("SES"), Some("SET"), Some("SEV"), Some("SEZ"), Some("SLEEP"), Some("SPM"), 
		Some("ST"), Some("STS"), Some("SUB"), Some("SUBI"), Some("SWAP"), Some("TST"), 
		Some("WDR"), Some("XCH"), Some("NEWLINE"), Some("WS"), Some("LINE_COMMENT"), 
		Some("STRING"), Some("ASTERISK"), Some("AT"), Some("CLOSEING_BRACKET"), 
		Some("COLON"), Some("COMMA"), Some("CSEG"), Some("DEF"), Some("DOT"), 
		Some("ELSE"), Some("END_MACRO"), Some("ENDIF"), Some("EQUALS"), Some("EQU"), 
		Some("ERROR"), Some("GT"), Some("IF"), Some("INCLUDE"), Some("LEFT_SHIFT"), 
		Some("LT"), Some("MACRO"), Some("MINUS"), Some("OPENING_BRACKET"), Some("ORG"), 
		Some("PLUS"), Some("RIGHT_SHIFT"), Some("SLASH"), Some("NUMBER"), Some("HEX_NUMBER"), 
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
	fn sempred(_localctx: Option<&(dyn assemblerParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					8 => assemblerParser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> assemblerParser<'input, I, DefaultErrorStrategy<'input,assemblerParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 5)
				}
				1=>{
					recog.precpred(None, 4)
				}
				2=>{
					recog.precpred(None, 3)
				}
				3=>{
					recog.precpred(None, 2)
				}
				4=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
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
			recog.base.set_state(27);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(24);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(29);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule row*/
			recog.base.set_state(30);
			recog.row()?;

			recog.base.set_state(40);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(34);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==NEWLINE {
						{
						{
						recog.base.set_state(31);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(36);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule row*/
					recog.base.set_state(37);
					recog.row()?;

					}
					} 
				}
				recog.base.set_state(42);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			}
			recog.base.set_state(46);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(43);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(48);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(49);
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

fn macro_usage(&self) -> Option<Rc<Macro_usageContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn label_definition(&self) -> Option<Rc<Label_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn instruction(&self) -> Option<Rc<InstructionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
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
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(55);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule macro_usage*/
					recog.base.set_state(51);
					recog.macro_usage()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule label_definition*/
					recog.base.set_state(52);
					recog.label_definition()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule instruction*/
					recog.base.set_state(53);
					recog.instruction()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule asm_instrinsic_instruction*/
					recog.base.set_state(54);
					recog.asm_instrinsic_instruction()?;

					}
				}

				_ => {}
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

fn mnemonic(&self) -> Option<Rc<MnemonicContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn param_all(&self) ->  Vec<Rc<ParamContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn param(&self, i: usize) -> Option<Rc<ParamContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
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
        recog.base.enter_rule(_localctx.clone(), 4, RULE_instruction);
        let mut _localctx: Rc<InstructionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule mnemonic*/
			recog.base.set_state(57);
			recog.mnemonic()?;

			recog.base.set_state(63);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule param*/
					recog.base.set_state(58);
					recog.param()?;

					recog.base.set_state(61);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(59);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule param*/
						recog.base.set_state(60);
						recog.param()?;

						}
					}

					}
				}

				_ => {}
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
//------------------- param ----------------
pub type ParamContextAll<'input> = ParamContext<'input>;


pub type ParamContext<'input> = BaseParserRuleContext<'input,ParamContextExt<'input>>;

#[derive(Clone)]
pub struct ParamContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for ParamContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for ParamContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_param(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_param(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for ParamContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_param(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_param }
	//fn type_rule_index() -> usize where Self: Sized { RULE_param }
}
antlr_rust::tid!{ParamContextExt<'a>}

impl<'input> ParamContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<ParamContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn asm_intrinsic_usage(&self) -> Option<Rc<Asm_intrinsic_usageContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn macro_placeholder(&self) -> Option<Rc<Macro_placeholderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParamContextAttrs<'input> for ParamContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn param(&mut self,)
	-> Result<Rc<ParamContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParamContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_param);
        let mut _localctx: Rc<ParamContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(69);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(65);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(66);
					recog.expression_rec(0)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule asm_intrinsic_usage*/
					recog.base.set_state(67);
					recog.asm_intrinsic_usage()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule macro_placeholder*/
					recog.base.set_state(68);
					recog.macro_placeholder()?;

					}
				}

				_ => {}
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
//------------------- macro_usage ----------------
pub type Macro_usageContextAll<'input> = Macro_usageContext<'input>;


pub type Macro_usageContext<'input> = BaseParserRuleContext<'input,Macro_usageContextExt<'input>>;

#[derive(Clone)]
pub struct Macro_usageContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for Macro_usageContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for Macro_usageContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_macro_usage(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_macro_usage(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for Macro_usageContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_macro_usage(self);
	}
}

impl<'input> CustomRuleContext<'input> for Macro_usageContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_macro_usage }
	//fn type_rule_index() -> usize where Self: Sized { RULE_macro_usage }
}
antlr_rust::tid!{Macro_usageContextExt<'a>}

impl<'input> Macro_usageContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Macro_usageContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Macro_usageContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Macro_usageContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<Macro_usageContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Macro_usageContextAttrs<'input> for Macro_usageContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn macro_usage(&mut self,)
	-> Result<Rc<Macro_usageContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Macro_usageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_macro_usage);
        let mut _localctx: Rc<Macro_usageContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(71);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(75);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule expression*/
					recog.base.set_state(72);
					recog.expression_rec(0)?;

					}
					} 
				}
				recog.base.set_state(77);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
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
        recog.base.enter_rule(_localctx.clone(), 10, RULE_label_definition);
        let mut _localctx: Rc<Label_definitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(78);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(79);
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
        recog.base.enter_rule(_localctx.clone(), 12, RULE_parameter);
        let mut _localctx: Rc<ParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(81);
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
//------------------- macro_placeholder ----------------
pub type Macro_placeholderContextAll<'input> = Macro_placeholderContext<'input>;


pub type Macro_placeholderContext<'input> = BaseParserRuleContext<'input,Macro_placeholderContextExt<'input>>;

#[derive(Clone)]
pub struct Macro_placeholderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for Macro_placeholderContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for Macro_placeholderContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_macro_placeholder(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_macro_placeholder(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for Macro_placeholderContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_macro_placeholder(self);
	}
}

impl<'input> CustomRuleContext<'input> for Macro_placeholderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_macro_placeholder }
	//fn type_rule_index() -> usize where Self: Sized { RULE_macro_placeholder }
}
antlr_rust::tid!{Macro_placeholderContextExt<'a>}

impl<'input> Macro_placeholderContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Macro_placeholderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Macro_placeholderContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Macro_placeholderContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<Macro_placeholderContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AT
/// Returns `None` if there is no child corresponding to token AT
fn AT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(AT, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}

}

impl<'input> Macro_placeholderContextAttrs<'input> for Macro_placeholderContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn macro_placeholder(&mut self,)
	-> Result<Rc<Macro_placeholderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Macro_placeholderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_macro_placeholder);
        let mut _localctx: Rc<Macro_placeholderContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(83);
			recog.base.match_token(AT,&mut recog.err_handler)?;

			recog.base.set_state(84);
			recog.base.match_token(NUMBER,&mut recog.err_handler)?;

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
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn macro_placeholder(&self) -> Option<Rc<Macro_placeholderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPENING_BRACKET
/// Returns `None` if there is no child corresponding to token OPENING_BRACKET
fn OPENING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(OPENING_BRACKET, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token CLOSEING_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSEING_BRACKET
fn CLOSEING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLOSEING_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token LEFT_SHIFT
/// Returns `None` if there is no child corresponding to token LEFT_SHIFT
fn LEFT_SHIFT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LEFT_SHIFT, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHT_SHIFT
/// Returns `None` if there is no child corresponding to token RIGHT_SHIFT
fn RIGHT_SHIFT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(RIGHT_SHIFT, 0)
}
/// Retrieves first TerminalNode corresponding to token SLASH
/// Returns `None` if there is no child corresponding to token SLASH
fn SLASH(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SLASH, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: isize)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 16, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 16;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(95);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 NUMBER 
				=> {
					{
					recog.base.set_state(87);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

			 HEX_NUMBER 
				=> {
					{
					recog.base.set_state(88);
					recog.base.match_token(HEX_NUMBER,&mut recog.err_handler)?;

					}
				}

			 IDENTIFIER 
				=> {
					{
					recog.base.set_state(89);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 AT 
				=> {
					{
					/*InvokeRule macro_placeholder*/
					recog.base.set_state(90);
					recog.macro_placeholder()?;

					}
				}

			 OPENING_BRACKET 
				=> {
					{
					recog.base.set_state(91);
					recog.base.match_token(OPENING_BRACKET,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(92);
					recog.expression_rec(0)?;

					recog.base.set_state(93);
					recog.base.match_token(CLOSEING_BRACKET,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(114);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(112);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(97);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(98);
							recog.base.match_token(LEFT_SHIFT,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(99);
							recog.expression_rec(6)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(100);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(101);
							recog.base.match_token(RIGHT_SHIFT,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(102);
							recog.expression_rec(5)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(103);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(104);
							recog.base.match_token(SLASH,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(105);
							recog.expression_rec(4)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(106);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(107);
							recog.base.match_token(GT,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(108);
							recog.expression_rec(3)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(109);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(110);
							recog.base.match_token(LT,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(111);
							recog.expression_rec(2)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(116);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

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
        recog.base.enter_rule(_localctx.clone(), 18, RULE_asm_instrinsic_instruction);
        let mut _localctx: Rc<Asm_instrinsic_instructionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(117);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(146);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INCLUDE 
				=> {
					{
					{
					recog.base.set_state(118);
					recog.base.match_token(INCLUDE,&mut recog.err_handler)?;

					recog.base.set_state(119);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
					}
				}

			 DEF 
				=> {
					{
					recog.base.set_state(120);
					recog.base.match_token(DEF,&mut recog.err_handler)?;

					recog.base.set_state(121);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(122);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					recog.base.set_state(125);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule expression*/
							recog.base.set_state(123);
							recog.expression_rec(0)?;

							}
						}
					,
						2 =>{
							{
							recog.base.set_state(124);
							recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}

			 EQU 
				=> {
					{
					recog.base.set_state(127);
					recog.base.match_token(EQU,&mut recog.err_handler)?;

					recog.base.set_state(128);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(129);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					recog.base.set_state(132);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule expression*/
							recog.base.set_state(130);
							recog.expression_rec(0)?;

							}
						}
					,
						2 =>{
							{
							recog.base.set_state(131);
							recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}

			 CSEG 
				=> {
					{
					recog.base.set_state(134);
					recog.base.match_token(CSEG,&mut recog.err_handler)?;

					}
				}

			 ORG 
				=> {
					{
					recog.base.set_state(135);
					recog.base.match_token(ORG,&mut recog.err_handler)?;

					recog.base.set_state(136);
					recog.base.match_token(HEX_NUMBER,&mut recog.err_handler)?;

					}
				}

			 MACRO 
				=> {
					{
					recog.base.set_state(137);
					recog.base.match_token(MACRO,&mut recog.err_handler)?;

					recog.base.set_state(138);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 END_MACRO 
				=> {
					{
					recog.base.set_state(139);
					recog.base.match_token(END_MACRO,&mut recog.err_handler)?;

					}
				}

			 IF 
				=> {
					{
					recog.base.set_state(140);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(141);
					recog.expression_rec(0)?;

					}
				}

			 ELSE 
				=> {
					{
					recog.base.set_state(142);
					recog.base.match_token(ELSE,&mut recog.err_handler)?;

					}
				}

			 ENDIF 
				=> {
					{
					recog.base.set_state(143);
					recog.base.match_token(ENDIF,&mut recog.err_handler)?;

					}
				}

			 ERROR 
				=> {
					{
					recog.base.set_state(144);
					recog.base.match_token(ERROR,&mut recog.err_handler)?;

					recog.base.set_state(145);
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
fn macro_placeholder(&self) -> Option<Rc<Macro_placeholderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
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
        recog.base.enter_rule(_localctx.clone(), 20, RULE_asm_intrinsic_usage);
        let mut _localctx: Rc<Asm_intrinsic_usageContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(148);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(149);
			recog.base.match_token(OPENING_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(152);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					{
					recog.base.set_state(150);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 AT 
				=> {
					{
					/*InvokeRule macro_placeholder*/
					recog.base.set_state(151);
					recog.macro_placeholder()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(154);
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
//------------------- mnemonic ----------------
pub type MnemonicContextAll<'input> = MnemonicContext<'input>;


pub type MnemonicContext<'input> = BaseParserRuleContext<'input,MnemonicContextExt<'input>>;

#[derive(Clone)]
pub struct MnemonicContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assemblerParserContext<'input> for MnemonicContext<'input>{}

impl<'input,'a> Listenable<dyn assemblerListener<'input> + 'a> for MnemonicContext<'input>{
		fn enter(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic(self);
		}
		fn exit(&self,listener: &mut (dyn assemblerListener<'input> + 'a)) {
			listener.exit_mnemonic(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assemblerVisitor<'input> + 'a> for MnemonicContext<'input>{
	fn accept(&self,visitor: &mut (dyn assemblerVisitor<'input> + 'a)) {
		visitor.visit_mnemonic(self);
	}
}

impl<'input> CustomRuleContext<'input> for MnemonicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assemblerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic }
}
antlr_rust::tid!{MnemonicContextExt<'a>}

impl<'input> MnemonicContextExt<'input>{
	fn new(parent: Option<Rc<dyn assemblerParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MnemonicContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MnemonicContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MnemonicContextAttrs<'input>: assemblerParserContext<'input> + BorrowMut<MnemonicContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token ADIW
/// Returns `None` if there is no child corresponding to token ADIW
fn ADIW(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ADIW, 0)
}
/// Retrieves first TerminalNode corresponding to token AND
/// Returns `None` if there is no child corresponding to token AND
fn AND(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(AND, 0)
}
/// Retrieves first TerminalNode corresponding to token ANDI
/// Returns `None` if there is no child corresponding to token ANDI
fn ANDI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ANDI, 0)
}
/// Retrieves first TerminalNode corresponding to token ASR
/// Returns `None` if there is no child corresponding to token ASR
fn ASR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ASR, 0)
}
/// Retrieves first TerminalNode corresponding to token BCLR
/// Returns `None` if there is no child corresponding to token BCLR
fn BCLR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BCLR, 0)
}
/// Retrieves first TerminalNode corresponding to token BLD
/// Returns `None` if there is no child corresponding to token BLD
fn BLD(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BLD, 0)
}
/// Retrieves first TerminalNode corresponding to token BRBC
/// Returns `None` if there is no child corresponding to token BRBC
fn BRBC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRBC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRBS
/// Returns `None` if there is no child corresponding to token BRBS
fn BRBS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRBS, 0)
}
/// Retrieves first TerminalNode corresponding to token BRCC
/// Returns `None` if there is no child corresponding to token BRCC
fn BRCC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRCC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRCS
/// Returns `None` if there is no child corresponding to token BRCS
fn BRCS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRCS, 0)
}
/// Retrieves first TerminalNode corresponding to token BREAK
/// Returns `None` if there is no child corresponding to token BREAK
fn BREAK(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BREAK, 0)
}
/// Retrieves first TerminalNode corresponding to token BREQ
/// Returns `None` if there is no child corresponding to token BREQ
fn BREQ(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BREQ, 0)
}
/// Retrieves first TerminalNode corresponding to token BRGE
/// Returns `None` if there is no child corresponding to token BRGE
fn BRGE(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRGE, 0)
}
/// Retrieves first TerminalNode corresponding to token BRHC
/// Returns `None` if there is no child corresponding to token BRHC
fn BRHC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRHC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRHS
/// Returns `None` if there is no child corresponding to token BRHS
fn BRHS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRHS, 0)
}
/// Retrieves first TerminalNode corresponding to token BRID
/// Returns `None` if there is no child corresponding to token BRID
fn BRID(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRID, 0)
}
/// Retrieves first TerminalNode corresponding to token BRIE
/// Returns `None` if there is no child corresponding to token BRIE
fn BRIE(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRIE, 0)
}
/// Retrieves first TerminalNode corresponding to token BRLO
/// Returns `None` if there is no child corresponding to token BRLO
fn BRLO(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRLO, 0)
}
/// Retrieves first TerminalNode corresponding to token BRLT
/// Returns `None` if there is no child corresponding to token BRLT
fn BRLT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRLT, 0)
}
/// Retrieves first TerminalNode corresponding to token BRMI
/// Returns `None` if there is no child corresponding to token BRMI
fn BRMI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRMI, 0)
}
/// Retrieves first TerminalNode corresponding to token BRNE
/// Returns `None` if there is no child corresponding to token BRNE
fn BRNE(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRNE, 0)
}
/// Retrieves first TerminalNode corresponding to token BRPL
/// Returns `None` if there is no child corresponding to token BRPL
fn BRPL(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRPL, 0)
}
/// Retrieves first TerminalNode corresponding to token BRSH
/// Returns `None` if there is no child corresponding to token BRSH
fn BRSH(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRSH, 0)
}
/// Retrieves first TerminalNode corresponding to token BRTC
/// Returns `None` if there is no child corresponding to token BRTC
fn BRTC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRTC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRTS
/// Returns `None` if there is no child corresponding to token BRTS
fn BRTS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRTS, 0)
}
/// Retrieves first TerminalNode corresponding to token BRVC
/// Returns `None` if there is no child corresponding to token BRVC
fn BRVC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRVC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRVS
/// Returns `None` if there is no child corresponding to token BRVS
fn BRVS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BRVS, 0)
}
/// Retrieves first TerminalNode corresponding to token BSET
/// Returns `None` if there is no child corresponding to token BSET
fn BSET(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BSET, 0)
}
/// Retrieves first TerminalNode corresponding to token BST
/// Returns `None` if there is no child corresponding to token BST
fn BST(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(BST, 0)
}
/// Retrieves first TerminalNode corresponding to token CALL
/// Returns `None` if there is no child corresponding to token CALL
fn CALL(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CALL, 0)
}
/// Retrieves first TerminalNode corresponding to token CBI
/// Returns `None` if there is no child corresponding to token CBI
fn CBI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CBI, 0)
}
/// Retrieves first TerminalNode corresponding to token CBR
/// Returns `None` if there is no child corresponding to token CBR
fn CBR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CBR, 0)
}
/// Retrieves first TerminalNode corresponding to token CLC
/// Returns `None` if there is no child corresponding to token CLC
fn CLC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLC, 0)
}
/// Retrieves first TerminalNode corresponding to token CLH
/// Returns `None` if there is no child corresponding to token CLH
fn CLH(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLH, 0)
}
/// Retrieves first TerminalNode corresponding to token CLI
/// Returns `None` if there is no child corresponding to token CLI
fn CLI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLI, 0)
}
/// Retrieves first TerminalNode corresponding to token CLN
/// Returns `None` if there is no child corresponding to token CLN
fn CLN(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLN, 0)
}
/// Retrieves first TerminalNode corresponding to token CLR
/// Returns `None` if there is no child corresponding to token CLR
fn CLR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLR, 0)
}
/// Retrieves first TerminalNode corresponding to token CLS
/// Returns `None` if there is no child corresponding to token CLS
fn CLS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLS, 0)
}
/// Retrieves first TerminalNode corresponding to token CLT
/// Returns `None` if there is no child corresponding to token CLT
fn CLT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLT, 0)
}
/// Retrieves first TerminalNode corresponding to token CLV
/// Returns `None` if there is no child corresponding to token CLV
fn CLV(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLV, 0)
}
/// Retrieves first TerminalNode corresponding to token CLZ
/// Returns `None` if there is no child corresponding to token CLZ
fn CLZ(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CLZ, 0)
}
/// Retrieves first TerminalNode corresponding to token COM
/// Returns `None` if there is no child corresponding to token COM
fn COM(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(COM, 0)
}
/// Retrieves first TerminalNode corresponding to token CP
/// Returns `None` if there is no child corresponding to token CP
fn CP(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CP, 0)
}
/// Retrieves first TerminalNode corresponding to token CPC
/// Returns `None` if there is no child corresponding to token CPC
fn CPC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CPC, 0)
}
/// Retrieves first TerminalNode corresponding to token CPI
/// Returns `None` if there is no child corresponding to token CPI
fn CPI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CPI, 0)
}
/// Retrieves first TerminalNode corresponding to token CPSE
/// Returns `None` if there is no child corresponding to token CPSE
fn CPSE(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(CPSE, 0)
}
/// Retrieves first TerminalNode corresponding to token DEC
/// Returns `None` if there is no child corresponding to token DEC
fn DEC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(DEC, 0)
}
/// Retrieves first TerminalNode corresponding to token DES
/// Returns `None` if there is no child corresponding to token DES
fn DES(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(DES, 0)
}
/// Retrieves first TerminalNode corresponding to token EICALL
/// Returns `None` if there is no child corresponding to token EICALL
fn EICALL(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(EICALL, 0)
}
/// Retrieves first TerminalNode corresponding to token EIJMP
/// Returns `None` if there is no child corresponding to token EIJMP
fn EIJMP(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(EIJMP, 0)
}
/// Retrieves first TerminalNode corresponding to token ELPM
/// Returns `None` if there is no child corresponding to token ELPM
fn ELPM(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ELPM, 0)
}
/// Retrieves first TerminalNode corresponding to token EOR
/// Returns `None` if there is no child corresponding to token EOR
fn EOR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(EOR, 0)
}
/// Retrieves first TerminalNode corresponding to token FMUL
/// Returns `None` if there is no child corresponding to token FMUL
fn FMUL(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(FMUL, 0)
}
/// Retrieves first TerminalNode corresponding to token FMULS
/// Returns `None` if there is no child corresponding to token FMULS
fn FMULS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(FMULS, 0)
}
/// Retrieves first TerminalNode corresponding to token FMULSU
/// Returns `None` if there is no child corresponding to token FMULSU
fn FMULSU(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(FMULSU, 0)
}
/// Retrieves first TerminalNode corresponding to token ICALL
/// Returns `None` if there is no child corresponding to token ICALL
fn ICALL(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ICALL, 0)
}
/// Retrieves first TerminalNode corresponding to token IJMP
/// Returns `None` if there is no child corresponding to token IJMP
fn IJMP(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IJMP, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token INC
/// Returns `None` if there is no child corresponding to token INC
fn INC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(INC, 0)
}
/// Retrieves first TerminalNode corresponding to token JMP
/// Returns `None` if there is no child corresponding to token JMP
fn JMP(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(JMP, 0)
}
/// Retrieves first TerminalNode corresponding to token LAC
/// Returns `None` if there is no child corresponding to token LAC
fn LAC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LAC, 0)
}
/// Retrieves first TerminalNode corresponding to token LAS
/// Returns `None` if there is no child corresponding to token LAS
fn LAS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LAS, 0)
}
/// Retrieves first TerminalNode corresponding to token LAT
/// Returns `None` if there is no child corresponding to token LAT
fn LAT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LAT, 0)
}
/// Retrieves first TerminalNode corresponding to token LD
/// Returns `None` if there is no child corresponding to token LD
fn LD(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LD, 0)
}
/// Retrieves first TerminalNode corresponding to token LDI
/// Returns `None` if there is no child corresponding to token LDI
fn LDI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LDI, 0)
}
/// Retrieves first TerminalNode corresponding to token LDS
/// Returns `None` if there is no child corresponding to token LDS
fn LDS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LDS, 0)
}
/// Retrieves first TerminalNode corresponding to token LPM
/// Returns `None` if there is no child corresponding to token LPM
fn LPM(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LPM, 0)
}
/// Retrieves first TerminalNode corresponding to token LSL
/// Returns `None` if there is no child corresponding to token LSL
fn LSL(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LSL, 0)
}
/// Retrieves first TerminalNode corresponding to token LSR
/// Returns `None` if there is no child corresponding to token LSR
fn LSR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(LSR, 0)
}
/// Retrieves first TerminalNode corresponding to token MOV
/// Returns `None` if there is no child corresponding to token MOV
fn MOV(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(MOV, 0)
}
/// Retrieves first TerminalNode corresponding to token MOVW
/// Returns `None` if there is no child corresponding to token MOVW
fn MOVW(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(MOVW, 0)
}
/// Retrieves first TerminalNode corresponding to token MUL
/// Returns `None` if there is no child corresponding to token MUL
fn MUL(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(MUL, 0)
}
/// Retrieves first TerminalNode corresponding to token MULS
/// Returns `None` if there is no child corresponding to token MULS
fn MULS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(MULS, 0)
}
/// Retrieves first TerminalNode corresponding to token MULSU
/// Returns `None` if there is no child corresponding to token MULSU
fn MULSU(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(MULSU, 0)
}
/// Retrieves first TerminalNode corresponding to token NEG
/// Returns `None` if there is no child corresponding to token NEG
fn NEG(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(NEG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOP
/// Returns `None` if there is no child corresponding to token NOP
fn NOP(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(NOP, 0)
}
/// Retrieves first TerminalNode corresponding to token OR
/// Returns `None` if there is no child corresponding to token OR
fn OR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(OR, 0)
}
/// Retrieves first TerminalNode corresponding to token ORI
/// Returns `None` if there is no child corresponding to token ORI
fn ORI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ORI, 0)
}
/// Retrieves first TerminalNode corresponding to token OUT
/// Returns `None` if there is no child corresponding to token OUT
fn OUT(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(OUT, 0)
}
/// Retrieves first TerminalNode corresponding to token POP
/// Returns `None` if there is no child corresponding to token POP
fn POP(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(POP, 0)
}
/// Retrieves first TerminalNode corresponding to token PUSH
/// Returns `None` if there is no child corresponding to token PUSH
fn PUSH(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(PUSH, 0)
}
/// Retrieves first TerminalNode corresponding to token RCALL
/// Returns `None` if there is no child corresponding to token RCALL
fn RCALL(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(RCALL, 0)
}
/// Retrieves first TerminalNode corresponding to token RET
/// Returns `None` if there is no child corresponding to token RET
fn RET(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(RET, 0)
}
/// Retrieves first TerminalNode corresponding to token RETI
/// Returns `None` if there is no child corresponding to token RETI
fn RETI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(RETI, 0)
}
/// Retrieves first TerminalNode corresponding to token RJMP
/// Returns `None` if there is no child corresponding to token RJMP
fn RJMP(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(RJMP, 0)
}
/// Retrieves first TerminalNode corresponding to token ROL
/// Returns `None` if there is no child corresponding to token ROL
fn ROL(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ROL, 0)
}
/// Retrieves first TerminalNode corresponding to token ROR
/// Returns `None` if there is no child corresponding to token ROR
fn ROR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ROR, 0)
}
/// Retrieves first TerminalNode corresponding to token SBC
/// Returns `None` if there is no child corresponding to token SBC
fn SBC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SBC, 0)
}
/// Retrieves first TerminalNode corresponding to token SBCI
/// Returns `None` if there is no child corresponding to token SBCI
fn SBCI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SBCI, 0)
}
/// Retrieves first TerminalNode corresponding to token SBI
/// Returns `None` if there is no child corresponding to token SBI
fn SBI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SBI, 0)
}
/// Retrieves first TerminalNode corresponding to token SBIC
/// Returns `None` if there is no child corresponding to token SBIC
fn SBIC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SBIC, 0)
}
/// Retrieves first TerminalNode corresponding to token SBIS
/// Returns `None` if there is no child corresponding to token SBIS
fn SBIS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SBIS, 0)
}
/// Retrieves first TerminalNode corresponding to token SBIW
/// Returns `None` if there is no child corresponding to token SBIW
fn SBIW(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SBIW, 0)
}
/// Retrieves first TerminalNode corresponding to token SBR
/// Returns `None` if there is no child corresponding to token SBR
fn SBR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SBR, 0)
}
/// Retrieves first TerminalNode corresponding to token SBRC
/// Returns `None` if there is no child corresponding to token SBRC
fn SBRC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SBRC, 0)
}
/// Retrieves first TerminalNode corresponding to token SBRS
/// Returns `None` if there is no child corresponding to token SBRS
fn SBRS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SBRS, 0)
}
/// Retrieves first TerminalNode corresponding to token SEC
/// Returns `None` if there is no child corresponding to token SEC
fn SEC(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SEC, 0)
}
/// Retrieves first TerminalNode corresponding to token SEH
/// Returns `None` if there is no child corresponding to token SEH
fn SEH(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SEH, 0)
}
/// Retrieves first TerminalNode corresponding to token SEI
/// Returns `None` if there is no child corresponding to token SEI
fn SEI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SEI, 0)
}
/// Retrieves first TerminalNode corresponding to token SEN
/// Returns `None` if there is no child corresponding to token SEN
fn SEN(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SEN, 0)
}
/// Retrieves first TerminalNode corresponding to token SER
/// Returns `None` if there is no child corresponding to token SER
fn SER(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SER, 0)
}
/// Retrieves first TerminalNode corresponding to token SES
/// Returns `None` if there is no child corresponding to token SES
fn SES(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SES, 0)
}
/// Retrieves first TerminalNode corresponding to token SET
/// Returns `None` if there is no child corresponding to token SET
fn SET(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SET, 0)
}
/// Retrieves first TerminalNode corresponding to token SEV
/// Returns `None` if there is no child corresponding to token SEV
fn SEV(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SEV, 0)
}
/// Retrieves first TerminalNode corresponding to token SEZ
/// Returns `None` if there is no child corresponding to token SEZ
fn SEZ(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SEZ, 0)
}
/// Retrieves first TerminalNode corresponding to token SLEEP
/// Returns `None` if there is no child corresponding to token SLEEP
fn SLEEP(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SLEEP, 0)
}
/// Retrieves first TerminalNode corresponding to token SPM
/// Returns `None` if there is no child corresponding to token SPM
fn SPM(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SPM, 0)
}
/// Retrieves first TerminalNode corresponding to token ST
/// Returns `None` if there is no child corresponding to token ST
fn ST(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(ST, 0)
}
/// Retrieves first TerminalNode corresponding to token STS
/// Returns `None` if there is no child corresponding to token STS
fn STS(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(STS, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SUB, 0)
}
/// Retrieves first TerminalNode corresponding to token SUBI
/// Returns `None` if there is no child corresponding to token SUBI
fn SUBI(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SUBI, 0)
}
/// Retrieves first TerminalNode corresponding to token SWAP
/// Returns `None` if there is no child corresponding to token SWAP
fn SWAP(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(SWAP, 0)
}
/// Retrieves first TerminalNode corresponding to token TST
/// Returns `None` if there is no child corresponding to token TST
fn TST(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(TST, 0)
}
/// Retrieves first TerminalNode corresponding to token WDR
/// Returns `None` if there is no child corresponding to token WDR
fn WDR(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(WDR, 0)
}
/// Retrieves first TerminalNode corresponding to token XCH
/// Returns `None` if there is no child corresponding to token XCH
fn XCH(&self) -> Option<Rc<TerminalNode<'input,assemblerParserContextType>>> where Self:Sized{
	self.get_token(XCH, 0)
}

}

impl<'input> MnemonicContextAttrs<'input> for MnemonicContext<'input>{}

impl<'input, I, H> assemblerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic(&mut self,)
	-> Result<Rc<MnemonicContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MnemonicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_mnemonic);
        let mut _localctx: Rc<MnemonicContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(156);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ADD) | (1usize << ADIW) | (1usize << AND) | (1usize << ANDI) | (1usize << ASR) | (1usize << BCLR) | (1usize << BLD) | (1usize << BRBC) | (1usize << BRBS) | (1usize << BRCC) | (1usize << BRCS) | (1usize << BREAK) | (1usize << BREQ) | (1usize << BRGE) | (1usize << BRHC) | (1usize << BRHS) | (1usize << BRID) | (1usize << BRIE) | (1usize << BRLO) | (1usize << BRLT) | (1usize << BRMI) | (1usize << BRNE) | (1usize << BRPL) | (1usize << BRSH) | (1usize << BRTC) | (1usize << BRTS) | (1usize << BRVC) | (1usize << BRVS) | (1usize << BSET) | (1usize << BST) | (1usize << CALL))) != 0) || ((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (CBI - 32)) | (1usize << (CBR - 32)) | (1usize << (CLC - 32)) | (1usize << (CLH - 32)) | (1usize << (CLI - 32)) | (1usize << (CLN - 32)) | (1usize << (CLR - 32)) | (1usize << (CLS - 32)) | (1usize << (CLT - 32)) | (1usize << (CLV - 32)) | (1usize << (CLZ - 32)) | (1usize << (COM - 32)) | (1usize << (CP - 32)) | (1usize << (CPC - 32)) | (1usize << (CPI - 32)) | (1usize << (CPSE - 32)) | (1usize << (DEC - 32)) | (1usize << (DES - 32)) | (1usize << (EICALL - 32)) | (1usize << (EIJMP - 32)) | (1usize << (ELPM - 32)) | (1usize << (EOR - 32)) | (1usize << (FMUL - 32)) | (1usize << (FMULS - 32)) | (1usize << (FMULSU - 32)) | (1usize << (ICALL - 32)) | (1usize << (IJMP - 32)) | (1usize << (IN - 32)) | (1usize << (INC - 32)) | (1usize << (JMP - 32)) | (1usize << (LAC - 32)) | (1usize << (LAS - 32)))) != 0) || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (LAT - 64)) | (1usize << (LD - 64)) | (1usize << (LDI - 64)) | (1usize << (LDS - 64)) | (1usize << (LPM - 64)) | (1usize << (LSL - 64)) | (1usize << (LSR - 64)) | (1usize << (MOV - 64)) | (1usize << (MOVW - 64)) | (1usize << (MUL - 64)) | (1usize << (MULS - 64)) | (1usize << (MULSU - 64)) | (1usize << (NEG - 64)) | (1usize << (NOP - 64)) | (1usize << (OR - 64)) | (1usize << (ORI - 64)) | (1usize << (OUT - 64)) | (1usize << (POP - 64)) | (1usize << (PUSH - 64)) | (1usize << (RCALL - 64)) | (1usize << (RET - 64)) | (1usize << (RETI - 64)) | (1usize << (RJMP - 64)) | (1usize << (ROL - 64)) | (1usize << (ROR - 64)) | (1usize << (SBC - 64)) | (1usize << (SBCI - 64)) | (1usize << (SBI - 64)) | (1usize << (SBIC - 64)) | (1usize << (SBIS - 64)) | (1usize << (SBIW - 64)) | (1usize << (SBR - 64)))) != 0) || ((((_la - 96)) & !0x3f) == 0 && ((1usize << (_la - 96)) & ((1usize << (SBRC - 96)) | (1usize << (SBRS - 96)) | (1usize << (SEC - 96)) | (1usize << (SEH - 96)) | (1usize << (SEI - 96)) | (1usize << (SEN - 96)) | (1usize << (SER - 96)) | (1usize << (SES - 96)) | (1usize << (SET - 96)) | (1usize << (SEV - 96)) | (1usize << (SEZ - 96)) | (1usize << (SLEEP - 96)) | (1usize << (SPM - 96)) | (1usize << (ST - 96)) | (1usize << (STS - 96)) | (1usize << (SUB - 96)) | (1usize << (SUBI - 96)) | (1usize << (SWAP - 96)) | (1usize << (TST - 96)) | (1usize << (WDR - 96)) | (1usize << (XCH - 96)))) != 0)) } {
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
	\u{97}\u{a1}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x03\x02\x07\
	\x02\x1c\x0a\x02\x0c\x02\x0e\x02\x1f\x0b\x02\x03\x02\x03\x02\x07\x02\x23\
	\x0a\x02\x0c\x02\x0e\x02\x26\x0b\x02\x03\x02\x07\x02\x29\x0a\x02\x0c\x02\
	\x0e\x02\x2c\x0b\x02\x03\x02\x07\x02\x2f\x0a\x02\x0c\x02\x0e\x02\x32\x0b\
	\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\x3a\x0a\x03\
	\x03\x04\x03\x04\x03\x04\x03\x04\x05\x04\x40\x0a\x04\x05\x04\x42\x0a\x04\
	\x03\x05\x03\x05\x03\x05\x03\x05\x05\x05\x48\x0a\x05\x03\x06\x03\x06\x07\
	\x06\x4c\x0a\x06\x0c\x06\x0e\x06\x4f\x0b\x06\x03\x07\x03\x07\x03\x07\x03\
	\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x05\x0a\x62\x0a\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x07\x0a\x73\x0a\x0a\x0c\x0a\x0e\x0a\x76\
	\x0b\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x05\x0b\u{80}\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{87}\
	\x0a\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{95}\x0a\x0b\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x05\x0c\u{9b}\x0a\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\
	\x02\x03\x12\x0e\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x02\x03\
	\x03\x02\x03\x76\x02\u{b7}\x02\x1d\x03\x02\x02\x02\x04\x39\x03\x02\x02\x02\
	\x06\x3b\x03\x02\x02\x02\x08\x47\x03\x02\x02\x02\x0a\x49\x03\x02\x02\x02\
	\x0c\x50\x03\x02\x02\x02\x0e\x53\x03\x02\x02\x02\x10\x55\x03\x02\x02\x02\
	\x12\x61\x03\x02\x02\x02\x14\x77\x03\x02\x02\x02\x16\u{96}\x03\x02\x02\x02\
	\x18\u{9e}\x03\x02\x02\x02\x1a\x1c\x07\x77\x02\x02\x1b\x1a\x03\x02\x02\x02\
	\x1c\x1f\x03\x02\x02\x02\x1d\x1b\x03\x02\x02\x02\x1d\x1e\x03\x02\x02\x02\
	\x1e\x20\x03\x02\x02\x02\x1f\x1d\x03\x02\x02\x02\x20\x2a\x05\x04\x03\x02\
	\x21\x23\x07\x77\x02\x02\x22\x21\x03\x02\x02\x02\x23\x26\x03\x02\x02\x02\
	\x24\x22\x03\x02\x02\x02\x24\x25\x03\x02\x02\x02\x25\x27\x03\x02\x02\x02\
	\x26\x24\x03\x02\x02\x02\x27\x29\x05\x04\x03\x02\x28\x24\x03\x02\x02\x02\
	\x29\x2c\x03\x02\x02\x02\x2a\x28\x03\x02\x02\x02\x2a\x2b\x03\x02\x02\x02\
	\x2b\x30\x03\x02\x02\x02\x2c\x2a\x03\x02\x02\x02\x2d\x2f\x07\x77\x02\x02\
	\x2e\x2d\x03\x02\x02\x02\x2f\x32\x03\x02\x02\x02\x30\x2e\x03\x02\x02\x02\
	\x30\x31\x03\x02\x02\x02\x31\x33\x03\x02\x02\x02\x32\x30\x03\x02\x02\x02\
	\x33\x34\x07\x02\x02\x03\x34\x03\x03\x02\x02\x02\x35\x3a\x05\x0a\x06\x02\
	\x36\x3a\x05\x0c\x07\x02\x37\x3a\x05\x06\x04\x02\x38\x3a\x05\x14\x0b\x02\
	\x39\x35\x03\x02\x02\x02\x39\x36\x03\x02\x02\x02\x39\x37\x03\x02\x02\x02\
	\x39\x38\x03\x02\x02\x02\x3a\x05\x03\x02\x02\x02\x3b\x41\x05\x18\x0d\x02\
	\x3c\x3f\x05\x08\x05\x02\x3d\x3e\x07\x7f\x02\x02\x3e\x40\x05\x08\x05\x02\
	\x3f\x3d\x03\x02\x02\x02\x3f\x40\x03\x02\x02\x02\x40\x42\x03\x02\x02\x02\
	\x41\x3c\x03\x02\x02\x02\x41\x42\x03\x02\x02\x02\x42\x07\x03\x02\x02\x02\
	\x43\x48\x07\u{97}\x02\x02\x44\x48\x05\x12\x0a\x02\x45\x48\x05\x16\x0c\x02\
	\x46\x48\x05\x10\x09\x02\x47\x43\x03\x02\x02\x02\x47\x44\x03\x02\x02\x02\
	\x47\x45\x03\x02\x02\x02\x47\x46\x03\x02\x02\x02\x48\x09\x03\x02\x02\x02\
	\x49\x4d\x07\u{97}\x02\x02\x4a\x4c\x05\x12\x0a\x02\x4b\x4a\x03\x02\x02\x02\
	\x4c\x4f\x03\x02\x02\x02\x4d\x4b\x03\x02\x02\x02\x4d\x4e\x03\x02\x02\x02\
	\x4e\x0b\x03\x02\x02\x02\x4f\x4d\x03\x02\x02\x02\x50\x51\x07\u{97}\x02\x02\
	\x51\x52\x07\x7e\x02\x02\x52\x0d\x03\x02\x02\x02\x53\x54\x07\u{97}\x02\x02\
	\x54\x0f\x03\x02\x02\x02\x55\x56\x07\x7c\x02\x02\x56\x57\x07\u{95}\x02\x02\
	\x57\x11\x03\x02\x02\x02\x58\x59\x08\x0a\x01\x02\x59\x62\x07\u{95}\x02\x02\
	\x5a\x62\x07\u{96}\x02\x02\x5b\x62\x07\u{97}\x02\x02\x5c\x62\x05\x10\x09\
	\x02\x5d\x5e\x07\u{90}\x02\x02\x5e\x5f\x05\x12\x0a\x02\x5f\x60\x07\x7d\x02\
	\x02\x60\x62\x03\x02\x02\x02\x61\x58\x03\x02\x02\x02\x61\x5a\x03\x02\x02\
	\x02\x61\x5b\x03\x02\x02\x02\x61\x5c\x03\x02\x02\x02\x61\x5d\x03\x02\x02\
	\x02\x62\x74\x03\x02\x02\x02\x63\x64\x0c\x07\x02\x02\x64\x65\x07\u{8c}\x02\
	\x02\x65\x73\x05\x12\x0a\x08\x66\x67\x0c\x06\x02\x02\x67\x68\x07\u{93}\x02\
	\x02\x68\x73\x05\x12\x0a\x07\x69\x6a\x0c\x05\x02\x02\x6a\x6b\x07\u{94}\x02\
	\x02\x6b\x73\x05\x12\x0a\x06\x6c\x6d\x0c\x04\x02\x02\x6d\x6e\x07\u{89}\x02\
	\x02\x6e\x73\x05\x12\x0a\x05\x6f\x70\x0c\x03\x02\x02\x70\x71\x07\u{8d}\x02\
	\x02\x71\x73\x05\x12\x0a\x04\x72\x63\x03\x02\x02\x02\x72\x66\x03\x02\x02\
	\x02\x72\x69\x03\x02\x02\x02\x72\x6c\x03\x02\x02\x02\x72\x6f\x03\x02\x02\
	\x02\x73\x76\x03\x02\x02\x02\x74\x72\x03\x02\x02\x02\x74\x75\x03\x02\x02\
	\x02\x75\x13\x03\x02\x02\x02\x76\x74\x03\x02\x02\x02\x77\u{94}\x07\u{82}\
	\x02\x02\x78\x79\x07\u{8b}\x02\x02\x79\u{95}\x07\x7a\x02\x02\x7a\x7b\x07\
	\u{81}\x02\x02\x7b\x7c\x07\u{97}\x02\x02\x7c\x7f\x07\u{86}\x02\x02\x7d\u{80}\
	\x05\x12\x0a\x02\x7e\u{80}\x07\u{97}\x02\x02\x7f\x7d\x03\x02\x02\x02\x7f\
	\x7e\x03\x02\x02\x02\u{80}\u{95}\x03\x02\x02\x02\u{81}\u{82}\x07\u{87}\x02\
	\x02\u{82}\u{83}\x07\u{97}\x02\x02\u{83}\u{86}\x07\u{86}\x02\x02\u{84}\u{87}\
	\x05\x12\x0a\x02\u{85}\u{87}\x07\u{97}\x02\x02\u{86}\u{84}\x03\x02\x02\x02\
	\u{86}\u{85}\x03\x02\x02\x02\u{87}\u{95}\x03\x02\x02\x02\u{88}\u{95}\x07\
	\u{80}\x02\x02\u{89}\u{8a}\x07\u{91}\x02\x02\u{8a}\u{95}\x07\u{96}\x02\x02\
	\u{8b}\u{8c}\x07\u{8e}\x02\x02\u{8c}\u{95}\x07\u{97}\x02\x02\u{8d}\u{95}\
	\x07\u{84}\x02\x02\u{8e}\u{8f}\x07\u{8a}\x02\x02\u{8f}\u{95}\x05\x12\x0a\
	\x02\u{90}\u{95}\x07\u{83}\x02\x02\u{91}\u{95}\x07\u{85}\x02\x02\u{92}\u{93}\
	\x07\u{88}\x02\x02\u{93}\u{95}\x07\x7a\x02\x02\u{94}\x78\x03\x02\x02\x02\
	\u{94}\x7a\x03\x02\x02\x02\u{94}\u{81}\x03\x02\x02\x02\u{94}\u{88}\x03\x02\
	\x02\x02\u{94}\u{89}\x03\x02\x02\x02\u{94}\u{8b}\x03\x02\x02\x02\u{94}\u{8d}\
	\x03\x02\x02\x02\u{94}\u{8e}\x03\x02\x02\x02\u{94}\u{90}\x03\x02\x02\x02\
	\u{94}\u{91}\x03\x02\x02\x02\u{94}\u{92}\x03\x02\x02\x02\u{95}\x15\x03\x02\
	\x02\x02\u{96}\u{97}\x07\u{97}\x02\x02\u{97}\u{9a}\x07\u{90}\x02\x02\u{98}\
	\u{9b}\x07\u{97}\x02\x02\u{99}\u{9b}\x05\x10\x09\x02\u{9a}\u{98}\x03\x02\
	\x02\x02\u{9a}\u{99}\x03\x02\x02\x02\u{9b}\u{9c}\x03\x02\x02\x02\u{9c}\u{9d}\
	\x07\x7d\x02\x02\u{9d}\x17\x03\x02\x02\x02\u{9e}\u{9f}\x09\x02\x02\x02\u{9f}\
	\x19\x03\x02\x02\x02\x12\x1d\x24\x2a\x30\x39\x3f\x41\x47\x4d\x61\x72\x74\
	\x7f\u{86}\u{94}\u{9a}";


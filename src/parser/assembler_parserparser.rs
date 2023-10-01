// Generated from ../../src/parser/assembler_parser.g4 by ANTLR 4.8
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
use super::assembler_parserlistener::*;
use super::assembler_parservisitor::*;

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
		pub const ADC:isize=2; 
		pub const ADIW:isize=3; 
		pub const AND:isize=4; 
		pub const ANDI:isize=5; 
		pub const ASR:isize=6; 
		pub const BCLR:isize=7; 
		pub const BLD:isize=8; 
		pub const BRBC:isize=9; 
		pub const BRBS:isize=10; 
		pub const BRCC:isize=11; 
		pub const BRCS:isize=12; 
		pub const BREAK:isize=13; 
		pub const BREQ:isize=14; 
		pub const BRGE:isize=15; 
		pub const BRHC:isize=16; 
		pub const BRHS:isize=17; 
		pub const BRID:isize=18; 
		pub const BRIE:isize=19; 
		pub const BRLO:isize=20; 
		pub const BRLT:isize=21; 
		pub const BRMI:isize=22; 
		pub const BRNE:isize=23; 
		pub const BRPL:isize=24; 
		pub const BRSH:isize=25; 
		pub const BRTC:isize=26; 
		pub const BRTS:isize=27; 
		pub const BRVC:isize=28; 
		pub const BRVS:isize=29; 
		pub const BSET:isize=30; 
		pub const BST:isize=31; 
		pub const CALL:isize=32; 
		pub const CBI:isize=33; 
		pub const CBR:isize=34; 
		pub const CLC:isize=35; 
		pub const CLH:isize=36; 
		pub const CLI:isize=37; 
		pub const CLN:isize=38; 
		pub const CLR:isize=39; 
		pub const CLS:isize=40; 
		pub const CLT:isize=41; 
		pub const CLV:isize=42; 
		pub const CLZ:isize=43; 
		pub const COM:isize=44; 
		pub const CP:isize=45; 
		pub const CPC:isize=46; 
		pub const CPI:isize=47; 
		pub const CPSE:isize=48; 
		pub const DEC:isize=49; 
		pub const DES:isize=50; 
		pub const EICALL:isize=51; 
		pub const EIJMP:isize=52; 
		pub const ELPM:isize=53; 
		pub const EOR:isize=54; 
		pub const FMUL:isize=55; 
		pub const FMULS:isize=56; 
		pub const FMULSU:isize=57; 
		pub const ICALL:isize=58; 
		pub const IJMP:isize=59; 
		pub const IN:isize=60; 
		pub const INC:isize=61; 
		pub const JMP:isize=62; 
		pub const LAC:isize=63; 
		pub const LAS:isize=64; 
		pub const LAT:isize=65; 
		pub const LD:isize=66; 
		pub const LDD:isize=67; 
		pub const LDI:isize=68; 
		pub const LDS:isize=69; 
		pub const LPM:isize=70; 
		pub const LSL:isize=71; 
		pub const LSR:isize=72; 
		pub const MOV:isize=73; 
		pub const MOVW:isize=74; 
		pub const MUL:isize=75; 
		pub const MULS:isize=76; 
		pub const MULSU:isize=77; 
		pub const NEG:isize=78; 
		pub const NOP:isize=79; 
		pub const OR:isize=80; 
		pub const ORI:isize=81; 
		pub const OUT:isize=82; 
		pub const POP:isize=83; 
		pub const PUSH:isize=84; 
		pub const RCALL:isize=85; 
		pub const RET:isize=86; 
		pub const RETI:isize=87; 
		pub const RJMP:isize=88; 
		pub const ROL:isize=89; 
		pub const ROR:isize=90; 
		pub const SBC:isize=91; 
		pub const SBCI:isize=92; 
		pub const SBI:isize=93; 
		pub const SBIC:isize=94; 
		pub const SBIS:isize=95; 
		pub const SBIW:isize=96; 
		pub const SBR:isize=97; 
		pub const SBRC:isize=98; 
		pub const SBRS:isize=99; 
		pub const SEC:isize=100; 
		pub const SEH:isize=101; 
		pub const SEI:isize=102; 
		pub const SEN:isize=103; 
		pub const SER:isize=104; 
		pub const SES:isize=105; 
		pub const SET:isize=106; 
		pub const SEV:isize=107; 
		pub const SEZ:isize=108; 
		pub const SLEEP:isize=109; 
		pub const SPM:isize=110; 
		pub const ST:isize=111; 
		pub const STD:isize=112; 
		pub const STS:isize=113; 
		pub const SUB:isize=114; 
		pub const SUBI:isize=115; 
		pub const SWAP:isize=116; 
		pub const TST:isize=117; 
		pub const WDR:isize=118; 
		pub const XCH:isize=119; 
		pub const ASTERISK:isize=120; 
		pub const AT:isize=121; 
		pub const BYTE:isize=122; 
		pub const BACKSLASH:isize=123; 
		pub const CLOSEING_BRACKET:isize=124; 
		pub const COLON:isize=125; 
		pub const COMMA:isize=126; 
		pub const CSEG:isize=127; 
		pub const DB:isize=128; 
		pub const DEF:isize=129; 
		pub const DEVICE:isize=130; 
		pub const DOT:isize=131; 
		pub const DSEG:isize=132; 
		pub const ELSE:isize=133; 
		pub const END_MACRO:isize=134; 
		pub const ENDIF:isize=135; 
		pub const EQUALS:isize=136; 
		pub const EQU:isize=137; 
		pub const ERROR:isize=138; 
		pub const GT:isize=139; 
		pub const HASH_TAG:isize=140; 
		pub const IF:isize=141; 
		pub const INCLUDE:isize=142; 
		pub const LEFT_SHIFT:isize=143; 
		pub const LT:isize=144; 
		pub const MACRO:isize=145; 
		pub const MINUS:isize=146; 
		pub const OPENING_BRACKET:isize=147; 
		pub const ORG:isize=148; 
		pub const PLUS:isize=149; 
		pub const RIGHT_SHIFT:isize=150; 
		pub const SLASH:isize=151; 
		pub const NEWLINE:isize=152; 
		pub const WS:isize=153; 
		pub const LINE_COMMENT:isize=154; 
		pub const BLOCK_COMMENT:isize=155; 
		pub const DOUBLE_SLASH_LINE_COMMENT:isize=156; 
		pub const STRING:isize=157; 
		pub const CHAR:isize=158; 
		pub const ESCAPED_CHAR:isize=159; 
		pub const NUMBER:isize=160; 
		pub const HEX_NUMBER:isize=161; 
		pub const BINARY_NUMBER:isize=162; 
		pub const IDENTIFIER:isize=163;
	pub const RULE_asm_file:usize = 0; 
	pub const RULE_row:usize = 1; 
	pub const RULE_instruction:usize = 2; 
	pub const RULE_param:usize = 3; 
	pub const RULE_register_pair:usize = 4; 
	pub const RULE_macro_usage:usize = 5; 
	pub const RULE_label_definition:usize = 6; 
	pub const RULE_macro_placeholder:usize = 7; 
	pub const RULE_expression:usize = 8; 
	pub const RULE_numeric:usize = 9; 
	pub const RULE_asm_intrinsic_instruction:usize = 10; 
	pub const RULE_byte_csv:usize = 11; 
	pub const RULE_asm_intrinsic_usage:usize = 12; 
	pub const RULE_preprocessor_directive:usize = 13; 
	pub const RULE_mnemonic:usize = 14; 
	pub const RULE_mnemonic_a:usize = 15; 
	pub const RULE_mnemonic_b:usize = 16; 
	pub const RULE_mnemonic_c:usize = 17; 
	pub const RULE_mnemonic_d:usize = 18; 
	pub const RULE_mnemonic_e:usize = 19; 
	pub const RULE_mnemonic_f:usize = 20; 
	pub const RULE_mnemonic_i:usize = 21; 
	pub const RULE_mnemonic_j:usize = 22; 
	pub const RULE_mnemonic_l:usize = 23; 
	pub const RULE_mnemonic_m:usize = 24; 
	pub const RULE_mnemonic_n:usize = 25; 
	pub const RULE_mnemonic_o:usize = 26; 
	pub const RULE_mnemonic_p:usize = 27; 
	pub const RULE_mnemonic_r:usize = 28; 
	pub const RULE_mnemonic_s:usize = 29; 
	pub const RULE_mnemonic_t:usize = 30; 
	pub const RULE_mnemonic_w:usize = 31; 
	pub const RULE_mnemonic_x:usize = 32;
	pub const ruleNames: [&'static str; 33] =  [
		"asm_file", "row", "instruction", "param", "register_pair", "macro_usage", 
		"label_definition", "macro_placeholder", "expression", "numeric", "asm_intrinsic_instruction", 
		"byte_csv", "asm_intrinsic_usage", "preprocessor_directive", "mnemonic", 
		"mnemonic_a", "mnemonic_b", "mnemonic_c", "mnemonic_d", "mnemonic_e", 
		"mnemonic_f", "mnemonic_i", "mnemonic_j", "mnemonic_l", "mnemonic_m", 
		"mnemonic_n", "mnemonic_o", "mnemonic_p", "mnemonic_r", "mnemonic_s", 
		"mnemonic_t", "mnemonic_w", "mnemonic_x"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;152] = [
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
		Some("'*'"), Some("'@'"), None, Some("'\\'"), Some("')'"), Some("':'"), 
		Some("','"), Some("'cseg'"), Some("'db'"), Some("'def'"), Some("'device'"), 
		Some("'.'"), Some("'dseg'"), Some("'else'"), Some("'endmacro'"), Some("'endif'"), 
		Some("'='"), None, Some("'error'"), Some("'>'"), Some("'#'"), Some("'if'"), 
		Some("'include'"), Some("'<<'"), Some("'<'"), Some("'macro'"), Some("'-'"), 
		Some("'('"), Some("'org'"), Some("'+'"), Some("'>>'"), Some("'/'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;164]  = [
		None, Some("ADD"), Some("ADC"), Some("ADIW"), Some("AND"), Some("ANDI"), 
		Some("ASR"), Some("BCLR"), Some("BLD"), Some("BRBC"), Some("BRBS"), Some("BRCC"), 
		Some("BRCS"), Some("BREAK"), Some("BREQ"), Some("BRGE"), Some("BRHC"), 
		Some("BRHS"), Some("BRID"), Some("BRIE"), Some("BRLO"), Some("BRLT"), 
		Some("BRMI"), Some("BRNE"), Some("BRPL"), Some("BRSH"), Some("BRTC"), 
		Some("BRTS"), Some("BRVC"), Some("BRVS"), Some("BSET"), Some("BST"), Some("CALL"), 
		Some("CBI"), Some("CBR"), Some("CLC"), Some("CLH"), Some("CLI"), Some("CLN"), 
		Some("CLR"), Some("CLS"), Some("CLT"), Some("CLV"), Some("CLZ"), Some("COM"), 
		Some("CP"), Some("CPC"), Some("CPI"), Some("CPSE"), Some("DEC"), Some("DES"), 
		Some("EICALL"), Some("EIJMP"), Some("ELPM"), Some("EOR"), Some("FMUL"), 
		Some("FMULS"), Some("FMULSU"), Some("ICALL"), Some("IJMP"), Some("IN"), 
		Some("INC"), Some("JMP"), Some("LAC"), Some("LAS"), Some("LAT"), Some("LD"), 
		Some("LDD"), Some("LDI"), Some("LDS"), Some("LPM"), Some("LSL"), Some("LSR"), 
		Some("MOV"), Some("MOVW"), Some("MUL"), Some("MULS"), Some("MULSU"), Some("NEG"), 
		Some("NOP"), Some("OR"), Some("ORI"), Some("OUT"), Some("POP"), Some("PUSH"), 
		Some("RCALL"), Some("RET"), Some("RETI"), Some("RJMP"), Some("ROL"), Some("ROR"), 
		Some("SBC"), Some("SBCI"), Some("SBI"), Some("SBIC"), Some("SBIS"), Some("SBIW"), 
		Some("SBR"), Some("SBRC"), Some("SBRS"), Some("SEC"), Some("SEH"), Some("SEI"), 
		Some("SEN"), Some("SER"), Some("SES"), Some("SET"), Some("SEV"), Some("SEZ"), 
		Some("SLEEP"), Some("SPM"), Some("ST"), Some("STD"), Some("STS"), Some("SUB"), 
		Some("SUBI"), Some("SWAP"), Some("TST"), Some("WDR"), Some("XCH"), Some("ASTERISK"), 
		Some("AT"), Some("BYTE"), Some("BACKSLASH"), Some("CLOSEING_BRACKET"), 
		Some("COLON"), Some("COMMA"), Some("CSEG"), Some("DB"), Some("DEF"), Some("DEVICE"), 
		Some("DOT"), Some("DSEG"), Some("ELSE"), Some("END_MACRO"), Some("ENDIF"), 
		Some("EQUALS"), Some("EQU"), Some("ERROR"), Some("GT"), Some("HASH_TAG"), 
		Some("IF"), Some("INCLUDE"), Some("LEFT_SHIFT"), Some("LT"), Some("MACRO"), 
		Some("MINUS"), Some("OPENING_BRACKET"), Some("ORG"), Some("PLUS"), Some("RIGHT_SHIFT"), 
		Some("SLASH"), Some("NEWLINE"), Some("WS"), Some("LINE_COMMENT"), Some("BLOCK_COMMENT"), 
		Some("DOUBLE_SLASH_LINE_COMMENT"), Some("STRING"), Some("CHAR"), Some("ESCAPED_CHAR"), 
		Some("NUMBER"), Some("HEX_NUMBER"), Some("BINARY_NUMBER"), Some("IDENTIFIER")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,assembler_parserExt<'input>, I, assembler_parserContextType , dyn assembler_parserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;

pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;

pub type assembler_parserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, assembler_parserContextType , dyn assembler_parserListener<'input> + 'a>;

/// Parser for assembler_parser grammar
pub struct assembler_parser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> assembler_parser<'input, I, H>
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
				assembler_parserExt{
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

impl<'input, I> assembler_parser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> assembler_parser<'input, I, DefaultErrorStrategy<'input,assembler_parserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for assembler_parser
pub trait assembler_parserContext<'input>:
	for<'x> Listenable<dyn assembler_parserListener<'input> + 'x > + 
	for<'x> Visitable<dyn assembler_parserVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=assembler_parserContextType>
{}

antlr_rust::coerce_from!{ 'input : assembler_parserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn assembler_parserContext<'input> + 'input
where
    T: assembler_parserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn assembler_parserVisitor<'input> + 'x))
    }
}

impl<'input> assembler_parserContext<'input> for TerminalNode<'input,assembler_parserContextType> {}
impl<'input> assembler_parserContext<'input> for ErrorNode<'input,assembler_parserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn assembler_parserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn assembler_parserListener<'input> + 'input }

pub struct assembler_parserContextType;
antlr_rust::tid!{assembler_parserContextType}

impl<'input> ParserNodeType<'input> for assembler_parserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn assembler_parserContext<'input> + 'input;
}

impl<'input, I, H> Deref for assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct assembler_parserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> assembler_parserExt<'input>{
}
antlr_rust::tid! { assembler_parserExt<'a> }

impl<'input> TokenAware<'input> for assembler_parserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for assembler_parserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for assembler_parserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "assembler_parser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn assembler_parserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					8 => assembler_parser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					11 => assembler_parser::<'input,I,_>::byte_csv_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> assembler_parser<'input, I, DefaultErrorStrategy<'input,assembler_parserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 17)
				}
				1=>{
					recog.precpred(None, 16)
				}
				2=>{
					recog.precpred(None, 15)
				}
				3=>{
					recog.precpred(None, 14)
				}
				4=>{
					recog.precpred(None, 13)
				}
				5=>{
					recog.precpred(None, 12)
				}
				6=>{
					recog.precpred(None, 11)
				}
				7=>{
					recog.precpred(None, 10)
				}
			_ => true
		}
	}
	fn byte_csv_sempred(_localctx: Option<&Byte_csvContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				8=>{
					recog.precpred(None, 2)
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

impl<'input> assembler_parserContext<'input> for Asm_fileContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Asm_fileContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_asm_file(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_asm_file(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Asm_fileContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_asm_file(self);
	}
}

impl<'input> CustomRuleContext<'input> for Asm_fileContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_asm_file }
	//fn type_rule_index() -> usize where Self: Sized { RULE_asm_file }
}
antlr_rust::tid!{Asm_fileContextExt<'a>}

impl<'input> Asm_fileContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Asm_fileContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Asm_fileContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Asm_fileContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Asm_fileContextExt<'input>>{

fn row_all(&self) ->  Vec<Rc<RowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn row(&self, i: usize) -> Option<Rc<RowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,assembler_parserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}

}

impl<'input> Asm_fileContextAttrs<'input> for Asm_fileContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
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
			recog.base.set_state(69);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(66);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(71);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule row*/
			recog.base.set_state(72);
			recog.row()?;

			recog.base.set_state(82);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(76);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==NEWLINE {
						{
						{
						recog.base.set_state(73);
						recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(78);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					/*InvokeRule row*/
					recog.base.set_state(79);
					recog.row()?;

					}
					} 
				}
				recog.base.set_state(84);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			}
			recog.base.set_state(88);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(85);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(90);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(91);
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

impl<'input> assembler_parserContext<'input> for RowContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for RowContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_row(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_row(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for RowContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_row(self);
	}
}

impl<'input> CustomRuleContext<'input> for RowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_row }
	//fn type_rule_index() -> usize where Self: Sized { RULE_row }
}
antlr_rust::tid!{RowContextExt<'a>}

impl<'input> RowContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RowContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RowContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<RowContextExt<'input>>{

fn asm_intrinsic_instruction(&self) -> Option<Rc<Asm_intrinsic_instructionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn macro_usage(&self) -> Option<Rc<Macro_usageContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn label_definition(&self) -> Option<Rc<Label_definitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn instruction(&self) -> Option<Rc<InstructionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn preprocessor_directive(&self) -> Option<Rc<Preprocessor_directiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RowContextAttrs<'input> for RowContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
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

			recog.base.set_state(98);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule asm_intrinsic_instruction*/
					recog.base.set_state(93);
					recog.asm_intrinsic_instruction()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule macro_usage*/
					recog.base.set_state(94);
					recog.macro_usage()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule label_definition*/
					recog.base.set_state(95);
					recog.label_definition()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule instruction*/
					recog.base.set_state(96);
					recog.instruction()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule preprocessor_directive*/
					recog.base.set_state(97);
					recog.preprocessor_directive()?;

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

impl<'input> assembler_parserContext<'input> for InstructionContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for InstructionContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_instruction(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_instruction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for InstructionContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_instruction(self);
	}
}

impl<'input> CustomRuleContext<'input> for InstructionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_instruction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_instruction }
}
antlr_rust::tid!{InstructionContextExt<'a>}

impl<'input> InstructionContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InstructionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InstructionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InstructionContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<InstructionContextExt<'input>>{

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
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> InstructionContextAttrs<'input> for InstructionContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
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
			recog.base.set_state(100);
			recog.mnemonic()?;

			recog.base.set_state(106);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule param*/
					recog.base.set_state(101);
					recog.param()?;

					recog.base.set_state(104);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==COMMA {
						{
						recog.base.set_state(102);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule param*/
						recog.base.set_state(103);
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

impl<'input> assembler_parserContext<'input> for ParamContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for ParamContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_param(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_param(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for ParamContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_param(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParamContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_param }
	//fn type_rule_index() -> usize where Self: Sized { RULE_param }
}
antlr_rust::tid!{ParamContextExt<'a>}

impl<'input> ParamContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParamContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParamContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParamContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<ParamContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
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
fn register_pair(&self) -> Option<Rc<Register_pairContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParamContextAttrs<'input> for ParamContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(119);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					{
					recog.base.set_state(109);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==MINUS {
						{
						recog.base.set_state(108);
						recog.base.match_token(MINUS,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(111);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(113);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PLUS {
						{
						recog.base.set_state(112);
						recog.base.match_token(PLUS,&mut recog.err_handler)?;

						}
					}

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(115);
					recog.expression_rec(0)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule asm_intrinsic_usage*/
					recog.base.set_state(116);
					recog.asm_intrinsic_usage()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule macro_placeholder*/
					recog.base.set_state(117);
					recog.macro_placeholder()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule register_pair*/
					recog.base.set_state(118);
					recog.register_pair()?;

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
//------------------- register_pair ----------------
pub type Register_pairContextAll<'input> = Register_pairContext<'input>;


pub type Register_pairContext<'input> = BaseParserRuleContext<'input,Register_pairContextExt<'input>>;

#[derive(Clone)]
pub struct Register_pairContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Register_pairContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Register_pairContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_register_pair(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_register_pair(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Register_pairContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_register_pair(self);
	}
}

impl<'input> CustomRuleContext<'input> for Register_pairContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_register_pair }
	//fn type_rule_index() -> usize where Self: Sized { RULE_register_pair }
}
antlr_rust::tid!{Register_pairContextExt<'a>}

impl<'input> Register_pairContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Register_pairContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Register_pairContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Register_pairContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Register_pairContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,assembler_parserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> Register_pairContextAttrs<'input> for Register_pairContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn register_pair(&mut self,)
	-> Result<Rc<Register_pairContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Register_pairContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_register_pair);
        let mut _localctx: Rc<Register_pairContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(121);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(122);
			recog.base.match_token(COLON,&mut recog.err_handler)?;

			recog.base.set_state(123);
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
//------------------- macro_usage ----------------
pub type Macro_usageContextAll<'input> = Macro_usageContext<'input>;


pub type Macro_usageContext<'input> = BaseParserRuleContext<'input,Macro_usageContextExt<'input>>;

#[derive(Clone)]
pub struct Macro_usageContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Macro_usageContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Macro_usageContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_macro_usage(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_macro_usage(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Macro_usageContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_macro_usage(self);
	}
}

impl<'input> CustomRuleContext<'input> for Macro_usageContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_macro_usage }
	//fn type_rule_index() -> usize where Self: Sized { RULE_macro_usage }
}
antlr_rust::tid!{Macro_usageContextExt<'a>}

impl<'input> Macro_usageContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Macro_usageContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Macro_usageContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Macro_usageContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Macro_usageContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
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

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn macro_usage(&mut self,)
	-> Result<Rc<Macro_usageContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Macro_usageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_macro_usage);
        let mut _localctx: Rc<Macro_usageContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(125);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(129);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule expression*/
					recog.base.set_state(126);
					recog.expression_rec(0)?;

					}
					} 
				}
				recog.base.set_state(131);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
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

impl<'input> assembler_parserContext<'input> for Label_definitionContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Label_definitionContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_label_definition(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_label_definition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Label_definitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_label_definition(self);
	}
}

impl<'input> CustomRuleContext<'input> for Label_definitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_label_definition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_label_definition }
}
antlr_rust::tid!{Label_definitionContextExt<'a>}

impl<'input> Label_definitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Label_definitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Label_definitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Label_definitionContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Label_definitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> Label_definitionContextAttrs<'input> for Label_definitionContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn label_definition(&mut self,)
	-> Result<Rc<Label_definitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Label_definitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_label_definition);
        let mut _localctx: Rc<Label_definitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(132);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(133);
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
//------------------- macro_placeholder ----------------
pub type Macro_placeholderContextAll<'input> = Macro_placeholderContext<'input>;


pub type Macro_placeholderContext<'input> = BaseParserRuleContext<'input,Macro_placeholderContextExt<'input>>;

#[derive(Clone)]
pub struct Macro_placeholderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Macro_placeholderContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Macro_placeholderContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_macro_placeholder(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_macro_placeholder(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Macro_placeholderContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_macro_placeholder(self);
	}
}

impl<'input> CustomRuleContext<'input> for Macro_placeholderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_macro_placeholder }
	//fn type_rule_index() -> usize where Self: Sized { RULE_macro_placeholder }
}
antlr_rust::tid!{Macro_placeholderContextExt<'a>}

impl<'input> Macro_placeholderContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Macro_placeholderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Macro_placeholderContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Macro_placeholderContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Macro_placeholderContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token AT
/// Returns `None` if there is no child corresponding to token AT
fn AT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(AT, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}

}

impl<'input> Macro_placeholderContextAttrs<'input> for Macro_placeholderContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
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
			recog.base.set_state(135);
			recog.base.match_token(AT,&mut recog.err_handler)?;

			recog.base.set_state(136);
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

impl<'input> assembler_parserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OPENING_BRACKET
/// Returns `None` if there is no child corresponding to token OPENING_BRACKET
fn OPENING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
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
fn CLOSEING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLOSEING_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
fn numeric(&self) -> Option<Rc<NumericContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token BINARY_NUMBER
/// Returns `None` if there is no child corresponding to token BINARY_NUMBER
fn BINARY_NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BINARY_NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX_NUMBER
/// Returns `None` if there is no child corresponding to token HEX_NUMBER
fn HEX_NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(HEX_NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token CHAR
/// Returns `None` if there is no child corresponding to token CHAR
fn CHAR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CHAR, 0)
}
/// Retrieves first TerminalNode corresponding to token ESCAPED_CHAR
/// Returns `None` if there is no child corresponding to token ESCAPED_CHAR
fn ESCAPED_CHAR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ESCAPED_CHAR, 0)
}
fn macro_placeholder(&self) -> Option<Rc<Macro_placeholderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LEFT_SHIFT
/// Returns `None` if there is no child corresponding to token LEFT_SHIFT
fn LEFT_SHIFT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LEFT_SHIFT, 0)
}
/// Retrieves first TerminalNode corresponding to token RIGHT_SHIFT
/// Returns `None` if there is no child corresponding to token RIGHT_SHIFT
fn RIGHT_SHIFT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(RIGHT_SHIFT, 0)
}
/// Retrieves first TerminalNode corresponding to token ASTERISK
/// Returns `None` if there is no child corresponding to token ASTERISK
fn ASTERISK(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ASTERISK, 0)
}
/// Retrieves first TerminalNode corresponding to token SLASH
/// Returns `None` if there is no child corresponding to token SLASH
fn SLASH(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SLASH, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
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
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(154);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 OPENING_BRACKET 
				=> {
					{
					recog.base.set_state(139);
					recog.base.match_token(OPENING_BRACKET,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(140);
					recog.expression_rec(0)?;

					recog.base.set_state(141);
					recog.base.match_token(CLOSEING_BRACKET,&mut recog.err_handler)?;

					}
				}

			 DOT 
				=> {
					{
					recog.base.set_state(143);
					recog.base.match_token(DOT,&mut recog.err_handler)?;

					recog.base.set_state(144);
					_la = recog.base.input.la(1);
					if { !(_la==MINUS || _la==PLUS) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule numeric*/
					recog.base.set_state(145);
					recog.numeric()?;

					}
				}

			 BINARY_NUMBER 
				=> {
					{
					recog.base.set_state(146);
					recog.base.match_token(BINARY_NUMBER,&mut recog.err_handler)?;

					}
				}

			 HEX_NUMBER 
				=> {
					{
					recog.base.set_state(147);
					recog.base.match_token(HEX_NUMBER,&mut recog.err_handler)?;

					}
				}

			 NUMBER 
				=> {
					{
					recog.base.set_state(148);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}

			 IDENTIFIER 
				=> {
					{
					recog.base.set_state(149);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 STRING 
				=> {
					{
					recog.base.set_state(150);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 CHAR 
				=> {
					{
					recog.base.set_state(151);
					recog.base.match_token(CHAR,&mut recog.err_handler)?;

					}
				}

			 ESCAPED_CHAR 
				=> {
					{
					recog.base.set_state(152);
					recog.base.match_token(ESCAPED_CHAR,&mut recog.err_handler)?;

					}
				}

			 AT 
				=> {
					{
					/*InvokeRule macro_placeholder*/
					recog.base.set_state(153);
					recog.macro_placeholder()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(182);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(180);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(156);
							if !({recog.precpred(None, 17)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 17)".to_owned()), None))?;
							}
							recog.base.set_state(157);
							recog.base.match_token(LEFT_SHIFT,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(158);
							recog.expression_rec(18)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(159);
							if !({recog.precpred(None, 16)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 16)".to_owned()), None))?;
							}
							recog.base.set_state(160);
							recog.base.match_token(RIGHT_SHIFT,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(161);
							recog.expression_rec(17)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(162);
							if !({recog.precpred(None, 15)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 15)".to_owned()), None))?;
							}
							recog.base.set_state(163);
							recog.base.match_token(ASTERISK,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(164);
							recog.expression_rec(16)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(165);
							if !({recog.precpred(None, 14)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 14)".to_owned()), None))?;
							}
							recog.base.set_state(166);
							recog.base.match_token(SLASH,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(167);
							recog.expression_rec(15)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(168);
							if !({recog.precpred(None, 13)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 13)".to_owned()), None))?;
							}
							recog.base.set_state(169);
							recog.base.match_token(GT,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(170);
							recog.expression_rec(14)?;

							}
						}
					,
						6 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(171);
							if !({recog.precpred(None, 12)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 12)".to_owned()), None))?;
							}
							recog.base.set_state(172);
							recog.base.match_token(LT,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(173);
							recog.expression_rec(13)?;

							}
						}
					,
						7 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(174);
							if !({recog.precpred(None, 11)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 11)".to_owned()), None))?;
							}
							recog.base.set_state(175);
							recog.base.match_token(PLUS,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(176);
							recog.expression_rec(12)?;

							}
						}
					,
						8 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(177);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(178);
							recog.base.match_token(MINUS,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(179);
							recog.expression_rec(11)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(184);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
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
//------------------- numeric ----------------
pub type NumericContextAll<'input> = NumericContext<'input>;


pub type NumericContext<'input> = BaseParserRuleContext<'input,NumericContextExt<'input>>;

#[derive(Clone)]
pub struct NumericContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for NumericContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for NumericContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_numeric(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_numeric(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for NumericContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_numeric(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumericContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_numeric }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numeric }
}
antlr_rust::tid!{NumericContextExt<'a>}

impl<'input> NumericContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumericContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumericContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumericContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<NumericContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BINARY_NUMBER
/// Returns `None` if there is no child corresponding to token BINARY_NUMBER
fn BINARY_NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BINARY_NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX_NUMBER
/// Returns `None` if there is no child corresponding to token HEX_NUMBER
fn HEX_NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(HEX_NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}

}

impl<'input> NumericContextAttrs<'input> for NumericContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numeric(&mut self,)
	-> Result<Rc<NumericContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumericContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_numeric);
        let mut _localctx: Rc<NumericContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(185);
			_la = recog.base.input.la(1);
			if { !(((((_la - 160)) & !0x3f) == 0 && ((1usize << (_la - 160)) & ((1usize << (NUMBER - 160)) | (1usize << (HEX_NUMBER - 160)) | (1usize << (BINARY_NUMBER - 160)))) != 0)) } {
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
//------------------- asm_intrinsic_instruction ----------------
pub type Asm_intrinsic_instructionContextAll<'input> = Asm_intrinsic_instructionContext<'input>;


pub type Asm_intrinsic_instructionContext<'input> = BaseParserRuleContext<'input,Asm_intrinsic_instructionContextExt<'input>>;

#[derive(Clone)]
pub struct Asm_intrinsic_instructionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Asm_intrinsic_instructionContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Asm_intrinsic_instructionContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_asm_intrinsic_instruction(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_asm_intrinsic_instruction(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Asm_intrinsic_instructionContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_asm_intrinsic_instruction(self);
	}
}

impl<'input> CustomRuleContext<'input> for Asm_intrinsic_instructionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_asm_intrinsic_instruction }
	//fn type_rule_index() -> usize where Self: Sized { RULE_asm_intrinsic_instruction }
}
antlr_rust::tid!{Asm_intrinsic_instructionContextExt<'a>}

impl<'input> Asm_intrinsic_instructionContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Asm_intrinsic_instructionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Asm_intrinsic_instructionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Asm_intrinsic_instructionContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Asm_intrinsic_instructionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DOT
/// Returns `None` if there is no child corresponding to token DOT
fn DOT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(DOT, 0)
}
/// Retrieves first TerminalNode corresponding to token INCLUDE
/// Returns `None` if there is no child corresponding to token INCLUDE
fn INCLUDE(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(INCLUDE, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
/// Retrieves first TerminalNode corresponding to token DEVICE
/// Returns `None` if there is no child corresponding to token DEVICE
fn DEVICE(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(DEVICE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token DEF
/// Returns `None` if there is no child corresponding to token DEF
fn DEF(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(DEF, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQU
/// Returns `None` if there is no child corresponding to token EQU
fn EQU(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(EQU, 0)
}
/// Retrieves first TerminalNode corresponding to token CSEG
/// Returns `None` if there is no child corresponding to token CSEG
fn CSEG(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CSEG, 0)
}
/// Retrieves first TerminalNode corresponding to token DSEG
/// Returns `None` if there is no child corresponding to token DSEG
fn DSEG(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(DSEG, 0)
}
/// Retrieves first TerminalNode corresponding to token DB
/// Returns `None` if there is no child corresponding to token DB
fn DB(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(DB, 0)
}
fn byte_csv(&self) -> Option<Rc<Byte_csvContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BYTE
/// Returns `None` if there is no child corresponding to token BYTE
fn BYTE(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BYTE, 0)
}
/// Retrieves first TerminalNode corresponding to token ORG
/// Returns `None` if there is no child corresponding to token ORG
fn ORG(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ORG, 0)
}
/// Retrieves first TerminalNode corresponding to token MACRO
/// Returns `None` if there is no child corresponding to token MACRO
fn MACRO(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(MACRO, 0)
}
/// Retrieves first TerminalNode corresponding to token END_MACRO
/// Returns `None` if there is no child corresponding to token END_MACRO
fn END_MACRO(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(END_MACRO, 0)
}
/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ELSE, 0)
}
/// Retrieves first TerminalNode corresponding to token ENDIF
/// Returns `None` if there is no child corresponding to token ENDIF
fn ENDIF(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ENDIF, 0)
}
/// Retrieves first TerminalNode corresponding to token ERROR
/// Returns `None` if there is no child corresponding to token ERROR
fn ERROR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ERROR, 0)
}
/// Retrieves first TerminalNode corresponding to token HEX_NUMBER
/// Returns `None` if there is no child corresponding to token HEX_NUMBER
fn HEX_NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(HEX_NUMBER, 0)
}

}

impl<'input> Asm_intrinsic_instructionContextAttrs<'input> for Asm_intrinsic_instructionContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn asm_intrinsic_instruction(&mut self,)
	-> Result<Rc<Asm_intrinsic_instructionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Asm_intrinsic_instructionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_asm_intrinsic_instruction);
        let mut _localctx: Rc<Asm_intrinsic_instructionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(187);
			recog.base.match_token(DOT,&mut recog.err_handler)?;

			recog.base.set_state(217);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INCLUDE 
				=> {
					{
					recog.base.set_state(188);
					recog.base.match_token(INCLUDE,&mut recog.err_handler)?;

					recog.base.set_state(189);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}

			 DEVICE 
				=> {
					{
					recog.base.set_state(190);
					recog.base.match_token(DEVICE,&mut recog.err_handler)?;

					recog.base.set_state(191);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 DEF 
				=> {
					{
					recog.base.set_state(192);
					recog.base.match_token(DEF,&mut recog.err_handler)?;

					recog.base.set_state(193);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(194);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(195);
					recog.expression_rec(0)?;

					}
				}

			 EQU 
				=> {
					{
					recog.base.set_state(196);
					recog.base.match_token(EQU,&mut recog.err_handler)?;

					recog.base.set_state(197);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(198);
					recog.base.match_token(EQUALS,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(199);
					recog.expression_rec(0)?;

					}
				}

			 CSEG 
				=> {
					{
					recog.base.set_state(200);
					recog.base.match_token(CSEG,&mut recog.err_handler)?;

					}
				}

			 DSEG 
				=> {
					{
					recog.base.set_state(201);
					recog.base.match_token(DSEG,&mut recog.err_handler)?;

					}
				}

			 DB 
				=> {
					{
					recog.base.set_state(202);
					recog.base.match_token(DB,&mut recog.err_handler)?;

					/*InvokeRule byte_csv*/
					recog.base.set_state(203);
					recog.byte_csv_rec(0)?;

					}
				}

			 BYTE 
				=> {
					{
					recog.base.set_state(204);
					recog.base.match_token(BYTE,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(205);
					recog.expression_rec(0)?;

					}
				}

			 ORG 
				=> {
					{
					recog.base.set_state(206);
					recog.base.match_token(ORG,&mut recog.err_handler)?;

					recog.base.set_state(207);
					_la = recog.base.input.la(1);
					if { !(_la==HEX_NUMBER || _la==IDENTIFIER) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
				}

			 MACRO 
				=> {
					{
					recog.base.set_state(208);
					recog.base.match_token(MACRO,&mut recog.err_handler)?;

					recog.base.set_state(209);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 END_MACRO 
				=> {
					{
					recog.base.set_state(210);
					recog.base.match_token(END_MACRO,&mut recog.err_handler)?;

					}
				}

			 IF 
				=> {
					{
					recog.base.set_state(211);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(212);
					recog.expression_rec(0)?;

					}
				}

			 ELSE 
				=> {
					{
					recog.base.set_state(213);
					recog.base.match_token(ELSE,&mut recog.err_handler)?;

					}
				}

			 ENDIF 
				=> {
					{
					recog.base.set_state(214);
					recog.base.match_token(ENDIF,&mut recog.err_handler)?;

					}
				}

			 ERROR 
				=> {
					{
					recog.base.set_state(215);
					recog.base.match_token(ERROR,&mut recog.err_handler)?;

					recog.base.set_state(216);
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
//------------------- byte_csv ----------------
pub type Byte_csvContextAll<'input> = Byte_csvContext<'input>;


pub type Byte_csvContext<'input> = BaseParserRuleContext<'input,Byte_csvContextExt<'input>>;

#[derive(Clone)]
pub struct Byte_csvContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Byte_csvContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Byte_csvContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_byte_csv(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_byte_csv(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Byte_csvContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_byte_csv(self);
	}
}

impl<'input> CustomRuleContext<'input> for Byte_csvContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_byte_csv }
	//fn type_rule_index() -> usize where Self: Sized { RULE_byte_csv }
}
antlr_rust::tid!{Byte_csvContextExt<'a>}

impl<'input> Byte_csvContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Byte_csvContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Byte_csvContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Byte_csvContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Byte_csvContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HEX_NUMBER
/// Returns `None` if there is no child corresponding to token HEX_NUMBER
fn HEX_NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(HEX_NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token NUMBER
/// Returns `None` if there is no child corresponding to token NUMBER
fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(NUMBER, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn byte_csv(&self) -> Option<Rc<Byte_csvContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}

}

impl<'input> Byte_csvContextAttrs<'input> for Byte_csvContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  byte_csv(&mut self,)
	-> Result<Rc<Byte_csvContextAll<'input>>,ANTLRError> {
		self.byte_csv_rec(0)
	}

	fn byte_csv_rec(&mut self, _p: isize)
	-> Result<Rc<Byte_csvContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Byte_csvContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 22, RULE_byte_csv, _p);
	    let mut _localctx: Rc<Byte_csvContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 22;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			recog.base.set_state(220);
			_la = recog.base.input.la(1);
			if { !(((((_la - 157)) & !0x3f) == 0 && ((1usize << (_la - 157)) & ((1usize << (STRING - 157)) | (1usize << (NUMBER - 157)) | (1usize << (HEX_NUMBER - 157)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(227);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Byte_csvContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_byte_csv);
					_localctx = tmp;
					recog.base.set_state(222);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					recog.base.set_state(223);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					recog.base.set_state(224);
					_la = recog.base.input.la(1);
					if { !(((((_la - 157)) & !0x3f) == 0 && ((1usize << (_la - 157)) & ((1usize << (STRING - 157)) | (1usize << (NUMBER - 157)) | (1usize << (HEX_NUMBER - 157)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
					} 
				}
				recog.base.set_state(229);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
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
//------------------- asm_intrinsic_usage ----------------
pub type Asm_intrinsic_usageContextAll<'input> = Asm_intrinsic_usageContext<'input>;


pub type Asm_intrinsic_usageContext<'input> = BaseParserRuleContext<'input,Asm_intrinsic_usageContextExt<'input>>;

#[derive(Clone)]
pub struct Asm_intrinsic_usageContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Asm_intrinsic_usageContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Asm_intrinsic_usageContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_asm_intrinsic_usage(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_asm_intrinsic_usage(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Asm_intrinsic_usageContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_asm_intrinsic_usage(self);
	}
}

impl<'input> CustomRuleContext<'input> for Asm_intrinsic_usageContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_asm_intrinsic_usage }
	//fn type_rule_index() -> usize where Self: Sized { RULE_asm_intrinsic_usage }
}
antlr_rust::tid!{Asm_intrinsic_usageContextExt<'a>}

impl<'input> Asm_intrinsic_usageContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Asm_intrinsic_usageContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Asm_intrinsic_usageContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Asm_intrinsic_usageContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Asm_intrinsic_usageContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token OPENING_BRACKET
/// Returns `None` if there is no child corresponding to token OPENING_BRACKET
fn OPENING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(OPENING_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOSEING_BRACKET
/// Returns `None` if there is no child corresponding to token CLOSEING_BRACKET
fn CLOSEING_BRACKET(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLOSEING_BRACKET, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn macro_placeholder(&self) -> Option<Rc<Macro_placeholderContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Asm_intrinsic_usageContextAttrs<'input> for Asm_intrinsic_usageContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn asm_intrinsic_usage(&mut self,)
	-> Result<Rc<Asm_intrinsic_usageContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Asm_intrinsic_usageContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_asm_intrinsic_usage);
        let mut _localctx: Rc<Asm_intrinsic_usageContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(230);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(231);
			recog.base.match_token(OPENING_BRACKET,&mut recog.err_handler)?;

			recog.base.set_state(234);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(16,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule expression*/
					recog.base.set_state(232);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule macro_placeholder*/
					recog.base.set_state(233);
					recog.macro_placeholder()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(236);
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
//------------------- preprocessor_directive ----------------
pub type Preprocessor_directiveContextAll<'input> = Preprocessor_directiveContext<'input>;


pub type Preprocessor_directiveContext<'input> = BaseParserRuleContext<'input,Preprocessor_directiveContextExt<'input>>;

#[derive(Clone)]
pub struct Preprocessor_directiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Preprocessor_directiveContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Preprocessor_directiveContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_preprocessor_directive(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_preprocessor_directive(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Preprocessor_directiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_preprocessor_directive(self);
	}
}

impl<'input> CustomRuleContext<'input> for Preprocessor_directiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_preprocessor_directive }
	//fn type_rule_index() -> usize where Self: Sized { RULE_preprocessor_directive }
}
antlr_rust::tid!{Preprocessor_directiveContextExt<'a>}

impl<'input> Preprocessor_directiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Preprocessor_directiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Preprocessor_directiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Preprocessor_directiveContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Preprocessor_directiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HASH_TAG
/// Returns `None` if there is no child corresponding to token HASH_TAG
fn HASH_TAG(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(HASH_TAG, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token IF in current rule
fn IF_all(&self) -> Vec<Rc<TerminalNode<'input,assembler_parserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IF, starting from 0.
/// Returns `None` if number of children corresponding to token IF is less or equal than `i`.
fn IF(&self, i: usize) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IF, i)
}
/// Retrieves all `TerminalNode`s corresponding to token ENDIF in current rule
fn ENDIF_all(&self) -> Vec<Rc<TerminalNode<'input,assembler_parserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ENDIF, starting from 0.
/// Returns `None` if number of children corresponding to token ENDIF is less or equal than `i`.
fn ENDIF(&self, i: usize) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ENDIF, i)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENTIFIER in current rule
fn IDENTIFIER_all(&self) -> Vec<Rc<TerminalNode<'input,assembler_parserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENTIFIER, starting from 0.
/// Returns `None` if number of children corresponding to token IDENTIFIER is less or equal than `i`.
fn IDENTIFIER(&self, i: usize) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, i)
}
/// Retrieves all `TerminalNode`s corresponding to token HEX_NUMBER in current rule
fn HEX_NUMBER_all(&self) -> Vec<Rc<TerminalNode<'input,assembler_parserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token HEX_NUMBER, starting from 0.
/// Returns `None` if number of children corresponding to token HEX_NUMBER is less or equal than `i`.
fn HEX_NUMBER(&self, i: usize) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(HEX_NUMBER, i)
}
/// Retrieves all `TerminalNode`s corresponding to token BINARY_NUMBER in current rule
fn BINARY_NUMBER_all(&self) -> Vec<Rc<TerminalNode<'input,assembler_parserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BINARY_NUMBER, starting from 0.
/// Returns `None` if number of children corresponding to token BINARY_NUMBER is less or equal than `i`.
fn BINARY_NUMBER(&self, i: usize) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BINARY_NUMBER, i)
}
/// Retrieves all `TerminalNode`s corresponding to token NUMBER in current rule
fn NUMBER_all(&self) -> Vec<Rc<TerminalNode<'input,assembler_parserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NUMBER, starting from 0.
/// Returns `None` if number of children corresponding to token NUMBER is less or equal than `i`.
fn NUMBER(&self, i: usize) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(NUMBER, i)
}

}

impl<'input> Preprocessor_directiveContextAttrs<'input> for Preprocessor_directiveContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn preprocessor_directive(&mut self,)
	-> Result<Rc<Preprocessor_directiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Preprocessor_directiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_preprocessor_directive);
        let mut _localctx: Rc<Preprocessor_directiveContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(238);
			recog.base.match_token(HASH_TAG,&mut recog.err_handler)?;

			recog.base.set_state(240); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(239);
					_la = recog.base.input.la(1);
					if { !(((((_la - 135)) & !0x3f) == 0 && ((1usize << (_la - 135)) & ((1usize << (ENDIF - 135)) | (1usize << (IF - 135)) | (1usize << (NUMBER - 135)) | (1usize << (HEX_NUMBER - 135)) | (1usize << (BINARY_NUMBER - 135)) | (1usize << (IDENTIFIER - 135)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(242); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
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
//------------------- mnemonic ----------------
pub type MnemonicContextAll<'input> = MnemonicContext<'input>;


pub type MnemonicContext<'input> = BaseParserRuleContext<'input,MnemonicContextExt<'input>>;

#[derive(Clone)]
pub struct MnemonicContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for MnemonicContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for MnemonicContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for MnemonicContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic(self);
	}
}

impl<'input> CustomRuleContext<'input> for MnemonicContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic }
}
antlr_rust::tid!{MnemonicContextExt<'a>}

impl<'input> MnemonicContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MnemonicContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MnemonicContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MnemonicContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<MnemonicContextExt<'input>>{

fn mnemonic_a(&self) -> Option<Rc<Mnemonic_aContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_b(&self) -> Option<Rc<Mnemonic_bContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_c(&self) -> Option<Rc<Mnemonic_cContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_d(&self) -> Option<Rc<Mnemonic_dContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_e(&self) -> Option<Rc<Mnemonic_eContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_f(&self) -> Option<Rc<Mnemonic_fContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_i(&self) -> Option<Rc<Mnemonic_iContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_j(&self) -> Option<Rc<Mnemonic_jContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_l(&self) -> Option<Rc<Mnemonic_lContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_m(&self) -> Option<Rc<Mnemonic_mContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_n(&self) -> Option<Rc<Mnemonic_nContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_o(&self) -> Option<Rc<Mnemonic_oContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_p(&self) -> Option<Rc<Mnemonic_pContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_r(&self) -> Option<Rc<Mnemonic_rContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_s(&self) -> Option<Rc<Mnemonic_sContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_t(&self) -> Option<Rc<Mnemonic_tContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_w(&self) -> Option<Rc<Mnemonic_wContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mnemonic_x(&self) -> Option<Rc<Mnemonic_xContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MnemonicContextAttrs<'input> for MnemonicContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic(&mut self,)
	-> Result<Rc<MnemonicContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MnemonicContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_mnemonic);
        let mut _localctx: Rc<MnemonicContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(262);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ADD | ADC | ADIW | AND | ANDI | ASR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule mnemonic_a*/
					recog.base.set_state(244);
					recog.mnemonic_a()?;

					}
				}

			 BCLR | BLD | BRBC | BRBS | BRCC | BRCS | BREAK | BREQ | BRGE | BRHC |
			 BRHS | BRID | BRIE | BRLO | BRLT | BRMI | BRNE | BRPL | BRSH | BRTC |
			 BRTS | BRVC | BRVS | BSET | BST 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule mnemonic_b*/
					recog.base.set_state(245);
					recog.mnemonic_b()?;

					}
				}

			 CALL | CBI | CBR | CLC | CLH | CLI | CLN | CLR | CLS | CLT | CLV | CLZ |
			 COM | CP | CPC | CPI | CPSE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule mnemonic_c*/
					recog.base.set_state(246);
					recog.mnemonic_c()?;

					}
				}

			 DEC | DES 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule mnemonic_d*/
					recog.base.set_state(247);
					recog.mnemonic_d()?;

					}
				}

			 EICALL | EIJMP | ELPM | EOR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule mnemonic_e*/
					recog.base.set_state(248);
					recog.mnemonic_e()?;

					}
				}

			 FMUL | FMULS | FMULSU 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule mnemonic_f*/
					recog.base.set_state(249);
					recog.mnemonic_f()?;

					}
				}

			 ICALL | IJMP | IN | INC 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule mnemonic_i*/
					recog.base.set_state(250);
					recog.mnemonic_i()?;

					}
				}

			 JMP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule mnemonic_j*/
					recog.base.set_state(251);
					recog.mnemonic_j()?;

					}
				}

			 LAC | LAS | LAT | LD | LDD | LDI | LDS | LPM | LSL | LSR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule mnemonic_l*/
					recog.base.set_state(252);
					recog.mnemonic_l()?;

					}
				}

			 MOV | MOVW | MUL | MULS | MULSU 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule mnemonic_m*/
					recog.base.set_state(253);
					recog.mnemonic_m()?;

					}
				}

			 NEG | NOP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					/*InvokeRule mnemonic_n*/
					recog.base.set_state(254);
					recog.mnemonic_n()?;

					}
				}

			 OR | ORI | OUT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					/*InvokeRule mnemonic_o*/
					recog.base.set_state(255);
					recog.mnemonic_o()?;

					}
				}

			 POP | PUSH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					/*InvokeRule mnemonic_p*/
					recog.base.set_state(256);
					recog.mnemonic_p()?;

					}
				}

			 RCALL | RET | RETI | RJMP | ROL | ROR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 14);
					recog.base.enter_outer_alt(None, 14);
					{
					/*InvokeRule mnemonic_r*/
					recog.base.set_state(257);
					recog.mnemonic_r()?;

					}
				}

			 SBC | SBCI | SBI | SBIC | SBIS | SBIW | SBR | SBRC | SBRS | SEC | SEH |
			 SEI | SEN | SER | SES | SET | SEV | SEZ | SLEEP | SPM | ST | STD | STS |
			 SUB | SUBI | SWAP 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 15);
					recog.base.enter_outer_alt(None, 15);
					{
					/*InvokeRule mnemonic_s*/
					recog.base.set_state(258);
					recog.mnemonic_s()?;

					}
				}

			 TST 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 16);
					recog.base.enter_outer_alt(None, 16);
					{
					/*InvokeRule mnemonic_t*/
					recog.base.set_state(259);
					recog.mnemonic_t()?;

					}
				}

			 WDR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 17);
					recog.base.enter_outer_alt(None, 17);
					{
					/*InvokeRule mnemonic_w*/
					recog.base.set_state(260);
					recog.mnemonic_w()?;

					}
				}

			 XCH 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 18);
					recog.base.enter_outer_alt(None, 18);
					{
					/*InvokeRule mnemonic_x*/
					recog.base.set_state(261);
					recog.mnemonic_x()?;

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
//------------------- mnemonic_a ----------------
pub type Mnemonic_aContextAll<'input> = Mnemonic_aContext<'input>;


pub type Mnemonic_aContext<'input> = BaseParserRuleContext<'input,Mnemonic_aContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_aContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_aContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_aContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_a(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_a(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_aContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_a(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_aContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_a }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_a }
}
antlr_rust::tid!{Mnemonic_aContextExt<'a>}

impl<'input> Mnemonic_aContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_aContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_aContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_aContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_aContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token ADC
/// Returns `None` if there is no child corresponding to token ADC
fn ADC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ADC, 0)
}
/// Retrieves first TerminalNode corresponding to token ADIW
/// Returns `None` if there is no child corresponding to token ADIW
fn ADIW(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ADIW, 0)
}
/// Retrieves first TerminalNode corresponding to token AND
/// Returns `None` if there is no child corresponding to token AND
fn AND(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(AND, 0)
}
/// Retrieves first TerminalNode corresponding to token ANDI
/// Returns `None` if there is no child corresponding to token ANDI
fn ANDI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ANDI, 0)
}
/// Retrieves first TerminalNode corresponding to token ASR
/// Returns `None` if there is no child corresponding to token ASR
fn ASR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ASR, 0)
}

}

impl<'input> Mnemonic_aContextAttrs<'input> for Mnemonic_aContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_a(&mut self,)
	-> Result<Rc<Mnemonic_aContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_aContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_mnemonic_a);
        let mut _localctx: Rc<Mnemonic_aContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(264);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << ADD) | (1usize << ADC) | (1usize << ADIW) | (1usize << AND) | (1usize << ANDI) | (1usize << ASR))) != 0)) } {
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
//------------------- mnemonic_b ----------------
pub type Mnemonic_bContextAll<'input> = Mnemonic_bContext<'input>;


pub type Mnemonic_bContext<'input> = BaseParserRuleContext<'input,Mnemonic_bContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_bContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_bContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_bContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_b(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_b(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_bContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_b(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_bContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_b }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_b }
}
antlr_rust::tid!{Mnemonic_bContextExt<'a>}

impl<'input> Mnemonic_bContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_bContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_bContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_bContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_bContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BCLR
/// Returns `None` if there is no child corresponding to token BCLR
fn BCLR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BCLR, 0)
}
/// Retrieves first TerminalNode corresponding to token BLD
/// Returns `None` if there is no child corresponding to token BLD
fn BLD(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BLD, 0)
}
/// Retrieves first TerminalNode corresponding to token BRBC
/// Returns `None` if there is no child corresponding to token BRBC
fn BRBC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRBC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRBS
/// Returns `None` if there is no child corresponding to token BRBS
fn BRBS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRBS, 0)
}
/// Retrieves first TerminalNode corresponding to token BRCC
/// Returns `None` if there is no child corresponding to token BRCC
fn BRCC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRCC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRCS
/// Returns `None` if there is no child corresponding to token BRCS
fn BRCS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRCS, 0)
}
/// Retrieves first TerminalNode corresponding to token BREAK
/// Returns `None` if there is no child corresponding to token BREAK
fn BREAK(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BREAK, 0)
}
/// Retrieves first TerminalNode corresponding to token BREQ
/// Returns `None` if there is no child corresponding to token BREQ
fn BREQ(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BREQ, 0)
}
/// Retrieves first TerminalNode corresponding to token BRGE
/// Returns `None` if there is no child corresponding to token BRGE
fn BRGE(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRGE, 0)
}
/// Retrieves first TerminalNode corresponding to token BRHC
/// Returns `None` if there is no child corresponding to token BRHC
fn BRHC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRHC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRHS
/// Returns `None` if there is no child corresponding to token BRHS
fn BRHS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRHS, 0)
}
/// Retrieves first TerminalNode corresponding to token BRID
/// Returns `None` if there is no child corresponding to token BRID
fn BRID(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRID, 0)
}
/// Retrieves first TerminalNode corresponding to token BRIE
/// Returns `None` if there is no child corresponding to token BRIE
fn BRIE(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRIE, 0)
}
/// Retrieves first TerminalNode corresponding to token BRLO
/// Returns `None` if there is no child corresponding to token BRLO
fn BRLO(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRLO, 0)
}
/// Retrieves first TerminalNode corresponding to token BRLT
/// Returns `None` if there is no child corresponding to token BRLT
fn BRLT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRLT, 0)
}
/// Retrieves first TerminalNode corresponding to token BRMI
/// Returns `None` if there is no child corresponding to token BRMI
fn BRMI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRMI, 0)
}
/// Retrieves first TerminalNode corresponding to token BRNE
/// Returns `None` if there is no child corresponding to token BRNE
fn BRNE(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRNE, 0)
}
/// Retrieves first TerminalNode corresponding to token BRPL
/// Returns `None` if there is no child corresponding to token BRPL
fn BRPL(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRPL, 0)
}
/// Retrieves first TerminalNode corresponding to token BRSH
/// Returns `None` if there is no child corresponding to token BRSH
fn BRSH(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRSH, 0)
}
/// Retrieves first TerminalNode corresponding to token BRTC
/// Returns `None` if there is no child corresponding to token BRTC
fn BRTC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRTC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRTS
/// Returns `None` if there is no child corresponding to token BRTS
fn BRTS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRTS, 0)
}
/// Retrieves first TerminalNode corresponding to token BRVC
/// Returns `None` if there is no child corresponding to token BRVC
fn BRVC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRVC, 0)
}
/// Retrieves first TerminalNode corresponding to token BRVS
/// Returns `None` if there is no child corresponding to token BRVS
fn BRVS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BRVS, 0)
}
/// Retrieves first TerminalNode corresponding to token BSET
/// Returns `None` if there is no child corresponding to token BSET
fn BSET(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BSET, 0)
}
/// Retrieves first TerminalNode corresponding to token BST
/// Returns `None` if there is no child corresponding to token BST
fn BST(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(BST, 0)
}

}

impl<'input> Mnemonic_bContextAttrs<'input> for Mnemonic_bContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_b(&mut self,)
	-> Result<Rc<Mnemonic_bContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_bContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_mnemonic_b);
        let mut _localctx: Rc<Mnemonic_bContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(266);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << BCLR) | (1usize << BLD) | (1usize << BRBC) | (1usize << BRBS) | (1usize << BRCC) | (1usize << BRCS) | (1usize << BREAK) | (1usize << BREQ) | (1usize << BRGE) | (1usize << BRHC) | (1usize << BRHS) | (1usize << BRID) | (1usize << BRIE) | (1usize << BRLO) | (1usize << BRLT) | (1usize << BRMI) | (1usize << BRNE) | (1usize << BRPL) | (1usize << BRSH) | (1usize << BRTC) | (1usize << BRTS) | (1usize << BRVC) | (1usize << BRVS) | (1usize << BSET) | (1usize << BST))) != 0)) } {
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
//------------------- mnemonic_c ----------------
pub type Mnemonic_cContextAll<'input> = Mnemonic_cContext<'input>;


pub type Mnemonic_cContext<'input> = BaseParserRuleContext<'input,Mnemonic_cContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_cContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_cContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_cContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_c(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_c(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_cContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_c(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_cContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_c }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_c }
}
antlr_rust::tid!{Mnemonic_cContextExt<'a>}

impl<'input> Mnemonic_cContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_cContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_cContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_cContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_cContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CALL
/// Returns `None` if there is no child corresponding to token CALL
fn CALL(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CALL, 0)
}
/// Retrieves first TerminalNode corresponding to token CBI
/// Returns `None` if there is no child corresponding to token CBI
fn CBI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CBI, 0)
}
/// Retrieves first TerminalNode corresponding to token CBR
/// Returns `None` if there is no child corresponding to token CBR
fn CBR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CBR, 0)
}
/// Retrieves first TerminalNode corresponding to token CLC
/// Returns `None` if there is no child corresponding to token CLC
fn CLC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLC, 0)
}
/// Retrieves first TerminalNode corresponding to token CLH
/// Returns `None` if there is no child corresponding to token CLH
fn CLH(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLH, 0)
}
/// Retrieves first TerminalNode corresponding to token CLI
/// Returns `None` if there is no child corresponding to token CLI
fn CLI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLI, 0)
}
/// Retrieves first TerminalNode corresponding to token CLN
/// Returns `None` if there is no child corresponding to token CLN
fn CLN(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLN, 0)
}
/// Retrieves first TerminalNode corresponding to token CLR
/// Returns `None` if there is no child corresponding to token CLR
fn CLR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLR, 0)
}
/// Retrieves first TerminalNode corresponding to token CLS
/// Returns `None` if there is no child corresponding to token CLS
fn CLS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLS, 0)
}
/// Retrieves first TerminalNode corresponding to token CLT
/// Returns `None` if there is no child corresponding to token CLT
fn CLT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLT, 0)
}
/// Retrieves first TerminalNode corresponding to token CLV
/// Returns `None` if there is no child corresponding to token CLV
fn CLV(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLV, 0)
}
/// Retrieves first TerminalNode corresponding to token CLZ
/// Returns `None` if there is no child corresponding to token CLZ
fn CLZ(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CLZ, 0)
}
/// Retrieves first TerminalNode corresponding to token COM
/// Returns `None` if there is no child corresponding to token COM
fn COM(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(COM, 0)
}
/// Retrieves first TerminalNode corresponding to token CP
/// Returns `None` if there is no child corresponding to token CP
fn CP(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CP, 0)
}
/// Retrieves first TerminalNode corresponding to token CPC
/// Returns `None` if there is no child corresponding to token CPC
fn CPC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CPC, 0)
}
/// Retrieves first TerminalNode corresponding to token CPI
/// Returns `None` if there is no child corresponding to token CPI
fn CPI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CPI, 0)
}
/// Retrieves first TerminalNode corresponding to token CPSE
/// Returns `None` if there is no child corresponding to token CPSE
fn CPSE(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(CPSE, 0)
}

}

impl<'input> Mnemonic_cContextAttrs<'input> for Mnemonic_cContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_c(&mut self,)
	-> Result<Rc<Mnemonic_cContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_cContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_mnemonic_c);
        let mut _localctx: Rc<Mnemonic_cContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(268);
			_la = recog.base.input.la(1);
			if { !(((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & ((1usize << (CALL - 32)) | (1usize << (CBI - 32)) | (1usize << (CBR - 32)) | (1usize << (CLC - 32)) | (1usize << (CLH - 32)) | (1usize << (CLI - 32)) | (1usize << (CLN - 32)) | (1usize << (CLR - 32)) | (1usize << (CLS - 32)) | (1usize << (CLT - 32)) | (1usize << (CLV - 32)) | (1usize << (CLZ - 32)) | (1usize << (COM - 32)) | (1usize << (CP - 32)) | (1usize << (CPC - 32)) | (1usize << (CPI - 32)) | (1usize << (CPSE - 32)))) != 0)) } {
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
//------------------- mnemonic_d ----------------
pub type Mnemonic_dContextAll<'input> = Mnemonic_dContext<'input>;


pub type Mnemonic_dContext<'input> = BaseParserRuleContext<'input,Mnemonic_dContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_dContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_dContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_dContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_d(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_d(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_dContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_d(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_dContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_d }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_d }
}
antlr_rust::tid!{Mnemonic_dContextExt<'a>}

impl<'input> Mnemonic_dContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_dContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_dContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_dContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_dContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DEC
/// Returns `None` if there is no child corresponding to token DEC
fn DEC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(DEC, 0)
}
/// Retrieves first TerminalNode corresponding to token DES
/// Returns `None` if there is no child corresponding to token DES
fn DES(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(DES, 0)
}

}

impl<'input> Mnemonic_dContextAttrs<'input> for Mnemonic_dContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_d(&mut self,)
	-> Result<Rc<Mnemonic_dContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_dContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_mnemonic_d);
        let mut _localctx: Rc<Mnemonic_dContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(270);
			_la = recog.base.input.la(1);
			if { !(_la==DEC || _la==DES) } {
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
//------------------- mnemonic_e ----------------
pub type Mnemonic_eContextAll<'input> = Mnemonic_eContext<'input>;


pub type Mnemonic_eContext<'input> = BaseParserRuleContext<'input,Mnemonic_eContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_eContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_eContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_eContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_e(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_e(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_eContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_e(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_eContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_e }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_e }
}
antlr_rust::tid!{Mnemonic_eContextExt<'a>}

impl<'input> Mnemonic_eContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_eContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_eContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_eContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_eContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EICALL
/// Returns `None` if there is no child corresponding to token EICALL
fn EICALL(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(EICALL, 0)
}
/// Retrieves first TerminalNode corresponding to token EIJMP
/// Returns `None` if there is no child corresponding to token EIJMP
fn EIJMP(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(EIJMP, 0)
}
/// Retrieves first TerminalNode corresponding to token ELPM
/// Returns `None` if there is no child corresponding to token ELPM
fn ELPM(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ELPM, 0)
}
/// Retrieves first TerminalNode corresponding to token EOR
/// Returns `None` if there is no child corresponding to token EOR
fn EOR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(EOR, 0)
}

}

impl<'input> Mnemonic_eContextAttrs<'input> for Mnemonic_eContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_e(&mut self,)
	-> Result<Rc<Mnemonic_eContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_eContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_mnemonic_e);
        let mut _localctx: Rc<Mnemonic_eContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(272);
			_la = recog.base.input.la(1);
			if { !(((((_la - 51)) & !0x3f) == 0 && ((1usize << (_la - 51)) & ((1usize << (EICALL - 51)) | (1usize << (EIJMP - 51)) | (1usize << (ELPM - 51)) | (1usize << (EOR - 51)))) != 0)) } {
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
//------------------- mnemonic_f ----------------
pub type Mnemonic_fContextAll<'input> = Mnemonic_fContext<'input>;


pub type Mnemonic_fContext<'input> = BaseParserRuleContext<'input,Mnemonic_fContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_fContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_fContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_fContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_f(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_f(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_fContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_f(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_fContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_f }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_f }
}
antlr_rust::tid!{Mnemonic_fContextExt<'a>}

impl<'input> Mnemonic_fContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_fContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_fContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_fContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_fContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FMUL
/// Returns `None` if there is no child corresponding to token FMUL
fn FMUL(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(FMUL, 0)
}
/// Retrieves first TerminalNode corresponding to token FMULS
/// Returns `None` if there is no child corresponding to token FMULS
fn FMULS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(FMULS, 0)
}
/// Retrieves first TerminalNode corresponding to token FMULSU
/// Returns `None` if there is no child corresponding to token FMULSU
fn FMULSU(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(FMULSU, 0)
}

}

impl<'input> Mnemonic_fContextAttrs<'input> for Mnemonic_fContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_f(&mut self,)
	-> Result<Rc<Mnemonic_fContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_fContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_mnemonic_f);
        let mut _localctx: Rc<Mnemonic_fContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(274);
			_la = recog.base.input.la(1);
			if { !(((((_la - 55)) & !0x3f) == 0 && ((1usize << (_la - 55)) & ((1usize << (FMUL - 55)) | (1usize << (FMULS - 55)) | (1usize << (FMULSU - 55)))) != 0)) } {
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
//------------------- mnemonic_i ----------------
pub type Mnemonic_iContextAll<'input> = Mnemonic_iContext<'input>;


pub type Mnemonic_iContext<'input> = BaseParserRuleContext<'input,Mnemonic_iContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_iContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_iContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_iContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_i(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_i(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_iContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_i(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_iContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_i }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_i }
}
antlr_rust::tid!{Mnemonic_iContextExt<'a>}

impl<'input> Mnemonic_iContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_iContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_iContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_iContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_iContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ICALL
/// Returns `None` if there is no child corresponding to token ICALL
fn ICALL(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ICALL, 0)
}
/// Retrieves first TerminalNode corresponding to token IJMP
/// Returns `None` if there is no child corresponding to token IJMP
fn IJMP(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IJMP, 0)
}
/// Retrieves first TerminalNode corresponding to token IN
/// Returns `None` if there is no child corresponding to token IN
fn IN(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(IN, 0)
}
/// Retrieves first TerminalNode corresponding to token INC
/// Returns `None` if there is no child corresponding to token INC
fn INC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(INC, 0)
}

}

impl<'input> Mnemonic_iContextAttrs<'input> for Mnemonic_iContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_i(&mut self,)
	-> Result<Rc<Mnemonic_iContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_iContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_mnemonic_i);
        let mut _localctx: Rc<Mnemonic_iContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(276);
			_la = recog.base.input.la(1);
			if { !(((((_la - 58)) & !0x3f) == 0 && ((1usize << (_la - 58)) & ((1usize << (ICALL - 58)) | (1usize << (IJMP - 58)) | (1usize << (IN - 58)) | (1usize << (INC - 58)))) != 0)) } {
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
//------------------- mnemonic_j ----------------
pub type Mnemonic_jContextAll<'input> = Mnemonic_jContext<'input>;


pub type Mnemonic_jContext<'input> = BaseParserRuleContext<'input,Mnemonic_jContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_jContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_jContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_jContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_j(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_j(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_jContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_j(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_jContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_j }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_j }
}
antlr_rust::tid!{Mnemonic_jContextExt<'a>}

impl<'input> Mnemonic_jContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_jContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_jContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_jContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_jContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token JMP
/// Returns `None` if there is no child corresponding to token JMP
fn JMP(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(JMP, 0)
}

}

impl<'input> Mnemonic_jContextAttrs<'input> for Mnemonic_jContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_j(&mut self,)
	-> Result<Rc<Mnemonic_jContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_jContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_mnemonic_j);
        let mut _localctx: Rc<Mnemonic_jContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(278);
			recog.base.match_token(JMP,&mut recog.err_handler)?;

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
//------------------- mnemonic_l ----------------
pub type Mnemonic_lContextAll<'input> = Mnemonic_lContext<'input>;


pub type Mnemonic_lContext<'input> = BaseParserRuleContext<'input,Mnemonic_lContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_lContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_lContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_lContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_l(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_l(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_lContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_l(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_lContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_l }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_l }
}
antlr_rust::tid!{Mnemonic_lContextExt<'a>}

impl<'input> Mnemonic_lContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_lContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_lContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_lContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_lContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LAC
/// Returns `None` if there is no child corresponding to token LAC
fn LAC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LAC, 0)
}
/// Retrieves first TerminalNode corresponding to token LAS
/// Returns `None` if there is no child corresponding to token LAS
fn LAS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LAS, 0)
}
/// Retrieves first TerminalNode corresponding to token LAT
/// Returns `None` if there is no child corresponding to token LAT
fn LAT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LAT, 0)
}
/// Retrieves first TerminalNode corresponding to token LD
/// Returns `None` if there is no child corresponding to token LD
fn LD(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LD, 0)
}
/// Retrieves first TerminalNode corresponding to token LDD
/// Returns `None` if there is no child corresponding to token LDD
fn LDD(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LDD, 0)
}
/// Retrieves first TerminalNode corresponding to token LDI
/// Returns `None` if there is no child corresponding to token LDI
fn LDI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LDI, 0)
}
/// Retrieves first TerminalNode corresponding to token LDS
/// Returns `None` if there is no child corresponding to token LDS
fn LDS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LDS, 0)
}
/// Retrieves first TerminalNode corresponding to token LPM
/// Returns `None` if there is no child corresponding to token LPM
fn LPM(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LPM, 0)
}
/// Retrieves first TerminalNode corresponding to token LSL
/// Returns `None` if there is no child corresponding to token LSL
fn LSL(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LSL, 0)
}
/// Retrieves first TerminalNode corresponding to token LSR
/// Returns `None` if there is no child corresponding to token LSR
fn LSR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(LSR, 0)
}

}

impl<'input> Mnemonic_lContextAttrs<'input> for Mnemonic_lContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_l(&mut self,)
	-> Result<Rc<Mnemonic_lContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_lContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_mnemonic_l);
        let mut _localctx: Rc<Mnemonic_lContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(280);
			_la = recog.base.input.la(1);
			if { !(((((_la - 63)) & !0x3f) == 0 && ((1usize << (_la - 63)) & ((1usize << (LAC - 63)) | (1usize << (LAS - 63)) | (1usize << (LAT - 63)) | (1usize << (LD - 63)) | (1usize << (LDD - 63)) | (1usize << (LDI - 63)) | (1usize << (LDS - 63)) | (1usize << (LPM - 63)) | (1usize << (LSL - 63)) | (1usize << (LSR - 63)))) != 0)) } {
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
//------------------- mnemonic_m ----------------
pub type Mnemonic_mContextAll<'input> = Mnemonic_mContext<'input>;


pub type Mnemonic_mContext<'input> = BaseParserRuleContext<'input,Mnemonic_mContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_mContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_mContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_mContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_m(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_m(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_mContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_m(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_mContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_m }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_m }
}
antlr_rust::tid!{Mnemonic_mContextExt<'a>}

impl<'input> Mnemonic_mContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_mContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_mContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_mContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_mContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MOV
/// Returns `None` if there is no child corresponding to token MOV
fn MOV(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(MOV, 0)
}
/// Retrieves first TerminalNode corresponding to token MOVW
/// Returns `None` if there is no child corresponding to token MOVW
fn MOVW(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(MOVW, 0)
}
/// Retrieves first TerminalNode corresponding to token MUL
/// Returns `None` if there is no child corresponding to token MUL
fn MUL(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(MUL, 0)
}
/// Retrieves first TerminalNode corresponding to token MULS
/// Returns `None` if there is no child corresponding to token MULS
fn MULS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(MULS, 0)
}
/// Retrieves first TerminalNode corresponding to token MULSU
/// Returns `None` if there is no child corresponding to token MULSU
fn MULSU(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(MULSU, 0)
}

}

impl<'input> Mnemonic_mContextAttrs<'input> for Mnemonic_mContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_m(&mut self,)
	-> Result<Rc<Mnemonic_mContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_mContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_mnemonic_m);
        let mut _localctx: Rc<Mnemonic_mContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(282);
			_la = recog.base.input.la(1);
			if { !(((((_la - 73)) & !0x3f) == 0 && ((1usize << (_la - 73)) & ((1usize << (MOV - 73)) | (1usize << (MOVW - 73)) | (1usize << (MUL - 73)) | (1usize << (MULS - 73)) | (1usize << (MULSU - 73)))) != 0)) } {
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
//------------------- mnemonic_n ----------------
pub type Mnemonic_nContextAll<'input> = Mnemonic_nContext<'input>;


pub type Mnemonic_nContext<'input> = BaseParserRuleContext<'input,Mnemonic_nContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_nContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_nContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_nContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_n(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_n(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_nContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_n(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_nContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_n }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_n }
}
antlr_rust::tid!{Mnemonic_nContextExt<'a>}

impl<'input> Mnemonic_nContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_nContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_nContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_nContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_nContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token NEG
/// Returns `None` if there is no child corresponding to token NEG
fn NEG(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(NEG, 0)
}
/// Retrieves first TerminalNode corresponding to token NOP
/// Returns `None` if there is no child corresponding to token NOP
fn NOP(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(NOP, 0)
}

}

impl<'input> Mnemonic_nContextAttrs<'input> for Mnemonic_nContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_n(&mut self,)
	-> Result<Rc<Mnemonic_nContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_nContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_mnemonic_n);
        let mut _localctx: Rc<Mnemonic_nContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(284);
			_la = recog.base.input.la(1);
			if { !(_la==NEG || _la==NOP) } {
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
//------------------- mnemonic_o ----------------
pub type Mnemonic_oContextAll<'input> = Mnemonic_oContext<'input>;


pub type Mnemonic_oContext<'input> = BaseParserRuleContext<'input,Mnemonic_oContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_oContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_oContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_oContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_o(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_o(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_oContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_o(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_oContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_o }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_o }
}
antlr_rust::tid!{Mnemonic_oContextExt<'a>}

impl<'input> Mnemonic_oContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_oContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_oContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_oContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_oContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token OR
/// Returns `None` if there is no child corresponding to token OR
fn OR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(OR, 0)
}
/// Retrieves first TerminalNode corresponding to token ORI
/// Returns `None` if there is no child corresponding to token ORI
fn ORI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ORI, 0)
}
/// Retrieves first TerminalNode corresponding to token OUT
/// Returns `None` if there is no child corresponding to token OUT
fn OUT(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(OUT, 0)
}

}

impl<'input> Mnemonic_oContextAttrs<'input> for Mnemonic_oContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_o(&mut self,)
	-> Result<Rc<Mnemonic_oContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_oContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_mnemonic_o);
        let mut _localctx: Rc<Mnemonic_oContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(286);
			_la = recog.base.input.la(1);
			if { !(((((_la - 80)) & !0x3f) == 0 && ((1usize << (_la - 80)) & ((1usize << (OR - 80)) | (1usize << (ORI - 80)) | (1usize << (OUT - 80)))) != 0)) } {
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
//------------------- mnemonic_p ----------------
pub type Mnemonic_pContextAll<'input> = Mnemonic_pContext<'input>;


pub type Mnemonic_pContext<'input> = BaseParserRuleContext<'input,Mnemonic_pContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_pContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_pContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_pContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_p(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_p(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_pContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_p(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_pContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_p }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_p }
}
antlr_rust::tid!{Mnemonic_pContextExt<'a>}

impl<'input> Mnemonic_pContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_pContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_pContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_pContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_pContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token POP
/// Returns `None` if there is no child corresponding to token POP
fn POP(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(POP, 0)
}
/// Retrieves first TerminalNode corresponding to token PUSH
/// Returns `None` if there is no child corresponding to token PUSH
fn PUSH(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(PUSH, 0)
}

}

impl<'input> Mnemonic_pContextAttrs<'input> for Mnemonic_pContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_p(&mut self,)
	-> Result<Rc<Mnemonic_pContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_pContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_mnemonic_p);
        let mut _localctx: Rc<Mnemonic_pContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(288);
			_la = recog.base.input.la(1);
			if { !(_la==POP || _la==PUSH) } {
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
//------------------- mnemonic_r ----------------
pub type Mnemonic_rContextAll<'input> = Mnemonic_rContext<'input>;


pub type Mnemonic_rContext<'input> = BaseParserRuleContext<'input,Mnemonic_rContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_rContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_rContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_rContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_r(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_r(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_rContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_r(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_rContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_r }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_r }
}
antlr_rust::tid!{Mnemonic_rContextExt<'a>}

impl<'input> Mnemonic_rContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_rContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_rContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_rContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_rContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token RCALL
/// Returns `None` if there is no child corresponding to token RCALL
fn RCALL(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(RCALL, 0)
}
/// Retrieves first TerminalNode corresponding to token RET
/// Returns `None` if there is no child corresponding to token RET
fn RET(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(RET, 0)
}
/// Retrieves first TerminalNode corresponding to token RETI
/// Returns `None` if there is no child corresponding to token RETI
fn RETI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(RETI, 0)
}
/// Retrieves first TerminalNode corresponding to token RJMP
/// Returns `None` if there is no child corresponding to token RJMP
fn RJMP(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(RJMP, 0)
}
/// Retrieves first TerminalNode corresponding to token ROL
/// Returns `None` if there is no child corresponding to token ROL
fn ROL(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ROL, 0)
}
/// Retrieves first TerminalNode corresponding to token ROR
/// Returns `None` if there is no child corresponding to token ROR
fn ROR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ROR, 0)
}

}

impl<'input> Mnemonic_rContextAttrs<'input> for Mnemonic_rContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_r(&mut self,)
	-> Result<Rc<Mnemonic_rContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_rContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_mnemonic_r);
        let mut _localctx: Rc<Mnemonic_rContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(290);
			_la = recog.base.input.la(1);
			if { !(((((_la - 85)) & !0x3f) == 0 && ((1usize << (_la - 85)) & ((1usize << (RCALL - 85)) | (1usize << (RET - 85)) | (1usize << (RETI - 85)) | (1usize << (RJMP - 85)) | (1usize << (ROL - 85)) | (1usize << (ROR - 85)))) != 0)) } {
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
//------------------- mnemonic_s ----------------
pub type Mnemonic_sContextAll<'input> = Mnemonic_sContext<'input>;


pub type Mnemonic_sContext<'input> = BaseParserRuleContext<'input,Mnemonic_sContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_sContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_sContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_sContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_s(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_s(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_sContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_s(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_sContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_s }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_s }
}
antlr_rust::tid!{Mnemonic_sContextExt<'a>}

impl<'input> Mnemonic_sContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_sContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_sContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_sContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_sContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SBC
/// Returns `None` if there is no child corresponding to token SBC
fn SBC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SBC, 0)
}
/// Retrieves first TerminalNode corresponding to token SBCI
/// Returns `None` if there is no child corresponding to token SBCI
fn SBCI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SBCI, 0)
}
/// Retrieves first TerminalNode corresponding to token SBI
/// Returns `None` if there is no child corresponding to token SBI
fn SBI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SBI, 0)
}
/// Retrieves first TerminalNode corresponding to token SBIC
/// Returns `None` if there is no child corresponding to token SBIC
fn SBIC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SBIC, 0)
}
/// Retrieves first TerminalNode corresponding to token SBIS
/// Returns `None` if there is no child corresponding to token SBIS
fn SBIS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SBIS, 0)
}
/// Retrieves first TerminalNode corresponding to token SBIW
/// Returns `None` if there is no child corresponding to token SBIW
fn SBIW(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SBIW, 0)
}
/// Retrieves first TerminalNode corresponding to token SBR
/// Returns `None` if there is no child corresponding to token SBR
fn SBR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SBR, 0)
}
/// Retrieves first TerminalNode corresponding to token SBRC
/// Returns `None` if there is no child corresponding to token SBRC
fn SBRC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SBRC, 0)
}
/// Retrieves first TerminalNode corresponding to token SBRS
/// Returns `None` if there is no child corresponding to token SBRS
fn SBRS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SBRS, 0)
}
/// Retrieves first TerminalNode corresponding to token SEC
/// Returns `None` if there is no child corresponding to token SEC
fn SEC(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SEC, 0)
}
/// Retrieves first TerminalNode corresponding to token SEH
/// Returns `None` if there is no child corresponding to token SEH
fn SEH(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SEH, 0)
}
/// Retrieves first TerminalNode corresponding to token SEI
/// Returns `None` if there is no child corresponding to token SEI
fn SEI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SEI, 0)
}
/// Retrieves first TerminalNode corresponding to token SEN
/// Returns `None` if there is no child corresponding to token SEN
fn SEN(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SEN, 0)
}
/// Retrieves first TerminalNode corresponding to token SER
/// Returns `None` if there is no child corresponding to token SER
fn SER(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SER, 0)
}
/// Retrieves first TerminalNode corresponding to token SES
/// Returns `None` if there is no child corresponding to token SES
fn SES(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SES, 0)
}
/// Retrieves first TerminalNode corresponding to token SET
/// Returns `None` if there is no child corresponding to token SET
fn SET(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SET, 0)
}
/// Retrieves first TerminalNode corresponding to token SEV
/// Returns `None` if there is no child corresponding to token SEV
fn SEV(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SEV, 0)
}
/// Retrieves first TerminalNode corresponding to token SEZ
/// Returns `None` if there is no child corresponding to token SEZ
fn SEZ(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SEZ, 0)
}
/// Retrieves first TerminalNode corresponding to token SLEEP
/// Returns `None` if there is no child corresponding to token SLEEP
fn SLEEP(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SLEEP, 0)
}
/// Retrieves first TerminalNode corresponding to token SPM
/// Returns `None` if there is no child corresponding to token SPM
fn SPM(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SPM, 0)
}
/// Retrieves first TerminalNode corresponding to token ST
/// Returns `None` if there is no child corresponding to token ST
fn ST(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(ST, 0)
}
/// Retrieves first TerminalNode corresponding to token STD
/// Returns `None` if there is no child corresponding to token STD
fn STD(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(STD, 0)
}
/// Retrieves first TerminalNode corresponding to token STS
/// Returns `None` if there is no child corresponding to token STS
fn STS(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(STS, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SUB, 0)
}
/// Retrieves first TerminalNode corresponding to token SUBI
/// Returns `None` if there is no child corresponding to token SUBI
fn SUBI(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SUBI, 0)
}
/// Retrieves first TerminalNode corresponding to token SWAP
/// Returns `None` if there is no child corresponding to token SWAP
fn SWAP(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(SWAP, 0)
}

}

impl<'input> Mnemonic_sContextAttrs<'input> for Mnemonic_sContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_s(&mut self,)
	-> Result<Rc<Mnemonic_sContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_sContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_mnemonic_s);
        let mut _localctx: Rc<Mnemonic_sContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(292);
			_la = recog.base.input.la(1);
			if { !(((((_la - 91)) & !0x3f) == 0 && ((1usize << (_la - 91)) & ((1usize << (SBC - 91)) | (1usize << (SBCI - 91)) | (1usize << (SBI - 91)) | (1usize << (SBIC - 91)) | (1usize << (SBIS - 91)) | (1usize << (SBIW - 91)) | (1usize << (SBR - 91)) | (1usize << (SBRC - 91)) | (1usize << (SBRS - 91)) | (1usize << (SEC - 91)) | (1usize << (SEH - 91)) | (1usize << (SEI - 91)) | (1usize << (SEN - 91)) | (1usize << (SER - 91)) | (1usize << (SES - 91)) | (1usize << (SET - 91)) | (1usize << (SEV - 91)) | (1usize << (SEZ - 91)) | (1usize << (SLEEP - 91)) | (1usize << (SPM - 91)) | (1usize << (ST - 91)) | (1usize << (STD - 91)) | (1usize << (STS - 91)) | (1usize << (SUB - 91)) | (1usize << (SUBI - 91)) | (1usize << (SWAP - 91)))) != 0)) } {
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
//------------------- mnemonic_t ----------------
pub type Mnemonic_tContextAll<'input> = Mnemonic_tContext<'input>;


pub type Mnemonic_tContext<'input> = BaseParserRuleContext<'input,Mnemonic_tContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_tContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_tContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_tContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_t(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_t(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_tContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_t(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_tContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_t }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_t }
}
antlr_rust::tid!{Mnemonic_tContextExt<'a>}

impl<'input> Mnemonic_tContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_tContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_tContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_tContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_tContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TST
/// Returns `None` if there is no child corresponding to token TST
fn TST(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(TST, 0)
}

}

impl<'input> Mnemonic_tContextAttrs<'input> for Mnemonic_tContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_t(&mut self,)
	-> Result<Rc<Mnemonic_tContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_tContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_mnemonic_t);
        let mut _localctx: Rc<Mnemonic_tContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(294);
			recog.base.match_token(TST,&mut recog.err_handler)?;

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
//------------------- mnemonic_w ----------------
pub type Mnemonic_wContextAll<'input> = Mnemonic_wContext<'input>;


pub type Mnemonic_wContext<'input> = BaseParserRuleContext<'input,Mnemonic_wContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_wContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_wContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_wContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_w(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_w(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_wContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_w(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_wContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_w }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_w }
}
antlr_rust::tid!{Mnemonic_wContextExt<'a>}

impl<'input> Mnemonic_wContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_wContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_wContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_wContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_wContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WDR
/// Returns `None` if there is no child corresponding to token WDR
fn WDR(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(WDR, 0)
}

}

impl<'input> Mnemonic_wContextAttrs<'input> for Mnemonic_wContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_w(&mut self,)
	-> Result<Rc<Mnemonic_wContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_wContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_mnemonic_w);
        let mut _localctx: Rc<Mnemonic_wContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(296);
			recog.base.match_token(WDR,&mut recog.err_handler)?;

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
//------------------- mnemonic_x ----------------
pub type Mnemonic_xContextAll<'input> = Mnemonic_xContext<'input>;


pub type Mnemonic_xContext<'input> = BaseParserRuleContext<'input,Mnemonic_xContextExt<'input>>;

#[derive(Clone)]
pub struct Mnemonic_xContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> assembler_parserContext<'input> for Mnemonic_xContext<'input>{}

impl<'input,'a> Listenable<dyn assembler_parserListener<'input> + 'a> for Mnemonic_xContext<'input>{
		fn enter(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mnemonic_x(self);
		}
		fn exit(&self,listener: &mut (dyn assembler_parserListener<'input> + 'a)) {
			listener.exit_mnemonic_x(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn assembler_parserVisitor<'input> + 'a> for Mnemonic_xContext<'input>{
	fn accept(&self,visitor: &mut (dyn assembler_parserVisitor<'input> + 'a)) {
		visitor.visit_mnemonic_x(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mnemonic_xContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = assembler_parserContextType;
	fn get_rule_index(&self) -> usize { RULE_mnemonic_x }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mnemonic_x }
}
antlr_rust::tid!{Mnemonic_xContextExt<'a>}

impl<'input> Mnemonic_xContextExt<'input>{
	fn new(parent: Option<Rc<dyn assembler_parserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mnemonic_xContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mnemonic_xContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mnemonic_xContextAttrs<'input>: assembler_parserContext<'input> + BorrowMut<Mnemonic_xContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token XCH
/// Returns `None` if there is no child corresponding to token XCH
fn XCH(&self) -> Option<Rc<TerminalNode<'input,assembler_parserContextType>>> where Self:Sized{
	self.get_token(XCH, 0)
}

}

impl<'input> Mnemonic_xContextAttrs<'input> for Mnemonic_xContext<'input>{}

impl<'input, I, H> assembler_parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mnemonic_x(&mut self,)
	-> Result<Rc<Mnemonic_xContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mnemonic_xContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_mnemonic_x);
        let mut _localctx: Rc<Mnemonic_xContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(298);
			recog.base.match_token(XCH,&mut recog.err_handler)?;

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
	\u{a5}\u{12f}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x03\x02\x07\x02\x46\x0a\x02\x0c\x02\
	\x0e\x02\x49\x0b\x02\x03\x02\x03\x02\x07\x02\x4d\x0a\x02\x0c\x02\x0e\x02\
	\x50\x0b\x02\x03\x02\x07\x02\x53\x0a\x02\x0c\x02\x0e\x02\x56\x0b\x02\x03\
	\x02\x07\x02\x59\x0a\x02\x0c\x02\x0e\x02\x5c\x0b\x02\x03\x02\x03\x02\x03\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x05\x03\x65\x0a\x03\x03\x04\x03\x04\
	\x03\x04\x03\x04\x05\x04\x6b\x0a\x04\x05\x04\x6d\x0a\x04\x03\x05\x05\x05\
	\x70\x0a\x05\x03\x05\x03\x05\x05\x05\x74\x0a\x05\x03\x05\x03\x05\x03\x05\
	\x03\x05\x05\x05\x7a\x0a\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x07\x03\
	\x07\x07\x07\u{82}\x0a\x07\x0c\x07\x0e\x07\u{85}\x0b\x07\x03\x08\x03\x08\
	\x03\x08\x03\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\
	\x03\x0a\x03\x0a\x05\x0a\u{9d}\x0a\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x07\x0a\u{b7}\x0a\x0a\x0c\x0a\x0e\x0a\u{ba}\x0b\x0a\x03\x0b\
	\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{dc}\x0a\x0c\x03\x0d\x03\x0d\x03\
	\x0d\x03\x0d\x03\x0d\x03\x0d\x07\x0d\u{e4}\x0a\x0d\x0c\x0d\x0e\x0d\u{e7}\
	\x0b\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{ed}\x0a\x0e\x03\x0e\x03\
	\x0e\x03\x0f\x03\x0f\x06\x0f\u{f3}\x0a\x0f\x0d\x0f\x0e\x0f\u{f4}\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x05\x10\
	\u{109}\x0a\x10\x03\x11\x03\x11\x03\x12\x03\x12\x03\x13\x03\x13\x03\x14\
	\x03\x14\x03\x15\x03\x15\x03\x16\x03\x16\x03\x17\x03\x17\x03\x18\x03\x18\
	\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1b\x03\x1b\x03\x1c\x03\x1c\x03\x1d\
	\x03\x1d\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\x20\x03\x20\x03\x21\x03\x21\
	\x03\x22\x03\x22\x03\x22\x02\x04\x12\x18\x23\x02\x04\x06\x08\x0a\x0c\x0e\
	\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\
	\x34\x36\x38\x3a\x3c\x3e\x40\x42\x02\x15\x04\x02\u{94}\u{94}\u{97}\u{97}\
	\x03\x02\u{a2}\u{a4}\x04\x02\u{a3}\u{a3}\u{a5}\u{a5}\x04\x02\u{9f}\u{9f}\
	\u{a2}\u{a3}\x05\x02\u{89}\u{89}\u{8f}\u{8f}\u{a2}\u{a5}\x03\x02\x03\x08\
	\x03\x02\x09\x21\x03\x02\x22\x32\x03\x02\x33\x34\x03\x02\x35\x38\x03\x02\
	\x39\x3b\x03\x02\x3c\x3f\x03\x02\x41\x4a\x03\x02\x4b\x4f\x03\x02\x50\x51\
	\x03\x02\x52\x54\x03\x02\x55\x56\x03\x02\x57\x5c\x03\x02\x5d\x76\x02\u{151}\
	\x02\x47\x03\x02\x02\x02\x04\x64\x03\x02\x02\x02\x06\x66\x03\x02\x02\x02\
	\x08\x79\x03\x02\x02\x02\x0a\x7b\x03\x02\x02\x02\x0c\x7f\x03\x02\x02\x02\
	\x0e\u{86}\x03\x02\x02\x02\x10\u{89}\x03\x02\x02\x02\x12\u{9c}\x03\x02\x02\
	\x02\x14\u{bb}\x03\x02\x02\x02\x16\u{bd}\x03\x02\x02\x02\x18\u{dd}\x03\x02\
	\x02\x02\x1a\u{e8}\x03\x02\x02\x02\x1c\u{f0}\x03\x02\x02\x02\x1e\u{108}\
	\x03\x02\x02\x02\x20\u{10a}\x03\x02\x02\x02\x22\u{10c}\x03\x02\x02\x02\x24\
	\u{10e}\x03\x02\x02\x02\x26\u{110}\x03\x02\x02\x02\x28\u{112}\x03\x02\x02\
	\x02\x2a\u{114}\x03\x02\x02\x02\x2c\u{116}\x03\x02\x02\x02\x2e\u{118}\x03\
	\x02\x02\x02\x30\u{11a}\x03\x02\x02\x02\x32\u{11c}\x03\x02\x02\x02\x34\u{11e}\
	\x03\x02\x02\x02\x36\u{120}\x03\x02\x02\x02\x38\u{122}\x03\x02\x02\x02\x3a\
	\u{124}\x03\x02\x02\x02\x3c\u{126}\x03\x02\x02\x02\x3e\u{128}\x03\x02\x02\
	\x02\x40\u{12a}\x03\x02\x02\x02\x42\u{12c}\x03\x02\x02\x02\x44\x46\x07\u{9a}\
	\x02\x02\x45\x44\x03\x02\x02\x02\x46\x49\x03\x02\x02\x02\x47\x45\x03\x02\
	\x02\x02\x47\x48\x03\x02\x02\x02\x48\x4a\x03\x02\x02\x02\x49\x47\x03\x02\
	\x02\x02\x4a\x54\x05\x04\x03\x02\x4b\x4d\x07\u{9a}\x02\x02\x4c\x4b\x03\x02\
	\x02\x02\x4d\x50\x03\x02\x02\x02\x4e\x4c\x03\x02\x02\x02\x4e\x4f\x03\x02\
	\x02\x02\x4f\x51\x03\x02\x02\x02\x50\x4e\x03\x02\x02\x02\x51\x53\x05\x04\
	\x03\x02\x52\x4e\x03\x02\x02\x02\x53\x56\x03\x02\x02\x02\x54\x52\x03\x02\
	\x02\x02\x54\x55\x03\x02\x02\x02\x55\x5a\x03\x02\x02\x02\x56\x54\x03\x02\
	\x02\x02\x57\x59\x07\u{9a}\x02\x02\x58\x57\x03\x02\x02\x02\x59\x5c\x03\x02\
	\x02\x02\x5a\x58\x03\x02\x02\x02\x5a\x5b\x03\x02\x02\x02\x5b\x5d\x03\x02\
	\x02\x02\x5c\x5a\x03\x02\x02\x02\x5d\x5e\x07\x02\x02\x03\x5e\x03\x03\x02\
	\x02\x02\x5f\x65\x05\x16\x0c\x02\x60\x65\x05\x0c\x07\x02\x61\x65\x05\x0e\
	\x08\x02\x62\x65\x05\x06\x04\x02\x63\x65\x05\x1c\x0f\x02\x64\x5f\x03\x02\
	\x02\x02\x64\x60\x03\x02\x02\x02\x64\x61\x03\x02\x02\x02\x64\x62\x03\x02\
	\x02\x02\x64\x63\x03\x02\x02\x02\x65\x05\x03\x02\x02\x02\x66\x6c\x05\x1e\
	\x10\x02\x67\x6a\x05\x08\x05\x02\x68\x69\x07\u{80}\x02\x02\x69\x6b\x05\x08\
	\x05\x02\x6a\x68\x03\x02\x02\x02\x6a\x6b\x03\x02\x02\x02\x6b\x6d\x03\x02\
	\x02\x02\x6c\x67\x03\x02\x02\x02\x6c\x6d\x03\x02\x02\x02\x6d\x07\x03\x02\
	\x02\x02\x6e\x70\x07\u{94}\x02\x02\x6f\x6e\x03\x02\x02\x02\x6f\x70\x03\x02\
	\x02\x02\x70\x71\x03\x02\x02\x02\x71\x73\x07\u{a5}\x02\x02\x72\x74\x07\u{97}\
	\x02\x02\x73\x72\x03\x02\x02\x02\x73\x74\x03\x02\x02\x02\x74\x7a\x03\x02\
	\x02\x02\x75\x7a\x05\x12\x0a\x02\x76\x7a\x05\x1a\x0e\x02\x77\x7a\x05\x10\
	\x09\x02\x78\x7a\x05\x0a\x06\x02\x79\x6f\x03\x02\x02\x02\x79\x75\x03\x02\
	\x02\x02\x79\x76\x03\x02\x02\x02\x79\x77\x03\x02\x02\x02\x79\x78\x03\x02\
	\x02\x02\x7a\x09\x03\x02\x02\x02\x7b\x7c\x07\u{a5}\x02\x02\x7c\x7d\x07\x7f\
	\x02\x02\x7d\x7e\x07\u{a5}\x02\x02\x7e\x0b\x03\x02\x02\x02\x7f\u{83}\x07\
	\u{a5}\x02\x02\u{80}\u{82}\x05\x12\x0a\x02\u{81}\u{80}\x03\x02\x02\x02\u{82}\
	\u{85}\x03\x02\x02\x02\u{83}\u{81}\x03\x02\x02\x02\u{83}\u{84}\x03\x02\x02\
	\x02\u{84}\x0d\x03\x02\x02\x02\u{85}\u{83}\x03\x02\x02\x02\u{86}\u{87}\x07\
	\u{a5}\x02\x02\u{87}\u{88}\x07\x7f\x02\x02\u{88}\x0f\x03\x02\x02\x02\u{89}\
	\u{8a}\x07\x7b\x02\x02\u{8a}\u{8b}\x07\u{a2}\x02\x02\u{8b}\x11\x03\x02\x02\
	\x02\u{8c}\u{8d}\x08\x0a\x01\x02\u{8d}\u{8e}\x07\u{95}\x02\x02\u{8e}\u{8f}\
	\x05\x12\x0a\x02\u{8f}\u{90}\x07\x7e\x02\x02\u{90}\u{9d}\x03\x02\x02\x02\
	\u{91}\u{92}\x07\u{85}\x02\x02\u{92}\u{93}\x09\x02\x02\x02\u{93}\u{9d}\x05\
	\x14\x0b\x02\u{94}\u{9d}\x07\u{a4}\x02\x02\u{95}\u{9d}\x07\u{a3}\x02\x02\
	\u{96}\u{9d}\x07\u{a2}\x02\x02\u{97}\u{9d}\x07\u{a5}\x02\x02\u{98}\u{9d}\
	\x07\u{9f}\x02\x02\u{99}\u{9d}\x07\u{a0}\x02\x02\u{9a}\u{9d}\x07\u{a1}\x02\
	\x02\u{9b}\u{9d}\x05\x10\x09\x02\u{9c}\u{8c}\x03\x02\x02\x02\u{9c}\u{91}\
	\x03\x02\x02\x02\u{9c}\u{94}\x03\x02\x02\x02\u{9c}\u{95}\x03\x02\x02\x02\
	\u{9c}\u{96}\x03\x02\x02\x02\u{9c}\u{97}\x03\x02\x02\x02\u{9c}\u{98}\x03\
	\x02\x02\x02\u{9c}\u{99}\x03\x02\x02\x02\u{9c}\u{9a}\x03\x02\x02\x02\u{9c}\
	\u{9b}\x03\x02\x02\x02\u{9d}\u{b8}\x03\x02\x02\x02\u{9e}\u{9f}\x0c\x13\x02\
	\x02\u{9f}\u{a0}\x07\u{91}\x02\x02\u{a0}\u{b7}\x05\x12\x0a\x14\u{a1}\u{a2}\
	\x0c\x12\x02\x02\u{a2}\u{a3}\x07\u{98}\x02\x02\u{a3}\u{b7}\x05\x12\x0a\x13\
	\u{a4}\u{a5}\x0c\x11\x02\x02\u{a5}\u{a6}\x07\x7a\x02\x02\u{a6}\u{b7}\x05\
	\x12\x0a\x12\u{a7}\u{a8}\x0c\x10\x02\x02\u{a8}\u{a9}\x07\u{99}\x02\x02\u{a9}\
	\u{b7}\x05\x12\x0a\x11\u{aa}\u{ab}\x0c\x0f\x02\x02\u{ab}\u{ac}\x07\u{8d}\
	\x02\x02\u{ac}\u{b7}\x05\x12\x0a\x10\u{ad}\u{ae}\x0c\x0e\x02\x02\u{ae}\u{af}\
	\x07\u{92}\x02\x02\u{af}\u{b7}\x05\x12\x0a\x0f\u{b0}\u{b1}\x0c\x0d\x02\x02\
	\u{b1}\u{b2}\x07\u{97}\x02\x02\u{b2}\u{b7}\x05\x12\x0a\x0e\u{b3}\u{b4}\x0c\
	\x0c\x02\x02\u{b4}\u{b5}\x07\u{94}\x02\x02\u{b5}\u{b7}\x05\x12\x0a\x0d\u{b6}\
	\u{9e}\x03\x02\x02\x02\u{b6}\u{a1}\x03\x02\x02\x02\u{b6}\u{a4}\x03\x02\x02\
	\x02\u{b6}\u{a7}\x03\x02\x02\x02\u{b6}\u{aa}\x03\x02\x02\x02\u{b6}\u{ad}\
	\x03\x02\x02\x02\u{b6}\u{b0}\x03\x02\x02\x02\u{b6}\u{b3}\x03\x02\x02\x02\
	\u{b7}\u{ba}\x03\x02\x02\x02\u{b8}\u{b6}\x03\x02\x02\x02\u{b8}\u{b9}\x03\
	\x02\x02\x02\u{b9}\x13\x03\x02\x02\x02\u{ba}\u{b8}\x03\x02\x02\x02\u{bb}\
	\u{bc}\x09\x03\x02\x02\u{bc}\x15\x03\x02\x02\x02\u{bd}\u{db}\x07\u{85}\x02\
	\x02\u{be}\u{bf}\x07\u{90}\x02\x02\u{bf}\u{dc}\x07\u{9f}\x02\x02\u{c0}\u{c1}\
	\x07\u{84}\x02\x02\u{c1}\u{dc}\x07\u{a5}\x02\x02\u{c2}\u{c3}\x07\u{83}\x02\
	\x02\u{c3}\u{c4}\x07\u{a5}\x02\x02\u{c4}\u{c5}\x07\u{8a}\x02\x02\u{c5}\u{dc}\
	\x05\x12\x0a\x02\u{c6}\u{c7}\x07\u{8b}\x02\x02\u{c7}\u{c8}\x07\u{a5}\x02\
	\x02\u{c8}\u{c9}\x07\u{8a}\x02\x02\u{c9}\u{dc}\x05\x12\x0a\x02\u{ca}\u{dc}\
	\x07\u{81}\x02\x02\u{cb}\u{dc}\x07\u{86}\x02\x02\u{cc}\u{cd}\x07\u{82}\x02\
	\x02\u{cd}\u{dc}\x05\x18\x0d\x02\u{ce}\u{cf}\x07\x7c\x02\x02\u{cf}\u{dc}\
	\x05\x12\x0a\x02\u{d0}\u{d1}\x07\u{96}\x02\x02\u{d1}\u{dc}\x09\x04\x02\x02\
	\u{d2}\u{d3}\x07\u{93}\x02\x02\u{d3}\u{dc}\x07\u{a5}\x02\x02\u{d4}\u{dc}\
	\x07\u{88}\x02\x02\u{d5}\u{d6}\x07\u{8f}\x02\x02\u{d6}\u{dc}\x05\x12\x0a\
	\x02\u{d7}\u{dc}\x07\u{87}\x02\x02\u{d8}\u{dc}\x07\u{89}\x02\x02\u{d9}\u{da}\
	\x07\u{8c}\x02\x02\u{da}\u{dc}\x07\u{9f}\x02\x02\u{db}\u{be}\x03\x02\x02\
	\x02\u{db}\u{c0}\x03\x02\x02\x02\u{db}\u{c2}\x03\x02\x02\x02\u{db}\u{c6}\
	\x03\x02\x02\x02\u{db}\u{ca}\x03\x02\x02\x02\u{db}\u{cb}\x03\x02\x02\x02\
	\u{db}\u{cc}\x03\x02\x02\x02\u{db}\u{ce}\x03\x02\x02\x02\u{db}\u{d0}\x03\
	\x02\x02\x02\u{db}\u{d2}\x03\x02\x02\x02\u{db}\u{d4}\x03\x02\x02\x02\u{db}\
	\u{d5}\x03\x02\x02\x02\u{db}\u{d7}\x03\x02\x02\x02\u{db}\u{d8}\x03\x02\x02\
	\x02\u{db}\u{d9}\x03\x02\x02\x02\u{dc}\x17\x03\x02\x02\x02\u{dd}\u{de}\x08\
	\x0d\x01\x02\u{de}\u{df}\x09\x05\x02\x02\u{df}\u{e5}\x03\x02\x02\x02\u{e0}\
	\u{e1}\x0c\x04\x02\x02\u{e1}\u{e2}\x07\u{80}\x02\x02\u{e2}\u{e4}\x09\x05\
	\x02\x02\u{e3}\u{e0}\x03\x02\x02\x02\u{e4}\u{e7}\x03\x02\x02\x02\u{e5}\u{e3}\
	\x03\x02\x02\x02\u{e5}\u{e6}\x03\x02\x02\x02\u{e6}\x19\x03\x02\x02\x02\u{e7}\
	\u{e5}\x03\x02\x02\x02\u{e8}\u{e9}\x07\u{a5}\x02\x02\u{e9}\u{ec}\x07\u{95}\
	\x02\x02\u{ea}\u{ed}\x05\x12\x0a\x02\u{eb}\u{ed}\x05\x10\x09\x02\u{ec}\u{ea}\
	\x03\x02\x02\x02\u{ec}\u{eb}\x03\x02\x02\x02\u{ed}\u{ee}\x03\x02\x02\x02\
	\u{ee}\u{ef}\x07\x7e\x02\x02\u{ef}\x1b\x03\x02\x02\x02\u{f0}\u{f2}\x07\u{8e}\
	\x02\x02\u{f1}\u{f3}\x09\x06\x02\x02\u{f2}\u{f1}\x03\x02\x02\x02\u{f3}\u{f4}\
	\x03\x02\x02\x02\u{f4}\u{f2}\x03\x02\x02\x02\u{f4}\u{f5}\x03\x02\x02\x02\
	\u{f5}\x1d\x03\x02\x02\x02\u{f6}\u{109}\x05\x20\x11\x02\u{f7}\u{109}\x05\
	\x22\x12\x02\u{f8}\u{109}\x05\x24\x13\x02\u{f9}\u{109}\x05\x26\x14\x02\u{fa}\
	\u{109}\x05\x28\x15\x02\u{fb}\u{109}\x05\x2a\x16\x02\u{fc}\u{109}\x05\x2c\
	\x17\x02\u{fd}\u{109}\x05\x2e\x18\x02\u{fe}\u{109}\x05\x30\x19\x02\u{ff}\
	\u{109}\x05\x32\x1a\x02\u{100}\u{109}\x05\x34\x1b\x02\u{101}\u{109}\x05\
	\x36\x1c\x02\u{102}\u{109}\x05\x38\x1d\x02\u{103}\u{109}\x05\x3a\x1e\x02\
	\u{104}\u{109}\x05\x3c\x1f\x02\u{105}\u{109}\x05\x3e\x20\x02\u{106}\u{109}\
	\x05\x40\x21\x02\u{107}\u{109}\x05\x42\x22\x02\u{108}\u{f6}\x03\x02\x02\
	\x02\u{108}\u{f7}\x03\x02\x02\x02\u{108}\u{f8}\x03\x02\x02\x02\u{108}\u{f9}\
	\x03\x02\x02\x02\u{108}\u{fa}\x03\x02\x02\x02\u{108}\u{fb}\x03\x02\x02\x02\
	\u{108}\u{fc}\x03\x02\x02\x02\u{108}\u{fd}\x03\x02\x02\x02\u{108}\u{fe}\
	\x03\x02\x02\x02\u{108}\u{ff}\x03\x02\x02\x02\u{108}\u{100}\x03\x02\x02\
	\x02\u{108}\u{101}\x03\x02\x02\x02\u{108}\u{102}\x03\x02\x02\x02\u{108}\
	\u{103}\x03\x02\x02\x02\u{108}\u{104}\x03\x02\x02\x02\u{108}\u{105}\x03\
	\x02\x02\x02\u{108}\u{106}\x03\x02\x02\x02\u{108}\u{107}\x03\x02\x02\x02\
	\u{109}\x1f\x03\x02\x02\x02\u{10a}\u{10b}\x09\x07\x02\x02\u{10b}\x21\x03\
	\x02\x02\x02\u{10c}\u{10d}\x09\x08\x02\x02\u{10d}\x23\x03\x02\x02\x02\u{10e}\
	\u{10f}\x09\x09\x02\x02\u{10f}\x25\x03\x02\x02\x02\u{110}\u{111}\x09\x0a\
	\x02\x02\u{111}\x27\x03\x02\x02\x02\u{112}\u{113}\x09\x0b\x02\x02\u{113}\
	\x29\x03\x02\x02\x02\u{114}\u{115}\x09\x0c\x02\x02\u{115}\x2b\x03\x02\x02\
	\x02\u{116}\u{117}\x09\x0d\x02\x02\u{117}\x2d\x03\x02\x02\x02\u{118}\u{119}\
	\x07\x40\x02\x02\u{119}\x2f\x03\x02\x02\x02\u{11a}\u{11b}\x09\x0e\x02\x02\
	\u{11b}\x31\x03\x02\x02\x02\u{11c}\u{11d}\x09\x0f\x02\x02\u{11d}\x33\x03\
	\x02\x02\x02\u{11e}\u{11f}\x09\x10\x02\x02\u{11f}\x35\x03\x02\x02\x02\u{120}\
	\u{121}\x09\x11\x02\x02\u{121}\x37\x03\x02\x02\x02\u{122}\u{123}\x09\x12\
	\x02\x02\u{123}\x39\x03\x02\x02\x02\u{124}\u{125}\x09\x13\x02\x02\u{125}\
	\x3b\x03\x02\x02\x02\u{126}\u{127}\x09\x14\x02\x02\u{127}\x3d\x03\x02\x02\
	\x02\u{128}\u{129}\x07\x77\x02\x02\u{129}\x3f\x03\x02\x02\x02\u{12a}\u{12b}\
	\x07\x78\x02\x02\u{12b}\x41\x03\x02\x02\x02\u{12c}\u{12d}\x07\x79\x02\x02\
	\u{12d}\x43\x03\x02\x02\x02\x15\x47\x4e\x54\x5a\x64\x6a\x6c\x6f\x73\x79\
	\u{83}\u{9c}\u{b6}\u{b8}\u{db}\u{e5}\u{ec}\u{f4}\u{108}";


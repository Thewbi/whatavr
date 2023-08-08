// Generated from c:\aaa_se\rust\whatavr\src\parser\assembler.g4 by ANTLR 4.9.2
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast"})
public class assemblerParser extends Parser {
	static { RuntimeMetaData.checkVersion("4.9.2", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		ADD=1, ADIW=2, AND=3, ANDI=4, ASR=5, BCLR=6, BLD=7, BRBC=8, BRBS=9, BRCC=10, 
		BRCS=11, BREAK=12, BREQ=13, BRGE=14, BRHC=15, BRHS=16, BRID=17, BRIE=18, 
		BRLO=19, BRLT=20, BRMI=21, BRNE=22, BRPL=23, BRSH=24, BRTC=25, BRTS=26, 
		BRVC=27, BRVS=28, BSET=29, BST=30, CALL=31, CBI=32, CBR=33, CLC=34, CLH=35, 
		CLI=36, CLN=37, CLR=38, CLS=39, CLT=40, CLV=41, CLZ=42, COM=43, CP=44, 
		CPC=45, CPI=46, CPSE=47, DEC=48, DES=49, EICALL=50, EIJMP=51, ELPM=52, 
		EOR=53, FMUL=54, FMULS=55, FMULSU=56, ICALL=57, IJMP=58, IN=59, INC=60, 
		JMP=61, LAC=62, LAS=63, LAT=64, LD=65, LDI=66, LDS=67, LPM=68, LSL=69, 
		LSR=70, MOV=71, MOVW=72, MUL=73, MULS=74, MULSU=75, NEG=76, NOP=77, OR=78, 
		ORI=79, OUT=80, POP=81, PUSH=82, RCALL=83, RET=84, RETI=85, RJMP=86, ROL=87, 
		ROR=88, SBC=89, SBCI=90, SBI=91, SBIC=92, SBIS=93, SBIW=94, SBR=95, SBRC=96, 
		SBRS=97, SEC=98, SEH=99, SEI=100, SEN=101, SER=102, SES=103, SET=104, 
		SEV=105, SEZ=106, SLEEP=107, SPM=108, ST=109, STS=110, SUB=111, SUBI=112, 
		SWAP=113, TST=114, WDR=115, XCH=116, ASTERISK=117, AT=118, CLOSEING_BRACKET=119, 
		COLON=120, COMMA=121, CSEG=122, DEF=123, DOT=124, ELSE=125, END_MACRO=126, 
		ENDIF=127, EQUALS=128, EQU=129, ERROR=130, GT=131, IF=132, INCLUDE=133, 
		LEFT_SHIFT=134, LT=135, MACRO=136, MINUS=137, OPENING_BRACKET=138, ORG=139, 
		PLUS=140, RIGHT_SHIFT=141, SLASH=142, NEWLINE=143, WS=144, LINE_COMMENT=145, 
		STRING=146, NUMBER=147, HEX_NUMBER=148, IDENTIFIER=149;
	public static final int
		RULE_asm_file = 0, RULE_row = 1, RULE_instruction = 2, RULE_param = 3, 
		RULE_macro_usage = 4, RULE_label_definition = 5, RULE_parameter = 6, RULE_macro_placeholder = 7, 
		RULE_expression = 8, RULE_asm_instrinsic_instruction = 9, RULE_asm_intrinsic_usage = 10, 
		RULE_mnemonic = 11, RULE_mnemonic_a = 12, RULE_mnemonic_b = 13, RULE_mnemonic_c = 14, 
		RULE_mnemonic_d = 15, RULE_mnemonic_e = 16, RULE_mnemonic_f = 17, RULE_mnemonic_i = 18, 
		RULE_mnemonic_j = 19, RULE_mnemonic_l = 20, RULE_mnemonic_m = 21, RULE_mnemonic_n = 22, 
		RULE_mnemonic_o = 23, RULE_mnemonic_p = 24, RULE_mnemonic_r = 25, RULE_mnemonic_s = 26, 
		RULE_mnemonic_t = 27, RULE_mnemonic_w = 28, RULE_mnemonic_x = 29;
	private static String[] makeRuleNames() {
		return new String[] {
			"asm_file", "row", "instruction", "param", "macro_usage", "label_definition", 
			"parameter", "macro_placeholder", "expression", "asm_instrinsic_instruction", 
			"asm_intrinsic_usage", "mnemonic", "mnemonic_a", "mnemonic_b", "mnemonic_c", 
			"mnemonic_d", "mnemonic_e", "mnemonic_f", "mnemonic_i", "mnemonic_j", 
			"mnemonic_l", "mnemonic_m", "mnemonic_n", "mnemonic_o", "mnemonic_p", 
			"mnemonic_r", "mnemonic_s", "mnemonic_t", "mnemonic_w", "mnemonic_x"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, "'*'", "'@'", "')'", 
			"':'", "','", "'cseg'", "'def'", "'.'", "'else'", "'endmacro'", "'endif'", 
			"'='", "'equ'", "'error'", "'>'", "'if'", "'include'", "'<<'", "'<'", 
			"'macro'", "'-'", "'('", "'org'", "'+'", "'>>'", "'/'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "ADD", "ADIW", "AND", "ANDI", "ASR", "BCLR", "BLD", "BRBC", "BRBS", 
			"BRCC", "BRCS", "BREAK", "BREQ", "BRGE", "BRHC", "BRHS", "BRID", "BRIE", 
			"BRLO", "BRLT", "BRMI", "BRNE", "BRPL", "BRSH", "BRTC", "BRTS", "BRVC", 
			"BRVS", "BSET", "BST", "CALL", "CBI", "CBR", "CLC", "CLH", "CLI", "CLN", 
			"CLR", "CLS", "CLT", "CLV", "CLZ", "COM", "CP", "CPC", "CPI", "CPSE", 
			"DEC", "DES", "EICALL", "EIJMP", "ELPM", "EOR", "FMUL", "FMULS", "FMULSU", 
			"ICALL", "IJMP", "IN", "INC", "JMP", "LAC", "LAS", "LAT", "LD", "LDI", 
			"LDS", "LPM", "LSL", "LSR", "MOV", "MOVW", "MUL", "MULS", "MULSU", "NEG", 
			"NOP", "OR", "ORI", "OUT", "POP", "PUSH", "RCALL", "RET", "RETI", "RJMP", 
			"ROL", "ROR", "SBC", "SBCI", "SBI", "SBIC", "SBIS", "SBIW", "SBR", "SBRC", 
			"SBRS", "SEC", "SEH", "SEI", "SEN", "SER", "SES", "SET", "SEV", "SEZ", 
			"SLEEP", "SPM", "ST", "STS", "SUB", "SUBI", "SWAP", "TST", "WDR", "XCH", 
			"ASTERISK", "AT", "CLOSEING_BRACKET", "COLON", "COMMA", "CSEG", "DEF", 
			"DOT", "ELSE", "END_MACRO", "ENDIF", "EQUALS", "EQU", "ERROR", "GT", 
			"IF", "INCLUDE", "LEFT_SHIFT", "LT", "MACRO", "MINUS", "OPENING_BRACKET", 
			"ORG", "PLUS", "RIGHT_SHIFT", "SLASH", "NEWLINE", "WS", "LINE_COMMENT", 
			"STRING", "NUMBER", "HEX_NUMBER", "IDENTIFIER"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "assembler.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public assemblerParser(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	public static class Asm_fileContext extends ParserRuleContext {
		public List<RowContext> row() {
			return getRuleContexts(RowContext.class);
		}
		public RowContext row(int i) {
			return getRuleContext(RowContext.class,i);
		}
		public TerminalNode EOF() { return getToken(assemblerParser.EOF, 0); }
		public List<TerminalNode> NEWLINE() { return getTokens(assemblerParser.NEWLINE); }
		public TerminalNode NEWLINE(int i) {
			return getToken(assemblerParser.NEWLINE, i);
		}
		public Asm_fileContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_asm_file; }
	}

	public final Asm_fileContext asm_file() throws RecognitionException {
		Asm_fileContext _localctx = new Asm_fileContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_asm_file);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(63);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==NEWLINE) {
				{
				{
				setState(60);
				match(NEWLINE);
				}
				}
				setState(65);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(66);
			row();
			setState(76);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(70);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==NEWLINE) {
						{
						{
						setState(67);
						match(NEWLINE);
						}
						}
						setState(72);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					setState(73);
					row();
					}
					} 
				}
				setState(78);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			}
			setState(82);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==NEWLINE) {
				{
				{
				setState(79);
				match(NEWLINE);
				}
				}
				setState(84);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(85);
			match(EOF);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class RowContext extends ParserRuleContext {
		public Asm_instrinsic_instructionContext asm_instrinsic_instruction() {
			return getRuleContext(Asm_instrinsic_instructionContext.class,0);
		}
		public Macro_usageContext macro_usage() {
			return getRuleContext(Macro_usageContext.class,0);
		}
		public Label_definitionContext label_definition() {
			return getRuleContext(Label_definitionContext.class,0);
		}
		public InstructionContext instruction() {
			return getRuleContext(InstructionContext.class,0);
		}
		public RowContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_row; }
	}

	public final RowContext row() throws RecognitionException {
		RowContext _localctx = new RowContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_row);
		try {
			setState(91);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,4,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(87);
				asm_instrinsic_instruction();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(88);
				macro_usage();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(89);
				label_definition();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(90);
				instruction();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class InstructionContext extends ParserRuleContext {
		public MnemonicContext mnemonic() {
			return getRuleContext(MnemonicContext.class,0);
		}
		public List<ParamContext> param() {
			return getRuleContexts(ParamContext.class);
		}
		public ParamContext param(int i) {
			return getRuleContext(ParamContext.class,i);
		}
		public TerminalNode COMMA() { return getToken(assemblerParser.COMMA, 0); }
		public InstructionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_instruction; }
	}

	public final InstructionContext instruction() throws RecognitionException {
		InstructionContext _localctx = new InstructionContext(_ctx, getState());
		enterRule(_localctx, 4, RULE_instruction);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(93);
			mnemonic();
			setState(99);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,6,_ctx) ) {
			case 1:
				{
				setState(94);
				param();
				setState(97);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==COMMA) {
					{
					setState(95);
					match(COMMA);
					setState(96);
					param();
					}
				}

				}
				break;
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ParamContext extends ParserRuleContext {
		public TerminalNode IDENTIFIER() { return getToken(assemblerParser.IDENTIFIER, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public Asm_intrinsic_usageContext asm_intrinsic_usage() {
			return getRuleContext(Asm_intrinsic_usageContext.class,0);
		}
		public Macro_placeholderContext macro_placeholder() {
			return getRuleContext(Macro_placeholderContext.class,0);
		}
		public ParamContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_param; }
	}

	public final ParamContext param() throws RecognitionException {
		ParamContext _localctx = new ParamContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_param);
		try {
			setState(105);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,7,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(101);
				match(IDENTIFIER);
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(102);
				expression(0);
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(103);
				asm_intrinsic_usage();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(104);
				macro_placeholder();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Macro_usageContext extends ParserRuleContext {
		public TerminalNode IDENTIFIER() { return getToken(assemblerParser.IDENTIFIER, 0); }
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public Macro_usageContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_macro_usage; }
	}

	public final Macro_usageContext macro_usage() throws RecognitionException {
		Macro_usageContext _localctx = new Macro_usageContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_macro_usage);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(107);
			match(IDENTIFIER);
			setState(111);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,8,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(108);
					expression(0);
					}
					} 
				}
				setState(113);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,8,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Label_definitionContext extends ParserRuleContext {
		public TerminalNode IDENTIFIER() { return getToken(assemblerParser.IDENTIFIER, 0); }
		public TerminalNode COLON() { return getToken(assemblerParser.COLON, 0); }
		public Label_definitionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_label_definition; }
	}

	public final Label_definitionContext label_definition() throws RecognitionException {
		Label_definitionContext _localctx = new Label_definitionContext(_ctx, getState());
		enterRule(_localctx, 10, RULE_label_definition);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(114);
			match(IDENTIFIER);
			setState(115);
			match(COLON);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ParameterContext extends ParserRuleContext {
		public TerminalNode IDENTIFIER() { return getToken(assemblerParser.IDENTIFIER, 0); }
		public ParameterContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_parameter; }
	}

	public final ParameterContext parameter() throws RecognitionException {
		ParameterContext _localctx = new ParameterContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_parameter);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(117);
			match(IDENTIFIER);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Macro_placeholderContext extends ParserRuleContext {
		public TerminalNode AT() { return getToken(assemblerParser.AT, 0); }
		public TerminalNode NUMBER() { return getToken(assemblerParser.NUMBER, 0); }
		public Macro_placeholderContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_macro_placeholder; }
	}

	public final Macro_placeholderContext macro_placeholder() throws RecognitionException {
		Macro_placeholderContext _localctx = new Macro_placeholderContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_macro_placeholder);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(119);
			match(AT);
			setState(120);
			match(NUMBER);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ExpressionContext extends ParserRuleContext {
		public TerminalNode OPENING_BRACKET() { return getToken(assemblerParser.OPENING_BRACKET, 0); }
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public TerminalNode CLOSEING_BRACKET() { return getToken(assemblerParser.CLOSEING_BRACKET, 0); }
		public TerminalNode NUMBER() { return getToken(assemblerParser.NUMBER, 0); }
		public TerminalNode HEX_NUMBER() { return getToken(assemblerParser.HEX_NUMBER, 0); }
		public TerminalNode IDENTIFIER() { return getToken(assemblerParser.IDENTIFIER, 0); }
		public TerminalNode STRING() { return getToken(assemblerParser.STRING, 0); }
		public Macro_placeholderContext macro_placeholder() {
			return getRuleContext(Macro_placeholderContext.class,0);
		}
		public TerminalNode LEFT_SHIFT() { return getToken(assemblerParser.LEFT_SHIFT, 0); }
		public TerminalNode RIGHT_SHIFT() { return getToken(assemblerParser.RIGHT_SHIFT, 0); }
		public TerminalNode SLASH() { return getToken(assemblerParser.SLASH, 0); }
		public TerminalNode GT() { return getToken(assemblerParser.GT, 0); }
		public TerminalNode LT() { return getToken(assemblerParser.LT, 0); }
		public TerminalNode EQUALS() { return getToken(assemblerParser.EQUALS, 0); }
		public ExpressionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expression; }
	}

	public final ExpressionContext expression() throws RecognitionException {
		return expression(0);
	}

	private ExpressionContext expression(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		ExpressionContext _localctx = new ExpressionContext(_ctx, _parentState);
		ExpressionContext _prevctx = _localctx;
		int _startState = 16;
		enterRecursionRule(_localctx, 16, RULE_expression, _p);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(132);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case OPENING_BRACKET:
				{
				setState(123);
				match(OPENING_BRACKET);
				setState(124);
				expression(0);
				setState(125);
				match(CLOSEING_BRACKET);
				}
				break;
			case NUMBER:
				{
				setState(127);
				match(NUMBER);
				}
				break;
			case HEX_NUMBER:
				{
				setState(128);
				match(HEX_NUMBER);
				}
				break;
			case IDENTIFIER:
				{
				setState(129);
				match(IDENTIFIER);
				}
				break;
			case STRING:
				{
				setState(130);
				match(STRING);
				}
				break;
			case AT:
				{
				setState(131);
				macro_placeholder();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			_ctx.stop = _input.LT(-1);
			setState(154);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,11,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(152);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,10,_ctx) ) {
					case 1:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(134);
						if (!(precpred(_ctx, 11))) throw new FailedPredicateException(this, "precpred(_ctx, 11)");
						setState(135);
						match(LEFT_SHIFT);
						setState(136);
						expression(12);
						}
						break;
					case 2:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(137);
						if (!(precpred(_ctx, 10))) throw new FailedPredicateException(this, "precpred(_ctx, 10)");
						setState(138);
						match(RIGHT_SHIFT);
						setState(139);
						expression(11);
						}
						break;
					case 3:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(140);
						if (!(precpred(_ctx, 9))) throw new FailedPredicateException(this, "precpred(_ctx, 9)");
						setState(141);
						match(SLASH);
						setState(142);
						expression(10);
						}
						break;
					case 4:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(143);
						if (!(precpred(_ctx, 8))) throw new FailedPredicateException(this, "precpred(_ctx, 8)");
						setState(144);
						match(GT);
						setState(145);
						expression(9);
						}
						break;
					case 5:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(146);
						if (!(precpred(_ctx, 7))) throw new FailedPredicateException(this, "precpred(_ctx, 7)");
						setState(147);
						match(LT);
						setState(148);
						expression(8);
						}
						break;
					case 6:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(149);
						if (!(precpred(_ctx, 6))) throw new FailedPredicateException(this, "precpred(_ctx, 6)");
						setState(150);
						match(EQUALS);
						setState(151);
						expression(7);
						}
						break;
					}
					} 
				}
				setState(156);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,11,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	public static class Asm_instrinsic_instructionContext extends ParserRuleContext {
		public TerminalNode DOT() { return getToken(assemblerParser.DOT, 0); }
		public TerminalNode INCLUDE() { return getToken(assemblerParser.INCLUDE, 0); }
		public TerminalNode STRING() { return getToken(assemblerParser.STRING, 0); }
		public TerminalNode DEF() { return getToken(assemblerParser.DEF, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode EQU() { return getToken(assemblerParser.EQU, 0); }
		public TerminalNode CSEG() { return getToken(assemblerParser.CSEG, 0); }
		public TerminalNode ORG() { return getToken(assemblerParser.ORG, 0); }
		public TerminalNode MACRO() { return getToken(assemblerParser.MACRO, 0); }
		public TerminalNode IDENTIFIER() { return getToken(assemblerParser.IDENTIFIER, 0); }
		public TerminalNode END_MACRO() { return getToken(assemblerParser.END_MACRO, 0); }
		public TerminalNode IF() { return getToken(assemblerParser.IF, 0); }
		public TerminalNode ELSE() { return getToken(assemblerParser.ELSE, 0); }
		public TerminalNode ENDIF() { return getToken(assemblerParser.ENDIF, 0); }
		public TerminalNode ERROR() { return getToken(assemblerParser.ERROR, 0); }
		public TerminalNode HEX_NUMBER() { return getToken(assemblerParser.HEX_NUMBER, 0); }
		public Asm_instrinsic_instructionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_asm_instrinsic_instruction; }
	}

	public final Asm_instrinsic_instructionContext asm_instrinsic_instruction() throws RecognitionException {
		Asm_instrinsic_instructionContext _localctx = new Asm_instrinsic_instructionContext(_ctx, getState());
		enterRule(_localctx, 18, RULE_asm_instrinsic_instruction);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(157);
			match(DOT);
			setState(176);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INCLUDE:
				{
				setState(158);
				match(INCLUDE);
				setState(159);
				match(STRING);
				}
				break;
			case DEF:
				{
				setState(160);
				match(DEF);
				setState(161);
				expression(0);
				}
				break;
			case EQU:
				{
				setState(162);
				match(EQU);
				setState(163);
				expression(0);
				}
				break;
			case CSEG:
				{
				setState(164);
				match(CSEG);
				}
				break;
			case ORG:
				{
				setState(165);
				match(ORG);
				setState(166);
				_la = _input.LA(1);
				if ( !(_la==HEX_NUMBER || _la==IDENTIFIER) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				}
				break;
			case MACRO:
				{
				setState(167);
				match(MACRO);
				setState(168);
				match(IDENTIFIER);
				}
				break;
			case END_MACRO:
				{
				setState(169);
				match(END_MACRO);
				}
				break;
			case IF:
				{
				setState(170);
				match(IF);
				setState(171);
				expression(0);
				}
				break;
			case ELSE:
				{
				setState(172);
				match(ELSE);
				}
				break;
			case ENDIF:
				{
				setState(173);
				match(ENDIF);
				}
				break;
			case ERROR:
				{
				setState(174);
				match(ERROR);
				setState(175);
				match(STRING);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Asm_intrinsic_usageContext extends ParserRuleContext {
		public List<TerminalNode> IDENTIFIER() { return getTokens(assemblerParser.IDENTIFIER); }
		public TerminalNode IDENTIFIER(int i) {
			return getToken(assemblerParser.IDENTIFIER, i);
		}
		public TerminalNode OPENING_BRACKET() { return getToken(assemblerParser.OPENING_BRACKET, 0); }
		public TerminalNode CLOSEING_BRACKET() { return getToken(assemblerParser.CLOSEING_BRACKET, 0); }
		public Macro_placeholderContext macro_placeholder() {
			return getRuleContext(Macro_placeholderContext.class,0);
		}
		public Asm_intrinsic_usageContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_asm_intrinsic_usage; }
	}

	public final Asm_intrinsic_usageContext asm_intrinsic_usage() throws RecognitionException {
		Asm_intrinsic_usageContext _localctx = new Asm_intrinsic_usageContext(_ctx, getState());
		enterRule(_localctx, 20, RULE_asm_intrinsic_usage);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(178);
			match(IDENTIFIER);
			setState(179);
			match(OPENING_BRACKET);
			setState(182);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case IDENTIFIER:
				{
				setState(180);
				match(IDENTIFIER);
				}
				break;
			case AT:
				{
				setState(181);
				macro_placeholder();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			setState(184);
			match(CLOSEING_BRACKET);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class MnemonicContext extends ParserRuleContext {
		public Mnemonic_aContext mnemonic_a() {
			return getRuleContext(Mnemonic_aContext.class,0);
		}
		public Mnemonic_bContext mnemonic_b() {
			return getRuleContext(Mnemonic_bContext.class,0);
		}
		public Mnemonic_cContext mnemonic_c() {
			return getRuleContext(Mnemonic_cContext.class,0);
		}
		public Mnemonic_dContext mnemonic_d() {
			return getRuleContext(Mnemonic_dContext.class,0);
		}
		public Mnemonic_eContext mnemonic_e() {
			return getRuleContext(Mnemonic_eContext.class,0);
		}
		public Mnemonic_fContext mnemonic_f() {
			return getRuleContext(Mnemonic_fContext.class,0);
		}
		public Mnemonic_iContext mnemonic_i() {
			return getRuleContext(Mnemonic_iContext.class,0);
		}
		public Mnemonic_jContext mnemonic_j() {
			return getRuleContext(Mnemonic_jContext.class,0);
		}
		public Mnemonic_lContext mnemonic_l() {
			return getRuleContext(Mnemonic_lContext.class,0);
		}
		public Mnemonic_mContext mnemonic_m() {
			return getRuleContext(Mnemonic_mContext.class,0);
		}
		public Mnemonic_nContext mnemonic_n() {
			return getRuleContext(Mnemonic_nContext.class,0);
		}
		public Mnemonic_oContext mnemonic_o() {
			return getRuleContext(Mnemonic_oContext.class,0);
		}
		public Mnemonic_pContext mnemonic_p() {
			return getRuleContext(Mnemonic_pContext.class,0);
		}
		public Mnemonic_rContext mnemonic_r() {
			return getRuleContext(Mnemonic_rContext.class,0);
		}
		public Mnemonic_sContext mnemonic_s() {
			return getRuleContext(Mnemonic_sContext.class,0);
		}
		public Mnemonic_tContext mnemonic_t() {
			return getRuleContext(Mnemonic_tContext.class,0);
		}
		public Mnemonic_wContext mnemonic_w() {
			return getRuleContext(Mnemonic_wContext.class,0);
		}
		public Mnemonic_xContext mnemonic_x() {
			return getRuleContext(Mnemonic_xContext.class,0);
		}
		public MnemonicContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic; }
	}

	public final MnemonicContext mnemonic() throws RecognitionException {
		MnemonicContext _localctx = new MnemonicContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_mnemonic);
		try {
			setState(204);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case ADD:
			case ADIW:
			case AND:
			case ANDI:
			case ASR:
				enterOuterAlt(_localctx, 1);
				{
				setState(186);
				mnemonic_a();
				}
				break;
			case BCLR:
			case BLD:
			case BRBC:
			case BRBS:
			case BRCC:
			case BRCS:
			case BREAK:
			case BREQ:
			case BRGE:
			case BRHC:
			case BRHS:
			case BRID:
			case BRIE:
			case BRLO:
			case BRLT:
			case BRMI:
			case BRNE:
			case BRPL:
			case BRSH:
			case BRTC:
			case BRTS:
			case BRVC:
			case BRVS:
			case BSET:
			case BST:
				enterOuterAlt(_localctx, 2);
				{
				setState(187);
				mnemonic_b();
				}
				break;
			case CALL:
			case CBI:
			case CBR:
			case CLC:
			case CLH:
			case CLI:
			case CLN:
			case CLR:
			case CLS:
			case CLT:
			case CLV:
			case CLZ:
			case COM:
			case CP:
			case CPC:
			case CPI:
			case CPSE:
				enterOuterAlt(_localctx, 3);
				{
				setState(188);
				mnemonic_c();
				}
				break;
			case DEC:
			case DES:
				enterOuterAlt(_localctx, 4);
				{
				setState(189);
				mnemonic_d();
				}
				break;
			case EICALL:
			case EIJMP:
			case ELPM:
			case EOR:
				enterOuterAlt(_localctx, 5);
				{
				setState(190);
				mnemonic_e();
				}
				break;
			case FMUL:
			case FMULS:
			case FMULSU:
				enterOuterAlt(_localctx, 6);
				{
				setState(191);
				mnemonic_f();
				}
				break;
			case ICALL:
			case IJMP:
			case IN:
			case INC:
				enterOuterAlt(_localctx, 7);
				{
				setState(192);
				mnemonic_i();
				}
				break;
			case JMP:
				enterOuterAlt(_localctx, 8);
				{
				setState(193);
				mnemonic_j();
				}
				break;
			case LAC:
			case LAS:
			case LAT:
			case LD:
			case LDI:
			case LDS:
			case LPM:
			case LSL:
			case LSR:
				enterOuterAlt(_localctx, 9);
				{
				setState(194);
				mnemonic_l();
				}
				break;
			case MOV:
			case MOVW:
			case MUL:
			case MULS:
			case MULSU:
				enterOuterAlt(_localctx, 10);
				{
				setState(195);
				mnemonic_m();
				}
				break;
			case NEG:
			case NOP:
				enterOuterAlt(_localctx, 11);
				{
				setState(196);
				mnemonic_n();
				}
				break;
			case OR:
			case ORI:
			case OUT:
				enterOuterAlt(_localctx, 12);
				{
				setState(197);
				mnemonic_o();
				}
				break;
			case POP:
			case PUSH:
				enterOuterAlt(_localctx, 13);
				{
				setState(198);
				mnemonic_p();
				}
				break;
			case RCALL:
			case RET:
			case RETI:
			case RJMP:
			case ROL:
			case ROR:
				enterOuterAlt(_localctx, 14);
				{
				setState(199);
				mnemonic_r();
				}
				break;
			case SBC:
			case SBCI:
			case SBI:
			case SBIC:
			case SBIS:
			case SBIW:
			case SBR:
			case SBRC:
			case SBRS:
			case SEC:
			case SEH:
			case SEI:
			case SEN:
			case SER:
			case SES:
			case SET:
			case SEV:
			case SEZ:
			case SLEEP:
			case SPM:
			case ST:
			case STS:
			case SUB:
			case SUBI:
			case SWAP:
				enterOuterAlt(_localctx, 15);
				{
				setState(200);
				mnemonic_s();
				}
				break;
			case TST:
				enterOuterAlt(_localctx, 16);
				{
				setState(201);
				mnemonic_t();
				}
				break;
			case WDR:
				enterOuterAlt(_localctx, 17);
				{
				setState(202);
				mnemonic_w();
				}
				break;
			case XCH:
				enterOuterAlt(_localctx, 18);
				{
				setState(203);
				mnemonic_x();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_aContext extends ParserRuleContext {
		public TerminalNode ADD() { return getToken(assemblerParser.ADD, 0); }
		public TerminalNode ADIW() { return getToken(assemblerParser.ADIW, 0); }
		public TerminalNode AND() { return getToken(assemblerParser.AND, 0); }
		public TerminalNode ANDI() { return getToken(assemblerParser.ANDI, 0); }
		public TerminalNode ASR() { return getToken(assemblerParser.ASR, 0); }
		public Mnemonic_aContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_a; }
	}

	public final Mnemonic_aContext mnemonic_a() throws RecognitionException {
		Mnemonic_aContext _localctx = new Mnemonic_aContext(_ctx, getState());
		enterRule(_localctx, 24, RULE_mnemonic_a);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(206);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << ADD) | (1L << ADIW) | (1L << AND) | (1L << ANDI) | (1L << ASR))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_bContext extends ParserRuleContext {
		public TerminalNode BCLR() { return getToken(assemblerParser.BCLR, 0); }
		public TerminalNode BLD() { return getToken(assemblerParser.BLD, 0); }
		public TerminalNode BRBC() { return getToken(assemblerParser.BRBC, 0); }
		public TerminalNode BRBS() { return getToken(assemblerParser.BRBS, 0); }
		public TerminalNode BRCC() { return getToken(assemblerParser.BRCC, 0); }
		public TerminalNode BRCS() { return getToken(assemblerParser.BRCS, 0); }
		public TerminalNode BREAK() { return getToken(assemblerParser.BREAK, 0); }
		public TerminalNode BREQ() { return getToken(assemblerParser.BREQ, 0); }
		public TerminalNode BRGE() { return getToken(assemblerParser.BRGE, 0); }
		public TerminalNode BRHC() { return getToken(assemblerParser.BRHC, 0); }
		public TerminalNode BRHS() { return getToken(assemblerParser.BRHS, 0); }
		public TerminalNode BRID() { return getToken(assemblerParser.BRID, 0); }
		public TerminalNode BRIE() { return getToken(assemblerParser.BRIE, 0); }
		public TerminalNode BRLO() { return getToken(assemblerParser.BRLO, 0); }
		public TerminalNode BRLT() { return getToken(assemblerParser.BRLT, 0); }
		public TerminalNode BRMI() { return getToken(assemblerParser.BRMI, 0); }
		public TerminalNode BRNE() { return getToken(assemblerParser.BRNE, 0); }
		public TerminalNode BRPL() { return getToken(assemblerParser.BRPL, 0); }
		public TerminalNode BRSH() { return getToken(assemblerParser.BRSH, 0); }
		public TerminalNode BRTC() { return getToken(assemblerParser.BRTC, 0); }
		public TerminalNode BRTS() { return getToken(assemblerParser.BRTS, 0); }
		public TerminalNode BRVC() { return getToken(assemblerParser.BRVC, 0); }
		public TerminalNode BRVS() { return getToken(assemblerParser.BRVS, 0); }
		public TerminalNode BSET() { return getToken(assemblerParser.BSET, 0); }
		public TerminalNode BST() { return getToken(assemblerParser.BST, 0); }
		public Mnemonic_bContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_b; }
	}

	public final Mnemonic_bContext mnemonic_b() throws RecognitionException {
		Mnemonic_bContext _localctx = new Mnemonic_bContext(_ctx, getState());
		enterRule(_localctx, 26, RULE_mnemonic_b);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(208);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << BCLR) | (1L << BLD) | (1L << BRBC) | (1L << BRBS) | (1L << BRCC) | (1L << BRCS) | (1L << BREAK) | (1L << BREQ) | (1L << BRGE) | (1L << BRHC) | (1L << BRHS) | (1L << BRID) | (1L << BRIE) | (1L << BRLO) | (1L << BRLT) | (1L << BRMI) | (1L << BRNE) | (1L << BRPL) | (1L << BRSH) | (1L << BRTC) | (1L << BRTS) | (1L << BRVC) | (1L << BRVS) | (1L << BSET) | (1L << BST))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_cContext extends ParserRuleContext {
		public TerminalNode CALL() { return getToken(assemblerParser.CALL, 0); }
		public TerminalNode CBI() { return getToken(assemblerParser.CBI, 0); }
		public TerminalNode CBR() { return getToken(assemblerParser.CBR, 0); }
		public TerminalNode CLC() { return getToken(assemblerParser.CLC, 0); }
		public TerminalNode CLH() { return getToken(assemblerParser.CLH, 0); }
		public TerminalNode CLI() { return getToken(assemblerParser.CLI, 0); }
		public TerminalNode CLN() { return getToken(assemblerParser.CLN, 0); }
		public TerminalNode CLR() { return getToken(assemblerParser.CLR, 0); }
		public TerminalNode CLS() { return getToken(assemblerParser.CLS, 0); }
		public TerminalNode CLT() { return getToken(assemblerParser.CLT, 0); }
		public TerminalNode CLV() { return getToken(assemblerParser.CLV, 0); }
		public TerminalNode CLZ() { return getToken(assemblerParser.CLZ, 0); }
		public TerminalNode COM() { return getToken(assemblerParser.COM, 0); }
		public TerminalNode CP() { return getToken(assemblerParser.CP, 0); }
		public TerminalNode CPC() { return getToken(assemblerParser.CPC, 0); }
		public TerminalNode CPI() { return getToken(assemblerParser.CPI, 0); }
		public TerminalNode CPSE() { return getToken(assemblerParser.CPSE, 0); }
		public Mnemonic_cContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_c; }
	}

	public final Mnemonic_cContext mnemonic_c() throws RecognitionException {
		Mnemonic_cContext _localctx = new Mnemonic_cContext(_ctx, getState());
		enterRule(_localctx, 28, RULE_mnemonic_c);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(210);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << CALL) | (1L << CBI) | (1L << CBR) | (1L << CLC) | (1L << CLH) | (1L << CLI) | (1L << CLN) | (1L << CLR) | (1L << CLS) | (1L << CLT) | (1L << CLV) | (1L << CLZ) | (1L << COM) | (1L << CP) | (1L << CPC) | (1L << CPI) | (1L << CPSE))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_dContext extends ParserRuleContext {
		public TerminalNode DEC() { return getToken(assemblerParser.DEC, 0); }
		public TerminalNode DES() { return getToken(assemblerParser.DES, 0); }
		public Mnemonic_dContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_d; }
	}

	public final Mnemonic_dContext mnemonic_d() throws RecognitionException {
		Mnemonic_dContext _localctx = new Mnemonic_dContext(_ctx, getState());
		enterRule(_localctx, 30, RULE_mnemonic_d);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(212);
			_la = _input.LA(1);
			if ( !(_la==DEC || _la==DES) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_eContext extends ParserRuleContext {
		public TerminalNode EICALL() { return getToken(assemblerParser.EICALL, 0); }
		public TerminalNode EIJMP() { return getToken(assemblerParser.EIJMP, 0); }
		public TerminalNode ELPM() { return getToken(assemblerParser.ELPM, 0); }
		public TerminalNode EOR() { return getToken(assemblerParser.EOR, 0); }
		public Mnemonic_eContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_e; }
	}

	public final Mnemonic_eContext mnemonic_e() throws RecognitionException {
		Mnemonic_eContext _localctx = new Mnemonic_eContext(_ctx, getState());
		enterRule(_localctx, 32, RULE_mnemonic_e);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(214);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << EICALL) | (1L << EIJMP) | (1L << ELPM) | (1L << EOR))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_fContext extends ParserRuleContext {
		public TerminalNode FMUL() { return getToken(assemblerParser.FMUL, 0); }
		public TerminalNode FMULS() { return getToken(assemblerParser.FMULS, 0); }
		public TerminalNode FMULSU() { return getToken(assemblerParser.FMULSU, 0); }
		public Mnemonic_fContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_f; }
	}

	public final Mnemonic_fContext mnemonic_f() throws RecognitionException {
		Mnemonic_fContext _localctx = new Mnemonic_fContext(_ctx, getState());
		enterRule(_localctx, 34, RULE_mnemonic_f);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(216);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << FMUL) | (1L << FMULS) | (1L << FMULSU))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_iContext extends ParserRuleContext {
		public TerminalNode ICALL() { return getToken(assemblerParser.ICALL, 0); }
		public TerminalNode IJMP() { return getToken(assemblerParser.IJMP, 0); }
		public TerminalNode IN() { return getToken(assemblerParser.IN, 0); }
		public TerminalNode INC() { return getToken(assemblerParser.INC, 0); }
		public Mnemonic_iContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_i; }
	}

	public final Mnemonic_iContext mnemonic_i() throws RecognitionException {
		Mnemonic_iContext _localctx = new Mnemonic_iContext(_ctx, getState());
		enterRule(_localctx, 36, RULE_mnemonic_i);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(218);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << ICALL) | (1L << IJMP) | (1L << IN) | (1L << INC))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_jContext extends ParserRuleContext {
		public TerminalNode JMP() { return getToken(assemblerParser.JMP, 0); }
		public Mnemonic_jContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_j; }
	}

	public final Mnemonic_jContext mnemonic_j() throws RecognitionException {
		Mnemonic_jContext _localctx = new Mnemonic_jContext(_ctx, getState());
		enterRule(_localctx, 38, RULE_mnemonic_j);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(220);
			match(JMP);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_lContext extends ParserRuleContext {
		public TerminalNode LAC() { return getToken(assemblerParser.LAC, 0); }
		public TerminalNode LAS() { return getToken(assemblerParser.LAS, 0); }
		public TerminalNode LAT() { return getToken(assemblerParser.LAT, 0); }
		public TerminalNode LD() { return getToken(assemblerParser.LD, 0); }
		public TerminalNode LDI() { return getToken(assemblerParser.LDI, 0); }
		public TerminalNode LDS() { return getToken(assemblerParser.LDS, 0); }
		public TerminalNode LPM() { return getToken(assemblerParser.LPM, 0); }
		public TerminalNode LSL() { return getToken(assemblerParser.LSL, 0); }
		public TerminalNode LSR() { return getToken(assemblerParser.LSR, 0); }
		public Mnemonic_lContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_l; }
	}

	public final Mnemonic_lContext mnemonic_l() throws RecognitionException {
		Mnemonic_lContext _localctx = new Mnemonic_lContext(_ctx, getState());
		enterRule(_localctx, 40, RULE_mnemonic_l);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(222);
			_la = _input.LA(1);
			if ( !(((((_la - 62)) & ~0x3f) == 0 && ((1L << (_la - 62)) & ((1L << (LAC - 62)) | (1L << (LAS - 62)) | (1L << (LAT - 62)) | (1L << (LD - 62)) | (1L << (LDI - 62)) | (1L << (LDS - 62)) | (1L << (LPM - 62)) | (1L << (LSL - 62)) | (1L << (LSR - 62)))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_mContext extends ParserRuleContext {
		public TerminalNode MOV() { return getToken(assemblerParser.MOV, 0); }
		public TerminalNode MOVW() { return getToken(assemblerParser.MOVW, 0); }
		public TerminalNode MUL() { return getToken(assemblerParser.MUL, 0); }
		public TerminalNode MULS() { return getToken(assemblerParser.MULS, 0); }
		public TerminalNode MULSU() { return getToken(assemblerParser.MULSU, 0); }
		public Mnemonic_mContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_m; }
	}

	public final Mnemonic_mContext mnemonic_m() throws RecognitionException {
		Mnemonic_mContext _localctx = new Mnemonic_mContext(_ctx, getState());
		enterRule(_localctx, 42, RULE_mnemonic_m);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(224);
			_la = _input.LA(1);
			if ( !(((((_la - 71)) & ~0x3f) == 0 && ((1L << (_la - 71)) & ((1L << (MOV - 71)) | (1L << (MOVW - 71)) | (1L << (MUL - 71)) | (1L << (MULS - 71)) | (1L << (MULSU - 71)))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_nContext extends ParserRuleContext {
		public TerminalNode NEG() { return getToken(assemblerParser.NEG, 0); }
		public TerminalNode NOP() { return getToken(assemblerParser.NOP, 0); }
		public Mnemonic_nContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_n; }
	}

	public final Mnemonic_nContext mnemonic_n() throws RecognitionException {
		Mnemonic_nContext _localctx = new Mnemonic_nContext(_ctx, getState());
		enterRule(_localctx, 44, RULE_mnemonic_n);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(226);
			_la = _input.LA(1);
			if ( !(_la==NEG || _la==NOP) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_oContext extends ParserRuleContext {
		public TerminalNode OR() { return getToken(assemblerParser.OR, 0); }
		public TerminalNode ORI() { return getToken(assemblerParser.ORI, 0); }
		public TerminalNode OUT() { return getToken(assemblerParser.OUT, 0); }
		public Mnemonic_oContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_o; }
	}

	public final Mnemonic_oContext mnemonic_o() throws RecognitionException {
		Mnemonic_oContext _localctx = new Mnemonic_oContext(_ctx, getState());
		enterRule(_localctx, 46, RULE_mnemonic_o);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(228);
			_la = _input.LA(1);
			if ( !(((((_la - 78)) & ~0x3f) == 0 && ((1L << (_la - 78)) & ((1L << (OR - 78)) | (1L << (ORI - 78)) | (1L << (OUT - 78)))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_pContext extends ParserRuleContext {
		public TerminalNode POP() { return getToken(assemblerParser.POP, 0); }
		public TerminalNode PUSH() { return getToken(assemblerParser.PUSH, 0); }
		public Mnemonic_pContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_p; }
	}

	public final Mnemonic_pContext mnemonic_p() throws RecognitionException {
		Mnemonic_pContext _localctx = new Mnemonic_pContext(_ctx, getState());
		enterRule(_localctx, 48, RULE_mnemonic_p);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(230);
			_la = _input.LA(1);
			if ( !(_la==POP || _la==PUSH) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_rContext extends ParserRuleContext {
		public TerminalNode RCALL() { return getToken(assemblerParser.RCALL, 0); }
		public TerminalNode RET() { return getToken(assemblerParser.RET, 0); }
		public TerminalNode RETI() { return getToken(assemblerParser.RETI, 0); }
		public TerminalNode RJMP() { return getToken(assemblerParser.RJMP, 0); }
		public TerminalNode ROL() { return getToken(assemblerParser.ROL, 0); }
		public TerminalNode ROR() { return getToken(assemblerParser.ROR, 0); }
		public Mnemonic_rContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_r; }
	}

	public final Mnemonic_rContext mnemonic_r() throws RecognitionException {
		Mnemonic_rContext _localctx = new Mnemonic_rContext(_ctx, getState());
		enterRule(_localctx, 50, RULE_mnemonic_r);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(232);
			_la = _input.LA(1);
			if ( !(((((_la - 83)) & ~0x3f) == 0 && ((1L << (_la - 83)) & ((1L << (RCALL - 83)) | (1L << (RET - 83)) | (1L << (RETI - 83)) | (1L << (RJMP - 83)) | (1L << (ROL - 83)) | (1L << (ROR - 83)))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_sContext extends ParserRuleContext {
		public TerminalNode SBC() { return getToken(assemblerParser.SBC, 0); }
		public TerminalNode SBCI() { return getToken(assemblerParser.SBCI, 0); }
		public TerminalNode SBI() { return getToken(assemblerParser.SBI, 0); }
		public TerminalNode SBIC() { return getToken(assemblerParser.SBIC, 0); }
		public TerminalNode SBIS() { return getToken(assemblerParser.SBIS, 0); }
		public TerminalNode SBIW() { return getToken(assemblerParser.SBIW, 0); }
		public TerminalNode SBR() { return getToken(assemblerParser.SBR, 0); }
		public TerminalNode SBRC() { return getToken(assemblerParser.SBRC, 0); }
		public TerminalNode SBRS() { return getToken(assemblerParser.SBRS, 0); }
		public TerminalNode SEC() { return getToken(assemblerParser.SEC, 0); }
		public TerminalNode SEH() { return getToken(assemblerParser.SEH, 0); }
		public TerminalNode SEI() { return getToken(assemblerParser.SEI, 0); }
		public TerminalNode SEN() { return getToken(assemblerParser.SEN, 0); }
		public TerminalNode SER() { return getToken(assemblerParser.SER, 0); }
		public TerminalNode SES() { return getToken(assemblerParser.SES, 0); }
		public TerminalNode SET() { return getToken(assemblerParser.SET, 0); }
		public TerminalNode SEV() { return getToken(assemblerParser.SEV, 0); }
		public TerminalNode SEZ() { return getToken(assemblerParser.SEZ, 0); }
		public TerminalNode SLEEP() { return getToken(assemblerParser.SLEEP, 0); }
		public TerminalNode SPM() { return getToken(assemblerParser.SPM, 0); }
		public TerminalNode ST() { return getToken(assemblerParser.ST, 0); }
		public TerminalNode STS() { return getToken(assemblerParser.STS, 0); }
		public TerminalNode SUB() { return getToken(assemblerParser.SUB, 0); }
		public TerminalNode SUBI() { return getToken(assemblerParser.SUBI, 0); }
		public TerminalNode SWAP() { return getToken(assemblerParser.SWAP, 0); }
		public Mnemonic_sContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_s; }
	}

	public final Mnemonic_sContext mnemonic_s() throws RecognitionException {
		Mnemonic_sContext _localctx = new Mnemonic_sContext(_ctx, getState());
		enterRule(_localctx, 52, RULE_mnemonic_s);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(234);
			_la = _input.LA(1);
			if ( !(((((_la - 89)) & ~0x3f) == 0 && ((1L << (_la - 89)) & ((1L << (SBC - 89)) | (1L << (SBCI - 89)) | (1L << (SBI - 89)) | (1L << (SBIC - 89)) | (1L << (SBIS - 89)) | (1L << (SBIW - 89)) | (1L << (SBR - 89)) | (1L << (SBRC - 89)) | (1L << (SBRS - 89)) | (1L << (SEC - 89)) | (1L << (SEH - 89)) | (1L << (SEI - 89)) | (1L << (SEN - 89)) | (1L << (SER - 89)) | (1L << (SES - 89)) | (1L << (SET - 89)) | (1L << (SEV - 89)) | (1L << (SEZ - 89)) | (1L << (SLEEP - 89)) | (1L << (SPM - 89)) | (1L << (ST - 89)) | (1L << (STS - 89)) | (1L << (SUB - 89)) | (1L << (SUBI - 89)) | (1L << (SWAP - 89)))) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_tContext extends ParserRuleContext {
		public TerminalNode TST() { return getToken(assemblerParser.TST, 0); }
		public Mnemonic_tContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_t; }
	}

	public final Mnemonic_tContext mnemonic_t() throws RecognitionException {
		Mnemonic_tContext _localctx = new Mnemonic_tContext(_ctx, getState());
		enterRule(_localctx, 54, RULE_mnemonic_t);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(236);
			match(TST);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_wContext extends ParserRuleContext {
		public TerminalNode WDR() { return getToken(assemblerParser.WDR, 0); }
		public Mnemonic_wContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_w; }
	}

	public final Mnemonic_wContext mnemonic_w() throws RecognitionException {
		Mnemonic_wContext _localctx = new Mnemonic_wContext(_ctx, getState());
		enterRule(_localctx, 56, RULE_mnemonic_w);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(238);
			match(WDR);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class Mnemonic_xContext extends ParserRuleContext {
		public TerminalNode XCH() { return getToken(assemblerParser.XCH, 0); }
		public Mnemonic_xContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic_x; }
	}

	public final Mnemonic_xContext mnemonic_x() throws RecognitionException {
		Mnemonic_xContext _localctx = new Mnemonic_xContext(_ctx, getState());
		enterRule(_localctx, 58, RULE_mnemonic_x);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(240);
			match(XCH);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public boolean sempred(RuleContext _localctx, int ruleIndex, int predIndex) {
		switch (ruleIndex) {
		case 8:
			return expression_sempred((ExpressionContext)_localctx, predIndex);
		}
		return true;
	}
	private boolean expression_sempred(ExpressionContext _localctx, int predIndex) {
		switch (predIndex) {
		case 0:
			return precpred(_ctx, 11);
		case 1:
			return precpred(_ctx, 10);
		case 2:
			return precpred(_ctx, 9);
		case 3:
			return precpred(_ctx, 8);
		case 4:
			return precpred(_ctx, 7);
		case 5:
			return precpred(_ctx, 6);
		}
		return true;
	}

	public static final String _serializedATN =
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\3\u0097\u00f5\4\2\t"+
		"\2\4\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4\13"+
		"\t\13\4\f\t\f\4\r\t\r\4\16\t\16\4\17\t\17\4\20\t\20\4\21\t\21\4\22\t\22"+
		"\4\23\t\23\4\24\t\24\4\25\t\25\4\26\t\26\4\27\t\27\4\30\t\30\4\31\t\31"+
		"\4\32\t\32\4\33\t\33\4\34\t\34\4\35\t\35\4\36\t\36\4\37\t\37\3\2\7\2@"+
		"\n\2\f\2\16\2C\13\2\3\2\3\2\7\2G\n\2\f\2\16\2J\13\2\3\2\7\2M\n\2\f\2\16"+
		"\2P\13\2\3\2\7\2S\n\2\f\2\16\2V\13\2\3\2\3\2\3\3\3\3\3\3\3\3\5\3^\n\3"+
		"\3\4\3\4\3\4\3\4\5\4d\n\4\5\4f\n\4\3\5\3\5\3\5\3\5\5\5l\n\5\3\6\3\6\7"+
		"\6p\n\6\f\6\16\6s\13\6\3\7\3\7\3\7\3\b\3\b\3\t\3\t\3\t\3\n\3\n\3\n\3\n"+
		"\3\n\3\n\3\n\3\n\3\n\3\n\5\n\u0087\n\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n"+
		"\3\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n\7\n\u009b\n\n\f\n\16\n\u009e"+
		"\13\n\3\13\3\13\3\13\3\13\3\13\3\13\3\13\3\13\3\13\3\13\3\13\3\13\3\13"+
		"\3\13\3\13\3\13\3\13\3\13\3\13\5\13\u00b3\n\13\3\f\3\f\3\f\3\f\5\f\u00b9"+
		"\n\f\3\f\3\f\3\r\3\r\3\r\3\r\3\r\3\r\3\r\3\r\3\r\3\r\3\r\3\r\3\r\3\r\3"+
		"\r\3\r\3\r\3\r\5\r\u00cf\n\r\3\16\3\16\3\17\3\17\3\20\3\20\3\21\3\21\3"+
		"\22\3\22\3\23\3\23\3\24\3\24\3\25\3\25\3\26\3\26\3\27\3\27\3\30\3\30\3"+
		"\31\3\31\3\32\3\32\3\33\3\33\3\34\3\34\3\35\3\35\3\36\3\36\3\37\3\37\3"+
		"\37\2\3\22 \2\4\6\b\n\f\16\20\22\24\26\30\32\34\36 \"$&(*,.\60\62\64\66"+
		"8:<\2\21\3\2\u0096\u0097\3\2\3\7\3\2\b \3\2!\61\3\2\62\63\3\2\64\67\3"+
		"\28:\3\2;>\3\2@H\3\2IM\3\2NO\3\2PR\3\2ST\3\2UZ\3\2[s\2\u010a\2A\3\2\2"+
		"\2\4]\3\2\2\2\6_\3\2\2\2\bk\3\2\2\2\nm\3\2\2\2\ft\3\2\2\2\16w\3\2\2\2"+
		"\20y\3\2\2\2\22\u0086\3\2\2\2\24\u009f\3\2\2\2\26\u00b4\3\2\2\2\30\u00ce"+
		"\3\2\2\2\32\u00d0\3\2\2\2\34\u00d2\3\2\2\2\36\u00d4\3\2\2\2 \u00d6\3\2"+
		"\2\2\"\u00d8\3\2\2\2$\u00da\3\2\2\2&\u00dc\3\2\2\2(\u00de\3\2\2\2*\u00e0"+
		"\3\2\2\2,\u00e2\3\2\2\2.\u00e4\3\2\2\2\60\u00e6\3\2\2\2\62\u00e8\3\2\2"+
		"\2\64\u00ea\3\2\2\2\66\u00ec\3\2\2\28\u00ee\3\2\2\2:\u00f0\3\2\2\2<\u00f2"+
		"\3\2\2\2>@\7\u0091\2\2?>\3\2\2\2@C\3\2\2\2A?\3\2\2\2AB\3\2\2\2BD\3\2\2"+
		"\2CA\3\2\2\2DN\5\4\3\2EG\7\u0091\2\2FE\3\2\2\2GJ\3\2\2\2HF\3\2\2\2HI\3"+
		"\2\2\2IK\3\2\2\2JH\3\2\2\2KM\5\4\3\2LH\3\2\2\2MP\3\2\2\2NL\3\2\2\2NO\3"+
		"\2\2\2OT\3\2\2\2PN\3\2\2\2QS\7\u0091\2\2RQ\3\2\2\2SV\3\2\2\2TR\3\2\2\2"+
		"TU\3\2\2\2UW\3\2\2\2VT\3\2\2\2WX\7\2\2\3X\3\3\2\2\2Y^\5\24\13\2Z^\5\n"+
		"\6\2[^\5\f\7\2\\^\5\6\4\2]Y\3\2\2\2]Z\3\2\2\2][\3\2\2\2]\\\3\2\2\2^\5"+
		"\3\2\2\2_e\5\30\r\2`c\5\b\5\2ab\7{\2\2bd\5\b\5\2ca\3\2\2\2cd\3\2\2\2d"+
		"f\3\2\2\2e`\3\2\2\2ef\3\2\2\2f\7\3\2\2\2gl\7\u0097\2\2hl\5\22\n\2il\5"+
		"\26\f\2jl\5\20\t\2kg\3\2\2\2kh\3\2\2\2ki\3\2\2\2kj\3\2\2\2l\t\3\2\2\2"+
		"mq\7\u0097\2\2np\5\22\n\2on\3\2\2\2ps\3\2\2\2qo\3\2\2\2qr\3\2\2\2r\13"+
		"\3\2\2\2sq\3\2\2\2tu\7\u0097\2\2uv\7z\2\2v\r\3\2\2\2wx\7\u0097\2\2x\17"+
		"\3\2\2\2yz\7x\2\2z{\7\u0095\2\2{\21\3\2\2\2|}\b\n\1\2}~\7\u008c\2\2~\177"+
		"\5\22\n\2\177\u0080\7y\2\2\u0080\u0087\3\2\2\2\u0081\u0087\7\u0095\2\2"+
		"\u0082\u0087\7\u0096\2\2\u0083\u0087\7\u0097\2\2\u0084\u0087\7\u0094\2"+
		"\2\u0085\u0087\5\20\t\2\u0086|\3\2\2\2\u0086\u0081\3\2\2\2\u0086\u0082"+
		"\3\2\2\2\u0086\u0083\3\2\2\2\u0086\u0084\3\2\2\2\u0086\u0085\3\2\2\2\u0087"+
		"\u009c\3\2\2\2\u0088\u0089\f\r\2\2\u0089\u008a\7\u0088\2\2\u008a\u009b"+
		"\5\22\n\16\u008b\u008c\f\f\2\2\u008c\u008d\7\u008f\2\2\u008d\u009b\5\22"+
		"\n\r\u008e\u008f\f\13\2\2\u008f\u0090\7\u0090\2\2\u0090\u009b\5\22\n\f"+
		"\u0091\u0092\f\n\2\2\u0092\u0093\7\u0085\2\2\u0093\u009b\5\22\n\13\u0094"+
		"\u0095\f\t\2\2\u0095\u0096\7\u0089\2\2\u0096\u009b\5\22\n\n\u0097\u0098"+
		"\f\b\2\2\u0098\u0099\7\u0082\2\2\u0099\u009b\5\22\n\t\u009a\u0088\3\2"+
		"\2\2\u009a\u008b\3\2\2\2\u009a\u008e\3\2\2\2\u009a\u0091\3\2\2\2\u009a"+
		"\u0094\3\2\2\2\u009a\u0097\3\2\2\2\u009b\u009e\3\2\2\2\u009c\u009a\3\2"+
		"\2\2\u009c\u009d\3\2\2\2\u009d\23\3\2\2\2\u009e\u009c\3\2\2\2\u009f\u00b2"+
		"\7~\2\2\u00a0\u00a1\7\u0087\2\2\u00a1\u00b3\7\u0094\2\2\u00a2\u00a3\7"+
		"}\2\2\u00a3\u00b3\5\22\n\2\u00a4\u00a5\7\u0083\2\2\u00a5\u00b3\5\22\n"+
		"\2\u00a6\u00b3\7|\2\2\u00a7\u00a8\7\u008d\2\2\u00a8\u00b3\t\2\2\2\u00a9"+
		"\u00aa\7\u008a\2\2\u00aa\u00b3\7\u0097\2\2\u00ab\u00b3\7\u0080\2\2\u00ac"+
		"\u00ad\7\u0086\2\2\u00ad\u00b3\5\22\n\2\u00ae\u00b3\7\177\2\2\u00af\u00b3"+
		"\7\u0081\2\2\u00b0\u00b1\7\u0084\2\2\u00b1\u00b3\7\u0094\2\2\u00b2\u00a0"+
		"\3\2\2\2\u00b2\u00a2\3\2\2\2\u00b2\u00a4\3\2\2\2\u00b2\u00a6\3\2\2\2\u00b2"+
		"\u00a7\3\2\2\2\u00b2\u00a9\3\2\2\2\u00b2\u00ab\3\2\2\2\u00b2\u00ac\3\2"+
		"\2\2\u00b2\u00ae\3\2\2\2\u00b2\u00af\3\2\2\2\u00b2\u00b0\3\2\2\2\u00b3"+
		"\25\3\2\2\2\u00b4\u00b5\7\u0097\2\2\u00b5\u00b8\7\u008c\2\2\u00b6\u00b9"+
		"\7\u0097\2\2\u00b7\u00b9\5\20\t\2\u00b8\u00b6\3\2\2\2\u00b8\u00b7\3\2"+
		"\2\2\u00b9\u00ba\3\2\2\2\u00ba\u00bb\7y\2\2\u00bb\27\3\2\2\2\u00bc\u00cf"+
		"\5\32\16\2\u00bd\u00cf\5\34\17\2\u00be\u00cf\5\36\20\2\u00bf\u00cf\5 "+
		"\21\2\u00c0\u00cf\5\"\22\2\u00c1\u00cf\5$\23\2\u00c2\u00cf\5&\24\2\u00c3"+
		"\u00cf\5(\25\2\u00c4\u00cf\5*\26\2\u00c5\u00cf\5,\27\2\u00c6\u00cf\5."+
		"\30\2\u00c7\u00cf\5\60\31\2\u00c8\u00cf\5\62\32\2\u00c9\u00cf\5\64\33"+
		"\2\u00ca\u00cf\5\66\34\2\u00cb\u00cf\58\35\2\u00cc\u00cf\5:\36\2\u00cd"+
		"\u00cf\5<\37\2\u00ce\u00bc\3\2\2\2\u00ce\u00bd\3\2\2\2\u00ce\u00be\3\2"+
		"\2\2\u00ce\u00bf\3\2\2\2\u00ce\u00c0\3\2\2\2\u00ce\u00c1\3\2\2\2\u00ce"+
		"\u00c2\3\2\2\2\u00ce\u00c3\3\2\2\2\u00ce\u00c4\3\2\2\2\u00ce\u00c5\3\2"+
		"\2\2\u00ce\u00c6\3\2\2\2\u00ce\u00c7\3\2\2\2\u00ce\u00c8\3\2\2\2\u00ce"+
		"\u00c9\3\2\2\2\u00ce\u00ca\3\2\2\2\u00ce\u00cb\3\2\2\2\u00ce\u00cc\3\2"+
		"\2\2\u00ce\u00cd\3\2\2\2\u00cf\31\3\2\2\2\u00d0\u00d1\t\3\2\2\u00d1\33"+
		"\3\2\2\2\u00d2\u00d3\t\4\2\2\u00d3\35\3\2\2\2\u00d4\u00d5\t\5\2\2\u00d5"+
		"\37\3\2\2\2\u00d6\u00d7\t\6\2\2\u00d7!\3\2\2\2\u00d8\u00d9\t\7\2\2\u00d9"+
		"#\3\2\2\2\u00da\u00db\t\b\2\2\u00db%\3\2\2\2\u00dc\u00dd\t\t\2\2\u00dd"+
		"\'\3\2\2\2\u00de\u00df\7?\2\2\u00df)\3\2\2\2\u00e0\u00e1\t\n\2\2\u00e1"+
		"+\3\2\2\2\u00e2\u00e3\t\13\2\2\u00e3-\3\2\2\2\u00e4\u00e5\t\f\2\2\u00e5"+
		"/\3\2\2\2\u00e6\u00e7\t\r\2\2\u00e7\61\3\2\2\2\u00e8\u00e9\t\16\2\2\u00e9"+
		"\63\3\2\2\2\u00ea\u00eb\t\17\2\2\u00eb\65\3\2\2\2\u00ec\u00ed\t\20\2\2"+
		"\u00ed\67\3\2\2\2\u00ee\u00ef\7t\2\2\u00ef9\3\2\2\2\u00f0\u00f1\7u\2\2"+
		"\u00f1;\3\2\2\2\u00f2\u00f3\7v\2\2\u00f3=\3\2\2\2\21AHNT]cekq\u0086\u009a"+
		"\u009c\u00b2\u00b8\u00ce";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
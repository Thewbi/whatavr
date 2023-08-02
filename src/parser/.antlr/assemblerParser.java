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
		ADD=1, CALL=2, CLR=3, EOR=4, LDI=5, OUT=6, POP=7, PUSH=8, RCALL=9, RET=10, 
		RJMP=11, NEWLINE=12, WS=13, LINE_COMMENT=14, STRING=15, ASTERISK=16, AT=17, 
		CLOSEING_BRACKET=18, COLON=19, COMMA=20, CSEG=21, DEF=22, DOT=23, ELSE=24, 
		END_MACRO=25, ENDIF=26, EQUALS=27, EQU=28, ERROR=29, GT=30, IF=31, INCLUDE=32, 
		LEFT_SHIFT=33, LT=34, MACRO=35, MINUS=36, OPENING_BRACKET=37, ORG=38, 
		PLUS=39, RIGHT_SHIFT=40, SLASH=41, NUMBER=42, HEX_NUMBER=43, IDENTIFIER=44;
	public static final int
		RULE_asm_file = 0, RULE_row = 1, RULE_instruction = 2, RULE_macro_usage = 3, 
		RULE_label_definition = 4, RULE_parameter = 5, RULE_macro_placeholder = 6, 
		RULE_expression = 7, RULE_asm_instrinsic_instruction = 8, RULE_asm_intrinsic_usage = 9, 
		RULE_mnemonic = 10;
	private static String[] makeRuleNames() {
		return new String[] {
			"asm_file", "row", "instruction", "macro_usage", "label_definition", 
			"parameter", "macro_placeholder", "expression", "asm_instrinsic_instruction", 
			"asm_intrinsic_usage", "mnemonic"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, "'*'", "'@'", "')'", "':'", "','", "'cseg'", 
			"'def'", "'.'", "'else'", "'endmacro'", "'endif'", "'='", "'equ'", "'error'", 
			"'>'", "'if'", "'include'", "'<<'", "'<'", "'macro'", "'-'", "'('", "'org'", 
			"'+'", "'>>'", "'/'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "ADD", "CALL", "CLR", "EOR", "LDI", "OUT", "POP", "PUSH", "RCALL", 
			"RET", "RJMP", "NEWLINE", "WS", "LINE_COMMENT", "STRING", "ASTERISK", 
			"AT", "CLOSEING_BRACKET", "COLON", "COMMA", "CSEG", "DEF", "DOT", "ELSE", 
			"END_MACRO", "ENDIF", "EQUALS", "EQU", "ERROR", "GT", "IF", "INCLUDE", 
			"LEFT_SHIFT", "LT", "MACRO", "MINUS", "OPENING_BRACKET", "ORG", "PLUS", 
			"RIGHT_SHIFT", "SLASH", "NUMBER", "HEX_NUMBER", "IDENTIFIER"
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
			setState(25);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==NEWLINE) {
				{
				{
				setState(22);
				match(NEWLINE);
				}
				}
				setState(27);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(28);
			row();
			setState(38);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(32);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==NEWLINE) {
						{
						{
						setState(29);
						match(NEWLINE);
						}
						}
						setState(34);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					setState(35);
					row();
					}
					} 
				}
				setState(40);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			}
			setState(44);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==NEWLINE) {
				{
				{
				setState(41);
				match(NEWLINE);
				}
				}
				setState(46);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(47);
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
		public Macro_usageContext macro_usage() {
			return getRuleContext(Macro_usageContext.class,0);
		}
		public Label_definitionContext label_definition() {
			return getRuleContext(Label_definitionContext.class,0);
		}
		public InstructionContext instruction() {
			return getRuleContext(InstructionContext.class,0);
		}
		public Asm_instrinsic_instructionContext asm_instrinsic_instruction() {
			return getRuleContext(Asm_instrinsic_instructionContext.class,0);
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
			setState(53);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,4,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(49);
				macro_usage();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(50);
				label_definition();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(51);
				instruction();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(52);
				asm_instrinsic_instruction();
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
		public List<TerminalNode> IDENTIFIER() { return getTokens(assemblerParser.IDENTIFIER); }
		public TerminalNode IDENTIFIER(int i) {
			return getToken(assemblerParser.IDENTIFIER, i);
		}
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public List<Asm_intrinsic_usageContext> asm_intrinsic_usage() {
			return getRuleContexts(Asm_intrinsic_usageContext.class);
		}
		public Asm_intrinsic_usageContext asm_intrinsic_usage(int i) {
			return getRuleContext(Asm_intrinsic_usageContext.class,i);
		}
		public List<Macro_placeholderContext> macro_placeholder() {
			return getRuleContexts(Macro_placeholderContext.class);
		}
		public Macro_placeholderContext macro_placeholder(int i) {
			return getRuleContext(Macro_placeholderContext.class,i);
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
			setState(55);
			mnemonic();
			setState(71);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,8,_ctx) ) {
			case 1:
				{
				setState(60);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,5,_ctx) ) {
				case 1:
					{
					setState(56);
					match(IDENTIFIER);
					}
					break;
				case 2:
					{
					setState(57);
					expression(0);
					}
					break;
				case 3:
					{
					setState(58);
					asm_intrinsic_usage();
					}
					break;
				case 4:
					{
					setState(59);
					macro_placeholder();
					}
					break;
				}
				setState(69);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==COMMA) {
					{
					setState(62);
					match(COMMA);
					setState(67);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,6,_ctx) ) {
					case 1:
						{
						setState(63);
						match(IDENTIFIER);
						}
						break;
					case 2:
						{
						setState(64);
						expression(0);
						}
						break;
					case 3:
						{
						setState(65);
						asm_intrinsic_usage();
						}
						break;
					case 4:
						{
						setState(66);
						macro_placeholder();
						}
						break;
					}
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
		enterRule(_localctx, 6, RULE_macro_usage);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(73);
			match(IDENTIFIER);
			setState(77);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,9,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(74);
					expression(0);
					}
					} 
				}
				setState(79);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,9,_ctx);
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
		enterRule(_localctx, 8, RULE_label_definition);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(80);
			match(IDENTIFIER);
			setState(81);
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
		enterRule(_localctx, 10, RULE_parameter);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(83);
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
		enterRule(_localctx, 12, RULE_macro_placeholder);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(85);
			match(AT);
			setState(86);
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
		public TerminalNode NUMBER() { return getToken(assemblerParser.NUMBER, 0); }
		public TerminalNode HEX_NUMBER() { return getToken(assemblerParser.HEX_NUMBER, 0); }
		public TerminalNode IDENTIFIER() { return getToken(assemblerParser.IDENTIFIER, 0); }
		public Macro_placeholderContext macro_placeholder() {
			return getRuleContext(Macro_placeholderContext.class,0);
		}
		public TerminalNode OPENING_BRACKET() { return getToken(assemblerParser.OPENING_BRACKET, 0); }
		public List<ExpressionContext> expression() {
			return getRuleContexts(ExpressionContext.class);
		}
		public ExpressionContext expression(int i) {
			return getRuleContext(ExpressionContext.class,i);
		}
		public TerminalNode CLOSEING_BRACKET() { return getToken(assemblerParser.CLOSEING_BRACKET, 0); }
		public TerminalNode LEFT_SHIFT() { return getToken(assemblerParser.LEFT_SHIFT, 0); }
		public TerminalNode RIGHT_SHIFT() { return getToken(assemblerParser.RIGHT_SHIFT, 0); }
		public TerminalNode SLASH() { return getToken(assemblerParser.SLASH, 0); }
		public TerminalNode GT() { return getToken(assemblerParser.GT, 0); }
		public TerminalNode LT() { return getToken(assemblerParser.LT, 0); }
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
		int _startState = 14;
		enterRecursionRule(_localctx, 14, RULE_expression, _p);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(97);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case NUMBER:
				{
				setState(89);
				match(NUMBER);
				}
				break;
			case HEX_NUMBER:
				{
				setState(90);
				match(HEX_NUMBER);
				}
				break;
			case IDENTIFIER:
				{
				setState(91);
				match(IDENTIFIER);
				}
				break;
			case AT:
				{
				setState(92);
				macro_placeholder();
				}
				break;
			case OPENING_BRACKET:
				{
				setState(93);
				match(OPENING_BRACKET);
				setState(94);
				expression(0);
				setState(95);
				match(CLOSEING_BRACKET);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			_ctx.stop = _input.LT(-1);
			setState(116);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,12,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(114);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,11,_ctx) ) {
					case 1:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(99);
						if (!(precpred(_ctx, 5))) throw new FailedPredicateException(this, "precpred(_ctx, 5)");
						setState(100);
						match(LEFT_SHIFT);
						setState(101);
						expression(6);
						}
						break;
					case 2:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(102);
						if (!(precpred(_ctx, 4))) throw new FailedPredicateException(this, "precpred(_ctx, 4)");
						setState(103);
						match(RIGHT_SHIFT);
						setState(104);
						expression(5);
						}
						break;
					case 3:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(105);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(106);
						match(SLASH);
						setState(107);
						expression(4);
						}
						break;
					case 4:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(108);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(109);
						match(GT);
						setState(110);
						expression(3);
						}
						break;
					case 5:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(111);
						if (!(precpred(_ctx, 1))) throw new FailedPredicateException(this, "precpred(_ctx, 1)");
						setState(112);
						match(LT);
						setState(113);
						expression(2);
						}
						break;
					}
					} 
				}
				setState(118);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,12,_ctx);
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
		public TerminalNode DEF() { return getToken(assemblerParser.DEF, 0); }
		public List<TerminalNode> IDENTIFIER() { return getTokens(assemblerParser.IDENTIFIER); }
		public TerminalNode IDENTIFIER(int i) {
			return getToken(assemblerParser.IDENTIFIER, i);
		}
		public TerminalNode EQUALS() { return getToken(assemblerParser.EQUALS, 0); }
		public TerminalNode EQU() { return getToken(assemblerParser.EQU, 0); }
		public TerminalNode CSEG() { return getToken(assemblerParser.CSEG, 0); }
		public TerminalNode ORG() { return getToken(assemblerParser.ORG, 0); }
		public TerminalNode HEX_NUMBER() { return getToken(assemblerParser.HEX_NUMBER, 0); }
		public TerminalNode MACRO() { return getToken(assemblerParser.MACRO, 0); }
		public TerminalNode END_MACRO() { return getToken(assemblerParser.END_MACRO, 0); }
		public TerminalNode IF() { return getToken(assemblerParser.IF, 0); }
		public ExpressionContext expression() {
			return getRuleContext(ExpressionContext.class,0);
		}
		public TerminalNode ELSE() { return getToken(assemblerParser.ELSE, 0); }
		public TerminalNode ENDIF() { return getToken(assemblerParser.ENDIF, 0); }
		public TerminalNode ERROR() { return getToken(assemblerParser.ERROR, 0); }
		public TerminalNode STRING() { return getToken(assemblerParser.STRING, 0); }
		public TerminalNode INCLUDE() { return getToken(assemblerParser.INCLUDE, 0); }
		public Asm_instrinsic_instructionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_asm_instrinsic_instruction; }
	}

	public final Asm_instrinsic_instructionContext asm_instrinsic_instruction() throws RecognitionException {
		Asm_instrinsic_instructionContext _localctx = new Asm_instrinsic_instructionContext(_ctx, getState());
		enterRule(_localctx, 16, RULE_asm_instrinsic_instruction);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(119);
			match(DOT);
			setState(148);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INCLUDE:
				{
				{
				setState(120);
				match(INCLUDE);
				setState(121);
				match(STRING);
				}
				}
				break;
			case DEF:
				{
				setState(122);
				match(DEF);
				setState(123);
				match(IDENTIFIER);
				setState(124);
				match(EQUALS);
				setState(127);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,13,_ctx) ) {
				case 1:
					{
					setState(125);
					expression(0);
					}
					break;
				case 2:
					{
					setState(126);
					match(IDENTIFIER);
					}
					break;
				}
				}
				break;
			case EQU:
				{
				setState(129);
				match(EQU);
				setState(130);
				match(IDENTIFIER);
				setState(131);
				match(EQUALS);
				setState(134);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,14,_ctx) ) {
				case 1:
					{
					setState(132);
					expression(0);
					}
					break;
				case 2:
					{
					setState(133);
					match(IDENTIFIER);
					}
					break;
				}
				}
				break;
			case CSEG:
				{
				setState(136);
				match(CSEG);
				}
				break;
			case ORG:
				{
				setState(137);
				match(ORG);
				setState(138);
				match(HEX_NUMBER);
				}
				break;
			case MACRO:
				{
				setState(139);
				match(MACRO);
				setState(140);
				match(IDENTIFIER);
				}
				break;
			case END_MACRO:
				{
				setState(141);
				match(END_MACRO);
				}
				break;
			case IF:
				{
				setState(142);
				match(IF);
				setState(143);
				expression(0);
				}
				break;
			case ELSE:
				{
				setState(144);
				match(ELSE);
				}
				break;
			case ENDIF:
				{
				setState(145);
				match(ENDIF);
				}
				break;
			case ERROR:
				{
				setState(146);
				match(ERROR);
				setState(147);
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
		enterRule(_localctx, 18, RULE_asm_intrinsic_usage);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(150);
			match(IDENTIFIER);
			setState(151);
			match(OPENING_BRACKET);
			setState(154);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case IDENTIFIER:
				{
				setState(152);
				match(IDENTIFIER);
				}
				break;
			case AT:
				{
				setState(153);
				macro_placeholder();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			setState(156);
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
		public TerminalNode ADD() { return getToken(assemblerParser.ADD, 0); }
		public TerminalNode CALL() { return getToken(assemblerParser.CALL, 0); }
		public TerminalNode CLR() { return getToken(assemblerParser.CLR, 0); }
		public TerminalNode EOR() { return getToken(assemblerParser.EOR, 0); }
		public TerminalNode LDI() { return getToken(assemblerParser.LDI, 0); }
		public TerminalNode OUT() { return getToken(assemblerParser.OUT, 0); }
		public TerminalNode POP() { return getToken(assemblerParser.POP, 0); }
		public TerminalNode PUSH() { return getToken(assemblerParser.PUSH, 0); }
		public TerminalNode RCALL() { return getToken(assemblerParser.RCALL, 0); }
		public TerminalNode RET() { return getToken(assemblerParser.RET, 0); }
		public TerminalNode RJMP() { return getToken(assemblerParser.RJMP, 0); }
		public MnemonicContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mnemonic; }
	}

	public final MnemonicContext mnemonic() throws RecognitionException {
		MnemonicContext _localctx = new MnemonicContext(_ctx, getState());
		enterRule(_localctx, 20, RULE_mnemonic);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(158);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << ADD) | (1L << CALL) | (1L << CLR) | (1L << EOR) | (1L << LDI) | (1L << OUT) | (1L << POP) | (1L << PUSH) | (1L << RCALL) | (1L << RET) | (1L << RJMP))) != 0)) ) {
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

	public boolean sempred(RuleContext _localctx, int ruleIndex, int predIndex) {
		switch (ruleIndex) {
		case 7:
			return expression_sempred((ExpressionContext)_localctx, predIndex);
		}
		return true;
	}
	private boolean expression_sempred(ExpressionContext _localctx, int predIndex) {
		switch (predIndex) {
		case 0:
			return precpred(_ctx, 5);
		case 1:
			return precpred(_ctx, 4);
		case 2:
			return precpred(_ctx, 3);
		case 3:
			return precpred(_ctx, 2);
		case 4:
			return precpred(_ctx, 1);
		}
		return true;
	}

	public static final String _serializedATN =
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\3.\u00a3\4\2\t\2\4"+
		"\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4\13\t"+
		"\13\4\f\t\f\3\2\7\2\32\n\2\f\2\16\2\35\13\2\3\2\3\2\7\2!\n\2\f\2\16\2"+
		"$\13\2\3\2\7\2\'\n\2\f\2\16\2*\13\2\3\2\7\2-\n\2\f\2\16\2\60\13\2\3\2"+
		"\3\2\3\3\3\3\3\3\3\3\5\38\n\3\3\4\3\4\3\4\3\4\3\4\5\4?\n\4\3\4\3\4\3\4"+
		"\3\4\3\4\5\4F\n\4\5\4H\n\4\5\4J\n\4\3\5\3\5\7\5N\n\5\f\5\16\5Q\13\5\3"+
		"\6\3\6\3\6\3\7\3\7\3\b\3\b\3\b\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\5\t"+
		"d\n\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\7\t"+
		"u\n\t\f\t\16\tx\13\t\3\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n\5\n\u0082\n\n\3\n"+
		"\3\n\3\n\3\n\3\n\5\n\u0089\n\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n"+
		"\3\n\3\n\5\n\u0097\n\n\3\13\3\13\3\13\3\13\5\13\u009d\n\13\3\13\3\13\3"+
		"\f\3\f\3\f\2\3\20\r\2\4\6\b\n\f\16\20\22\24\26\2\3\3\2\3\r\2\u00bd\2\33"+
		"\3\2\2\2\4\67\3\2\2\2\69\3\2\2\2\bK\3\2\2\2\nR\3\2\2\2\fU\3\2\2\2\16W"+
		"\3\2\2\2\20c\3\2\2\2\22y\3\2\2\2\24\u0098\3\2\2\2\26\u00a0\3\2\2\2\30"+
		"\32\7\16\2\2\31\30\3\2\2\2\32\35\3\2\2\2\33\31\3\2\2\2\33\34\3\2\2\2\34"+
		"\36\3\2\2\2\35\33\3\2\2\2\36(\5\4\3\2\37!\7\16\2\2 \37\3\2\2\2!$\3\2\2"+
		"\2\" \3\2\2\2\"#\3\2\2\2#%\3\2\2\2$\"\3\2\2\2%\'\5\4\3\2&\"\3\2\2\2\'"+
		"*\3\2\2\2(&\3\2\2\2()\3\2\2\2).\3\2\2\2*(\3\2\2\2+-\7\16\2\2,+\3\2\2\2"+
		"-\60\3\2\2\2.,\3\2\2\2./\3\2\2\2/\61\3\2\2\2\60.\3\2\2\2\61\62\7\2\2\3"+
		"\62\3\3\2\2\2\638\5\b\5\2\648\5\n\6\2\658\5\6\4\2\668\5\22\n\2\67\63\3"+
		"\2\2\2\67\64\3\2\2\2\67\65\3\2\2\2\67\66\3\2\2\28\5\3\2\2\29I\5\26\f\2"+
		":?\7.\2\2;?\5\20\t\2<?\5\24\13\2=?\5\16\b\2>:\3\2\2\2>;\3\2\2\2><\3\2"+
		"\2\2>=\3\2\2\2?G\3\2\2\2@E\7\26\2\2AF\7.\2\2BF\5\20\t\2CF\5\24\13\2DF"+
		"\5\16\b\2EA\3\2\2\2EB\3\2\2\2EC\3\2\2\2ED\3\2\2\2FH\3\2\2\2G@\3\2\2\2"+
		"GH\3\2\2\2HJ\3\2\2\2I>\3\2\2\2IJ\3\2\2\2J\7\3\2\2\2KO\7.\2\2LN\5\20\t"+
		"\2ML\3\2\2\2NQ\3\2\2\2OM\3\2\2\2OP\3\2\2\2P\t\3\2\2\2QO\3\2\2\2RS\7.\2"+
		"\2ST\7\25\2\2T\13\3\2\2\2UV\7.\2\2V\r\3\2\2\2WX\7\23\2\2XY\7,\2\2Y\17"+
		"\3\2\2\2Z[\b\t\1\2[d\7,\2\2\\d\7-\2\2]d\7.\2\2^d\5\16\b\2_`\7\'\2\2`a"+
		"\5\20\t\2ab\7\24\2\2bd\3\2\2\2cZ\3\2\2\2c\\\3\2\2\2c]\3\2\2\2c^\3\2\2"+
		"\2c_\3\2\2\2dv\3\2\2\2ef\f\7\2\2fg\7#\2\2gu\5\20\t\bhi\f\6\2\2ij\7*\2"+
		"\2ju\5\20\t\7kl\f\5\2\2lm\7+\2\2mu\5\20\t\6no\f\4\2\2op\7 \2\2pu\5\20"+
		"\t\5qr\f\3\2\2rs\7$\2\2su\5\20\t\4te\3\2\2\2th\3\2\2\2tk\3\2\2\2tn\3\2"+
		"\2\2tq\3\2\2\2ux\3\2\2\2vt\3\2\2\2vw\3\2\2\2w\21\3\2\2\2xv\3\2\2\2y\u0096"+
		"\7\31\2\2z{\7\"\2\2{\u0097\7\21\2\2|}\7\30\2\2}~\7.\2\2~\u0081\7\35\2"+
		"\2\177\u0082\5\20\t\2\u0080\u0082\7.\2\2\u0081\177\3\2\2\2\u0081\u0080"+
		"\3\2\2\2\u0082\u0097\3\2\2\2\u0083\u0084\7\36\2\2\u0084\u0085\7.\2\2\u0085"+
		"\u0088\7\35\2\2\u0086\u0089\5\20\t\2\u0087\u0089\7.\2\2\u0088\u0086\3"+
		"\2\2\2\u0088\u0087\3\2\2\2\u0089\u0097\3\2\2\2\u008a\u0097\7\27\2\2\u008b"+
		"\u008c\7(\2\2\u008c\u0097\7-\2\2\u008d\u008e\7%\2\2\u008e\u0097\7.\2\2"+
		"\u008f\u0097\7\33\2\2\u0090\u0091\7!\2\2\u0091\u0097\5\20\t\2\u0092\u0097"+
		"\7\32\2\2\u0093\u0097\7\34\2\2\u0094\u0095\7\37\2\2\u0095\u0097\7\21\2"+
		"\2\u0096z\3\2\2\2\u0096|\3\2\2\2\u0096\u0083\3\2\2\2\u0096\u008a\3\2\2"+
		"\2\u0096\u008b\3\2\2\2\u0096\u008d\3\2\2\2\u0096\u008f\3\2\2\2\u0096\u0090"+
		"\3\2\2\2\u0096\u0092\3\2\2\2\u0096\u0093\3\2\2\2\u0096\u0094\3\2\2\2\u0097"+
		"\23\3\2\2\2\u0098\u0099\7.\2\2\u0099\u009c\7\'\2\2\u009a\u009d\7.\2\2"+
		"\u009b\u009d\5\16\b\2\u009c\u009a\3\2\2\2\u009c\u009b\3\2\2\2\u009d\u009e"+
		"\3\2\2\2\u009e\u009f\7\24\2\2\u009f\25\3\2\2\2\u00a0\u00a1\t\2\2\2\u00a1"+
		"\27\3\2\2\2\23\33\"(.\67>EGIOctv\u0081\u0088\u0096\u009c";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
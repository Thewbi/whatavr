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
		RULE_asm_file = 0, RULE_row = 1, RULE_macro_usage = 2, RULE_label_definition = 3, 
		RULE_parameter = 4, RULE_macro_placeholder = 5, RULE_expression = 6, RULE_asm_instrinsic_instruction = 7, 
		RULE_asm_intrinsic_usage = 8, RULE_instruction = 9;
	private static String[] makeRuleNames() {
		return new String[] {
			"asm_file", "row", "macro_usage", "label_definition", "parameter", "macro_placeholder", 
			"expression", "asm_instrinsic_instruction", "asm_intrinsic_usage", "instruction"
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
			setState(23);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==NEWLINE) {
				{
				{
				setState(20);
				match(NEWLINE);
				}
				}
				setState(25);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(26);
			row();
			setState(36);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(30);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==NEWLINE) {
						{
						{
						setState(27);
						match(NEWLINE);
						}
						}
						setState(32);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					setState(33);
					row();
					}
					} 
				}
				setState(38);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			}
			setState(42);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==NEWLINE) {
				{
				{
				setState(39);
				match(NEWLINE);
				}
				}
				setState(44);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(45);
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
		public Label_definitionContext label_definition() {
			return getRuleContext(Label_definitionContext.class,0);
		}
		public InstructionContext instruction() {
			return getRuleContext(InstructionContext.class,0);
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
		public Asm_instrinsic_instructionContext asm_instrinsic_instruction() {
			return getRuleContext(Asm_instrinsic_instructionContext.class,0);
		}
		public Macro_usageContext macro_usage() {
			return getRuleContext(Macro_usageContext.class,0);
		}
		public RowContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_row; }
	}

	public final RowContext row() throws RecognitionException {
		RowContext _localctx = new RowContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_row);
		int _la;
		try {
			setState(68);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,8,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(47);
				label_definition();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(48);
				instruction();
				setState(64);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,7,_ctx) ) {
				case 1:
					{
					setState(53);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,4,_ctx) ) {
					case 1:
						{
						setState(49);
						match(IDENTIFIER);
						}
						break;
					case 2:
						{
						setState(50);
						expression(0);
						}
						break;
					case 3:
						{
						setState(51);
						asm_intrinsic_usage();
						}
						break;
					case 4:
						{
						setState(52);
						macro_placeholder();
						}
						break;
					}
					setState(62);
					_errHandler.sync(this);
					_la = _input.LA(1);
					if (_la==COMMA) {
						{
						setState(55);
						match(COMMA);
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
						}
					}

					}
					break;
				}
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(66);
				asm_instrinsic_instruction();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(67);
				macro_usage();
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
		enterRule(_localctx, 4, RULE_macro_usage);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(70);
			match(IDENTIFIER);
			setState(74);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,9,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(71);
					expression(0);
					}
					} 
				}
				setState(76);
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
		enterRule(_localctx, 6, RULE_label_definition);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(77);
			match(IDENTIFIER);
			setState(78);
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
		enterRule(_localctx, 8, RULE_parameter);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(80);
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
		enterRule(_localctx, 10, RULE_macro_placeholder);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(82);
			match(AT);
			setState(83);
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
		int _startState = 12;
		enterRecursionRule(_localctx, 12, RULE_expression, _p);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(94);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case NUMBER:
				{
				setState(86);
				match(NUMBER);
				}
				break;
			case HEX_NUMBER:
				{
				setState(87);
				match(HEX_NUMBER);
				}
				break;
			case IDENTIFIER:
				{
				setState(88);
				match(IDENTIFIER);
				}
				break;
			case AT:
				{
				setState(89);
				macro_placeholder();
				}
				break;
			case OPENING_BRACKET:
				{
				setState(90);
				match(OPENING_BRACKET);
				setState(91);
				expression(0);
				setState(92);
				match(CLOSEING_BRACKET);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			_ctx.stop = _input.LT(-1);
			setState(113);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,12,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(111);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,11,_ctx) ) {
					case 1:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(96);
						if (!(precpred(_ctx, 5))) throw new FailedPredicateException(this, "precpred(_ctx, 5)");
						setState(97);
						match(LEFT_SHIFT);
						setState(98);
						expression(6);
						}
						break;
					case 2:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(99);
						if (!(precpred(_ctx, 4))) throw new FailedPredicateException(this, "precpred(_ctx, 4)");
						setState(100);
						match(RIGHT_SHIFT);
						setState(101);
						expression(5);
						}
						break;
					case 3:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(102);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(103);
						match(SLASH);
						setState(104);
						expression(4);
						}
						break;
					case 4:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(105);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(106);
						match(GT);
						setState(107);
						expression(3);
						}
						break;
					case 5:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(108);
						if (!(precpred(_ctx, 1))) throw new FailedPredicateException(this, "precpred(_ctx, 1)");
						setState(109);
						match(LT);
						setState(110);
						expression(2);
						}
						break;
					}
					} 
				}
				setState(115);
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
		enterRule(_localctx, 14, RULE_asm_instrinsic_instruction);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(116);
			match(DOT);
			setState(145);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INCLUDE:
				{
				{
				setState(117);
				match(INCLUDE);
				setState(118);
				match(STRING);
				}
				}
				break;
			case DEF:
				{
				setState(119);
				match(DEF);
				setState(120);
				match(IDENTIFIER);
				setState(121);
				match(EQUALS);
				setState(124);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,13,_ctx) ) {
				case 1:
					{
					setState(122);
					expression(0);
					}
					break;
				case 2:
					{
					setState(123);
					match(IDENTIFIER);
					}
					break;
				}
				}
				break;
			case EQU:
				{
				setState(126);
				match(EQU);
				setState(127);
				match(IDENTIFIER);
				setState(128);
				match(EQUALS);
				setState(131);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,14,_ctx) ) {
				case 1:
					{
					setState(129);
					expression(0);
					}
					break;
				case 2:
					{
					setState(130);
					match(IDENTIFIER);
					}
					break;
				}
				}
				break;
			case CSEG:
				{
				setState(133);
				match(CSEG);
				}
				break;
			case ORG:
				{
				setState(134);
				match(ORG);
				setState(135);
				match(HEX_NUMBER);
				}
				break;
			case MACRO:
				{
				setState(136);
				match(MACRO);
				setState(137);
				match(IDENTIFIER);
				}
				break;
			case END_MACRO:
				{
				setState(138);
				match(END_MACRO);
				}
				break;
			case IF:
				{
				setState(139);
				match(IF);
				setState(140);
				expression(0);
				}
				break;
			case ELSE:
				{
				setState(141);
				match(ELSE);
				}
				break;
			case ENDIF:
				{
				setState(142);
				match(ENDIF);
				}
				break;
			case ERROR:
				{
				setState(143);
				match(ERROR);
				setState(144);
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
		enterRule(_localctx, 16, RULE_asm_intrinsic_usage);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(147);
			match(IDENTIFIER);
			setState(148);
			match(OPENING_BRACKET);
			setState(151);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case IDENTIFIER:
				{
				setState(149);
				match(IDENTIFIER);
				}
				break;
			case AT:
				{
				setState(150);
				macro_placeholder();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			setState(153);
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

	public static class InstructionContext extends ParserRuleContext {
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
		public InstructionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_instruction; }
	}

	public final InstructionContext instruction() throws RecognitionException {
		InstructionContext _localctx = new InstructionContext(_ctx, getState());
		enterRule(_localctx, 18, RULE_instruction);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(155);
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
		case 6:
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
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\3.\u00a0\4\2\t\2\4"+
		"\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4\13\t"+
		"\13\3\2\7\2\30\n\2\f\2\16\2\33\13\2\3\2\3\2\7\2\37\n\2\f\2\16\2\"\13\2"+
		"\3\2\7\2%\n\2\f\2\16\2(\13\2\3\2\7\2+\n\2\f\2\16\2.\13\2\3\2\3\2\3\3\3"+
		"\3\3\3\3\3\3\3\3\3\5\38\n\3\3\3\3\3\3\3\3\3\3\3\5\3?\n\3\5\3A\n\3\5\3"+
		"C\n\3\3\3\3\3\5\3G\n\3\3\4\3\4\7\4K\n\4\f\4\16\4N\13\4\3\5\3\5\3\5\3\6"+
		"\3\6\3\7\3\7\3\7\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\5\ba\n\b\3\b\3\b"+
		"\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\7\br\n\b\f\b\16\b"+
		"u\13\b\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\5\t\177\n\t\3\t\3\t\3\t\3\t\3\t"+
		"\5\t\u0086\n\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\5\t\u0094"+
		"\n\t\3\n\3\n\3\n\3\n\5\n\u009a\n\n\3\n\3\n\3\13\3\13\3\13\2\3\16\f\2\4"+
		"\6\b\n\f\16\20\22\24\2\3\3\2\3\r\2\u00bb\2\31\3\2\2\2\4F\3\2\2\2\6H\3"+
		"\2\2\2\bO\3\2\2\2\nR\3\2\2\2\fT\3\2\2\2\16`\3\2\2\2\20v\3\2\2\2\22\u0095"+
		"\3\2\2\2\24\u009d\3\2\2\2\26\30\7\16\2\2\27\26\3\2\2\2\30\33\3\2\2\2\31"+
		"\27\3\2\2\2\31\32\3\2\2\2\32\34\3\2\2\2\33\31\3\2\2\2\34&\5\4\3\2\35\37"+
		"\7\16\2\2\36\35\3\2\2\2\37\"\3\2\2\2 \36\3\2\2\2 !\3\2\2\2!#\3\2\2\2\""+
		" \3\2\2\2#%\5\4\3\2$ \3\2\2\2%(\3\2\2\2&$\3\2\2\2&\'\3\2\2\2\',\3\2\2"+
		"\2(&\3\2\2\2)+\7\16\2\2*)\3\2\2\2+.\3\2\2\2,*\3\2\2\2,-\3\2\2\2-/\3\2"+
		"\2\2.,\3\2\2\2/\60\7\2\2\3\60\3\3\2\2\2\61G\5\b\5\2\62B\5\24\13\2\638"+
		"\7.\2\2\648\5\16\b\2\658\5\22\n\2\668\5\f\7\2\67\63\3\2\2\2\67\64\3\2"+
		"\2\2\67\65\3\2\2\2\67\66\3\2\2\28@\3\2\2\29>\7\26\2\2:?\7.\2\2;?\5\16"+
		"\b\2<?\5\22\n\2=?\5\f\7\2>:\3\2\2\2>;\3\2\2\2><\3\2\2\2>=\3\2\2\2?A\3"+
		"\2\2\2@9\3\2\2\2@A\3\2\2\2AC\3\2\2\2B\67\3\2\2\2BC\3\2\2\2CG\3\2\2\2D"+
		"G\5\20\t\2EG\5\6\4\2F\61\3\2\2\2F\62\3\2\2\2FD\3\2\2\2FE\3\2\2\2G\5\3"+
		"\2\2\2HL\7.\2\2IK\5\16\b\2JI\3\2\2\2KN\3\2\2\2LJ\3\2\2\2LM\3\2\2\2M\7"+
		"\3\2\2\2NL\3\2\2\2OP\7.\2\2PQ\7\25\2\2Q\t\3\2\2\2RS\7.\2\2S\13\3\2\2\2"+
		"TU\7\23\2\2UV\7,\2\2V\r\3\2\2\2WX\b\b\1\2Xa\7,\2\2Ya\7-\2\2Za\7.\2\2["+
		"a\5\f\7\2\\]\7\'\2\2]^\5\16\b\2^_\7\24\2\2_a\3\2\2\2`W\3\2\2\2`Y\3\2\2"+
		"\2`Z\3\2\2\2`[\3\2\2\2`\\\3\2\2\2as\3\2\2\2bc\f\7\2\2cd\7#\2\2dr\5\16"+
		"\b\bef\f\6\2\2fg\7*\2\2gr\5\16\b\7hi\f\5\2\2ij\7+\2\2jr\5\16\b\6kl\f\4"+
		"\2\2lm\7 \2\2mr\5\16\b\5no\f\3\2\2op\7$\2\2pr\5\16\b\4qb\3\2\2\2qe\3\2"+
		"\2\2qh\3\2\2\2qk\3\2\2\2qn\3\2\2\2ru\3\2\2\2sq\3\2\2\2st\3\2\2\2t\17\3"+
		"\2\2\2us\3\2\2\2v\u0093\7\31\2\2wx\7\"\2\2x\u0094\7\21\2\2yz\7\30\2\2"+
		"z{\7.\2\2{~\7\35\2\2|\177\5\16\b\2}\177\7.\2\2~|\3\2\2\2~}\3\2\2\2\177"+
		"\u0094\3\2\2\2\u0080\u0081\7\36\2\2\u0081\u0082\7.\2\2\u0082\u0085\7\35"+
		"\2\2\u0083\u0086\5\16\b\2\u0084\u0086\7.\2\2\u0085\u0083\3\2\2\2\u0085"+
		"\u0084\3\2\2\2\u0086\u0094\3\2\2\2\u0087\u0094\7\27\2\2\u0088\u0089\7"+
		"(\2\2\u0089\u0094\7-\2\2\u008a\u008b\7%\2\2\u008b\u0094\7.\2\2\u008c\u0094"+
		"\7\33\2\2\u008d\u008e\7!\2\2\u008e\u0094\5\16\b\2\u008f\u0094\7\32\2\2"+
		"\u0090\u0094\7\34\2\2\u0091\u0092\7\37\2\2\u0092\u0094\7\21\2\2\u0093"+
		"w\3\2\2\2\u0093y\3\2\2\2\u0093\u0080\3\2\2\2\u0093\u0087\3\2\2\2\u0093"+
		"\u0088\3\2\2\2\u0093\u008a\3\2\2\2\u0093\u008c\3\2\2\2\u0093\u008d\3\2"+
		"\2\2\u0093\u008f\3\2\2\2\u0093\u0090\3\2\2\2\u0093\u0091\3\2\2\2\u0094"+
		"\21\3\2\2\2\u0095\u0096\7.\2\2\u0096\u0099\7\'\2\2\u0097\u009a\7.\2\2"+
		"\u0098\u009a\5\f\7\2\u0099\u0097\3\2\2\2\u0099\u0098\3\2\2\2\u009a\u009b"+
		"\3\2\2\2\u009b\u009c\7\24\2\2\u009c\23\3\2\2\2\u009d\u009e\t\2\2\2\u009e"+
		"\25\3\2\2\2\23\31 &,\67>@BFL`qs~\u0085\u0093\u0099";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
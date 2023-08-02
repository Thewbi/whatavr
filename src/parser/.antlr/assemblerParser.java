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
		public InstructionContext instruction() {
			return getRuleContext(InstructionContext.class,0);
		}
		public Label_definitionContext label_definition() {
			return getRuleContext(Label_definitionContext.class,0);
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
			setState(70);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,9,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				{
				setState(48);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==IDENTIFIER) {
					{
					setState(47);
					label_definition();
					}
				}

				{
				setState(50);
				instruction();
				setState(66);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,8,_ctx) ) {
				case 1:
					{
					setState(55);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,5,_ctx) ) {
					case 1:
						{
						setState(51);
						match(IDENTIFIER);
						}
						break;
					case 2:
						{
						setState(52);
						expression(0);
						}
						break;
					case 3:
						{
						setState(53);
						asm_intrinsic_usage();
						}
						break;
					case 4:
						{
						setState(54);
						macro_placeholder();
						}
						break;
					}
					setState(64);
					_errHandler.sync(this);
					_la = _input.LA(1);
					if (_la==COMMA) {
						{
						setState(57);
						match(COMMA);
						setState(62);
						_errHandler.sync(this);
						switch ( getInterpreter().adaptivePredict(_input,6,_ctx) ) {
						case 1:
							{
							setState(58);
							match(IDENTIFIER);
							}
							break;
						case 2:
							{
							setState(59);
							expression(0);
							}
							break;
						case 3:
							{
							setState(60);
							asm_intrinsic_usage();
							}
							break;
						case 4:
							{
							setState(61);
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
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(68);
				asm_instrinsic_instruction();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(69);
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
			setState(72);
			match(IDENTIFIER);
			setState(76);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,10,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(73);
					expression(0);
					}
					} 
				}
				setState(78);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,10,_ctx);
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
			setState(79);
			match(IDENTIFIER);
			setState(80);
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
			setState(82);
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
			setState(84);
			match(AT);
			setState(85);
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
			setState(96);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case NUMBER:
				{
				setState(88);
				match(NUMBER);
				}
				break;
			case HEX_NUMBER:
				{
				setState(89);
				match(HEX_NUMBER);
				}
				break;
			case IDENTIFIER:
				{
				setState(90);
				match(IDENTIFIER);
				}
				break;
			case AT:
				{
				setState(91);
				macro_placeholder();
				}
				break;
			case OPENING_BRACKET:
				{
				setState(92);
				match(OPENING_BRACKET);
				setState(93);
				expression(0);
				setState(94);
				match(CLOSEING_BRACKET);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			_ctx.stop = _input.LT(-1);
			setState(115);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,13,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(113);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,12,_ctx) ) {
					case 1:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(98);
						if (!(precpred(_ctx, 5))) throw new FailedPredicateException(this, "precpred(_ctx, 5)");
						setState(99);
						match(LEFT_SHIFT);
						setState(100);
						expression(6);
						}
						break;
					case 2:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(101);
						if (!(precpred(_ctx, 4))) throw new FailedPredicateException(this, "precpred(_ctx, 4)");
						setState(102);
						match(RIGHT_SHIFT);
						setState(103);
						expression(5);
						}
						break;
					case 3:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(104);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(105);
						match(SLASH);
						setState(106);
						expression(4);
						}
						break;
					case 4:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(107);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(108);
						match(GT);
						setState(109);
						expression(3);
						}
						break;
					case 5:
						{
						_localctx = new ExpressionContext(_parentctx, _parentState);
						pushNewRecursionContext(_localctx, _startState, RULE_expression);
						setState(110);
						if (!(precpred(_ctx, 1))) throw new FailedPredicateException(this, "precpred(_ctx, 1)");
						setState(111);
						match(LT);
						setState(112);
						expression(2);
						}
						break;
					}
					} 
				}
				setState(117);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,13,_ctx);
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
			setState(118);
			match(DOT);
			setState(147);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INCLUDE:
				{
				{
				setState(119);
				match(INCLUDE);
				setState(120);
				match(STRING);
				}
				}
				break;
			case DEF:
				{
				setState(121);
				match(DEF);
				setState(122);
				match(IDENTIFIER);
				setState(123);
				match(EQUALS);
				setState(126);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,14,_ctx) ) {
				case 1:
					{
					setState(124);
					expression(0);
					}
					break;
				case 2:
					{
					setState(125);
					match(IDENTIFIER);
					}
					break;
				}
				}
				break;
			case EQU:
				{
				setState(128);
				match(EQU);
				setState(129);
				match(IDENTIFIER);
				setState(130);
				match(EQUALS);
				setState(133);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,15,_ctx) ) {
				case 1:
					{
					setState(131);
					expression(0);
					}
					break;
				case 2:
					{
					setState(132);
					match(IDENTIFIER);
					}
					break;
				}
				}
				break;
			case CSEG:
				{
				setState(135);
				match(CSEG);
				}
				break;
			case ORG:
				{
				setState(136);
				match(ORG);
				setState(137);
				match(HEX_NUMBER);
				}
				break;
			case MACRO:
				{
				setState(138);
				match(MACRO);
				setState(139);
				match(IDENTIFIER);
				}
				break;
			case END_MACRO:
				{
				setState(140);
				match(END_MACRO);
				}
				break;
			case IF:
				{
				setState(141);
				match(IF);
				setState(142);
				expression(0);
				}
				break;
			case ELSE:
				{
				setState(143);
				match(ELSE);
				}
				break;
			case ENDIF:
				{
				setState(144);
				match(ENDIF);
				}
				break;
			case ERROR:
				{
				setState(145);
				match(ERROR);
				setState(146);
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
			setState(149);
			match(IDENTIFIER);
			setState(150);
			match(OPENING_BRACKET);
			setState(153);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case IDENTIFIER:
				{
				setState(151);
				match(IDENTIFIER);
				}
				break;
			case AT:
				{
				setState(152);
				macro_placeholder();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			setState(155);
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
			setState(157);
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
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\3.\u00a2\4\2\t\2\4"+
		"\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4\13\t"+
		"\13\3\2\7\2\30\n\2\f\2\16\2\33\13\2\3\2\3\2\7\2\37\n\2\f\2\16\2\"\13\2"+
		"\3\2\7\2%\n\2\f\2\16\2(\13\2\3\2\7\2+\n\2\f\2\16\2.\13\2\3\2\3\2\3\3\5"+
		"\3\63\n\3\3\3\3\3\3\3\3\3\3\3\5\3:\n\3\3\3\3\3\3\3\3\3\3\3\5\3A\n\3\5"+
		"\3C\n\3\5\3E\n\3\3\3\3\3\5\3I\n\3\3\4\3\4\7\4M\n\4\f\4\16\4P\13\4\3\5"+
		"\3\5\3\5\3\6\3\6\3\7\3\7\3\7\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\5\bc"+
		"\n\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\7\bt"+
		"\n\b\f\b\16\bw\13\b\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\5\t\u0081\n\t\3\t"+
		"\3\t\3\t\3\t\3\t\5\t\u0088\n\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t\3\t"+
		"\3\t\3\t\5\t\u0096\n\t\3\n\3\n\3\n\3\n\5\n\u009c\n\n\3\n\3\n\3\13\3\13"+
		"\3\13\2\3\16\f\2\4\6\b\n\f\16\20\22\24\2\3\3\2\3\r\2\u00bd\2\31\3\2\2"+
		"\2\4H\3\2\2\2\6J\3\2\2\2\bQ\3\2\2\2\nT\3\2\2\2\fV\3\2\2\2\16b\3\2\2\2"+
		"\20x\3\2\2\2\22\u0097\3\2\2\2\24\u009f\3\2\2\2\26\30\7\16\2\2\27\26\3"+
		"\2\2\2\30\33\3\2\2\2\31\27\3\2\2\2\31\32\3\2\2\2\32\34\3\2\2\2\33\31\3"+
		"\2\2\2\34&\5\4\3\2\35\37\7\16\2\2\36\35\3\2\2\2\37\"\3\2\2\2 \36\3\2\2"+
		"\2 !\3\2\2\2!#\3\2\2\2\" \3\2\2\2#%\5\4\3\2$ \3\2\2\2%(\3\2\2\2&$\3\2"+
		"\2\2&\'\3\2\2\2\',\3\2\2\2(&\3\2\2\2)+\7\16\2\2*)\3\2\2\2+.\3\2\2\2,*"+
		"\3\2\2\2,-\3\2\2\2-/\3\2\2\2.,\3\2\2\2/\60\7\2\2\3\60\3\3\2\2\2\61\63"+
		"\5\b\5\2\62\61\3\2\2\2\62\63\3\2\2\2\63\64\3\2\2\2\64D\5\24\13\2\65:\7"+
		".\2\2\66:\5\16\b\2\67:\5\22\n\28:\5\f\7\29\65\3\2\2\29\66\3\2\2\29\67"+
		"\3\2\2\298\3\2\2\2:B\3\2\2\2;@\7\26\2\2<A\7.\2\2=A\5\16\b\2>A\5\22\n\2"+
		"?A\5\f\7\2@<\3\2\2\2@=\3\2\2\2@>\3\2\2\2@?\3\2\2\2AC\3\2\2\2B;\3\2\2\2"+
		"BC\3\2\2\2CE\3\2\2\2D9\3\2\2\2DE\3\2\2\2EI\3\2\2\2FI\5\20\t\2GI\5\6\4"+
		"\2H\62\3\2\2\2HF\3\2\2\2HG\3\2\2\2I\5\3\2\2\2JN\7.\2\2KM\5\16\b\2LK\3"+
		"\2\2\2MP\3\2\2\2NL\3\2\2\2NO\3\2\2\2O\7\3\2\2\2PN\3\2\2\2QR\7.\2\2RS\7"+
		"\25\2\2S\t\3\2\2\2TU\7.\2\2U\13\3\2\2\2VW\7\23\2\2WX\7,\2\2X\r\3\2\2\2"+
		"YZ\b\b\1\2Zc\7,\2\2[c\7-\2\2\\c\7.\2\2]c\5\f\7\2^_\7\'\2\2_`\5\16\b\2"+
		"`a\7\24\2\2ac\3\2\2\2bY\3\2\2\2b[\3\2\2\2b\\\3\2\2\2b]\3\2\2\2b^\3\2\2"+
		"\2cu\3\2\2\2de\f\7\2\2ef\7#\2\2ft\5\16\b\bgh\f\6\2\2hi\7*\2\2it\5\16\b"+
		"\7jk\f\5\2\2kl\7+\2\2lt\5\16\b\6mn\f\4\2\2no\7 \2\2ot\5\16\b\5pq\f\3\2"+
		"\2qr\7$\2\2rt\5\16\b\4sd\3\2\2\2sg\3\2\2\2sj\3\2\2\2sm\3\2\2\2sp\3\2\2"+
		"\2tw\3\2\2\2us\3\2\2\2uv\3\2\2\2v\17\3\2\2\2wu\3\2\2\2x\u0095\7\31\2\2"+
		"yz\7\"\2\2z\u0096\7\21\2\2{|\7\30\2\2|}\7.\2\2}\u0080\7\35\2\2~\u0081"+
		"\5\16\b\2\177\u0081\7.\2\2\u0080~\3\2\2\2\u0080\177\3\2\2\2\u0081\u0096"+
		"\3\2\2\2\u0082\u0083\7\36\2\2\u0083\u0084\7.\2\2\u0084\u0087\7\35\2\2"+
		"\u0085\u0088\5\16\b\2\u0086\u0088\7.\2\2\u0087\u0085\3\2\2\2\u0087\u0086"+
		"\3\2\2\2\u0088\u0096\3\2\2\2\u0089\u0096\7\27\2\2\u008a\u008b\7(\2\2\u008b"+
		"\u0096\7-\2\2\u008c\u008d\7%\2\2\u008d\u0096\7.\2\2\u008e\u0096\7\33\2"+
		"\2\u008f\u0090\7!\2\2\u0090\u0096\5\16\b\2\u0091\u0096\7\32\2\2\u0092"+
		"\u0096\7\34\2\2\u0093\u0094\7\37\2\2\u0094\u0096\7\21\2\2\u0095y\3\2\2"+
		"\2\u0095{\3\2\2\2\u0095\u0082\3\2\2\2\u0095\u0089\3\2\2\2\u0095\u008a"+
		"\3\2\2\2\u0095\u008c\3\2\2\2\u0095\u008e\3\2\2\2\u0095\u008f\3\2\2\2\u0095"+
		"\u0091\3\2\2\2\u0095\u0092\3\2\2\2\u0095\u0093\3\2\2\2\u0096\21\3\2\2"+
		"\2\u0097\u0098\7.\2\2\u0098\u009b\7\'\2\2\u0099\u009c\7.\2\2\u009a\u009c"+
		"\5\f\7\2\u009b\u0099\3\2\2\2\u009b\u009a\3\2\2\2\u009c\u009d\3\2\2\2\u009d"+
		"\u009e\7\24\2\2\u009e\23\3\2\2\2\u009f\u00a0\t\2\2\2\u00a0\25\3\2\2\2"+
		"\24\31 &,\629@BDHNbsu\u0080\u0087\u0095\u009b";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
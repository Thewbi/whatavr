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
		LDI=1, LINE_COMMENT=2, STRING=3, ASTERISK=4, CLOSEING_BRACKET=5, COLON=6, 
		COMMA=7, CSEG=8, DEF=9, DOT=10, ELSE=11, END_MACRO=12, ENDIF=13, EQUALS=14, 
		EQU=15, ERROR=16, IF=17, INCLUDE=18, MACRO=19, MINUS=20, OPENING_BRACKET=21, 
		ORG=22, PLUS=23, SLASH=24, NEWLINE=25, WS=26, NUMBER=27, HEX_NUMBER=28, 
		IDENTIFIER=29;
	public static final int
		RULE_asm_file = 0, RULE_row = 1, RULE_label_definition = 2, RULE_parameter = 3, 
		RULE_expression = 4, RULE_asm_instrinsic_instruction = 5, RULE_asm_intrinsic_usage = 6, 
		RULE_instruction = 7;
	private static String[] makeRuleNames() {
		return new String[] {
			"asm_file", "row", "label_definition", "parameter", "expression", "asm_instrinsic_instruction", 
			"asm_intrinsic_usage", "instruction"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, null, null, null, "'*'", "')'", "':'", "','", "'cseg'", "'def'", 
			"'.'", "'else'", "'endmacro'", "'endif'", "'='", "'equ'", "'error'", 
			"'if'", "'include'", "'macro'", "'-'", "'('", "'org'", "'+'", "'/'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "LDI", "LINE_COMMENT", "STRING", "ASTERISK", "CLOSEING_BRACKET", 
			"COLON", "COMMA", "CSEG", "DEF", "DOT", "ELSE", "END_MACRO", "ENDIF", 
			"EQUALS", "EQU", "ERROR", "IF", "INCLUDE", "MACRO", "MINUS", "OPENING_BRACKET", 
			"ORG", "PLUS", "SLASH", "NEWLINE", "WS", "NUMBER", "HEX_NUMBER", "IDENTIFIER"
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
			setState(19);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==NEWLINE) {
				{
				{
				setState(16);
				match(NEWLINE);
				}
				}
				setState(21);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(22);
			row();
			setState(32);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(26);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==NEWLINE) {
						{
						{
						setState(23);
						match(NEWLINE);
						}
						}
						setState(28);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					setState(29);
					row();
					}
					} 
				}
				setState(34);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,2,_ctx);
			}
			setState(38);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==NEWLINE) {
				{
				{
				setState(35);
				match(NEWLINE);
				}
				}
				setState(40);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(41);
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
		public TerminalNode COMMA() { return getToken(assemblerParser.COMMA, 0); }
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
		int _la;
		try {
			setState(61);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case LDI:
			case IDENTIFIER:
				enterOuterAlt(_localctx, 1);
				{
				{
				setState(44);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==IDENTIFIER) {
					{
					setState(43);
					label_definition();
					}
				}

				{
				setState(46);
				instruction();
				{
				setState(50);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,5,_ctx) ) {
				case 1:
					{
					setState(47);
					match(IDENTIFIER);
					}
					break;
				case 2:
					{
					setState(48);
					expression();
					}
					break;
				case 3:
					{
					setState(49);
					asm_intrinsic_usage();
					}
					break;
				}
				setState(58);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==COMMA) {
					{
					setState(52);
					match(COMMA);
					setState(56);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,6,_ctx) ) {
					case 1:
						{
						setState(53);
						match(IDENTIFIER);
						}
						break;
					case 2:
						{
						setState(54);
						expression();
						}
						break;
					case 3:
						{
						setState(55);
						asm_intrinsic_usage();
						}
						break;
					}
					}
				}

				}
				}
				}
				}
				break;
			case DOT:
				enterOuterAlt(_localctx, 2);
				{
				setState(60);
				asm_instrinsic_instruction();
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
		enterRule(_localctx, 4, RULE_label_definition);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(63);
			match(IDENTIFIER);
			setState(64);
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
		enterRule(_localctx, 6, RULE_parameter);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(66);
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

	public static class ExpressionContext extends ParserRuleContext {
		public TerminalNode NUMBER() { return getToken(assemblerParser.NUMBER, 0); }
		public TerminalNode HEX_NUMBER() { return getToken(assemblerParser.HEX_NUMBER, 0); }
		public ExpressionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expression; }
	}

	public final ExpressionContext expression() throws RecognitionException {
		ExpressionContext _localctx = new ExpressionContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_expression);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(68);
			_la = _input.LA(1);
			if ( !(_la==NUMBER || _la==HEX_NUMBER) ) {
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
		enterRule(_localctx, 10, RULE_asm_instrinsic_instruction);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(70);
			match(DOT);
			setState(99);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INCLUDE:
				{
				{
				setState(71);
				match(INCLUDE);
				setState(72);
				match(STRING);
				}
				}
				break;
			case DEF:
				{
				setState(73);
				match(DEF);
				setState(74);
				match(IDENTIFIER);
				setState(75);
				match(EQUALS);
				setState(78);
				_errHandler.sync(this);
				switch (_input.LA(1)) {
				case NUMBER:
				case HEX_NUMBER:
					{
					setState(76);
					expression();
					}
					break;
				case IDENTIFIER:
					{
					setState(77);
					match(IDENTIFIER);
					}
					break;
				default:
					throw new NoViableAltException(this);
				}
				}
				break;
			case EQU:
				{
				setState(80);
				match(EQU);
				setState(81);
				match(IDENTIFIER);
				setState(82);
				match(EQUALS);
				setState(85);
				_errHandler.sync(this);
				switch (_input.LA(1)) {
				case NUMBER:
				case HEX_NUMBER:
					{
					setState(83);
					expression();
					}
					break;
				case IDENTIFIER:
					{
					setState(84);
					match(IDENTIFIER);
					}
					break;
				default:
					throw new NoViableAltException(this);
				}
				}
				break;
			case CSEG:
				{
				setState(87);
				match(CSEG);
				}
				break;
			case ORG:
				{
				setState(88);
				match(ORG);
				setState(89);
				match(HEX_NUMBER);
				}
				break;
			case MACRO:
				{
				setState(90);
				match(MACRO);
				setState(91);
				match(IDENTIFIER);
				}
				break;
			case END_MACRO:
				{
				setState(92);
				match(END_MACRO);
				}
				break;
			case IF:
				{
				setState(93);
				match(IF);
				setState(94);
				expression();
				}
				break;
			case ELSE:
				{
				setState(95);
				match(ELSE);
				}
				break;
			case ENDIF:
				{
				setState(96);
				match(ENDIF);
				}
				break;
			case ERROR:
				{
				setState(97);
				match(ERROR);
				setState(98);
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
		public Asm_intrinsic_usageContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_asm_intrinsic_usage; }
	}

	public final Asm_intrinsic_usageContext asm_intrinsic_usage() throws RecognitionException {
		Asm_intrinsic_usageContext _localctx = new Asm_intrinsic_usageContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_asm_intrinsic_usage);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(101);
			match(IDENTIFIER);
			setState(102);
			match(OPENING_BRACKET);
			setState(103);
			match(IDENTIFIER);
			setState(104);
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
		public TerminalNode LDI() { return getToken(assemblerParser.LDI, 0); }
		public InstructionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_instruction; }
	}

	public final InstructionContext instruction() throws RecognitionException {
		InstructionContext _localctx = new InstructionContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_instruction);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(106);
			match(LDI);
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

	public static final String _serializedATN =
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\3\37o\4\2\t\2\4\3\t"+
		"\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\3\2\7\2\24\n\2\f\2"+
		"\16\2\27\13\2\3\2\3\2\7\2\33\n\2\f\2\16\2\36\13\2\3\2\7\2!\n\2\f\2\16"+
		"\2$\13\2\3\2\7\2\'\n\2\f\2\16\2*\13\2\3\2\3\2\3\3\5\3/\n\3\3\3\3\3\3\3"+
		"\3\3\5\3\65\n\3\3\3\3\3\3\3\3\3\5\3;\n\3\5\3=\n\3\3\3\5\3@\n\3\3\4\3\4"+
		"\3\4\3\5\3\5\3\6\3\6\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\5\7Q\n\7\3\7\3\7"+
		"\3\7\3\7\3\7\5\7X\n\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7"+
		"\5\7f\n\7\3\b\3\b\3\b\3\b\3\b\3\t\3\t\3\t\2\2\n\2\4\6\b\n\f\16\20\2\3"+
		"\3\2\35\36\2}\2\25\3\2\2\2\4?\3\2\2\2\6A\3\2\2\2\bD\3\2\2\2\nF\3\2\2\2"+
		"\fH\3\2\2\2\16g\3\2\2\2\20l\3\2\2\2\22\24\7\33\2\2\23\22\3\2\2\2\24\27"+
		"\3\2\2\2\25\23\3\2\2\2\25\26\3\2\2\2\26\30\3\2\2\2\27\25\3\2\2\2\30\""+
		"\5\4\3\2\31\33\7\33\2\2\32\31\3\2\2\2\33\36\3\2\2\2\34\32\3\2\2\2\34\35"+
		"\3\2\2\2\35\37\3\2\2\2\36\34\3\2\2\2\37!\5\4\3\2 \34\3\2\2\2!$\3\2\2\2"+
		"\" \3\2\2\2\"#\3\2\2\2#(\3\2\2\2$\"\3\2\2\2%\'\7\33\2\2&%\3\2\2\2\'*\3"+
		"\2\2\2(&\3\2\2\2()\3\2\2\2)+\3\2\2\2*(\3\2\2\2+,\7\2\2\3,\3\3\2\2\2-/"+
		"\5\6\4\2.-\3\2\2\2./\3\2\2\2/\60\3\2\2\2\60\64\5\20\t\2\61\65\7\37\2\2"+
		"\62\65\5\n\6\2\63\65\5\16\b\2\64\61\3\2\2\2\64\62\3\2\2\2\64\63\3\2\2"+
		"\2\65<\3\2\2\2\66:\7\t\2\2\67;\7\37\2\28;\5\n\6\29;\5\16\b\2:\67\3\2\2"+
		"\2:8\3\2\2\2:9\3\2\2\2;=\3\2\2\2<\66\3\2\2\2<=\3\2\2\2=@\3\2\2\2>@\5\f"+
		"\7\2?.\3\2\2\2?>\3\2\2\2@\5\3\2\2\2AB\7\37\2\2BC\7\b\2\2C\7\3\2\2\2DE"+
		"\7\37\2\2E\t\3\2\2\2FG\t\2\2\2G\13\3\2\2\2He\7\f\2\2IJ\7\24\2\2Jf\7\5"+
		"\2\2KL\7\13\2\2LM\7\37\2\2MP\7\20\2\2NQ\5\n\6\2OQ\7\37\2\2PN\3\2\2\2P"+
		"O\3\2\2\2Qf\3\2\2\2RS\7\21\2\2ST\7\37\2\2TW\7\20\2\2UX\5\n\6\2VX\7\37"+
		"\2\2WU\3\2\2\2WV\3\2\2\2Xf\3\2\2\2Yf\7\n\2\2Z[\7\30\2\2[f\7\36\2\2\\]"+
		"\7\25\2\2]f\7\37\2\2^f\7\16\2\2_`\7\23\2\2`f\5\n\6\2af\7\r\2\2bf\7\17"+
		"\2\2cd\7\22\2\2df\7\5\2\2eI\3\2\2\2eK\3\2\2\2eR\3\2\2\2eY\3\2\2\2eZ\3"+
		"\2\2\2e\\\3\2\2\2e^\3\2\2\2e_\3\2\2\2ea\3\2\2\2eb\3\2\2\2ec\3\2\2\2f\r"+
		"\3\2\2\2gh\7\37\2\2hi\7\27\2\2ij\7\37\2\2jk\7\7\2\2k\17\3\2\2\2lm\7\3"+
		"\2\2m\21\3\2\2\2\16\25\34\"(.\64:<?PWe";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
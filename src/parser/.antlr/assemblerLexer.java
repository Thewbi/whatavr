// Generated from c:\aaa_se\rust\whatavr\src\parser\assembler.g4 by ANTLR 4.9.2
import org.antlr.v4.runtime.Lexer;
import org.antlr.v4.runtime.CharStream;
import org.antlr.v4.runtime.Token;
import org.antlr.v4.runtime.TokenStream;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.misc.*;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast"})
public class assemblerLexer extends Lexer {
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
	public static String[] channelNames = {
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	};

	public static String[] modeNames = {
		"DEFAULT_MODE"
	};

	private static String[] makeRuleNames() {
		return new String[] {
			"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", 
			"O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "LDI", "LINE_COMMENT", 
			"STRING", "ASTERISK", "CLOSEING_BRACKET", "COLON", "COMMA", "CSEG", "DEF", 
			"DOT", "ELSE", "END_MACRO", "ENDIF", "EQUALS", "EQU", "ERROR", "IF", 
			"INCLUDE", "MACRO", "MINUS", "OPENING_BRACKET", "ORG", "PLUS", "SLASH", 
			"NEWLINE", "WS", "NUMBER", "HEX_NUMBER", "IDENTIFIER"
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


	public assemblerLexer(CharStream input) {
		super(input);
		_interp = new LexerATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@Override
	public String getGrammarFileName() { return "assembler.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public String[] getChannelNames() { return channelNames; }

	@Override
	public String[] getModeNames() { return modeNames; }

	@Override
	public ATN getATN() { return _ATN; }

	public static final String _serializedATN =
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\2\37\u012b\b\1\4\2"+
		"\t\2\4\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4"+
		"\13\t\13\4\f\t\f\4\r\t\r\4\16\t\16\4\17\t\17\4\20\t\20\4\21\t\21\4\22"+
		"\t\22\4\23\t\23\4\24\t\24\4\25\t\25\4\26\t\26\4\27\t\27\4\30\t\30\4\31"+
		"\t\31\4\32\t\32\4\33\t\33\4\34\t\34\4\35\t\35\4\36\t\36\4\37\t\37\4 \t"+
		" \4!\t!\4\"\t\"\4#\t#\4$\t$\4%\t%\4&\t&\4\'\t\'\4(\t(\4)\t)\4*\t*\4+\t"+
		"+\4,\t,\4-\t-\4.\t.\4/\t/\4\60\t\60\4\61\t\61\4\62\t\62\4\63\t\63\4\64"+
		"\t\64\4\65\t\65\4\66\t\66\4\67\t\67\48\t8\3\2\3\2\3\3\3\3\3\4\3\4\3\5"+
		"\3\5\3\6\3\6\3\7\3\7\3\b\3\b\3\t\3\t\3\n\3\n\3\13\3\13\3\f\3\f\3\r\3\r"+
		"\3\16\3\16\3\17\3\17\3\20\3\20\3\21\3\21\3\22\3\22\3\23\3\23\3\24\3\24"+
		"\3\25\3\25\3\26\3\26\3\27\3\27\3\30\3\30\3\31\3\31\3\32\3\32\3\33\3\33"+
		"\3\34\3\34\3\34\3\34\3\35\3\35\7\35\u00ac\n\35\f\35\16\35\u00af\13\35"+
		"\3\35\3\35\3\36\3\36\3\36\3\36\7\36\u00b7\n\36\f\36\16\36\u00ba\13\36"+
		"\3\36\3\36\3\37\3\37\3 \3 \3!\3!\3\"\3\"\3#\3#\3#\3#\3#\3$\3$\3$\3$\3"+
		"%\3%\3&\3&\3&\3&\3&\3\'\3\'\3\'\3\'\3\'\3\'\3\'\3\'\3\'\3(\3(\3(\3(\3"+
		"(\3(\3)\3)\3*\3*\3*\3*\3+\3+\3+\3+\3+\3+\3,\3,\3,\3-\3-\3-\3-\3-\3-\3"+
		"-\3-\3.\3.\3.\3.\3.\3.\3/\3/\3\60\3\60\3\61\3\61\3\61\3\61\3\62\3\62\3"+
		"\63\3\63\3\64\5\64\u010f\n\64\3\64\3\64\3\65\6\65\u0114\n\65\r\65\16\65"+
		"\u0115\3\65\3\65\3\66\6\66\u011b\n\66\r\66\16\66\u011c\3\67\3\67\3\67"+
		"\6\67\u0122\n\67\r\67\16\67\u0123\38\38\68\u0128\n8\r8\168\u0129\2\29"+
		"\3\2\5\2\7\2\t\2\13\2\r\2\17\2\21\2\23\2\25\2\27\2\31\2\33\2\35\2\37\2"+
		"!\2#\2%\2\'\2)\2+\2-\2/\2\61\2\63\2\65\2\67\39\4;\5=\6?\7A\bC\tE\nG\13"+
		"I\fK\rM\16O\17Q\20S\21U\22W\23Y\24[\25]\26_\27a\30c\31e\32g\33i\34k\35"+
		"m\36o\37\3\2#\4\2CCcc\4\2DDdd\4\2EEee\4\2FFff\4\2GGgg\4\2HHhh\4\2IIii"+
		"\4\2JJjj\4\2KKkk\4\2LLll\4\2MMmm\4\2NNnn\4\2OOoo\4\2PPpp\4\2QQqq\4\2R"+
		"Rrr\4\2SSss\4\2TTtt\4\2UUuu\4\2VVvv\4\2WWww\4\2XXxx\4\2YYyy\4\2ZZzz\4"+
		"\2[[{{\4\2\\\\||\4\2\f\f\17\17\3\2$$\5\2\13\f\16\17\"\"\3\2\62;\5\2\62"+
		";CHch\4\2C\\c|\6\2\62;C\\aac|\2\u0118\2\67\3\2\2\2\29\3\2\2\2\2;\3\2\2"+
		"\2\2=\3\2\2\2\2?\3\2\2\2\2A\3\2\2\2\2C\3\2\2\2\2E\3\2\2\2\2G\3\2\2\2\2"+
		"I\3\2\2\2\2K\3\2\2\2\2M\3\2\2\2\2O\3\2\2\2\2Q\3\2\2\2\2S\3\2\2\2\2U\3"+
		"\2\2\2\2W\3\2\2\2\2Y\3\2\2\2\2[\3\2\2\2\2]\3\2\2\2\2_\3\2\2\2\2a\3\2\2"+
		"\2\2c\3\2\2\2\2e\3\2\2\2\2g\3\2\2\2\2i\3\2\2\2\2k\3\2\2\2\2m\3\2\2\2\2"+
		"o\3\2\2\2\3q\3\2\2\2\5s\3\2\2\2\7u\3\2\2\2\tw\3\2\2\2\13y\3\2\2\2\r{\3"+
		"\2\2\2\17}\3\2\2\2\21\177\3\2\2\2\23\u0081\3\2\2\2\25\u0083\3\2\2\2\27"+
		"\u0085\3\2\2\2\31\u0087\3\2\2\2\33\u0089\3\2\2\2\35\u008b\3\2\2\2\37\u008d"+
		"\3\2\2\2!\u008f\3\2\2\2#\u0091\3\2\2\2%\u0093\3\2\2\2\'\u0095\3\2\2\2"+
		")\u0097\3\2\2\2+\u0099\3\2\2\2-\u009b\3\2\2\2/\u009d\3\2\2\2\61\u009f"+
		"\3\2\2\2\63\u00a1\3\2\2\2\65\u00a3\3\2\2\2\67\u00a5\3\2\2\29\u00a9\3\2"+
		"\2\2;\u00b2\3\2\2\2=\u00bd\3\2\2\2?\u00bf\3\2\2\2A\u00c1\3\2\2\2C\u00c3"+
		"\3\2\2\2E\u00c5\3\2\2\2G\u00ca\3\2\2\2I\u00ce\3\2\2\2K\u00d0\3\2\2\2M"+
		"\u00d5\3\2\2\2O\u00de\3\2\2\2Q\u00e4\3\2\2\2S\u00e6\3\2\2\2U\u00ea\3\2"+
		"\2\2W\u00f0\3\2\2\2Y\u00f3\3\2\2\2[\u00fb\3\2\2\2]\u0101\3\2\2\2_\u0103"+
		"\3\2\2\2a\u0105\3\2\2\2c\u0109\3\2\2\2e\u010b\3\2\2\2g\u010e\3\2\2\2i"+
		"\u0113\3\2\2\2k\u011a\3\2\2\2m\u011e\3\2\2\2o\u0125\3\2\2\2qr\t\2\2\2"+
		"r\4\3\2\2\2st\t\3\2\2t\6\3\2\2\2uv\t\4\2\2v\b\3\2\2\2wx\t\5\2\2x\n\3\2"+
		"\2\2yz\t\6\2\2z\f\3\2\2\2{|\t\7\2\2|\16\3\2\2\2}~\t\b\2\2~\20\3\2\2\2"+
		"\177\u0080\t\t\2\2\u0080\22\3\2\2\2\u0081\u0082\t\n\2\2\u0082\24\3\2\2"+
		"\2\u0083\u0084\t\13\2\2\u0084\26\3\2\2\2\u0085\u0086\t\f\2\2\u0086\30"+
		"\3\2\2\2\u0087\u0088\t\r\2\2\u0088\32\3\2\2\2\u0089\u008a\t\16\2\2\u008a"+
		"\34\3\2\2\2\u008b\u008c\t\17\2\2\u008c\36\3\2\2\2\u008d\u008e\t\20\2\2"+
		"\u008e \3\2\2\2\u008f\u0090\t\21\2\2\u0090\"\3\2\2\2\u0091\u0092\t\22"+
		"\2\2\u0092$\3\2\2\2\u0093\u0094\t\23\2\2\u0094&\3\2\2\2\u0095\u0096\t"+
		"\24\2\2\u0096(\3\2\2\2\u0097\u0098\t\25\2\2\u0098*\3\2\2\2\u0099\u009a"+
		"\t\26\2\2\u009a,\3\2\2\2\u009b\u009c\t\27\2\2\u009c.\3\2\2\2\u009d\u009e"+
		"\t\30\2\2\u009e\60\3\2\2\2\u009f\u00a0\t\31\2\2\u00a0\62\3\2\2\2\u00a1"+
		"\u00a2\t\32\2\2\u00a2\64\3\2\2\2\u00a3\u00a4\t\33\2\2\u00a4\66\3\2\2\2"+
		"\u00a5\u00a6\5\31\r\2\u00a6\u00a7\5\t\5\2\u00a7\u00a8\5\23\n\2\u00a88"+
		"\3\2\2\2\u00a9\u00ad\7=\2\2\u00aa\u00ac\n\34\2\2\u00ab\u00aa\3\2\2\2\u00ac"+
		"\u00af\3\2\2\2\u00ad\u00ab\3\2\2\2\u00ad\u00ae\3\2\2\2\u00ae\u00b0\3\2"+
		"\2\2\u00af\u00ad\3\2\2\2\u00b0\u00b1\b\35\2\2\u00b1:\3\2\2\2\u00b2\u00b8"+
		"\7$\2\2\u00b3\u00b4\7$\2\2\u00b4\u00b7\7$\2\2\u00b5\u00b7\n\35\2\2\u00b6"+
		"\u00b3\3\2\2\2\u00b6\u00b5\3\2\2\2\u00b7\u00ba\3\2\2\2\u00b8\u00b6\3\2"+
		"\2\2\u00b8\u00b9\3\2\2\2\u00b9\u00bb\3\2\2\2\u00ba\u00b8\3\2\2\2\u00bb"+
		"\u00bc\7$\2\2\u00bc<\3\2\2\2\u00bd\u00be\7,\2\2\u00be>\3\2\2\2\u00bf\u00c0"+
		"\7+\2\2\u00c0@\3\2\2\2\u00c1\u00c2\7<\2\2\u00c2B\3\2\2\2\u00c3\u00c4\7"+
		".\2\2\u00c4D\3\2\2\2\u00c5\u00c6\7e\2\2\u00c6\u00c7\7u\2\2\u00c7\u00c8"+
		"\7g\2\2\u00c8\u00c9\7i\2\2\u00c9F\3\2\2\2\u00ca\u00cb\7f\2\2\u00cb\u00cc"+
		"\7g\2\2\u00cc\u00cd\7h\2\2\u00cdH\3\2\2\2\u00ce\u00cf\7\60\2\2\u00cfJ"+
		"\3\2\2\2\u00d0\u00d1\7g\2\2\u00d1\u00d2\7n\2\2\u00d2\u00d3\7u\2\2\u00d3"+
		"\u00d4\7g\2\2\u00d4L\3\2\2\2\u00d5\u00d6\7g\2\2\u00d6\u00d7\7p\2\2\u00d7"+
		"\u00d8\7f\2\2\u00d8\u00d9\7o\2\2\u00d9\u00da\7c\2\2\u00da\u00db\7e\2\2"+
		"\u00db\u00dc\7t\2\2\u00dc\u00dd\7q\2\2\u00ddN\3\2\2\2\u00de\u00df\7g\2"+
		"\2\u00df\u00e0\7p\2\2\u00e0\u00e1\7f\2\2\u00e1\u00e2\7k\2\2\u00e2\u00e3"+
		"\7h\2\2\u00e3P\3\2\2\2\u00e4\u00e5\7?\2\2\u00e5R\3\2\2\2\u00e6\u00e7\7"+
		"g\2\2\u00e7\u00e8\7s\2\2\u00e8\u00e9\7w\2\2\u00e9T\3\2\2\2\u00ea\u00eb"+
		"\7g\2\2\u00eb\u00ec\7t\2\2\u00ec\u00ed\7t\2\2\u00ed\u00ee\7q\2\2\u00ee"+
		"\u00ef\7t\2\2\u00efV\3\2\2\2\u00f0\u00f1\7k\2\2\u00f1\u00f2\7h\2\2\u00f2"+
		"X\3\2\2\2\u00f3\u00f4\7k\2\2\u00f4\u00f5\7p\2\2\u00f5\u00f6\7e\2\2\u00f6"+
		"\u00f7\7n\2\2\u00f7\u00f8\7w\2\2\u00f8\u00f9\7f\2\2\u00f9\u00fa\7g\2\2"+
		"\u00faZ\3\2\2\2\u00fb\u00fc\7o\2\2\u00fc\u00fd\7c\2\2\u00fd\u00fe\7e\2"+
		"\2\u00fe\u00ff\7t\2\2\u00ff\u0100\7q\2\2\u0100\\\3\2\2\2\u0101\u0102\7"+
		"/\2\2\u0102^\3\2\2\2\u0103\u0104\7*\2\2\u0104`\3\2\2\2\u0105\u0106\7q"+
		"\2\2\u0106\u0107\7t\2\2\u0107\u0108\7i\2\2\u0108b\3\2\2\2\u0109\u010a"+
		"\7-\2\2\u010ad\3\2\2\2\u010b\u010c\7\61\2\2\u010cf\3\2\2\2\u010d\u010f"+
		"\7\17\2\2\u010e\u010d\3\2\2\2\u010e\u010f\3\2\2\2\u010f\u0110\3\2\2\2"+
		"\u0110\u0111\7\f\2\2\u0111h\3\2\2\2\u0112\u0114\t\36\2\2\u0113\u0112\3"+
		"\2\2\2\u0114\u0115\3\2\2\2\u0115\u0113\3\2\2\2\u0115\u0116\3\2\2\2\u0116"+
		"\u0117\3\2\2\2\u0117\u0118\b\65\2\2\u0118j\3\2\2\2\u0119\u011b\t\37\2"+
		"\2\u011a\u0119\3\2\2\2\u011b\u011c\3\2\2\2\u011c\u011a\3\2\2\2\u011c\u011d"+
		"\3\2\2\2\u011dl\3\2\2\2\u011e\u011f\7\62\2\2\u011f\u0121\7z\2\2\u0120"+
		"\u0122\t \2\2\u0121\u0120\3\2\2\2\u0122\u0123\3\2\2\2\u0123\u0121\3\2"+
		"\2\2\u0123\u0124\3\2\2\2\u0124n\3\2\2\2\u0125\u0127\t!\2\2\u0126\u0128"+
		"\t\"\2\2\u0127\u0126\3\2\2\2\u0128\u0129\3\2\2\2\u0129\u0127\3\2\2\2\u0129"+
		"\u012a\3\2\2\2\u012ap\3\2\2\2\13\2\u00ad\u00b6\u00b8\u010e\u0115\u011c"+
		"\u0123\u0129\3\2\3\2";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
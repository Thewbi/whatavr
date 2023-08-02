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
		LDI=1, ASTERISK=2, CLOSEING_BRACKET=3, COLON=4, COMMA=5, MINUS=6, OPENING_BRACKET=7, 
		PLUS=8, SLASH=9, NEWLINE=10, WS=11, LINE_COMMENT=12, NUMBER=13, IDENTIFIER=14;
	public static String[] channelNames = {
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	};

	public static String[] modeNames = {
		"DEFAULT_MODE"
	};

	private static String[] makeRuleNames() {
		return new String[] {
			"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", 
			"O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "LDI", "ASTERISK", 
			"CLOSEING_BRACKET", "COLON", "COMMA", "MINUS", "OPENING_BRACKET", "PLUS", 
			"SLASH", "NEWLINE", "WS", "LINE_COMMENT", "NUMBER", "IDENTIFIER"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, null, "'*'", "')'", "':'", "','", "'-'", "'('", "'+'", "'/'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "LDI", "ASTERISK", "CLOSEING_BRACKET", "COLON", "COMMA", "MINUS", 
			"OPENING_BRACKET", "PLUS", "SLASH", "NEWLINE", "WS", "LINE_COMMENT", 
			"NUMBER", "IDENTIFIER"
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
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\2\20\u00bb\b\1\4\2"+
		"\t\2\4\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4"+
		"\13\t\13\4\f\t\f\4\r\t\r\4\16\t\16\4\17\t\17\4\20\t\20\4\21\t\21\4\22"+
		"\t\22\4\23\t\23\4\24\t\24\4\25\t\25\4\26\t\26\4\27\t\27\4\30\t\30\4\31"+
		"\t\31\4\32\t\32\4\33\t\33\4\34\t\34\4\35\t\35\4\36\t\36\4\37\t\37\4 \t"+
		" \4!\t!\4\"\t\"\4#\t#\4$\t$\4%\t%\4&\t&\4\'\t\'\4(\t(\4)\t)\3\2\3\2\3"+
		"\3\3\3\3\4\3\4\3\5\3\5\3\6\3\6\3\7\3\7\3\b\3\b\3\t\3\t\3\n\3\n\3\13\3"+
		"\13\3\f\3\f\3\r\3\r\3\16\3\16\3\17\3\17\3\20\3\20\3\21\3\21\3\22\3\22"+
		"\3\23\3\23\3\24\3\24\3\25\3\25\3\26\3\26\3\27\3\27\3\30\3\30\3\31\3\31"+
		"\3\32\3\32\3\33\3\33\3\34\3\34\3\34\3\34\3\35\3\35\3\36\3\36\3\37\3\37"+
		"\3 \3 \3!\3!\3\"\3\"\3#\3#\3$\3$\3%\5%\u009d\n%\3%\3%\3&\6&\u00a2\n&\r"+
		"&\16&\u00a3\3&\3&\3\'\3\'\7\'\u00aa\n\'\f\'\16\'\u00ad\13\'\3\'\3\'\3"+
		"(\6(\u00b2\n(\r(\16(\u00b3\3)\3)\6)\u00b8\n)\r)\16)\u00b9\2\2*\3\2\5\2"+
		"\7\2\t\2\13\2\r\2\17\2\21\2\23\2\25\2\27\2\31\2\33\2\35\2\37\2!\2#\2%"+
		"\2\'\2)\2+\2-\2/\2\61\2\63\2\65\2\67\39\4;\5=\6?\7A\bC\tE\nG\13I\fK\r"+
		"M\16O\17Q\20\3\2!\4\2CCcc\4\2DDdd\4\2EEee\4\2FFff\4\2GGgg\4\2HHhh\4\2"+
		"IIii\4\2JJjj\4\2KKkk\4\2LLll\4\2MMmm\4\2NNnn\4\2OOoo\4\2PPpp\4\2QQqq\4"+
		"\2RRrr\4\2SSss\4\2TTtt\4\2UUuu\4\2VVvv\4\2WWww\4\2XXxx\4\2YYyy\4\2ZZz"+
		"z\4\2[[{{\4\2\\\\||\5\2\13\f\16\17\"\"\4\2\f\f\17\17\3\2\62;\4\2C\\c|"+
		"\6\2\62;C\\aac|\2\u00a5\2\67\3\2\2\2\29\3\2\2\2\2;\3\2\2\2\2=\3\2\2\2"+
		"\2?\3\2\2\2\2A\3\2\2\2\2C\3\2\2\2\2E\3\2\2\2\2G\3\2\2\2\2I\3\2\2\2\2K"+
		"\3\2\2\2\2M\3\2\2\2\2O\3\2\2\2\2Q\3\2\2\2\3S\3\2\2\2\5U\3\2\2\2\7W\3\2"+
		"\2\2\tY\3\2\2\2\13[\3\2\2\2\r]\3\2\2\2\17_\3\2\2\2\21a\3\2\2\2\23c\3\2"+
		"\2\2\25e\3\2\2\2\27g\3\2\2\2\31i\3\2\2\2\33k\3\2\2\2\35m\3\2\2\2\37o\3"+
		"\2\2\2!q\3\2\2\2#s\3\2\2\2%u\3\2\2\2\'w\3\2\2\2)y\3\2\2\2+{\3\2\2\2-}"+
		"\3\2\2\2/\177\3\2\2\2\61\u0081\3\2\2\2\63\u0083\3\2\2\2\65\u0085\3\2\2"+
		"\2\67\u0087\3\2\2\29\u008b\3\2\2\2;\u008d\3\2\2\2=\u008f\3\2\2\2?\u0091"+
		"\3\2\2\2A\u0093\3\2\2\2C\u0095\3\2\2\2E\u0097\3\2\2\2G\u0099\3\2\2\2I"+
		"\u009c\3\2\2\2K\u00a1\3\2\2\2M\u00a7\3\2\2\2O\u00b1\3\2\2\2Q\u00b5\3\2"+
		"\2\2ST\t\2\2\2T\4\3\2\2\2UV\t\3\2\2V\6\3\2\2\2WX\t\4\2\2X\b\3\2\2\2YZ"+
		"\t\5\2\2Z\n\3\2\2\2[\\\t\6\2\2\\\f\3\2\2\2]^\t\7\2\2^\16\3\2\2\2_`\t\b"+
		"\2\2`\20\3\2\2\2ab\t\t\2\2b\22\3\2\2\2cd\t\n\2\2d\24\3\2\2\2ef\t\13\2"+
		"\2f\26\3\2\2\2gh\t\f\2\2h\30\3\2\2\2ij\t\r\2\2j\32\3\2\2\2kl\t\16\2\2"+
		"l\34\3\2\2\2mn\t\17\2\2n\36\3\2\2\2op\t\20\2\2p \3\2\2\2qr\t\21\2\2r\""+
		"\3\2\2\2st\t\22\2\2t$\3\2\2\2uv\t\23\2\2v&\3\2\2\2wx\t\24\2\2x(\3\2\2"+
		"\2yz\t\25\2\2z*\3\2\2\2{|\t\26\2\2|,\3\2\2\2}~\t\27\2\2~.\3\2\2\2\177"+
		"\u0080\t\30\2\2\u0080\60\3\2\2\2\u0081\u0082\t\31\2\2\u0082\62\3\2\2\2"+
		"\u0083\u0084\t\32\2\2\u0084\64\3\2\2\2\u0085\u0086\t\33\2\2\u0086\66\3"+
		"\2\2\2\u0087\u0088\5\31\r\2\u0088\u0089\5\t\5\2\u0089\u008a\5\23\n\2\u008a"+
		"8\3\2\2\2\u008b\u008c\7,\2\2\u008c:\3\2\2\2\u008d\u008e\7+\2\2\u008e<"+
		"\3\2\2\2\u008f\u0090\7<\2\2\u0090>\3\2\2\2\u0091\u0092\7.\2\2\u0092@\3"+
		"\2\2\2\u0093\u0094\7/\2\2\u0094B\3\2\2\2\u0095\u0096\7*\2\2\u0096D\3\2"+
		"\2\2\u0097\u0098\7-\2\2\u0098F\3\2\2\2\u0099\u009a\7\61\2\2\u009aH\3\2"+
		"\2\2\u009b\u009d\7\17\2\2\u009c\u009b\3\2\2\2\u009c\u009d\3\2\2\2\u009d"+
		"\u009e\3\2\2\2\u009e\u009f\7\f\2\2\u009fJ\3\2\2\2\u00a0\u00a2\t\34\2\2"+
		"\u00a1\u00a0\3\2\2\2\u00a2\u00a3\3\2\2\2\u00a3\u00a1\3\2\2\2\u00a3\u00a4"+
		"\3\2\2\2\u00a4\u00a5\3\2\2\2\u00a5\u00a6\b&\2\2\u00a6L\3\2\2\2\u00a7\u00ab"+
		"\7=\2\2\u00a8\u00aa\n\35\2\2\u00a9\u00a8\3\2\2\2\u00aa\u00ad\3\2\2\2\u00ab"+
		"\u00a9\3\2\2\2\u00ab\u00ac\3\2\2\2\u00ac\u00ae\3\2\2\2\u00ad\u00ab\3\2"+
		"\2\2\u00ae\u00af\b\'\2\2\u00afN\3\2\2\2\u00b0\u00b2\t\36\2\2\u00b1\u00b0"+
		"\3\2\2\2\u00b2\u00b3\3\2\2\2\u00b3\u00b1\3\2\2\2\u00b3\u00b4\3\2\2\2\u00b4"+
		"P\3\2\2\2\u00b5\u00b7\t\37\2\2\u00b6\u00b8\t \2\2\u00b7\u00b6\3\2\2\2"+
		"\u00b8\u00b9\3\2\2\2\u00b9\u00b7\3\2\2\2\u00b9\u00ba\3\2\2\2\u00baR\3"+
		"\2\2\2\b\2\u009c\u00a3\u00ab\u00b3\u00b9\3\2\3\2";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
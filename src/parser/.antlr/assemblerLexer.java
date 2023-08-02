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
		LDI=1, OPENING_BRACKET=2, CLOSEING_BRACKET=3, COLON=4, COMMA=5, WS=6, 
		NUMBER=7, IDENTIFIER=8;
	public static String[] channelNames = {
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	};

	public static String[] modeNames = {
		"DEFAULT_MODE"
	};

	private static String[] makeRuleNames() {
		return new String[] {
			"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", 
			"O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "LDI", "OPENING_BRACKET", 
			"CLOSEING_BRACKET", "COLON", "COMMA", "WS", "NUMBER", "IDENTIFIER"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, null, "'('", "')'", "':'", "','"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "LDI", "OPENING_BRACKET", "CLOSEING_BRACKET", "COLON", "COMMA", 
			"WS", "NUMBER", "IDENTIFIER"
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
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\2\n\u0099\b\1\4\2\t"+
		"\2\4\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4\13"+
		"\t\13\4\f\t\f\4\r\t\r\4\16\t\16\4\17\t\17\4\20\t\20\4\21\t\21\4\22\t\22"+
		"\4\23\t\23\4\24\t\24\4\25\t\25\4\26\t\26\4\27\t\27\4\30\t\30\4\31\t\31"+
		"\4\32\t\32\4\33\t\33\4\34\t\34\4\35\t\35\4\36\t\36\4\37\t\37\4 \t \4!"+
		"\t!\4\"\t\"\4#\t#\3\2\3\2\3\3\3\3\3\4\3\4\3\5\3\5\3\6\3\6\3\7\3\7\3\b"+
		"\3\b\3\t\3\t\3\n\3\n\3\13\3\13\3\f\3\f\3\r\3\r\3\16\3\16\3\17\3\17\3\20"+
		"\3\20\3\21\3\21\3\22\3\22\3\23\3\23\3\24\3\24\3\25\3\25\3\26\3\26\3\27"+
		"\3\27\3\30\3\30\3\31\3\31\3\32\3\32\3\33\3\33\3\34\3\34\3\34\3\34\3\35"+
		"\3\35\3\36\3\36\3\37\3\37\3 \3 \3!\6!\u0089\n!\r!\16!\u008a\3!\3!\3\""+
		"\6\"\u0090\n\"\r\"\16\"\u0091\3#\3#\6#\u0096\n#\r#\16#\u0097\2\2$\3\2"+
		"\5\2\7\2\t\2\13\2\r\2\17\2\21\2\23\2\25\2\27\2\31\2\33\2\35\2\37\2!\2"+
		"#\2%\2\'\2)\2+\2-\2/\2\61\2\63\2\65\2\67\39\4;\5=\6?\7A\bC\tE\n\3\2 \4"+
		"\2CCcc\4\2DDdd\4\2EEee\4\2FFff\4\2GGgg\4\2HHhh\4\2IIii\4\2JJjj\4\2KKk"+
		"k\4\2LLll\4\2MMmm\4\2NNnn\4\2OOoo\4\2PPpp\4\2QQqq\4\2RRrr\4\2SSss\4\2"+
		"TTtt\4\2UUuu\4\2VVvv\4\2WWww\4\2XXxx\4\2YYyy\4\2ZZzz\4\2[[{{\4\2\\\\|"+
		"|\5\2\13\f\16\17\"\"\3\2\62;\4\2C\\c|\6\2\62;C\\aac|\2\u0081\2\67\3\2"+
		"\2\2\29\3\2\2\2\2;\3\2\2\2\2=\3\2\2\2\2?\3\2\2\2\2A\3\2\2\2\2C\3\2\2\2"+
		"\2E\3\2\2\2\3G\3\2\2\2\5I\3\2\2\2\7K\3\2\2\2\tM\3\2\2\2\13O\3\2\2\2\r"+
		"Q\3\2\2\2\17S\3\2\2\2\21U\3\2\2\2\23W\3\2\2\2\25Y\3\2\2\2\27[\3\2\2\2"+
		"\31]\3\2\2\2\33_\3\2\2\2\35a\3\2\2\2\37c\3\2\2\2!e\3\2\2\2#g\3\2\2\2%"+
		"i\3\2\2\2\'k\3\2\2\2)m\3\2\2\2+o\3\2\2\2-q\3\2\2\2/s\3\2\2\2\61u\3\2\2"+
		"\2\63w\3\2\2\2\65y\3\2\2\2\67{\3\2\2\29\177\3\2\2\2;\u0081\3\2\2\2=\u0083"+
		"\3\2\2\2?\u0085\3\2\2\2A\u0088\3\2\2\2C\u008f\3\2\2\2E\u0093\3\2\2\2G"+
		"H\t\2\2\2H\4\3\2\2\2IJ\t\3\2\2J\6\3\2\2\2KL\t\4\2\2L\b\3\2\2\2MN\t\5\2"+
		"\2N\n\3\2\2\2OP\t\6\2\2P\f\3\2\2\2QR\t\7\2\2R\16\3\2\2\2ST\t\b\2\2T\20"+
		"\3\2\2\2UV\t\t\2\2V\22\3\2\2\2WX\t\n\2\2X\24\3\2\2\2YZ\t\13\2\2Z\26\3"+
		"\2\2\2[\\\t\f\2\2\\\30\3\2\2\2]^\t\r\2\2^\32\3\2\2\2_`\t\16\2\2`\34\3"+
		"\2\2\2ab\t\17\2\2b\36\3\2\2\2cd\t\20\2\2d \3\2\2\2ef\t\21\2\2f\"\3\2\2"+
		"\2gh\t\22\2\2h$\3\2\2\2ij\t\23\2\2j&\3\2\2\2kl\t\24\2\2l(\3\2\2\2mn\t"+
		"\25\2\2n*\3\2\2\2op\t\26\2\2p,\3\2\2\2qr\t\27\2\2r.\3\2\2\2st\t\30\2\2"+
		"t\60\3\2\2\2uv\t\31\2\2v\62\3\2\2\2wx\t\32\2\2x\64\3\2\2\2yz\t\33\2\2"+
		"z\66\3\2\2\2{|\5\31\r\2|}\5\t\5\2}~\5\23\n\2~8\3\2\2\2\177\u0080\7*\2"+
		"\2\u0080:\3\2\2\2\u0081\u0082\7+\2\2\u0082<\3\2\2\2\u0083\u0084\7<\2\2"+
		"\u0084>\3\2\2\2\u0085\u0086\7.\2\2\u0086@\3\2\2\2\u0087\u0089\t\34\2\2"+
		"\u0088\u0087\3\2\2\2\u0089\u008a\3\2\2\2\u008a\u0088\3\2\2\2\u008a\u008b"+
		"\3\2\2\2\u008b\u008c\3\2\2\2\u008c\u008d\b!\2\2\u008dB\3\2\2\2\u008e\u0090"+
		"\t\35\2\2\u008f\u008e\3\2\2\2\u0090\u0091\3\2\2\2\u0091\u008f\3\2\2\2"+
		"\u0091\u0092\3\2\2\2\u0092D\3\2\2\2\u0093\u0095\t\36\2\2\u0094\u0096\t"+
		"\37\2\2\u0095\u0094\3\2\2\2\u0096\u0097\3\2\2\2\u0097\u0095\3\2\2\2\u0097"+
		"\u0098\3\2\2\2\u0098F\3\2\2\2\6\2\u008a\u0091\u0097\3\2\3\2";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
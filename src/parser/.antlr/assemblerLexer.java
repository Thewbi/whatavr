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
		ADD=1, CALL=2, CLR=3, EOR=4, LDI=5, OUT=6, POP=7, PUSH=8, RCALL=9, RET=10, 
		RJMP=11, NEWLINE=12, WS=13, LINE_COMMENT=14, STRING=15, ASTERISK=16, AT=17, 
		CLOSEING_BRACKET=18, COLON=19, COMMA=20, CSEG=21, DEF=22, DOT=23, ELSE=24, 
		END_MACRO=25, ENDIF=26, EQUALS=27, EQU=28, ERROR=29, GT=30, IF=31, INCLUDE=32, 
		LEFT_SHIFT=33, LT=34, MACRO=35, MINUS=36, OPENING_BRACKET=37, ORG=38, 
		PLUS=39, RIGHT_SHIFT=40, SLASH=41, NUMBER=42, HEX_NUMBER=43, IDENTIFIER=44;
	public static String[] channelNames = {
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	};

	public static String[] modeNames = {
		"DEFAULT_MODE"
	};

	private static String[] makeRuleNames() {
		return new String[] {
			"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", 
			"O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "ADD", "CALL", 
			"CLR", "EOR", "LDI", "OUT", "POP", "PUSH", "RCALL", "RET", "RJMP", "NEWLINE", 
			"WS", "LINE_COMMENT", "STRING", "ASTERISK", "AT", "CLOSEING_BRACKET", 
			"COLON", "COMMA", "CSEG", "DEF", "DOT", "ELSE", "END_MACRO", "ENDIF", 
			"EQUALS", "EQU", "ERROR", "GT", "IF", "INCLUDE", "LEFT_SHIFT", "LT", 
			"MACRO", "MINUS", "OPENING_BRACKET", "ORG", "PLUS", "RIGHT_SHIFT", "SLASH", 
			"NUMBER", "HEX_NUMBER", "IDENTIFIER"
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
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\2.\u0182\b\1\4\2\t"+
		"\2\4\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4\13"+
		"\t\13\4\f\t\f\4\r\t\r\4\16\t\16\4\17\t\17\4\20\t\20\4\21\t\21\4\22\t\22"+
		"\4\23\t\23\4\24\t\24\4\25\t\25\4\26\t\26\4\27\t\27\4\30\t\30\4\31\t\31"+
		"\4\32\t\32\4\33\t\33\4\34\t\34\4\35\t\35\4\36\t\36\4\37\t\37\4 \t \4!"+
		"\t!\4\"\t\"\4#\t#\4$\t$\4%\t%\4&\t&\4\'\t\'\4(\t(\4)\t)\4*\t*\4+\t+\4"+
		",\t,\4-\t-\4.\t.\4/\t/\4\60\t\60\4\61\t\61\4\62\t\62\4\63\t\63\4\64\t"+
		"\64\4\65\t\65\4\66\t\66\4\67\t\67\48\t8\49\t9\4:\t:\4;\t;\4<\t<\4=\t="+
		"\4>\t>\4?\t?\4@\t@\4A\tA\4B\tB\4C\tC\4D\tD\4E\tE\4F\tF\4G\tG\3\2\3\2\3"+
		"\3\3\3\3\4\3\4\3\5\3\5\3\6\3\6\3\7\3\7\3\b\3\b\3\t\3\t\3\n\3\n\3\13\3"+
		"\13\3\f\3\f\3\r\3\r\3\16\3\16\3\17\3\17\3\20\3\20\3\21\3\21\3\22\3\22"+
		"\3\23\3\23\3\24\3\24\3\25\3\25\3\26\3\26\3\27\3\27\3\30\3\30\3\31\3\31"+
		"\3\32\3\32\3\33\3\33\3\34\3\34\3\34\3\34\3\35\3\35\3\35\3\35\3\35\3\36"+
		"\3\36\3\36\3\36\3\37\3\37\3\37\3\37\3 \3 \3 \3 \3!\3!\3!\3!\3\"\3\"\3"+
		"\"\3\"\3#\3#\3#\3#\3#\3$\3$\3$\3$\3$\3$\3%\3%\3%\3%\3&\3&\3&\3&\3&\3\'"+
		"\5\'\u00f6\n\'\3\'\3\'\3(\6(\u00fb\n(\r(\16(\u00fc\3(\3(\3)\3)\7)\u0103"+
		"\n)\f)\16)\u0106\13)\3)\3)\3*\3*\3*\3*\7*\u010e\n*\f*\16*\u0111\13*\3"+
		"*\3*\3+\3+\3,\3,\3-\3-\3.\3.\3/\3/\3\60\3\60\3\60\3\60\3\60\3\61\3\61"+
		"\3\61\3\61\3\62\3\62\3\63\3\63\3\63\3\63\3\63\3\64\3\64\3\64\3\64\3\64"+
		"\3\64\3\64\3\64\3\64\3\65\3\65\3\65\3\65\3\65\3\65\3\66\3\66\3\67\3\67"+
		"\3\67\3\67\38\38\38\38\38\38\39\39\3:\3:\3:\3;\3;\3;\3;\3;\3;\3;\3;\3"+
		"<\3<\3<\3=\3=\3>\3>\3>\3>\3>\3>\3?\3?\3@\3@\3A\3A\3A\3A\3B\3B\3C\3C\3"+
		"C\3D\3D\3E\6E\u0172\nE\rE\16E\u0173\3F\3F\3F\6F\u0179\nF\rF\16F\u017a"+
		"\3G\3G\6G\u017f\nG\rG\16G\u0180\2\2H\3\2\5\2\7\2\t\2\13\2\r\2\17\2\21"+
		"\2\23\2\25\2\27\2\31\2\33\2\35\2\37\2!\2#\2%\2\'\2)\2+\2-\2/\2\61\2\63"+
		"\2\65\2\67\39\4;\5=\6?\7A\bC\tE\nG\13I\fK\rM\16O\17Q\20S\21U\22W\23Y\24"+
		"[\25]\26_\27a\30c\31e\32g\33i\34k\35m\36o\37q s!u\"w#y${%}&\177\'\u0081"+
		"(\u0083)\u0085*\u0087+\u0089,\u008b-\u008d.\3\2#\4\2CCcc\4\2DDdd\4\2E"+
		"Eee\4\2FFff\4\2GGgg\4\2HHhh\4\2IIii\4\2JJjj\4\2KKkk\4\2LLll\4\2MMmm\4"+
		"\2NNnn\4\2OOoo\4\2PPpp\4\2QQqq\4\2RRrr\4\2SSss\4\2TTtt\4\2UUuu\4\2VVv"+
		"v\4\2WWww\4\2XXxx\4\2YYyy\4\2ZZzz\4\2[[{{\4\2\\\\||\5\2\13\f\16\17\"\""+
		"\4\2\f\f\17\17\3\2$$\3\2\62;\5\2\62;CHch\4\2C\\c|\6\2\62;C\\aac|\2\u016f"+
		"\2\67\3\2\2\2\29\3\2\2\2\2;\3\2\2\2\2=\3\2\2\2\2?\3\2\2\2\2A\3\2\2\2\2"+
		"C\3\2\2\2\2E\3\2\2\2\2G\3\2\2\2\2I\3\2\2\2\2K\3\2\2\2\2M\3\2\2\2\2O\3"+
		"\2\2\2\2Q\3\2\2\2\2S\3\2\2\2\2U\3\2\2\2\2W\3\2\2\2\2Y\3\2\2\2\2[\3\2\2"+
		"\2\2]\3\2\2\2\2_\3\2\2\2\2a\3\2\2\2\2c\3\2\2\2\2e\3\2\2\2\2g\3\2\2\2\2"+
		"i\3\2\2\2\2k\3\2\2\2\2m\3\2\2\2\2o\3\2\2\2\2q\3\2\2\2\2s\3\2\2\2\2u\3"+
		"\2\2\2\2w\3\2\2\2\2y\3\2\2\2\2{\3\2\2\2\2}\3\2\2\2\2\177\3\2\2\2\2\u0081"+
		"\3\2\2\2\2\u0083\3\2\2\2\2\u0085\3\2\2\2\2\u0087\3\2\2\2\2\u0089\3\2\2"+
		"\2\2\u008b\3\2\2\2\2\u008d\3\2\2\2\3\u008f\3\2\2\2\5\u0091\3\2\2\2\7\u0093"+
		"\3\2\2\2\t\u0095\3\2\2\2\13\u0097\3\2\2\2\r\u0099\3\2\2\2\17\u009b\3\2"+
		"\2\2\21\u009d\3\2\2\2\23\u009f\3\2\2\2\25\u00a1\3\2\2\2\27\u00a3\3\2\2"+
		"\2\31\u00a5\3\2\2\2\33\u00a7\3\2\2\2\35\u00a9\3\2\2\2\37\u00ab\3\2\2\2"+
		"!\u00ad\3\2\2\2#\u00af\3\2\2\2%\u00b1\3\2\2\2\'\u00b3\3\2\2\2)\u00b5\3"+
		"\2\2\2+\u00b7\3\2\2\2-\u00b9\3\2\2\2/\u00bb\3\2\2\2\61\u00bd\3\2\2\2\63"+
		"\u00bf\3\2\2\2\65\u00c1\3\2\2\2\67\u00c3\3\2\2\29\u00c7\3\2\2\2;\u00cc"+
		"\3\2\2\2=\u00d0\3\2\2\2?\u00d4\3\2\2\2A\u00d8\3\2\2\2C\u00dc\3\2\2\2E"+
		"\u00e0\3\2\2\2G\u00e5\3\2\2\2I\u00eb\3\2\2\2K\u00ef\3\2\2\2M\u00f5\3\2"+
		"\2\2O\u00fa\3\2\2\2Q\u0100\3\2\2\2S\u0109\3\2\2\2U\u0114\3\2\2\2W\u0116"+
		"\3\2\2\2Y\u0118\3\2\2\2[\u011a\3\2\2\2]\u011c\3\2\2\2_\u011e\3\2\2\2a"+
		"\u0123\3\2\2\2c\u0127\3\2\2\2e\u0129\3\2\2\2g\u012e\3\2\2\2i\u0137\3\2"+
		"\2\2k\u013d\3\2\2\2m\u013f\3\2\2\2o\u0143\3\2\2\2q\u0149\3\2\2\2s\u014b"+
		"\3\2\2\2u\u014e\3\2\2\2w\u0156\3\2\2\2y\u0159\3\2\2\2{\u015b\3\2\2\2}"+
		"\u0161\3\2\2\2\177\u0163\3\2\2\2\u0081\u0165\3\2\2\2\u0083\u0169\3\2\2"+
		"\2\u0085\u016b\3\2\2\2\u0087\u016e\3\2\2\2\u0089\u0171\3\2\2\2\u008b\u0175"+
		"\3\2\2\2\u008d\u017c\3\2\2\2\u008f\u0090\t\2\2\2\u0090\4\3\2\2\2\u0091"+
		"\u0092\t\3\2\2\u0092\6\3\2\2\2\u0093\u0094\t\4\2\2\u0094\b\3\2\2\2\u0095"+
		"\u0096\t\5\2\2\u0096\n\3\2\2\2\u0097\u0098\t\6\2\2\u0098\f\3\2\2\2\u0099"+
		"\u009a\t\7\2\2\u009a\16\3\2\2\2\u009b\u009c\t\b\2\2\u009c\20\3\2\2\2\u009d"+
		"\u009e\t\t\2\2\u009e\22\3\2\2\2\u009f\u00a0\t\n\2\2\u00a0\24\3\2\2\2\u00a1"+
		"\u00a2\t\13\2\2\u00a2\26\3\2\2\2\u00a3\u00a4\t\f\2\2\u00a4\30\3\2\2\2"+
		"\u00a5\u00a6\t\r\2\2\u00a6\32\3\2\2\2\u00a7\u00a8\t\16\2\2\u00a8\34\3"+
		"\2\2\2\u00a9\u00aa\t\17\2\2\u00aa\36\3\2\2\2\u00ab\u00ac\t\20\2\2\u00ac"+
		" \3\2\2\2\u00ad\u00ae\t\21\2\2\u00ae\"\3\2\2\2\u00af\u00b0\t\22\2\2\u00b0"+
		"$\3\2\2\2\u00b1\u00b2\t\23\2\2\u00b2&\3\2\2\2\u00b3\u00b4\t\24\2\2\u00b4"+
		"(\3\2\2\2\u00b5\u00b6\t\25\2\2\u00b6*\3\2\2\2\u00b7\u00b8\t\26\2\2\u00b8"+
		",\3\2\2\2\u00b9\u00ba\t\27\2\2\u00ba.\3\2\2\2\u00bb\u00bc\t\30\2\2\u00bc"+
		"\60\3\2\2\2\u00bd\u00be\t\31\2\2\u00be\62\3\2\2\2\u00bf\u00c0\t\32\2\2"+
		"\u00c0\64\3\2\2\2\u00c1\u00c2\t\33\2\2\u00c2\66\3\2\2\2\u00c3\u00c4\5"+
		"\3\2\2\u00c4\u00c5\5\t\5\2\u00c5\u00c6\5\t\5\2\u00c68\3\2\2\2\u00c7\u00c8"+
		"\5\7\4\2\u00c8\u00c9\5\3\2\2\u00c9\u00ca\5\31\r\2\u00ca\u00cb\5\31\r\2"+
		"\u00cb:\3\2\2\2\u00cc\u00cd\5\7\4\2\u00cd\u00ce\5\31\r\2\u00ce\u00cf\5"+
		"%\23\2\u00cf<\3\2\2\2\u00d0\u00d1\5\13\6\2\u00d1\u00d2\5\37\20\2\u00d2"+
		"\u00d3\5%\23\2\u00d3>\3\2\2\2\u00d4\u00d5\5\31\r\2\u00d5\u00d6\5\t\5\2"+
		"\u00d6\u00d7\5\23\n\2\u00d7@\3\2\2\2\u00d8\u00d9\5\37\20\2\u00d9\u00da"+
		"\5+\26\2\u00da\u00db\5)\25\2\u00dbB\3\2\2\2\u00dc\u00dd\5!\21\2\u00dd"+
		"\u00de\5\37\20\2\u00de\u00df\5!\21\2\u00dfD\3\2\2\2\u00e0\u00e1\5!\21"+
		"\2\u00e1\u00e2\5+\26\2\u00e2\u00e3\5\'\24\2\u00e3\u00e4\5\21\t\2\u00e4"+
		"F\3\2\2\2\u00e5\u00e6\5%\23\2\u00e6\u00e7\5\7\4\2\u00e7\u00e8\5\3\2\2"+
		"\u00e8\u00e9\5\31\r\2\u00e9\u00ea\5\31\r\2\u00eaH\3\2\2\2\u00eb\u00ec"+
		"\5%\23\2\u00ec\u00ed\5\13\6\2\u00ed\u00ee\5)\25\2\u00eeJ\3\2\2\2\u00ef"+
		"\u00f0\5%\23\2\u00f0\u00f1\5\25\13\2\u00f1\u00f2\5\33\16\2\u00f2\u00f3"+
		"\5!\21\2\u00f3L\3\2\2\2\u00f4\u00f6\7\17\2\2\u00f5\u00f4\3\2\2\2\u00f5"+
		"\u00f6\3\2\2\2\u00f6\u00f7\3\2\2\2\u00f7\u00f8\7\f\2\2\u00f8N\3\2\2\2"+
		"\u00f9\u00fb\t\34\2\2\u00fa\u00f9\3\2\2\2\u00fb\u00fc\3\2\2\2\u00fc\u00fa"+
		"\3\2\2\2\u00fc\u00fd\3\2\2\2\u00fd\u00fe\3\2\2\2\u00fe\u00ff\b(\2\2\u00ff"+
		"P\3\2\2\2\u0100\u0104\7=\2\2\u0101\u0103\n\35\2\2\u0102\u0101\3\2\2\2"+
		"\u0103\u0106\3\2\2\2\u0104\u0102\3\2\2\2\u0104\u0105\3\2\2\2\u0105\u0107"+
		"\3\2\2\2\u0106\u0104\3\2\2\2\u0107\u0108\b)\3\2\u0108R\3\2\2\2\u0109\u010f"+
		"\7$\2\2\u010a\u010b\7$\2\2\u010b\u010e\7$\2\2\u010c\u010e\n\36\2\2\u010d"+
		"\u010a\3\2\2\2\u010d\u010c\3\2\2\2\u010e\u0111\3\2\2\2\u010f\u010d\3\2"+
		"\2\2\u010f\u0110\3\2\2\2\u0110\u0112\3\2\2\2\u0111\u010f\3\2\2\2\u0112"+
		"\u0113\7$\2\2\u0113T\3\2\2\2\u0114\u0115\7,\2\2\u0115V\3\2\2\2\u0116\u0117"+
		"\7B\2\2\u0117X\3\2\2\2\u0118\u0119\7+\2\2\u0119Z\3\2\2\2\u011a\u011b\7"+
		"<\2\2\u011b\\\3\2\2\2\u011c\u011d\7.\2\2\u011d^\3\2\2\2\u011e\u011f\7"+
		"e\2\2\u011f\u0120\7u\2\2\u0120\u0121\7g\2\2\u0121\u0122\7i\2\2\u0122`"+
		"\3\2\2\2\u0123\u0124\7f\2\2\u0124\u0125\7g\2\2\u0125\u0126\7h\2\2\u0126"+
		"b\3\2\2\2\u0127\u0128\7\60\2\2\u0128d\3\2\2\2\u0129\u012a\7g\2\2\u012a"+
		"\u012b\7n\2\2\u012b\u012c\7u\2\2\u012c\u012d\7g\2\2\u012df\3\2\2\2\u012e"+
		"\u012f\7g\2\2\u012f\u0130\7p\2\2\u0130\u0131\7f\2\2\u0131\u0132\7o\2\2"+
		"\u0132\u0133\7c\2\2\u0133\u0134\7e\2\2\u0134\u0135\7t\2\2\u0135\u0136"+
		"\7q\2\2\u0136h\3\2\2\2\u0137\u0138\7g\2\2\u0138\u0139\7p\2\2\u0139\u013a"+
		"\7f\2\2\u013a\u013b\7k\2\2\u013b\u013c\7h\2\2\u013cj\3\2\2\2\u013d\u013e"+
		"\7?\2\2\u013el\3\2\2\2\u013f\u0140\7g\2\2\u0140\u0141\7s\2\2\u0141\u0142"+
		"\7w\2\2\u0142n\3\2\2\2\u0143\u0144\7g\2\2\u0144\u0145\7t\2\2\u0145\u0146"+
		"\7t\2\2\u0146\u0147\7q\2\2\u0147\u0148\7t\2\2\u0148p\3\2\2\2\u0149\u014a"+
		"\7@\2\2\u014ar\3\2\2\2\u014b\u014c\7k\2\2\u014c\u014d\7h\2\2\u014dt\3"+
		"\2\2\2\u014e\u014f\7k\2\2\u014f\u0150\7p\2\2\u0150\u0151\7e\2\2\u0151"+
		"\u0152\7n\2\2\u0152\u0153\7w\2\2\u0153\u0154\7f\2\2\u0154\u0155\7g\2\2"+
		"\u0155v\3\2\2\2\u0156\u0157\7>\2\2\u0157\u0158\7>\2\2\u0158x\3\2\2\2\u0159"+
		"\u015a\7>\2\2\u015az\3\2\2\2\u015b\u015c\7o\2\2\u015c\u015d\7c\2\2\u015d"+
		"\u015e\7e\2\2\u015e\u015f\7t\2\2\u015f\u0160\7q\2\2\u0160|\3\2\2\2\u0161"+
		"\u0162\7/\2\2\u0162~\3\2\2\2\u0163\u0164\7*\2\2\u0164\u0080\3\2\2\2\u0165"+
		"\u0166\7q\2\2\u0166\u0167\7t\2\2\u0167\u0168\7i\2\2\u0168\u0082\3\2\2"+
		"\2\u0169\u016a\7-\2\2\u016a\u0084\3\2\2\2\u016b\u016c\7@\2\2\u016c\u016d"+
		"\7@\2\2\u016d\u0086\3\2\2\2\u016e\u016f\7\61\2\2\u016f\u0088\3\2\2\2\u0170"+
		"\u0172\t\37\2\2\u0171\u0170\3\2\2\2\u0172\u0173\3\2\2\2\u0173\u0171\3"+
		"\2\2\2\u0173\u0174\3\2\2\2\u0174\u008a\3\2\2\2\u0175\u0176\7\62\2\2\u0176"+
		"\u0178\7z\2\2\u0177\u0179\t \2\2\u0178\u0177\3\2\2\2\u0179\u017a\3\2\2"+
		"\2\u017a\u0178\3\2\2\2\u017a\u017b\3\2\2\2\u017b\u008c\3\2\2\2\u017c\u017e"+
		"\t!\2\2\u017d\u017f\t\"\2\2\u017e\u017d\3\2\2\2\u017f\u0180\3\2\2\2\u0180"+
		"\u017e\3\2\2\2\u0180\u0181\3\2\2\2\u0181\u008e\3\2\2\2\13\2\u00f5\u00fc"+
		"\u0104\u010d\u010f\u0173\u017a\u0180\4\b\2\2\2\3\2";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}
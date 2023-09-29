lexer grammar assembler_lexer;

fragment A:[aA];
fragment B:[bB];
fragment C:[cC];
fragment D:[dD];
fragment E:[eE];
fragment F:[fF];
fragment G:[gG];
fragment H:[hH];
fragment I:[iI];
fragment J:[jJ];
fragment K:[kK];
fragment L:[lL];
fragment M:[mM];
fragment N:[nN];
fragment O:[oO];
fragment P:[pP];
fragment Q:[qQ];
fragment R:[rR];
fragment S:[sS];
fragment T:[tT];
fragment U:[uU];
fragment V:[vV];
fragment W:[wW];
fragment X:[xX];
fragment Y:[yY];
fragment Z:[zZ];

ADD : A D D ;
ADC : A D C ;
ADIW : A D I W ;
AND : A N D ;
ANDI : A N D I ;
ASR : A S R ;

BCLR : B C L R ;
BLD : B L D ;
BRBC : B R B C ;
BRBS : B R B S ;
BRCC : B R C C ;
BRCS : B R C S ;
BREAK : B R E A K ;
BREQ : B R E Q ;
BRGE : B R G E ;
BRHC : B R H C ;
BRHS : B R H S ;
BRID : B R I D ;
BRIE : B R I E ;
BRLO : B R L O ;
BRLT : B R L T ;
BRMI : B R M I ;
BRNE : B R N E ;
BRPL : B R P L ;
BRSH : B R S H ;
BRTC : B R T C ;
BRTS : B R T S ;
BRVC : B R V C ;
BRVS : B R V S ;
BSET : B S E T ;
BST : B S T ;
    
CALL : C A L L ;
CBI : C B I ;
CBR : C B R ;
CLC : C L C ;
CLH : C L H ;
CLI : C L I ;
CLN : C L N ;
CLR : C L R ;
CLS : C L S ;
CLT : C L T ;
CLV : C L V ;
CLZ : C L Z ;
COM : C O M ;
CP : C P ;
CPC : C P C ;
CPI : C P I ;
CPSE : C P S E ;

DEC : D E C ;
DES : D E S ;

EICALL : E I C A L L ;
EIJMP : E I J M P ;
ELPM : E L P M ;
EOR : E O R ;

FMUL : F M U L ;
FMULS : F M U L S ;
FMULSU : F M U L S U ;

ICALL : I C A L L ;
IJMP : I J M P ;
IN : I N ;
INC : I N C ;

JMP : J M P ;

LAC : L A C ;
LAS : L A S ;
LAT : L A T ;
LD : L D ;
LDD : L D D ;
LDI : L D I ; 
LDS : L D S ;
LPM : L P M ;
LSL : L S L ;
LSR : L S R ;

MOV : M O V ;
MOVW : M O V W ;
MUL : M U L ;
MULS : M U L S ;
MULSU : M U L S U ;

NEG : N E G ;
NOP : N O P ;

OR : O R ; 
ORI : O R I ;
OUT : O U T ;

POP : P O P ;
PUSH : P U S H ;

RCALL : R C A L L ;
RET : R E T ;
RETI : R E T I ;
RJMP : R J M P ;
ROL : R O L ;
ROR : R O R ;

SBC : S B C ;
SBCI : S B C I ;
SBI : S B I ;
SBIC : S B I C ;
SBIS : S B I S ;
SBIW : S B I W ;
SBR : S B R ;
SBRC : S B R C ;
SBRS : S B R S ;
SEC : S E C ;
SEH : S E H ;
SEI : S E I ;
SEN : S E N ;
SER : S E R ;
SES : S E S ;
SET : S E T ;
SEV : S E V ;
SEZ : S E Z ;
SLEEP : S L E E P ;
SPM : S P M ;
ST : S T ;
STD : S T D ;
STS : S T S ;
SUB : S U B ;
SUBI : S U B I ;
SWAP : S W A P ;
    
TST : T S T ;

WDR : W D R ;

XCH : X C H ;



ASTERISK : '*' ;
AT : '@' ;

BYTE : B Y T E ;
BACKSLASH: '\\' ;

CLOSEING_BRACKET : ')' ;
COLON : ':' ;
COMMA : ',' ;
CSEG : 'cseg' ;

DB : 'db' ;
DEF : 'def' ;
DEVICE : 'device' ;
DOT : '.' ;
DSEG : 'dseg' ;

ELSE : 'else' ;
END_MACRO : 'endmacro' ;
ENDIF : 'endif' ;
EQUALS : '=' ;
EQU : E Q U ;
ERROR : 'error' ;

GT : '>' ;

HASH_TAG : '#' ;

IF : 'if' ;
INCLUDE : 'include' ;

LEFT_SHIFT : '<<' ;
LT : '<' ;

MACRO : 'macro' ;
MINUS : '-' ;

OPENING_BRACKET : '(' ;
ORG : 'org' ;

PLUS : '+' ;

RIGHT_SHIFT : '>>' ;

SLASH : '/' ;

NEWLINE : '\r'? '\n' ;

//WS : [ \t\n\r\f]+ -> channel(HIDDEN) ;
WS : [ \t\n\r\f]+ -> skip  ;
//WS : [ \t\f]+ -> skip  ;

//LINE_COMMENT : ';' ~[\r\n]* -> channel(HIDDEN) ;
LINE_COMMENT 
    : 
    ';' ~[\r\n]* -> skip 
    ;

//BLOCK_COMMENT 
//    : 
    // non-greedy
//    '/*' .*? '*/' -> skip 
    // greedy
    //'/*' .* '*/' -> skip 
//    ;

BLOCK_COMMENT : '/*' (BLOCK_COMMENT|.)*? '*/' -> channel(HIDDEN) ;
DOUBLE_SLASH_LINE_COMMENT : '//' .*? '\n' -> channel(HIDDEN) ;

STRING : '"' ( '""' | ~'"' )* '"' ; // quote-quote is an escaped quote
CHAR : '\'' (.) '\'' ;
ESCAPED_CHAR : '\'' '\\' (.) '\'' ;

NUMBER : ( PLUS | MINUS )?[0-9]+ ;
HEX_NUMBER : ('0' 'x' | '$') [a-fA-F0-9]+ ;
BINARY_NUMBER : '0' 'b' [0,1]+ ;

IDENTIFIER : [a-zA-Z_]([a-zA-Z0-9_])* ;
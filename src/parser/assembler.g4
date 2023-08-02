// http://lab.antlr.org/

grammar assembler;

@tokenfactory{
pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;
}

asm_file : row (NEWLINE+ row)* NEWLINE* EOF ;

row : 
    label_definition? 
    (instruction ( (IDENTIFIER | expression | asm_intrinsic_usage) ( COMMA (IDENTIFIER | expression | asm_intrinsic_usage))? ) ) 
    ;

label_definition : IDENTIFIER COLON ;

parameter : IDENTIFIER ;

expression : NUMBER ;

asm_intrinsic_usage :
    IDENTIFIER OPENING_BRACKET IDENTIFIER CLOSEING_BRACKET
    ;

instruction :
    LDI
    ;

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

LDI : L D I ;

ASTERISK : '*';

CLOSEING_BRACKET : ')' ;
COLON : ':' ;
COMMA : ',' ;

MINUS : '-';

OPENING_BRACKET : '(' ;

PLUS : '+';

SLASH : '/';

NEWLINE : '\r'? '\n';

WS : [ \t\n\r\f]+ -> channel(HIDDEN) ;

LINE_COMMENT : ';' ~[\r\n]* -> channel(HIDDEN) ;

NUMBER : [0-9]+ ;

IDENTIFIER : [a-zA-Z]([a-zA-Z0-9_])+ ;





// TEXT : ~[ ,\n\r"]+ ;

//STRING : '"' ('""'|~'"')* '"' ; // quote-quote is an escaped quote
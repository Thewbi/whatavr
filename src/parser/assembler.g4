// http://lab.antlr.org/

grammar assembler;

@tokenfactory{
pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;
}

asmFile : row* EOF;

row : label_definition? (instruction ((parameter | macro_usage) ( COMMA (parameter | macro_usage))? )) ;

label_definition : IDENTIFIER COLON ;

parameter : IDENTIFIER | NUMBER;

macro_usage :
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

OPENING_BRACKET : '(' ;
CLOSEING_BRACKET : ')' ;
COLON : ':' ;
COMMA : ',' ;

WS : [ \t\n\r\f]+ -> channel(HIDDEN) ;

NUMBER : [0-9]+ ;

IDENTIFIER : [a-zA-Z]([a-zA-Z0-9_])+ ;

// TEXT : ~[ ,\n\r"]+ ;

//STRING : '"' ('""'|~'"')* '"' ; // quote-quote is an escaped quote
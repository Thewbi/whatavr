// http://lab.antlr.org/

grammar assembler;

@tokenfactory{
pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;
}

asm_file : NEWLINE* row (NEWLINE* row)* NEWLINE* EOF ;

row : 
    (
        label_definition? 
        ( instruction ( (IDENTIFIER | expression | asm_intrinsic_usage) ( COMMA (IDENTIFIER | expression | asm_intrinsic_usage))? ) )
    )
    | 
    asm_instrinsic_instruction
    ;

label_definition : IDENTIFIER COLON ;

parameter : IDENTIFIER ;

expression : NUMBER | HEX_NUMBER ;

asm_instrinsic_instruction :
    DOT ( 
        (INCLUDE STRING) 
        | DEF IDENTIFIER EQUALS (expression | IDENTIFIER)
        | EQU IDENTIFIER EQUALS (expression | IDENTIFIER)
        | CSEG 
        | ORG HEX_NUMBER
        | MACRO IDENTIFIER
        | END_MACRO
        | IF expression
        | ELSE
        | ENDIF
        | ERROR STRING
    )
    ;

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

LINE_COMMENT : ';' ~[\r\n]* -> channel(HIDDEN) ;

STRING : '"' ('""'|~'"')* '"' ; // quote-quote is an escaped quote

ASTERISK : '*';

CLOSEING_BRACKET : ')' ;
COLON : ':' ;
COMMA : ',' ;
CSEG : 'cseg' ;

DEF : 'def' ; 
DOT : '.' ;

ELSE : 'else' ;
END_MACRO : 'endmacro' ;
ENDIF : 'endif' ;
EQUALS : '=' ;
EQU : 'equ' ;
ERROR : 'error' ;

IF : 'if' ;
INCLUDE : 'include' ;

MACRO : 'macro' ;
MINUS : '-' ;

OPENING_BRACKET : '(' ;
ORG : 'org' ;

PLUS : '+' ;

SLASH : '/' ;

NEWLINE : '\r'? '\n' ;

WS : [ \t\n\r\f]+ -> channel(HIDDEN) ;

NUMBER : [0-9]+ ;
HEX_NUMBER: '0' 'x' [a-fA-F0-9]+ ;

IDENTIFIER : [a-zA-Z]([a-zA-Z0-9_])+ ;

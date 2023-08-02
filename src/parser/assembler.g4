// http://lab.antlr.org/

grammar assembler;

@tokenfactory{
pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;
}

asm_file : NEWLINE* row (NEWLINE* row)* NEWLINE* EOF ;

row : 
    macro_usage
    |
    label_definition
    |
    instruction
    |
    asm_instrinsic_instruction
    ;

instruction : mnemonic ( (IDENTIFIER | expression | asm_intrinsic_usage | macro_placeholder) ( COMMA (IDENTIFIER | expression | asm_intrinsic_usage | macro_placeholder) )? )?;

// examples: delayms 500
macro_usage : IDENTIFIER ( expression )* ;

label_definition : IDENTIFIER COLON ;

parameter : IDENTIFIER ;

macro_placeholder : AT NUMBER ;

expression : 
    NUMBER 
    | 
    HEX_NUMBER
    |
    IDENTIFIER
    |
    macro_placeholder
    |
    OPENING_BRACKET expression CLOSEING_BRACKET
    |
    expression LEFT_SHIFT expression
    |
    expression RIGHT_SHIFT expression
    |
    expression SLASH expression
    |
    expression GT expression
    |
    expression LT expression
    ;

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
    IDENTIFIER OPENING_BRACKET (IDENTIFIER | macro_placeholder) CLOSEING_BRACKET
    ;

mnemonic :
    ADD
    |
    CALL
    | CLR
    |
    EOR
    |
    LDI
    |
    OUT
    |
    POP
    | PUSH
    |
    RCALL
    | RET
    | RJMP
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

ADD : A D D ;

CALL : C A L L ;
CLR : C L R ;

EOR : E O R ;

LDI : L D I ;

OUT : O U T ;

POP : P O P ;
PUSH : P U S H ;

RCALL : R C A L L ;
RET : R E T ;
RJMP : R J M P ;

NEWLINE : '\r'? '\n' ;

//WS : [ \t\n\r\f]+ -> channel(HIDDEN) ;
WS : [ \t\n\r\f]+ -> skip  ;
//WS : [ \t\f]+ -> skip  ;

LINE_COMMENT : ';' ~[\r\n]* -> channel(HIDDEN) ;

STRING : '"' ('""'|~'"')* '"' ; // quote-quote is an escaped quote

ASTERISK : '*' ;
AT : '@' ;

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

GT : '>' ;

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





NUMBER : [0-9]+ ;
HEX_NUMBER: '0' 'x' [a-fA-F0-9]+ ;

IDENTIFIER : [a-zA-Z]([a-zA-Z0-9_])+ ;

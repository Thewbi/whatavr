// http://lab.antlr.org/

parser grammar assembler_parser;
options { tokenVocab=assembler_lexer; }

@tokenfactory{
pub type LocalTokenFactory<'input> = antlr_rust::token_factory::ArenaCommonFactory<'input>;
}

asm_file : NEWLINE* row ( NEWLINE* row )* NEWLINE* EOF ;

row : 
    asm_intrinsic_instruction
    |
    macro_usage
    |
    label_definition
    |
    instruction
    |
    preprocessor_directive
    ;

instruction : mnemonic ( param ( COMMA param )? )? ;

param : ( ( MINUS )? IDENTIFIER ( PLUS )? ) | expression | asm_intrinsic_usage | macro_placeholder | register_pair ;

register_pair : IDENTIFIER COLON IDENTIFIER ;

macro_usage : IDENTIFIER ( expression )* ;

label_definition : IDENTIFIER COLON ;

macro_placeholder : AT NUMBER ;

expression : 
    OPENING_BRACKET expression CLOSEING_BRACKET
    |
    expression LEFT_SHIFT expression
    |
    expression RIGHT_SHIFT expression
    |
    expression ASTERISK expression // multiplication
    |
    expression SLASH expression  // division
    |
    expression GT expression
    |
    expression LT expression
    |
//    expression EQUALS expression
//    |
    expression PLUS expression
    |
    expression MINUS expression
    |
    DOT ( PLUS | MINUS ) numeric
    |
    BINARY_NUMBER
    |
    HEX_NUMBER
    | 
    NUMBER 
    |
    IDENTIFIER
    |
    STRING
    |
    CHAR
    |
    ESCAPED_CHAR
    |
    macro_placeholder
    ;

numeric :
    BINARY_NUMBER
    |
    HEX_NUMBER
    | 
    NUMBER
    ;

asm_intrinsic_instruction :
    DOT (
        INCLUDE STRING
        |
        DEVICE IDENTIFIER
        |
        DEF IDENTIFIER EQUALS expression
        |
        EQU IDENTIFIER EQUALS expression
        | 
        CSEG
        |
        DSEG
        |
        DB byte_csv
        |
        BYTE expression
        |
        ORG ( HEX_NUMBER | IDENTIFIER )
        | 
        MACRO IDENTIFIER
        | 
        END_MACRO
        | 
        IF expression
        | 
        ELSE
        | 
        ENDIF
        | 
        ERROR STRING
    )
    ;

byte_csv :
    byte_csv COMMA ( HEX_NUMBER | NUMBER | STRING )
    |
    ( HEX_NUMBER | NUMBER | STRING )
    ;

asm_intrinsic_usage :
    IDENTIFIER OPENING_BRACKET ( expression | macro_placeholder ) CLOSEING_BRACKET
    ;

preprocessor_directive : 
    HASH_TAG ( IF | ENDIF | IDENTIFIER | HEX_NUMBER | BINARY_NUMBER | NUMBER )+
    ;

mnemonic :
    mnemonic_a
    |
    mnemonic_b
    |
    mnemonic_c
    |
    mnemonic_d
    |
    mnemonic_e
    |
    mnemonic_f
    |
    mnemonic_i
    |
    mnemonic_j
    |
    mnemonic_l
    |
    mnemonic_m
    |
    mnemonic_n
    |
    mnemonic_o
    |
    mnemonic_p
    |
    mnemonic_r
    |
    mnemonic_s
    |
    mnemonic_t
    |
    mnemonic_w
    |
    mnemonic_x
    ;

mnemonic_a :
    ADD | ADC | ADIW | AND | ANDI | ASR
    ;

mnemonic_b :
    BCLR | BLD | BRBC | BRBS | BRCC | BRCS | BREAK | BREQ | BRGE | BRHC | BRHS | BRID | BRIE | BRLO | BRLT | BRMI | BRNE | BRPL | BRSH | BRTC | BRTS | BRVC | BRVS | BSET | BST 
    ;

mnemonic_c :
    CALL | CBI | CBR | CLC | CLH | CLI | CLN | CLR | CLS | CLT | CLV | CLZ | COM | CP | CPC | CPI | CPSE 
    ;

mnemonic_d :
    DEC | DES
    ;

mnemonic_e :
    EICALL | EIJMP | ELPM | EOR
    ;

mnemonic_f :
    FMUL | FMULS | FMULSU
    ;

mnemonic_i :
    ICALL | IJMP | IN | INC
    ;

mnemonic_j :
    JMP
    ;

mnemonic_l :
    LAC | LAS | LAT | LD | LDD | LDI | LDS | LDS | LPM | LSL | LSR
    ; 

mnemonic_m :
    MOV | MOVW | MUL | MULS | MULSU
    ;

mnemonic_n :
    NEG | NOP
    ;

mnemonic_o :
    OR | ORI | OUT
    ;

mnemonic_p :
    POP | PUSH
    ;

mnemonic_r :
    RCALL | RET | RETI | RJMP | ROL|  ROR
    ;

mnemonic_s :
    SBC | SBCI | SBI | SBIC | SBIS | SBIW | SBR | SBRC | SBRS | SEC | SEH | SEI | SEN | SER | SES | SET | SEV | SEZ | SLEEP | SPM | ST | STD | STS | SUB | SUBI | SWAP
    ;

mnemonic_t :
    TST
    ;

mnemonic_w :
    WDR
    ;

mnemonic_x :
    XCH
    ;



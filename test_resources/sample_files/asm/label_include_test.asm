.include "label_include.inc"
; the second include will update the global symbol table and a warning is output
; for every updated variable!
.include "label_include.inc"

; no error for duplicate pragma once because the second one is skipped!
;.include "label_include_pragma_once.inc"
; no warning is outout since the command is skipped
;.include "label_include_pragma_once.inc"

; this will cause an error since this file recursively includes itself and generates a cycle
;.include "label_include_cyclic.inc"

; this overrides a variable's value in the global symbol table also the global symbol table
; has to change the location of where this variable was declared to this new location since
; it is overriden here
;
; The update/override has to cause a warning!
.equ    OVERRIDEME = 0x0002

.cseg                               ; code segments starts here ...
.org 0x0000                         ; no interrupt vector table, just enter code

; this will insert the same set of assembly instructions twice!
; The instructions have to contained twice in the generated code otherwise the assembly has a bug!
.include "label_include_code.inc"
.include "label_include_code.inc"

main:
    ; uses the RAMEND variable defined in "label_include.inc"
    ldi r24, HIGH(RAMEND)

    ; random normal instruction, nothing special
    clr r24

    ; teststring is used before it's address is defined!
    ; TODO: find a solution for ldi ZH, HIGH(teststring * 2). The assembly record has to contain the tree
    ; which models the HIGH(teststring * 2) expression since this expression has to be evaluated shortly
    ; before encoding and after the address of the symbol teststring is known!
    ldi ZH, HIGH(teststring * 2)

; teststring is used before it's address is defined!
teststring: .db "Hello world", 0
.include "label_include.inc"
# the second include will update the global symbol table and a warning is output
.include "label_include.inc"

# no error for duplicate pragma once because the second one is skipped!
.include "label_include_pragma_once.inc"
# no warning is outout since the command is skipped
.include "label_include_pragma_once.inc"

# this will cause an error since this file recursively includes itself and generates a cycle
.include "label_include_cyclic.inc"

.equ    OVERRIDEME = 0x0002

.cseg                               ; code segments starts here ...
.org 0x0000                         ; no interrupt vector table, just enter code

    ldi r16, HIGH(RAMEND)

main:
    clr r24
    ldi ZH, HIGH(teststring * 2)

teststring: .db "Hello world", 0
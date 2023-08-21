.include "m328Pdef.inc"

ldi r16, LOW(RAMEND)
out SPL, r16
ldi r16, HIGH(RAMEND)
out SPH, r16

call target_label

target_label: add r17, r16
use super::instruction_type::InstructionType;

pub struct InstructionDefinition {

    // id of instruction in atmel-0856-avr-instruction-set-manual.pdf
    pub index: u8,

    // type of this instruction as an enum
    pub instruction_type: InstructionType,

    // bit_pattern for decoding of an instruction
    //pub bit_pattern: String,
    pub bit_pattern: &'static str,

}
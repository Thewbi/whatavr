use super::{
    bit_pattern_matching::{bit_match, bit_pattern_match},
    instruction_definition::InstructionDefinition,
};
use std::collections::HashMap;

/// finds a command that matches data and returns that instruction definition
/// If no matching instruction is found, returns the instruction definition 'unknown'
pub fn decode_instruction<'a>(
    data: u16,
    instructions_list: &'static [InstructionDefinition],
    unknown_instruction: &'a InstructionDefinition,
    value_storage: &mut HashMap<char, u16>,
) -> &'a InstructionDefinition {
    log::info!("d: {data:#018b} {data:#06x}");

    // go through all the instructions
    for instruction in instructions_list {
        // find the instruction that matches the bit pattern
        if bit_match(data, &instruction.bit_pattern) {
            // parse the bit pattern into the values
            bit_pattern_match(data, &instruction.bit_pattern, value_storage);

            return &instruction;
        }
    }

    return unknown_instruction;
}

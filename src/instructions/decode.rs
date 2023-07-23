use std::collections::HashMap;
use super::{instruction_definition::InstructionDefinition, bit_pattern_matching::{bit_match, bit_pattern_match}};

/// finds a command that matches data and returns that instruction definition
/// If no matching instruction is found, returns the instruction definition 'unknown'
//fn decode<'a>(data:u16, instructions_list: &'a Vec<InstructionDefinition>, unknown_instruction: &'a InstructionDefinition, value_storage:&mut HashMap<char, u16>) -> &'a InstructionDefinition {
pub fn decode_instruction<'a>(data:u16, instructions_list: &'static [InstructionDefinition], 
    unknown_instruction: &'a InstructionDefinition, value_storage:&mut HashMap<char, u16>) -> &'a InstructionDefinition {

    for instruction in instructions_list 
    {
        if bit_match(data, &instruction.bit_pattern)
        {
            bit_pattern_match(data, &instruction.bit_pattern, value_storage);

            return &instruction;
        }
    }
        
    return unknown_instruction;
}
use crate::instructions::instruction_type::InstructionType;

use super::io_destination::IoDestination;

/// This struct is the model for a line in a assembler source code file
#[derive(Clone)]
pub struct AsmRecord {
    pub label: String,

    pub reg_1: u16,
    pub reg_2: u16,

    pub instruction_type: InstructionType,

    pub data: u16,

    pub target_label: String,

    pub io_dest: IoDestination,

    pub idx: usize,
}

impl AsmRecord {
    pub fn new(
        label: String,
        instruction_type: InstructionType,
        reg_1: u16,
        reg_2: u16,
        data: u16,
        target_label: String,
        io_dest: IoDestination,
    ) -> AsmRecord {
        AsmRecord {
            label: label,
            instruction_type: instruction_type,
            reg_1: reg_1,
            reg_2: reg_2,
            data: data,
            target_label: target_label,
            io_dest: io_dest,
            idx: 0usize,
        }
    }
}

use crate::instructions::instruction_type::InstructionType;

use super::io_destination::IoDestination;

use std::fmt;

/// This struct is the model for a line in a assembler source code file
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct AsmRecord {

    pub label: String,

    pub instruction_type: InstructionType,

    pub reg_1: u16,
    pub reg_2: u16,

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

    pub fn clear(&mut self) {
        self.label = String::default();
        self.instruction_type = InstructionType::UNKNOWN;
        self.reg_1 = 0xFF;
        self.reg_2 = 0xFF;
        self.data = u16::default();
        self.target_label = String::default();
        self.io_dest = IoDestination::UNKNOWN;
        self.idx = usize::default();
    }

    pub fn set_idx(&mut self, idx: usize) {
        self.idx = idx;
    }
    
}

impl fmt::Display for AsmRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(label:{}, instruction_type:{}, reg_1:{} {:#04x}, reg_2:{} {:#04x}, data:{} {:#04x}, target_label:{})", 
            self.label, 
            self.instruction_type.to_string(),
            self.reg_1, self.reg_1,
            self.reg_2, self.reg_2,
            self.data, self.data,
            self.target_label)
    }
}

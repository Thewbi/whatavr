use crate::instructions::instruction_type::InstructionType;

use super::io_destination::IoDestination;

use std::fmt;

/// This struct is the model for a line in a assembler source code file
#[derive(Debug, Clone)]
pub struct AsmRecord {

    pub label: String,

    pub instruction_type: InstructionType,

    pub reg_1: u16,
    pub reg_2: u16,

    pub data: u16,

    pub target_label: String,
    pub target_address: i16,

    pub io_dest: IoDestination,

    pub idx: u16,

    pub direct_data: Vec<u8>,

}

impl AsmRecord {

    pub fn new(
        label: String,
        instruction_type: InstructionType,
        reg_1: u16,
        reg_2: u16,
        data: u16,
        target_label: String,
        target_address: i16,
        io_dest: IoDestination,
    ) -> AsmRecord {
        AsmRecord {
            label: label,
            instruction_type: instruction_type,
            reg_1: reg_1,
            reg_2: reg_2,
            data: data,
            target_label: target_label,
            target_address: target_address,
            io_dest: io_dest,
            idx: 0u16,
            direct_data: Vec::default(),
        }
    }

    pub fn clear(&mut self) {
        self.label = String::default();
        self.instruction_type = InstructionType::UNKNOWN;
        self.reg_1 = 0xFF;
        self.reg_2 = 0xFF;
        self.data = u16::default();
        self.target_label = String::default();
        self.target_address = 0i16;
        self.io_dest = IoDestination::UNKNOWN;
        self.idx = u16::default();
        self.direct_data = Vec::default();
    }

    pub fn set_idx(&mut self, idx: u16) {
        self.idx = idx;
    }
    
}

impl Default for AsmRecord {
    fn default() -> Self {
        Self {
            label: String::default(),
            instruction_type: InstructionType::UNKNOWN,
            reg_1: 0xFF,
            reg_2: 0xFF,
            data: u16::default(),
            target_label: String::default(),
            target_address : 0i16,
            io_dest: IoDestination::UNKNOWN,
            idx: u16::default(),
            direct_data: Vec::default(),
        }
    }
}

impl fmt::Display for AsmRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(label:{:<10}, itype:{:<5}, reg_1:{:<3} {:#04x}, reg_2:{:<3} {:#04x}, data:{} {:#04x}, tgt_label:{}, tgt_addr:{})", 
            self.label, 
            self.instruction_type.to_string(),
            self.reg_1, self.reg_1,
            self.reg_2, self.reg_2,
            self.data, self.data,
            self.target_label, self.target_address)
    }
}

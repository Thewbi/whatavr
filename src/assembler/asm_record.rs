use crate::instructions::instruction_type::InstructionType;

use super::{io_destination::IoDestination, asm_record_type::AsmRecordType, node::Node};

use std::fmt;

/// This struct is the model for a line in a assembler source code file
#[derive(Debug, Clone)]
pub struct AsmRecord {

    pub record_type: AsmRecordType,

    pub label: String,

    pub instruction_type: InstructionType,

    pub reg_1: u16,
    pub reg_2: u16,

    pub expression_1: Option<Box<Node<String>>>,
    pub expression_2: Option<Box<Node<String>>>,

    pub data: u16,

    pub target_label: String,
    pub target_address: i16,

    pub io_dest: IoDestination,

    pub idx: u32,
    pub address: u32,

    pub direct_data: Vec<u8>,

    pub source_file: String,
    pub line: isize,
    pub column: isize,

    pub remove: bool,

}

impl AsmRecord {

    pub fn new(
        record_type: AsmRecordType,
        label: String,
        instruction_type: InstructionType,
        reg_1: u16,
        reg_2: u16,
        data: u16,
        target_label: String,
        target_address: i16,
        io_dest: IoDestination,
        source_file: String,
        line: isize,
        column: isize,
        remove: bool,
    ) -> AsmRecord {
        AsmRecord {
            record_type: record_type,
            label: label,
            instruction_type: instruction_type,
            reg_1: reg_1,
            reg_2: reg_2,
            expression_1: None,
            expression_2: None,
            data: data,
            target_label: target_label,
            target_address: target_address,
            io_dest: io_dest,
            idx: 0u32,
            address: 0u32,
            direct_data: Vec::default(),
            source_file: source_file,
            line: line,
            column: column,
            remove: false,
        }
    }

    pub fn clear(&mut self) {
        self.record_type = AsmRecordType::UNKNOWN;
        self.label = String::default();
        self.instruction_type = InstructionType::UNKNOWN;
        self.reg_1 = 0xFF;
        self.reg_2 = 0xFF;
        self.expression_1 = None;
        self.expression_2 = None;
        self.data = u16::default();
        self.target_label = String::default();
        self.target_address = 0i16;
        self.io_dest = IoDestination::UNKNOWN;
        self.idx = u32::default();
        self.address = u32::default();
        self.direct_data = Vec::default();
        self.source_file = String::default();
        self.line = isize::default();
        self.column = isize::default();
        self.remove = false;
    }

    pub fn set_idx(&mut self, idx: u32) {
        self.idx = idx;
    }

    pub fn set_address(&mut self, address: u32) {
        self.address = address;
    }
    
}

impl Default for AsmRecord {
    fn default() -> Self {
        Self {
            record_type: AsmRecordType::UNKNOWN,
            label: String::default(),
            instruction_type: InstructionType::UNKNOWN,
            reg_1: 0xFF,
            reg_2: 0xFF,
            expression_1: None,
            expression_2: None,
            data: u16::default(),
            target_label: String::default(),
            target_address : 0i16,
            io_dest: IoDestination::UNKNOWN,
            idx: u32::default(),
            address: u32::default(),
            direct_data: Vec::default(),
            source_file: String::default(),
            line: isize::default(),
            column: isize::default(),
            remove: false,
        }
    }
}

impl fmt::Display for AsmRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(idx:{:<4} {:#04x}, addr:{:<4} {:#04x}, label:{:<10}, rtype:{:<5}, itype:{:<5}, reg_1:{:<3} {:#04x}, reg_2:{:<3} {:#04x}, expr_1: {:?}, expr_2: {:?}, data:{} {:#04x}, tgt_label:{}, tgt_addr:{}, source_file:{}, line:{}, column:{})", 
            self.idx, self.idx,
            self.address, self.address,
            self.label, 
            self.record_type.to_string(), self.instruction_type.to_string(),
            self.reg_1, self.reg_1,
            self.reg_2, self.reg_2,
            self.expression_1, self.expression_2,
            self.data, self.data,
            self.target_label, self.target_address,
            self.source_file, self.line, self.column)
    }
}

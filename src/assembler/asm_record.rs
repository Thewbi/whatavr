use crate::instructions::instruction_type::InstructionType;

use super::io_destination::IoDestination;
use std::ops::{Deref, DerefMut};

/// This struct is the model for a line in a assembler source code file
#[derive(Debug, Clone)]
pub struct AsmRecord/*<'a, T>*/ {

    pub label: String,

    pub reg_1: u16,
    pub reg_2: u16,

    pub instruction_type: InstructionType,

    pub data: u16,

    pub target_label: String,

    pub io_dest: IoDestination,

    pub idx: usize,

    //pub text: T,

    //pub reffff: &'a Vec<&'a AsmRecord<'a, &'a str>>,
}

// impl<T> Deref for AsmRecord {

//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.idx
//     }
// }


impl/*<'a, T>*/ AsmRecord/*<'a, T>*/ {

    // pub fn new() -> AsmRecord{
    //     AsmRecord {
    //         label: "",
    //         instruction_type: InstructionType::Unknown,
    //         reg_1: "",
    //         reg_2: "",
    //         data: "",
    //         target_label: "",
    //         io_dest: IoDestination::UNKNOWN,
    //         idx: 0usize,
    //     }
    // }

    pub fn new(
        label: String,
        instruction_type: InstructionType,
        reg_1: u16,
        reg_2: u16,
        data: u16,
        target_label: String,
        io_dest: IoDestination,
        //text: T,
        //reffff: &'a Vec<&'a AsmRecord<'a, &'a str>>
    ) -> AsmRecord/*<'a, T>*/ {
        AsmRecord {
            label: label,
            instruction_type: instruction_type,
            reg_1: reg_1,
            reg_2: reg_2,
            data: data,
            target_label: target_label,
            io_dest: io_dest,
            idx: 0usize,
            //text: text,
            //reffff: reffff,
        }
    }

    pub fn clear(&mut self) {
        self.label = String::default();
        self.instruction_type = InstructionType::UNKNOWN;
        //self.reg_1 = u16::default();
        self.reg_1 = 0xFF;
        //self.reg_2 = u16::default();
        self.reg_2 = 0xFF;
        self.data = u16::default();
        self.target_label = String::default();
        self.io_dest = IoDestination::UNKNOWN;
        self.idx = usize::default();
        //self.text = T::default();
    }

    pub fn emit(&self) {
        log::info!("emit");
        //self.reffff.get(0).as_mut().unwrap().as_mut().data = 0x01;
        //self.reffff.first_mut().unwrap().reg_1 = 0x01;

        //let first = &mut self.reffff[0];

        //let mut ref0 = &mut self.reffff[0];
    }

    pub fn set_idx(&mut self, idx: usize) {
        self.idx = idx;
    }
}

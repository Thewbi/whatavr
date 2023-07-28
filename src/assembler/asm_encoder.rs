use std::collections::HashMap;

use crate::{ihex_mgmt::ihex_mgmt::Segment, instructions::instruction_type::InstructionType};

use super::{asm_record::AsmRecord, io_destination::IoDestination};

#[macro_export] macro_rules! HIGH {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            ($a >> 8u16)
        }
    }
}

#[macro_export] macro_rules! LOW {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            ($a & 0xFFu16)
        }
    }
}

#[macro_export] macro_rules! HIGH_I16 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            ($a >> 8u16)
        }
    }
}

#[macro_export] macro_rules! LOW_I16 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            ($a & 0xFFi16)
        }
    }
}

#[macro_export] macro_rules! HIGH_HIGH_I32 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            (($a >> 24) & 0xFFi32)
        }
    }
}

#[macro_export] macro_rules! HIGH_I32 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            (($a >> 16) & 0xFFi32)
        }
    }
}

#[macro_export] macro_rules! LOW_I32 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            (($a >> 8) & 0xFFi32)
        }
    }
}

#[macro_export] macro_rules! LOW_LOW_I32 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            (($a >> 0) & 0xFFi32)
        }
    }
}

pub struct AsmEncoder {

    pub labels: HashMap<String, usize>,

}

impl AsmEncoder {

    pub fn new() -> AsmEncoder {

        AsmEncoder {
            labels: HashMap::new(),
        }

    }

    // // initialize the stack
    // encode_ldi(&mut assembler_segment, &mut idx, 16u16, LOW!(RAMEND));
    // encode_out(&mut assembler_segment, &mut idx, IoDestination::SPL, 16u16);
    // encode_ldi(&mut assembler_segment, &mut idx, 16u16, HIGH!(RAMEND));
    // encode_out(&mut assembler_segment, &mut idx, IoDestination::SPH, 16u16);

    // create_label(&mut labels, String::from("main"), idx); // main:
    // encode_rjmp(&mut assembler_segment, &mut idx, &labels, &String::from("reset"));  // rjmp reset

    // create_label(&mut labels, String::from("swap"), idx); // swap:
    // encode_push(&mut assembler_segment, &mut idx, 16u16); // push r18
    // encode_mov(&mut assembler_segment, &mut idx, 18u16, 16u16); // mov r18, r16
    // encode_mov(&mut assembler_segment, &mut idx, 16u16, 17u16); // mov r16, r17
    // encode_mov(&mut assembler_segment, &mut idx, 17u16, 18u16); // mov r17, r18
    // encode_pop(&mut assembler_segment, &mut idx, 18u16); // pop r18
    // encode_ret(&mut assembler_segment, &mut idx); // ret

    // create_label(&mut labels, String::from("reset"), idx);  // reset:
    // encode_ldi(&mut assembler_segment, &mut idx, 18u16, 0x21); // ldi r18, 33d == 0x21

    // encode_ldi(&mut assembler_segment, &mut idx, 16u16, 0x0B); // ldi r16, 11
    // encode_ldi(&mut assembler_segment, &mut idx, 17u16, 0x16); // ldi r17, 22
    // encode_rcall(&mut assembler_segment, &mut idx, &labels, &String::from("swap"));
    // encode_rjmp(&mut assembler_segment, &mut idx, &labels, &String::from("main"));

    pub fn encode(&self, segment:&mut Segment, asm_record: &AsmRecord)
    {
        match asm_record.instruction_type {

            /*   6 */ InstructionType::ADD => { Self::encode_add(&self, segment, asm_record.reg_1, asm_record.reg_2); }
            /*  27 */ InstructionType::BRNE => { Self::encode_brne(&self, segment, &asm_record.target_label); }
            /*  36 */ InstructionType::CALL => { Self::encode_call(&self, segment, &asm_record.idx, &asm_record.target_label); }
            /*  53 */ InstructionType::DEC => { Self::encode_dec(&self, segment, asm_record.reg_1); }
            /*  66 */ InstructionType::JMP => { Self::encode_jmp(&self, segment, &asm_record.idx, &asm_record.target_label); }
            /*  73 */ InstructionType::LDI => { Self::encode_ldi(&self, segment, asm_record.reg_1, asm_record.data); }
            /*  79 */ InstructionType::MOV => { Self::encode_mov(&self, segment, asm_record.reg_1, asm_record.reg_2); }
            /*  88 */ InstructionType::OUT => { Self::encode_out(&self, segment, asm_record.io_dest, asm_record.reg_1); }
            /*  89 */ InstructionType::POP => { Self::encode_pop(&self, segment, asm_record.reg_1); }
            /*  90 */ InstructionType::PUSH => { Self::encode_push(&self, segment, asm_record.reg_1); }
            /*  91 */ InstructionType::RCALL => { Self::encode_rcall(&self, segment, &asm_record.idx, &asm_record.target_label); }
            /*  92 */ InstructionType::RET => { Self::encode_ret(&self, segment); }

            _ => panic!("Unknown instruction!")
        }
    }

    /// 5. ADC – Add with Carry
    /// add - Add without carry (Rd ← Rd + Rr)
    /// 0000 11rd dddd rrrr
    fn encode_add(&self, segment:&mut Segment, register_d: u16, register_r: u16)
    {
        // register is increased by 16 to arrive at the register id
        let register_d_offset: u16 = register_d;
        let register_r_offset: u16 = register_r;

        let r_mask: u16 = ((register_r_offset >> 4u16) << 9u16) | (register_r_offset & 0x0Fu16);

        let result: u16 = 0x0C00u16 | (r_mask | register_d_offset << 4u16);

        log::info!("ENC ADD: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC ADD: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 27. BRNE – Branch if Not Equal
    /// 1111 01kk kkkk k001
    fn encode_brne(&self, segment:&mut Segment, label: &String)
    {
        // register is increased by 16 to arrive at the register id
        let offset_k: u16 = self.labels[label] as u16;
        let result: u16 = 0xF401u16 | (offset_k << 3u16);
        
        log::info!("ENC BRNE: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC BRNE: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 36. CALL – Long Call to a Subroutine
    /// 1001 010k kkkk 111k
    /// kkkk kkkk kkkk kkkk
    fn encode_call(&self, segment:&mut Segment, idx:&usize, label: &String)
    {
        // register is increased by 16 to arrive at the register id
        let label_address: i32 = self.labels[label] as i32;
        let mut offset_k: i32 = label_address - (*idx as i32);

        log::info!("offset_k: {:#06x}", offset_k);
        log::info!("offset_k: {:#06x}", offset_k as u32);

        offset_k &= 0b00000000001111111111111111111111i32;

        log::info!("offset_k: {:#06x}", offset_k);
        log::info!("offset_k: {:#06x}", offset_k as u32);

        let result: u32 = (0b1001010u32 << 25) | ((offset_k as u32 >> 17) << 20) | (0b111u32 << 17) | (offset_k as u32 & 0b11111111111111111u32);

        log::info!("result: {:#32b}", result);
        
        log::info!("ENC CALL: {:#02x}", (result >> 16u16) as u8);
        segment.data.push((result >> 16u16) as u8);
        segment.size += 1u32;

        log::info!("ENC CALL: {:#02x}", (result >> 24u16) as u8);
        segment.data.push((result >> 24u16) as u8);
        segment.size += 1u32;

        log::info!("ENC CALL: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC CALL: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        //log::info!("result: {:#026b}", result);
    }

    /// 53. DEC – Decrement
    /// 1001 010d dddd 1010
    fn encode_dec(&self, segment:&mut Segment, register_d: u16)
    {
        // register is increased by 16 to arrive at the register id
        let register: u16 = register_d;
        let result: u16 = 0x940Au16 | (register << 4u16);
        
        log::info!("ENC DEC: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC DEC: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 66. JMP – Jump
    /// 1001 010k kkkk 110k
    /// kkkk kkkk kkkk kkkk
    fn encode_jmp(&self, segment: &mut Segment, idx:&usize, label: &String)
    {
        // register is increased by 16 to arrive at the register id
        let label_address: i32 = self.labels[label] as i32;
        let mut offset_k: i32 = label_address - (*idx as i32);

        log::info!("offset_k: {:#06x}", offset_k);
        log::info!("offset_k: {:#06x}", offset_k as u32);

        offset_k &= 0b00000000001111111111111111111111i32;

        log::info!("offset_k: {:#06x}", offset_k);
        log::info!("offset_k: {:#06x}", offset_k as u32);

        let result: u32 = (0b1001010u32 << 25) | ((offset_k as u32 >> 17) << 20) | (0b110u32 << 17) | (offset_k as u32 & 0b11111111111111111u32);

        log::info!("result: {:#32b}", result);
        
        log::info!("ENC JMP: {:#02x}", (result >> 16u16) as u8);
        segment.data.push((result >> 16u16) as u8);
        segment.size += 1u32;

        log::info!("ENC JMP: {:#02x}", (result >> 24u16) as u8);
        segment.data.push((result >> 24u16) as u8);
        segment.size += 1u32;

        log::info!("ENC JMP: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC JMP: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        //log::info!("result: {:#026b}", result);

    }

    /// 73. LDI – Load Immediate
    /// 1110 KKKK dddd KKKK
    /// 
    /// NOTE: LDI is only allowed for registers in the range from [r16, r31]
    /// The full 32 regsters cannot be used since there are only 4 bit of space to store the target register.
    /// The register parameter is diminished by 16 so that it fits into the 4 bit space in the instruction word.
    fn encode_ldi(&self, segment:&mut Segment, register_d: u16, imm_value_k: u16)
    {
        if register_d < 15 || register_d > 31 {
            panic!("Invalid register for LDI! Only registers [r16, r31] are allowed")
        }
        // register is increased by 16 to arrive at the register id
        let register: u16 = register_d - 16u16;

        let k_mask: u16 = 0xE000u16 | ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);
        let result: u16 = 0xEFFFu16 & (k_mask | (register << 4u16));
        
        log::info!("ENC LDI: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;
        
        log::info!("ENC LDI: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 79. MOV – Copy Register
    /// 0010 11rd dddd rrrr
    fn encode_mov(&self, segment:&mut Segment, register_d: u16, register_r: u16)
    {
        if register_d > 31 {
            panic!("Invalid register d for MOV! Only registers [r00, r31] are allowed")
        }
        if register_r > 31 {
            panic!("Invalid register r for MOV! Only registers [r00, r31] are allowed")
        }
        let result: u16 = (0b0010110000000000u16 | ((register_r >> 4u16) << 9u16) | ((register_d << 4u16) & 0x1Fu16) | (register_r << 0x04u16)) as u16;

        log::info!("ENC MOV: {:b}", result);

        log::info!("ENC MOV: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC MOV: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 88. OUT – Store Register to I/O Location
    /// 1011 1AAr rrrr AAAA
    fn encode_out(&self, segment:&mut Segment, io_dest: IoDestination, register_r: u16)
    {
        let mut a_val: u16 = 0x00;
        let r_val: u16 = register_r;

        match io_dest {
            IoDestination::SPL => {
                a_val = 0x01;
            }
            IoDestination::SPH => {
                a_val = 0x02;
            }
            _ => panic!("Unknown enum value")
        }

        let result: u16 = (0b1011100000000000u16 | ((a_val >> 4u16) << 8u16) | (a_val & 0x0Fu16) | (r_val << 0x04u16)) as u16;

        log::info!("ENC OUT: {:b}", result);

        log::info!("ENC OUT: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC OUT: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 89. POP – Pop Register from Stack
    fn encode_pop(&self, segment:&mut Segment, register_d: u16)
    {
        if register_d > 31 {
            panic!("Invalid register for PUSH! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x900Fu16 | (register_d << 4u16);
        
        log::info!("ENC POP: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;
        
        log::info!("ENC POP: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 90. PUSH – Push Register on Stack
    fn encode_push(&self, segment:&mut Segment, register_d: u16)
    {
        if register_d > 31 {
            panic!("Invalid register for PUSH! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x920Fu16 | (register_d << 4u16);
        
        log::info!("ENC PUSH: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;
        
        log::info!("ENC PUSH: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 91. RCALL – Relative Call to Subroutine
    /// 1101 kkkk kkkk kkkk
    fn encode_rcall(&self, segment:&mut Segment, idx:&usize, label: &String)
    {
        // THIS CODE HAS BEEN COPIED FROM RJMP

        // register is increased by 16 to arrive at the register id
        let label_address: i16 = self.labels[label] as i16;

        log::info!("label_address: {:#06x}", label_address);

        let mut offset_k: i16 = label_address - (*idx as i16);

        log::info!("offset_k: {:#06x} {}", offset_k, offset_k);

        offset_k &= 0b0000111111111111i16;

        log::info!("offset_k: {:#06x} {}", offset_k, offset_k);

        let result: i16 = (0b1101 << 12) | offset_k;

        log::info!("result: {:#32b}", result);

        log::info!("ENC RCALL: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC RCALL: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::info!("result: {:#026b}", result);

    }

    /// 92. RET – Return from Subroutine
    /// Returns from subroutine. The return address is loaded from the STACK. 
    /// The Stack Pointer uses a pre-increment scheme during RET
    /// 1001 0101 0000 1000
    fn encode_ret(&self, segment:&mut Segment) {

        let result: u16 = 0b1001010100001000u16;

        segment.data.push(LOW!(result) as u8);
        segment.size += 1u32;

        segment.data.push(HIGH!(result) as u8);
        segment.size += 1u32;
    }

    /// 94. RJMP – Relative Jump
    /// 1100 kkkk kkkk kkkk
    fn encode_rjmp(&self, segment: &mut Segment, idx:&usize, label: &String)
    {
        // register is increased by 16 to arrive at the register id
        let label_address: i16 = self.labels[label] as i16;

        log::info!("label_address: {:#06x}", label_address);

        let mut offset_k: i16 = label_address - (*idx as i16);

        log::info!("offset_k: {:#06x} {}", offset_k, offset_k);

        offset_k &= 0b0000111111111111i16;

        log::info!("offset_k: {:#06x} {}", offset_k, offset_k);

        let result: i16 = (0b1100 << 12) | offset_k;

        log::info!("result: {:#32b}", result);

        log::info!("ENC RJMP: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC RJMP: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::info!("result: {:#026b}", result);

    }

}

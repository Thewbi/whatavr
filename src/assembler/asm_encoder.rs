use std::collections::HashMap;

use crate::{ihex_mgmt::ihex_mgmt::Segment, instructions::instruction_type::InstructionType, common::number_literal_parser::number_literal_to_u16};

use super::asm_record::AsmRecord;

#[macro_export]
macro_rules! HIGH {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            ($a >> 8u16)
        }
    };
}

#[macro_export]
macro_rules! LOW {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            ($a & 0xFFu16)
        }
    };
}

#[macro_export]
macro_rules! HIGH_U16 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            ($a >> 8u16) as u8
        }
    };
}

#[macro_export]
macro_rules! LOW_U16 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            (($a & 0xFFu16) & 0xFFu16) as u8
        }
    };
}

#[macro_export]
macro_rules! HIGH_I16 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            ($a >> 8i16)
        }
    };
}

#[macro_export]
macro_rules! LOW_I16 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            ($a & 0xFFi16)
        }
    };
}

#[macro_export]
macro_rules! HIGH_HIGH_I32 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            (($a >> 24) & 0xFFi32)
        }
    };
}

#[macro_export]
macro_rules! HIGH_I32 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            (($a >> 16) & 0xFFi32)
        }
    };
}

#[macro_export]
macro_rules! LOW_I32 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            (($a >> 8) & 0xFFi32)
        }
    };
}

#[macro_export]
macro_rules! LOW_LOW_I32 {
    // match like arm for macro
    ($a:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            (($a >> 0) & 0xFFi32)
        }
    };
}

pub fn create_label(labels: &mut HashMap<String, u16>, label: String, idx: u16) {
    labels.insert(label.clone(), idx);
    println!("Label: {} -> idx: {:#04X}", label, idx);
}

// 1. enter all commands into a list
// 2. resolve all macros and insert new entries (created from the resolved macros) into the list
// 3. go through the list of all commands when a label is found, insert the label into a map along with the current idx
//    but do not encode any command in this phase
// 4. got through the list of commands and call encode for each command using the table of resolved labels
//    but this time ignore the creation of labels and do not insert the labels int the map any more since they are already resolved in phase 1

// 1. Add a cycle counter
pub struct AsmEncoder {

    pub labels: HashMap<String, u16>,

    pub encoding_success: bool,

}

impl AsmEncoder {

    pub fn new() -> AsmEncoder {
        AsmEncoder {
            labels: HashMap::new(),
            encoding_success: true,
        }
    }

    pub fn assemble(&mut self, asm_records: &mut Vec<AsmRecord>, segment: &mut Segment) {

        //
        // phase 1 - scan for labels
        //

        let mut idx: u16 = 0u16;
        for asm_record in asm_records.iter_mut() {

            // assign the current address to the record
            asm_record.set_idx(idx);

            // if a label was specified for the current address,
            // manage the mapping of the label to the current address
            if asm_record.label != "" {
                create_label(&mut self.labels, asm_record.label.clone(), idx);
            }

            // advance the address by the actual length of the instruction.
            // Some instructions are 1 word (2 byte) whereas others are 2 word (4 byte)
            idx += InstructionType::words(&asm_record.instruction_type);
        }

        //
        // phase 2 - encode (with addresses resolved to labels)
        //

        for rec in asm_records.iter() {
            self.encode(segment, rec);
        }

    }

    pub fn encode(&mut self, segment: &mut Segment, asm_record: &AsmRecord) {

        log::info!("Encoding: {}\n" , asm_record);

        match asm_record.instruction_type {

            /*   5 */
            InstructionType::ADC => {
                Self::encode_adc(&self, segment, asm_record.reg_1, asm_record.reg_2);
            }
            /*   6 */
            InstructionType::ADD => {
                Self::encode_add(&self, segment, asm_record.reg_1, asm_record.reg_2);
            }
            /*   8 */
            InstructionType::AND => {
                Self::encode_and(&self, segment, asm_record.reg_1, asm_record.reg_2);
            }
            /*   9 */
            InstructionType::ANDI => {
                Self::encode_andi(&self, segment, asm_record.reg_1, asm_record.reg_2);
            }
            /*  17 */
            InstructionType::BREAK => {
                Self::encode_break(&self, segment);
            }
            /*  18 */
            InstructionType::BREQ => {
                Self::encode_breq(&self, segment, &asm_record.idx, &asm_record.target_label, asm_record.target_address);
            }
            /*  27 */
            InstructionType::BRNE => {
                Self::encode_brne(&self, segment, &asm_record.idx, &asm_record.target_label);
            }
            /*  36 */
            InstructionType::CALL => {
                Self::encode_call(&self, segment, &asm_record.idx, &asm_record.target_label, asm_record.target_address);
            }
            /*  37 */
            InstructionType::CBI => {
                Self::encode_cbi(&self, segment, asm_record.reg_1, asm_record.reg_2);
            }
            /*  41 */
            InstructionType::CLI => {
                Self::encode_cli(&self, segment);
            }
            /*  43 */
            InstructionType::CLR => {
                Self::encode_clr(&self, segment, asm_record.reg_1);
            }
            /*  51 */
            InstructionType::CPI => {
                Self::encode_cpi(&self, segment, asm_record.reg_1, asm_record.data);
            }
            /*  53 */
            InstructionType::DEC => {
                Self::encode_dec(&self, segment, asm_record.reg_1);
            }
            /*  58 */
            InstructionType::EOR => {
                Self::encode_eor(&self, segment, asm_record.reg_1, asm_record.reg_2);
            }
            /*  64 */
            InstructionType::IN => {
                Self::encode_in(&self, segment, asm_record.reg_1, asm_record.data);
            }
            /*  65 */
            InstructionType::INC => {
                Self::encode_inc(&self, segment, asm_record.reg_1);
            }
            /*  66 */
            InstructionType::JMP => {
                Self::encode_jmp(self, segment, &asm_record.idx, &asm_record.target_label, asm_record.target_address);
            }


            /*  70 */
            InstructionType::LD_LDD_X_1 => {
                Self::encode_ld_ldd_x_1(&self, segment, asm_record.reg_1);
            }
            InstructionType::LD_LDD_X_2 => {
                Self::encode_ld_ldd_x_2(&self, segment, asm_record.reg_1);
            }

            /*  71 */
            InstructionType::LD_LDD_Y_2 => {
                Self::encode_ld_ldd_y_2(&self, segment, asm_record.reg_1);
            }

            /*  72 */
            InstructionType::LD_LDD_Z_1 => {
                Self::encode_ld_ldd_z_1(&self, segment, asm_record.reg_1);
            }


            /*  73 */
            InstructionType::LDI => {
                Self::encode_ldi(&self, segment, asm_record.reg_1, asm_record.data);
            }
            /*  74 */
            InstructionType::LDS => {
                Self::encode_lds(&self, segment, asm_record.reg_1, asm_record.data);
            }
            /*  75 */
            InstructionType::LDS_16bit => {
                Self::encode_lds_16bit(&self, segment, asm_record.reg_1, asm_record.data);
            }

            /*  76 */
            InstructionType::LPM_1 => {
                Self::encode_lpm_1(&self, segment);
            }
            InstructionType::LPM_2 => {
                Self::encode_lpm_2(&self, segment, asm_record.reg_1);
            }
            InstructionType::LPM_3 => {
                Self::encode_lpm_3(&self, segment, asm_record.reg_1);
            }

            /*  79 */
            InstructionType::MOV => {
                Self::encode_mov(&self, segment, asm_record.reg_1, asm_record.reg_2);
            }
            /*  85 */
            InstructionType::NOP => {
                Self::encode_nop(&self, segment);
            }
            /*  87 */
            InstructionType::ORI => {
                Self::encode_ori(&self, segment, asm_record.reg_1, asm_record.data);
            }
            /*  88 */
            InstructionType::OUT => {
                // let mut param_1: u16 = asm_record.reg_1;
                // let mut param_2: u16 = asm_record.reg_2;
                // if param_2 == 0xFF {
                //     param_1 = number_literal_to_u16(&asm_record.target_label);
                //     param_2 = asm_record.reg_1;
                // }
                //Self::encode_out(self, segment, param_1, param_2);

                Self::encode_out(self, segment, asm_record.reg_1, asm_record.reg_2);
            }
            /*  89 */
            InstructionType::POP => {
                Self::encode_pop(&self, segment, asm_record.reg_1);
            }
            /*  90 */
            InstructionType::PUSH => {
                Self::encode_push(&self, segment, asm_record.reg_1);
            }
            /*  91 */
            InstructionType::RCALL => {
                Self::encode_rcall(&self, segment, &asm_record.idx, &asm_record.target_label);
            }
            /*  92 */
            InstructionType::RET => {
                Self::encode_ret(&self, segment);
            }
            /*  93 */
            InstructionType::RETI => {
                Self::encode_reti(&self, segment);
            }
            /*  94 */
            InstructionType::RJMP => {
                Self::encode_rjmp(&self, segment, &asm_record.idx, &asm_record.target_label, asm_record.target_address);
            }
            /*  99 */
            InstructionType::SBI => {
                let param2_value: u16;
                if asm_record.reg_2 == 255 { param2_value = asm_record.data; } else { param2_value = asm_record.reg_2; }
                Self::encode_sbi(&self, segment, &asm_record.idx, asm_record.reg_1, param2_value);
                //Self::encode_sbi(&self, segment, &asm_record.idx, asm_record.reg_1, asm_record.data);
                //Self::encode_sbi(&self, segment, &asm_record.idx, asm_record.reg_1, asm_record.reg_2);
            }
            /*  105 */
            InstructionType::SBRS => {
                let param2_value: u16;
                if asm_record.reg_2 == 255 { param2_value = asm_record.data; } else { param2_value = asm_record.reg_2; }
                Self::encode_sbrs(&self, segment, &asm_record.idx, asm_record.reg_1, param2_value);
            }
            /*  108 */
            InstructionType::SEI => {
                Self::encode_sei(&self, segment, &asm_record.idx);
            }

            /* 118 */
            InstructionType::ST_STD_X_1 => {
                Self::encode_st_std_x_1(&self, segment, asm_record.reg_1);
            }
            InstructionType::ST_STD_X_2 => {
                Self::encode_st_std_x_2(&self, segment, asm_record.reg_1);
            }
            InstructionType::ST_STD_X_3 => {
                Self::encode_st_std_x_3(&self, segment, asm_record.reg_1);
            }

            /* 119 */
            InstructionType::ST_STD_Y_1 => {
                Self::encode_st_std_y_1(&self, segment, asm_record.reg_1);
            }
            InstructionType::ST_STD_Y_2 => {
                Self::encode_st_std_y_2(&self, segment, asm_record.reg_1);
            }
            InstructionType::ST_STD_Y_3 => {
                Self::encode_st_std_y_3(&self, segment, asm_record.reg_1);
            }

            /* 120 */
            InstructionType::ST_STD_Z_1 => {
                Self::encode_st_std_z_1(&self, segment, asm_record.reg_1);
            }
            InstructionType::ST_STD_Z_2 => {
                Self::encode_st_std_z_2(&self, segment, asm_record.reg_1);
            }
            InstructionType::ST_STD_Z_3 => {
                Self::encode_st_std_z_3(&self, segment, asm_record.reg_1);
            }

            /* 121 */
            InstructionType::STS => {
                Self::encode_sts(&self, segment, asm_record.reg_2, asm_record.reg_1);
            }
            /* 122 */
            InstructionType::STS_16bit => {
                Self::encode_sts_16bit(&self, segment, asm_record.reg_2, asm_record.data);
            }

            /* 124 */
            InstructionType::SUBI => {
                Self::encode_subi(&self, segment, asm_record.reg_1, asm_record.data);
            }

            _ => panic!("Unknown instruction! {:?}", asm_record.instruction_type),
        }
    }

    /// 5. ADC – Add with Carry
    /// adc - Add without carry (Rd ← Rd + Rr)
    /// Adds two registers without the C Flag and places the result in the destination register Rd.
    /// 0000 11rd dddd rrrr
    fn encode_adc(&self, segment: &mut Segment, register_d: u16, register_r: u16) {

        let register_d_offset: u16 = register_d;
        let register_r_offset: u16 = register_r;

        let r_mask: u16 = ((register_r_offset >> 4u16) << 9u16) | (register_r_offset & 0x0Fu16);

        let result: u16 = 0x1C00u16 | (r_mask | register_d_offset << 4u16);

        log::trace!("add d{} r{}", register_d_offset, register_r_offset);

        log::trace!("ENC ADC: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC ADC: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 6. ADD – Add without Carry
    /// add - Add without carry (Rd ← Rd + Rr)
    /// 0000 11rd dddd rrrr
    fn encode_add(&self, segment: &mut Segment, register_d: u16, register_r: u16) {

        let register_d_offset: u16 = register_d;
        let register_r_offset: u16 = register_r;

        let r_mask: u16 = ((register_r_offset >> 4u16) << 9u16) | (register_r_offset & 0x0Fu16);

        let result: u16 = 0x0C00u16 | (r_mask | register_d_offset << 4u16);

        log::trace!("add d{} r{}", register_d_offset, register_r_offset);

        log::trace!("ENC ADD: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC ADD: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 8. AND – Logical AND
    /// (Rd ← Rd • Rr)
    /// 0010 00rd dddd rrrr
    fn encode_and(&self, segment: &mut Segment, register_d: u16, register_r: u16) {

        // let register_d_offset: u16 = register_d;
        // let register_r_offset: u16 = register_r;

        let r_mask: u16 = ((register_r >> 4u16) << 9u16) | (register_r & 0x0Fu16);

        let result: u16 = 0x2000u16 | (r_mask | register_d << 4u16);

        log::trace!("and d{} r{}", register_d, register_r);

        log::trace!("ENC AND: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC AND: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 9. ANDI – Logical AND with Immediate
    /// Performs the logical AND between the contents of register Rd and a constant, and places the result in the
    /// destination register Rd. (Rd ← Rd • K)
    /// 0111 KKKK dddd KKKK
    fn encode_andi(&self, segment: &mut Segment, register_d: u16, imm_value_k: u16) {

        //let register_d_offset: u16 = register_d;
        //let register_r_offset: u16 = register_r;

        //let r_mask: u16 = ((register_r_offset >> 4u16) << 9u16) | (register_r_offset & 0x0Fu16);

        log::info!("andi d{:#02x} imm{:#02x}", register_d, imm_value_k);

        let result: u16 = 0x7000u16 | ((imm_value_k & 0b11110000) << 8u16) | (register_d << 4u16) | ((imm_value_k & 0b00001111) << 0u16);

        log::trace!("ENC ANDI: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC ANDI: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 17. BREAK
    /// The BREAK instruction is used by the On-chip Debug system, and is normally not used in the application
    /// software. When the BREAK instruction is executed, the AVR CPU is set in the Stopped Mode. This gives
    /// the On-chip Debugger access to internal resources.
    /// If any Lock bits are set, or either the JTAGEN or OCDEN Fuses are unprogrammed, the CPU will treat
    /// the BREAK instruction as a NOP and will not enter the Stopped mode.
    /// This instruction is not available in all devices. Refer to the device specific instruction set summary
    /// 1001 0101 1001 1000
    fn encode_break(&self, segment: &mut Segment) {

        let result: u16 = 0x9598;

        log::trace!("ENC BREAK: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC BREAK: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 18. BREQ – Branch if Equal
    /// 1111 00kk kkkk k001
    /// 1111 00kk kkkk ksss // BRBS (more general instruction that entails BREQ)
    fn encode_breq(&self, segment: &mut Segment, idx: &u16, label: &String, address: i16) {

        // asdf
        // let offset_k: i32;
        // if !label.is_empty()
        // {
        //     let target_address: i32 = self.labels[label] as i32;

        //     // convert from bytes to words
        //     offset_k = target_address / 2i32;
        // }
        // else
        // {
        //     // the relative notation is given in bytes not words
        //     // so do not divide by 2 to convert from byte to wor
        //     offset_k = address as i32;
        // }

        // // only seven bits of offset
        // let k: u32 = offset_k as u32 & 0x7f;

        let target_address: i16;
        if !label.is_empty()
        {
            target_address = self.labels[label] as i16;
        }
        else
        {
            target_address = address as i16;
        }

        // convert from bytes to words
        let offset_k: i16 = target_address / 2i16;
        
        log::trace!("offset_k (in words): {:#06x}", offset_k);
        log::trace!("offset_k (in words): {:#06x}", offset_k as u32);

        //offset_k &= 0b0000 0000 0011 1111 1111 1111 1111 1111i32;

        // ff cf == 1111 1111 1100 1111
        // cf ff  == 1100 1111 1111 1111 

        // 1100 kkkk kkkk kkkk

        let result: u16 = 0xF001u16 | (((offset_k << 3) as u16) & 0b1111111000);
        log::info!("result: {:#34b}", result);
        log::info!("result: {:#06x}", result);

        //let result: u16 = 0xF001u16 | ((k as u16) << 3u16);

        log::info!("ENC BREQ: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC BREQ: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 27. BRNE – Branch if Not Equal
    /// 1111 01kk kkkk k001
    fn encode_brne(&self, segment: &mut Segment, idx: &u16, label: &String) {

        //let mut offset_k: u16 = self.labels[label] as u16;

        // why do I need this? is this correct?
        //offset_k = offset_k * 2;

        let label_address: i32 = self.labels[label] as i32;
        let offset_k: i32 = label_address - (*idx as i32);

        // do I need to use some kind of little endian encoding?
        let result: u16 = 0xF401u16 | (((offset_k & 0x3Fi32) as u16) << 3u16);

        log::trace!("ENC BRNE: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC BRNE: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("{:#18b}\n", result);
    }

    /// 36. CALL – Long Call to a Subroutine
    /// 1001 010k kkkk 111k
    /// kkkk kkkk kkkk kkkk
    fn encode_call(&self, segment: &mut Segment, idx: &u16, label: &String, address: i16) {

        if 0x00 == address && label.is_empty() {
            panic!("No label or address found for call instruction!");
        }

        let target_address: i32;
        if !label.is_empty()
        {
            target_address = self.labels[label] as i32;
        }
        else
        {
            target_address = address as i32;
        }
        
        // convert from bytes to words
        let mut offset_k: i32 = target_address / 2i32;

        log::trace!("offset_k (in words): {:#06x}\n", offset_k);
        log::trace!("offset_k (in words): {:#06x}\n", offset_k as u32);

        offset_k &= 0b00000000001111111111111111111111i32;

        log::trace!("offset_k (in words): {:#06x}\n", offset_k);
        log::trace!("offset_k (in words): {:#06x}\n", offset_k as u32);

        let result: u32 = (0b1001010u32 << 25)
            | ((offset_k as u32 >> 17) << 20)
            | (0b111u32 << 17)
            | (offset_k as u32 & 0b11111111111111111u32);

        log::trace!("result (in words): {:#32b}\n", result);

        log::trace!("ENC CALL: {:#02x}", (result >> 16u16) as u8);
        segment.data.push((result >> 16u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC CALL: {:#02x}", (result >> 24u16) as u8);
        segment.data.push((result >> 24u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC CALL: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC CALL: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        //log::info!("result: {:#026b}", result);
    }

    /// 37. CBI – Clear Bit in I/O Register
    /// Clears a specified bit in an I/O register. This instruction operates on the lower 32 I/O registers –
    /// addresses 0-31.
    /// 1001 1000 AAAA Abbb
    fn encode_cbi(&self, segment: &mut Segment, register_a: u16, bit_to_clear: u16) {

        if register_a > 31 {
            panic!("Invalid register for CBI! Only registers [0, 31] are allowed!")
        }
        if bit_to_clear > 8 {
            panic!("Invalid bit for CBI! Only bits [0, 7] are allowed!")
        }

        let result: u16 = 0x9800u16 | register_a << 3usize | bit_to_clear;

        log::trace!("ENC CBI: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC CBI: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 41. CLI – Clear Global Interrupt Flag
    /// Clears the Global Interrupt Flag (I) in SREG (Status Register).
    fn encode_cli(&self, segment: &mut Segment) {

        let result: u16 = 0x94F8u16;

        log::trace!("ENC CLI: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC CLI: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 43. CLR – Clear
    /// Clears a register. This instruction performs an Exclusive OR between a register and itself. This will clear
    /// all bits in the register
    /// 0010 01dd dddd dddd
    fn encode_clr(&self, segment: &mut Segment, register_d: u16)
    {
        let result: u16 = 0x2400u16 | register_d;

        log::trace!("ENC CLR: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC CLR: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    fn encode_cpi(&self, segment: &mut Segment, register_d: u16, constant: u16) 
    {
        let result: u16 = 0x3000 | ((constant >> 4) << 8) | register_d << 4 | constant & 0x0F;

        log::trace!("ENC CPI: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC CPI: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 53. DEC – Decrement
    /// 1001 010d dddd 1010
    fn encode_dec(&self, segment: &mut Segment, register_d: u16) 
    {

        let register: u16 = register_d;
        let result: u16 = 0x940Au16 | (register << 4u16);

        log::trace!("ENC DEC: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC DEC: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 58. EOR – Exclusive OR
    /// 0010 01rd dddd rrrr -- EOR - Exclusive OR
    /// 0010 01dd dddd dddd -- CLR – Clear Register
    fn encode_eor(&self, segment: &mut Segment, register_d: u16, register_r: u16) {

        if register_d > 31 {
            panic!("Invalid register for EOR! Only registers [r0, r31] are allowed")
        }
        if register_r > 31 {
            panic!("Invalid address for EOR! Only address [r0, r31] are allowed")
        }

        let result: u16 = 0x2400u16 | ((register_r >> 4u16) << 9u16)| (register_d << 4u16);

        log::trace!("ENC EOR: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC EOR: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 64. IN - Load an I/O Location to Register
    /// 1011 0AAd dddd AAAA
    fn encode_in(&self, segment: &mut Segment, register_d: u16, address: u16) {

        if register_d > 31 {
            panic!("Invalid register for IN! Only registers [r0, r31] are allowed")
        }
        if address > 64 {
            panic!("Invalid address for IN! Only address [0, 0x3F] are allowed")
        }

        let result: u16 =
            0xB000u16 | ((address >> 4u16) << 9u16) | ((register_d) << 4u16) | (address & 0x0Fu16);

        log::trace!("ENC IN: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC IN: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 65. INC - Adds one -1- to the contents of register Rd and places the result in the destination register Rd.
    /// 1001 010d dddd 0011
    fn encode_inc(&self, segment: &mut Segment, register_d: u16) {

        if register_d > 31 {
            panic!("Invalid register for INC! Only registers [r0, r31] are allowed")
        }

        let result: u16 =
            0x9403u16 | ((register_d) << 4u16);

        log::trace!("ENC INC: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC INC: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 66. JMP – Jump
    /// 1001 010k kkkk 110k
    /// kkkk kkkk kkkk kkkk
    //fn encode_jmp(&mut self, segment: &mut Segment, idx: &usize, label_or_immediate: &String) {
    fn encode_jmp(&mut self, segment: &mut Segment, idx: &u16, label: &String, address: i16) {

        // if 0x00 == address && label.is_empty() {
        //     panic!("No label or address found for call instruction!");
        // }

        // if label_or_immediate.is_empty() {
        //     log::error!("Encoding JMP instruction but the label/immediate is missing!");
        //     self.encoding_success = false;
        //     return
        // }

        // let mut offset_k: i32;

        // if self.labels.contains_key(label_or_immediate) {

        //     let label_address: i32 = self.labels[label_or_immediate] as i32;
        //     offset_k = label_address - (*idx as i32);

        // } else {

        //     offset_k = number_literal_to_u16(label_or_immediate) as i32;

        // }
        // // else {

        // //     log::error!("Encoding JMP instruction but there is no immediate and label \"{}\" is not defined!", label_or_immediate);
        // //     self.encoding_success = false;
        // //     return

        // // }
        // // convert from bytes to words
        // offset_k /= 2i32;

        let target_address: i32;
        if !label.is_empty()
        {
            target_address = self.labels[label] as i32;
        }
        else
        {
            target_address = address as i32;
        }

        // convert from bytes to words
        let mut offset_k: i32 = target_address / 2i32;

        log::trace!("offset_k (in words): {:#06x}\n", offset_k);
        log::trace!("offset_k (in words): {:#06x}\n", offset_k as u32);

        offset_k &= 0b00000000001111111111111111111111i32;

        log::trace!("offset_k (in words): {:#06x}\n", offset_k);
        log::trace!("offset_k (in words): {:#06x}\n", offset_k as u32);

        let result: u32 = (0b1001010u32 << 25)
            | ((offset_k as u32 >> 17) << 20)
            | (0b110u32 << 17)
            | (offset_k as u32 & 0b11111111111111111u32);

        log::trace!("result: {:#32b}\n", result);

        log::trace!("ENC JMP: {:#02x}\n", (result >> 16u16) as u8);
        segment.data.push((result >> 16u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC JMP: {:#02x}\n", (result >> 24u16) as u8);
        segment.data.push((result >> 24u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC JMP: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC JMP: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        //log::info!("result: {:#026b}", result);
    }

    fn encode_ld_ldd_x_1(&self, segment: &mut Segment, register_d: u16)
    {
        if register_d > 31 {
            panic!("Invalid register for encode_ld_ldd_x_1! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x900Cu16 | (register_d << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC ld_ldd_x_1: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC ld_ldd_x_1: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }
    fn encode_ld_ldd_x_2(&self, segment: &mut Segment, register_d: u16)
    {
        if register_d > 31 {
            panic!("Invalid register for encode_ld_ldd_x_2! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x900Du16 | (register_d << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC ld_ldd_x_2: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC ld_ldd_x_2: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }

    fn encode_ld_ldd_y_2(&self, segment: &mut Segment, register_d: u16)
    {
        if register_d > 31 {
            panic!("Invalid register for encode_ld_ldd_y_2! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x9009u16 | (register_d << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC ld_ldd_y_2: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC ld_ldd_y_2: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }

    /// 72. LD (LDD) – Load Indirect From Data Space to Register using Index Z
    fn encode_ld_ldd_z_1(&self, segment: &mut Segment, register_d: u16)
    {
        if register_d > 31 {
            panic!("Invalid register for encode_ld_ldd_z_1! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x8000u16 | (register_d << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC ld_ldd_z_1: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC ld_ldd_z_1: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }

    /// 73. LDI – Load Immediate
    /// 1110 KKKK dddd KKKK
    ///
    /// NOTE: LDI is only allowed for registers in the range from [r16, r31]
    /// The full 32 regsters cannot be used since there are only 4 bit of space to store the target register.
    /// The register parameter is diminished by 16 so that it fits into the 4 bit space in the instruction word.
    fn encode_ldi(&self, segment: &mut Segment, register_d: u16, imm_value_k: u16) {

        if register_d < 15 || register_d > 31 {
            panic!("Invalid register for LDI! Only registers [r16, r31] are allowed")
        }
        // register is decreased by 16 to arrive at the register id
        let register: u16 = register_d - 16u16;

        let k_mask: u16 = 0xE000u16 | ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);
        let result: u16 = 0xEFFFu16 & (k_mask | (register << 4u16));

        log::trace!("ENC LDI: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC LDI: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 74. LDS – Load Direct from Data Space
    /// This is the 32 bit variant of the LDS command. There is also a 16 variant (75. LDS 16bit)
    /// 1001 000d dddd 0000
    /// kkkk kkkk kkkk kkkk
    fn encode_lds(&self, segment: &mut Segment, register_d: u16, imm_value_k: u16) {

        if register_d > 31 {
            panic!("Invalid register for LDS (32bit)! Only registers [r0, r31] are allowed")
        }

        let result: u32 = (0b1001000u32 << 25)
            | ((register_d as u32) << 20)
            | (imm_value_k as u32);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC LDS (32bit): {:#02x}", (result >> 16u16) as u8);
        segment.data.push((result >> 16u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC LDS (32bit): {:#02x}", (result >> 24u16) as u8);
        segment.data.push((result >> 24u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC LDS (32bit): {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC LDS (32bit): {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        // let k_mask: u16 = 0xE000u16 | ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);
        // let result: u16 = 0xEFFFu16 & (k_mask | (register << 4u16));

        // log::trace!("ENC LDS: {:#02x}", (result >> 0u16) as u8);
        // segment.data.push((result >> 0u16) as u8);
        // segment.size += 1u32;

        // log::trace!("ENC LDS: {:#02x}", (result >> 8u16) as u8);
        // segment.data.push((result >> 8u16) as u8);
        // segment.size += 1u32;
    }

    /// 75. LDS (16-bit) – Load Direct from Data Space
    /// 1010 0kkk dddd kkkk
    ///
    /// NOTE: LDS is only allowed for registers in the range from [r16, r31]
    /// The full 32 regsters cannot be used since there are only 4 bit of space to store the target register.
    /// The register parameter is diminished by 16 so that it fits into the 4 bit space in the instruction word.
    fn encode_lds_16bit(&self, segment: &mut Segment, register_d: u16, imm_value_k: u16) {

        if register_d < 15 || register_d > 31 {
            panic!("Invalid register for LDI 16Bit! Only registers [r16, r31] are allowed")
        }

        // register is decreased by 16 to arrive at the register id
        let register: u16 = register_d - 16u16;

        let k_mask: u16 = ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);

        let result: u16 = 0xA000u16 | k_mask | (register << 4u16);

        log::trace!("ENC LDS (16 bit): {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC LDS (16 bit): {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    fn encode_lpm_1(&self, segment: &mut Segment)
    {
        let result: u16 = 0x95C8u16;
        
        log::trace!("ENC LPM_1: {:b}\n", result);

        log::trace!("ENC LPM_1: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC LPM_1: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    fn encode_lpm_2(&self, segment: &mut Segment, register_d: u16)
    {
        let result: u16 = 0x9004u16 | (register_d << 4u16) & 0x1Fu16;

        log::trace!("ENC LPM_2: {:b}\n", result);

        log::trace!("ENC LPM_2: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC LPM_2: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    fn encode_lpm_3(&self, segment: &mut Segment, register_d: u16)
    {
        let result: u16 = 0x9005u16 | (register_d << 4u16) & 0x1Fu16;

        log::trace!("ENC LPM_3: {:b}\n", result);

        log::trace!("ENC LPM_3: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC LPM_3: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 79. MOV – Copy Register
    /// 0010 11rd dddd rrrr
    fn encode_mov(&self, segment: &mut Segment, register_d: u16, register_r: u16) {
        if register_d > 31 {
            panic!("Invalid register d for MOV! Only registers [r00, r31] are allowed")
        }
        if register_r > 31 {
            panic!("Invalid register r for MOV! Only registers [r00, r31] are allowed")
        }
        let result: u16 = (0b0010110000000000u16
            | ((register_r >> 4u16) << 9u16)
            | ((register_d << 4u16) & 0x1Fu16)
            | (register_r << 0x04u16)) as u16;

        log::info!("ENC MOV: {:b}", result);

        log::info!("ENC MOV: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::info!("ENC MOV: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 85. NOP - No Operation
    /// This instruction performs a single cycle No Operation.
    fn encode_nop(&self, segment: &mut Segment) {
        let result: u16 = 0x00;

        log::trace!("ENC NOP: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC NOP: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 87. ORI – Logical OR with Immediate
    /// Performs the logical OR between the contents of register Rd and a constant, and places the result in the
    /// destination register Rd.
    /// 0110 KKKK dddd KKKK
    fn encode_ori(&self, segment: &mut Segment, register_d: u16, imm_value_k: u16) {

        if register_d < 15 || register_d > 31 {
            panic!("Invalid register for ORI! Only registers [r16, r31] are allowed")
        }

        // register is decreased by 16 to arrive at the register id
        let register: u16 = register_d - 16u16;

        let k_mask: u16 = ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);

        let result: u16 = 0x6000u16 | k_mask | (register << 4u16);

        log::trace!("ENC ORI: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC ORI: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 88. OUT – Store Register to I/O Location
    /// 1011 1AAr rrrr AAAA
    //fn encode_out(&mut self, segment: &mut Segment, io_dest: IoDestination, register_r: u16) {
    fn encode_out(&mut self, segment: &mut Segment, register_a: u16, register_r: u16) {

        if register_r > 31 {
            log::error!("Invalid register for OUT! Only registers [r0, r31] are allowed");
            self.encoding_success = false;
            return;
        }

        let a_val: u16 = register_a;
        let r_val: u16 = register_r;

        let result: u16 = (0b1011100000000000u16
            | ((a_val >> 4u16) << 9u16)
            | (a_val & 0x0Fu16)
            | (r_val << 0x04u16)) as u16;

        log::trace!("ENC OUT: {:b}", result);

        log::trace!("ENC OUT: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC OUT: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 89. POP – Pop Register from Stack
    fn encode_pop(&self, segment: &mut Segment, register_d: u16) {
        if register_d > 31 {
            panic!("Invalid register for PUSH! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x900Fu16 | (register_d << 4u16);

        log::trace!("ENC POP: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC POP: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 90. PUSH – Push Register on Stack
    fn encode_push(&self, segment: &mut Segment, register_d: u16) {

        // if register_d > 31 {
        //     panic!("Invalid register for PUSH! Only registers [r0, r31] are allowed")
        // }

        let result: u16 = 0x920Fu16 | (register_d << 4u16);

        log::trace!("ENC PUSH: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC PUSH: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 91. RCALL – Relative Call to Subroutine
    /// 1101 kkkk kkkk kkkk
    fn encode_rcall(&self, segment: &mut Segment, idx: &u16, label: &String) {
        // THIS CODE HAS BEEN COPIED FROM RJMP

        let label_address: i16 = self.labels[label] as i16;

        log::info!("label_address: {:#06x}", label_address);

        let mut offset_k: i16 = label_address - (*idx as i16);

        log::info!("offset_k: {:#06x} {}", offset_k, offset_k);

        offset_k &= 0b0000111111111111i16;

        // convert from bytes to words
        offset_k /= 2i16;

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
    fn encode_ret(&self, segment: &mut Segment) {
        let result: u16 = 0b1001010100001000u16;

        segment.data.push(LOW!(result) as u8);
        segment.size += 1u32;

        segment.data.push(HIGH!(result) as u8);
        segment.size += 1u32;
    }

    /// 93. RETI – Return from Interrupt
    /// Returns from interrupt. The return address is loaded from the STACK and the Global Interrupt Flag is set.
    /// 1001 0101 0001 1000
    fn encode_reti(&self, segment: &mut Segment) {

        let result: u16 = 0b1001010100011000u16;

        segment.data.push(LOW!(result) as u8);
        segment.size += 1u32;

        segment.data.push(HIGH!(result) as u8);
        segment.size += 1u32;
    }

    /// 94. RJMP – Relative Jump
    /// 1100 kkkk kkkk kkkk
    #[allow(dead_code)]
    fn encode_rjmp(&self, segment: &mut Segment, idx: &u16, label: &String, address: i16) {

        // let label_address: i16 = self.labels[label] as i16;
        // log::trace!("label_address: {:#06x}", label_address);

        // let mut offset_k: i16 = label_address - (*idx as i16);
        // log::trace!("offset_k: {:#06x} {}", offset_k, offset_k);

        // offset_k &= 0b0000111111111111i16;
        // log::trace!("offset_k: {:#06x} {}", offset_k, offset_k);

        // // convert from bytes to words
        // offset_k /= 2i16;

        let target_address: i16;
        if !label.is_empty()
        {
            target_address = self.labels[label] as i16;
        }
        else
        {
            target_address = address as i16;
        }

        // convert from bytes to words
        //let offset_k: i16 = target_address / 2i16;

        // absolute offset
        //let offset_k: i16 = target_address;

        // relative offset
        let idx_isize: i16 = *idx as i16;
        let offset_k: i16 = target_address - idx_isize;
        
        log::trace!("offset_k (in words): {:#06x}", offset_k);
        log::trace!("offset_k (in words): {:#06x}", offset_k as u32);

        //offset_k &= 0b0000 0000 0011 1111 1111 1111 1111 1111i32;

        // ff cf == 1111 1111 1100 1111
        // cf ff  == 1100 1111 1111 1111 

        // 1100 kkkk kkkk kkkk

        let result: u16 = 0xC000u16 | ((offset_k as u16) & 0b111111111111);
        //let result: u16 = 0xC000u16;
        log::trace!("result: {:#34b}\n", result);
        //log::info!("result: {:#026b}", result);

        log::trace!("ENC RJMP: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC RJMP: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
        
    }

    /// 99. SBI – Set Bit in I/O Register
    /// 1001 1010 AAAA Abbb
    #[allow(dead_code)]
    fn encode_sbi(&self, segment: &mut Segment, _idx: &u16, register_a: u16, bit_to_set: u16) {

        if register_a > 31 {
            panic!("Invalid register for SBI! Only registers [0, 31] are allowed!")
        }
        if bit_to_set > 8 {
            panic!("Invalid bit for SBI! Only bits [0, 7] are allowed!")
        }

        let result: u16 = 0x9A00u16 | register_a << 3usize | bit_to_set;

        log::trace!("ENC SBI: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC SBI: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 105. SBRS – Skip if Bit in Register is Set
    /// 1111 111r rrrr 0bbb
    #[allow(dead_code)]
    fn encode_sbrs(&self, segment: &mut Segment, _idx: &u16, register_r: u16, bit_to_set: u16) {

        if register_r > 31 {
            panic!("Invalid register for SBRS! Only registers [0, 31] are allowed!")
        }
        if bit_to_set > 8 {
            panic!("Invalid bit for SBRS! Only bits [0, 7] are allowed!")
        }

        let result: u16 = 0xFE00u16 | register_r << 4usize | bit_to_set;

        log::trace!("ENC SBRS: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC SBRS: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 108. SEI – Set Global Interrupt Flag
    /// 1001 0100 0111 1000
    #[allow(dead_code)]
    fn encode_sei(&self, segment: &mut Segment, _idx: &u16) {

        let result: u16 = 0x9478u16;

        log::trace!("ENC SEI: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC SEI: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 118. ST (STD) – Store Indirect From Register to Data Space using Index X
    /// Stores one byte indirect with or without displacement from a register to data space.
    /// ST X, Rr
    /// 1001 001r rrrr 1100
    fn encode_st_std_x_1(&self, segment: &mut Segment, register_r: u16) {

        if register_r > 31 {
            panic!("Invalid register for encode_st_std_x_1! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x920Cu16 | (register_r << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC st_std_x_1: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC st_std_x_1: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }
    /// ST X+, Rr
    /// 1001 001r rrrr 1101
    fn encode_st_std_x_2(&self, segment: &mut Segment, register_r: u16) {

        if register_r > 31 {
            panic!("Invalid register for encode_st_std_x_2! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x920du16 | (register_r << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC st_std_x_2: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC st_std_x_2: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }
    /// ST -X, Rr
    /// 1001 001r rrrr 1110
    fn encode_st_std_x_3(&self, segment: &mut Segment, register_r: u16) {

        if register_r > 31 {
            panic!("Invalid register for encode_st_std_x_3! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x920eu16 | (register_r << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC st_std_x_3: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC st_std_x_3: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }

    /// 119. ST (STD) – Store Indirect From Register to Data Space using Index Y
    /// Stores one byte indirect with or without displacement from a register to data space.
    /// ST Y, Rr
    /// 1000 001r rrrr 1000
    fn encode_st_std_y_1(&self, segment: &mut Segment, register_r: u16) {

        if register_r > 31 {
            panic!("Invalid register for encode_st_std_y_1! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x8208u16 | (register_r << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC st_std_y_1: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC st_std_y_1: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }
    /// ST X+, Rr
    /// 1001 001r rrrr 1101
    fn encode_st_std_y_2(&self, segment: &mut Segment, register_r: u16) {

        if register_r > 31 {
            panic!("Invalid register for encode_st_std_y_2! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x9209u16 | (register_r << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC st_std_y_2: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC st_std_y_2: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }
    /// ST -X, Rr
    /// 1001 001r rrrr 1110
    fn encode_st_std_y_3(&self, segment: &mut Segment, register_r: u16) {

        if register_r > 31 {
            panic!("Invalid register for encode_st_std_y_3! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x920Au16 | (register_r << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC st_std_y_3: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC st_std_y_3: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }

    /// 120. ST (STD) – Store Indirect From Register to Data Space using Index Z
    /// Stores one byte indirect with or without displacement from a register to data space.
    /// ST Z, Rr
    /// 1000 001r rrrr 0000
    fn encode_st_std_z_1(&self, segment: &mut Segment, register_r: u16) {

        if register_r > 31 {
            panic!("Invalid register for encode_st_std_z_1! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x8200u16 | (register_r << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC st_std_z_1: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC st_std_z_1: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }
    /// ST Z+, Rr
    /// 1001 001r rrrr 0001
    fn encode_st_std_z_2(&self, segment: &mut Segment, register_r: u16) {

        if register_r > 31 {
            panic!("Invalid register for encode_st_std_z_2! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x9201u16 | (register_r << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC st_std_z_2: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC st_std_z_2: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }
    /// ST -Z, Rr
    /// 1001 001r rrrr 0010
    fn encode_st_std_z_3(&self, segment: &mut Segment, register_r: u16) {

        if register_r > 31 {
            panic!("Invalid register for encode_st_std_z_3! Only registers [r0, r31] are allowed")
        }

        let result: u16 = 0x9202u16 | (register_r << 4u16);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC st_std_z_3: {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC st_std_z_3: {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;

        log::trace!("result: {:#026b}", result);
    }

    /// 121. STS – Store Direct to Data Space
    /// This is the 32 bit variant of the STS command. There is also a 16 variant (122. STS 16bit)
    /// 1001 001d dddd 0000
    /// kkkk kkkk kkkk kkkk
    fn encode_sts(&self, segment: &mut Segment, register_d: u16, imm_value_k: u16) {

        if register_d > 31 {
            panic!("Invalid register for STS (32bit)! Only registers [r0, r31] are allowed")
        }

        let result: u32 = (0b1001001u32 << 25)
            | ((register_d as u32) << 20)
            | (imm_value_k as u32);

        log::trace!("result: {:#32b}", result);

        log::trace!("ENC STS (32bit): {:#02x}", (result >> 16u16) as u8);
        segment.data.push((result >> 16u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC STS (32bit): {:#02x}", (result >> 24u16) as u8);
        segment.data.push((result >> 24u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC STS (32bit): {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC STS (32bit): {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    /// 122. STS (16-bit) – Store Direct to Data Space
    /// 1010 1kkk dddd kkkk
    ///
    /// NOTE: STS is only allowed for registers in the range from [r16, r31]
    /// The full 32 regsters cannot be used since there are only 4 bit of space to store the target register.
    /// The register parameter is diminished by 16 so that it fits into the 4 bit space in the instruction word.
    fn encode_sts_16bit(&self, segment: &mut Segment, register_d: u16, imm_value_k: u16) {

        if register_d < 15 || register_d > 31 {
            panic!("Invalid register for STS 16Bit! Only registers [r16, r31] are allowed")
        }
        // register is decreased by 16 to arrive at the register id
        let register: u16 = register_d - 16u16;

        let k_mask: u16 = ((imm_value_k >> 4u16) << 8u16) | (imm_value_k & 0x0Fu16);

        let result: u16 = 0xA800u16 | k_mask | (register << 4u16);

        log::trace!("ENC STS (16 bit): {:#02x}", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC STS (16 bit): {:#02x}", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }

    fn encode_subi(&self, segment: &mut Segment, register_d: u16, constant: u16) 
    {
        let result: u16 = 0x5000 | ((constant >> 4) << 8) | register_d << 4 | constant & 0x0F;

        log::trace!("ENC SUBI: {:#02x}\n", (result >> 0u16) as u8);
        segment.data.push((result >> 0u16) as u8);
        segment.size += 1u32;

        log::trace!("ENC SUBI: {:#02x}\n", (result >> 8u16) as u8);
        segment.data.push((result >> 8u16) as u8);
        segment.size += 1u32;
    }
}

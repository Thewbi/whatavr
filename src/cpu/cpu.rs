use std::collections::HashMap;

use crate::CSEG_HASHMAP;
use crate::HIGH_U16;
use crate::LOW_U16;
use crate::common::common_constants::RAMEND;
use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::instructions::instructions::INSTRUCTIONS;
use crate::instructions::instructions::UNKNOWN;
use crate::instructions::{
    decode::decode_instruction, instruction_definition::InstructionDefinition,
    instruction_type::InstructionType,
};
use crate::{HIGH, HIGH_HIGH_I32, HIGH_I16, HIGH_I32, LOW, LOW_I16, LOW_I32, LOW_LOW_I32};

// the value is placed at the stackpointer then, after that, the stack pointer is decremented
pub fn push_to_stack_u8(cpu: &mut CPU, data: u8) {
    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize] = data;
    decrement_stack_pointer(cpu);
}

#[allow(dead_code)]
pub fn push_to_stack_u16(cpu: &mut CPU, data: &u16) {
    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);
}

pub fn push_to_stack_i16(cpu: &mut CPU, data: i16) {
    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH_I16!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW_I16!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);
}

#[allow(dead_code)]
pub fn push_to_stack_i32(cpu: &mut CPU, data: i32) {
    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH_HIGH_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW_LOW_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);
}

pub fn pop_from_stack_u8(cpu: &mut CPU) -> u8 {
    // pre-increment
    increment_stack_pointer(cpu);

    // get value
    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    cpu.sram[(stack_pointer - 1) as usize]
}

fn increment_stack_pointer(cpu: &mut CPU) {

    let mut stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    log::trace!("{:#06x}\n", stack_pointer);
    
    stack_pointer += 1u16;
    *cpu.sph() = HIGH!(stack_pointer).try_into().unwrap();
    *cpu.spl() = LOW!(stack_pointer).try_into().unwrap();

    stack_pointer = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    log::trace!("{:#06x}\n", stack_pointer);
}

fn decrement_stack_pointer(cpu: &mut CPU) {
    let mut stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    stack_pointer -= 1u16;
    *cpu.sph() = HIGH!(stack_pointer).try_into().unwrap();
    *cpu.spl() = LOW!(stack_pointer).try_into().unwrap();
}

pub struct CPU {

    // the carry flag
    pub c: bool,

    // the zero flag
    pub z: bool,

    // Negative Flag
    pub n: bool,

    // Two's complement overflow indicator
    pub v: bool,

    // N ⊕ V, for signed tests
    pub s: bool,

    // the half-carry flag
    pub h: bool,

    // Transfer bit used by BLD and BST instructions
    pub t: bool,

    // Global Interrupt Enable/Disable Flag
    pub i: bool,

    // pc always points to the instruction after the current instruction so it does not start at 0x00 but at 0x02
    pub pc: i32,

    // register file, 32 8bit registers
    pub register_file: [u8; 32usize],

    // sram todo: move to own struct / trait!
    // sram is the data segment
    pub sram: [u8; RAMEND as usize],

    // special function register
    pub sfr: [u8; 255usize],
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            c: false,
            z: false,
            n: false,
            v: false,
            s: false,
            h: false,
            t: false,
            i: false,
            pc: 0x00i32,
            register_file: [0; 32usize],
            sram: [0; RAMEND as usize],
            sfr: [0; 255usize]
        }
    }
}

impl std::fmt::Display for CPU {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "r16: {:02X?}", self.register_file[16])?;
        write!(f, "\nr17: {:02X?}", self.register_file[17])?;
        write!(f, "\nr18: {:02X?}", self.register_file[18])?;
        write!(f, "\nr19: {:02X?}", self.register_file[19])
    }

}

impl CPU {

    #[allow(dead_code, unused)]
    pub fn new(pc: i32, register_file: [u8; 32usize], sram: [u8; RAMEND as usize], sfr: [u8; 255usize]) -> Self {
        Self {
            c: false,
            z: false,
            n: false,
            v: false,
            s: false,
            h: false,
            t: false,
            i: false,
            pc: pc,
            register_file: register_file,
            sram: sram,
            sfr: sfr
        }
    }

    pub fn stack_info_high(&mut self) -> String {
        format!("{:02X?}", self.sph())
    }

    pub fn stack_info_low(&mut self) -> String {
        format!("{:02X?}", self.spl())
    }

    // https://stackoverflow.com/questions/35390615/writing-getter-setter-properties-in-rust
    fn sph(&mut self) -> &mut u8 {

        let map = CSEG_HASHMAP.lock().unwrap();
        let value_as_string = map.get("SPH").unwrap();

        let without_prefix = value_as_string.trim_start_matches("0x");
        let value: usize = usize::from_str_radix(without_prefix, 16).unwrap();

        return &mut self.sfr[value];

        // // ATmega328p
        // // 6.5.1 SPH and SPL – Stack Pointer High and Stack Pointer Low Register
        // // 0x3E (0x5E) stack pointer high
        // return &mut self.sfr[0x3E];
    }

    fn spl(&mut self) -> &mut u8 {

        let map = CSEG_HASHMAP.lock().unwrap();
        let value_as_string = map.get("SPL").unwrap();

        let without_prefix = value_as_string.trim_start_matches("0x");
        let value: usize = usize::from_str_radix(without_prefix, 16).unwrap();

        return &mut self.sfr[value];

        // // ATmega328p
        // // 6.5.1 SPH and SPL – Stack Pointer High and Stack Pointer Low Register
        // // 0x3D (0x5D) stack pointer low
        // return &mut self.sfr[0x3D];
    }

    fn get_sph(&self) -> u8 {

        let map = CSEG_HASHMAP.lock().unwrap();
        let value_as_string = map.get("SPH").unwrap();

        let without_prefix = value_as_string.trim_start_matches("0x");
        let value: usize = usize::from_str_radix(without_prefix, 16).unwrap();

        return self.sfr[value];

        // return self.sfr[0x3E];
    }

    fn get_spl(&self) -> u8 {

        let map = CSEG_HASHMAP.lock().unwrap();
        let value_as_string = map.get("SPL").unwrap();

        let without_prefix = value_as_string.trim_start_matches("0x");
        let value: usize = usize::from_str_radix(without_prefix, 16).unwrap();

        return self.sfr[value];

        // return self.sfr[0x3D];
    }

    #[allow(dead_code, unused)]
    fn register_z_high(&mut self) -> &mut u8 {
        return &mut self.register_file[30];
    }

    #[allow(dead_code, unused)]
    fn register_z_low(&mut self) -> &mut u8 {
        return &mut self.register_file[31];
    }

    fn set_c(&mut self) {
        self.c = true;
    }

    #[allow(dead_code)]
    fn reset_c(&mut self) {
        self.c = false;
    }

    #[allow(dead_code)]
    fn get_c(&mut self) -> bool {
        self.c
    }

    fn set_h(&mut self) {
        self.h = true;
    }

    #[allow(dead_code)]
    fn reset_h(&mut self) {
        self.h = false;
    }

    #[allow(dead_code)]
    fn get_h(&mut self) -> bool {
        self.h
    }

    fn get_x(&mut self) -> u16 {
        //((self.register_file[26] as u16) << 8u16) | self.register_file[27] as u16
        ((self.register_file[27] as u16) << 8u16) | self.register_file[26] as u16
    }

    fn set_x(&mut self, value: u16) {
        // self.register_file[26] = HIGH_U16!(value);
        // self.register_file[27] = LOW_U16!(value);
        self.register_file[26] = LOW_U16!(value);
        self.register_file[27] = HIGH_U16!(value);
    }

    fn get_y(&mut self) -> u16 {
        //((self.register_file[28] as u16) << 8u16) | self.register_file[29] as u16
        ((self.register_file[29] as u16) << 8u16) | self.register_file[28] as u16
    }

    fn set_y(&mut self, value: u16) {
        // self.register_file[28] = HIGH_U16!(value);
        // self.register_file[29] = LOW_U16!(value);
        self.register_file[28] = LOW_U16!(value);
        self.register_file[29] = HIGH_U16!(value);
    }

    fn get_z(&mut self) -> u16 {
        //((self.register_file[30] as u16) << 8u16) | self.register_file[31] as u16
        ((self.register_file[31] as u16) << 8u16) | self.register_file[30] as u16
    }

    fn set_z(&mut self, value: u16) {
        // self.register_file[30] = HIGH_U16!(value);
        // self.register_file[31] = LOW_U16!(value);
        self.register_file[30] = LOW_U16!(value);
        self.register_file[31] = HIGH_U16!(value);
    }

    // pub fn is_sfr_name(&mut self, input: &str) -> bool 
    // {
    //     log::info!("sfr: {}\n", input);
    //     if input.to_lowercase().eq("sph") || input.to_lowercase().eq("spl")
    //     {
    //         return true;
    //     }
    //     false
    // }

    // pub fn sfr_name_to_u8(&mut self, input: &str) -> u8 
    // {
    //     let map = HASHMAP.lock().unwrap();
    //     let value_as_string = map.get(input).unwrap();

    //     let without_prefix = value_as_string.trim_start_matches("0x");
    //     let value: usize = usize::from_str_radix(without_prefix, 16).unwrap();

    //     return self.sfr[value];
    // }

    // data_space is terminology used in the datasheet of the ATmega328p for SRAM memory access
    // as opposed to I/O space which is terminology for the memory mapped special function registers
    fn store_to_data_space(&mut self, address: usize, value: u8) {
        //todo!("Store value {} to address {} in data space!", value, address);
        log::info!("\tStore value {} {:#04X} to address {} {:#04X} in data space!\n", value, value, address, address);
        log::trace!("\n");

        self.sram[address] = value;
    }

    fn load_from_data_space(&mut self, address: usize) -> u8 {
        //todo!("Load byte from address {} in data space!", address);

        let value: u8 = self.sram[address];
        log::info!("\tLoaded value {} {:#04X} at address {} {:#04X} from data space!\n", value, value, address, address);
        log::trace!("\n");

        value
    }

    fn store_to_i_o_space(&mut self, address: usize, value: u8) {
        // output the value stored in register r_val into memory to the address a_val
        self.sfr[address] = value;
    }

    fn read_from_i_o_space(&mut self, address: usize) -> u8 {
        self.sfr[address]
    }

    fn read_next_two_byte(&mut self, segment: &Segment) -> u16 {

        // get the next 16 stored at the pc since the JMP command is encoded using 32 bit (4 byte)
        let temp_pc: i32 = self.pc;

        let word_hi: u16 = segment.data[(temp_pc + 1i32) as usize] as u16;
        //log::info!("READ: {:#02x}", word_hi as u8);

        let word_lo: u16 = segment.data[temp_pc as usize] as u16;
        //log::info!("READ: {:#02x}", word_lo as u8);

        let result: u16 = ((word_hi << 8u8) + word_lo).into();

        result

    }

    fn read_next_four_byte(&mut self, segment: &Segment, k_hi: &u32) -> i32 {

        // // get the first 16 bit
        // let k_hi: u32 = value_storage[&'k'].into();

        // // get the next 16 stored at the pc since the JMP command is encoded using 32 bit (4 byte)
        // let temp_pc: i32 = cpu.pc;

        // let word_hi: u16 = segment.data[(temp_pc + 1i32) as usize] as u16;
        // log::trace!("READ: {:#02x}", word_hi as u8);

        // let word_lo: u16 = segment.data[temp_pc as usize] as u16;
        // log::trace!("READ: {:#02x}", word_lo as u8);

        // let k_lo: u32 = ((word_hi << 8u8) + word_lo).into();

        // // assemble the parameter k
        // let k_val: i16 = ((k_hi << 16u8) + k_lo) as i16;

        // // extend to i32
        // let k_val_i32: i32 = k_val as i32;

        // get the first 16 bit
        //let k_hi: u32 = self.value_storage[&'k'].into();

        let k_lo: u32 = self.read_next_two_byte(segment) as u32;

        // assemble the parameter k
        let k_val: i16 = ((k_hi << 16u8) + k_lo) as i16;

        // extend to i32
        let k_val_i32: i32 = k_val as i32;

        k_val_i32

    }

    pub fn execute_instruction(&mut self, segment: &Segment) {

        // get the current instruction
        let temp_pc: i32 = self.pc;

        //log::info!("pc: {:#04x} | ", temp_pc);
        //let mut os: String = "pc: ".to_string();
        let mut os: String = format!("\tpc: {:#04x} | ", temp_pc);
        //os.push_str(&temp_pc.to_string());
        //os.push_str(&format!("pc: {} | ", temp_pc));
        // os.push_str(&" | ".to_string());

        // check for end of code
        if segment.size <= temp_pc as u32 {
            log::info!("End of Code reached! Application Finished!\n");
            return;
        }

        // assemble the instruction word
        let word_hi: u16 = segment.data[(temp_pc + 1i32) as usize].into();
        let word_lo: u16 = segment.data[temp_pc as usize].into();
        let word: u16 = (word_hi << 8u8) + word_lo;

        // this will print the word that contains the encoded instruction that is executed next
        log::trace!("hex: {:04x}\n", word);
        log::trace!("bin: {:#018b}\n", word);

        // decode the current instruction
        let mut value_storage: HashMap<char, u16> = HashMap::new();
        let instruction: &InstructionDefinition =
            decode_instruction(word, INSTRUCTIONS, &UNKNOWN, &mut value_storage);

        let mut cpu: &mut CPU = self;

        // DEBUG
        log::trace!("Executing instruction: {:?}\n", instruction.instruction_type);

        // execute the instruction
        match instruction.instruction_type {

            /*   5 */
            InstructionType::ADC => {
                log::trace!("[ADC]\n");

                let d_value: usize = value_storage[&'d'] as usize;
                let r_value: usize = value_storage[&'r'] as usize;

                let carry_as_u8: u8 = cpu.c as u8;

                // since rust panics when a result does not fit into a variable
                // and two u8 can make a u16, store temp result in a u16 variable
                let mut temp_result: u16 = cpu.register_file[d_value] as u16;
                temp_result += cpu.register_file[r_value] as u16;
                temp_result += carry_as_u8 as u16;

                // if there is an overflow ...
                if temp_result > 255u16 {

                    // set carry and half carry
                    cpu.set_c();
                    cpu.set_h();

                    // store only the lower byte into the register
                    cpu.register_file[d_value] = LOW_U16!(temp_result);

                } else {
                    cpu.register_file[d_value] += cpu.register_file[r_value] + carry_as_u8;
                }

                // DEBUG
                log::trace!("[ADC] result value: {}\n", cpu.register_file[d_value]);

                log::info!("adc r{}, d{} - carry: {}\n", r_value, d_value, cpu.c);

                cpu.pc += 2i32;
            }

            /*   6 */
            InstructionType::ADD => {
                log::trace!("[ADD]\n");

                let d_value: usize = value_storage[&'d'] as usize;
                let r_value: usize = value_storage[&'r'] as usize;

                // since rust panics when a result does not fit into a variable
                // and two u8 can make a u16, store temp result in a u16 variable
                let mut temp_result: u16 = cpu.register_file[d_value] as u16;
                temp_result += cpu.register_file[r_value] as u16;

                // if there is an overflow ...
                if temp_result > 255u16 {

                    // set carry and half carry
                    cpu.set_c();
                    cpu.set_h();

                    // store only the lower byte into the register
                    cpu.register_file[d_value] = LOW_U16!(temp_result);

                } else {
                    cpu.register_file[d_value] += cpu.register_file[r_value];
                }

                log::trace!(
                    "[ADD] result value: {}\n",
                    cpu.register_file[d_value]
                );

                //log::info!("add r{}, r{}\n", d_value, r_value);
                os.push_str(&format!("add r{}, r{}", d_value, r_value));
                log::info!("{}\n", os);

                cpu.pc += 2i32;
            }

            /*   8 */
            // AND – Logical AND
            // 0010 00rd dddd rrrr
            InstructionType::AND => {
                log::trace!("[AND]\n");

                let d_reg_idx: usize = value_storage[&'d'] as usize;
                let r_reg_idx: usize = value_storage[&'r'] as usize;

                //log::info!("and d{}, r{}\n", d_reg_idx, r_reg_idx);
                os.push_str(&format!("and d{}, r{}", d_reg_idx, r_reg_idx));
                log::info!("{}\n", os);

                let d_reg_val = cpu.register_file[d_reg_idx];
                let r_reg_val = cpu.register_file[r_reg_idx];

                // // since rust panics when a result does not fit into a variable
                // // and two u8 can make a u16, store temp result in a u16 variable
                // let mut temp_result: u16 = cpu.register_file[d_value] as u16;
                // temp_result &= cpu.register_file[r_value] as u16;

                // // if there is an overflow ...
                // if temp_result > 255u16 {

                //     // set carry and half carry
                //     cpu.set_c();
                //     cpu.set_h();

                //     // store only the lower byte into the register
                //     cpu.register_file[d_value] = LOW_U16!(temp_result);

                // } else {
                //     cpu.register_file[d_value] += cpu.register_file[r_value];
                // }

                cpu.register_file[d_reg_idx] = d_reg_val & r_reg_val;

                log::trace!("[AND] result value: {}\n", cpu.register_file[d_reg_idx]);

                cpu.pc += 2i32;
            }

            /*  18 */
            InstructionType::ANDI => {
                log::trace!("[ANDI]\n");

                let d_value: usize = value_storage[&'d'] as usize;
                let k_value: u8 = value_storage[&'K'] as u8;

                let register: usize = d_value + 16usize;
                log::trace!("[ANDI] Using register: r{}\n", register);

                //log::info!("andi r{}, {}\n", register, k_value);
                os.push_str(&format!("andi r{}, {}\n", register, k_value));
                log::info!("{}\n", os);

                let register_value: u8 = cpu.register_file[register];

                let add_result: u8 = register_value & k_value;

                // zero flag
                cpu.z = false;
                if 0x00 == add_result
                {
                    cpu.z = true;
                }

                // write back value
                cpu.register_file[register] = add_result;

                cpu.pc += 2i32;
            }

            /*  13 */
            // 1111 01kk kkkk ksss – BRBC - Branch if Bit in SREG is Cleared
            // 1111 01kk kkkk k001 - BRNE – Branch if Not Equal
            InstructionType::BRBC => {
                log::trace!("[BRBC]\n");

                //process_brbc(rdr, &word, index, value_storage);

                let k_val: u8 = value_storage[&'k'] as u8;
                log::trace!("K: {k_val:#b} {k_val:#x} {k_val}\n");

                // todo
                // converting bit mask to i8
                // 1. the amount of bits in the input has to be known, e.g. six bits
                // 2. sign extend the bits with 1 to then next datatype six bits -> u8, 12 bits -> u16
                // 3. convert from unsigned to signed: e.g. u8 -> i8, u16 -> i16, u32 -> i32
                // 4. The correct value with correct sign is contained in the signed variable. Done

                // sign extend
                let k_val_sign_extend: u8 = (k_val | 0b11000000).try_into().unwrap();

                let k_val_signed: i8 = k_val_sign_extend as i8;
                log::trace!("k_val_signed: {}\n", k_val_signed);

                let s_val = value_storage[&'s'];
                //log::info!("d: {d_val:#b} {d_val:#x}");

                // https://en.wikipedia.org/wiki/Two%27s_complement
                // Step 1: starting with the equivalent positive number.
                // 2 = 000010
                // Step 2: inverting (or flipping) all bits – changing every 0 to 1, and every 1 to 0;
                // 111101
                // Step 3: adding 1 to the entire inverted number, ignoring any overflow. 
                // Accounting for overflow will produce the wrong value for the result.
                // 111110

                // twos-complement
                //let offset: i8 = (k_val as i8 * -1i8) as i8;
                let offset: i8 = k_val as i8;
                //let offset: u8 = (k_val as i8 * -1i8) as u8;

                log::trace!("offset: {}\n", offset);

                //log::info!("brbc {}, {}\n", s_val, k_val);
                os.push_str(&format!("brbc s{}, k{}", s_val, k_val));
                log::info!("{}\n", os);

                // todo find a better way to deal with the SREG array of status bits
                if s_val == 1 {
                    // zero flag

                    // same code as BRNE

                    // check the Z-flag
                    if cpu.z {
                        cpu.pc += 2i32;
                    } else {
                        //let offset_in_bytes: i8 = offset / 8i8;
                        //cpu.pc = (cpu.pc as i16 + 2*offset_in_bytes as i16) as i32;

                        cpu.pc += k_val_signed as i32;
                    }
                }
            }

            /*  14 */
            InstructionType::BRBS => {
                log::trace!("[BRBS]\n");

                let k_val: i32 = value_storage[&'k'] as i32;
                log::trace!("k_val: {}\n", k_val);
                let s_val: u16 = value_storage[&'s'];
                log::trace!("s_val: {}\n", s_val);

                //log::info!("brbs {}, {}\n", s_val, k_val);
                os.push_str(&format!("brbs {}, {}", s_val, k_val));
                log::info!("{}\n", os);

                let perform_jump: bool;

                match s_val
                {
                    1 => { perform_jump = cpu.z },
                    _ => panic!("not implemented yet!"),
                }

                if perform_jump
                {
                    // jump (relative to pc branch)
                    cpu.pc += k_val * 2i32;
                }
            }

            /*  17 */
            // The BREAK instruction is used by the On-chip Debug system, and is normally not used in the application
            // software. When the BREAK instruction is executed, the AVR CPU is set in the Stopped Mode. This gives
            // the On-chip Debugger access to internal resources.
            // If any Lock bits are set, or either the JTAGEN or OCDEN Fuses are unprogrammed, the CPU will treat
            // the BREAK instruction as a NOP and will not enter the Stopped mode.
            // This instruction is not available in all devices. Refer to the device specific instruction set summary.
            InstructionType::BREAK => {
                log::trace!("[BREAK]\n");

                // nop
                panic!("Break reached");

                cpu.pc += 2i32;
            }

            // /*  18 */
            // InstructionType::BREQ => {
            //     log::info!("[BREQ]");
            //     // same as BRBS which is the more general command
            //     // whereas BREQ is only an alias to one of the BRBS variants
            // }

            /*  27 */
            InstructionType::BRNE => {
                log::trace!("[BRNE]\n");

                let offset: i32 = value_storage[&'k'] as i32;

                os.push_str(&format!("brne relative_offset:{}", offset));
                log::info!("{}\n", os);

                // check the Z-flag
                if cpu.z {
                    cpu.pc += 2*offset;
                } else {
                    cpu.pc += 2i32;
                }
            }

            /* 36 */
            InstructionType::CALL => {
                log::trace!("[CALL]\n");

                // get the first 16 bit
                let k_val: u32 = value_storage[&'k'].into();

                // get the next 16 bit
                cpu.pc += 2;
                let k_val_i32: i32 = cpu.read_next_four_byte(segment, &k_val);

                os.push_str(&format!("stack pointer (before): {} {} | ", cpu.stack_info_high(), cpu.stack_info_low()));

                // push return address onto the stack
                let mut data: i32 = cpu.pc;

                // the manual explicitly states that pc+2 is pushed onto the stack
                data += 2i32;

                push_to_stack_i16(&mut cpu, data as i16);

                log::trace!("call - >>> Pushed to stack: {data:#06X}\n");

                let sph_temp: u8 = cpu.get_sph();
                let spl_temp: u8 = cpu.get_spl();

                log::trace!("call - stack pointer: {:#04x} {:#04x}\n", sph_temp, spl_temp);
                log::trace!("call {:#04x} {:#04x}\n", sph_temp, spl_temp);

                log::trace!("stack pointer: {} {}\n", cpu.stack_info_high(), cpu.stack_info_low());

                // jump to address
                cpu.pc = k_val_i32 * 2;

                os.push_str(&format!("call cpu.pc: {:#06x}", cpu.pc));
                os.push_str(&format!(" | stack pointer (after): {} {}", cpu.stack_info_high(), cpu.stack_info_low()));
                log::info!("{}\n", os);
            }

            // /*  41 */
            // InstructionType::CLI => {
            //     let k_val = value_storage[&'K'];
            //     //log::info!("K: {k_val:#b} {k_val:#x}");
            //     let d_val = value_storage[&'d'];
            //     //log::info!("d: {d_val:#b} {d_val:#x}");

            //     // "Loads an 8-bit constant directly to register 16 to 31."
            //     // To compute the register to use, add the offset 16 to the parsed value
            //     //let register = d_val + 16;
            //     let register = d_val;

            //     //log::info!("[LDI] Using register: r{}", register);

            //     log::info!("{temp_pc:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}");

            //     pc += 2u32;
            // },

            /*  43 */
            // eor	r1, r1 (register with itself, is encoded to the same machine code as clr r1)
            InstructionType::CLR => {
                log::trace!("[CLR]\n");

                let d: u16 = value_storage[&'d'];

                log::trace!("Clearing register d: {:#06x}\n", d);

                //log::info!("clr {}\n", d);
                os.push_str(&format!("clr {}", d));
                log::info!("{}\n", os);

                cpu.register_file[d as usize] = 0x00;

                // updating flags
                cpu.s = false;
                cpu.v = false;
                cpu.n = false;
                cpu.z = true;

                cpu.pc += 2i32;
            }

            /*  51 */
            // CPI Rd,K This instruction performs a compare between register Rd and a constant. The register is not changed. All
            // conditional branches can be used after this instruction.
            InstructionType::CPI => {
                log::trace!("[CPI]\n");

                let d_val: u16 = value_storage[&'d'];
                let k: u16 = value_storage[&'K'];

                // To compute the register to use, add the offset 16 to the parsed value
                let register: u16 = d_val + 16u16;

                os.push_str(&format!("cpi r{}, {:#04x}", register, k));
                log::info!("{}\n", os);

                let register_value: u8 = cpu.register_file[register as usize];

                // todo compute Rd - K as specified in the datasheet and set all other bits correctly
                if register_value == (k as u8)
                {
                    cpu.z = true;
                }

                // cpu.register_file[d as usize] = 0x00;

                // // updating flags
                // cpu.s = false;
                // cpu.v = false;
                // cpu.n = false;
                // cpu.z = true;

                cpu.pc += 2i32;
            }

            /*  53 */
            InstructionType::DEC => {
                log::trace!("[DEC]\n");

                let d: u16 = value_storage[&'d'];

                // perform the decrement
                cpu.register_file[d as usize] -= 0x01u8;

                // set the z flag
                if cpu.register_file[d as usize] == 0x00u8 {
                    cpu.z = true;
                }

                log::trace!(
                    "[DEC] Register r{}: value at reg:{:#06x}\n",
                    d,
                    cpu.register_file[d as usize]
                );

                //log::info!("dec {}\n", d);
                os.push_str(&format!("dec r{}", d));
                log::info!("{}\n", os);

                cpu.pc += 2i32;
            }

            /*  64 */
            InstructionType::IN => {
                log::trace!("[IN]\n");

                let register_d: u16 = value_storage[&'d'];
                let address: u16 = value_storage[&'A'];

                let val: u8 = cpu.sram[address as usize];

                log::info!("[IN] value from read-operation: {:#06x}\n", val);

                cpu.register_file[register_d as usize] = val as u8;

                log::info!(
                    "[IN] Register r{}: value at reg:{:#06x}\n",
                    register_d,
                    cpu.register_file[register_d as usize]
                );

                cpu.pc += 2i32;
            }

            /*  65 */
            InstructionType::INC => {
                log::trace!("[INC]\n");

                let register_d: u16 = value_storage[&'d'];

                os.push_str(&format!("inc r{}", register_d));
                log::info!("{}\n", os);

                log::trace!(
                    "[INC] Before Inc: Register r{}: value at reg:{:#06x}\n",
                    register_d,
                    cpu.register_file[register_d as usize]
                );

                os.push_str(&format!("inc r{}", register_d));
                log::info!("{}\n", os);

                let mut val: u8 = cpu.register_file[register_d as usize];
                //todo: handle overflow panic and set the zero (and all other relevant) flags in this case!
                if std::u8::MAX == val
                {
                    val = 0;
                    cpu.z = true;
                }
                else
                {
                    val = val + 1;
                    cpu.z = false;
                }
                cpu.register_file[register_d as usize] = val as u8;

                log::trace!(
                    "[INC] Before Inc: Register r{}: value at reg:{:#06x}\n",
                    register_d,
                    cpu.register_file[register_d as usize]
                );

                cpu.pc += 2i32;
            }

            /*  66 */
            // 1001 010k kkkk 110k
            // kkkk kkkk kkkk kkkk
            InstructionType::JMP => {
                log::trace!("[JMP]\n");

                // get the first 16 bit
                let k_hi: u32 = value_storage[&'k'].into();
                log::trace!("k_hi: {:#4x}\n", k_hi);

                cpu.pc += 2i32;

                // get the next 16 bit
                let k_lo: u32 = cpu.read_next_two_byte(segment) as u32;
                log::trace!("k_lo: {:#4x}", k_lo);
                //let k_val_i32: i32 = cpu.read_next_four_byte(segment, &k_hi);

                let k_val: i32 = ((k_hi << 8u32) + k_lo as u32) as i32;

                log::trace!("k_val: {:#6x}\n", k_val);

                // log::info!("jmp: {:#6x}\n", k_val);
                os.push_str(&format!("jmp: {:#6x}", k_val));
                log::info!("{}\n", os);

                //cpu.pc += k_val;

                cpu.pc = k_val * 2i32;
            }

            /*  70 */
            InstructionType::LD_LDD_X_1 => {
                log::trace!("[LD_LDD_X_1]\n");

                // retrieved the encoded value for the d register
                let d_val: u16 = value_storage[&'d'];
                log::trace!("d: {d_val:#b} {d_val:#x} {d_val}\n");

                // retrieve the current value from the r register
                //let data: u8 = cpu.register_file[d_val as usize];

                // retrieve the data (address) stored inside X
                let value_x: u16 = cpu.get_x();

                // store data into memory (data space) at the address stored in X
                // (data space != I/O space (for I/O space, use the OUT instruction))
                let value: u8 = cpu.load_from_data_space(value_x as usize);

                // store the loaded value into the r register
                cpu.register_file[d_val as usize] = value;

                // // (post) increment X
                // value_x = value_x + 1;

                // // write back Z
                // cpu.set_z(value_z);

                cpu.pc += 2i32;
            }
            InstructionType::LD_LDD_X_2 => {
                log::trace!("[LD_LDD_X_2]\n");

                // retrieved the encoded value for the d register
                let d_val: u16 = value_storage[&'d'];
                log::trace!("d: {d_val:#b} {d_val:#x} {d_val}\n");

                // retrieve the data (address) stored inside X
                let mut value_x: u16 = cpu.get_x();
                log::trace!("X: {value_x:#b} {value_x:#x} {value_x}\n");

                // store data into memory (data space) at the address stored in X
                // (data space != I/O space (for I/O space, use the OUT instruction))
                let value: u8 = cpu.load_from_data_space(value_x as usize);

                os.push_str(&format!("ld r{}, X+", d_val));
                log::info!("{}\n", os);

                // store the loaded value into the r register
                cpu.register_file[d_val as usize] = value;

                // (post) increment X
                value_x = value_x + 1;

                // write back X
                cpu.set_x(value_x);

                cpu.pc += 2i32;
            }

            /*  72 */
            // 72. LD (LDD) – Load Indirect From Data Space to Register using Index Z
            InstructionType::LD_LDD_Z_1 => {
                log::trace!("[LD_LDD_Z_1]\n");

                // retrieved the encoded value for the d register
                let d_val: u16 = value_storage[&'d'];
                log::trace!("d: {d_val:#b} {d_val:#x} {d_val}\n");

                //log::info!("ld r{}, Z\n", d_val);
                os.push_str(&format!("ld r{}, Z", d_val));
                log::info!("{}\n", os);

                // retrieve the current value from the r register
                //let data: u8 = cpu.register_file[d_val as usize];

                // retrieve the data (address) stored inside Z
                let value_z: u16 = cpu.get_z();

                // store data into memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                let value: u8 = cpu.load_from_data_space(value_z as usize);

                // store the loaded value into the r register
                cpu.register_file[d_val as usize] = value;

                // // (post) increment X
                // value_x = value_x + 1;

                // // write back Z
                // cpu.set_z(value_z);

                cpu.pc += 2i32;
            }

            /*  73 */
            InstructionType::LDI => {
                log::trace!("[LDI]\n");

                let k_val = value_storage[&'K'];
                log::trace!("K: {k_val:#b} {k_val:#x}\n");
                let d_val = value_storage[&'d'];
                log::trace!("d: {d_val:#b} {d_val:#x}\n");

                // "Loads an 8-bit constant directly to register 16 to 31."
                // To compute the register to use, add the offset 16 to the parsed value
                let register: u16 = d_val + 16u16;
                //log::info!("[LDI] Using register: r{}", register);

                log::trace!("{temp_pc:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}\n");
                //log::info!("ldi r{register:#02}, {k_val:#02x}\n");

                os.push_str(&format!("ldi r{register:#02}, {k_val:#02x}"));
                log::info!("{}\n", os);

                // execute
                cpu.register_file[register as usize] = k_val as u8;

                cpu.pc += 2i32;
            }

            /*  76 */
            InstructionType::LPM_1 => {
                log::trace!("[LPM_1]\n");

                // copy value pointed to by Z into the register r0

                // retrieve the data (address) stored inside Z
                let value_z: u16 = cpu.get_z();

                // load data from memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                let value: u8 = cpu.load_from_data_space(value_z as usize);

                let register: u16 = 0u16;
                cpu.register_file[register as usize] = value;

                cpu.pc += 2i32;
            }
            InstructionType::LPM_2 => {
                log::trace!("[LPM_2]\n");

                // copy value pointed to by Z into the destination register r?

                // retrieve the data (address) stored inside Z
                let value_z: u16 = cpu.get_z();

                // load data from memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                let value: u8 = cpu.load_from_data_space(value_z as usize);

                let d_val: u16 = value_storage[&'d'];
                cpu.register_file[d_val as usize] = value;

                cpu.pc += 2i32;
            }
            InstructionType::LPM_3 => {
                log::trace!("[LPM_3]\n");

                // copy value pointed to by Z into the destination register r?
                // post increment Z

                // retrieve the data (address) stored inside Z
                let mut value_z: u16 = cpu.get_z();

                // load data from memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                let value: u8 = cpu.load_from_data_space(value_z as usize);

                let d_val: u16 = value_storage[&'d'];
                cpu.register_file[d_val as usize] = value;

                // (post) increment Z
                value_z = value_z + 1;

                // write back Z
                cpu.set_z(value_z);

                cpu.pc += 2i32;
            }

            /*  78 */
            // 78. LSR – Logical Shift Right
            // Shifts all bits in Rd one place to the right. Bit 7 is cleared. Bit 0 is loaded into the C Flag of the SREG.
            // This operation effectively divides an unsigned value by two. The C Flag can be used to round the result.
            InstructionType::LSR => {
                log::trace!("[LSR]\n");

                let d_register: u16 = value_storage[&'d'];
                log::trace!("d: {d_register:#b} {d_register:#x}\n");

                os.push_str(&format!("lsr r{:?}", d_register));
                log::info!("{}\n", os);

                let mut d_val: u8 = cpu.register_file[d_register as usize];

                log::info!("Before: {}\n", d_val);

                // Bit 0 is loaded into the C Flag of the SREG
                cpu.c = false;
                if d_val % 2u8 == 1
                {
                    cpu.c = true;
                }

                d_val /= 2u8;

                // set zero flag
                cpu.z = false;
                if d_val == 0
                {
                    cpu.z = true;
                }

                cpu.register_file[d_register as usize] = d_val;

                log::info!("After: {}\n", d_val);

                cpu.pc += 2i32;
            }

            /*  79 */
            InstructionType::MOV => {
                log::trace!("[MOV]\n");

                let r_register: u16 = value_storage[&'r'];
                log::trace!("K: {r_register:#b} {r_register:#x}\n");
                let d_register: u16 = value_storage[&'d'];
                log::trace!("d: {d_register:#b} {d_register:#x}\n");

                let k_val: u8 = cpu.register_file[r_register as usize];

                cpu.register_file[d_register as usize] = k_val as u8;

                cpu.pc += 2i32;
            }

            /*  85 */
            InstructionType::NOP => {
                log::trace!("[NOP]\n");

                log::info!("nop\n");

                cpu.pc += 2i32;
            }

            /*  87 */
            // ORI – Logical OR with Immediate
            // 0110 KKKK dddd KKKK
            InstructionType::ORI => {
                log::trace!("[ORI]\n");

                // Stores data from register Rr in the Register File to I/O Space (Ports, Timers, Configuration Registers, etc.).
                let k_val: u8 = value_storage[&'K'] as u8;
                log::trace!("K: {k_val:#b} {k_val:#x} {k_val}\n");
                let d_val: u16 = value_storage[&'d'];
                log::trace!("d: {d_val:#b} {d_val:#x} {d_val}\n");

                let register: u16 = d_val + 16;

                // take value out of the register
                // log::trace!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                let reg_val: u8 = cpu.register_file[register as usize];

                // DEBUG
                //log::info!("ori r{:?}, {:?}\n", register, k_val);
                os.push_str(&format!("ori r{:?}, {:#04X}", register, k_val));
                log::info!("{}\n", os);

                // // output the value stored in register r_val into memory to the address a_val
                // cpu.store_to_i_o_space(a_val as usize, val);

                cpu.register_file[register as usize] = reg_val | k_val;

                cpu.pc += 2i32;
            }

            /*  88 */
            InstructionType::OUT => {
                log::trace!("[OUT]\n");

                // Stores data from register Rr in the Register File to I/O Space (Ports, Timers, Configuration Registers, etc.).
                let a_val: u16 = value_storage[&'A'];
                log::trace!("A: {a_val:#b} {a_val:#x} {a_val}\n");
                let r_val = value_storage[&'r'];
                log::trace!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // take value out of the register
                log::trace!("r_val: {r_val:#b} {r_val:#x} {r_val}\n");
                let val: u8 = cpu.register_file[r_val as usize];

                // DEBUG
                // log::info!("out {:?}, {:?}\n", a_val, val);
                os.push_str(&format!("out {:?}, {:?}", a_val, val));
                log::info!("{}\n", os);

                // output the value stored in register r_val into memory to the address a_val
                cpu.store_to_i_o_space(a_val as usize, val);

                cpu.pc += 2i32;
            }

            /*  89 */
            InstructionType::POP => {
                log::trace!("[POP]\n");

                let d_val: u16 = value_storage[&'d'];
                log::trace!("d: {d_val:#b} {d_val:#x} {d_val}\n");

                // load value
                let stack_content: u8 = pop_from_stack_u8(&mut cpu);

                os.push_str(&format!("pop r{} ", d_val));
                os.push_str(&format!(" (popped_value: {}, {:#04x})", stack_content, stack_content));
                log::info!("{}\n", os);

                cpu.register_file[d_val as usize] = stack_content;

                log::trace!("stack pointer: {} {}\n", cpu.stack_info_high(), cpu.stack_info_low());

                cpu.pc += 2i32;
            }

            /*  90 */
            InstructionType::PUSH => {
                log::trace!("[PUSH]\n");

                let d_val: u16 = value_storage[&'d'];
                log::trace!("d: {d_val:#b} {d_val:#x} {d_val}\n");

                let data: u8 = cpu.register_file[d_val as usize];

                // stack pointer before
                os.push_str(&format!("stack pointer (before): {} {} | ", cpu.stack_info_high(), cpu.stack_info_low()));

                push_to_stack_u8(&mut cpu, data);

                os.push_str(&format!("push {data:#x}"));

                // stack pointer after
                os.push_str(&format!(" | stack pointer (after): {} {}", cpu.stack_info_high(), cpu.stack_info_low()));
                log::info!("{}\n", os);

                log::trace!("stack pointer: {} {}\n", cpu.stack_info_high(), cpu.stack_info_low());

                cpu.pc += 2i32;
            }

            /* 91 */
            InstructionType::RCALL => {
                log::trace!("[RCALL]\n");

                // get the first 16 bit
                let mut k: u16 = value_storage[&'k'] as u16;

                log::trace!("k: {:04x} {:016b}\n", k, k);

                // sign extend (800 decimal = 1000 0000 0000 binary)
                if k >= 800 {
                    k |= 0xF000;
                }
                log::trace!("k: {:04x} {:016b} {}\n", k as i16, k as i16, k as i16);

                let kk: i16 = k as i16;

                log::trace!("kk: {:04x} {:016b} {}\n", kk, kk, kk);

                // push return address onto the stack
                let mut data: i32 = cpu.pc;

                // the manual explicitly states that pc+2 is pushed onto the stack
                data += 2i32;

                push_to_stack_i16(&mut cpu, data as i16);

                log::trace!("stack pointer: {} {}\n", cpu.stack_info_high(), cpu.stack_info_low());

                // jump to address
                cpu.pc += kk as i32;
            }

            /* 92 */
            InstructionType::RET => {
                log::trace!("[RET]\n");

                let k_lo: u16 = pop_from_stack_u8(&mut cpu) as u16;
                let k_hi: u16 = pop_from_stack_u8(&mut cpu) as u16;

                log::trace!("ret - <<< stack popped: {:#04x} {:#04x}\n", k_hi, k_lo);

                let k_val: i16 = ((k_hi << 8i16) + k_lo) as i16;

                log::trace!("ret - <<< stack popped: {k_val:#06X}\n");

                cpu.pc = k_val as i32;

                os.push_str(&format!("ret"));
                os.push_str(&format!(" (pc: {:#06x})", cpu.pc));
                log::info!("{}\n", os);

                //log::info!("ret - stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                //log::trace!("stack pointer: {} {}\n", cpu.stack_info_high(), cpu.stack_info_low());
                
                //log::info!("pc: {:#06x}\n", cpu.pc);
            }

            /*  94 */
            InstructionType::RJMP => {
                log::trace!("[RJMP]\n");

                // get the first 16 bit
                let mut k: u16 = value_storage[&'k'] as u16;
                log::trace!("k: {:04x} {:016b}\n", k, k);

                // sign extend (800 decimal = 1000 0000 0000 binary)
                if k >= 800 {
                    k |= 0xF000;
                }
                log::trace!("k: {:04x} {:016b} {}\n", k as i16, k as i16, k as i16);

                let twos_complement_1: i16 = k as i16;
                log::trace!("{}\n", twos_complement_1);

                let twos_complement: u16 = (k as i16 * -1i16) as u16;
                log::trace!("{}\n", twos_complement);

                let resultt: i16 = twos_complement as i16;
                log::trace!("{}\n", resultt);

                let kk: i32 = k as i32;
                log::trace!("kk: {:04x} {:016b} {}\n", kk, kk, kk);

                //let offset: i32 = ((twos_complement_1 + 1i16) * 2i16) as i32;
                //let offset: i32 = twos_complement_1 as i32;
                let offset: i32 = i32::from(twos_complement_1);

                //log::info!("rjmp: {:#04x} {}\n", offset, offset);
                os.push_str(&format!("rjmp: {:#04x} {}", offset, offset));
                log::info!("{}\n", os);

                if 0x00 == offset {
                  panic!("endless loop detected!\n");
                }

                cpu.pc += offset * 2i32;
            }

            /*  98 */
            InstructionType::SBCI => {
                log::trace!("[SBCI]\n");

                let k_val: u8 = value_storage[&'K'] as u8;
                let d_val: u16 = value_storage[&'d'] as u16;

                let register: u16 = d_val + 16;

                log::info!("sbci k: {}, d: {}\n", k_val, register);

                let data: u8 = cpu.register_file[register as usize];
                let mut arithmetic_val: u8 = data;
                arithmetic_val -= k_val;
                if cpu.c {
                    arithmetic_val -= 1u8;
                }
                cpu.register_file[register as usize] = arithmetic_val;

                cpu.pc += 2i32;
            }

            /*  99 */
            InstructionType::SBI => {
                log::trace!("[SBI]\n");

                let address: usize = value_storage[&'A'] as usize;
                let bit: u16 = value_storage[&'b'] as u16;

                log::info!("sbi {}, {}\n", address, bit);

                // read from special function registers
                let mut register_value:u8 = cpu.read_from_i_o_space(address);

                register_value |= 1 << bit;

                // output the value stored in register r_val into memory to the address a_val
                cpu.store_to_i_o_space(address as usize, register_value);

                cpu.pc += 2i32;
            }

            /* 104 */
            // 104. SBRC – Skip if Bit in Register is Cleared
            // 1111 110r rrrr 0bbb
            InstructionType::SBRC => {

                log::trace!("[SBRC]\n");

                let r_val: u16 = value_storage[&'r'];
                log::trace!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                let b_val: u16 = value_storage[&'b'];
                log::trace!("b: {b_val:#b} {b_val:#x} {b_val}\n");

                log::trace!("[SBRC]\n");

                os.push_str(&format!("sbrc r{} bit{}", r_val, b_val));
                log::info!("{}\n", os);

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                log::info!("{} {:#04x} {:#10b}\n", data, data, data);

                if (data & (1u8 << b_val as u8)) == 0u8
                {
                    // this will skip the next instruction
                    cpu.pc += 2i32;
                }

                cpu.pc += 2i32;
            }

            /* 105 */
            // 105. SBRS – Skip if Bit in Register is Set
            // 1111 111r rrrr 0bbb
            //
            // 1111 01kk kkkk k001 - BRNE - branch if not equal
            InstructionType::SBRS => {

                log::trace!("[SBRS]\n");

                let r_val: u16 = value_storage[&'r'];
                log::trace!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                let b_val: u16 = value_storage[&'b'];
                log::trace!("b: {b_val:#b} {b_val:#x} {b_val}\n");

                log::trace!("[SBRS]\n");

                os.push_str(&format!("sbrs r{} bit{}", r_val, b_val));
                log::info!("{}\n", os);

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                if (data & (1u8 << b_val as u8)) > 0u8
                {
                    // this will skip the next instruction
                    cpu.pc += 2i32;
                }

                cpu.pc += 2i32;
            }

            /* 118 */
            // store data into memory (data space) at the address stored in X
            // (data space != I/O space (for I/O space, use the OUT instruction))
            InstructionType::ST_STD_X_1 => {

                // Z: Post incremented
                log::trace!("[ST_STD_X_1]\n");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                // retrieve the data (address) stored inside X
                let value_x: u16 = cpu.get_x();
                log::info!("X: {value_x:#b} {value_x:#x} {value_x}\n");

                // store data into memory (data space) at the address stored in X
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_x as usize, data);

                cpu.pc += 2i32;
            }
            InstructionType::ST_STD_X_2 => {

                // X: Post incremented
                log::trace!("[ST_STD_X_2]\n");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::trace!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                os.push_str(&format!("st X+ r{}", r_val));
                log::info!("{}\n", os);

                // retrieve the data (address) stored inside X
                let mut value_x: u16 = cpu.get_x();
                log::trace!("X: {value_x:#b} {value_x:#x} {value_x}\n");

                // store data into memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_x as usize, data);

                // (post) increment X
                value_x = value_x + 1;

                // write back X
                cpu.set_x(value_x);

                cpu.pc += 2i32;
            }
            InstructionType::ST_STD_X_3 => {

                // X: Post incremented
                log::trace!("[ST_STD_X_3]\n");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                // retrieve the data (address) stored inside X
                let mut value_x: u16 = cpu.get_x();
                log::info!("X: {value_x:#b} {value_x:#x} {value_x}\n");

                // (pre) decrement X
                value_x = value_x - 1;

                // store data into memory (data space) at the address stored in X
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_x as usize, data);

                // write back X
                cpu.set_x(value_x);

                cpu.pc += 2i32;
            }

            /* 119 */
            // store data into memory (data space) at the address stored in Y
            // (data space != I/O space (for I/O space, use the OUT instruction))
            InstructionType::ST_STD_Y_1 => {

                // Y: Post incremented
                log::trace!("[ST_STD_Y_1]\n");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                // retrieve the data (address) stored inside Y
                let value_y: u16 = cpu.get_y();
                log::info!("Y: {value_y:#b} {value_y:#x} {value_y}\n");

                // store data into memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_y as usize, data);

                cpu.pc += 2i32;
            }
            InstructionType::ST_STD_Y_2 => {

                // Y: Post incremented
                log::trace!("[ST_STD_Y_2]\n");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                // retrieve the data (address) stored inside Y
                let mut value_y: u16 = cpu.get_y();
                log::info!("Y: {value_y:#b} {value_y:#x} {value_y}\n");

                // store data into memory (data space) at the address stored in Y
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_y as usize, data);

                // (post) increment Y
                value_y = value_y + 1;

                // write back Y
                cpu.set_y(value_y);

                cpu.pc += 2i32;
            }
            InstructionType::ST_STD_Y_3 => {

                // Y: Post incremented
                log::trace!("[ST_STD_Y_3]\n");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                // retrieve the data (address) stored inside Y
                let mut value_y: u16 = cpu.get_y();
                log::info!("Y: {value_y:#b} {value_y:#x} {value_y}\n");

                // (pre) decrement Y
                value_y = value_y - 1;

                // store data into memory (data space) at the address stored in Y
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_y as usize, data);

                // write back Y
                cpu.set_y(value_y);

                cpu.pc += 2i32;
            }

            /* 120 */
            // store data into memory (data space) at the address stored in Z
            // (data space != I/O space (for I/O space, use the OUT instruction))
            InstructionType::ST_STD_Z_1 => {

                // Z: Post incremented
                log::trace!("[ST_STD_Z_1]\n");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::trace!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // log::info!("st Z, r{}\n", r_val);
                os.push_str(&format!("st Z, r{}", r_val));
                log::info!("{}\n", os);

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                // retrieve the data (address) stored inside Z
                let value_z: u16 = cpu.get_z();

                // store data into memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_z as usize, data);

                cpu.pc += 2i32;
            }
            InstructionType::ST_STD_Z_2 => {

                // Z: Post incremented
                log::trace!("[ST_STD_Z_2]\n");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                // retrieve the data (address) stored inside Z
                let mut value_z: u16 = cpu.get_z();

                // store data into memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_z as usize, data);

                // (post) increment Z
                value_z = value_z + 1;

                // write back Z
                cpu.set_z(value_z);

                cpu.pc += 2i32;
            }
            InstructionType::ST_STD_Z_3 => {

                // Z: Post incremented
                log::trace!("[ST_STD_Z_3]\n");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}\n");

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                // retrieve the data (address) stored inside Z
                let mut value_z: u16 = cpu.get_z();

                // (pre) decrement Z
                value_z = value_z - 1;

                // store data into memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_z as usize, data);

                // write back Z
                cpu.set_z(value_z);

                cpu.pc += 2i32;
            }

            /* 120 */
            // Stores one byte from a Register to the data space
            // 1001 001d dddd 0000
            // kkkk kkkk kkkk kkkk
            InstructionType::STS => {
                log::trace!("[STS] 32 bit version\n");

                log::info!("value_storage: {:?}\n", value_storage);

                // retrieved the encoded value for the r register
                let register: u16 = value_storage[&'d'];
                log::info!("d: {register:#b} {register:#x} {register}\n");

                cpu.pc += 2i32;

                // read the address from the next two bytes in memory
                let address: u16 = cpu.read_next_two_byte(segment);

                os.push_str(&format!("sts addr:{:#06x}, r{}", address, register));
                log::info!("{}\n", os);

                // retrieve the current value from the r register
                let register_value: u8 = cpu.register_file[register as usize];

                cpu.store_to_data_space(address as usize, register_value);

                cpu.pc += 2i32;
            }

            // 124. SUBI – Subtract Immediate
            // Rd ← Rd - K
            // Subtracts a register and a constant, and places the result in the destination register Rd. This instruction is
            // working on Register R16 to R31 and is very well suited for operations on the X, Y, and Z-pointers.
            InstructionType::SUBI => {
                log::trace!("[SUBI]\n");

                let d_val: u16 = value_storage[&'d'];
                let k: i16 = value_storage[&'K'] as i16;

                //log::trace!("Clearing register d: {:#06x}\n", d);

                //log::info!("clr {}\n", d);
                os.push_str(&format!("cpi r{}, {}", d_val, k));
                log::info!("{}\n", os);

                // To compute the register to use, add the offset 16 to the parsed value
                let register: u16 = d_val + 16u16;

                let register_value: i16 = cpu.register_file[register as usize] as i16;

                let temp_result: i16 = register_value - k;

                cpu.register_file[register as usize] = temp_result as u8;

                // todo compute Rd - K as specified in the datasheet and set all other bits correctly
                if temp_result == 0x00
                {
                    cpu.z = true;
                }

                // cpu.register_file[d as usize] = 0x00;

                // // updating flags
                // cpu.s = false;
                // cpu.v = false;
                // cpu.n = false;
                // cpu.z = true;

                cpu.pc += 2i32;
            }

            InstructionType::UNKNOWN => {
                panic!("Unknown instruction: {:?}\n", instruction.instruction_type);
            }

            _ => {
                panic!("Unknown instruction: {:?}\n", instruction.instruction_type);
            }
        }

    }
}

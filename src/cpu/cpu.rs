use std::collections::HashMap;

use crate::HASHMAP;
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
    increment_stack_pointer(cpu);
    let stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    let data: u8 = cpu.sram[(stack_pointer - 1) as usize];
    data
}

fn increment_stack_pointer(cpu: &mut CPU) {
    let mut stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    stack_pointer += 1u16;
    *cpu.sph() = HIGH!(stack_pointer).try_into().unwrap();
    *cpu.spl() = LOW!(stack_pointer).try_into().unwrap();
}

fn decrement_stack_pointer(cpu: &mut CPU) {
    let mut stack_pointer: u16 = ((*cpu.sph() as u16) << 8u16) | *cpu.spl() as u16;
    stack_pointer -= 1u16;
    *cpu.sph() = HIGH!(stack_pointer).try_into().unwrap();
    *cpu.spl() = LOW!(stack_pointer).try_into().unwrap();
}

pub struct CPU {

    // the zero flag
    pub z: bool,

    // the carry flag
    pub c: bool,

    // the half-carry flag
    pub h: bool,

    // pc always points to the instruction after the current instruction so it does not start at 0x00 but at 0x02
    pub pc: i32,

    // register file, 32 8bit registers
    pub register_file: [u8; 32usize],

    pub sram: [u8; RAMEND as usize],

    // special function register
    pub sfr: [u8; 255usize],

}

impl Default for CPU {
    fn default() -> Self {
        Self { 
            z: false,
            c: false,
            h: false,
            pc: 0x02i32,
            register_file: [0; 32usize],
            sram: [0; RAMEND as usize],
            sfr: [0; 255usize] 
        }
    }
}

impl CPU {

    #[allow(dead_code, unused)]
    pub fn new(z: bool, c: bool, pc: i32, register_file: [u8; 32usize], sram: [u8; RAMEND as usize], sfr: [u8; 255usize]) -> Self {
        Self { 
            z: z,
            c: c,
            h: false,
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

        let map = HASHMAP.lock().unwrap();
        let value_as_string = map.get("SPH").unwrap();

        let without_prefix = value_as_string.trim_start_matches("0x");
        let value: usize = usize::from_str_radix(without_prefix, 16).unwrap();

        return &mut self.sfr[value];
    }

    fn spl(&mut self) -> &mut u8 {
        
        let map = HASHMAP.lock().unwrap();
        let value_as_string = map.get("SPL").unwrap();

        let without_prefix = value_as_string.trim_start_matches("0x");
        let value: usize = usize::from_str_radix(without_prefix, 16).unwrap();

        return &mut self.sfr[value];
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

    fn reset_c(&mut self) {
        self.c = false;
    }

    fn get_c(&mut self) -> bool {
        self.c
    }

    fn set_h(&mut self) {
        self.h = true;
    }

    fn reset_h(&mut self) {
        self.h = false;
    }

    fn get_h(&mut self) -> bool {
        self.h
    }

    fn get_z(&mut self) -> u16 {
        ((self.register_file[30] as u16) << 8u16) | self.register_file[31] as u16
    }

    fn set_z(&mut self, value: u16) {
        self.register_file[30] = HIGH_U16!(value);
        self.register_file[31] = LOW_U16!(value);
    }

    // data_space is terminology used in the datasheet of the ATmega328p for SRAM memory access
    // as opposed to I/O space which is terminology for the memory mapped special function registers
    fn store_to_data_space(&mut self, address: usize, value: u8) {
        //todo!("Store value {} to address {} in data space!", value, address);
        log::info!("Store value {} to address {} in data space!", value, address);
    }

    fn store_to_i_o_space(&mut self, address: usize, value: u8) {
        // output the value stored in register r_val into memory to the address a_val
        self.sfr[address] = value;
    }

    pub fn execute_instruction(&mut self, segment: &Segment) {

        // get the current instruction
        let temp_pc: i32 = self.pc - 0x02;

        // check for end of code
        if segment.size <= temp_pc as u32 {
            log::info!("End of Code reached! Application Finished!");
            return;
        }

        // assemble the instruction word
        let word_hi: u16 = segment.data[(temp_pc + 1i32) as usize].into();
        let word_lo: u16 = segment.data[temp_pc as usize].into();
        let word: u16 = (word_hi << 8u8) + word_lo;

        // decode the current instruction
        let mut value_storage: HashMap<char, u16> = HashMap::new();
        let instruction: &InstructionDefinition =
            decode_instruction(word, INSTRUCTIONS, &UNKNOWN, &mut value_storage);

        let mut cpu: &mut CPU = self;

        // DEBUG
        log::info!("Executing instruction: {:?}", instruction.instruction_type);

        // execute the instruction
        match instruction.instruction_type {

            /*   5 */
            InstructionType::ADC => {
                log::info!("[ADC]");

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
                log::trace!(
                    "[ADC] result value: {}",
                    cpu.register_file[d_value]
                );

                log::info!("adc r{}, d{} - carry{}", r_value, d_value, cpu.c);

                cpu.pc += 2i32;
            }
            
            /*   6 */
            InstructionType::ADD => {
                log::info!("[ADD]");

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
                    "[ADD] result value: {}",
                    cpu.register_file[d_value]
                );

                log::info!("add r{}, d{}", r_value, d_value);

                cpu.pc += 2i32;
            }

            /*  13 */
            InstructionType::BRBC => {
                log::info!("[BRBC]");

                //process_brbc(rdr, &word, index, value_storage);

                let k_val: i8 = value_storage[&'k'] as i8;
                //log::info!("K: {k_val:#b} {k_val:#x}");
                let s_val = value_storage[&'s'];
                //log::info!("d: {d_val:#b} {d_val:#x}");

                // twos-complement
                let offset: i8 = (k_val as i8 * -1i8) as i8;

                // todo find a better way to deal with the SREG array of status bits
                if s_val == 1 {
                    // zero flag

                    // same code as BRNE

                    // check the Z-flag
                    if cpu.z {
                        cpu.pc += 2i32;
                    } else {
                        let offset_in_bytes: i8 = offset / 8i8;
                        cpu.pc = (cpu.pc as i16 + offset_in_bytes as i16) as i32;
                    }
                }
            }

            /*  27 */
            InstructionType::BRNE => {
                log::info!("[BRNE]");

                let offset: i32 = value_storage[&'k'] as i32;

                // check the Z-flag
                if cpu.z {
                    cpu.pc += offset;
                } else {
                    cpu.pc += 2i32;
                }
            }

            /* 36 */
            InstructionType::CALL => {
                log::trace!("[CALL]");

                // get the first 16 bit
                let k_hi: u32 = value_storage[&'k'].into();

                // get the next 16 stored at the pc since the JMP command is encoded using 32 bit (4 byte)
                let temp_pc: i32 = cpu.pc;

                let word_hi: u16 = segment.data[(temp_pc + 1i32) as usize] as u16;
                log::trace!("READ: {:#02x}", word_hi as u8);

                let word_lo: u16 = segment.data[temp_pc as usize] as u16;
                log::trace!("READ: {:#02x}", word_lo as u8);

                let k_lo: u32 = ((word_hi << 8u8) + word_lo).into();

                // assemble the parameter k
                let k_val: i16 = ((k_hi << 16u8) + k_lo) as i16;

                // extend to i32
                let k_val_i32: i32 = k_val as i32;

                // push return address onto the stack
                let data = cpu.pc;
                push_to_stack_i16(&mut cpu, data as i16);

                //log::info!("call - stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                log::info!("stack pointer: {} {}", cpu.stack_info_high(), cpu.stack_info_low());

                // jump to address
                cpu.pc += k_val_i32;
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
            InstructionType::CLR => {
                log::info!("[CLR]");

                let d: u16 = value_storage[&'d'];

                log::trace!("Clearing register d: {:#06x}", d);
                cpu.register_file[d as usize] = 0x00;

                cpu.pc += 2i32;
            }

            /*  53 */
            InstructionType::DEC => {
                log::info!("[DEC]");

                let d: u16 = value_storage[&'d'];

                // perform the decrement
                cpu.register_file[d as usize] -= 0x01u8;

                // set the z flag
                if cpu.register_file[d as usize] == 0x00u8 {
                    cpu.z = true;
                }

                log::info!(
                    "[DEC] Register r{}: value at reg:{:#06x}",
                    d,
                    cpu.register_file[d as usize]
                );

                cpu.pc += 2i32;
            }

            /*  64 */
            InstructionType::IN => {
                log::info!("[IN]");

                let register_d: u16 = value_storage[&'d'];
                let address: u16 = value_storage[&'A'];

                let val: u8 = cpu.sram[address as usize];

                log::info!("[IN] value from read-operation: {:#06x}", val);

                cpu.register_file[register_d as usize] = val as u8;

                log::info!(
                    "[IN] Register r{}: value at reg:{:#06x}",
                    register_d,
                    cpu.register_file[register_d as usize]
                );

                cpu.pc += 2i32;
            }

            /*  65 */
            InstructionType::INC => {
                log::info!("[INC]");

                let register_d: u16 = value_storage[&'d'];

                log::info!(
                    "[INC] Before Inc: Register r{}: value at reg:{:#06x}",
                    register_d,
                    cpu.register_file[register_d as usize]
                );

                let mut val: u8 = cpu.register_file[register_d as usize];
                val = val + 1;
                cpu.register_file[register_d as usize] = val as u8;

                log::info!(
                    "[INC] Before Inc: Register r{}: value at reg:{:#06x}",
                    register_d,
                    cpu.register_file[register_d as usize]
                );

                cpu.pc += 2i32;
            }

            /*  66 */
            InstructionType::JMP => {
                log::info!("[JMP]");

                // get the first 16 bit
                let k_hi: u32 = value_storage[&'k'].into();

                // get the next 16 stored at the pc since the JMP command is encoded using 32 bit (4 byte)
                let temp_pc: i32 = cpu.pc;

                let word_hi: u16 = segment.data[(temp_pc + 1i32) as usize] as u16;
                //log::info!("READ: {:#02x}", word_hi as u8);

                let word_lo: u16 = segment.data[temp_pc as usize] as u16;
                //log::info!("READ: {:#02x}", word_lo as u8);

                let k_lo: u32 = ((word_hi << 8u8) + word_lo).into();

                // assemble the parameter k
                let k_val: i16 = ((k_hi << 16u8) + k_lo) as i16;

                // extend to i32
                let k_val_i32: i32 = k_val as i32;

                cpu.pc += k_val_i32;
            }

            /*  73 */
            InstructionType::LDI => {
                log::info!("[LDI]");

                let k_val = value_storage[&'K'];
                log::info!("K: {k_val:#b} {k_val:#x}");
                let d_val = value_storage[&'d'];
                log::info!("d: {d_val:#b} {d_val:#x}");

                // "Loads an 8-bit constant directly to register 16 to 31."
                // To compute the register to use, add the offset 16 to the parsed value
                let register = d_val + 16;
                //log::info!("[LDI] Using register: r{}", register);

                log::trace!("{temp_pc:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}");
                log::info!("ldi r{register:#02}, {k_val:#02x}");

                // execute
                cpu.register_file[register as usize] = k_val as u8;

                cpu.pc += 2i32;
            }

            /*  79 */
            InstructionType::MOV => {
                log::info!("[MOV]");

                let r_register = value_storage[&'r'];
                log::trace!("K: {r_register:#b} {r_register:#x}");
                let d_register = value_storage[&'d'];
                log::trace!("d: {d_register:#b} {d_register:#x}");

                let k_val: u8 = cpu.register_file[r_register as usize];

                cpu.register_file[d_register as usize] = k_val as u8;

                cpu.pc += 2i32;
            }

            /*  85 */
            InstructionType::NOP => {
                log::trace!("[NOP]");
                cpu.pc += 2i32;
            }

            /*  88 */
            InstructionType::OUT => {

                log::trace!("[OUT]");

                // Stores data from register Rr in the Register File to I/O Space (Ports, Timers, Configuration Registers, etc.).
                let a_val: u16 = value_storage[&'A'];
                log::trace!("A: {a_val:#b} {a_val:#x} {a_val}");
                let r_val = value_storage[&'r'];
                log::trace!("r: {r_val:#b} {r_val:#x} {r_val}");

                // take value out of the register
                log::trace!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                let val: u8 = cpu.register_file[r_val as usize];

                // DEBUG
                log::info!("out {:?}, {:?}", a_val, val);

                // output the value stored in register r_val into memory to the address a_val
                cpu.store_to_i_o_space(a_val as usize, val);

                cpu.pc += 2i32;
            }

            /*  89 */
            InstructionType::POP => {
                log::info!("[POP]");

                let d_val: u16 = value_storage[&'d'];
                log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                cpu.pc += 2i32;

                pop_from_stack_u8(&mut cpu);

                log::info!("stack pointer: {} {}", cpu.stack_info_high(), cpu.stack_info_low());
            }

            /*  90 */
            InstructionType::PUSH => {
                log::info!("[PUSH]");

                let d_val: u16 = value_storage[&'d'];
                log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                cpu.pc += 2i32;

                let data = cpu.register_file[d_val as usize];
                push_to_stack_u8(&mut cpu, data);

                log::info!("stack pointer: {} {}", cpu.stack_info_high(), cpu.stack_info_low());
            }

            /* 91 */
            InstructionType::RCALL => {
                log::info!("[RCALL]");

                // get the first 16 bit
                let mut k: u16 = value_storage[&'k'] as u16;

                log::info!("k: {:04x} {:016b}", k, k);

                // sign extend (800 decimal = 1000 0000 0000 binary)
                if k >= 800 {
                    k |= 0xF000;
                }
                log::info!("k: {:04x} {:016b} {}", k as i16, k as i16, k as i16);

                let kk: i16 = k as i16;

                log::info!("kk: {:04x} {:016b} {}", kk, kk, kk);

                // push return address onto the stack
                let data: i32 = cpu.pc;
                push_to_stack_i16(&mut cpu, data as i16);

                log::info!("stack pointer: {} {}", cpu.stack_info_high(), cpu.stack_info_low());

                // jump to address
                cpu.pc += kk as i32;
            }

            /* 92 */
            InstructionType::RET => {
                log::trace!("[RET]");

                // let d_val: u16 = value_storage[&'d'];
                // log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                // cpu.pc += 2i32;

                // let data = cpu.register_file[d_val as usize];
                let k_hi: u16 = pop_from_stack_u8(&mut cpu) as u16;
                let k_lo: u16 = pop_from_stack_u8(&mut cpu) as u16;

                let k_val: i16 = ((k_hi << 8i16) + k_lo) as i16;

                cpu.pc = k_val as i32;

                //log::info!("ret - stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                log::info!("stack pointer: {} {}", cpu.stack_info_high(), cpu.stack_info_low());
            }

            /*  94 */
            InstructionType::RJMP => {
                log::info!("[RJMP]");

                // get the first 16 bit
                let mut k: u16 = value_storage[&'k'] as u16;
                log::info!("k: {:04x} {:016b}", k, k);

                // sign extend (800 decimal = 1000 0000 0000 binary)
                if k >= 800 {
                    k |= 0xF000;
                }
                log::info!("k: {:04x} {:016b} {}", k as i16, k as i16, k as i16);

                let kk: i16 = k as i16;
                log::info!("kk: {:04x} {:016b} {}", kk, kk, kk);

                cpu.pc += kk as i32;
            }

            /* 120 */
            // store data into memory (data space) at the address stored in Z
            // (data space != I/O space (for I/O space, use the OUT instruction))
            InstructionType::ST_STD_Z_1 => {

                // Z: Post incremented
                log::info!("[ST_STD_Z_1]");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

                // retrieve the current value from the r register
                let data: u8 = cpu.register_file[r_val as usize];

                // retrieve the data (address) stored inside Z
                let value_z: u16 = cpu.get_z();

                // store data into memory (data space) at the address stored in Z
                // (data space != I/O space (for I/O space, use the OUT instruction))
                cpu.store_to_data_space(value_z as usize, data);

                // // (post) increment Z
                // value_z = value_z + 1;

                // // write back Z
                // cpu.set_z(value_z);

                cpu.pc += 2i32;
            }
            InstructionType::ST_STD_Z_2 => {

                // Z: Post incremented
                log::info!("[ST_STD_Z_2]");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

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
                log::info!("[ST_STD_Z_3]");

                // retrieved the encoded value for the r register
                let r_val: u16 = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

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

            InstructionType::UNKNOWN => {
                panic!("Unknown instruction: {:?}", instruction.instruction_type);
            }

            _ => {
                panic!("Unknown instruction: {:?}", instruction.instruction_type);
            }
        }
        
    }
}

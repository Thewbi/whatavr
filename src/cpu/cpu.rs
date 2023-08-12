use std::collections::HashMap;

use crate::HASHMAP;
use crate::assembler::io_destination::IoDestination;
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

    // the z flag
    pub z: bool,

    // // stack pointer high, low
    // pub sph: u8,
    sph: u8,
    // pub spl: u8,
    spl: u8,

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
        Self { z: false,
            sph: 0x00u8,
            spl: 0x00u8,
            pc: 0x02i32,
            register_file: [0; 32usize],
            sram: [0; RAMEND as usize],
            sfr: [0; 255usize] 
        }
    }
}

impl CPU {

    pub fn new(z: bool, pc: i32, register_file: [u8; 32usize], sram: [u8; RAMEND as usize], sfr: [u8; 255usize]) -> Self {
        Self { 
            z: z,
            sph: 0x00u8,
            spl: 0x00u8,
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

        // execute the instruction
        match instruction.instruction_type {
            
            /*   6 */
            InstructionType::ADD => {
                log::trace!("[ADD]");

                let r_value16 = value_storage[&'r'];
                let d_value16 = value_storage[&'d'];

                cpu.register_file[d_value16 as usize] += cpu.register_file[r_value16 as usize];

                log::trace!(
                    "[ADD] result value: {}",
                    cpu.register_file[d_value16 as usize]
                );

                log::info!("add r{}, d{}", r_value16, d_value16);

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
                        cpu.pc = (cpu.pc as i16 + offset as i16) as i32;
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

                // sign extend to i32
                let k_val_i32: i32 = k_val as i32;

                // push return address onto the stack
                let data = cpu.pc;
                push_to_stack_i16(&mut cpu, data as i16);

                log::info!("call - stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);

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

                // sign extend to i32
                let k_val_i32: i32 = k_val as i32;

                cpu.pc += k_val_i32;
            }

            /*  73 */
            InstructionType::LDI => {
                log::trace!("[LDI]");

                let k_val = value_storage[&'K'];
                log::trace!("K: {k_val:#b} {k_val:#x}");
                let d_val = value_storage[&'d'];
                log::trace!("d: {d_val:#b} {d_val:#x}");

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

                cpu.sfr[a_val as usize] = val;

                log::info!("out {:?}, {:?}", a_val, val);

                // // TODO output the value stored in register r_val into memory to the address a_val
                // let dest: IoDestination = IoDestination::from_code(a_val);
                // log::trace!("[OUT] dest: {:?} source-register: {:?}", dest, r_val);
                // log::trace!("out dest: {:?} src: {:?}", dest, r_val);

                // match dest {

                //     IoDestination::SPH => {

                //         // take value out of the register
                //         log::trace!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                //         let val: u8 = cpu.register_file[r_val as usize];

                //         // write value into the destination
                //         log::trace!("val: {val:#b} {val:#x} {val}");
                //         cpu.sph = val;

                //         log::trace!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                //     }

                //     IoDestination::SPL => {
                //         //log::info!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                //         let val: u8 = cpu.register_file[r_val as usize];
                //         //log::info!("val: {val:#b} {val:#x} {val}");
                //         cpu.spl = val;

                //         log::trace!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                //     }

                //     IoDestination::DDRB | IoDestination::PORTB | IoDestination::PINB => {
                //         log::trace!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                //         let val: u8 = cpu.register_file[r_val as usize];
                //         log::trace!("val: {val:#b} {val:#x} {val}");

                //         // write the value into SRAM
                //         cpu.sram[a_val as usize] = val;
                //     }

                //     IoDestination::DDRC | IoDestination::PORTC | IoDestination::PINC => {
                //         log::trace!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                //         let val: u8 = cpu.register_file[r_val as usize];
                //         log::trace!("val: {val:#b} {val:#x} {val}");

                //         // write the value into SRAM
                //         cpu.sram[a_val as usize] = val;
                //     }

                //     IoDestination::UNKNOWN => {
                //         panic!("UNKNOWN enum value!")
                //     } // _ => panic!("Unknown enum value!")
                // }

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
                //cpu.stack_info();
            }

            /*  90 */
            InstructionType::PUSH => {
                log::info!("[PUSH]");

                let d_val: u16 = value_storage[&'d'];
                log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                cpu.pc += 2i32;

                let data = cpu.register_file[d_val as usize];
                push_to_stack_u8(&mut cpu, data);

                //log::info!("stack pointer: {:#04x} {:#04x}", *cpu.sph(), *cpu.spl());
                log::info!("stack pointer: {} {}", cpu.stack_info_high(), cpu.stack_info_low());
            }

            /* 91 */
            InstructionType::RCALL => {
                log::info!("[RCALL]");

                // get the first 16 bit
                let mut k: u16 = value_storage[&'k'] as u16;

                log::info!("k: {:04x} {:016b}", k, k);

                // sign extend
                k |= 0xF000;

                log::info!("k: {:04x} {:016b} {}", k as i16, k as i16, k as i16);

                let kk: i16 = k as i16;

                log::info!("kk: {:04x} {:016b} {}", kk, kk, kk);

                //cpu.pc += kk as i32;

                // push return address onto the stack
                let data: i32 = cpu.pc;
                push_to_stack_i16(&mut cpu, data as i16);

                log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);

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

                log::info!("ret - stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
            }

            /*  94 */
            InstructionType::RJMP => {
                log::info!("[RJMP]");

                // get the first 16 bit
                let mut k: u16 = value_storage[&'k'] as u16;

                log::info!("k: {:04x} {:016b}", k, k);

                // sign extend
                k |= 0xF000;

                log::info!("k: {:04x} {:016b} {}", k as i16, k as i16, k as i16);

                let kk: i16 = k as i16;

                log::info!("kk: {:04x} {:016b} {}", kk, kk, kk);

                cpu.pc += kk as i32;
            }

            InstructionType::UNKNOWN => {
                panic!("Unknown instruction!");
            }

            _ => {
                panic!("Unknown instruction!");
            }
        }
        
    }
}

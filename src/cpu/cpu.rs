
use std::collections::HashMap;

use crate::{HIGH, LOW, HIGH_HIGH_I32, HIGH_I32, LOW_I32, LOW_LOW_I32, LOW_I16, HIGH_I16};
use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::assembler::io_destination::IoDestination;
use crate::instructions::{instruction_definition::InstructionDefinition, instruction_type::InstructionType, decode::decode_instruction};
use crate::instructions::instructions::INSTRUCTIONS;
use crate::instructions::instructions::UNKNOWN;

pub const RAMEND: u16 = 0x08ff;


// the value is placed at the stackpointer then, after that, the stack pointer is decremented
pub fn push_to_stack_u8(cpu: &mut CPU, data: u8) 
{
    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = data;

    decrement_stack_pointer(cpu);
}

#[allow(dead_code)]
pub fn push_to_stack_u16(cpu: &mut CPU, data: &u16) 
{
    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);
}

pub fn push_to_stack_i16(cpu: &mut CPU, data: i16) 
{
    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH_I16!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW_I16!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);
}

#[allow(dead_code)]
pub fn push_to_stack_i32(cpu: &mut CPU, data: i32) 
{
    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH_HIGH_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = HIGH_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    cpu.sram[(stack_pointer - 1) as usize] = LOW_LOW_I32!(data).try_into().unwrap();
    decrement_stack_pointer(cpu);
}

pub fn pop_from_stack_u8(cpu: &mut CPU) -> u8 {

    increment_stack_pointer(cpu);

    let stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    let data: u8 = cpu.sram[(stack_pointer - 1) as usize];

    data
}

fn increment_stack_pointer(cpu: &mut CPU)
{
    let mut stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    stack_pointer += 1u16;
    cpu.sph = HIGH!(stack_pointer).try_into().unwrap();
    cpu.spl = LOW!(stack_pointer).try_into().unwrap();
}

fn decrement_stack_pointer(cpu: &mut CPU)
{
    let mut stack_pointer: u16 = ((cpu.sph as u16) << 8u16) | cpu.spl as u16;
    stack_pointer -= 1u16;
    cpu.sph = HIGH!(stack_pointer).try_into().unwrap();
    cpu.spl = LOW!(stack_pointer).try_into().unwrap();
}

pub struct CPU {
    
    // the z flag
    pub z: bool,

    // stack pointer high, low
    pub sph: u8,
    pub spl: u8,

    // pc always points to the instruction after the current instruction so it does not start at 0x00 but at 0x02
    pub pc: i32,

    // register file, 32 8bit registers
    pub register_file: [u8; 32usize],

    pub sram: [u8; RAMEND as usize],

}

impl CPU {

    pub fn execute_instruction(&mut self, segment:&Segment)
    {
        // get the current instruction
        let temp_pc:i32 = self.pc - 0x02;

        // check for end of code
        if segment.size <= temp_pc as u32 {
            log::info!("End of Code reached! Application Finished!");
            return
        }

        // assemble the instruction word
        let word_hi:u16 = segment.data[(temp_pc + 1i32) as usize].into();
        let word_lo:u16 = segment.data[temp_pc as usize].into();
        let word:u16 = (word_hi << 8u8) + word_lo;

        // decode the current instruction
        let mut value_storage:HashMap<char, u16> = HashMap::new();
        let instruction: &InstructionDefinition = decode_instruction(word, 
            INSTRUCTIONS, &UNKNOWN, &mut value_storage);

        let mut cpu = self;

        // execute the instruction
        match instruction.instruction_type {

            /*   6 */ 
            InstructionType::ADD => { 
                log::info!("[ADD]");

                let r_valu16 = value_storage[&'r'];
                let d_valu16 = value_storage[&'d'];

                cpu.register_file[d_valu16 as usize] += cpu.register_file[r_valu16 as usize];

                log::info!("[ADD] result value: {}", cpu.register_file[d_valu16 as usize]);

                cpu.pc += 2i32;
            },

            /*  13 */ 
            InstructionType::BRBC => { 
                log::info!("[BRBC]");

                //process_brbc(rdr, &word, index, value_storage);

                let k_val:i8 = value_storage[&'k'] as i8;
                //log::info!("K: {k_val:#b} {k_val:#x}");
                let s_val = value_storage[&'s'];
                //log::info!("d: {d_val:#b} {d_val:#x}");

                // twos-complement
                let offset:i8 = (k_val as i8 * -1i8) as i8;

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
            },

            /*  27 */ 
            InstructionType::BRNE => { 
                log::info!("[BRNE]");

                let offset:i32 = value_storage[&'k'] as i32;

                // check the Z-flag
                if cpu.z {
                    cpu.pc += offset;
                } else {
                    cpu.pc += 2i32;
                }
            },

            /* 36 */ 
            InstructionType::CALL => {

                log::info!("[CALL]");

                // get the first 16 bit
                let k_hi:u32 = value_storage[&'k'].into();

                // get the next 16 stored at the pc since the JMP command is encoded using 32 bit (4 byte)
                let temp_pc:i32 = cpu.pc;

                let word_hi:u16 = segment.data[(temp_pc + 1i32) as usize] as u16;
                //log::info!("READ: {:#02x}", word_hi as u8);

                let word_lo:u16 = segment.data[temp_pc as usize] as u16;
                //log::info!("READ: {:#02x}", word_lo as u8);

                let k_lo:u32 = ((word_hi << 8u8) + word_lo).into();

                // assemble the parameter k
                let k_val:i16 = ((k_hi << 16u8) + k_lo) as i16;

                // sign extend to i32
                let k_val_i32:i32 = k_val as i32;

                // push return address onto the stack 
                let data = cpu.pc;
                push_to_stack_i16(&mut cpu, data as i16);

                log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                
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

                let d:u16 = value_storage[&'d'];
                
                //log::info!("Clearing register d: {:#06x}", d);
                cpu.register_file[d as usize] = 0x00;

                cpu.pc += 2i32;
            },

            /*  53 */ 
            InstructionType::DEC => { 
                log::info!("[DEC]");

                let d:u16 = value_storage[&'d'];
                
                // perform the decrement
                cpu.register_file[d as usize] -= 0x01u8;

                // set the z flag
                if cpu.register_file[d as usize] == 0x00u8 {
                    cpu.z = true;
                }

                log::info!("[DEC] Register r{}: value at reg:{:#06x}", d, cpu.register_file[d as usize]);

                cpu.pc += 2i32;
            },

            /*  64 */ 
            InstructionType::IN => { 
                log::info!("[IN]");

                let register_d:u16 = value_storage[&'d'];
                let address:u16 = value_storage[&'A'];
                
                let val: u8 = cpu.sram[address as usize];

                log::info!("[IN] value from read-operation: {:#06x}", val);

                cpu.register_file[register_d as usize] = val as u8;

                log::info!("[IN] Register r{}: value at reg:{:#06x}", register_d, cpu.register_file[register_d as usize]);

                cpu.pc += 2i32;
            },
        
            /*  66 */ 
            InstructionType::JMP => {

                log::info!("[JMP]");

                // get the first 16 bit
                let k_hi:u32 = value_storage[&'k'].into();

                // get the next 16 stored at the pc since the JMP command is encoded using 32 bit (4 byte)
                let temp_pc:i32 = cpu.pc;

                let word_hi:u16 = segment.data[(temp_pc + 1i32) as usize] as u16;
                //log::info!("READ: {:#02x}", word_hi as u8);

                let word_lo:u16 = segment.data[temp_pc as usize] as u16;
                //log::info!("READ: {:#02x}", word_lo as u8);

                let k_lo:u32 = ((word_hi << 8u8) + word_lo).into();

                // assemble the parameter k
                let k_val:i16 = ((k_hi << 16u8) + k_lo) as i16;

                // sign extend to i32
                let k_val_i32:i32 = k_val as i32;
                
                cpu.pc += k_val_i32;
            },

            /*  73 */ 
            InstructionType::LDI => { 
                log::info!("[LDI]");

                let k_val = value_storage[&'K'];
                //log::info!("K: {k_val:#b} {k_val:#x}");
                let d_val = value_storage[&'d'];
                //log::info!("d: {d_val:#b} {d_val:#x}");

                // "Loads an 8-bit constant directly to register 16 to 31."
                // To compute the register to use, add the offset 16 to the parsed value
                let register = d_val + 16;
                //log::info!("[LDI] Using register: r{}", register);

                log::info!("{temp_pc:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}");

                // execute
                cpu.register_file[register as usize] = k_val as u8;

                cpu.pc += 2i32;
            },

            /*  79 */
            InstructionType::MOV => {
                log::info!("[MOV]");

                let r_register = value_storage[&'r'];
                //log::info!("K: {r_register:#b} {r_register:#x}");
                let d_register = value_storage[&'d'];
                //log::info!("d: {d_register:#b} {d_register:#x}");

                let k_val: u8 = cpu.register_file[r_register as usize];

                cpu.register_file[d_register as usize] = k_val as u8;

                cpu.pc += 2i32;
            }

            /*  88 */ 
            InstructionType::OUT => {
                log::info!("[OUT]");

                // Stores data from register Rr in the Register File to I/O Space (Ports, Timers, Configuration Registers, etc.).
                let a_val: u16 = value_storage[&'A'];
                log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
                let r_val = value_storage[&'r'];
                log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

                // TODO output the value stored in register r_val into memory to the address a_val
                let dest: IoDestination = IoDestination::from_code(a_val);
                log::info!("dest: {:?}", dest);

                match dest {

                    IoDestination::SPH => {
                        log::info!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                        let val:u8 = cpu.register_file[r_val as usize];
                        log::info!("val: {val:#b} {val:#x} {val}");
                        cpu.sph = val;

                        log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                    }

                    IoDestination::SPL => {
                        log::info!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                        let val:u8 = cpu.register_file[r_val as usize];
                        log::info!("val: {val:#b} {val:#x} {val}");
                        cpu.spl = val;

                        log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
                    }

                    IoDestination::DDRB | IoDestination::PORTB | IoDestination::PINB => {
                        log::info!("r_val: {r_val:#b} {r_val:#x} {r_val}");
                        let val:u8 = cpu.register_file[r_val as usize];
                        log::info!("val: {val:#b} {val:#x} {val}");

                        // write the value into SRAM
                        cpu.sram[a_val as usize] = val;
                    }

                    IoDestination::UNKNOWN => {
                        panic!("UNKNOWN enum value!")
                    }

                    // _ => panic!("Unknown enum value!")
                }

                cpu.pc += 2i32;
            },

            /*  89 */ 
            InstructionType::POP => { 
                log::info!("[POP]");

                let d_val: u16 = value_storage[&'d'];
                log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                cpu.pc += 2i32;

                pop_from_stack_u8(&mut cpu);

                log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
            },

            /*  90 */ 
            InstructionType::PUSH => { 
                log::info!("[PUSH]");

                let d_val: u16 = value_storage[&'d'];
                log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                cpu.pc += 2i32;

                let data = cpu.register_file[d_val as usize];
                push_to_stack_u8(&mut cpu, data);

                log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
            },

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
                log::info!("[RET]");

                // let d_val: u16 = value_storage[&'d'];
                // log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

                // cpu.pc += 2i32;

                // let data = cpu.register_file[d_val as usize];
                let k_hi: u16 = pop_from_stack_u8(&mut cpu) as u16;
                let k_lo: u16 = pop_from_stack_u8(&mut cpu) as u16;

                let k_val:i16 = ((k_hi << 8i16) + k_lo) as i16;

                cpu.pc = k_val as i32;

                log::info!("stack pointer: {:#04x} {:#04x}", cpu.sph, cpu.spl);
            },

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
            },

            InstructionType::Unknown => { panic!("Unknown instruction!"); },
    
            _ => { panic!("Unknown instruction!"); }

        }

        // // check the interrupt queue for a interrupt
        // // but only if the CPU has not executed an interrupt right before.
        // // The CPU has to execute at least one normal instruction before
        // // going into the next interrupt
        // if !force_normal_instruction_execution && is_global_interrupt_flag_enabled {

        //     // get next interrupt from the interrupt queue
        //     // the entry contains the type of interrupt and the address in the global interrupt vector table

        //     // check if this particular interrupt is enabled in the particular periphery register
        //     // if the specific interrupt is disabled, skip the event and check the next interrupt in the queue

        //     // turn of global interrupt enable flag

        //     // push the current PC onto the stack

        //     // disable global interrupt flag

        //     // disable the flag of the periphery specific interrupt in the specific interrupt register
            
        //     // execute the interrupt handler stored in the global interrupt vector at the address of the interrupt

        // }
    }

}
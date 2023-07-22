mod ihex_mgmt;
mod file_mgmt;

use std::io;
use std::io::Write;
use std::io::Cursor;
use std::collections::HashMap;
use std::fmt;

use env_logger::{Builder, Target};
use log::LevelFilter;

use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;

use byteorder::{LittleEndian, ReadBytesExt};

fn bit_match(data:u16, pattern:&str) -> bool {

    let mut bit_mask:u16 = 0x8000u16;
    let mut shift_counter:u8 = 15u8;

    for c in pattern.chars() {        

        // x means dont care, the function accepts any value
        if c != '1' && c != '0' {

            // next character
            if shift_counter > 0 {
                shift_counter = shift_counter - 1;
            }
            bit_mask = bit_mask >> 1;

            continue;
        }

        let temp = (data & bit_mask) >> shift_counter;

        // next character
        if shift_counter > 0 {
            shift_counter = shift_counter - 1;
        }
        bit_mask = bit_mask >> 1;

        if c == '1' && temp == 0 {
            return false;
        }
        if c == '0' && temp == 1 {
            return false;
        }
    }

    return true;
}

fn bit_pattern_match(data:u16, pattern:&str, value_storage:&mut HashMap<char, u16>) {

    let mut bit_mask:u16 = 0x8000u16; // 1000 0000 0000 0000
    let mut shift_counter:u8 = 15u8;

    for c in pattern.chars() {        

        // only look at placeholder values
        if c == '0' || c == '1' {

            // next character
            if shift_counter > 0 {
                shift_counter = shift_counter - 1;
            }
            bit_mask = bit_mask >> 1;

            continue;
        }

        // retrieve the bit value
        let temp = (data & bit_mask) >> shift_counter;

        // next character
        if shift_counter > 0 {
            shift_counter = shift_counter - 1;
        }
        bit_mask = bit_mask >> 1;

        //println!("");

        //println!("placeholder: {}, value: {}", c, temp);

        // check if c is part of the hashmap and insert it if not
        if !value_storage.contains_key(&c) {

            //println!("value_storage does not contain {}", c);
            value_storage.insert(c, temp);

            // // DEBUG
            // for (key, val) in value_storage.iter() {
            //     println!("{} {}", key, val);
            // }
        } else {
            //println!("value_storage contains {}", c);

            //*value_storage.get_mut(&c).unwrap() += 1;

            *value_storage.get_mut(&c).unwrap() = (value_storage.get(&c).unwrap() << 1) + temp;

            // let mut storage_value = *value_storage.get_mut(&c).unwrap();
            // storage_value = storage_value << 1;
            // storage_value = storage_value + temp;

            // let val_option = value_storage.get(&c);
            // let mut val_unwrap = val_option.unwrap();
            // println!("{}", val_unwrap);

            // let mut val_unwrap_temp = &(val_unwrap << 1);
            // val_unwrap_temp = &(val_unwrap_temp | temp);

            // value_storage.insert(c, *val_unwrap_temp);

            // // DEBUG
            // for (key, val) in value_storage.iter() {
            //     println!("{} {}", key, val);
            // }
        }

        
    }

    // // DEBUG
    // for (key, val) in value_storage.iter() {
    //     println!("{} {}", key, val);
    // }
}


#[derive(PartialEq)]
#[derive(Debug, Clone, Copy)]
//#[derive(Display, Clone, Copy)]
pub enum InstructionType {

    /* 5 */ ADC,
    /* 6 */ ADD,
    /* 7 */ ADIW,
    /* 8 */ AND,
    /* 9 */ ANDI,
    /* 10 */ ASR,
    /* 11 */ BCLR,
    /* 12 */ BLD,
    /* 13 */ BRBC,
    /* 14 */ BRBS,
    /* 15 */ BRCC,
    /* 16 */ BRCS,
    /* 17 */ BREAK, 
    /* 18 */ BREQ,
    /* 19 */ BRGE,
    /* 20 */ BRHC,
    /* 21 */ BRHS,
    /* 22 */ BRID,
    /* 23 */ BRIE,
    /* 24 */ BRLO,
    /* 25 */ BRLT,
    /* 26 */ BRMI,
    /* 27 */ BRNE,
    /* 28 */ BRPL,
    /* 29 */ BRSH,
    /* 30 */ BRTC,
    /* 31 */ BRTS,
    /* 32 */ BRVC,
    /* 33 */ BRVS,
    /* 34 */ BSET,
    /* 35 */ BST,
    /* 36 */ CALL,
    /* 37 */ CBI, 
    /* 38 */ CBR,
    /* 39 */ CLC,
    /* 40 */ CLH,
    /* 41 */ CLI,
    /* 42 */ CLN,
    /* 43 */ CLR,
    /* 44 */ CLS,
    /* 45 */ CLT,
    /* 46 */ CLV,
    /* 47 */ CLZ,
    /* 48 */ COM,
    /* 49 */ CP,
    /* 50 */ CPC,
    /* 51 */ CPI,
    /* 52 */ CPSE,
    /* 53 */ DEC,
    /* 54 */ DES,
    /* 55 */ EICALL,
    /* 56 */ EIJMP,
    /* 57 */ ELPM,
    /* 58 */ EOR,
    /* 59 */ FMUL,
    /* 60 */ FMULS,
    /* 61 */ FMULSU,
    /* 62 */ ICALL,
    /* 63 */ IJMP,
    /* 64 */ IN,
    /* 65.*/ INC,
    /* 66 */ JMP,
    /* 67 */ LAC,
    /* 68 */ LAS,
    /* 69 */ LAT, 
    /* 70 */ LD,
    /* 71 */ LD_LDD_Y,
    /* 72 */ LD_LDD_Z,
    /* 73 */ LDI,
    /* 74 */ LDS,
    /* 75 */ LDS_16bit, // (16-bit) – Load Direct from Data Space......................................................... 117
    /* 76 */ LPM, // – Load Program Memory...............................................................................118
    /* 77 */ LSL, // – Logical Shift Left..........................................................................................120
    /* 78 */ LSR, // – Logical Shift Right.......................................................................................122
    /* 79 */ MOV, // – Copy Register............................................................................................123
    /* 80 */ MOVW, // – Copy Register Word...............................................................................124
    /* 81 */ MUL, // – Multiply Unsigned.......................................................................................125
    /* 82 */ MULS, // – Multiply Signed........................................................................................ 126
    /* 83 */ MULSU , //
    /* 84 */ NEG,
    /* 85 */ NOP,
    /* 86 */ OR,
    /* 87 */ ORI,
    /* 88 */ OUT,
    /* 89 */ POP, // – Pop Register from Stack............................................................................ 135
    /* 90 */ PUSH, // – Push Register on Stack........................................................................... 136
    /* 91 */ RCALL, // – Relative Call to Subroutine.................................................................... 137
    /* 92 */ RET, // – Return from Subroutine.............................................................................. 139
    /* 93 */ RETI, // – Return from Interrupt................................................................................. 140
    /* 94 */ RJMP, // – Relative Jump.......................................................................................... 142
    /* 95 */ ROL, //
    /* 96 */ ROR, // – Rotate Right through Carry........................................................................145
    /* 97 */ SBC, // – Subtract with Carry.....................................................................................147
    /* 98 */ SBCI, // – Subtract Immediate with Carry SBI – Set Bit in I/O Register.................... 149
    /* 99 */ SBI, // – Set Bit in I/O Register.................................................................................. 151
    /* 100 */ SBIC, // – Skip if Bit in I/O Register is Cleared........................................................ 152
    /* 101 */ SBIS, // – Skip if Bit in I/O Register is Set............................................................... 153
    /* 102 */ SBIW, // – Subtract Immediate from Word...............................................................154
    /* 103 */ SBR, // – Set Bits in Register...................................................................................156
    /* 104 */ SBRC, // – Skip if Bit in Register is Cleared............................................................ 157
    /* 105 */ SBRS, // – Skip if Bit in Register is Set....................................................................158
    /* 106 */ SEC, // – Set Carry Flag.......................................................................................... 159
    /* 107 */ SEH, // – Set Half Carry Flag...................................................................................160
    /* 108 */ SEI, //
    /* 109 */ SEN, // – Set Negative Flag.....................................................................................162
    /* 110 */ SER, // – Set all Bits in Register.............................................................................. 163
    /* 111 */ SES, // – Set Signed Flag........................................................................................ 164
    /* 112 */ SET, // – Set T Flag................................................................................................. 165
    /* 113 */ SEV, // – Set Overflow Flag..................................................................................... 166
    /* 114 */ SEZ, // – Set Zero Flag............................................................................................ 167
    /* 115 */ SLEEP, //................................................................................................................. 168
    /* 116 */ SPM, // – Store Program Memory............................................................................169
    /* 117 */ SPM_2, // SPM #2 – Store Program Memory.......................................................................171
    /* 118 */ ST, // – Store Indirect From Register to Data Space using Index X.........................173
    /* 119 */ ST_STD_Y, // – Store Indirect From Register to Data Space using Index Y..............175
    /* 120 */ ST_STD_Z, 
    /* 121 */ STS, // STS – Store Direct to Data Space.......................................................................179
    /* 122 */ STS_16bit, // STS (16-bit) – Store Direct to Data Space.......................................................... 180
    /* 123 */ SUB, // – Subtract Without Carry.............................................................................181
    /* 124 */ SUBI, // – Subtract Immediate................................................................................. 183
    /* 125 */ SWAP, // – Swap Nibbles........................................................................................ 185
    /* 126 */ TST, // – Test for Zero or Minus...............................................................................186
    /* 127 */ WDR, // – Watchdog Reset......................................................................................187
    /* 128 */ XCH, // 
    Unknown,
}

impl InstructionType {

    pub fn from_code(code: u8) -> InstructionType {
        match code {
            5 => InstructionType::ADC,
            6 => InstructionType::ADD,
            7 => InstructionType::ADIW,
            8 => InstructionType::AND,
            9 => InstructionType::ANDI,
            10 => InstructionType::ASR,
            11 => InstructionType::BCLR,
            12 => InstructionType::BLD,
            13 => InstructionType::BRBC,
            14 => InstructionType::BRBS,
            15 => InstructionType::BRCC,
            16 => InstructionType::BRCS,
            17 => InstructionType::BREAK, 
            18 => InstructionType::BREQ,
            19 => InstructionType::BRGE,
            20 => InstructionType::BRHC,
            21 => InstructionType::BRHS,
            22 => InstructionType::BRID,
            23 => InstructionType::BRIE,
            24 => InstructionType::BRLO,
            25 => InstructionType::BRLT,
            27 => InstructionType::BRNE,
            28 => InstructionType::BRPL,
            29 => InstructionType::BRSH,
            30 => InstructionType::BRTC,
            31 => InstructionType::BRTS,
            32 => InstructionType::BRVC,
            33 => InstructionType::BRVS,
            34 => InstructionType::BSET,
            35 => InstructionType::BST,
            36 => InstructionType::CALL,
            37 => InstructionType::CBI, 
            38 => InstructionType::CBR,
            39 => InstructionType::CLC,
            40 => InstructionType::CLH,
            41 => InstructionType::CLI,
            42 => InstructionType::CLN,
            43 => InstructionType::CLR,
            44 => InstructionType::CLS,
            45 => InstructionType::CLT,
            46 => InstructionType::CLV,
            47 => InstructionType::CLZ,
            48 => InstructionType::COM,
            49 => InstructionType::CP,
            50 => InstructionType::CPC,
            51 => InstructionType::CPI,
            52 => InstructionType::CPSE,
            53 => InstructionType::DEC,
            54 => InstructionType::DES,
            55 => InstructionType::EICALL,
            56 => InstructionType::EIJMP,
            57 => InstructionType::ELPM,
            58 => InstructionType::EOR,
            59 => InstructionType::FMUL,
            60 => InstructionType::FMULS,
            61 => InstructionType::FMULSU,
            62 => InstructionType::ICALL,
            63 => InstructionType::IJMP,
            64 => InstructionType::IN,
            65 => InstructionType::INC,
            66 => InstructionType::JMP,
            67 => InstructionType::LAC,
            68 => InstructionType::LAS,
            69 => InstructionType::LAT, 
            70 => InstructionType::LD,
            71 => InstructionType::LD_LDD_Y,
            72 => InstructionType::LD_LDD_Z,
            73 => InstructionType::LDI,
            74 => InstructionType::LDS,
            75 => InstructionType::LDS_16bit, // (16-bit) – Load Direct from Data Space......................................................... 117
            76 => InstructionType::LPM, // – Load Program Memory...............................................................................118
            77 => InstructionType::LSL, // – Logical Shift Left..........................................................................................120
            78 => InstructionType::LSR, // – Logical Shift Right.......................................................................................122
            79 => InstructionType::MOV, // – Copy Register............................................................................................123
            80 => InstructionType::MOVW, // – Copy Register Word...............................................................................124
            81 => InstructionType::MUL, // – Multiply Unsigned.......................................................................................125
            82 => InstructionType::MULS, // – Multiply Signed........................................................................................ 126
            83 => InstructionType::MULSU , //
            84 => InstructionType::NEG,
            85 => InstructionType::NOP, 
            86 => InstructionType::OR,
            87 => InstructionType::ORI,
            88 => InstructionType::OUT,
            89 => InstructionType::POP,
            90 => InstructionType::PUSH,
            91 => InstructionType::RCALL,
            92 => InstructionType::RET,
            93 => InstructionType::RETI, 
            94 => InstructionType::RJMP,
            95 => InstructionType::ROL, //
            96 => InstructionType::ROR, // – Rotate Right through Carry........................................................................145
            97 => InstructionType::SBC, 
            98 => InstructionType::SBCI,
            99 => InstructionType::SBI, // – Set Bit in I/O Register.................................................................................. 151
            100 => InstructionType::SBIC, // – Skip if Bit in I/O Register is Cleared........................................................ 152
            101 => InstructionType::SBIS, // – Skip if Bit in I/O Register is Set............................................................... 153
            102 => InstructionType::SBIW, // – Subtract Immediate from Word...............................................................154
            103 => InstructionType::SBR, // – Set Bits in Register...................................................................................156
            104 => InstructionType::SBRC, // – Skip if Bit in Register is Cleared............................................................ 157
            105 => InstructionType::SBRS, // – Skip if Bit in Register is Set....................................................................158
            106 => InstructionType::SEC, // – Set Carry Flag.......................................................................................... 159
            107 => InstructionType::SEH, // – Set Half Carry Flag...................................................................................160
            108 => InstructionType::SEI, //
            109 => InstructionType::SEN, // – Set Negative Flag.....................................................................................162
            110 => InstructionType::SER, // – Set all Bits in Register.............................................................................. 163
            111 => InstructionType::SES, // – Set Signed Flag........................................................................................ 164
            112 => InstructionType::SET, // – Set T Flag................................................................................................. 165
            113 => InstructionType::SEV, // – Set Overflow Flag..................................................................................... 166
            114 => InstructionType::SEZ, // – Set Zero Flag............................................................................................ 167
            115 => InstructionType::SLEEP, //................................................................................................................. 168
            116 => InstructionType::SPM, // – Store Program Memory............................................................................169
            117 => InstructionType::SPM_2, // SPM #2 – Store Program Memory.......................................................................171
            118 => InstructionType::ST,
            119 => InstructionType::ST_STD_Y, 
            120 => InstructionType::ST_STD_Z, 
            121 => InstructionType::STS, // STS – Store Direct to Data Space.......................................................................179
            122 => InstructionType::STS_16bit, // STS (16-bit) – Store Direct to Data Space.......................................................... 180
            123 => InstructionType::SUB, // – Subtract Without Carry.............................................................................181
            124 => InstructionType::SUBI, // – Subtract Immediate................................................................................. 183
            125 => InstructionType::SWAP, // – Swap Nibbles........................................................................................ 185
            126 => InstructionType::TST, // – Test for Zero or Minus...............................................................................186
            127 => InstructionType::WDR, // – Watchdog Reset......................................................................................187
            128 => InstructionType::XCH,

            // STD Y (B) = 119. 
            // STD Y (C) = 119.
            // STD Y (D) = 119.
            // STD Z (A) = 120.
            // STD Z (B) = 120.
            // STD Z (C) = 120.
            // STD Z (D) = 120.
            
            
            _ => InstructionType::Unknown,
        }
    }

    pub fn to_code(instruction_type: &InstructionType) -> u8 {
        match instruction_type {
            InstructionType::IN => 64,
            InstructionType::JMP => 66,
            InstructionType::LDI => 73,
            InstructionType::OUT => 88,
            InstructionType::Unknown => 0xFF,
            _ => 0xFF,
        }
    }

}

// impl fmt::Debug for InstructionType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             // Day::Monday => write!(f, "Monday"),
//             // Day::Tuesday => write!(f, "Tuesday"),
//             // Day::Wednesday => write!(f, "Wednesday"),
//             // Day::Thursday => write!(f, "Thursday"),
//             // Day::Friday => write!(f, "Friday"),
//             InstructionType::CALL => { log::info!("CALL y"); }
//             InstructionType::EOR => { log::info!("EOR y"); }
//             InstructionType::IN => { log::info!("IN y"); }
//             InstructionType::JMP => { log::info!("JMP y"); }
//             InstructionType::OUT => { log::info!("OUT y"); }
//             InstructionType::Unknown => { write!(f, "Unknown y"); }
//             _ => { write!(f, "Unknown"); }
//         }
//     }
// }

struct InstructionDefinition {

    // id of instruction in atmel-0856-avr-instruction-set-manual.pdf
    index: u8,

    // type of this instruction as an enum
    instruction_type: InstructionType,

    // bit_pattern for decoding of an instruction
    bit_pattern: String,

}

/// finds a command that matches data and returns that instruction definition
/// If no matching instruction is found, returns the instruction definition 'unknown'
fn decode<'a>(data:u16, instructions: &'a Vec<InstructionDefinition>, unknown: &'a InstructionDefinition, value_storage:&mut HashMap<char, u16>) -> &'a InstructionDefinition {

    for instruction in instructions 
    {
        if bit_match(data, &instruction.bit_pattern)
        {
            bit_pattern_match(data, &instruction.bit_pattern, value_storage);

            return &instruction;
        }
    }
     
    return unknown;
}

fn main() -> io::Result<()> {

    println!("whatavr starting ...");

    init_logging();
    log_start();

    //
    // prepare instructions
    //

    /* 5 ADC */ let adc: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ADC), instruction_type: InstructionType::ADC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 6 ADD */ let add: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ADD), instruction_type: InstructionType::ADD, bit_pattern: String::from("000111rdddddrrrr") };
    /* 7 ADIW */ let adiw: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ADIW), instruction_type: InstructionType::ADIW, bit_pattern: String::from("000111rdddddrrrr") };
    /* 8 AND */ let and: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::AND), instruction_type: InstructionType::AND, bit_pattern: String::from("000111rdddddrrrr") };
    /* 9 ANDI */ let andi: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ANDI), instruction_type: InstructionType::ANDI, bit_pattern: String::from("000111rdddddrrrr") };
    /* 10 ASR */ let asr: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ASR), instruction_type: InstructionType::ASR, bit_pattern: String::from("000111rdddddrrrr") };
    /* 11 BCLR */ let bclr: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BCLR), instruction_type: InstructionType::BCLR, bit_pattern: String::from("000111rdddddrrrr") };
    /* 12 BLD */ let bld: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BLD), instruction_type: InstructionType::BLD, bit_pattern: String::from("000111rdddddrrrr") };
    /* 13 BRBC */ let brbc: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRBC), instruction_type: InstructionType::BRBC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 14 BRBS */ let brbs: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRBS), instruction_type: InstructionType::BRBS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 15 BRCC */ let brcc: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRCC), instruction_type: InstructionType::BRCC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 16 => InstructionType::BRCS */ let BRCS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRCS), instruction_type: InstructionType::BRCS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 17 => InstructionType::BREAK */ let BREAK: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BREAK), instruction_type: InstructionType::BREAK, bit_pattern: String::from("000111rdddddrrrr") };
    /* 18 => InstructionType::BREQ */ let BREQ: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BREQ), instruction_type: InstructionType::BREQ, bit_pattern: String::from("000111rdddddrrrr") };
    /* 19 => InstructionType::BRGE */ let BRGE: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRGE), instruction_type: InstructionType::BRGE, bit_pattern: String::from("000111rdddddrrrr") };
    /* 20 => InstructionType::BRHC */ let BRHC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRHC), instruction_type: InstructionType::BRHC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 21 => InstructionType::BRHS */ let BRHS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRHS), instruction_type: InstructionType::BRHS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 22 => InstructionType::BRID */ let BRID: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRID), instruction_type: InstructionType::BRID, bit_pattern: String::from("000111rdddddrrrr") };
    /* 23 => InstructionType::BRIE */ let BRIE: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRIE), instruction_type: InstructionType::BRIE, bit_pattern: String::from("000111rdddddrrrr") };
    /* 24 => InstructionType::BRLO */ let BRLO: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRLO), instruction_type: InstructionType::BRLO, bit_pattern: String::from("000111rdddddrrrr") };
    /* 25 => InstructionType::BRLT */ let BRLT: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRLT), instruction_type: InstructionType::BRLT, bit_pattern: String::from("000111rdddddrrrr") };
    /* 27 => InstructionType::BRNE */ let BRNE: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRNE), instruction_type: InstructionType::BRNE, bit_pattern: String::from("000111rdddddrrrr") };
    /* 28 => InstructionType::BRPL */ let BRPL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRPL), instruction_type: InstructionType::BRPL, bit_pattern: String::from("000111rdddddrrrr") };
    /* 29 => InstructionType::BRSH */ let BRSH: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRSH), instruction_type: InstructionType::BRSH, bit_pattern: String::from("000111rdddddrrrr") };
    /* 30 => InstructionType::BRTC */ let BRTC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRTC), instruction_type: InstructionType::BRTC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 31 => InstructionType::BRTS */ let BRTS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRTS), instruction_type: InstructionType::BRTS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 32 => InstructionType::BRVC */ let BRVC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRVC), instruction_type: InstructionType::BRVC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 33 => InstructionType::BRVS */ let BRVS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRVS), instruction_type: InstructionType::BRVS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 34 => InstructionType::BSET */ let BSET: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BSET), instruction_type: InstructionType::BSET, bit_pattern: String::from("000111rdddddrrrr") };
    /* 35 => InstructionType::BST */ let BST: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BST), instruction_type: InstructionType::BST, bit_pattern: String::from("000111rdddddrrrr") };
    /* 36 */ let call: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CALL), instruction_type: InstructionType::CALL, bit_pattern: String::from("1001010kkkkk111k") };
    /* 37 => InstructionType::CBI */  let CBI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CBI), instruction_type: InstructionType::CBI, bit_pattern: String::from("000111rdddddrrrr") };
    /* 38 => InstructionType::CBR */ let CBR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CBR), instruction_type: InstructionType::CBR, bit_pattern: String::from("000111rdddddrrrr") };
    /* 39 => InstructionType::CLC */ let CLC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLC), instruction_type: InstructionType::CLC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 40 => InstructionType::CLH */ let CLH: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLH), instruction_type: InstructionType::CLH, bit_pattern: String::from("000111rdddddrrrr") };
    /* 41 => InstructionType::CLI */ let CLI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLI), instruction_type: InstructionType::CLI, bit_pattern: String::from("000111rdddddrrrr") };
    /* 42 => InstructionType::CLN */ let CLN: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLN), instruction_type: InstructionType::CLN, bit_pattern: String::from("000111rdddddrrrr") };
    /* 43 => InstructionType::CLR */ let CLR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLR), instruction_type: InstructionType::CLR, bit_pattern: String::from("000111rdddddrrrr") };
    /* 44 => InstructionType::CLS */ let CLS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLS), instruction_type: InstructionType::CLS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 45 => InstructionType::CLT */ let CLT: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLT), instruction_type: InstructionType::CLT, bit_pattern: String::from("000111rdddddrrrr") };
    /* 46 => InstructionType::CLV */ let CLV: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLV), instruction_type: InstructionType::CLV, bit_pattern: String::from("000111rdddddrrrr") };
    /* 47 => InstructionType::CLZ */ let CLZ: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLZ), instruction_type: InstructionType::CLZ, bit_pattern: String::from("000111rdddddrrrr") };
    /* 48 => InstructionType::COM */ let COM: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::COM), instruction_type: InstructionType::COM, bit_pattern: String::from("000111rdddddrrrr") };
    /* 49 => InstructionType::CP */ let CP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CP), instruction_type: InstructionType::CP, bit_pattern: String::from("000111rdddddrrrr") };
    /* 50 => InstructionType::CPC */ let CPC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CPC), instruction_type: InstructionType::CPC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 51 => InstructionType::CPI */ let CPI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CPI), instruction_type: InstructionType::CPI, bit_pattern: String::from("000111rdddddrrrr") };
    /* 52 => InstructionType::CPSE */ let CPSE: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CPSE), instruction_type: InstructionType::CPSE, bit_pattern: String::from("000111rdddddrrrr") };
    /* 53 => InstructionType::DEC */ let DEC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::DEC), instruction_type: InstructionType::DEC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 54 => InstructionType::DES */ let DES: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::DES), instruction_type: InstructionType::DES, bit_pattern: String::from("000111rdddddrrrr") };
    /* 55 => InstructionType::EICALL */ let EICALL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::EICALL), instruction_type: InstructionType::EICALL, bit_pattern: String::from("000111rdddddrrrr") };
    /* 56 => InstructionType::EIJMP */ let EIJMP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::EIJMP), instruction_type: InstructionType::EIJMP, bit_pattern: String::from("000111rdddddrrrr") };
    /* 57 => InstructionType::ELPM */ let ELPM: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ELPM), instruction_type: InstructionType::ELPM, bit_pattern: String::from("000111rdddddrrrr") };
    /* 58 */ let eor: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::EOR), instruction_type: InstructionType::EOR, bit_pattern: String::from("001001rdddddrrrr") };
    /* 59 => InstructionType::FMUL */ let FMUL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::FMUL), instruction_type: InstructionType::FMUL, bit_pattern: String::from("000111rdddddrrrr") };
    /* 60 => InstructionType::FMULS */ let FMULS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::FMULS), instruction_type: InstructionType::FMULS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 61 => InstructionType::FMULSU */ let FMULSU: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::FMULSU), instruction_type: InstructionType::FMULSU, bit_pattern: String::from("000111rdddddrrrr") };
    /* 62 => InstructionType::ICALL */ let ICALL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ICALL), instruction_type: InstructionType::ICALL, bit_pattern: String::from("000111rdddddrrrr") };
    /* 63 => InstructionType::IJMP */ let IJMP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::IJMP), instruction_type: InstructionType::IJMP, bit_pattern: String::from("000111rdddddrrrr") };
    /* 64 */ let in_: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::IN), instruction_type: InstructionType::IN, bit_pattern: String::from("10110AAdddddAAAA") };
    /* 65 => InstructionType::INC */ let INC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::INC), instruction_type: InstructionType::INC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 66 */ let jmp: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::JMP), instruction_type: InstructionType::JMP, bit_pattern: String::from("1001010kkkkk110k") };
    /* 67 => InstructionType::LAC */ let LAC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LAC), instruction_type: InstructionType::LAC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 68 => InstructionType::LAS */ let LAS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LAS), instruction_type: InstructionType::LAS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 69 => InstructionType::LAT */  let LAT: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LAT), instruction_type: InstructionType::LAT, bit_pattern: String::from("000111rdddddrrrr") };
    /* 70 => InstructionType::LD */ let LD: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LD), instruction_type: InstructionType::LD, bit_pattern: String::from("000111rdddddrrrr") };
    /* 71 => InstructionType::LD_LDD_Y */ let LD_LDD_Y: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LD_LDD_Y), instruction_type: InstructionType::LD_LDD_Y, bit_pattern: String::from("000111rdddddrrrr") };
    /* 72 => InstructionType::LD_LDD_Z */ let LD_LDD_Z: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LD_LDD_Z), instruction_type: InstructionType::LD_LDD_Z, bit_pattern: String::from("000111rdddddrrrr") };
    /* 73 => InstructionType::LDI */ let ldi: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LDI), instruction_type: InstructionType::LDI, bit_pattern: String::from("1110KKKKddddKKKK") };
    /* 74 => InstructionType::LDS */ let LDS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LDS), instruction_type: InstructionType::LDS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 75 => InstructionType::LDS_16bit */  let LDS_16bit: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LDS_16bit), instruction_type: InstructionType::LDS_16bit, bit_pattern: String::from("000111rdddddrrrr") };
    /* 76 => InstructionType::LPM */  let LPM: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LPM), instruction_type: InstructionType::LPM, bit_pattern: String::from("000111rdddddrrrr") };
    /* 77 => InstructionType::LSL */  let LSL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LSL), instruction_type: InstructionType::LSL, bit_pattern: String::from("000111rdddddrrrr") };
    /* 78 => InstructionType::LSR */  let LSR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LSR), instruction_type: InstructionType::LSR, bit_pattern: String::from("000111rdddddrrrr") };
    /* 79 => InstructionType::MOV */  let MOV: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MOV), instruction_type: InstructionType::MOV, bit_pattern: String::from("000111rdddddrrrr") };
    /* 80 => InstructionType::MOVW */ let MOVW: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MOVW), instruction_type: InstructionType::MOVW, bit_pattern: String::from("000111rdddddrrrr") };
    /* 81 => InstructionType::MUL */  let MUL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MUL), instruction_type: InstructionType::MUL, bit_pattern: String::from("000111rdddddrrrr") };
    /* 82 => InstructionType::MULS */  let MULS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MULS), instruction_type: InstructionType::MULS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 83 => InstructionType::MULSU */  let MULSU: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MULSU), instruction_type: InstructionType::MULSU, bit_pattern: String::from("000111rdddddrrrr") };
    /* 84 => InstructionType::NEG */ let NEG: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::NEG), instruction_type: InstructionType::NEG, bit_pattern: String::from("000111rdddddrrrr") };
    /* 85 => InstructionType::NOP */  let NOP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::NOP), instruction_type: InstructionType::NOP, bit_pattern: String::from("000111rdddddrrrr") };
    /* 86 => InstructionType::OR */ let OR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::OR), instruction_type: InstructionType::OR, bit_pattern: String::from("000111rdddddrrrr") };
    /* 87 => InstructionType::ORI */ let ORI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ORI), instruction_type: InstructionType::ORI, bit_pattern: String::from("000111rdddddrrrr") };
    /* 88 */ let out: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::OUT), instruction_type: InstructionType::OUT, bit_pattern: String::from("10111AArrrrrAAAA") };
    /* 89 => InstructionType::POP */ let POP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::POP), instruction_type: InstructionType::POP, bit_pattern: String::from("000111rdddddrrrr") };
    /* 90 => InstructionType::PUSH */ let push: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::PUSH), instruction_type: InstructionType::PUSH, bit_pattern: String::from("1001001ddddd1111") };
    /* 91 => InstructionType::RCALL */ let RCALL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RCALL), instruction_type: InstructionType::RCALL, bit_pattern: String::from("000111rdddddrrrr") };
    /* 92 => InstructionType::RET */ let RET: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RET), instruction_type: InstructionType::RET, bit_pattern: String::from("000111rdddddrrrr") };
    /* 93 => InstructionType::RETI */  let RETI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RETI), instruction_type: InstructionType::RETI, bit_pattern: String::from("000111rdddddrrrr") };
    /* 94 => InstructionType::RJMP */ let RJMP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RJMP), instruction_type: InstructionType::RJMP, bit_pattern: String::from("000111rdddddrrrr") };
    /* 95 => InstructionType::ROL */  let ROL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ROL), instruction_type: InstructionType::ROL, bit_pattern: String::from("000111rdddddrrrr") };
    /* 96 => InstructionType::ROR */  let ROR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ROR), instruction_type: InstructionType::ROR, bit_pattern: String::from("000111rdddddrrrr") };
    /* 97 => InstructionType::SBC */  let SBC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBC), instruction_type: InstructionType::SBC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 98 => InstructionType::SBCI */ let SBCI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBCI), instruction_type: InstructionType::SBCI, bit_pattern: String::from("000111rdddddrrrr") };
    /* 99 => InstructionType::SBI */  let SBI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBI), instruction_type: InstructionType::SBI, bit_pattern: String::from("000111rdddddrrrr") };
    /* 100 => InstructionType::SBIC */  let SBIC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBIC), instruction_type: InstructionType::SBIC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 101 => InstructionType::SBIS */  let SBIS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBIS), instruction_type: InstructionType::SBIS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 102 => InstructionType::SBIW */  let SBIW: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBIW), instruction_type: InstructionType::SBIW, bit_pattern: String::from("000111rdddddrrrr") };
    /* 103 => InstructionType::SBR */  let SBR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBR), instruction_type: InstructionType::SBR, bit_pattern: String::from("000111rdddddrrrr") };
    /* 104 => InstructionType::SBRC */  let SBRC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBRC), instruction_type: InstructionType::SBRC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 105 => InstructionType::SBRS */  let SBRS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBRS), instruction_type: InstructionType::SBRS, bit_pattern: String::from("000111rdddddrrrr") };
    /* 106 => InstructionType::SEC */  let SEC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEC), instruction_type: InstructionType::SEC, bit_pattern: String::from("000111rdddddrrrr") };
    /* 107 => InstructionType::SEH */  let SEH: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEH), instruction_type: InstructionType::SEH, bit_pattern: String::from("000111rdddddrrrr") };
    /* 108 => InstructionType::SEI */  let SEI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEI), instruction_type: InstructionType::SEI, bit_pattern: String::from("000111rdddddrrrr") };
    /* 109 => InstructionType::SEN */  let SEN: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEN), instruction_type: InstructionType::SEN, bit_pattern: String::from("000111rdddddrrrr") };
    /* 110 => InstructionType::SER */  let SER: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SER), instruction_type: InstructionType::SER, bit_pattern: String::from("000111rdddddrrrr") };
    /* 111 => InstructionType::SES */  let SES: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SES), instruction_type: InstructionType::SES, bit_pattern: String::from("000111rdddddrrrr") };
    /* 112 => InstructionType::SET */  let SET: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SET), instruction_type: InstructionType::SET, bit_pattern: String::from("000111rdddddrrrr") };
    /* 113 => InstructionType::SEV */  let SEV: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEV), instruction_type: InstructionType::SEV, bit_pattern: String::from("000111rdddddrrrr") };
    /* 114 => InstructionType::SEZ */  let SEZ: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEZ), instruction_type: InstructionType::SEZ, bit_pattern: String::from("000111rdddddrrrr") };
    /* 115 => InstructionType::SLEEP */  let SLEEP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SLEEP), instruction_type: InstructionType::SLEEP, bit_pattern: String::from("000111rdddddrrrr") };
    /* 116 => InstructionType::SPM */  let SPM: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SPM), instruction_type: InstructionType::SPM, bit_pattern: String::from("000111rdddddrrrr") };
    /* 117 => InstructionType::SPM_2 */  let SPM_2: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SPM_2), instruction_type: InstructionType::SPM_2, bit_pattern: String::from("000111rdddddrrrr") };
    /* 118 => InstructionType::ST */  let ST: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST), instruction_type: InstructionType::ST, bit_pattern: String::from("000111rdddddrrrr") };
    /*  119 => InstructionType::ST_STD_Y */   let ST_STD_Y: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Y), instruction_type: InstructionType::ST_STD_Y, bit_pattern: String::from("000111rdddddrrrr") };
    /*  120 => InstructionType::ST_STD_Z */   let ST_STD_Z: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Z), instruction_type: InstructionType::ST_STD_Z, bit_pattern: String::from("000111rdddddrrrr") };
    /*  121 => InstructionType::STS */  let STS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::STS), instruction_type: InstructionType::STS, bit_pattern: String::from("000111rdddddrrrr") };
    /*  122 => InstructionType::STS_16bit */  let STS_16bit: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::STS_16bit), instruction_type: InstructionType::STS_16bit, bit_pattern: String::from("000111rdddddrrrr") };
    /*  123 => InstructionType::SUB */  let SUB: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SUB), instruction_type: InstructionType::SUB, bit_pattern: String::from("000111rdddddrrrr") };
    /*  124 => InstructionType::SUBI */  let SUBI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SUBI), instruction_type: InstructionType::SUBI, bit_pattern: String::from("000111rdddddrrrr") };
    /*  125 => InstructionType::SWAP */  let SWAP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SWAP), instruction_type: InstructionType::SWAP, bit_pattern: String::from("000111rdddddrrrr") };
    /*  126 => InstructionType::TST */  let TST: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::TST), instruction_type: InstructionType::TST, bit_pattern: String::from("000111rdddddrrrr") };
    /*  127 => InstructionType::WDR */  let WDR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::WDR), instruction_type: InstructionType::WDR, bit_pattern: String::from("000111rdddddrrrr") };
    /*  128 => InstructionType::XCH */ let XCH: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::XCH), instruction_type: InstructionType::XCH, bit_pattern: String::from("000111rdddddrrrr") };
    let unknown: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::Unknown), instruction_type: InstructionType::Unknown, bit_pattern: String::from("xxxxxxxxxxxxxxxx") };

    let mut instructions: Vec<InstructionDefinition> = Vec::new();
    instructions.push(adc);
    instructions.push(add);
    instructions.push(adiw);
    instructions.push(and);
    instructions.push(andi);
    instructions.push(asr);
    instructions.push(bclr);
    instructions.push(bld);
    instructions.push(brbc);
    instructions.push(brbs);
    instructions.push(brcc);
    instructions.push(BRCS);
    instructions.push(BREAK);
    instructions.push(BREQ);
    instructions.push(BRGE);
    instructions.push(BRHC);
    instructions.push(BRHS);
    instructions.push(BRID);
    instructions.push(BRIE);
    instructions.push(BRLO);
    instructions.push(BRLT);
    instructions.push(BRNE);
    instructions.push(BRPL);
    instructions.push(BRSH);
    instructions.push(BRTC);
    instructions.push(BRTS);
    instructions.push(BRVC);
    instructions.push(BRVS);
    instructions.push(BSET);
    instructions.push(BST);
    instructions.push(call);
    instructions.push(CBI);
    instructions.push(CBR);
    instructions.push(CLC);
    instructions.push(CLH);
    instructions.push(CLI);
    instructions.push(CLN);
    instructions.push(CLR);
    instructions.push(CLS);
    instructions.push(CLT);
    instructions.push(CLV);
    instructions.push(CLZ);
    instructions.push(COM);
    instructions.push(CP);
    instructions.push(CPC);
    instructions.push(CPI);
    instructions.push(CPSE);
    instructions.push(DEC);
    instructions.push(DES);
    instructions.push(EICALL);
    instructions.push(EIJMP);
    instructions.push(ELPM);
    instructions.push(eor);
    instructions.push(FMUL);
    instructions.push(FMULS);
    instructions.push(FMULSU);
    instructions.push(ICALL);
    instructions.push(IJMP);
    instructions.push(in_);
    instructions.push(INC);
    instructions.push(jmp);
    instructions.push(LAC);
    instructions.push(LAS);
    instructions.push(LAT);
    instructions.push(LD);
    instructions.push(LD_LDD_Y);
    instructions.push(LD_LDD_Z);
    instructions.push(ldi);
    instructions.push(LDS);
    instructions.push(LDS_16bit);
    instructions.push(LPM);
    instructions.push(LSL);
    instructions.push(LSR);
    instructions.push(MOV);
    instructions.push(MOVW);
    instructions.push(MUL);
    instructions.push(MULS);
    instructions.push(MULSU);
    instructions.push(NEG);
    instructions.push(NOP);
    instructions.push(OR);
    instructions.push(ORI);
    instructions.push(out);
    instructions.push(POP);
    instructions.push(push);
    instructions.push(RCALL);
    instructions.push(RET);
    instructions.push(RETI);
    instructions.push(RJMP);
    instructions.push(ROL);
    instructions.push(ROR);
    instructions.push(SBC);
    instructions.push(SBCI);
    instructions.push(SBI);
    instructions.push(SBIC);
    instructions.push(SBIS);
    instructions.push(SBIW);
    instructions.push(SBR);
    instructions.push(SBRC);
    instructions.push(SBRS);
    instructions.push(SEC);
    instructions.push(SEH);
    instructions.push(SEI);
    instructions.push(SEN);
    instructions.push(SER);
    instructions.push(SES);
    instructions.push(SET);
    instructions.push(SEV);
    instructions.push(SEZ);
    instructions.push(SLEEP);
    instructions.push(SPM);
    instructions.push(SPM_2);
    instructions.push(ST);
    instructions.push(ST_STD_Y);
    instructions.push(ST_STD_Z);
    instructions.push(STS);
    instructions.push(STS_16bit);
    instructions.push(SUB);
    instructions.push(SUBI);
    instructions.push(SWAP);
    instructions.push(TST);
    instructions.push(WDR);
    instructions.push(XCH);
    //instructions.push(unknown);

    //
    // load hex file
    //

    let mut hex_file_path:String = String::new();
    //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank1.hex");
    //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank2.hex") {
    //hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication1/GccApplication1.hex");
    hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication2/GccApplication1.hex");
    //hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/arduboy/Ardynia/ardynia.hex");

    // split into segments
    // each segment has to have a segment_start and a segment_size
    let mut segments: Vec<Segment> = Vec::new();
    match parse_hex_file(&mut segments, &hex_file_path) {
        Ok(_name) => log::info!("File read"),
        Err(err) => {
            log::error!("An error occured while retrieving the peername: {:?}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error at load hex file!"));
        }
    }

    // // DEBUG dump parsed segments
    // for seg in segments.iter_mut() {
    //     log::info!("Segment: {}", seg);
    // }

    // process the first segment only
    let ref segment_0 = &segments[0];
    log::info!("Segment: {}", segment_0);

    let mut index: usize = 0;

    // TODO output disassembly so that a comparison to the .lss is possible
    // TODO build records from lines so that parsing a .lss file produces the exact same output
    // TODO build an executor for records
    //
    // loop:
    // Set PC to 0
    // Fetch instruction from PC
    // increment PC by 2
    // if the current instruction is a jump, set instruction pointer to jump destination
    // goto loop

    let mut rdr = Cursor::new(&segment_0.data);
    while index < segment_0.data.len()
    {
        let word:u16 = rdr.read_u16::<LittleEndian>().unwrap().into();
        index += 2;

        let wword:u32 = word.into();

        log::trace!("word: {:#06x} {:b}", word, word);

        let mut value_storage:HashMap<char, u16> = HashMap::new();
        let instruction: &InstructionDefinition = decode(word, &instructions, &unknown, &mut value_storage);

        log::info!("instruction {:?}", instruction.instruction_type);

        match instruction.instruction_type {

            InstructionType::ADC => {},
            InstructionType::ADD => {},
            
    /* 7 */ InstructionType::ADIW => {},
    /* 8 */ InstructionType::AND => {},
    /* 9 */ InstructionType::ANDI => {}
    /* 10 */ InstructionType::ASR => {},
    /* 11 */ InstructionType::BCLR => {},
    /* 12 */ InstructionType::BLD => {},
    /* 13 */ InstructionType::BRBC => {},
    /* 14 */ InstructionType::BRBS => {},
    /* 15 */ InstructionType::BRCC => {},
    /* 16 */ InstructionType::BRCS => {},
    /* 17 */ InstructionType::BREAK => {}, 
    /* 18 */ InstructionType::BREQ => {},
    /* 19 */ InstructionType::BRGE => {},
    /* 20 */ InstructionType::BRHC => {},
    /* 21 */ InstructionType::BRHS => {},
    /* 22 */ InstructionType::BRID => {},
    /* 23 */ InstructionType::BRIE => {},
    /* 24 */ InstructionType::BRLO => {},
    /* 25 */ InstructionType::BRLT => {},
    /* 26 */ InstructionType::BRMI => {},
    /* 27 */ InstructionType::BRNE => {},
    /* 28 */ InstructionType::BRPL => {},
    /* 29 */ InstructionType::BRSH => {},
    /* 30 */ InstructionType::BRTC => {},
    /* 31 */ InstructionType::BRTS => {},
    /* 32 */ InstructionType::BRVC => {},
    /* 33 */ InstructionType::BRVS => {},
    /* 34 */ InstructionType::BSET => {},
    /* 35 */ InstructionType::BST => {},
    /* 36 */ InstructionType::CALL => {},
    /* 37 */ InstructionType::CBI => {}, 
    /* 38 */ InstructionType::CBR => {},
    /* 39 */ InstructionType::CLC => {},
    /* 40 */ InstructionType::CLH => {},
    /* 41 */ InstructionType::CLI => {},
    /* 42 */ InstructionType::CLN => {},
    /* 43 */ InstructionType::CLR => {},
    /* 44 */ InstructionType::CLS => {},
    /* 45 */ InstructionType::CLT => {},
    /* 46 */ InstructionType::CLV => {},
    /* 47 */ InstructionType::CLZ => {},
    /* 48 */ InstructionType::COM => {},
    /* 49 */ InstructionType::CP => {},
    /* 50 */ InstructionType::CPC => {},
    /* 51 */ InstructionType::CPI => {},
    /* 52 */ InstructionType::CPSE => {},
    /* 53 */ InstructionType::DEC => {},
    /* 54 */ InstructionType::DES => {},
    /* 55 */ InstructionType::EICALL => {},
    /* 56 */ InstructionType::EIJMP => {},
    /* 57 */ InstructionType::ELPM => {},
    /* 58 */ InstructionType::EOR => { process_eor(&mut rdr, &word, &mut index, &mut value_storage); },
    /* 59 */ InstructionType::FMUL => {},
    /* 60 */ InstructionType::FMULS => {},
    /* 61 */ InstructionType::FMULSU => {},
    /* 62 */ InstructionType::ICALL => {},
    /* 63 */ InstructionType::IJMP => {},
    /* 64 */ InstructionType::IN => {},
    /* 65.*/ InstructionType::INC => {},
    /* 66 */ InstructionType::JMP => { process_jmp(&mut rdr, &word, &mut index, &mut value_storage); },
    /* 67 */ InstructionType::LAC => {},
    /* 68 */ InstructionType::LAS => {},
    /* 69 */ InstructionType::LAT => {}, 
    /* 70 */ InstructionType::LD => {},
    /* 71 */ InstructionType::LD_LDD_Y => {},
    /* 72 */ InstructionType::LD_LDD_Z => {},
    /* 73 */ InstructionType::LDI => { process_ldi(&mut rdr, &word, &mut index, &mut value_storage); },
    /* 74 */ InstructionType::LDS => {},
    /* 75 */ InstructionType::LDS_16bit => {}, // (16-bit) – Load Direct from Data Space......................................................... 117
    /* 76 */ InstructionType::LPM => {}, // – Load Program Memory...............................................................................118
    /* 77 */ InstructionType::LSL => {}, // – Logical Shift Left..........................................................................................120
    /* 78 */ InstructionType::LSR => {}, // – Logical Shift Right.......................................................................................122
    /* 79 */ InstructionType::MOV => {}, // – Copy Register............................................................................................123
    /* 80 */ InstructionType::MOVW => {}, // – Copy Register Word...............................................................................124
    /* 81 */ InstructionType::MUL => {}, // – Multiply Unsigned.......................................................................................125
    /* 82 */ InstructionType::MULS => {}, // – Multiply Signed........................................................................................ 126
    /* 83 */ InstructionType::MULSU  => {}, //
    /* 84 */ InstructionType::NEG => {},
    /* 85 */ InstructionType::NOP => {},
    /* 86 */ InstructionType::OR => {},
    /* 87 */ InstructionType::ORI => {},
    /* 88 */ InstructionType::OUT => { process_out(&mut rdr, &word, &mut index, &mut value_storage); },
    /* 89 */ InstructionType::POP => {}, // – Pop Register from Stack............................................................................ 135
    /* 90 */ InstructionType::PUSH => {}, // – Push Register on Stack........................................................................... 136
    /* 91 */ InstructionType::RCALL => {}, // – Relative Call to Subroutine.................................................................... 137
    /* 92 */ InstructionType::RET => {}, // – Return from Subroutine.............................................................................. 139
    /* 93 */ InstructionType::RETI => {}, // – Return from Interrupt................................................................................. 140
    /* 94 */ InstructionType::RJMP => {}, // – Relative Jump.......................................................................................... 142
    /* 95 */ InstructionType::ROL => {}, //
    /* 96 */ InstructionType::ROR => {}, // – Rotate Right through Carry........................................................................145
    /* 97 */ InstructionType::SBC => {}, // – Subtract with Carry.....................................................................................147
    /* 98 */ InstructionType::SBCI => {}, // – Subtract Immediate with Carry SBI – Set Bit in I/O Register.................... 149
    /* 99 */ InstructionType::SBI => {}, // – Set Bit in I/O Register.................................................................................. 151
    /* 100 */ InstructionType::SBIC => {}, // – Skip if Bit in I/O Register is Cleared........................................................ 152
    /* 101 */ InstructionType::SBIS => {}, // – Skip if Bit in I/O Register is Set............................................................... 153
    /* 102 */ InstructionType::SBIW => {}, // – Subtract Immediate from Word...............................................................154
    /* 103 */ InstructionType::SBR => {}, // – Set Bits in Register...................................................................................156
    /* 104 */ InstructionType::SBRC => {}, // – Skip if Bit in Register is Cleared............................................................ 157
    /* 105 */ InstructionType::SBRS => {}, // – Skip if Bit in Register is Set....................................................................158
    /* 106 */ InstructionType::SEC => {}, // – Set Carry Flag.......................................................................................... 159
    /* 107 */ InstructionType::SEH => {}, // – Set Half Carry Flag...................................................................................160
    /* 108 */ InstructionType::SEI => {}, //
    /* 109 */ InstructionType::SEN => {}, // – Set Negative Flag.....................................................................................162
    /* 110 */ InstructionType::SER => {}, // – Set all Bits in Register.............................................................................. 163
    /* 111 */ InstructionType::SES => {}, // – Set Signed Flag........................................................................................ 164
    /* 112 */ InstructionType::SET => {}, // – Set T Flag................................................................................................. 165
    /* 113 */ InstructionType::SEV => {}, // – Set Overflow Flag..................................................................................... 166
    /* 114 */ InstructionType::SEZ => {}, // – Set Zero Flag............................................................................................ 167
    /* 115 */ InstructionType::SLEEP => {}, //................................................................................................................. 168
    /* 116 */ InstructionType::SPM => {}, // – Store Program Memory............................................................................169
    /* 117 */ InstructionType::SPM_2 => {}, // SPM #2 – Store Program Memory.......................................................................171
    /* 118 */ InstructionType::ST => {}, // – Store Indirect From Register to Data Space using Index X.........................173
    /* 119 */ InstructionType::ST_STD_Y => {}, // – Store Indirect From Register to Data Space using Index Y..............175
    /* 120 */ InstructionType::ST_STD_Z => {}, 
    /* 121 */ InstructionType::STS => {}, // STS – Store Direct to Data Space.......................................................................179
    /* 122 */ InstructionType::STS_16bit => {}, // STS (16-bit) – Store Direct to Data Space.......................................................... 180
    /* 123 */ InstructionType::SUB => {}, // – Subtract Without Carry.............................................................................181
    /* 124 */ InstructionType::SUBI => {}, // – Subtract Immediate................................................................................. 183
    /* 125 */ InstructionType::SWAP => {}, // – Swap Nibbles........................................................................................ 185
    /* 126 */ InstructionType::TST => {}, // – Test for Zero or Minus...............................................................................186
    /* 127 */ InstructionType::WDR => {}, // – Watchdog Reset......................................................................................187
    /* 128 */ InstructionType::XCH => {}, // 
    InstructionType::Unknown => {},
            _ => {}
        }

        
/*
        if bit_match(word, "1001010xxxxx110x") {

            // 1001 010k kkkk 110k
            // kkkk kkkk kkkk kkkk

            log::trace!("JMP  = 66. JMP – Jump");
            log::trace!("{word:#b}");

            // build k - parse out all occurences of the k bits and combine them into k
            log::trace!("wword: {:b}", wword);
            let mut k_hi:u32 = ((wword & 0b0000000111110000u32) >> 3u16).into();
            log::trace!("k_hi: {:b}", k_hi);
            k_hi       = k_hi | (wword & 0b0000000000000001u32);
            log::trace!("k_hi: {:b}", k_hi);
            
            // read the next two byte of the 32 bit instruction 
            let k_lo:u32 = (rdr.read_u16::<LittleEndian>().unwrap()).into();
            index += 2;
            log::trace!("k_lo: {:b}", k_lo);
            
            let k:u32 = (k_hi << 16u8) + k_lo;

            log::trace!("k: {:#06x}", k);

            // since the amount of elements to jump are words, to find the address, multiply by two
            log::trace!("k: {:#06x}", k*2);

            // let mut var_storage:HashMap<char, u16> = HashMap::new();
            // bit_pattern_match(word, "1001010kkkkk110k", &mut var_storage);

            // let k_val = var_storage[&'k'];
            // log::info!("k: {}", k_val);

            log::info!("{:#02x}: {word:#06x} {k_lo:#06x} jmp {:#06x}", index-4, k*2);

        } else if bit_match(word, "10111xxxxxxxxxxx") {

            //log::info!("OUT  = 88. OUT – Store Register to I/O Location");
            
            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  10111AArrrrrAAAA");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "10111AArrrrrAAAA", &mut var_storage);

            let a_val = var_storage[&'A'];
            //log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

            log::info!("{:#02x}: {word:#06x} out {a_val:#02x} r{r_val}", index-2);

        }  else if bit_match(word, "10110xxxxxxxxxxx") {

            //log::info!("IN   = 64. IN - Load an I/O Location to Register");

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  10110AAdddddAAAA");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "10110AAdddddAAAA", &mut var_storage);

            let a_val = var_storage[&'A'];
            //log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

            log::info!("{:#02x}: {word:#06x} in r{d_val:#02} {a_val:#02x}", index-2);

        } else if bit_match(word, "1110xxxxxxxxxxxx") {

            //log::info!("LDI  = 73. LDI – Load Immediate");

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1110KKKKddddKKKK");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1110KKKKddddKKKK", &mut var_storage);

            let k_val = var_storage[&'K'];
            //log::info!("K: {k_val:#b} {k_val:#x}");
            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x}");

            // "Loads an 8-bit constant directly to register 16 to 31."
            // To compute the register to use, add the offset 16 to the parsed value
            let register = d_val + 16;
            //log::info!("[LDI] Using register: r{}", register);

            log::info!("{:#02x}: {word:#06x} ldi r{register:#02} {k_val:#02x}", index-2);

        } else if bit_match(word, "1001010xxxxx111x") {

            //log::info!("CALL = 36. CALL – Long Call to a Subroutine");

            // 1001 010k kkkk 111k
            // kkkk kkkk kkkk kkkk

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1001010kkkkk111k");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1001010kkkkk111k", &mut var_storage);

            let k_hi:u32 = var_storage[&'k'].into();
            //log::info!("k: {k_hi:#b} {k_hi:#x} {k_hi}");

            // read the next two byte of the 32 bit instruction 
            let k_lo:u32 = (rdr.read_u16::<LittleEndian>().unwrap()).into();
            index += 2;
            log::trace!("k_lo: {:b}", k_lo);
            
            let k:u32 = (k_hi << 16u8) + k_lo;

            log::trace!("k: {:#06x}", k);

            // since the amount of elements to jump are words, to find the address, multiply by two
            //log::info!("k: {:#06x}", k*2);

            let addr_value = k*2;

            log::info!("{:#02x}: {word:#06x} {k_lo:#06x} call {addr_value:#02x}", index-4);

        } else if bit_match(word, "001001xxxxxxxxxx") {

            //log::info!("EOR  = 58. EOR – Exclusive OR");
            
            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  001001rdddddrrrr");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "001001rdddddrrrr", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x}");
            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x}");

            log::info!("{:#02x}: {word:#06x} eor r{r_val} r{d_val}", index-2);

        } else if bit_match(word, "0110xxxxxxxxxxxx") {

            //log::info!("ORI  = 87. ORI – Logical OR with Immediate");

            // 0110 KKKK dddd KKKK

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  0110KKKKddddKKKK");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "0110KKKKddddKKKK", &mut var_storage);

            let k_val = var_storage[&'K'];
            //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

        } else if bit_match(word, "0101xxxxxxxxxxxx") {

            //log::info!("SUBI = 124. SUBI – Subtract Immediate");

            // 0101 KKKK dddd KKKK

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  0101KKKKddddKKKK");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "0101KKKKddddKKKK", &mut var_storage);

            let k_val = var_storage[&'K'];
            //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

        } else if bit_match(word, "0100xxxxxxxxxxxx") {

            //log::info!("SBCI = 98. SBCI – Subtract Immediate with Carry SBI – Set Bit in I/O Register");

            // 0100 KKKK dddd KKKK

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  0100KKKKddddKKKK");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "0100KKKKddddKKKK", &mut var_storage);

            let k_val = var_storage[&'K'];
            //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

        } else if bit_match(word, "111101xxxxxxx001") {

            //log::info!("BRNE = 27. BRNE – Branch if Not Equal");

            // 1111 01kk kkkk k001

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  111101kkkkkkk001");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "111101kkkkkkk001", &mut var_storage);

            let k_val = var_storage[&'k'];
            //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");

        } else if bit_match(word, "1100xxxxxxxxxxxx") {

            //log::info!("RJMP = 94. RJMP – Relative Jump");

            // 1100 kkkk kkkk kkkk

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1100kkkkkkkkkkkk");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1100kkkkkkkkkkkk", &mut var_storage);

            let k_val = var_storage[&'k'];
            //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");

        } else if bit_match(word, "0000000000000000") {

            //log::info!("NOP  = 85. NOP – No Operation");

            // 0000 0000 0000 0000

        } else if bit_match(word, "0111xxxxxxxxxxxx") {

            //log::info!("ANDI = 9. ANDI – Logical AND with Immediate");

            // 0111 KKKK dddd KKKK

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  0111KKKKddddKKKK");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "0100KKKKddddKKKK", &mut var_storage);

            let k_val = var_storage[&'K'];
            //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

        } else if bit_match(word, "1001010011111000") {

            //log::info!("CLI  = 41. CLI – Clear Global Interrupt Flag");

            // 1001 0100 1111 1000

        } else if bit_match(word, "1001001xxxxx1111") {

            //log::info!("PUSH  = 90. PUSH – Push Register on Stack");
            
            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1001001ddddd1111");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1001001ddddd1111", &mut var_storage);

            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

            let register = d_val;

            log::info!("{:#02x}: {word:#06x} push r{register:#02}", index-2);
            
        } else if bit_match(word, "1101xxxxxxxxxxxx") {

            //log::info!("RCALL  = 91. RCALL – Relative Call to Subroutine");
            
            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1101kkkkkkkkkkkk");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1101kkkkkkkkkkkk", &mut var_storage);

            let k_val = var_storage[&'k'];
            //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");

            log::info!("{:#02x}: {word:#06x} rcall .+{k_val:#02}", index-2);
            
        } else if bit_match(word, "10010110xxxxxxxx") {

            //log::info!("ADIW  = 7. ADIW – Add Immediate to Word");
            //log::info!("{word:#b}");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "10010110kkddkkkk", &mut var_storage);

            let k_val = var_storage[&'k'];
            let d_val = var_storage[&'d'];

            //log::info!("k: {}, d: {}", k_val, d_val);
            
        } else if bit_match(word, "0011xxxxxxxxxxxx") {

            //log::info!("CPI  = 51. CPI – Compare with Immediate");

            // 0011 KKKK dddd KKKK

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  0011KKKKddddKKKK");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "0011KKKKddddKKKK", &mut var_storage);

            let k_val = var_storage[&'K'];
            //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
            
        } else if bit_match(word, "000001xxxxxxxxxx") {

            //log::info!("CPC   = 50. CPC – Compare with Carry");

            // 0000 01rd dddd rrrr

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  000001rdddddrrrr");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "000001rdddddrrrr", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
            
        } else if bit_match(word, "111100xxxxxxx100") {

            //log::info!("BRLT   = 25. BRLT – Branch if Less Than (Signed)");

            // 1111 00kk kkkk k100

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  111100kkkkkkk100");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "111100kkkkkkk100", &mut var_storage);

            let k_val = var_storage[&'k'];
            //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
            
        } else if bit_match(word, "1001000xxxxx1111") {

            //log::info!("POP   = 89. POP – Pop Register from Stack");

            // 1001 000d dddd 1111

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1001000ddddd1111");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1001000ddddd1111", &mut var_storage);

            let d_val = var_storage[&'d'];
            //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
            
        } else if bit_match(word, "1001010100001000") {

            //log::info!("RET   = 92. RET – Return from Subroutine");

            // 1001 0101 0000 1000
            
        } else if bit_match(word, "1000001xxxxx1000") {

            log::info!("STD Y (A) = 119. ST (STD) – Store Indirect From Register to Data Space using Index Y");

            // 1000 001r rrrr 1000

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1000001rrrrr1000");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1000001rrrrr1000", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
            
        } else if bit_match(word, "1001001xxxxx1001") {

            //log::info!("STD Y (B) = 119. ST (STD) – Store Indirect From Register to Data Space using Index Y");

            // 1001 001r rrrr 1001

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1001001rrrrr1001");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1001001rrrrr1001", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
            
        } else if bit_match(word, "1001001xxxxx1010") {

            //log::info!("STD Y (C) = 119. ST (STD) – Store Indirect From Register to Data Space using Index Y");

            // 1001 001r rrrr 1010

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1001001rrrrr1010");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1001001rrrrr1010", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
            
        } else if bit_match(word, "10x0xx1xxxxx1xxx") {

            //log::info!("STD Y (D) = 119. ST (STD) – Store Indirect From Register to Data Space using Index Y");

            // 10q0 qq1r rrrr 1qqq

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  10q0qq1rrrrr1qqq");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "10q0qq1rrrrr1qqq", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
            let q_val = var_storage[&'q'];
            //log::info!("k: {q_val:#b} {q_val:#x} {q_val}");
            
        } else if bit_match(word, "1000001xxxxx0000") {

            //log::info!("STD Z (A) = 120. ST (STD) – Store Indirect From Register to Data Space using Index Z");

            // 1000 001r rrrr 0000

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1000001rrrrr0000");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1000001rrrrr0000", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
            
        } else if bit_match(word, "1001001xxxxx0001") {

            //log::info!("STD Z (B) = 120. ST (STD) – Store Indirect From Register to Data Space using Index Z");

            // 1001 001r rrrr 0001

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1001001rrrrr0001");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1001001rrrrr0001", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
            
        } else if bit_match(word, "1001001xxxxx0010") {

            //log::info!("STD Z (C) = 120. ST (STD) – Store Indirect From Register to Data Space using Index Z");

            // 1001 001r rrrr 0010

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  1001001rrrrr0010");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "1001001rrrrr0010", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
            
        } else if bit_match(word, "10x0xx1xxxxx0xxx") {

            //log::info!("STD Z (D) = 120. ST (STD) – Store Indirect From Register to Data Space using Index Z");

            // 10q0 qq1r rrrr 0qqq

            //log::info!("{word:#018b} {word:#02x}");
            //log::info!("  10q0qq1rrrrr0qqq");

            let mut var_storage:HashMap<char, u16> = HashMap::new();
            bit_pattern_match(word, "10q0qq1rrrrr0qqq", &mut var_storage);

            let r_val = var_storage[&'r'];
            //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
            let q_val = var_storage[&'q'];
            //log::info!("k: {q_val:#b} {q_val:#x} {q_val}");
            
        } else {
            //log::error!("UNKNOWN");
            log::error!("UNKNOWN word: {:#06x} {:b}", word, word);
        }

        // else if bit_match(word, "xxxxxxxxxxxxxxxx") {
        // } 

        //break;
        */
    }
 

    // 56. EIJMP – Extended Indirect Jump
    // 63. IJMP – Indirect Jump
    // 66. JMP – Jump
    // 0C, 94, 34, 00
    // 00001100 10010100 00110100 00000000
    // kkkk110k 1001010k kkkkkkkk kkkkkkkk
    // 0000   0        0 00110100 00000000 -> 34 (per konvention the jump counts words so this means 68 bytes)

    // 10010100 00001100 00000000 00110100 94 0C 
    //        k kkkk   k kk 
    // 1001010k kkkk110k kkkkkkkk kkkkkkkk
    //                     kkkkkk kkkkkkkk
    //        0 0000   0 00000000 00110100

    // 00000000 00110100 10010100 00001100
    // kkkkkkkk kkkkkkkk 1001010k kkkk110k

    // 58. EOR – Exclusive OR
    // 11 24
    // 00010001 00100100
    //
    // 0010 0100 0001 0001
    // 0010 01rd dddd rrrr
    //        0       0001 - 1
    //         0 0001      - 1

    // 88. OUT – Store Register to I/O Location
    // 1f be
    // 00011111 10111110
    // 1011 1110 0001 1111
    // 1011 1AAr rrrr AAAA
    //       11       1111 - 3f
    //         0 0001      -  1


    log_end();

    Ok(())
}

fn init_logging() {
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        // https://stackoverflow.com/questions/61810740/log-source-file-and-line-numbers
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

fn log_start() {
    log::trace!("Application starts ...");
    log::debug!("Application starts ...");
    log::info!("Application starts ...");
    log::warn!("Application starts ...");
    log::error!("Application starts ...");
}

fn log_end() {
    log::trace!("Application terminates.");
    log::debug!("Application terminates.");
    log::info!("Application terminates.");
    log::warn!("Application terminates.");
    log::error!("Application terminates.");
}

fn process_eor<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("EOR  = 58. EOR – Exclusive OR");
            
    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  001001rdddddrrrr");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "001001rdddddrrrr", &mut var_storage);

    let r_val = value_storage[&'r'];
    //log::info!("r: {r_val:#b} {r_val:#x}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x}");

    log::info!("{:#02x}: {word:#06x} eor r{r_val}, r{d_val}", *index-2usize);
}

fn process_out<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("OUT  = 88. OUT – Store Register to I/O Location");
            
    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  10111AArrrrrAAAA");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "10111AArrrrrAAAA", &mut var_storage);

    let a_val = value_storage[&'A'];
    //log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
    let r_val = value_storage[&'r'];
    //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

    log::info!("{:#02x}: {word:#06x} out {a_val:#02x}, r{r_val}", *index-2usize);
}

fn process_jmp<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    log::trace!("JMP  = 66. JMP – Jump");
    log::trace!("{word:#b}");

    let k_hi:u32 = value_storage[&'k'].into();
    log::trace!("k_hi: {}", k_hi);

    // read the next two byte of the 32 bit instruction 
    let k_lo:u32 = (rdr.read_u16::<LittleEndian>().unwrap()).into();
    //index = index + (usize)2;
    //index += 2;
    *index = *index + 2usize;
    log::trace!("k_lo: {:b}", k_lo);
    
    let k:u32 = (k_hi << 16u8) + k_lo;

    log::trace!("k: {:#06x}", k);

    // since the amount of elements to jump are words, to find the address, multiply by two
    log::trace!("k: {:#06x}", k*2);

    log::info!("{:#02x}: {word:#06x} {k_lo:#06x} jmp {:#06x}", *index-4usize, k*2);

    // // build k - parse out all occurences of the k bits and combine them into k
    // log::trace!("wword: {:b}", wword);
    // let mut k_hi:u32 = ((wword & 0b0000000111110000u32) >> 3u16).into();
    // log::trace!("k_hi: {:b}", k_hi);
    // k_hi       = k_hi | (wword & 0b0000000000000001u32);
    // log::trace!("k_hi: {:b}", k_hi);
    
    // // read the next two byte of the 32 bit instruction 
    // let k_lo:u32 = (rdr.read_u16::<LittleEndian>().unwrap()).into();
    // index += 2;
    // log::trace!("k_lo: {:b}", k_lo);
    
    // let k:u32 = (k_hi << 16u8) + k_lo;

    // log::trace!("k: {:#06x}", k);

    // // since the amount of elements to jump are words, to find the address, multiply by two
    // log::trace!("k: {:#06x}", k*2);

    // // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // // bit_pattern_match(word, "1001010kkkkk110k", &mut var_storage);

    // // let k_val = var_storage[&'k'];
    // // log::info!("k: {}", k_val);

    // log::info!("{:#02x}: {word:#06x} {k_lo:#06x} jmp {:#06x}", index-4, k*2);
}

fn process_ldi<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("LDI  = 73. LDI – Load Immediate");

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1110KKKKddddKKKK");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1110KKKKddddKKKK", &mut var_storage);

    let k_val = value_storage[&'K'];
    //log::info!("K: {k_val:#b} {k_val:#x}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x}");

    // "Loads an 8-bit constant directly to register 16 to 31."
    // To compute the register to use, add the offset 16 to the parsed value
    let register = d_val + 16;
    //log::info!("[LDI] Using register: r{}", register);

    log::info!("{:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}", *index-2usize);
}

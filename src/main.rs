mod ihex_mgmt;
mod file_mgmt;

use std::io;
use std::io::Write;
use std::io::Cursor;
use std::collections::HashMap;

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
        if c == 'x' {

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

fn main() -> io::Result<()> {
    println!("whatavr starting ...");

    init_logging();
    log_start();

    //
    // load hex file
    //

    let mut hex_file_path:String = String::new();
    //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank1.hex");
    //hex_file_path.push_str("C:/aaa_se/rust/rust_blt_2/test_resources/output_bank2.hex") {
    //hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication1/GccApplication1.hex");
    hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication2/GccApplication1.hex");

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

            //log::info!("STD Y (A) = 119. ST (STD) – Store Indirect From Register to Data Space using Index Y");

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
            
        }
        
        // else if bit_match(word, "xxxxxxxxxxxxxxxx") {
        // } 
        else {
            //log::error!("UNKNOWN");
            log::error!("UNKNOWN word: {:#06x} {:b}", word, word);
        }

        //break;
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

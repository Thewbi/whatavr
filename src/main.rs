mod ihex_mgmt;
mod file_mgmt;

use std::io;
use std::io::Write;
use std::io::Cursor;

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
            if shift_counter > 0 {
                shift_counter = shift_counter - 1;
            }
            bit_mask = bit_mask >> 1;

            continue;
        }

        let temp = (data & bit_mask) >> shift_counter;
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
    hex_file_path.push_str("C:/aaa_se/rust/whatavr/test_resources/sample_files/GccApplication1/GccApplication1.hex");

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

    let mut rdr = Cursor::new(&segment_0.data);
    while index < segment_0.data.len()
    {
        let word:u16 = rdr.read_u16::<LittleEndian>().unwrap().into();
        index += 2;

        let wword:u32 = word.into();

        log::trace!("word: {:#06x} {:b}", word, word);

        if bit_match(word, "1001010xxxxx110x") {

            log::info!("JMP  = 66. JMP – Jump");

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

            //log::info!("JMP {:#06x}", (k*2));

        } else if bit_match(word, "10111xxxxxxxxxxx") {

            log::info!("OUT  = 88. OUT – Store Register to I/O Location");

        }  else if bit_match(word, "10110xxxxxxxxxxx") {

            log::info!("IN   = 64. IN - Load an I/O Location to Register");

        } else if bit_match(word, "1110xxxxxxxxxxxx") {

            log::info!("LDI  = 73. LDI – Load Immediate");

        } else if bit_match(word, "1001010xxxxx111x") {

            log::info!("CALL = 36. CALL – Long Call to a Subroutine");

        } else if bit_match(word, "001001xxxxxxxxxx") {

            log::info!("EOR  = 58. EOR – Exclusive OR");

        } else if bit_match(word, "0110xxxxxxxxxxxx") {

            log::info!("ORI  = 87. ORI – Logical OR with Immediate");

        } else if bit_match(word, "0101xxxxxxxxxxxx") {

            log::info!("SUBI = 124. SUBI – Subtract Immediate");

        } else if bit_match(word, "0100xxxxxxxxxxxx") {

            log::info!("SBCI = 98. SBCI – Subtract Immediate with Carry SBI – Set Bit in I/O Register");

        } else if bit_match(word, "111101xxxxxxx001") {

            log::info!("BRNE = 27. BRNE – Branch if Not Equal");

        } else if bit_match(word, "1100xxxxxxxxxxxx") {

            log::info!("RJMP = 94. RJMP – Relative Jump");

        } else if bit_match(word, "0000000000000000") {

            log::info!("NOP  = 85. NOP – No Operation");

        } else if bit_match(word, "0111xxxxxxxxxxxx") {

            log::info!("ANDI = 9. ANDI – Logical AND with Immediate");

        } else if bit_match(word, "1001010011111000") {

            log::info!("CLI  = 41. CLI – Clear Global Interrupt Flag");

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

mod ihex_mgmt;
mod file_mgmt;
mod instructions;

use std::io;
use std::io::Write;
use std::io::Cursor;
use std::collections::HashMap;

use env_logger::{Builder, Target};
use instructions::bit_pattern_matching::bit_match;
use instructions::bit_pattern_matching::bit_pattern_match;
use instructions::instruction_definition::InstructionDefinition;
use log::LevelFilter;

use crate::ihex_mgmt::ihex_mgmt::Segment;
use crate::ihex_mgmt::ihex_mgmt::parse_hex_file;
use crate::instructions::instruction_type::InstructionType;
use crate::instructions::instructions::instructions;
use crate::instructions::instructions::unknown;
use crate::instructions::process::*;

use byteorder::{LittleEndian, ReadBytesExt};




/// finds a command that matches data and returns that instruction definition
/// If no matching instruction is found, returns the instruction definition 'unknown'
//fn decode<'a>(data:u16, instructions_list: &'a Vec<InstructionDefinition>, unknown_instruction: &'a InstructionDefinition, value_storage:&mut HashMap<char, u16>) -> &'a InstructionDefinition {
fn decode<'a>(data:u16, instructions_list: &'static [InstructionDefinition], unknown_instruction: &'a InstructionDefinition, value_storage:&mut HashMap<char, u16>) -> &'a InstructionDefinition {

    for instruction in instructions_list 
    {
        if bit_match(data, &instruction.bit_pattern)
        {
            bit_pattern_match(data, &instruction.bit_pattern, value_storage);

            return &instruction;
        }
    }
     
    return unknown_instruction;
}

fn main() -> io::Result<()> {

    println!("whatavr starting ...");

    // logging setup
    init_logging();
    log_start();

    // load hex file
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

        log::trace!("word: {:#06x} {:b}", word, word);

        let mut value_storage:HashMap<char, u16> = HashMap::new();
        let instruction: &InstructionDefinition = decode(word, instructions, &unknown, &mut value_storage);

        log::info!("instruction {:?}", instruction.instruction_type);
        if instruction.instruction_type == InstructionType::EOR || instruction.instruction_type == InstructionType::CLR {
            log::info!("EOR and CLR similar. CLI is implemented by EOR the register with itself!");
        }

        match instruction.instruction_type {

            /*   5 */ InstructionType::ADC          => { process_adc(&mut rdr, &word, &mut index, &mut value_storage); },
            /*   6 */ InstructionType::ADD          => { process_add(&mut rdr, &word, &mut index, &mut value_storage); },
            /*   7 */ InstructionType::ADIW         => { process_adiw(&mut rdr, &word, &mut index, &mut value_storage); },
            /*   8 */ InstructionType::AND          => { process_and(&mut rdr, &word, &mut index, &mut value_storage); },
            /*   9 */ InstructionType::ANDI         => { process_andi(&mut rdr, &word, &mut index, &mut value_storage); }
            /*  10 */ InstructionType::ASR          => { process_asr(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  11 */ InstructionType::BCLR         => { process_bclr(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  12 */ InstructionType::BLD          => { process_bld(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  13 */ InstructionType::BRBC         => { process_brbc(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  14 */ InstructionType::BRBS         => { process_brbs(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  15 */ InstructionType::BRCC         => { process_brcc(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  16 */ InstructionType::BRCS         => { process_brcs(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  17 */ InstructionType::BREAK        => { process_break(&mut rdr, &word, &mut index, &mut value_storage); }, 
            /*  18 */ InstructionType::BREQ         => { process_breq(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  19 */ InstructionType::BRGE         => { process_brge(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  20 */ InstructionType::BRHC         => { process_brhc(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  21 */ InstructionType::BRHS         => { process_brhs(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  22 */ InstructionType::BRID         => { process_brid(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  23 */ InstructionType::BRIE         => { process_brie(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  24 */ InstructionType::BRLO         => { process_brlo(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  25 */ InstructionType::BRLT         => { process_brlt(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  26 */ InstructionType::BRMI         => { process_brmi(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  27 */ InstructionType::BRNE         => { process_brne(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  28 */ InstructionType::BRPL         => { process_brpl(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  29 */ InstructionType::BRSH         => { process_brsh(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  30 */ InstructionType::BRTC         => { process_brtc(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  31 */ InstructionType::BRTS         => { process_brts(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  32 */ InstructionType::BRVC         => { process_brvc(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  33 */ InstructionType::BRVS         => { process_brvs(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  34 */ InstructionType::BSET         => { process_bset(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  35 */ InstructionType::BST          => { process_bst(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  36 */ InstructionType::CALL         => { process_call(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  37 */ InstructionType::CBI          => { process_cbi(&mut rdr, &word, &mut index, &mut value_storage); }, 
            /*  38 */ InstructionType::CBR          => { process_cbr(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  39 */ InstructionType::CLC          => { process_clc(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  40 */ InstructionType::CLH          => { process_clh(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  41 */ InstructionType::CLI          => { process_cli(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  42 */ InstructionType::CLN          => { process_cln(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  43 */ InstructionType::CLR          => { process_clr(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  44 */ InstructionType::CLS          => { process_cls(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  45 */ InstructionType::CLT          => { process_clt(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  46 */ InstructionType::CLV          => { process_clv(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  47 */ InstructionType::CLZ          => { process_clz(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  48 */ InstructionType::COM          => { process_com(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  49 */ InstructionType::CP           => { process_cp(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  50 */ InstructionType::CPC          => { process_cpc(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  51 */ InstructionType::CPI          => { process_cpi(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  52 */ InstructionType::CPSE         => { process_cpse(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  53 */ InstructionType::DEC          => { process_dec(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  54 */ InstructionType::DES          => { process_des(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  55 */ InstructionType::EICALL       => { process_eicall(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  56 */ InstructionType::EIJMP        => { process_eijmp(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  57 */ InstructionType::ELPM         => { process_elpm(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  58 */ InstructionType::EOR          => { process_eor(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  59 */ InstructionType::FMUL         => { process_fmul(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  60 */ InstructionType::FMULS        => { process_fmuls(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  61 */ InstructionType::FMULSU       => { process_fmulsu(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  62 */ InstructionType::ICALL        => { process_icall(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  63 */ InstructionType::IJMP         => { process_ijmp(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  64 */ InstructionType::IN           => { process_in(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  65 */ InstructionType::INC          => { process_inc(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  66 */ InstructionType::JMP          => { process_jmp(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  67 */ InstructionType::LAC          => { process_lac(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  68 */ InstructionType::LAS          => { process_las(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  69 */ InstructionType::LAT          => { process_lat(&mut rdr, &word, &mut index, &mut value_storage); }, 
            /*  70 */ InstructionType::LD           => { process_ld(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  71 */ InstructionType::LD_LDD_Y     => { process_ld_ldd_y(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  72 */ InstructionType::LD_LDD_Z     => { process_ld_ldd_z(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  73 */ InstructionType::LDI          => { process_ldi(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  74 */ InstructionType::LDS          => { process_lds(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  75 */ InstructionType::LDS_16bit    => { process_lds_16bit(&mut rdr, &word, &mut index, &mut value_storage); }, // (16-bit) – Load Direct from Data Space......................................................... 117
            /*  76 */ InstructionType::LPM          => { process_lpm(&mut rdr, &word, &mut index, &mut value_storage); }, // – Load Program Memory...............................................................................118
            /*  77 */ InstructionType::LSL          => { process_lsl(&mut rdr, &word, &mut index, &mut value_storage); }, // – Logical Shift Left..........................................................................................120
            /*  78 */ InstructionType::LSR          => { process_lsr(&mut rdr, &word, &mut index, &mut value_storage); }, // – Logical Shift Right.......................................................................................122
            /*  79 */ InstructionType::MOV          => { process_mov(&mut rdr, &word, &mut index, &mut value_storage); }, // – Copy Register............................................................................................123
            /*  80 */ InstructionType::MOVW         => { process_movw(&mut rdr, &word, &mut index, &mut value_storage); }, // – Copy Register Word...............................................................................124
            /*  81 */ InstructionType::MUL          => { process_mul(&mut rdr, &word, &mut index, &mut value_storage); }, // – Multiply Unsigned.......................................................................................125
            /*  82 */ InstructionType::MULS         => { process_muls(&mut rdr, &word, &mut index, &mut value_storage); }, // – Multiply Signed........................................................................................ 126
            /*  83 */ InstructionType::MULSU        => { process_mulsu(&mut rdr, &word, &mut index, &mut value_storage); }, //
            /*  84 */ InstructionType::NEG          => { process_neg(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  85 */ InstructionType::NOP          => { process_nop(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  86 */ InstructionType::OR           => { process_or(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  87 */ InstructionType::ORI          => { process_ori(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  88 */ InstructionType::OUT          => { process_out(&mut rdr, &word, &mut index, &mut value_storage); },
            /*  89 */ InstructionType::POP          => { process_pop(&mut rdr, &word, &mut index, &mut value_storage); }, // – Pop Register from Stack............................................................................ 135
            /*  90 */ InstructionType::PUSH         => { process_push(&mut rdr, &word, &mut index, &mut value_storage); }, // – Push Register on Stack........................................................................... 136
            /*  91 */ InstructionType::RCALL        => { process_rcall(&mut rdr, &word, &mut index, &mut value_storage); }, // – Relative Call to Subroutine.................................................................... 137
            /*  92 */ InstructionType::RET          => { process_ret(&mut rdr, &word, &mut index, &mut value_storage); }, // – Return from Subroutine.............................................................................. 139
            /*  93 */ InstructionType::RETI         => { process_reti(&mut rdr, &word, &mut index, &mut value_storage); }, // – Return from Interrupt................................................................................. 140
            /*  94 */ InstructionType::RJMP         => { process_rjmp(&mut rdr, &word, &mut index, &mut value_storage); }, // – Relative Jump.......................................................................................... 142
            /*  95 */ InstructionType::ROL          => { process_rol(&mut rdr, &word, &mut index, &mut value_storage); }, //
            /*  96 */ InstructionType::ROR          => { process_ror(&mut rdr, &word, &mut index, &mut value_storage); }, // – Rotate Right through Carry........................................................................145
            /*  97 */ InstructionType::SBC          => { process_sbc(&mut rdr, &word, &mut index, &mut value_storage); }, // – Subtract with Carry.....................................................................................147
            /*  98 */ InstructionType::SBCI         => { process_sbci(&mut rdr, &word, &mut index, &mut value_storage); }, // – Subtract Immediate with Carry SBI – Set Bit in I/O Register.................... 149
            /*  99 */ InstructionType::SBI          => { process_sbi(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set Bit in I/O Register.................................................................................. 151
            /* 100 */ InstructionType::SBIC         => { process_sbic(&mut rdr, &word, &mut index, &mut value_storage); }, // – Skip if Bit in I/O Register is Cleared........................................................ 152
            /* 101 */ InstructionType::SBIS         => { process_sbis(&mut rdr, &word, &mut index, &mut value_storage); }, // – Skip if Bit in I/O Register is Set............................................................... 153
            /* 102 */ InstructionType::SBIW         => { process_sbiw(&mut rdr, &word, &mut index, &mut value_storage); }, // – Subtract Immediate from Word...............................................................154
            /* 103 */ InstructionType::SBR          => { process_sbr(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set Bits in Register...................................................................................156
            /* 104 */ InstructionType::SBRC         => { process_sbrc(&mut rdr, &word, &mut index, &mut value_storage); }, // – Skip if Bit in Register is Cleared............................................................ 157
            /* 105 */ InstructionType::SBRS         => { process_sbrs(&mut rdr, &word, &mut index, &mut value_storage); }, // – Skip if Bit in Register is Set....................................................................158
            /* 106 */ InstructionType::SEC          => { process_sec(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set Carry Flag.......................................................................................... 159
            /* 107 */ InstructionType::SEH          => { process_seh(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set Half Carry Flag...................................................................................160
            /* 108 */ InstructionType::SEI          => { process_sei(&mut rdr, &word, &mut index, &mut value_storage); }, //
            /* 109 */ InstructionType::SEN          => { process_sen(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set Negative Flag.....................................................................................162
            /* 110 */ InstructionType::SER          => { process_ser(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set all Bits in Register.............................................................................. 163
            /* 111 */ InstructionType::SES          => { process_ses(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set Signed Flag........................................................................................ 164
            /* 112 */ InstructionType::SET          => { process_set(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set T Flag................................................................................................. 165
            /* 113 */ InstructionType::SEV          => { process_sev(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set Overflow Flag..................................................................................... 166
            /* 114 */ InstructionType::SEZ          => { process_sez(&mut rdr, &word, &mut index, &mut value_storage); }, // – Set Zero Flag............................................................................................ 167
            /* 115 */ InstructionType::SLEEP        => { process_sleep(&mut rdr, &word, &mut index, &mut value_storage); }, //................................................................................................................. 168
            /* 116 */ InstructionType::SPM          => { process_spm(&mut rdr, &word, &mut index, &mut value_storage); }, // – Store Program Memory............................................................................169
            /* 117 */ InstructionType::SPM_2        => { process_spm2(&mut rdr, &word, &mut index, &mut value_storage); }, // SPM #2 – Store Program Memory.......................................................................171
            /* 118 */ InstructionType::ST           => { process_st(&mut rdr, &word, &mut index, &mut value_storage); }, // – Store Indirect From Register to Data Space using Index X.........................173
            
            // /* 119 */ InstructionType::ST_STD_Y => { process_st_std_y(&mut rdr, &word, &mut index, &mut value_storage); }, // – Store Indirect From Register to Data Space using Index Y..............175
            InstructionType::ST_STD_Y_1             => { process_st_std_y_1(&mut rdr, &word, &mut index, &mut value_storage); },
            InstructionType::ST_STD_Y_2             => { process_st_std_y_2(&mut rdr, &word, &mut index, &mut value_storage); },
            InstructionType::ST_STD_Y_3             => { process_st_std_y_3(&mut rdr, &word, &mut index, &mut value_storage); },
            InstructionType::ST_STD_Y_4             => { process_st_std_y_4(&mut rdr, &word, &mut index, &mut value_storage); },

            // /* 120 */ InstructionType::ST_STD_Z => { process_st_std_z(&mut rdr, &word, &mut index, &mut value_storage); }, 
            InstructionType::ST_STD_Z_1             => { process_st_std_z_1(&mut rdr, &word, &mut index, &mut value_storage); },
            InstructionType::ST_STD_Z_2             => { process_st_std_z_2(&mut rdr, &word, &mut index, &mut value_storage); },
            InstructionType::ST_STD_Z_3             => { process_st_std_z_3(&mut rdr, &word, &mut index, &mut value_storage); },
            InstructionType::ST_STD_Z_4             => { process_st_std_z_4(&mut rdr, &word, &mut index, &mut value_storage); },
            
            /* 121 */ InstructionType::STS          => { process_sts(&mut rdr, &word, &mut index, &mut value_storage); }, // STS – Store Direct to Data Space.......................................................................179
            /* 122 */ InstructionType::STS_16bit    => { process_sts_16bit(&mut rdr, &word, &mut index, &mut value_storage); }, // STS (16-bit) – Store Direct to Data Space.......................................................... 180
            /* 123 */ InstructionType::SUB          => { process_sub(&mut rdr, &word, &mut index, &mut value_storage); }, // – Subtract Without Carry.............................................................................181
            /* 124 */ InstructionType::SUBI         => { process_subi(&mut rdr, &word, &mut index, &mut value_storage); }, // – Subtract Immediate................................................................................. 183
            /* 125 */ InstructionType::SWAP         => { process_swap(&mut rdr, &word, &mut index, &mut value_storage); }, // – Swap Nibbles........................................................................................ 185
            /* 126 */ InstructionType::TST          => { process_tst(&mut rdr, &word, &mut index, &mut value_storage); }, // – Test for Zero or Minus...............................................................................186
            /* 127 */ InstructionType::WDR          => { process_wdr(&mut rdr, &word, &mut index, &mut value_storage); }, // – Watchdog Reset......................................................................................187
            /* 128 */ InstructionType::XCH          => { process_xch(&mut rdr, &word, &mut index, &mut value_storage); }, // 
            InstructionType::Unknown => { panic!("Unknown instruction!"); },
            _ => { panic!("Unknown instruction!"); }
        }

    }

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

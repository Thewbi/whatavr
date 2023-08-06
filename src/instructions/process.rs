use byteorder::{LittleEndian, ReadBytesExt};

use crate::HashMap;

use super::{instruction_definition::InstructionDefinition, instruction_type::InstructionType};

#[allow(dead_code, unused)]
pub fn match_instruction<R: crate::io::Read>(
    instruction: &InstructionDefinition,
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    match instruction.instruction_type {
        /*   5 */
        InstructionType::ADC => {
            process_adc(rdr, &word, index, value_storage);
        }
        /*   6 */
        InstructionType::ADD => {
            process_add(rdr, &word, index, value_storage);
        }
        /*   7 */
        InstructionType::ADIW => {
            process_adiw(rdr, &word, index, value_storage);
        }
        /*   8 */
        InstructionType::AND => {
            process_and(rdr, &word, index, value_storage);
        }
        /*   9 */
        InstructionType::ANDI => {
            process_andi(rdr, &word, index, value_storage);
        }
        /*  10 */
        InstructionType::ASR => {
            process_asr(rdr, &word, index, value_storage);
        }
        /*  11 */
        InstructionType::BCLR => {
            process_bclr(rdr, &word, index, value_storage);
        }
        /*  12 */
        InstructionType::BLD => {
            process_bld(rdr, &word, index, value_storage);
        }
        /*  13 */
        InstructionType::BRBC => {
            process_brbc(rdr, &word, index, value_storage);
        }
        /*  14 */
        InstructionType::BRBS => {
            process_brbs(rdr, &word, index, value_storage);
        }
        /*  15 */
        InstructionType::BRCC => {
            process_brcc(rdr, &word, index, value_storage);
        }
        /*  16 */
        InstructionType::BRCS => {
            process_brcs(rdr, &word, index, value_storage);
        }
        /*  17 */
        InstructionType::BREAK => {
            process_break(rdr, &word, index, value_storage);
        }
        /*  18 */
        InstructionType::BREQ => {
            process_breq(rdr, &word, index, value_storage);
        }
        /*  19 */
        InstructionType::BRGE => {
            process_brge(rdr, &word, index, value_storage);
        }
        /*  20 */
        InstructionType::BRHC => {
            process_brhc(rdr, &word, index, value_storage);
        }
        /*  21 */
        InstructionType::BRHS => {
            process_brhs(rdr, &word, index, value_storage);
        }
        /*  22 */
        InstructionType::BRID => {
            process_brid(rdr, &word, index, value_storage);
        }
        /*  23 */
        InstructionType::BRIE => {
            process_brie(rdr, &word, index, value_storage);
        }
        /*  24 */
        InstructionType::BRLO => {
            process_brlo(rdr, &word, index, value_storage);
        }
        /*  25 */
        InstructionType::BRLT => {
            process_brlt(rdr, &word, index, value_storage);
        }
        /*  26 */
        InstructionType::BRMI => {
            process_brmi(rdr, &word, index, value_storage);
        }
        /*  27 */
        InstructionType::BRNE => {
            process_brne(rdr, &word, index, value_storage);
        }
        /*  28 */
        InstructionType::BRPL => {
            process_brpl(rdr, &word, index, value_storage);
        }
        /*  29 */
        InstructionType::BRSH => {
            process_brsh(rdr, &word, index, value_storage);
        }
        /*  30 */
        InstructionType::BRTC => {
            process_brtc(rdr, &word, index, value_storage);
        }
        /*  31 */
        InstructionType::BRTS => {
            process_brts(rdr, &word, index, value_storage);
        }
        /*  32 */
        InstructionType::BRVC => {
            process_brvc(rdr, &word, index, value_storage);
        }
        /*  33 */
        InstructionType::BRVS => {
            process_brvs(rdr, &word, index, value_storage);
        }
        /*  34 */
        InstructionType::BSET => {
            process_bset(rdr, &word, index, value_storage);
        }
        /*  35 */
        InstructionType::BST => {
            process_bst(rdr, &word, index, value_storage);
        }
        /*  36 */
        InstructionType::CALL => {
            process_call(rdr, &word, index, value_storage);
        }
        /*  37 */
        InstructionType::CBI => {
            process_cbi(rdr, &word, index, value_storage);
        }
        /*  38 */
        InstructionType::CBR => {
            process_cbr(rdr, &word, index, value_storage);
        }
        /*  39 */
        InstructionType::CLC => {
            process_clc(rdr, &word, index, value_storage);
        }
        /*  40 */
        InstructionType::CLH => {
            process_clh(rdr, &word, index, value_storage);
        }
        /*  41 */
        InstructionType::CLI => {
            process_cli(rdr, &word, index, value_storage);
        }
        /*  42 */
        InstructionType::CLN => {
            process_cln(rdr, &word, index, value_storage);
        }
        /*  43 */
        InstructionType::CLR => {
            process_clr(rdr, &word, index, value_storage);
        }
        /*  44 */
        InstructionType::CLS => {
            process_cls(rdr, &word, index, value_storage);
        }
        /*  45 */
        InstructionType::CLT => {
            process_clt(rdr, &word, index, value_storage);
        }
        /*  46 */
        InstructionType::CLV => {
            process_clv(rdr, &word, index, value_storage);
        }
        /*  47 */
        InstructionType::CLZ => {
            process_clz(rdr, &word, index, value_storage);
        }
        /*  48 */
        InstructionType::COM => {
            process_com(rdr, &word, index, value_storage);
        }
        /*  49 */
        InstructionType::CP => {
            process_cp(rdr, &word, index, value_storage);
        }
        /*  50 */
        InstructionType::CPC => {
            process_cpc(rdr, &word, index, value_storage);
        }
        /*  51 */
        InstructionType::CPI => {
            process_cpi(rdr, &word, index, value_storage);
        }
        /*  52 */
        InstructionType::CPSE => {
            process_cpse(rdr, &word, index, value_storage);
        }

        /*  53 */
        InstructionType::DEC => {
            process_dec(rdr, &word, index, value_storage);
        }
        /*  54 */
        InstructionType::DES => {
            process_des(rdr, &word, index, value_storage);
        }

        /*  55 */
        InstructionType::EICALL => {
            process_eicall(rdr, &word, index, value_storage);
        }
        /*  56 */
        InstructionType::EIJMP => {
            process_eijmp(rdr, &word, index, value_storage);
        }
        /*  57 */
        InstructionType::ELPM => {
            process_elpm(rdr, &word, index, value_storage);
        }
        /*  58 */
        InstructionType::EOR => {
            process_eor(rdr, &word, index, value_storage);
        }

        /*  59 */
        InstructionType::FMUL => {
            process_fmul(rdr, &word, index, value_storage);
        }
        /*  60 */
        InstructionType::FMULS => {
            process_fmuls(rdr, &word, index, value_storage);
        }
        /*  61 */
        InstructionType::FMULSU => {
            process_fmulsu(rdr, &word, index, value_storage);
        }

        /*  62 */
        InstructionType::ICALL => {
            process_icall(rdr, &word, index, value_storage);
        }
        /*  63 */
        InstructionType::IJMP => {
            process_ijmp(rdr, &word, index, value_storage);
        }
        /*  64 */
        InstructionType::IN => {
            process_in(rdr, &word, index, value_storage);
        }
        /*  65 */
        InstructionType::INC => {
            process_inc(rdr, &word, index, value_storage);
        }

        /*  66 */
        InstructionType::JMP => {
            process_jmp(rdr, &word, index, value_storage);
        }

        /*  67 */
        InstructionType::LAC => {
            process_lac(rdr, &word, index, value_storage);
        }
        /*  68 */
        InstructionType::LAS => {
            process_las(rdr, &word, index, value_storage);
        }
        /*  69 */
        InstructionType::LAT => {
            process_lat(rdr, &word, index, value_storage);
        }
        /*  70 */
        InstructionType::LD => {
            process_ld(rdr, &word, index, value_storage);
        }
        /*  71 */
        InstructionType::LD_LDD_Y => {
            process_ld_ldd_y(rdr, &word, index, value_storage);
        }
        /*  72 */
        InstructionType::LD_LDD_Z => {
            process_ld_ldd_z(rdr, &word, index, value_storage);
        }
        /*  73 */
        InstructionType::LDI => {
            process_ldi(rdr, &word, index, value_storage);
        }
        /*  74 */
        InstructionType::LDS => {
            process_lds(rdr, &word, index, value_storage);
        }
        /*  75 */
        InstructionType::LDS_16bit => {
            process_lds_16bit(rdr, &word, index, value_storage);
        } // (16-bit) – Load Direct from Data Space......................................................... 117
        /*  76 */
        InstructionType::LPM => {
            process_lpm(rdr, &word, index, value_storage);
        } // – Load Program Memory...............................................................................118
        /*  77 */
        InstructionType::LSL => {
            process_lsl(rdr, &word, index, value_storage);
        } // – Logical Shift Left..........................................................................................120
        /*  78 */
        InstructionType::LSR => {
            process_lsr(rdr, &word, index, value_storage);
        } // – Logical Shift Right.......................................................................................122

        /*  79 */
        InstructionType::MOV => {
            process_mov(rdr, &word, index, value_storage);
        } // – Copy Register............................................................................................123
        /*  80 */
        InstructionType::MOVW => {
            process_movw(rdr, &word, index, value_storage);
        } // – Copy Register Word...............................................................................124
        /*  81 */
        InstructionType::MUL => {
            process_mul(rdr, &word, index, value_storage);
        } // – Multiply Unsigned.......................................................................................125
        /*  82 */
        InstructionType::MULS => {
            process_muls(rdr, &word, index, value_storage);
        } // – Multiply Signed........................................................................................ 126
        /*  83 */
        InstructionType::MULSU => {
            process_mulsu(rdr, &word, index, value_storage);
        } //

        /*  84 */
        InstructionType::NEG => {
            process_neg(rdr, &word, index, value_storage);
        }
        /*  85 */
        InstructionType::NOP => {
            process_nop(rdr, &word, index, value_storage);
        }

        /*  86 */
        InstructionType::OR => {
            process_or(rdr, &word, index, value_storage);
        }
        /*  87 */
        InstructionType::ORI => {
            process_ori(rdr, &word, index, value_storage);
        }
        /*  88 */
        InstructionType::OUT => {
            process_out(rdr, &word, index, value_storage);
        }

        /*  89 */
        InstructionType::POP => {
            process_pop(rdr, &word, index, value_storage);
        } // – Pop Register from Stack............................................................................ 135
        /*  90 */
        InstructionType::PUSH => {
            process_push(rdr, &word, index, value_storage);
        } // – Push Register on Stack........................................................................... 136

        /*  91 */
        InstructionType::RCALL => {
            process_rcall(rdr, &word, index, value_storage);
        } // – Relative Call to Subroutine.................................................................... 137
        /*  92 */
        InstructionType::RET => {
            process_ret(rdr, &word, index, value_storage);
        } // – Return from Subroutine.............................................................................. 139
        /*  93 */
        InstructionType::RETI => {
            process_reti(rdr, &word, index, value_storage);
        } // – Return from Interrupt................................................................................. 140
        /*  94 */
        InstructionType::RJMP => {
            process_rjmp(rdr, &word, index, value_storage);
        } // – Relative Jump.......................................................................................... 142
        /*  95 */
        InstructionType::ROL => {
            process_rol(rdr, &word, index, value_storage);
        } //
        /*  96 */
        InstructionType::ROR => {
            process_ror(rdr, &word, index, value_storage);
        } // – Rotate Right through Carry........................................................................145

        /*  97 */
        InstructionType::SBC => {
            process_sbc(rdr, &word, index, value_storage);
        } // – Subtract with Carry.....................................................................................147
        /*  98 */
        InstructionType::SBCI => {
            process_sbci(rdr, &word, index, value_storage);
        } // – Subtract Immediate with Carry SBI – Set Bit in I/O Register.................... 149
        /*  99 */
        InstructionType::SBI => {
            process_sbi(rdr, &word, index, value_storage);
        } // – Set Bit in I/O Register.................................................................................. 151
        /* 100 */
        InstructionType::SBIC => {
            process_sbic(rdr, &word, index, value_storage);
        } // – Skip if Bit in I/O Register is Cleared........................................................ 152
        /* 101 */
        InstructionType::SBIS => {
            process_sbis(rdr, &word, index, value_storage);
        } // – Skip if Bit in I/O Register is Set............................................................... 153
        /* 102 */
        InstructionType::SBIW => {
            process_sbiw(rdr, &word, index, value_storage);
        } // – Subtract Immediate from Word...............................................................154
        /* 103 */
        InstructionType::SBR => {
            process_sbr(rdr, &word, index, value_storage);
        } // – Set Bits in Register...................................................................................156
        /* 104 */
        InstructionType::SBRC => {
            process_sbrc(rdr, &word, index, value_storage);
        } // – Skip if Bit in Register is Cleared............................................................ 157
        /* 105 */
        InstructionType::SBRS => {
            process_sbrs(rdr, &word, index, value_storage);
        } // – Skip if Bit in Register is Set....................................................................158
        /* 106 */
        InstructionType::SEC => {
            process_sec(rdr, &word, index, value_storage);
        } // – Set Carry Flag.......................................................................................... 159
        /* 107 */
        InstructionType::SEH => {
            process_seh(rdr, &word, index, value_storage);
        } // – Set Half Carry Flag...................................................................................160
        /* 108 */
        InstructionType::SEI => {
            process_sei(rdr, &word, index, value_storage);
        } //
        /* 109 */
        InstructionType::SEN => {
            process_sen(rdr, &word, index, value_storage);
        } // – Set Negative Flag.....................................................................................162
        /* 110 */
        InstructionType::SER => {
            process_ser(rdr, &word, index, value_storage);
        } // – Set all Bits in Register.............................................................................. 163
        /* 111 */
        InstructionType::SES => {
            process_ses(rdr, &word, index, value_storage);
        } // – Set Signed Flag........................................................................................ 164
        /* 112 */
        InstructionType::SET => {
            process_set(rdr, &word, index, value_storage);
        } // – Set T Flag................................................................................................. 165
        /* 113 */
        InstructionType::SEV => {
            process_sev(rdr, &word, index, value_storage);
        } // – Set Overflow Flag..................................................................................... 166
        /* 114 */
        InstructionType::SEZ => {
            process_sez(rdr, &word, index, value_storage);
        } // – Set Zero Flag............................................................................................ 167
        /* 115 */
        InstructionType::SLEEP => {
            process_sleep(rdr, &word, index, value_storage);
        } //................................................................................................................. 168
        /* 116 */
        InstructionType::SPM => {
            process_spm(rdr, &word, index, value_storage);
        } // – Store Program Memory............................................................................169
        /* 117 */
        InstructionType::SPM_2 => {
            process_spm2(rdr, &word, index, value_storage);
        } // SPM #2 – Store Program Memory.......................................................................171
        /* 118 */
        InstructionType::ST => {
            process_st(rdr, &word, index, value_storage);
        } // – Store Indirect From Register to Data Space using Index X.........................173
        // /* 119 */ InstructionType::ST_STD_Y => { process_st_std_y(rdr, &word, index, value_storage); }, // – Store Indirect From Register to Data Space using Index Y..............175
        InstructionType::ST_STD_Y_1 => {
            process_st_std_y_1(rdr, &word, index, value_storage);
        }
        InstructionType::ST_STD_Y_2 => {
            process_st_std_y_2(rdr, &word, index, value_storage);
        }
        InstructionType::ST_STD_Y_3 => {
            process_st_std_y_3(rdr, &word, index, value_storage);
        }
        InstructionType::ST_STD_Y_4 => {
            process_st_std_y_4(rdr, &word, index, value_storage);
        }
        // /* 120 */ InstructionType::ST_STD_Z => { process_st_std_z(rdr, &word, index, value_storage); },
        InstructionType::ST_STD_Z_1 => {
            process_st_std_z_1(rdr, &word, index, value_storage);
        }
        InstructionType::ST_STD_Z_2 => {
            process_st_std_z_2(rdr, &word, index, value_storage);
        }
        InstructionType::ST_STD_Z_3 => {
            process_st_std_z_3(rdr, &word, index, value_storage);
        }
        InstructionType::ST_STD_Z_4 => {
            process_st_std_z_4(rdr, &word, index, value_storage);
        }
        /* 121 */
        InstructionType::STS => {
            process_sts(rdr, &word, index, value_storage);
        } // STS – Store Direct to Data Space.......................................................................179
        /* 122 */
        InstructionType::STS_16bit => {
            process_sts_16bit(rdr, &word, index, value_storage);
        } // STS (16-bit) – Store Direct to Data Space.......................................................... 180
        /* 123 */
        InstructionType::SUB => {
            process_sub(rdr, &word, index, value_storage);
        } // – Subtract Without Carry.............................................................................181
        /* 124 */
        InstructionType::SUBI => {
            process_subi(rdr, &word, index, value_storage);
        } // – Subtract Immediate................................................................................. 183
        /* 125 */
        InstructionType::SWAP => {
            process_swap(rdr, &word, index, value_storage);
        } // – Swap Nibbles........................................................................................ 185

        /* 126 */
        InstructionType::TST => {
            process_tst(rdr, &word, index, value_storage);
        } // – Test for Zero or Minus...............................................................................186

        /* 127 */
        InstructionType::WDR => {
            process_wdr(rdr, &word, index, value_storage);
        } // – Watchdog Reset......................................................................................187

        /* 128 */
        InstructionType::XCH => {
            process_xch(rdr, &word, index, value_storage);
        } //

        InstructionType::UNKNOWN => {
            panic!("Unknown instruction!");
        }
        // _ => { panic!("Unknown instruction!"); }
    }
}

#[allow(dead_code, unused)]
pub fn process_adc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_add<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_adiw<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("ADIW  = 7. ADIW – Add Immediate to Word");
    //log::info!("{word:#b}");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "10010110kkddkkkk", &mut var_storage);

    let k_val = value_storage[&'K'];
    let d_val = value_storage[&'d'];

    //log::info!("k: {}, d: {}", k_val, d_val);
}

#[allow(dead_code, unused)]
pub fn process_and<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_andi<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("ANDI = 9. ANDI – Logical AND with Immediate");

    // 0111 KKKK dddd KKKK

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  0111KKKKddddKKKK");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "0100KKKKddddKKKK", &mut var_storage);

    let k_val = value_storage[&'K'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
}

#[allow(dead_code, unused)]
pub fn process_asr<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_bclr<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_bld<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brbc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brbs<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brcc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brcs<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_break<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_breq<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brge<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brhc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brhs<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brid<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brie<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brlo<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brlt<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("BRLT   = 25. BRLT – Branch if Less Than (Signed)");

    // 1111 00kk kkkk k100

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  111100kkkkkkk100");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "111100kkkkkkk100", &mut var_storage);

    let k_val = value_storage[&'k'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
}

#[allow(dead_code, unused)]
pub fn process_brne<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("BRNE = 27. BRNE – Branch if Not Equal");

    // 1111 01kk kkkk k001

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  111101kkkkkkk001");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "111101kkkkkkk001", &mut var_storage);

    let k_val = value_storage[&'k'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
}

#[allow(dead_code, unused)]
pub fn process_brmi<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brpl<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brsh<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brtc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brts<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brvc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_brvs<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_bset<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_bst<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_call<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("CALL = 36. CALL – Long Call to a Subroutine");

    // 1001 010k kkkk 111k
    // kkkk kkkk kkkk kkkk

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1001010kkkkk111k");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1001010kkkkk111k", &mut var_storage);

    let k_hi: u32 = value_storage[&'k'].into();
    //log::info!("k: {k_hi:#b} {k_hi:#x} {k_hi}");

    // read the next two byte of the 32 bit instruction
    let k_lo: u32 = (rdr.read_u16::<LittleEndian>().unwrap()).into();
    *index += 2usize;
    log::trace!("k_lo: {:b}", k_lo);

    let k: u32 = (k_hi << 16u8) + k_lo;

    log::trace!("k: {:#06x}", k);

    // since the amount of elements to jump are words, to find the address, multiply by two
    //log::info!("k: {:#06x}", k*2);

    let addr_value = k * 2;

    log::info!(
        "{:#02x}: {word:#06x} {k_lo:#06x} call {addr_value:#02x}",
        *index - 4usize
    );
}

#[allow(dead_code, unused)]
pub fn process_cbi<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_cbr<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_clc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_clh<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_cli<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("CLI  = 41. CLI – Clear Global Interrupt Flag");

    // 1001 0100 1111 1000
}

#[allow(dead_code, unused)]
pub fn process_cln<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_clr<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    // CLR and EOR are the same thing!
    // CLR is implemented by performing an EOR of the register with itself.
    //
    // Clears a register. This instruction performs an Exclusive OR between a register and itself. This will clear
    // all bits in the register.
}

#[allow(dead_code, unused)]
pub fn process_cls<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_clt<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_clv<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_clz<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_com<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_cp<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_cpc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("CPC   = 50. CPC – Compare with Carry");

    // 0000 01rd dddd rrrr

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  000001rdddddrrrr");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "000001rdddddrrrr", &mut var_storage);

    let r_val = value_storage[&'r'];
    //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
}

#[allow(dead_code, unused)]
pub fn process_cpi<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("CPI  = 51. CPI – Compare with Immediate");

    // 0011 KKKK dddd KKKK

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  0011KKKKddddKKKK");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "0011KKKKddddKKKK", &mut var_storage);

    let k_val = value_storage[&'K'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
}

#[allow(dead_code, unused)]
pub fn process_cpse<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_dec<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_des<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_eicall<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_eijmp<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_elpm<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_eor<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("EOR  = 58. EOR – Exclusive OR");

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  001001rdddddrrrr");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "001001rdddddrrrr", &mut var_storage);

    let r_val = value_storage[&'r'];
    //log::info!("r: {r_val:#b} {r_val:#x}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x}");

    log::info!(
        "{:#02x}: {word:#06x} eor r{r_val}, r{d_val}",
        *index - 2usize
    );
}

#[allow(dead_code, unused)]
pub fn process_fmul<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_fmuls<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_fmulsu<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_icall<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_ijmp<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_in<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("IN   = 64. IN - Load an I/O Location to Register");

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  10110AAdddddAAAA");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "10110AAdddddAAAA", &mut var_storage);

    let a_val = value_storage[&'A'];
    //log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

    log::info!(
        "{:#02x}: {word:#06x} in r{d_val:#02} {a_val:#02x}",
        *index - 2usize
    );
}

#[allow(dead_code, unused)]
pub fn process_inc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_jmp<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    log::trace!("JMP  = 66. JMP – Jump");
    log::trace!("{word:#b}");

    let k_hi: u32 = value_storage[&'k'].into();
    log::trace!("k_hi: {}", k_hi);

    // read the next two byte of the 32 bit instruction
    let k_lo: u32 = (rdr.read_u16::<LittleEndian>().unwrap()).into();
    //index = index + (usize)2;
    //index += 2;
    *index += 2usize;
    log::trace!("k_lo: {:b}", k_lo);

    let k: u32 = (k_hi << 16u8) + k_lo;

    log::trace!("k: {:#06x}", k);

    // since the amount of elements to jump are words, to find the address, multiply by two
    log::trace!("k: {:#06x}", k * 2);

    log::info!(
        "{:#02x}: {word:#06x} {k_lo:#06x} jmp {:#06x}",
        *index - 4usize,
        k * 2
    );

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

#[allow(dead_code, unused)]
pub fn process_lac<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_las<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_lat<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_ld<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_ld_ldd_y<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_ld_ldd_z<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_ldi<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
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

    log::info!(
        "{:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}",
        *index - 2usize
    );
}

#[allow(dead_code, unused)]
pub fn process_lds<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_lds_16bit<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_lpm<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_lsl<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_lsr<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_mov<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_movw<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_mul<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_muls<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_mulsu<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_neg<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_nop<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("NOP  = 85. NOP – No Operation");

    // 0000 0000 0000 0000
}

#[allow(dead_code, unused)]
pub fn process_or<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_ori<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("ORI  = 87. ORI – Logical OR with Immediate");

    // 0110 KKKK dddd KKKK

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  0110KKKKddddKKKK");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "0110KKKKddddKKKK", &mut var_storage);

    let k_val = value_storage[&'K'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
}

#[allow(dead_code, unused)]
pub fn process_out<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("OUT  = 88. OUT – Store Register to I/O Location");

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  10111AArrrrrAAAA");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "10111AArrrrrAAAA", &mut var_storage);

    let a_val = value_storage[&'A'];
    //log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
    let r_val = value_storage[&'r'];
    //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

    log::info!(
        "{:#02x}: {word:#06x} out {a_val:#02x}, r{r_val}",
        *index - 2usize
    );
}

#[allow(dead_code, unused)]
pub fn process_pop<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("POP   = 89. POP – Pop Register from Stack");

    // 1001 000d dddd 1111

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1001000ddddd1111");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1001000ddddd1111", &mut var_storage);

    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
}

#[allow(dead_code, unused)]
pub fn process_push<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("PUSH  = 90. PUSH – Push Register on Stack");

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1001001ddddd1111");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1001001ddddd1111", &mut var_storage);

    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

    let register = d_val;

    log::info!("{:#02x}: {word:#06x} push r{register:#02}", *index - 2usize);
}

#[allow(dead_code, unused)]
pub fn process_rcall<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("RCALL  = 91. RCALL – Relative Call to Subroutine");

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1101kkkkkkkkkkkk");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1101kkkkkkkkkkkk", &mut var_storage);

    let k_val = value_storage[&'k'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");

    log::info!("{:#02x}: {word:#06x} rcall .+{k_val:#02}", *index - 22usize);
}

#[allow(dead_code, unused)]
pub fn process_ret<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("RET   = 92. RET – Return from Subroutine");

    // 1001 0101 0000 1000
}

#[allow(dead_code, unused)]
pub fn process_reti<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_rjmp<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("RJMP = 94. RJMP – Relative Jump");

    // 1100 kkkk kkkk kkkk

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1100kkkkkkkkkkkk");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1100kkkkkkkkkkkk", &mut var_storage);

    let k_val = value_storage[&'k'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
}

#[allow(dead_code, unused)]
pub fn process_rol<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_ror<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sbc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sbci<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("SBCI = 98. SBCI – Subtract Immediate with Carry SBI – Set Bit in I/O Register");

    // 0100 KKKK dddd KKKK

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  0100KKKKddddKKKK");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "0100KKKKddddKKKK", &mut var_storage);

    let k_val = value_storage[&'K'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
}

#[allow(dead_code, unused)]
pub fn process_sbi<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sbic<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sbis<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sbiw<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sbr<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sbrc<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sbrs<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sec<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_seh<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sei<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sen<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_ser<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_ses<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_set<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sev<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sez<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sleep<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_spm<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_spm2<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_st<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_st_std_y_1<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    log::info!(
        "STD Y (A) = 119. ST (STD) – Store Indirect From Register to Data Space using Index Y"
    );

    // 1000 001r rrrr 1000

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1000001rrrrr1000");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1000001rrrrr1000", &mut var_storage);

    let r_val = value_storage[&'r'];
    //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
}

#[allow(dead_code, unused)]
pub fn process_st_std_y_2<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_st_std_y_3<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_st_std_y_4<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_st_std_z_1<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_st_std_z_2<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_st_std_z_3<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_st_std_z_4<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_st_std_z<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sts<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sts_16bit<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_sub<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_subi<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
    //log::info!("SUBI = 124. SUBI – Subtract Immediate");

    // 0101 KKKK dddd KKKK

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  0101KKKKddddKKKK");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "0101KKKKddddKKKK", &mut var_storage);

    let k_val = value_storage[&'K'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
}

#[allow(dead_code, unused)]
pub fn process_swap<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_tst<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_wdr<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

#[allow(dead_code, unused)]
pub fn process_xch<R: crate::io::Read>(
    rdr: &mut R,
    word: &u16,
    index: &mut usize,
    value_storage: &mut HashMap<char, u16>,
) {
}

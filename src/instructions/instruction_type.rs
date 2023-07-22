#[derive(PartialEq)]
#[derive(Debug, Clone, Copy)]
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

    pub const fn from_code(code: u8) -> InstructionType {
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

    pub const fn to_code(instruction_type: &InstructionType) -> u8 {
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
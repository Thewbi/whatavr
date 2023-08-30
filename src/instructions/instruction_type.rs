use std::fmt::Debug;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[derive(Debug, Clone, Copy)]
#[derive(Default)]
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
    /* 65 */ INC,

    /* 66 */ JMP,

    /* 67 */ LAC,
    /* 68 */ LAS,
    /* 69 */ LAT,

    /* 70 */ 
    LD_LDD_X_1,
    LD_LDD_X_2,
    LD_LDD_X_3,
    /* 71 */ 
    LD_LDD_Y_1,
    LD_LDD_Y_2,
    LD_LDD_Y_3,
    LD_LDD_Y_4,
    /* 72 */ 
    LD_LDD_Z_1,
    LD_LDD_Z_2,
    LD_LDD_Z_3,
    LD_LDD_Z_4,

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
    // /* 118 */ ST, // – Store Indirect From Register to Data Space using Index X.........................173
    ST_STD_X_1,
    ST_STD_X_2,
    ST_STD_X_3,
    // /* 119 */ ST_STD_Y, // – Store Indirect From Register to Data Space using Index Y..............175
    ST_STD_Y_1,
    ST_STD_Y_2,
    ST_STD_Y_3,
    ST_STD_Y_4,
    // /* 120 */ ST_STD_Z,
    ST_STD_Z_1,
    ST_STD_Z_2,
    ST_STD_Z_3,
    ST_STD_Z_4,
    /* 121 */ STS, // STS – Store Direct to Data Space.......................................................................179
    /* 122 */ STS_16bit, // STS (16-bit) – Store Direct to Data Space.......................................................... 180
    /* 123 */ SUB, // – Subtract Without Carry.............................................................................181
    /* 124 */ SUBI, // – Subtract Immediate................................................................................. 183
    /* 125 */ SWAP, // – Swap Nibbles........................................................................................ 185
    
    /* 126 */ TST, // – Test for Zero or Minus...............................................................................186
    
    /* 127 */ WDR, // – Watchdog Reset......................................................................................187
    
    /* 128 */ XCH, // 
    
    #[default]
    UNKNOWN,
}

impl InstructionType {

    #[allow(dead_code)]
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
            26 => InstructionType::BRMI,
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

            70 => InstructionType::LD_LDD_X_1,
            // 118 => InstructionType::LD_STD_X_2,
            // 118 => InstructionType::LD_STD_X_3,
            
            71 => InstructionType::LD_LDD_Y_1, 
            // 119 => InstructionType::LD_STD_Y_2,
            // 119 => InstructionType::LD_STD_Y_3,
            // 119 => InstructionType::LD_STD_Y_4,
            
            72 => InstructionType::LD_LDD_Z_1, 
            // 120 => InstructionType::LD_STD_Z_2,
            // 120 => InstructionType::LD_STD_Z_3,
            // 120 => InstructionType::LD_STD_Z_4,

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
            
            118 => InstructionType::ST_STD_X_1,
            // 118 => InstructionType::ST_STD_X_2,
            // 118 => InstructionType::ST_STD_X_3,
            
            119 => InstructionType::ST_STD_Y_1, 
            // 119 => InstructionType::ST_STD_Y_2,
            // 119 => InstructionType::ST_STD_Y_3,
            // 119 => InstructionType::ST_STD_Y_4,
            
            120 => InstructionType::ST_STD_Z_1, 
            // 120 => InstructionType::ST_STD_Z_2,
            // 120 => InstructionType::ST_STD_Z_3,
            // 120 => InstructionType::ST_STD_Z_4,
            
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
            
            _ => InstructionType::UNKNOWN,
        }
    }

    pub fn from_string(code: &str) -> InstructionType {

        let uc = code.to_uppercase();

        let upper_case: &str = uc.as_str();

        match upper_case {
            "ADC" => InstructionType::ADC,
            "ADD" => InstructionType::ADD,
            "ADIW" => InstructionType::ADIW,
            "AND" => InstructionType::AND,
            "ANDI" => InstructionType::ANDI,
            "ASR" => InstructionType::ASR,
            "BCLR" => InstructionType::BCLR,
            "BLD" => InstructionType::BLD,
            "BRBC" => InstructionType::BRBC,
            "BRBS" => InstructionType::BRBS,
            "BRCC" => InstructionType::BRCC,
            "BRCS" => InstructionType::BRCS,
            "BREAK" => InstructionType::BREAK, 
            "BREQ" => InstructionType::BREQ,
            "BRGE" => InstructionType::BRGE,
            "BRHC" => InstructionType::BRHC,
            "BRHS" => InstructionType::BRHS,
            "BRID" => InstructionType::BRID,
            "BRIE" => InstructionType::BRIE,
            "BRLO" => InstructionType::BRLO,
            "BRLT" => InstructionType::BRLT,
            "BRMI" => InstructionType::BRMI,
            "BRNE" => InstructionType::BRNE,
            "BRPL" => InstructionType::BRPL,
            "BRSH" => InstructionType::BRSH,
            "BRTC" => InstructionType::BRTC,
            "BRTS" => InstructionType::BRTS,
            "BRVC" => InstructionType::BRVC,
            "BRVS" => InstructionType::BRVS,
            "BSET" => InstructionType::BSET,
            "BST" => InstructionType::BST,
            "CALL" => InstructionType::CALL,
            "CBI" => InstructionType::CBI, 
            "CBR" => InstructionType::CBR,
            "CLC" => InstructionType::CLC,
            "CLH" => InstructionType::CLH,
            "CLI" => InstructionType::CLI,
            "CLN" => InstructionType::CLN,
            "CLR" => InstructionType::CLR,
            "CLS" => InstructionType::CLS,
            "CLT" => InstructionType::CLT,
            "CLV" => InstructionType::CLV,
            "CLZ" => InstructionType::CLZ,
            "COM" => InstructionType::COM,
            "CP" => InstructionType::CP,
            "CPC" => InstructionType::CPC,
            "CPI" => InstructionType::CPI,
            "CPSE" => InstructionType::CPSE,
            "DEC" => InstructionType::DEC,
            "DES" => InstructionType::DES,
            "EICALL" => InstructionType::EICALL,
            "EIJMP" => InstructionType::EIJMP,
            "ELPM" => InstructionType::ELPM,
            "EOR" => InstructionType::EOR,
            "FMUL" => InstructionType::FMUL,
            "FMULS" => InstructionType::FMULS,
            "FMULSU" => InstructionType::FMULSU,
            "ICALL" => InstructionType::ICALL,
            "IJMP" => InstructionType::IJMP,
            "IN" => InstructionType::IN,
            "INC" => InstructionType::INC,
            "JMP" => InstructionType::JMP,
            "LAC" => InstructionType::LAC,
            "LAS" => InstructionType::LAS,
            "LAT" => InstructionType::LAT, 

            "LD_LDD_X_1" => InstructionType::LD_LDD_X_1,
            "LD_LDD_X_2" => InstructionType::LD_LDD_X_2,
            "LD_LDD_X_3" => InstructionType::LD_LDD_X_3,

            "LD_LDD_Y_1" => InstructionType::LD_LDD_Y_1, 
            "LD_LDD_Y_2" => InstructionType::LD_LDD_Y_2, 
            "LD_LDD_Y_3" => InstructionType::LD_LDD_Y_3, 
            "LD_LDD_Y_4" => InstructionType::LD_LDD_Y_4, 
            
            "LD_LDD_Z_1" => InstructionType::LD_LDD_Z_1, 
            "LD_LDD_Z_2" => InstructionType::LD_LDD_Z_2, 
            "LD_LDD_Z_3" => InstructionType::LD_LDD_Z_3, 
            "LD_LDD_Z_4" => InstructionType::LD_LDD_Z_4,

            "LDI" => InstructionType::LDI,
            "LDS" => InstructionType::LDS,
            "LDS_16bit" => InstructionType::LDS_16bit, // (16-bit) – Load Direct from Data Space......................................................... 117
            "LPM" => InstructionType::LPM, // – Load Program Memory...............................................................................118
            "LSL" => InstructionType::LSL, // – Logical Shift Left..........................................................................................120
            "LSR" => InstructionType::LSR, // – Logical Shift Right.......................................................................................122
            "MOV" => InstructionType::MOV, // – Copy Register............................................................................................123
            "MOVW" => InstructionType::MOVW, // – Copy Register Word...............................................................................124
            "MUL" => InstructionType::MUL, // – Multiply Unsigned.......................................................................................125
            "MULS" => InstructionType::MULS, // – Multiply Signed........................................................................................ 126
            "MULSU" => InstructionType::MULSU , //
            "NEG" => InstructionType::NEG,
            "NOP" => InstructionType::NOP, 
            "OR" => InstructionType::OR,
            "ORI" => InstructionType::ORI,
            "OUT" => InstructionType::OUT,
            "POP" => InstructionType::POP,
            "PUSH" => InstructionType::PUSH,
            "RCALL" => InstructionType::RCALL,
            "RET" => InstructionType::RET,
            "RETI" => InstructionType::RETI, 
            "RJMP" => InstructionType::RJMP,
            "ROL" => InstructionType::ROL, //
            "ROR" => InstructionType::ROR, // – Rotate Right through Carry........................................................................145
            "SBC" => InstructionType::SBC, 
            "SBCI" => InstructionType::SBCI,
            "SBI" => InstructionType::SBI, // – Set Bit in I/O Register.................................................................................. 151
            "SBIC" => InstructionType::SBIC, // – Skip if Bit in I/O Register is Cleared........................................................ 152
            "SBIS" => InstructionType::SBIS, // – Skip if Bit in I/O Register is Set............................................................... 153
            "SBIW" => InstructionType::SBIW, // – Subtract Immediate from Word...............................................................154
            "SBR" => InstructionType::SBR, // – Set Bits in Register...................................................................................156
            "SBRC" => InstructionType::SBRC, // – Skip if Bit in Register is Cleared............................................................ 157
            "SBRS" => InstructionType::SBRS, // – Skip if Bit in Register is Set....................................................................158
            "SEC" => InstructionType::SEC, // – Set Carry Flag.......................................................................................... 159
            "SEH" => InstructionType::SEH, // – Set Half Carry Flag...................................................................................160
            "SEI" => InstructionType::SEI, //
            "SEN" => InstructionType::SEN, // – Set Negative Flag.....................................................................................162
            "SER" => InstructionType::SER, // – Set all Bits in Register.............................................................................. 163
            "SES" => InstructionType::SES, // – Set Signed Flag........................................................................................ 164
            "SET" => InstructionType::SET, // – Set T Flag................................................................................................. 165
            "SEV" => InstructionType::SEV, // – Set Overflow Flag..................................................................................... 166
            "SEZ" => InstructionType::SEZ, // – Set Zero Flag............................................................................................ 167
            "SLEEP" => InstructionType::SLEEP, //................................................................................................................. 168
            "SPM" => InstructionType::SPM, // – Store Program Memory............................................................................169
            "SPM_2" => InstructionType::SPM_2, // SPM #2 – Store Program Memory.......................................................................171
            
            "ST_STD_X_1" => InstructionType::ST_STD_X_1,
            "ST_STD_X_2" => InstructionType::ST_STD_X_2,
            "ST_STD_X_3" => InstructionType::ST_STD_X_3,

            "ST_STD_Y_1" => InstructionType::ST_STD_Y_1, 
            "ST_STD_Y_2" => InstructionType::ST_STD_Y_2, 
            "ST_STD_Y_3" => InstructionType::ST_STD_Y_3, 
            "ST_STD_Y_4" => InstructionType::ST_STD_Y_4, 

            "ST_STD_Z_1" => InstructionType::ST_STD_Z_1, 
            "ST_STD_Z_2" => InstructionType::ST_STD_Z_2, 
            "ST_STD_Z_3" => InstructionType::ST_STD_Z_3, 
            "ST_STD_Z_4" => InstructionType::ST_STD_Z_4, 
            
            "STS" => InstructionType::STS, // STS – Store Direct to Data Space.......................................................................179
            "STS_16bit" => InstructionType::STS_16bit, // STS (16-bit) – Store Direct to Data Space.......................................................... 180
            "SUB" => InstructionType::SUB, // – Subtract Without Carry.............................................................................181
            "SUBI" => InstructionType::SUBI, // – Subtract Immediate................................................................................. 183
            "SWAP" => InstructionType::SWAP, // – Swap Nibbles........................................................................................ 185
            "TST" => InstructionType::TST, // – Test for Zero or Minus...............................................................................186
            "WDR" => InstructionType::WDR, // – Watchdog Reset......................................................................................187
            "XCH" => InstructionType::XCH,

            // STD Y (B) = 119. 
            // STD Y (C) = 119.
            // STD Y (D) = 119.
            // STD Z (A) = 120.
            // STD Z (B) = 120.
            // STD Z (C) = 120.
            // STD Z (D) = 120.
            
            _ => InstructionType::UNKNOWN,
        }
    }

    pub const fn to_code(instruction_type: &InstructionType) -> u8 {
        match instruction_type {

            //TODO:
            // InstructionType::IN => 64,
            // InstructionType::JMP => 66,
            // InstructionType::LDI => 73,
            // InstructionType::OUT => 88,
            // InstructionType::Unknown => 0xFF,
            _ => 0xFF,
        }
    }

    pub fn to_string(&self) -> String { 

        match self {
            InstructionType::ADC => String::from("ADC"),
            InstructionType::ADD => String::from("ADD"),
            InstructionType::ADIW => String::from("ADIW"),
            InstructionType::AND => String::from("AND"),
            InstructionType::ANDI => String::from("ANDI"),
            InstructionType::ASR => String::from("ASR"),
            InstructionType::BCLR => String::from("BCLR"),
            InstructionType::BLD => String::from("BLD"),
            InstructionType::BRBC => String::from("BRBC"),
            InstructionType::BRBS => String::from("BRBS"),
            InstructionType::BRCC => String::from("BRCC"),
            InstructionType::BRCS => String::from("BRCS"),
            InstructionType::BREAK => String::from("BREAK"), 
            InstructionType::BREQ => String::from("BREQ"),
            InstructionType::BRGE => String::from("BRGE"),
            InstructionType::BRHC => String::from("BRHC"),
            InstructionType::BRHS => String::from("BRHS"),
            InstructionType::BRID => String::from("BRID"),
            InstructionType::BRIE => String::from("BRIE"),
            InstructionType::BRLO => String::from("BRLO"),
            InstructionType::BRLT => String::from("BRLT"),
            InstructionType::BRMI => String::from("BRMI"),
            InstructionType::BRNE => String::from("BRNE"),
            InstructionType::BRPL => String::from("BRPL"),
            InstructionType::BRSH => String::from("BRSH"),
            InstructionType::BRTC => String::from("BRTC"),
            InstructionType::BRTS => String::from("BRTS"),
            InstructionType::BRVC => String::from("BRVC"),
            InstructionType::BRVS => String::from("BRVS"),
            InstructionType::BSET => String::from("BSET"),
            InstructionType::BST => String::from("BST"),
            InstructionType::CALL => String::from("CALL"),
            InstructionType::CBI => String::from("CBI"), 
            InstructionType::CBR => String::from("CBR"),
            InstructionType::CLC => String::from("CLC"),
            InstructionType::CLH => String::from("CLH"),
            InstructionType::CLI => String::from("CLI"),
            InstructionType::CLN => String::from("CLN"),
            InstructionType::CLR => String::from("CLR"),
            InstructionType::CLS => String::from("CLS"),
            InstructionType::CLT => String::from("CLT"),
            InstructionType::CLV => String::from("CLV"),
            InstructionType::CLZ => String::from("CLZ"),
            InstructionType::COM => String::from("COM"),
            InstructionType::CP => String::from("CP"),
            InstructionType::CPC => String::from("CPC"),
            InstructionType::CPI => String::from("CPI"),
            InstructionType::CPSE => String::from("CPSE"),
            InstructionType::DEC => String::from("DEC"),
            InstructionType::DES => String::from("DES"),
            InstructionType::EICALL => String::from("EICALL"),
            InstructionType::EIJMP => String::from("EIJMP"),
            InstructionType::ELPM => String::from("ELPM"),
            InstructionType::EOR => String::from("EOR"),
            InstructionType::FMUL => String::from("FMUL"),
            InstructionType::FMULS => String::from("FMULS"),
            InstructionType::FMULSU => String::from("FMULSU"),
            InstructionType::ICALL => String::from("ICALL"),
            InstructionType::IJMP => String::from("IJMP"),
            InstructionType::IN => String::from("IN"),
            InstructionType::INC => String::from("INC"),
            InstructionType::JMP => String::from("JMP"),
            InstructionType::LAC => String::from("LAC"),
            InstructionType::LAS => String::from("LAS"),
            InstructionType::LAT => String::from("LAT"), 

            InstructionType::LD_LDD_X_1 => String::from("LD_LDD_X_1"),
            InstructionType::LD_LDD_X_2 => String::from("LD_LDD_X_2"),
            InstructionType::LD_LDD_X_3 => String::from("LD_LDD_X_3"),

            InstructionType::LD_LDD_Y_1 => String::from("LD_LDD_Y_1"), 
            InstructionType::LD_LDD_Y_2 => String::from("LD_LDD_Y_2"), 
            InstructionType::LD_LDD_Y_3 => String::from("LD_LDD_Y_3"), 
            InstructionType::LD_LDD_Y_4 => String::from("LD_LDD_Y_4"), 

            InstructionType::LD_LDD_Z_1 => String::from("LD_LDD_Z_1"),
            InstructionType::LD_LDD_Z_2 => String::from("LD_LDD_Z_2"),
            InstructionType::LD_LDD_Z_3 => String::from("LD_LDD_Z_3"),
            InstructionType::LD_LDD_Z_4 => String::from("LD_LDD_Z_4"),

            InstructionType::LDI => String::from("LDI"),
            InstructionType::LDS => String::from("LDS"),
            InstructionType::LDS_16bit => String::from("LDS_16bit"), // (16-bit) – Load Direct from Data Space......................................................... 117
            InstructionType::LPM => String::from("LPM"), // – Load Program Memory...............................................................................118
            InstructionType::LSL => String::from("LSL"), // – Logical Shift Left..........................................................................................120
            InstructionType::LSR => String::from("LSR"), // – Logical Shift Right.......................................................................................122
            InstructionType::MOV => String::from("MOV"), // – Copy Register............................................................................................123
            InstructionType::MOVW => String::from("MOVW"), // – Copy Register Word...............................................................................124
            InstructionType::MUL => String::from("MUL"), // – Multiply Unsigned.......................................................................................125
            InstructionType::MULS => String::from("MULS"), // – Multiply Signed........................................................................................ 126
            InstructionType::MULSU  => String::from("MULSU"), //
            InstructionType::NEG => String::from("NEG"),
            InstructionType::NOP => String::from("NOP"), 
            InstructionType::OR => String::from("OR"),
            InstructionType::ORI => String::from("ORI"),
            InstructionType::OUT => String::from("OUT"),
            InstructionType::POP => String::from("POP"),
            InstructionType::PUSH => String::from("PUSH"),
            InstructionType::RCALL => String::from("RCALL"),
            InstructionType::RET => String::from("RET"),
            InstructionType::RETI => String::from("RETI"), 
            InstructionType::RJMP => String::from("RJMP"),
            InstructionType::ROL => String::from("ROL"), //
            InstructionType::ROR => String::from("ROR"), // – Rotate Right through Carry........................................................................145
            InstructionType::SBC => String::from("SBC"), 
            InstructionType::SBCI => String::from("SBCI"),
            InstructionType::SBI => String::from("SBI"), // – Set Bit in I/O Register.................................................................................. 151
            InstructionType::SBIC => String::from("SBIC"), // – Skip if Bit in I/O Register is Cleared........................................................ 152
            InstructionType::SBIS => String::from("SBIS"), // – Skip if Bit in I/O Register is Set............................................................... 153
            InstructionType::SBIW => String::from("SBIW"), // – Subtract Immediate from Word...............................................................154
            InstructionType::SBR => String::from("SBR"), // – Set Bits in Register...................................................................................156
            InstructionType::SBRC => String::from("SBRC"), // – Skip if Bit in Register is Cleared............................................................ 157
            InstructionType::SBRS => String::from("SBRS"), // – Skip if Bit in Register is Set....................................................................158
            InstructionType::SEC => String::from("SEC"), // – Set Carry Flag.......................................................................................... 159
            InstructionType::SEH => String::from("SEH"), // – Set Half Carry Flag...................................................................................160
            InstructionType::SEI => String::from("SEI"), //
            InstructionType::SEN => String::from("SEN"), // – Set Negative Flag.....................................................................................162
            InstructionType::SER => String::from("SER"), // – Set all Bits in Register.............................................................................. 163
            InstructionType::SES => String::from("SES"), // – Set Signed Flag........................................................................................ 164
            InstructionType::SET => String::from("SET"), // – Set T Flag................................................................................................. 165
            InstructionType::SEV => String::from("SEV"), // – Set Overflow Flag..................................................................................... 166
            InstructionType::SEZ => String::from("SEZ"), // – Set Zero Flag............................................................................................ 167
            InstructionType::SLEEP => String::from("SLEEP"), //................................................................................................................. 168
            InstructionType::SPM => String::from("SPM"), // – Store Program Memory............................................................................169
            InstructionType::SPM_2 => String::from("SPM_2"), // SPM #2 – Store Program Memory.......................................................................171
            
            InstructionType::ST_STD_X_1 => String::from("ST_STD_X_1"),
            InstructionType::ST_STD_X_2 => String::from("ST_STD_X_2"),
            InstructionType::ST_STD_X_3 => String::from("ST_STD_X_3"),

            InstructionType::ST_STD_Y_1 => String::from("ST_STD_Y_1"), 
            InstructionType::ST_STD_Y_2 => String::from("ST_STD_Y_2"), 
            InstructionType::ST_STD_Y_3 => String::from("ST_STD_Y_3"), 
            InstructionType::ST_STD_Y_4 => String::from("ST_STD_Y_4"), 

            InstructionType::ST_STD_Z_1 => String::from("ST_STD_Z_1"),
            InstructionType::ST_STD_Z_2 => String::from("ST_STD_Z_2"),
            InstructionType::ST_STD_Z_3 => String::from("ST_STD_Z_3"),
            InstructionType::ST_STD_Z_4 => String::from("ST_STD_Z_4"),

            InstructionType::STS => String::from("STS"), // STS – Store Direct to Data Space.......................................................................179
            InstructionType::STS_16bit => String::from("STS_16bit"), // STS (16-bit) – Store Direct to Data Space.......................................................... 180
            InstructionType::SUB => String::from("SUB"), // – Subtract Without Carry.............................................................................181
            InstructionType::SUBI => String::from("SUBI"), // – Subtract Immediate................................................................................. 183
            InstructionType::SWAP => String::from("SWAP"), // – Swap Nibbles........................................................................................ 185
            InstructionType::TST => String::from("TST"), // – Test for Zero or Minus...............................................................................186
            InstructionType::WDR => String::from("WDR"), // – Watchdog Reset......................................................................................187
            InstructionType::XCH => String::from("XCH"),
            
            _ => String::from("UNKNOWN"),
        }
    }

    #[allow(dead_code)]
    pub const fn cycles(instruction_type: &InstructionType) -> usize {

        match instruction_type {
            /*   5 */ InstructionType::ADC => 2usize,
            /*   6 */ InstructionType::ADD => 2usize,
            /*   7 */ InstructionType::ADIW => 2usize,
            /*   8 */ InstructionType::AND => 2usize,
            /*   9 */ InstructionType::ANDI => 2usize,
            /*  10 */ InstructionType::ASR => 2usize,
            /*  11 */ InstructionType::BCLR => 2usize,
            /*  12 */ InstructionType::BLD => 2usize,
            /*  13 */ InstructionType::BRBC => 2usize,
            /*  14 */ InstructionType::BRBS => 2usize,
            /*  15 */ InstructionType::BRCC => 2usize,
            /*  16 */ InstructionType::BRCS => 2usize,
            /*  17 */ InstructionType::BREAK => 2usize, 
            /*  18 */ InstructionType::BREQ => 2usize,
            /*  19 */ InstructionType::BRGE => 2usize,
            /*  20 */ InstructionType::BRHC => 2usize,
            /*  21 */ InstructionType::BRHS => 2usize,
            /*  22 */ InstructionType::BRID => 2usize,
            /*  23 */ InstructionType::BRIE => 2usize,
            /*  24 */ InstructionType::BRLO => 2usize,
            /*  25 */ InstructionType::BRLT => 2usize,
            /*  26 */ InstructionType::BRMI => 2usize,
            /*  27 */ InstructionType::BRNE => 2usize,
            /*  28 */ InstructionType::BRPL => 2usize,
            /*  29 */ InstructionType::BRSH => 2usize,
            /*  30 */ InstructionType::BRTC => 2usize,
            /*  31 */ InstructionType::BRTS => 2usize,
            /*  32 */ InstructionType::BRVC => 2usize,
            /*  33 */ InstructionType::BRVS => 2usize,
            /*  34 */ InstructionType::BSET => 2usize,
            /*  35 */ InstructionType::BST => 2usize,
            /*  36 */ InstructionType::CALL => 2usize,
            /*  37 */ InstructionType::CBI => 2usize, 
            /*  38 */ InstructionType::CBR => 2usize,
            /*  39 */ InstructionType::CLC => 2usize,
            /*  40 */ InstructionType::CLH => 2usize,
            /*  41 */ InstructionType::CLI => 2usize,
            /*  42 */ InstructionType::CLN => 2usize,
            /*  43 */ InstructionType::CLR => 2usize,
            /*  44 */ InstructionType::CLS => 2usize,
            /*  45 */ InstructionType::CLT => 2usize,
            /*  46 */ InstructionType::CLV => 2usize,
            /*  47 */ InstructionType::CLZ => 2usize,
            /*  48 */ InstructionType::COM => 2usize,
            /*  49 */ InstructionType::CP => 2usize,
            /*  50 */ InstructionType::CPC => 2usize,
            /*  51 */ InstructionType::CPI => 2usize,
            /*  52 */ InstructionType::CPSE => 2usize,
            /*  53 */ InstructionType::DEC => 2usize,
            /*  54 */ InstructionType::DES => 2usize,
            /*  55 */ InstructionType::EICALL => 2usize,
            /*  56 */ InstructionType::EIJMP => 2usize,
            /*  57 */ InstructionType::ELPM => 2usize,
            /*  58 */ InstructionType::EOR => 2usize,
            /*  59 */ InstructionType::FMUL => 2usize,
            /*  60 */ InstructionType::FMULS => 2usize,
            /*  61 */ InstructionType::FMULSU => 2usize,
            /*  62 */ InstructionType::ICALL => 2usize,
            /*  63 */ InstructionType::IJMP => 2usize,
            /*  64 */ InstructionType::IN => 2usize,
            /*  65 */ InstructionType::INC => 2usize,
            /*  66 */ InstructionType::JMP => 2usize,
            /*  67 */ InstructionType::LAC => 2usize,
            /*  68 */ InstructionType::LAS => 2usize,
            /*  69 */ InstructionType::LAT => 2usize, 
            /*  70 */ 
            InstructionType::LD_LDD_X_1 => 2usize,
            InstructionType::LD_LDD_X_2 => 2usize,
            InstructionType::LD_LDD_X_3 => 2usize,
            /*  71*/ 
            InstructionType::LD_LDD_Y_1 => 2usize, 
            InstructionType::LD_LDD_Y_2 => 2usize, 
            InstructionType::LD_LDD_Y_3 => 2usize, 
            InstructionType::LD_LDD_Y_4 => 2usize, 
            /*  72 */ 
            InstructionType::LD_LDD_Z_1 => 2usize, 
            InstructionType::LD_LDD_Z_2 => 2usize, 
            InstructionType::LD_LDD_Z_3 => 2usize, 
            InstructionType::LD_LDD_Z_4 => 2usize,
            /*  73 */ InstructionType::LDI => 2usize,
            /*  74 */ InstructionType::LDS => 2usize,
            /*  75 */ InstructionType::LDS_16bit => 2usize, //  (16-bit) – Load Direct from Data Space......................................................... 117
            /*  76 */ InstructionType::LPM => 2usize, //  – Load Program Memory...............................................................................118
            /*  77 */ InstructionType::LSL => 2usize, //  – Logical Shift Left..........................................................................................120
            /*  78 */ InstructionType::LSR => 2usize, //  – Logical Shift Right.......................................................................................122
            /*  79 */ InstructionType::MOV => 2usize, //  – Copy Register............................................................................................123
            /*  80 */ InstructionType::MOVW => 2usize, //  – Copy Register Word...............................................................................124
            /*  81 */ InstructionType::MUL => 2usize, //  – Multiply Unsigned.......................................................................................125
            /*  82 */ InstructionType::MULS => 2usize, //  – Multiply Signed........................................................................................ 126
            /*  83 */ InstructionType::MULSU  => 2usize, // 
            /*  84 */ InstructionType::NEG => 2usize,
            /*  85 */ InstructionType::NOP => 2usize, 
            /*  86 */ InstructionType::OR => 2usize,
            /*  87 */ InstructionType::ORI => 2usize,
            /*  88 */ InstructionType::OUT => 2usize,
            /*  89 */ InstructionType::POP => 2usize,
            /*  90 */ InstructionType::PUSH => 2usize,
            /*  91 */ InstructionType::RCALL => 2usize,
            /*  92 */ InstructionType::RET => 2usize,
            /*  93 */ InstructionType::RETI => 2usize, 
            /*  94 */ InstructionType::RJMP => 2usize,
            /*  95 */ InstructionType::ROL => 2usize,  
            /*  96 */ InstructionType::ROR => 2usize, //  – Rotate Right through Carry........................................................................145
            /*  97 */ InstructionType::SBC => 2usize, 
            /*  98 */ InstructionType::SBCI => 2usize,
            /*  99 */ InstructionType::SBI => 2usize, //  – Set Bit in I/O Register.................................................................................. 151
            /*  100 */ InstructionType::SBIC => 2usize, //  – Skip if Bit in I/O Register is Cleared........................................................ 152
            /*  101 */ InstructionType::SBIS => 2usize, //  – Skip if Bit in I/O Register is Set............................................................... 153
            /*  102 */ InstructionType::SBIW => 2usize, //  – Subtract Immediate from Word...............................................................154
            /*  103 */ InstructionType::SBR => 2usize, //  – Set Bits in Register...................................................................................156
            /*  104 */ InstructionType::SBRC => 2usize, //  – Skip if Bit in Register is Cleared............................................................ 157
            /*  105 */ InstructionType::SBRS => 2usize, //  – Skip if Bit in Register is Set....................................................................158
            /*  106 */ InstructionType::SEC => 2usize, //  – Set Carry Flag.......................................................................................... 159
            /*  107 */ InstructionType::SEH => 2usize, //  – Set Half Carry Flag...................................................................................160
            /*  108 */ InstructionType::SEI => 2usize, // 
            /*  109 */ InstructionType::SEN => 2usize, //  – Set Negative Flag.....................................................................................162
            /*  110 */ InstructionType::SER => 2usize, //  – Set all Bits in Register.............................................................................. 163
            /*  111 */ InstructionType::SES => 2usize, //  – Set Signed Flag........................................................................................ 164
            /*  112 */ InstructionType::SET => 2usize, //  – Set T Flag................................................................................................. 165
            /*  113 */ InstructionType::SEV => 2usize, //  – Set Overflow Flag..................................................................................... 166
            /*  114 */ InstructionType::SEZ => 2usize, //  – Set Zero Flag............................................................................................ 167
            /*  115 */ InstructionType::SLEEP => 2usize, // ................................................................................................................. 168
            /*  116 */ InstructionType::SPM => 2usize, //  – Store Program Memory............................................................................169
            /*  117 */ InstructionType::SPM_2 => 2usize, //  SPM #2 – Store Program Memory.......................................................................171
            /*  118 */ 
            InstructionType::ST_STD_X_1 => 2usize,
            InstructionType::ST_STD_X_2 => 2usize,
            InstructionType::ST_STD_X_3 => 2usize,
            /*  119 */ 
            InstructionType::ST_STD_Y_1 => 2usize, 
            InstructionType::ST_STD_Y_2 => 2usize, 
            InstructionType::ST_STD_Y_3 => 2usize, 
            InstructionType::ST_STD_Y_4 => 2usize, 
            /*  120 */ 
            InstructionType::ST_STD_Z_1 => 2usize, 
            InstructionType::ST_STD_Z_2 => 2usize, 
            InstructionType::ST_STD_Z_3 => 2usize, 
            InstructionType::ST_STD_Z_4 => 2usize, 
            /*  121 */ InstructionType::STS => 2usize, //  STS – Store Direct to Data Space.......................................................................179
            /*  122 */ InstructionType::STS_16bit => 2usize, //  STS (16-bit) – Store Direct to Data Space.......................................................... 180
            /*  123 */ InstructionType::SUB => 2usize, //  – Subtract Without Carry.............................................................................181
            /*  124 */ InstructionType::SUBI => 2usize, //  – Subtract Immediate................................................................................. 183
            /*  125 */ InstructionType::SWAP => 2usize, //  – Swap Nibbles........................................................................................ 185
            /*  126 */ InstructionType::TST => 2usize, //  – Test for Zero or Minus...............................................................................186
            /*  127 */ InstructionType::WDR => 2usize, //  – Watchdog Reset......................................................................................187
            /*  128 */ InstructionType::XCH => 2usize,
            
            _ => 0,
        }
    }

    pub const fn words(instruction_type: &InstructionType) -> usize {

        match instruction_type {

            /*   5 */ InstructionType::ADC => 2usize,
            /*   6 */ InstructionType::ADD => 2usize,
            /*   7 */ InstructionType::ADIW => 2usize,
            /*   8 */ InstructionType::AND => 2usize,
            /*   9 */ InstructionType::ANDI => 2usize,
            /*  10 */ InstructionType::ASR => 2usize,

            /*  11 */ InstructionType::BCLR => 2usize,
            /*  12 */ InstructionType::BLD => 2usize,
            /*  13 */ InstructionType::BRBC => 2usize,
            /*  14 */ InstructionType::BRBS => 2usize,
            /*  15 */ InstructionType::BRCC => 2usize,
            /*  16 */ InstructionType::BRCS => 2usize,
            /*  17 */ InstructionType::BREAK => 2usize, 
            /*  18 */ InstructionType::BREQ => 2usize,
            /*  19 */ InstructionType::BRGE => 2usize,
            /*  20 */ InstructionType::BRHC => 2usize,
            /*  21 */ InstructionType::BRHS => 2usize,
            /*  22 */ InstructionType::BRID => 2usize,
            /*  23 */ InstructionType::BRIE => 2usize,
            /*  24 */ InstructionType::BRLO => 2usize,
            /*  25 */ InstructionType::BRLT => 2usize,
            /*  26 */ InstructionType::BRMI => 2usize,
            /*  27 */ InstructionType::BRNE => 2usize,
            /*  28 */ InstructionType::BRPL => 2usize,
            /*  29 */ InstructionType::BRSH => 2usize,
            /*  30 */ InstructionType::BRTC => 2usize,
            /*  31 */ InstructionType::BRTS => 2usize,
            /*  32 */ InstructionType::BRVC => 2usize,
            /*  33 */ InstructionType::BRVS => 2usize,
            /*  34 */ InstructionType::BSET => 2usize,
            /*  35 */ InstructionType::BST => 2usize,

            /*  36 */ InstructionType::CALL => 4usize,
            /*  37 */ InstructionType::CBI => 2usize, 
            /*  38 */ InstructionType::CBR => 2usize,
            /*  39 */ InstructionType::CLC => 2usize,
            /*  40 */ InstructionType::CLH => 2usize,
            /*  41 */ InstructionType::CLI => 2usize,
            /*  42 */ InstructionType::CLN => 2usize,
            /*  43 */ InstructionType::CLR => 2usize,
            /*  44 */ InstructionType::CLS => 2usize,
            /*  45 */ InstructionType::CLT => 2usize,
            /*  46 */ InstructionType::CLV => 2usize,
            /*  47 */ InstructionType::CLZ => 2usize,
            /*  48 */ InstructionType::COM => 2usize,
            /*  49 */ InstructionType::CP => 2usize,
            /*  50 */ InstructionType::CPC => 2usize,
            /*  51 */ InstructionType::CPI => 2usize,
            /*  52 */ InstructionType::CPSE => 2usize,

            /*  53 */ InstructionType::DEC => 2usize,
            /*  54 */ InstructionType::DES => 2usize,

            /*  55 */ InstructionType::EICALL => 2usize,
            /*  56 */ InstructionType::EIJMP => 2usize,
            /*  57 */ InstructionType::ELPM => 2usize,
            /*  58 */ InstructionType::EOR => 2usize,

            /*  59 */ InstructionType::FMUL => 2usize,
            /*  60 */ InstructionType::FMULS => 2usize,
            /*  61 */ InstructionType::FMULSU => 2usize,

            /*  62 */ InstructionType::ICALL => 2usize,
            /*  63 */ InstructionType::IJMP => 2usize,
            /*  64 */ InstructionType::IN => 2usize,
            /*  65 */ InstructionType::INC => 2usize,

            /*  66 */ InstructionType::JMP => 4usize,

            /*  67 */ InstructionType::LAC => 2usize,
            /*  68 */ InstructionType::LAS => 2usize,
            /*  69 */ InstructionType::LAT => 2usize, 
            /*  70 */ 
            InstructionType::LD_LDD_X_1 => 2usize,
            InstructionType::LD_LDD_X_2 => 2usize,
            InstructionType::LD_LDD_X_3 => 2usize,
            /*  71 */ 
            InstructionType::LD_LDD_Y_1 => 2usize, 
            InstructionType::LD_LDD_Y_2 => 2usize, 
            InstructionType::LD_LDD_Y_3 => 2usize, 
            InstructionType::LD_LDD_Y_4 => 2usize, 
            /*  72 */ 
            InstructionType::LD_LDD_Z_1 => 2usize, 
            InstructionType::LD_LDD_Z_2 => 2usize,
            InstructionType::LD_LDD_Z_3 => 2usize,
            InstructionType::LD_LDD_Z_4 => 2usize,
            /*  73 */ InstructionType::LDI => 2usize,
            /*  74 */ InstructionType::LDS => 4usize,
            /*  75 */ InstructionType::LDS_16bit => 2usize, //  (16-bit) – Load Direct from Data Space......................................................... 117
            /*  76 */ InstructionType::LPM => 2usize, //  – Load Program Memory...............................................................................118
            /*  77 */ InstructionType::LSL => 2usize, //  – Logical Shift Left..........................................................................................120
            /*  78 */ InstructionType::LSR => 2usize, //  – Logical Shift Right.......................................................................................122
            
            /*  79 */ InstructionType::MOV => 2usize, //  – Copy Register............................................................................................123
            /*  80 */ InstructionType::MOVW => 2usize, //  – Copy Register Word...............................................................................124
            /*  81 */ InstructionType::MUL => 2usize, //  – Multiply Unsigned.......................................................................................125
            /*  82 */ InstructionType::MULS => 2usize, //  – Multiply Signed........................................................................................ 126
            /*  83 */ InstructionType::MULSU  => 2usize, // 
            
            /*  84 */ InstructionType::NEG => 2usize,
            /*  85 */ InstructionType::NOP => 2usize, 
            
            /*  86 */ InstructionType::OR => 2usize,
            /*  87 */ InstructionType::ORI => 2usize,
            /*  88 */ InstructionType::OUT => 2usize,
           
            /*  89 */ InstructionType::POP => 2usize,
            /*  90 */ InstructionType::PUSH => 2usize,
            
            /*  91 */ InstructionType::RCALL => 2usize,
            /*  92 */ InstructionType::RET => 2usize,
            /*  93 */ InstructionType::RETI => 2usize, 
            /*  94 */ InstructionType::RJMP => 2usize,
            /*  95 */ InstructionType::ROL => 2usize,  
            /*  96 */ InstructionType::ROR => 2usize, //  – Rotate Right through Carry........................................................................145
            
            /*  97 */ InstructionType::SBC => 2usize, 
            /*  98 */ InstructionType::SBCI => 2usize,
            /*  99 */ InstructionType::SBI => 2usize, //  – Set Bit in I/O Register.................................................................................. 151
            /*  100 */ InstructionType::SBIC => 2usize, //  – Skip if Bit in I/O Register is Cleared........................................................ 152
            /*  101 */ InstructionType::SBIS => 2usize, //  – Skip if Bit in I/O Register is Set............................................................... 153
            /*  102 */ InstructionType::SBIW => 2usize, //  – Subtract Immediate from Word...............................................................154
            /*  103 */ InstructionType::SBR => 2usize, //  – Set Bits in Register...................................................................................156
            /*  104 */ InstructionType::SBRC => 2usize, //  – Skip if Bit in Register is Cleared............................................................ 157
            /*  105 */ InstructionType::SBRS => 2usize, //  – Skip if Bit in Register is Set....................................................................158
            /*  106 */ InstructionType::SEC => 2usize, //  – Set Carry Flag.......................................................................................... 159
            /*  107 */ InstructionType::SEH => 2usize, //  – Set Half Carry Flag...................................................................................160
            /*  108 */ InstructionType::SEI => 2usize, // 
            /*  109 */ InstructionType::SEN => 2usize, //  – Set Negative Flag.....................................................................................162
            /*  110 */ InstructionType::SER => 2usize, //  – Set all Bits in Register.............................................................................. 163
            /*  111 */ InstructionType::SES => 2usize, //  – Set Signed Flag........................................................................................ 164
            /*  112 */ InstructionType::SET => 2usize, //  – Set T Flag................................................................................................. 165
            /*  113 */ InstructionType::SEV => 2usize, //  – Set Overflow Flag..................................................................................... 166
            /*  114 */ InstructionType::SEZ => 2usize, //  – Set Zero Flag............................................................................................ 167
            /*  115 */ InstructionType::SLEEP => 2usize, // ................................................................................................................. 168
            /*  116 */ InstructionType::SPM => 2usize, //  – Store Program Memory............................................................................169
            /*  117 */ InstructionType::SPM_2 => 2usize, //  SPM #2 – Store Program Memory.......................................................................171
            /*  118 */ 
            InstructionType::ST_STD_X_1 => 2usize,
            InstructionType::ST_STD_X_2 => 2usize,
            InstructionType::ST_STD_X_3 => 2usize,
            /*  119 */ 
            InstructionType::ST_STD_Y_1 => 2usize, 
            InstructionType::ST_STD_Y_2 => 2usize, 
            InstructionType::ST_STD_Y_3 => 2usize, 
            InstructionType::ST_STD_Y_4 => 2usize, 
            /*  120 */ 
            InstructionType::ST_STD_Z_1 => 2usize, 
            InstructionType::ST_STD_Z_2 => 2usize,
            InstructionType::ST_STD_Z_3 => 2usize,
            InstructionType::ST_STD_Z_4 => 2usize,
            /*  121 */ InstructionType::STS => 4usize, //  STS – Store Direct to Data Space.......................................................................179
            /*  122 */ InstructionType::STS_16bit => 2usize, //  STS (16-bit) – Store Direct to Data Space.......................................................... 180
            /*  123 */ InstructionType::SUB => 2usize, //  – Subtract Without Carry.............................................................................181
            /*  124 */ InstructionType::SUBI => 2usize, //  – Subtract Immediate................................................................................. 183
            /*  125 */ InstructionType::SWAP => 2usize, //  – Swap Nibbles........................................................................................ 185
            
            /*  126 */ InstructionType::TST => 2usize, //  – Test for Zero or Minus...............................................................................186
            
            /*  127 */ InstructionType::WDR => 2usize, //  – Watchdog Reset......................................................................................187
            
            /*  128 */ InstructionType::XCH => 2usize,
            
            _ => 0,
        }
    }

}
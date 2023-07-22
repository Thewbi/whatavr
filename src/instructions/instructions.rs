use super::{instruction_definition::InstructionDefinition, instruction_type::InstructionType};

    /* 5 ADC */ const ADC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ADC), instruction_type: InstructionType::ADC, bit_pattern: "000111rdddddrrrr" };
    /* 6 ADD */ const ADD: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ADD), instruction_type: InstructionType::ADD, bit_pattern: "000111rdddddrrrr" };
    /* 7 ADIW */ const ADIW: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ADIW), instruction_type: InstructionType::ADIW, bit_pattern: "000111rdddddrrrr" };
    /* 8 AND */ const AND: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::AND), instruction_type: InstructionType::AND, bit_pattern: "000111rdddddrrrr" };
    /* 9 ANDI */ const ANDI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ANDI), instruction_type: InstructionType::ANDI, bit_pattern: "000111rdddddrrrr" };
    /* 10 ASR */ const ASR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ASR), instruction_type: InstructionType::ASR, bit_pattern: "000111rdddddrrrr" };
    /* 11 BCLR */ const BCLR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BCLR), instruction_type: InstructionType::BCLR, bit_pattern: "000111rdddddrrrr" };
    /* 12 BLD */ const BLD: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BLD), instruction_type: InstructionType::BLD, bit_pattern: "000111rdddddrrrr" };
    /* 13 BRBC */ const BRBC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRBC), instruction_type: InstructionType::BRBC, bit_pattern: "000111rdddddrrrr" };
    /* 14 BRBS */ const BRBS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRBS), instruction_type: InstructionType::BRBS, bit_pattern: "000111rdddddrrrr" };
    /* 15 BRCC */ const BRCC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRCC), instruction_type: InstructionType::BRCC, bit_pattern: "000111rdddddrrrr" };
    /* 16 => InstructionType::BRCS */ const BRCS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRCS), instruction_type: InstructionType::BRCS, bit_pattern: "000111rdddddrrrr" };
    /* 17 => InstructionType::BREAK */ const BREAK: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BREAK), instruction_type: InstructionType::BREAK, bit_pattern: "000111rdddddrrrr" };
    /* 18 => InstructionType::BREQ */ const BREQ: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BREQ), instruction_type: InstructionType::BREQ, bit_pattern: "000111rdddddrrrr" };
    /* 19 => InstructionType::BRGE */ const BRGE: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRGE), instruction_type: InstructionType::BRGE, bit_pattern: "000111rdddddrrrr" };
    /* 20 => InstructionType::BRHC */ const BRHC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRHC), instruction_type: InstructionType::BRHC, bit_pattern: "000111rdddddrrrr" };
    /* 21 => InstructionType::BRHS */ const BRHS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRHS), instruction_type: InstructionType::BRHS, bit_pattern: "000111rdddddrrrr" };
    /* 22 => InstructionType::BRID */ const BRID: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRID), instruction_type: InstructionType::BRID, bit_pattern: "000111rdddddrrrr" };
    /* 23 => InstructionType::BRIE */ const BRIE: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRIE), instruction_type: InstructionType::BRIE, bit_pattern: "000111rdddddrrrr" };
    /* 24 => InstructionType::BRLO */ const BRLO: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRLO), instruction_type: InstructionType::BRLO, bit_pattern: "000111rdddddrrrr" };
    /* 25 => InstructionType::BRLT */ const BRLT: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRLT), instruction_type: InstructionType::BRLT, bit_pattern: "000111rdddddrrrr" };
    /* 27 => InstructionType::BRNE */ const BRNE: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRNE), instruction_type: InstructionType::BRNE, bit_pattern: "000111rdddddrrrr" };
    /* 28 => InstructionType::BRPL */ const BRPL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRPL), instruction_type: InstructionType::BRPL, bit_pattern: "000111rdddddrrrr" };
    /* 29 => InstructionType::BRSH */ const BRSH: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRSH), instruction_type: InstructionType::BRSH, bit_pattern: "000111rdddddrrrr" };
    /* 30 => InstructionType::BRTC */ const BRTC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRTC), instruction_type: InstructionType::BRTC, bit_pattern: "000111rdddddrrrr" };
    /* 31 => InstructionType::BRTS */ const BRTS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRTS), instruction_type: InstructionType::BRTS, bit_pattern: "000111rdddddrrrr" };
    /* 32 => InstructionType::BRVC */ const BRVC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRVC), instruction_type: InstructionType::BRVC, bit_pattern: "000111rdddddrrrr" };
    /* 33 => InstructionType::BRVS */ const BRVS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRVS), instruction_type: InstructionType::BRVS, bit_pattern: "000111rdddddrrrr" };
    /* 34 => InstructionType::BSET */ const BSET: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BSET), instruction_type: InstructionType::BSET, bit_pattern: "000111rdddddrrrr" };
    /* 35 => InstructionType::BST */ const BST: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BST), instruction_type: InstructionType::BST, bit_pattern: "000111rdddddrrrr" };
    /* 36 */ const CALL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CALL), instruction_type: InstructionType::CALL, bit_pattern: "1001010kkkkk111k" };
    /* 37 => InstructionType::CBI */  const CBI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CBI), instruction_type: InstructionType::CBI, bit_pattern: "000111rdddddrrrr" };
    /* 38 => InstructionType::CBR */ const CBR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CBR), instruction_type: InstructionType::CBR, bit_pattern: "000111rdddddrrrr" };
    /* 39 => InstructionType::CLC */ const CLC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLC), instruction_type: InstructionType::CLC, bit_pattern: "000111rdddddrrrr" };
    /* 40 => InstructionType::CLH */ const CLH: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLH), instruction_type: InstructionType::CLH, bit_pattern: "000111rdddddrrrr" };
    /* 41 => InstructionType::CLI */ const CLI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLI), instruction_type: InstructionType::CLI, bit_pattern: "000111rdddddrrrr" };
    /* 42 => InstructionType::CLN */ const CLN: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLN), instruction_type: InstructionType::CLN, bit_pattern: "000111rdddddrrrr" };
    /* 43 => InstructionType::CLR */ const CLR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLR), instruction_type: InstructionType::CLR, bit_pattern: "000111rdddddrrrr" };
    /* 44 => InstructionType::CLS */ const CLS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLS), instruction_type: InstructionType::CLS, bit_pattern: "000111rdddddrrrr" };
    /* 45 => InstructionType::CLT */ const CLT: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLT), instruction_type: InstructionType::CLT, bit_pattern: "000111rdddddrrrr" };
    /* 46 => InstructionType::CLV */ const CLV: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLV), instruction_type: InstructionType::CLV, bit_pattern: "000111rdddddrrrr" };
    /* 47 => InstructionType::CLZ */ const CLZ: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLZ), instruction_type: InstructionType::CLZ, bit_pattern: "000111rdddddrrrr" };
    /* 48 => InstructionType::COM */ const COM: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::COM), instruction_type: InstructionType::COM, bit_pattern: "000111rdddddrrrr" };
    /* 49 => InstructionType::CP */ const CP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CP), instruction_type: InstructionType::CP, bit_pattern: "000111rdddddrrrr" };
    /* 50 => InstructionType::CPC */ const CPC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CPC), instruction_type: InstructionType::CPC, bit_pattern: "000111rdddddrrrr" };
    /* 51 => InstructionType::CPI */ const CPI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CPI), instruction_type: InstructionType::CPI, bit_pattern: "000111rdddddrrrr" };
    /* 52 => InstructionType::CPSE */ const CPSE: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CPSE), instruction_type: InstructionType::CPSE, bit_pattern: "000111rdddddrrrr" };
    /* 53 => InstructionType::DEC */ const DEC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::DEC), instruction_type: InstructionType::DEC, bit_pattern: "000111rdddddrrrr" };
    /* 54 => InstructionType::DES */ const DES: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::DES), instruction_type: InstructionType::DES, bit_pattern: "000111rdddddrrrr" };
    /* 55 => InstructionType::EICALL */ const EICALL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::EICALL), instruction_type: InstructionType::EICALL, bit_pattern: "000111rdddddrrrr" };
    /* 56 => InstructionType::EIJMP */ const EIJMP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::EIJMP), instruction_type: InstructionType::EIJMP, bit_pattern: "000111rdddddrrrr" };
    /* 57 => InstructionType::ELPM */ const ELPM: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ELPM), instruction_type: InstructionType::ELPM, bit_pattern: "000111rdddddrrrr" };
    /* 58 */ const EOR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::EOR), instruction_type: InstructionType::EOR, bit_pattern: "001001rdddddrrrr" };
    /* 59 => InstructionType::FMUL */ const FMUL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::FMUL), instruction_type: InstructionType::FMUL, bit_pattern: "000111rdddddrrrr" };
    /* 60 => InstructionType::FMULS */ const FMULS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::FMULS), instruction_type: InstructionType::FMULS, bit_pattern: "000111rdddddrrrr" };
    /* 61 => InstructionType::FMULSU */ const FMULSU: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::FMULSU), instruction_type: InstructionType::FMULSU, bit_pattern: "000111rdddddrrrr" };
    /* 62 => InstructionType::ICALL */ const ICALL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ICALL), instruction_type: InstructionType::ICALL, bit_pattern: "000111rdddddrrrr" };
    /* 63 => InstructionType::IJMP */ const IJMP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::IJMP), instruction_type: InstructionType::IJMP, bit_pattern: "000111rdddddrrrr" };
    /* 64 */ const IN: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::IN), instruction_type: InstructionType::IN, bit_pattern: "10110AAdddddAAAA" };
    /* 65 => InstructionType::INC */ const INC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::INC), instruction_type: InstructionType::INC, bit_pattern: "000111rdddddrrrr" };
    /* 66 */ const JMP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::JMP), instruction_type: InstructionType::JMP, bit_pattern: "1001010kkkkk110k" };
    /* 67 => InstructionType::LAC */ const LAC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LAC), instruction_type: InstructionType::LAC, bit_pattern: "000111rdddddrrrr" };
    /* 68 => InstructionType::LAS */ const LAS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LAS), instruction_type: InstructionType::LAS, bit_pattern: "000111rdddddrrrr" };
    /* 69 => InstructionType::LAT */  const LAT: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LAT), instruction_type: InstructionType::LAT, bit_pattern: "000111rdddddrrrr" };
    /* 70 => InstructionType::LD */ const LD: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LD), instruction_type: InstructionType::LD, bit_pattern: "000111rdddddrrrr" };
    /* 71 => InstructionType::LD_LDD_Y */ const LD_LDD_Y: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LD_LDD_Y), instruction_type: InstructionType::LD_LDD_Y, bit_pattern: "000111rdddddrrrr" };
    /* 72 => InstructionType::LD_LDD_Z */ const LD_LDD_Z: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LD_LDD_Z), instruction_type: InstructionType::LD_LDD_Z, bit_pattern: "000111rdddddrrrr" };
    /* 73 => InstructionType::LDI */ const LDI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LDI), instruction_type: InstructionType::LDI, bit_pattern: "1110KKKKddddKKKK" };
    /* 74 => InstructionType::LDS */ const LDS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LDS), instruction_type: InstructionType::LDS, bit_pattern: "000111rdddddrrrr" };
    /* 75 => InstructionType::LDS_16bit */  const LDS_16bit: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LDS_16bit), instruction_type: InstructionType::LDS_16bit, bit_pattern: "000111rdddddrrrr" };
    /* 76 => InstructionType::LPM */  const LPM: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LPM), instruction_type: InstructionType::LPM, bit_pattern: "000111rdddddrrrr" };
    /* 77 => InstructionType::LSL */  const LSL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LSL), instruction_type: InstructionType::LSL, bit_pattern: "000111rdddddrrrr" };
    /* 78 => InstructionType::LSR */  const LSR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LSR), instruction_type: InstructionType::LSR, bit_pattern: "000111rdddddrrrr" };
    /* 79 => InstructionType::MOV */  const MOV: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MOV), instruction_type: InstructionType::MOV, bit_pattern: "000111rdddddrrrr" };
    /* 80 => InstructionType::MOVW */ const MOVW: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MOVW), instruction_type: InstructionType::MOVW, bit_pattern: "000111rdddddrrrr" };
    /* 81 => InstructionType::MUL */  const MUL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MUL), instruction_type: InstructionType::MUL, bit_pattern: "000111rdddddrrrr" };
    /* 82 => InstructionType::MULS */  const MULS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MULS), instruction_type: InstructionType::MULS, bit_pattern: "000111rdddddrrrr" };
    /* 83 => InstructionType::MULSU */  const MULSU: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MULSU), instruction_type: InstructionType::MULSU, bit_pattern: "000111rdddddrrrr" };
    /* 84 => InstructionType::NEG */ const NEG: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::NEG), instruction_type: InstructionType::NEG, bit_pattern: "000111rdddddrrrr" };
    /* 85 => InstructionType::NOP */  const NOP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::NOP), instruction_type: InstructionType::NOP, bit_pattern: "000111rdddddrrrr" };
    /* 86 => InstructionType::OR */ const OR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::OR), instruction_type: InstructionType::OR, bit_pattern: "000111rdddddrrrr" };
    /* 87 => InstructionType::ORI */ const ORI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ORI), instruction_type: InstructionType::ORI, bit_pattern: "000111rdddddrrrr" };
    /* 88 */ const OUT: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::OUT), instruction_type: InstructionType::OUT, bit_pattern: "10111AArrrrrAAAA" };
    /* 89 => InstructionType::POP */ const POP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::POP), instruction_type: InstructionType::POP, bit_pattern: "000111rdddddrrrr" };
    /* 90 => InstructionType::PUSH */ const PUSH: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::PUSH), instruction_type: InstructionType::PUSH, bit_pattern: "1001001ddddd1111" };
    /* 91 => InstructionType::RCALL */ const RCALL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RCALL), instruction_type: InstructionType::RCALL, bit_pattern: "000111rdddddrrrr" };
    /* 92 => InstructionType::RET */ const RET: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RET), instruction_type: InstructionType::RET, bit_pattern: "000111rdddddrrrr" };
    /* 93 => InstructionType::RETI */  const RETI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RETI), instruction_type: InstructionType::RETI, bit_pattern: "000111rdddddrrrr" };
    /* 94 => InstructionType::RJMP */ const RJMP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RJMP), instruction_type: InstructionType::RJMP, bit_pattern: "000111rdddddrrrr" };
    /* 95 => InstructionType::ROL */  const ROL: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ROL), instruction_type: InstructionType::ROL, bit_pattern: "000111rdddddrrrr" };
    /* 96 => InstructionType::ROR */  const ROR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ROR), instruction_type: InstructionType::ROR, bit_pattern: "000111rdddddrrrr" };
    /* 97 => InstructionType::SBC */  const SBC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBC), instruction_type: InstructionType::SBC, bit_pattern: "000111rdddddrrrr" };
    /* 98 => InstructionType::SBCI */ const SBCI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBCI), instruction_type: InstructionType::SBCI, bit_pattern: "000111rdddddrrrr" };
    /* 99 => InstructionType::SBI */  const SBI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBI), instruction_type: InstructionType::SBI, bit_pattern: "000111rdddddrrrr" };
    /* 100 => InstructionType::SBIC */  const SBIC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBIC), instruction_type: InstructionType::SBIC, bit_pattern: "000111rdddddrrrr" };
    /* 101 => InstructionType::SBIS */  const SBIS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBIS), instruction_type: InstructionType::SBIS, bit_pattern: "000111rdddddrrrr" };
    /* 102 => InstructionType::SBIW */  const SBIW: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBIW), instruction_type: InstructionType::SBIW, bit_pattern: "000111rdddddrrrr" };
    /* 103 => InstructionType::SBR */  const SBR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBR), instruction_type: InstructionType::SBR, bit_pattern: "000111rdddddrrrr" };
    /* 104 => InstructionType::SBRC */  const SBRC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBRC), instruction_type: InstructionType::SBRC, bit_pattern: "000111rdddddrrrr" };
    /* 105 => InstructionType::SBRS */  const SBRS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBRS), instruction_type: InstructionType::SBRS, bit_pattern: "000111rdddddrrrr" };
    /* 106 => InstructionType::SEC */  const SEC: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEC), instruction_type: InstructionType::SEC, bit_pattern: "000111rdddddrrrr" };
    /* 107 => InstructionType::SEH */  const SEH: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEH), instruction_type: InstructionType::SEH, bit_pattern: "000111rdddddrrrr" };
    /* 108 => InstructionType::SEI */  const SEI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEI), instruction_type: InstructionType::SEI, bit_pattern: "000111rdddddrrrr" };
    /* 109 => InstructionType::SEN */  const SEN: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEN), instruction_type: InstructionType::SEN, bit_pattern: "000111rdddddrrrr" };
    /* 110 => InstructionType::SER */  const SER: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SER), instruction_type: InstructionType::SER, bit_pattern: "000111rdddddrrrr" };
    /* 111 => InstructionType::SES */  const SES: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SES), instruction_type: InstructionType::SES, bit_pattern: "000111rdddddrrrr" };
    /* 112 => InstructionType::SET */  const SET: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SET), instruction_type: InstructionType::SET, bit_pattern: "000111rdddddrrrr" };
    /* 113 => InstructionType::SEV */  const SEV: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEV), instruction_type: InstructionType::SEV, bit_pattern: "000111rdddddrrrr" };
    /* 114 => InstructionType::SEZ */  const SEZ: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEZ), instruction_type: InstructionType::SEZ, bit_pattern: "000111rdddddrrrr" };
    /* 115 => InstructionType::SLEEP */  const SLEEP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SLEEP), instruction_type: InstructionType::SLEEP, bit_pattern: "000111rdddddrrrr" };
    /* 116 => InstructionType::SPM */  const SPM: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SPM), instruction_type: InstructionType::SPM, bit_pattern: "000111rdddddrrrr" };
    /* 117 => InstructionType::SPM_2 */  const SPM_2: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SPM_2), instruction_type: InstructionType::SPM_2, bit_pattern: "000111rdddddrrrr" };
    /* 118 => InstructionType::ST */  const ST: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST), instruction_type: InstructionType::ST, bit_pattern: "000111rdddddrrrr" };
    /*  119 => InstructionType::ST_STD_Y */   const ST_STD_Y: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Y), instruction_type: InstructionType::ST_STD_Y, bit_pattern: "000111rdddddrrrr" };
    /*  120 => InstructionType::ST_STD_Z */   const ST_STD_Z: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Z), instruction_type: InstructionType::ST_STD_Z, bit_pattern: "000111rdddddrrrr" };
    /*  121 => InstructionType::STS */  const STS: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::STS), instruction_type: InstructionType::STS, bit_pattern: "000111rdddddrrrr" };
    /*  122 => InstructionType::STS_16bit */  const STS_16bit: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::STS_16bit), instruction_type: InstructionType::STS_16bit, bit_pattern: "000111rdddddrrrr" };
    /*  123 => InstructionType::SUB */  const SUB: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SUB), instruction_type: InstructionType::SUB, bit_pattern: "000111rdddddrrrr" };
    /*  124 => InstructionType::SUBI */  const SUBI: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SUBI), instruction_type: InstructionType::SUBI, bit_pattern: "000111rdddddrrrr" };
    /*  125 => InstructionType::SWAP */  const SWAP: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SWAP), instruction_type: InstructionType::SWAP, bit_pattern: "000111rdddddrrrr" };
    /*  126 => InstructionType::TST */  const TST: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::TST), instruction_type: InstructionType::TST, bit_pattern: "000111rdddddrrrr" };
    /*  127 => InstructionType::WDR */  const WDR: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::WDR), instruction_type: InstructionType::WDR, bit_pattern: "000111rdddddrrrr" };
    /*  128 => InstructionType::XCH */ const XCH: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::XCH), instruction_type: InstructionType::XCH, bit_pattern: "000111rdddddrrrr" };
    pub const unknown: InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::Unknown), instruction_type: InstructionType::Unknown, bit_pattern: "xxxxxxxxxxxxxxxx" };

    
    pub const instructions: &'static [InstructionDefinition] = &[ 
        ADC, 
        ADD, 
        ADIW, 
        AND,
        ANDI,
        ASR,
        BCLR,
        BLD,
        BRBC,
        BRBS,
        BRCC,
        BRCS,
        BREAK,
        BREQ,
        BRGE,
        BRHC,
        BRHS,
        BRID,
        BRIE,
        BRLO,
        BRLT,
        BRNE,
        BRPL,
        BRSH,
        BRTC,
        BRTS,
        BRVC,
        BRVS,
        BSET,
        BST,
        CALL,
        CBI,
        CBR,
        CLC,
        CLH,
        CLI,
        CLN,
        CLR,
        CLS,
        CLT,
        CLV,
        CLZ,
        COM,
        CP,
        CPC,
        CPI,
        CPSE,
        DEC,
        DES,
        EICALL,
        EIJMP,
        ELPM,
        EOR,
        FMUL,
        FMULS,
        FMULSU,
        ICALL,
        IJMP,
        IN,
        INC,
        JMP,
        LAC,
        LAS,
        LAT,
        LD,
        LD_LDD_Y,
        LD_LDD_Z,
        LDI,
        LDS,
        LDS_16bit,
        LPM,
        LSL,
        LSR,
        MOV,
        MOVW,
        MUL,
        MULS,
        MULSU,
        NEG,
        NOP,
        OR,
        ORI,
        OUT,
        POP,
        PUSH,
        RCALL,
        RET,
        RETI,
        RJMP,
        ROL,
        ROR,
        SBC,
        SBCI,
        SBI,
        SBIC,
        SBIS,
        SBIW,
        SBR,
        SBRC,
        SBRS,
        SEC,
        SEH,
        SEI,
        SEN,
        SER,
        SES,
        SET,
        SEV,
        SEZ,
        SLEEP,
        SPM,
        SPM_2,
        ST,
        ST_STD_Y,
        ST_STD_Z,
        STS,
        STS_16bit,
        SUB,
        SUBI,
        SWAP,
        TST,
        WDR,
        XCH
    ];

// pub fn prepare_instructions(instructions: &mut Vec<InstructionDefinition>)
// {
//     //let mut instructions: Vec<InstructionDefinition> = Vec::new();
//     instructions.push(adc);
//     instructions.push(add);
//     // instructions.push(adiw);
//     // instructions.push(and);
//     // instructions.push(andi);
//     // instructions.push(asr);
//     // instructions.push(bclr);
//     // instructions.push(bld);
//     // instructions.push(brbc);
//     // instructions.push(brbs);
//     // instructions.push(brcc);
//     // instructions.push(BRCS);
//     // instructions.push(BREAK);
//     // instructions.push(BREQ);
//     // instructions.push(BRGE);
//     // instructions.push(BRHC);
//     // instructions.push(BRHS);
//     // instructions.push(BRID);
//     // instructions.push(BRIE);
//     // instructions.push(BRLO);
//     // instructions.push(BRLT);
//     // instructions.push(BRNE);
//     // instructions.push(BRPL);
//     // instructions.push(BRSH);
//     // instructions.push(BRTC);
//     // instructions.push(BRTS);
//     // instructions.push(BRVC);
//     // instructions.push(BRVS);
//     // instructions.push(BSET);
//     // instructions.push(BST);
//     // instructions.push(call);
//     // instructions.push(CBI);
//     // instructions.push(CBR);
//     // instructions.push(CLC);
//     // instructions.push(CLH);
//     // instructions.push(CLI);
//     // instructions.push(CLN);
//     // instructions.push(CLR);
//     // instructions.push(CLS);
//     // instructions.push(CLT);
//     // instructions.push(CLV);
//     // instructions.push(CLZ);
//     // instructions.push(COM);
//     // instructions.push(CP);
//     // instructions.push(CPC);
//     // instructions.push(CPI);
//     // instructions.push(CPSE);
//     // instructions.push(DEC);
//     // instructions.push(DES);
//     // instructions.push(EICALL);
//     // instructions.push(EIJMP);
//     // instructions.push(ELPM);
//     // instructions.push(eor);
//     // instructions.push(FMUL);
//     // instructions.push(FMULS);
//     // instructions.push(FMULSU);
//     // instructions.push(ICALL);
//     // instructions.push(IJMP);
//     // instructions.push(in_);
//     // instructions.push(INC);
//     // instructions.push(jmp);
//     // instructions.push(LAC);
//     // instructions.push(LAS);
//     // instructions.push(LAT);
//     // instructions.push(LD);
//     // instructions.push(LD_LDD_Y);
//     // instructions.push(LD_LDD_Z);
//     // instructions.push(ldi);
//     // instructions.push(LDS);
//     // instructions.push(LDS_16bit);
//     // instructions.push(LPM);
//     // instructions.push(LSL);
//     // instructions.push(LSR);
//     // instructions.push(MOV);
//     // instructions.push(MOVW);
//     // instructions.push(MUL);
//     // instructions.push(MULS);
//     // instructions.push(MULSU);
//     // instructions.push(NEG);
//     // instructions.push(NOP);
//     // instructions.push(OR);
//     // instructions.push(ORI);
//     // instructions.push(out);
//     // instructions.push(POP);
//     // instructions.push(push);
//     // instructions.push(RCALL);
//     // instructions.push(RET);
//     // instructions.push(RETI);
//     // instructions.push(RJMP);
//     // instructions.push(ROL);
//     // instructions.push(ROR);
//     // instructions.push(SBC);
//     // instructions.push(SBCI);
//     // instructions.push(SBI);
//     // instructions.push(SBIC);
//     // instructions.push(SBIS);
//     // instructions.push(SBIW);
//     // instructions.push(SBR);
//     // instructions.push(SBRC);
//     // instructions.push(SBRS);
//     // instructions.push(SEC);
//     // instructions.push(SEH);
//     // instructions.push(SEI);
//     // instructions.push(SEN);
//     // instructions.push(SER);
//     // instructions.push(SES);
//     // instructions.push(SET);
//     // instructions.push(SEV);
//     // instructions.push(SEZ);
//     // instructions.push(SLEEP);
//     // instructions.push(SPM);
//     // instructions.push(SPM_2);
//     // instructions.push(ST);
//     // instructions.push(ST_STD_Y);
//     // instructions.push(ST_STD_Z);
//     // instructions.push(STS);
//     // instructions.push(STS_16bit);
//     // instructions.push(SUB);
//     // instructions.push(SUBI);
//     // instructions.push(SWAP);
//     // instructions.push(TST);
//     // instructions.push(WDR);
//     // instructions.push(XCH);

//     //return instructions;
// }
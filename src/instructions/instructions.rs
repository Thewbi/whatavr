use super::{instruction_definition::InstructionDefinition, instruction_type::InstructionType};

    /* 5 => InstructionType::ADC */             const ADC:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ADC),         instruction_type: InstructionType::ADC,         bit_pattern: "0001 11rd dddd rrrr" };
    /* 6 => InstructionType::ADD */             const ADD:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ADD),         instruction_type: InstructionType::ADD,         bit_pattern: "0000 11rd dddd rrrr" };
    /* 7 => InstructionType::ADIW */            const ADIW:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ADIW),        instruction_type: InstructionType::ADIW,        bit_pattern: "1001 0110 KKdd KKKK" };
    /* 8 => InstructionType::AND */             const AND:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::AND),         instruction_type: InstructionType::AND,         bit_pattern: "0010 00rd dddd rrrr" };
    /* 9 => InstructionType::ANDI */            const ANDI:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ANDI),        instruction_type: InstructionType::ANDI,        bit_pattern: "0111 KKKK dddd KKKK" };
    /* 10 => InstructionType::ASR */            const ASR:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ASR),         instruction_type: InstructionType::ASR,         bit_pattern: "1001 010d dddd 0101" };
    
    /* 11 => InstructionType::BCLR */           const BCLR:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BCLR),        instruction_type: InstructionType::BCLR,        bit_pattern: "1001 0100 1sss 1000" };
    /* 12 => InstructionType::BLD */            const BLD:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BLD),         instruction_type: InstructionType::BLD,         bit_pattern: "1111 100d dddd 0bbb" };
    /* 13 => InstructionType::BRBC */           const BRBC:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRBC),        instruction_type: InstructionType::BRBC,        bit_pattern: "1111 01kk kkkk ksss" };
    /* 14 => InstructionType::BRBS */           const BRBS:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRBS),        instruction_type: InstructionType::BRBS,        bit_pattern: "1111 00kk kkkk ksss" };
    /* 15 => InstructionType::BRCC */           const BRCC:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRCC),        instruction_type: InstructionType::BRCC,        bit_pattern: "1111 01kk kkkk k000" };
    /* 16 => InstructionType::BRCS */           const BRCS:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRCS),        instruction_type: InstructionType::BRCS,        bit_pattern: "1111 00kk kkkk k000" };
    /* 17 => InstructionType::BREAK */          const BREAK:        InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BREAK),       instruction_type: InstructionType::BREAK,       bit_pattern: "1001 0101 1001 1000" };
    /* 18 => InstructionType::BREQ */           const BREQ:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BREQ),        instruction_type: InstructionType::BREQ,        bit_pattern: "1111 00kk kkkk k001" };
    /* 19 => InstructionType::BRGE */           const BRGE:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRGE),        instruction_type: InstructionType::BRGE,        bit_pattern: "1111 01kk kkkk k100" };
    /* 20 => InstructionType::BRHC */           const BRHC:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRHC),        instruction_type: InstructionType::BRHC,        bit_pattern: "1111 01kk kkkk k101" };
    /* 21 => InstructionType::BRHS */           const BRHS:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRHS),        instruction_type: InstructionType::BRHS,        bit_pattern: "1111 00kk kkkk k101" };
    /* 22 => InstructionType::BRID */           const BRID:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRID),        instruction_type: InstructionType::BRID,        bit_pattern: "1111 01kk kkkk k111" };
    /* 23 => InstructionType::BRIE */           const BRIE:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRIE),        instruction_type: InstructionType::BRIE,        bit_pattern: "1111 00kk kkkk k111" };
    /* 24 => InstructionType::BRLO */           const BRLO:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRLO),        instruction_type: InstructionType::BRLO,        bit_pattern: "1111 00kk kkkk k000" };
    /* 25 => InstructionType::BRLT */           const BRLT:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRLT),        instruction_type: InstructionType::BRLT,        bit_pattern: "1111 00kk kkkk k100" };
    /* 26 => InstructionType::BRMI */           const BRMI:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRMI),        instruction_type: InstructionType::BRMI,        bit_pattern: "1111 00kk kkkk k010" };
    /* 27 => InstructionType::BRNE */           const BRNE:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRNE),        instruction_type: InstructionType::BRNE,        bit_pattern: "1111 01kk kkkk k001" };
    /* 28 => InstructionType::BRPL */           const BRPL:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRPL),        instruction_type: InstructionType::BRPL,        bit_pattern: "1111 01kk kkkk k010" };
    /* 29 => InstructionType::BRSH */           const BRSH:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRSH),        instruction_type: InstructionType::BRSH,        bit_pattern: "1111 01kk kkkk k000" };
    /* 30 => InstructionType::BRTC */           const BRTC:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRTC),        instruction_type: InstructionType::BRTC,        bit_pattern: "1111 01kk kkkk k110" };
    /* 31 => InstructionType::BRTS */           const BRTS:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRTS),        instruction_type: InstructionType::BRTS,        bit_pattern: "1111 00kk kkkk k110" };
    /* 32 => InstructionType::BRVC */           const BRVC:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRVC),        instruction_type: InstructionType::BRVC,        bit_pattern: "1111 01kk kkkk k011" };
    /* 33 => InstructionType::BRVS */           const BRVS:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BRVS),        instruction_type: InstructionType::BRVS,        bit_pattern: "1111 00kk kkkk k011" };
    /* 34 => InstructionType::BSET */           const BSET:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BSET),        instruction_type: InstructionType::BSET,        bit_pattern: "1001 0100 0sss 1000" };
    /* 35 => InstructionType::BST */            const BST:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::BST),         instruction_type: InstructionType::BST,         bit_pattern: "1111 101d dddd 0bbb" };
    
    /* 36 => InstructionType::CALL */           const CALL:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CALL),        instruction_type: InstructionType::CALL,        bit_pattern: "1001 010k kkkk 111k" };
    /* 37 => InstructionType::CBI */            const CBI:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CBI),         instruction_type: InstructionType::CBI,         bit_pattern: "1001 1000 AAAA Abbb" };
    // /* 38 => InstructionType::CBR */            const CBR:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CBR),         instruction_type: InstructionType::CBR,         bit_pattern: "" };
    /* 39 => InstructionType::CLC */            const CLC:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLC),         instruction_type: InstructionType::CLC,         bit_pattern: "1001 0100 1000 1000" };
    /* 40 => InstructionType::CLH */            const CLH:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLH),         instruction_type: InstructionType::CLH,         bit_pattern: "1001 0100 1101 1000" };
    /* 41 => InstructionType::CLI */            const CLI:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLI),         instruction_type: InstructionType::CLI,         bit_pattern: "1001 0100 1111 1000" };
    /* 42 => InstructionType::CLN */            const CLN:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLN),         instruction_type: InstructionType::CLN,         bit_pattern: "1001 0100 1010 1000" };
    /* 43 => InstructionType::CLR */            const CLR:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLR),         instruction_type: InstructionType::CLR,         bit_pattern: "0010 01dd dddd dddd" };
    /* 44 => InstructionType::CLS */            const CLS:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLS),         instruction_type: InstructionType::CLS,         bit_pattern: "1001 0100 1100 1000" };
    /* 45 => InstructionType::CLT */            const CLT:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLT),         instruction_type: InstructionType::CLT,         bit_pattern: "1001 0100 1110 1000" };
    /* 46 => InstructionType::CLV */            const CLV:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLV),         instruction_type: InstructionType::CLV,         bit_pattern: "1001 0100 1011 1000" };
    /* 47 => InstructionType::CLZ */            const CLZ:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CLZ),         instruction_type: InstructionType::CLZ,         bit_pattern: "1001 0100 1001 1000" };
    /* 48 => InstructionType::COM */            const COM:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::COM),         instruction_type: InstructionType::COM,         bit_pattern: "1001 010d dddd 0000" };
    /* 49 => InstructionType::CP */             const CP:           InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CP),          instruction_type: InstructionType::CP,          bit_pattern: "0001 01rd dddd rrrr" };
    /* 50 => InstructionType::CPC */            const CPC:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CPC),         instruction_type: InstructionType::CPC,         bit_pattern: "0000 01rd dddd rrrr" };
    /* 51 => InstructionType::CPI */            const CPI:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CPI),         instruction_type: InstructionType::CPI,         bit_pattern: "0011 KKKK dddd KKKK" };
    /* 52 => InstructionType::CPSE */           const CPSE:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::CPSE),        instruction_type: InstructionType::CPSE,        bit_pattern: "0001 00rd dddd rrrr" };
    
    /* 53 => InstructionType::DEC */            const DEC:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::DEC),         instruction_type: InstructionType::DEC,         bit_pattern: "1001 010d dddd 1010" };
    /* 54 => InstructionType::DES */            const DES:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::DES),         instruction_type: InstructionType::DES,         bit_pattern: "1001 0100 KKKK 1011" };
    
    /* 55 => InstructionType::EICALL */         const EICALL:       InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::EICALL),      instruction_type: InstructionType::EICALL,      bit_pattern: "1001 0101 0001 1001" };
    /* 56 => InstructionType::EIJMP */          const EIJMP:        InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::EIJMP),       instruction_type: InstructionType::EIJMP,       bit_pattern: "1001 0100 0001 1001" };
    // /* 57 => InstructionType::ELPM */           const ELPM:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ELPM),        instruction_type: InstructionType::ELPM,        bit_pattern: "" };
    /* 58 => InstructionType::EOR */            const EOR:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::EOR),         instruction_type: InstructionType::EOR,         bit_pattern: "0010 01rd dddd rrrr" };
    
    /* 59 => InstructionType::FMUL */           const FMUL:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::FMUL),        instruction_type: InstructionType::FMUL,        bit_pattern: "0000 0011 0ddd 1rrr" };
    /* 60 => InstructionType::FMULS */          const FMULS:        InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::FMULS),       instruction_type: InstructionType::FMULS,       bit_pattern: "0000 0011 1ddd 0rrr" };
    /* 61 => InstructionType::FMULSU */         const FMULSU:       InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::FMULSU),      instruction_type: InstructionType::FMULSU,      bit_pattern: "0000 0011 1ddd 1rrr" };
    
    /* 62 => InstructionType::ICALL */          const ICALL:        InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ICALL),       instruction_type: InstructionType::ICALL,       bit_pattern: "1001 0101 0000 1001" };
    /* 63 => InstructionType::IJMP */           const IJMP:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::IJMP),        instruction_type: InstructionType::IJMP,        bit_pattern: "1001 0100 0000 1001" };
    /* 64 */                                    const IN:           InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::IN),          instruction_type: InstructionType::IN,          bit_pattern: "1011 0AAd dddd AAAA" };
    /* 65 => InstructionType::INC */            const INC:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::INC),         instruction_type: InstructionType::INC,         bit_pattern: "1001 010d dddd 0011" };
    
    /* 66 => InstructionType::JMP */            const JMP:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::JMP),         instruction_type: InstructionType::JMP,         bit_pattern: "1001 010k kkkk 110k" };
    
    /* 67 => InstructionType::LAC */            const LAC:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LAC),         instruction_type: InstructionType::LAC,         bit_pattern: "1001 001r rrrr 0110" };
    /* 68 => InstructionType::LAS */            const LAS:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LAS),         instruction_type: InstructionType::LAS,         bit_pattern: "1001 001r rrrr 0101" };
    /* 69 => InstructionType::LAT */            const LAT:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LAT),         instruction_type: InstructionType::LAT,         bit_pattern: "1001 001r rrrr 0111" };
    // /* 70 => InstructionType::LD */             const LD:           InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LD),          instruction_type: InstructionType::LD,          bit_pattern: "" };
    // /* 71 => InstructionType::LD_LDD_Y */       const LD_LDD_Y:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LD_LDD_Y),    instruction_type: InstructionType::LD_LDD_Y,    bit_pattern: "" };
    // /* 72 => InstructionType::LD_LDD_Z */       const LD_LDD_Z:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LD_LDD_Z),    instruction_type: InstructionType::LD_LDD_Z,    bit_pattern: "" };
    /* 73 => InstructionType::LDI */            const LDI:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LDI),         instruction_type: InstructionType::LDI,         bit_pattern: "1110 KKKK dddd KKKK" };
    /* 74 => InstructionType::LDS */            const LDS:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LDS),         instruction_type: InstructionType::LDS,         bit_pattern: "1001 000d dddd 0000" };
    /* 75 => InstructionType::LDS_16bit */      const LDS_16bit:    InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LDS_16bit),   instruction_type: InstructionType::LDS_16bit,   bit_pattern: "1010 0kkk dddd kkkk" };
    // /* 76 => InstructionType::LPM */            const LPM:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LPM),         instruction_type: InstructionType::LPM,         bit_pattern: "" };
    /* 77 => InstructionType::LSL */            const LSL:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LSL),         instruction_type: InstructionType::LSL,         bit_pattern: "0000 11dd dddd dddd" };
    /* 78 => InstructionType::LSR */            const LSR:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::LSR),         instruction_type: InstructionType::LSR,         bit_pattern: "1001 010d dddd 0110" };
    
    /* 79 => InstructionType::MOV */            const MOV:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MOV),         instruction_type: InstructionType::MOV,         bit_pattern: "0010 11rd dddd rrrr" };
    /* 80 => InstructionType::MOVW */           const MOVW:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MOVW),        instruction_type: InstructionType::MOVW,        bit_pattern: "0000 0001 dddd rrrr" };
    /* 81 => InstructionType::MUL */            const MUL:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MUL),         instruction_type: InstructionType::MUL,         bit_pattern: "1001 11rd dddd rrrr" };
    /* 82 => InstructionType::MULS */           const MULS:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MULS),        instruction_type: InstructionType::MULS,        bit_pattern: "0000 0010 dddd rrrr" };
    /* 83 => InstructionType::MULSU */          const MULSU:        InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::MULSU),       instruction_type: InstructionType::MULSU,       bit_pattern: "0000 0011 0ddd 0rrr" };
    
    /* 84 => InstructionType::NEG */            const NEG:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::NEG),         instruction_type: InstructionType::NEG,         bit_pattern: "1001 010d dddd 0001" };
    /* 85 => InstructionType::NOP */            const NOP:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::NOP),         instruction_type: InstructionType::NOP,         bit_pattern: "0000 0000 0000 0000" };
    
    /* 86 => InstructionType::OR */             const OR:           InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::OR),          instruction_type: InstructionType::OR,          bit_pattern: "0010 10rd dddd rrrr" };
    /* 87 => InstructionType::ORI */            const ORI:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ORI),         instruction_type: InstructionType::ORI,         bit_pattern: "0110 KKKK dddd KKKK" };
    /* 88 => InstructionType::OUT*/             const OUT:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::OUT),         instruction_type: InstructionType::OUT,         bit_pattern: "1011 1AAr rrrr AAAA" };
    
    /* 89 => InstructionType::POP */            const POP:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::POP),         instruction_type: InstructionType::POP,         bit_pattern: "1001 000d dddd 1111" };
    /* 90 => InstructionType::PUSH */           const PUSH:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::PUSH),        instruction_type: InstructionType::PUSH,        bit_pattern: "1001 001d dddd 1111" };
    
    /* 91 => InstructionType::RCALL */          const RCALL:        InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RCALL),       instruction_type: InstructionType::RCALL,       bit_pattern: "1101 kkkk kkkk kkkk" };
    /* 92 => InstructionType::RET */            const RET:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RET),         instruction_type: InstructionType::RET,         bit_pattern: "1001 0101 0000 1000" };
    /* 93 => InstructionType::RETI */           const RETI:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RETI),        instruction_type: InstructionType::RETI,        bit_pattern: "1001 0101 0001 1000" };
    /* 94 => InstructionType::RJMP */           const RJMP:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::RJMP),        instruction_type: InstructionType::RJMP,        bit_pattern: "1100 kkkk kkkk kkkk" };
    /* 95 => InstructionType::ROL */            const ROL:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ROL),         instruction_type: InstructionType::ROL,         bit_pattern: "0001 11dd dddd dddd" };
    /* 96 => InstructionType::ROR */            const ROR:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ROR),         instruction_type: InstructionType::ROR,         bit_pattern: "1001 010d dddd 0111" };
    
    /* 97 => InstructionType::SBC */            const SBC:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBC),         instruction_type: InstructionType::SBC,         bit_pattern: "0000 10rd dddd rrrr" };
    /* 98 => InstructionType::SBCI */           const SBCI:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBCI),        instruction_type: InstructionType::SBCI,        bit_pattern: "0100 KKKK dddd KKKK" };
    /* 99 => InstructionType::SBI */            const SBI:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBI),         instruction_type: InstructionType::SBI,         bit_pattern: "1001 1010 AAAA Abbb" };
    /* 100 => InstructionType::SBIC */          const SBIC:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBIC),        instruction_type: InstructionType::SBIC,        bit_pattern: "1001 1001 AAAA Abbb" };
    /* 101 => InstructionType::SBIS */          const SBIS:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBIS),        instruction_type: InstructionType::SBIS,        bit_pattern: "1001 1011 AAAA Abbb" };
    /* 102 => InstructionType::SBIW */          const SBIW:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBIW),        instruction_type: InstructionType::SBIW,        bit_pattern: "1001 0111 KKdd KKKK" };
    /* 103 => InstructionType::SBR */           const SBR:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBR),         instruction_type: InstructionType::SBR,         bit_pattern: "0110 KKKK dddd KKKK" };
    /* 104 => InstructionType::SBRC */          const SBRC:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBRC),        instruction_type: InstructionType::SBRC,        bit_pattern: "1111 110r rrrr 0bbb" };
    /* 105 => InstructionType::SBRS */          const SBRS:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SBRS),        instruction_type: InstructionType::SBRS,        bit_pattern: "1111 111r rrrr 0bbb" };
    /* 106 => InstructionType::SEC */           const SEC:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEC),         instruction_type: InstructionType::SEC,         bit_pattern: "1001 0100 0000 1000" };
    /* 107 => InstructionType::SEH */           const SEH:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEH),         instruction_type: InstructionType::SEH,         bit_pattern: "1001 0100 0101 1000" };
    /* 108 => InstructionType::SEI */           const SEI:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEI),         instruction_type: InstructionType::SEI,         bit_pattern: "1001 0100 0111 1000" };
    /* 109 => InstructionType::SEN */           const SEN:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEN),         instruction_type: InstructionType::SEN,         bit_pattern: "1001 0100 0010 1000" };
    /* 110 => InstructionType::SER */           const SER:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SER),         instruction_type: InstructionType::SER,         bit_pattern: "1110 1111 dddd 1111" };
    /* 111 => InstructionType::SES */           const SES:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SES),         instruction_type: InstructionType::SES,         bit_pattern: "1001 0100 0100 1000" };
    /* 112 => InstructionType::SET */           const SET:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SET),         instruction_type: InstructionType::SET,         bit_pattern: "1001 0100 0110 1000" };
    /* 113 => InstructionType::SEV */           const SEV:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEV),         instruction_type: InstructionType::SEV,         bit_pattern: "1001 0100 0011 1000" };
    /* 114 => InstructionType::SEZ */           const SEZ:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SEZ),         instruction_type: InstructionType::SEZ,         bit_pattern: "1001 0100 0001 1000" };
    /* 115 => InstructionType::SLEEP */         const SLEEP:        InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SLEEP),       instruction_type: InstructionType::SLEEP,       bit_pattern: "1001 0101 1000 1000" };
    /* 116 => InstructionType::SPM */           const SPM:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SPM),         instruction_type: InstructionType::SPM,         bit_pattern: "1001 0101 1110 1000" };
    // /* 117 => InstructionType::SPM_2 */         const SPM_2:        InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SPM_2),       instruction_type: InstructionType::SPM_2,       bit_pattern: "" };
    // /* 118 => InstructionType::ST */            const ST:           InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST),          instruction_type: InstructionType::ST,          bit_pattern: "" };
    // /*  119 => InstructionType::ST_STD_Y */     const ST_STD_Y:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Y),    instruction_type: InstructionType::ST_STD_Y,    bit_pattern: "" };
    const ST_STD_Y_1:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Y_1),    instruction_type: InstructionType::ST_STD_Y_1,    bit_pattern: "1000 001r rrrr 1000" };
    const ST_STD_Y_2:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Y_2),    instruction_type: InstructionType::ST_STD_Y_2,    bit_pattern: "1001 001r rrrr 1001" };
    const ST_STD_Y_3:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Y_3),    instruction_type: InstructionType::ST_STD_Y_3,    bit_pattern: "1001 001r rrrr 1010" };
    const ST_STD_Y_4:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Y_4),    instruction_type: InstructionType::ST_STD_Y_4,    bit_pattern: "10q0 qq1r rrrr 1qqq" };
    // /*  120 => InstructionType::ST_STD_Z */     const ST_STD_Z:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Z),    instruction_type: InstructionType::ST_STD_Z,    bit_pattern: "" };
    const ST_STD_Z_1:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Z_1),    instruction_type: InstructionType::ST_STD_Z_1,    bit_pattern: "1000 001r rrrr 0000" };
    const ST_STD_Z_2:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Z_2),    instruction_type: InstructionType::ST_STD_Z_2,    bit_pattern: "1001 001r rrrr 0001" };
    const ST_STD_Z_3:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Z_3),    instruction_type: InstructionType::ST_STD_Z_3,    bit_pattern: "1001 001r rrrr 0010" };
    const ST_STD_Z_4:     InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::ST_STD_Z_4),    instruction_type: InstructionType::ST_STD_Z_4,    bit_pattern: "10q0 qq1r rrrr 0qqq" };
    /*  121 => InstructionType::STS */          const STS:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::STS),         instruction_type: InstructionType::STS,         bit_pattern: "1001 001d dddd 0000" };
    /*  122 => InstructionType::STS_16bit */    const STS_16bit:    InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::STS_16bit),   instruction_type: InstructionType::STS_16bit,   bit_pattern: "1010 1kkk dddd kkkk" };
    /*  123 => InstructionType::SUB */          const SUB:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SUB),         instruction_type: InstructionType::SUB,         bit_pattern: "0001 10rd dddd rrrr" };
    /*  124 => InstructionType::SUBI */         const SUBI:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SUBI),        instruction_type: InstructionType::SUBI,        bit_pattern: "0101 KKKK dddd KKKK" };
    /*  125 => InstructionType::SWAP */         const SWAP:         InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::SWAP),        instruction_type: InstructionType::SWAP,        bit_pattern: "1001 010d dddd 0010" };
    
    /*  126 => InstructionType::TST */          const TST:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::TST),         instruction_type: InstructionType::TST,         bit_pattern: "0010 00dd dddd dddd" };
    
    /*  127 => InstructionType::WDR */          const WDR:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::WDR),         instruction_type: InstructionType::WDR,         bit_pattern: "1001 0101 1010 1000" };
    
    /*  128 => InstructionType::XCH */          const XCH:          InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::XCH),         instruction_type: InstructionType::XCH,         bit_pattern: "1001 001r rrrr 0100" };
    
    pub                                         const unknown:      InstructionDefinition = InstructionDefinition { index: InstructionType::to_code(&InstructionType::Unknown),     instruction_type: InstructionType::Unknown, bit_pattern: "xxxxxxxxxxxxxxxx" };

    
    pub const instructions: &'static [InstructionDefinition] = &[

        /* 5 */ ADC, 
        /* 6 */ ADD, 
        /* 7 */ ADIW, 
        /* 8 */ AND,
        /* 9 */ ANDI,
        /* 10 */ ASR,

        /* 41 */ CLI, // 1001 0100 1111 1000 cli conflicts with 1001 0100 1sss 1000 BCLR. Moved up to give CLI preference
        /* 11 */ BCLR,
        /* 12 */ BLD,
        /* 13 */ BRBC,
        /* 25 */ BRLT, // moved upwords to give 1111 00kk kkkk k100 BRLT preference before 1111 00kk kkkk ksss BRBS
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
//        /* 25 */ BRLT, // moved upwords to give 1111 00kk kkkk k100 BRLT preference before 1111 00kk kkkk ksss BRBS
        BRMI,
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
        // CBR, // TODO
        CLC,
        CLH,
//        /* 41 */ CLI, // 1001 0100 1111 1000 cli conflicts with 1001 0100 1sss 1000 BCLR. Moved up to give CLI preference
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
        // ELPM, // TODO
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
        // LD, // TODO
        // LD_LDD_Y, // TODO
        // LD_LDD_Z, // TODO
        LDI,
        LDS,
        LDS_16bit,
        // LPM, // TODO
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
        // SPM_2, // TODO
        // ST, // TODO
        // ST_STD_Y, // split up into subcommands
        ST_STD_Y_1,
        ST_STD_Y_2,
        ST_STD_Y_3,
        ST_STD_Y_4,
        // ST_STD_Z, // split up into subcommands
        ST_STD_Z_1,
        ST_STD_Z_2,
        ST_STD_Z_3,
        ST_STD_Z_4,
        STS,
        STS_16bit,
        SUB,
        SUBI,
        SWAP,

        TST,

        WDR,

        XCH
    ];
use std::fmt;

#[derive(PartialEq)]
#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub enum AsmRecordType {

    BYTE,

    // assembler directive CSEG
    CSEG,

    DB,

    DSEG,

    DEF,

    DEVICE,

    EQU,

    // for instructions
    INSTRUCTION,

    // for new addresses to encode to (set via the .org directive)
    ORG,

    DIRECTDATA,

    #[default]
    UNKNOWN,

}

impl AsmRecordType {

    pub fn to_string(&self) -> String { 

        match self {
            AsmRecordType::BYTE => String::from("BYTE"),
            AsmRecordType::CSEG => String::from("CSEG"),
            AsmRecordType::DB => String::from("DB"),
            AsmRecordType::DSEG => String::from("DSEG"),
            AsmRecordType::DEF => String::from("DEF"),
            AsmRecordType::DEVICE => String::from("DEVICE"),
            AsmRecordType::EQU => String::from("EQU"),
            AsmRecordType::INSTRUCTION => String::from("INSTRUCTION"),
            AsmRecordType::ORG => String::from("ORG"),
            AsmRecordType::DIRECTDATA => String::from("DIRECTDATA"),
            _ => String::from("UNKNOWN"),
        }

    }

}

// impl fmt::Display for AsmRecordType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "UNKNOWN")
//     }
// }
use std::fmt;

#[derive(PartialEq)]
#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub enum AsmRecordType {

    // for instructions
    INSTRUCTION,

    // for new addresses to encode to (set via the .org directive)
    ORG,

    DIRECTDATA,

    #[default]
    UNKNOWN,

}

// impl fmt::Display for AsmRecordType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "UNKNOWN")
//     }
// }
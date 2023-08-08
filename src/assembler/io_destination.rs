#[derive(PartialEq)]
#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub enum IoDestination {
    
    // stack pointer
    SPL,
    SPH,

    // port B
    PORTB,
    DDRB,
    PINB,

    // port C
    PORTC,
    DDRC,
    PINC,

    #[default]
    UNKNOWN,
}

impl IoDestination {

    #[allow(dead_code)]
    pub const fn to_code(io_destination: &IoDestination) -> u16 {

        match io_destination {

            IoDestination::SPL => 0x01u16,
            IoDestination::SPH => 0x02u16,

            IoDestination::PORTB => 0x25u16,
            IoDestination::DDRB => 0x24u16,
            IoDestination::PINB => 0x23u16,

            IoDestination::PORTC => 0x28u16, 
            IoDestination::DDRC => 0x27u16,
            IoDestination::PINC => 0x26u16,

            _ => 0xFF,
        }
    }

    pub const fn from_code(code: u16) -> IoDestination {

        match code {

            0x01u16 => IoDestination::SPL,
            0x02u16 => IoDestination::SPH,

            0x25u16 => IoDestination::PORTB,
            0x24u16 => IoDestination::DDRB,
            0x23u16 => IoDestination::PINB,

            0x28u16 => IoDestination::PORTC,
            0x27u16 => IoDestination::DDRC,
            0x26u16 => IoDestination::PINC,

            _ => IoDestination::UNKNOWN,
        }
    }

    pub fn from_string(code: &str) -> IoDestination {

        let uc = code.to_uppercase();

        let upper_case: &str = uc.as_str();

        match upper_case {

            "SPL" => IoDestination::SPL,
            "SPH" => IoDestination::SPH,

            "DDRB" => IoDestination::DDRB,
            "PORTB" => IoDestination::PORTB,
            "PINB" => IoDestination::PINB,

            "DDRC" => IoDestination::DDRC,
            "PORTC" => IoDestination::PORTC,
            "PINC" => IoDestination::PINC,

            _ => IoDestination::UNKNOWN,
        }
    }
}

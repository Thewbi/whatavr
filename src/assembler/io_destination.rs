#[derive(Debug, Copy, Clone)]
pub enum IoDestination {
    SPL,
    SPH,

    UNKNOWN,
}

impl IoDestination {

    pub const fn to_code(io_destination: &IoDestination) -> u16 {
        match io_destination {
            IoDestination::SPL => 0x01u16,
            IoDestination::SPH => 0x02u16,
            _ => 0xFF,
        }
    }

    pub const fn from_code(code: u16) -> IoDestination {
        match code {
            0x01u16 => IoDestination::SPL,
            0x02u16 => IoDestination::SPH,
            _ => IoDestination::UNKNOWN,
        }
    }

}
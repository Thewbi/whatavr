pub const RAMEND: u16 = 0x08ff;

pub struct CPU {
    
    // the z flag
    pub z: bool,

    // stack pointer high, low
    pub sph: u8,
    pub spl: u8,

    // pc always points to the instruction after the current instruction so it does not start at 0x00 but at 0x02
    pub pc: i32,

    // register file, 32 8bit registers
    pub register_file: [u8; 32usize],

    pub sram: [u8; RAMEND as usize],

}
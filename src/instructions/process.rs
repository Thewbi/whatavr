use byteorder::{LittleEndian, ReadBytesExt};

use crate::HashMap;

pub fn process_adc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_add<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_adiw<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("ADIW  = 7. ADIW – Add Immediate to Word");
    //log::info!("{word:#b}");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "10010110kkddkkkk", &mut var_storage);

    let k_val = value_storage[&'K'];
    let d_val = value_storage[&'d'];

    //log::info!("k: {}, d: {}", k_val, d_val);
}

pub fn process_and<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_andi<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
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

pub fn process_asr<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_bclr<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_bld<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brbc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brbs<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brcc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brcs<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_break<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_breq<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brge<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brhc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brhs<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brid<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brie<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brlo<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brlt<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("BRLT   = 25. BRLT – Branch if Less Than (Signed)");

    // 1111 00kk kkkk k100

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  111100kkkkkkk100");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "111100kkkkkkk100", &mut var_storage);

    let k_val = value_storage[&'k'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
}

pub fn process_brne<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
     //log::info!("BRNE = 27. BRNE – Branch if Not Equal");

    // 1111 01kk kkkk k001

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  111101kkkkkkk001");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "111101kkkkkkk001", &mut var_storage);

    let k_val = value_storage[&'k'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
}

pub fn process_brmi<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brpl<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brsh<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brtc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brts<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brvc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_brvs<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_bset<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_bst<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_call<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("CALL = 36. CALL – Long Call to a Subroutine");

    // 1001 010k kkkk 111k
    // kkkk kkkk kkkk kkkk

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1001010kkkkk111k");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1001010kkkkk111k", &mut var_storage);

    let k_hi:u32 = value_storage[&'k'].into();
    //log::info!("k: {k_hi:#b} {k_hi:#x} {k_hi}");

    // read the next two byte of the 32 bit instruction 
    let k_lo:u32 = (rdr.read_u16::<LittleEndian>().unwrap()).into();
    *index += 2usize;
    log::trace!("k_lo: {:b}", k_lo);
    
    let k:u32 = (k_hi << 16u8) + k_lo;

    log::trace!("k: {:#06x}", k);

    // since the amount of elements to jump are words, to find the address, multiply by two
    //log::info!("k: {:#06x}", k*2);

    let addr_value = k*2;

    log::info!("{:#02x}: {word:#06x} {k_lo:#06x} call {addr_value:#02x}", *index-4usize);
}

pub fn process_cbi<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_cbr<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_clc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_clh<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_cli<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
     //log::info!("CLI  = 41. CLI – Clear Global Interrupt Flag");

            // 1001 0100 1111 1000
}

pub fn process_cln<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_clr<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    // CLR and EOR are the same thing!
    // CLR is implemented by performing an EOR of the register with itself.
    //
    // Clears a register. This instruction performs an Exclusive OR between a register and itself. This will clear
    // all bits in the register.
}

pub fn process_cls<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_clt<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_clv<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_clz<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_com<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_cp<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_cpc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
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

pub fn process_cpi<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
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

pub fn process_cpse<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_dec<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_des<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_eicall<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_eijmp<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_elpm<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_eor<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("EOR  = 58. EOR – Exclusive OR");
            
    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  001001rdddddrrrr");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "001001rdddddrrrr", &mut var_storage);

    let r_val = value_storage[&'r'];
    //log::info!("r: {r_val:#b} {r_val:#x}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x}");

    log::info!("{:#02x}: {word:#06x} eor r{r_val}, r{d_val}", *index-2usize);
}

pub fn process_fmul<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_fmuls<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_fmulsu<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_icall<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_ijmp<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_in<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("IN   = 64. IN - Load an I/O Location to Register");

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  10110AAdddddAAAA");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "10110AAdddddAAAA", &mut var_storage);

    let a_val = value_storage[&'A'];
    //log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

    log::info!("{:#02x}: {word:#06x} in r{d_val:#02} {a_val:#02x}", *index-2usize);
}

pub fn process_inc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_jmp<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    log::trace!("JMP  = 66. JMP – Jump");
    log::trace!("{word:#b}");

    let k_hi:u32 = value_storage[&'k'].into();
    log::trace!("k_hi: {}", k_hi);

    // read the next two byte of the 32 bit instruction 
    let k_lo:u32 = (rdr.read_u16::<LittleEndian>().unwrap()).into();
    //index = index + (usize)2;
    //index += 2;
    *index += 2usize;
    log::trace!("k_lo: {:b}", k_lo);
    
    let k:u32 = (k_hi << 16u8) + k_lo;

    log::trace!("k: {:#06x}", k);

    // since the amount of elements to jump are words, to find the address, multiply by two
    log::trace!("k: {:#06x}", k*2);

    log::info!("{:#02x}: {word:#06x} {k_lo:#06x} jmp {:#06x}", *index-4usize, k*2);

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

pub fn process_lac<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_las<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_lat<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_ld<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_ld_ldd_y<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_ld_ldd_z<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_ldi<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
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

    log::info!("{:#02x}: {word:#06x} ldi r{register:#02}, {k_val:#02x}", *index-2usize);
}

pub fn process_lds<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_lds_16bit<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_lpm<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_lsl<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_lsr<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_mov<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_movw<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_mul<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_muls<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_mulsu<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_neg<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_nop<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("NOP  = 85. NOP – No Operation");

    // 0000 0000 0000 0000
}

pub fn process_or<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_ori<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
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

pub fn process_out<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("OUT  = 88. OUT – Store Register to I/O Location");
            
    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  10111AArrrrrAAAA");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "10111AArrrrrAAAA", &mut var_storage);

    let a_val = value_storage[&'A'];
    //log::info!("A: {a_val:#b} {a_val:#x} {a_val}");
    let r_val = value_storage[&'r'];
    //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");

    log::info!("{:#02x}: {word:#06x} out {a_val:#02x}, r{r_val}", *index-2usize);
}

pub fn process_pop<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("POP   = 89. POP – Pop Register from Stack");

    // 1001 000d dddd 1111

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1001000ddddd1111");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1001000ddddd1111", &mut var_storage);

    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");
}

pub fn process_push<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("PUSH  = 90. PUSH – Push Register on Stack");
            
    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1001001ddddd1111");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1001001ddddd1111", &mut var_storage);

    let d_val = value_storage[&'d'];
    //log::info!("d: {d_val:#b} {d_val:#x} {d_val}");

    let register = d_val;

    log::info!("{:#02x}: {word:#06x} push r{register:#02}", *index-2usize);
}

pub fn process_rcall<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("RCALL  = 91. RCALL – Relative Call to Subroutine");
            
    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1101kkkkkkkkkkkk");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1101kkkkkkkkkkkk", &mut var_storage);

    let k_val = value_storage[&'k'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");

    log::info!("{:#02x}: {word:#06x} rcall .+{k_val:#02}", *index-22usize);
}

pub fn process_ret<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
     //log::info!("RET   = 92. RET – Return from Subroutine");

            // 1001 0101 0000 1000
}

pub fn process_reti<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_rjmp<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    //log::info!("RJMP = 94. RJMP – Relative Jump");

    // 1100 kkkk kkkk kkkk

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1100kkkkkkkkkkkk");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1100kkkkkkkkkkkk", &mut var_storage);

    let k_val = value_storage[&'k'];
    //log::info!("k: {k_val:#b} {k_val:#x} {k_val}");
}

pub fn process_rol<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_ror<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sbc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sbci<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
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

pub fn process_sbi<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sbic<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sbis<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sbiw<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sbr<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sbrc<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sbrs<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sec<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_seh<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sei<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sen<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_ser<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_ses<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_set<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sev<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sez<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sleep<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_spm<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_spm2<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_st<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_st_std_y_1<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
    log::info!("STD Y (A) = 119. ST (STD) – Store Indirect From Register to Data Space using Index Y");

    // 1000 001r rrrr 1000

    //log::info!("{word:#018b} {word:#02x}");
    //log::info!("  1000001rrrrr1000");

    // let mut var_storage:HashMap<char, u16> = HashMap::new();
    // bit_pattern_match(word, "1000001rrrrr1000", &mut var_storage);

    let r_val = value_storage[&'r'];
    //log::info!("r: {r_val:#b} {r_val:#x} {r_val}");
}

pub fn process_st_std_y_2<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_st_std_y_3<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_st_std_y_4<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_st_std_z_1<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_st_std_z_2<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_st_std_z_3<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_st_std_z_4<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_st_std_z<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sts<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sts_16bit<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_sub<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_subi<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
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

pub fn process_swap<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_tst<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_wdr<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

pub fn process_xch<R: crate::io::Read>(rdr: &mut R, word: &u16, index: &mut usize, value_storage: &mut HashMap<char, u16>)
{
}

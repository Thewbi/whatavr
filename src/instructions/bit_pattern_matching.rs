use std::collections::HashMap;

pub fn bit_match(data:u16, pattern:&str) -> bool {

    let mut bit_mask:u16 = 0x8000u16;
    let mut shift_counter:u8 = 15u8;

    for c in pattern.chars() {

        if c == ' ' {
            continue;
        }

        // x means dont care, the function accepts any value
        if c != '1' && c != '0' {

            // next character
            if shift_counter > 0 {
                shift_counter = shift_counter - 1;
            }
            bit_mask = bit_mask >> 1;

            continue;
        }

        let temp = (data & bit_mask) >> shift_counter;

        // next character
        if shift_counter > 0 {
            shift_counter = shift_counter - 1;
        }
        bit_mask = bit_mask >> 1;

        if c == '1' && temp == 0 {
            return false;
        }
        if c == '0' && temp == 1 {
            return false;
        }
    }

    return true;
}

pub fn bit_pattern_match(data:u16, pattern:&str, value_storage:&mut HashMap<char, u16>) {

    let mut bit_mask:u16 = 0x8000u16; // 1000 0000 0000 0000
    let mut shift_counter:u8 = 15u8;

    for c in pattern.chars() {

        if c == ' ' {
            continue;
        }

        // only look at placeholder values
        if c == '0' || c == '1' {

            // next character
            if shift_counter > 0 {
                shift_counter = shift_counter - 1;
            }
            bit_mask = bit_mask >> 1;

            continue;
        }

        // retrieve the bit value
        let temp = (data & bit_mask) >> shift_counter;

        // next character
        if shift_counter > 0 {
            shift_counter = shift_counter - 1;
        }
        bit_mask = bit_mask >> 1;

        // check if c is part of the hashmap and insert it if not
        if !value_storage.contains_key(&c) {
            value_storage.insert(c, temp);
        } else {
            *value_storage.get_mut(&c).unwrap() = (value_storage.get(&c).unwrap() << 1) + temp;
        }
    }
}

use regex::Regex;

pub fn is_number_literal_u16(data: &String) -> bool {

    if data.to_lowercase().starts_with("0b")
    {
        // parse binary
        let without_prefix = data.trim_start_matches("0b");
        return u16::from_str_radix(without_prefix, 2).is_ok();
    } 
    else if data.to_lowercase().starts_with("0x")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("0x");
        return u16::from_str_radix(without_prefix, 16).is_ok();
    } 
    else if data.starts_with("$")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("$");
        return u16::from_str_radix(without_prefix, 16).is_ok();
    }
    else if data.starts_with("0") && 1 != data.len()
    {
        // parse octal
        let without_prefix = data.trim_start_matches("0");
        return u16::from_str_radix(without_prefix, 8).is_ok();
    }

    let parse_result = data.parse::<u16>();
    parse_result.is_ok()

}

pub fn is_number_literal_i16(data: &String) -> bool {

    if data.to_lowercase().starts_with("0b")
    {
        // parse binary
        let without_prefix = data.trim_start_matches("0b");
        return u16::from_str_radix(without_prefix, 2).is_ok();
    } 
    else if data.to_lowercase().starts_with("0x")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("0x");
        return u16::from_str_radix(without_prefix, 16).is_ok();
    } 
    else if data.starts_with("$")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("$");
        return u16::from_str_radix(without_prefix, 16).is_ok();
    }
    else if data.starts_with("0") && 1 != data.len()
    {
        // parse octal
        let without_prefix = data.trim_start_matches("0");
        return u16::from_str_radix(without_prefix, 8).is_ok();
    }

    let parse_result = data.parse::<i16>();
    parse_result.is_ok()

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_number_literal_u16_test_zero() {
        assert!(is_number_literal_u16(&"0".to_string()));
    }

    #[test]
    fn is_number_literal_u16_test_one() {
        assert!(is_number_literal_u16(&"1".to_string()));
    }

    #[test]
    fn is_number_literal_u16_test_minus_eight() {
        // -8 is not a u8
        assert!(!is_number_literal_u16(&"-8".to_string()));
    }

    #[test]
    fn is_number_literal_u16_test_hex() {
        assert!(is_number_literal_u16(&"0x01".to_string()));
        assert!(is_number_literal_u16(&"$01".to_string()));
    }

    #[test]
    fn is_number_literal_u16_test_octal() {
        assert!(is_number_literal_u16(&"0123".to_string()));
    }

    #[test]
    fn is_number_literal_u16_test_binary() {
        assert!(is_number_literal_u16(&"0b00010001".to_string()));
    }

    #[test]
    fn is_number_literal_u16_test_decimal() {
        assert!(is_number_literal_u16(&"12".to_string()));
    }

}

pub fn number_literal_to_u8(data: &String) -> u8 {

    if data.to_lowercase().starts_with("0b")
    {
        // parse binary
        let without_prefix = data.trim_start_matches("0b");
        return u8::from_str_radix(without_prefix, 2).unwrap();
    } 
    else if data.to_lowercase().starts_with("0x")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("0x");
        return u8::from_str_radix(without_prefix, 16).unwrap();
    } 
    else if data.starts_with("$")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("$");
        return u8::from_str_radix(without_prefix, 16).unwrap();
    }
    else if data.starts_with("0") && 1 != data.len()
    {
        // parse octal
        let without_prefix = data.trim_start_matches("0");
        return u8::from_str_radix(without_prefix, 8).unwrap();
    }

    return data.parse::<u8>().unwrap();

}

pub fn number_literal_to_u16(data: &String) -> u16 {

    if data.to_lowercase().starts_with("0b")
    {
        // parse binary
        let without_prefix = data.trim_start_matches("0b");
        return u16::from_str_radix(without_prefix, 2).unwrap();
    } 
    else if data.to_lowercase().starts_with("0x")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("0x");
        return u16::from_str_radix(without_prefix, 16).unwrap();
    } 
    else if data.starts_with("$")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("$");
        return u16::from_str_radix(without_prefix, 16).unwrap();
    }
    else if data.starts_with("0") && 1 != data.len()
    {
        // parse octal
        let without_prefix = data.trim_start_matches("0");
        return u16::from_str_radix(without_prefix, 8).unwrap();
    }

    return data.parse::<u16>().unwrap();

}

pub fn number_literal_to_u32(data: &String) -> u32 {

    if data.to_lowercase().starts_with("0b")
    {
        // parse binary
        let without_prefix = data.trim_start_matches("0b");
        return u32::from_str_radix(without_prefix, 2).unwrap();
    } 
    else if data.to_lowercase().starts_with("0x")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("0x");
        return u32::from_str_radix(without_prefix, 16).unwrap();
    } 
    else if data.starts_with("$")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("$");
        return u32::from_str_radix(without_prefix, 16).unwrap();
    }
    else if data.starts_with("0") && 1 != data.len()
    {
        // parse octal
        let without_prefix = data.trim_start_matches("0");
        return u32::from_str_radix(without_prefix, 8).unwrap();
    }

    return data.parse::<u32>().unwrap();

}

pub fn number_literal_to_i16(data: &String) -> i16 {

    if data.to_lowercase().starts_with("0b")
    {
        // parse binary
        let without_prefix = data.trim_start_matches("0b");
        return i16::from_str_radix(without_prefix, 2).unwrap();
    } 
    else if data.to_lowercase().starts_with("0x")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("0x");
        return i16::from_str_radix(without_prefix, 16).unwrap();
    } 
    else if data.starts_with("$")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("$");
        return i16::from_str_radix(without_prefix, 16).unwrap();
    }
    else if data.starts_with("0") && 1 != data.len()
    {
        // parse octal
        let without_prefix = data.trim_start_matches("0");
        return i16::from_str_radix(without_prefix, 8).unwrap();
    }

    return data.parse::<i16>().unwrap();

}

pub fn number_literal_to_i32(data: &String) -> i32 {

    if data.to_lowercase().starts_with("0b")
    {
        // parse binary
        let without_prefix = data.trim_start_matches("0b");
        return i32::from_str_radix(without_prefix, 2).unwrap();
    } 
    else if data.to_lowercase().starts_with("0x")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("0x");
        return i32::from_str_radix(without_prefix, 16).unwrap();
    } 
    else if data.starts_with("$")
    {
        // parse hex
        let without_prefix = data.trim_start_matches("$");
        return i32::from_str_radix(without_prefix, 16).unwrap();
    }
    else if data.starts_with("0") && 1 != data.len()
    {
        // parse octal
        let without_prefix = data.trim_start_matches("0");
        return i32::from_str_radix(without_prefix, 8).unwrap();
    }

    return data.parse::<i32>().unwrap();

}

pub fn is_char_literal(input: &String) -> bool
{
    let trimmed_input = input.trim();

    let re = Regex::new("^'.|\\.'$").unwrap();

    re.is_match(trimmed_input)
}

pub fn char_literal_to_u16(input: &String) -> u16
{
    let trimmed_input = input.trim();
    let first_character: char = trimmed_input.chars().nth(1).unwrap();

    if first_character == '\\'
    {
        let second_character: char = trimmed_input.chars().nth(2).unwrap();
        match second_character {
            'n' => { return 0x0Au16; },
            _ => panic!("Not implemented!"),
        }
    }

    first_character as u16
}

#[cfg(test)]
mod char_literal_tests {

    use super::*;

    #[test]
    fn is_char_literal_zero_test() {
        assert!(!is_char_literal(&"0".to_string()));
    }

    #[test]
    fn is_char_literal_space_test() {
        assert!(is_char_literal(&"' '".to_string()));
    }

    #[test]
    fn is_char_literal_newline_test() {
        assert!(is_char_literal(&"'\\n'".to_string()));
    }

    #[test]
    fn char_literal_to_u16_space_test() {
        assert_eq!(0x20, char_literal_to_u16(&"' '".to_string()));
    }

    #[test]
    fn char_literal_to_u16_newline_test() {
        assert_eq!(0x0A, char_literal_to_u16(&"'\\n'".to_string()));
    }
}
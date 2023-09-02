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
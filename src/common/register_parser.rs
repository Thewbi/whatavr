use regex::Regex;

// https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
pub fn is_register_name(input: &str) -> bool 
{
    let input_uc = input.to_uppercase();
    if input_uc.eq("XH") || input_uc.eq("XL") ||
        input_uc.eq("YH") || input_uc.eq("YL") ||
        input_uc.eq("ZH") || input_uc.eq("ZL")
    {
        return true;
    }
    let re = Regex::new("^(r|R)(\\d|[12]\\d|3[01])$").unwrap();
    re.is_match(input)
}

pub fn register_name_to_u16(input: &str) -> u16 
{
    let input_uc = input.to_uppercase();
    match input_uc.as_str() {
        "XL" => {
            return 26u16; //  0x1A
        }
        "XH" => {
            return 27u16; //  0x1B
        }

        "YL" => {
            return 28u16; //  0x1C
        }
        "YH" => {
            return 29u16; //  0x1D
        }

        "ZL" => {
            return 30u16; //  0x1E
        }
        "ZH" => {
            return 31u16; //  0x1F
        }

        _ => {
            input[1..].parse::<u16>().unwrap()
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn process_x_y_z_low_high() {
        assert!(is_register_name("xh"));
        assert!(is_register_name("xl"));
        assert!(is_register_name("yh"));
        assert!(is_register_name("yl"));
        assert!(is_register_name("zh"));
        assert!(is_register_name("zl"));
    }

    #[test]
    fn register_name_to_u16_test() {
        assert_eq!(26, register_name_to_u16("xl"));
        assert_eq!(27, register_name_to_u16("xh"));

        assert_eq!(28, register_name_to_u16("yl"));
        assert_eq!(29, register_name_to_u16("yh"));

        assert_eq!(30, register_name_to_u16("zl"));
        assert_eq!(31, register_name_to_u16("zh"));
    }

}

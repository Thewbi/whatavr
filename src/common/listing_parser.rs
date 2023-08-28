use regex::Regex;

pub fn is_code_offset_c_listing(input: &str) -> bool {
    let trimmed_input = input.trim();
    let re = Regex::new("^(\\d|[aAbBcCdDeEfF])+\\:").unwrap();
    re.is_match(trimmed_input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_code_offset_not_a_line_test() {
        assert!(!is_code_offset_c_listing(&"1".to_string()));
    }

    #[test]
    fn is_code_offset_is_a_line_test() {
        assert!(is_code_offset_c_listing(&"ca:	ff cf       	rjmp	.-2      	; 0xca <main+0x6>".to_string()));
    }

    #[test]
    fn is_code_offset_is_a_line_three_digits_test() {
        assert!(is_code_offset_c_listing(&"100:	a0 e0       	ldi	r26, 0x00	; 0".to_string()));
    }

    #[test]
    fn is_code_offset_is_a_line_three_digits_not_trimmed_test() {
        assert!(is_code_offset_c_listing(&"    1ba:	3e 2e       	mov	r3, r30".to_string()));
    }

    #[test]
    fn is_code_offset_empty_line() {
        assert!(!is_code_offset_c_listing(&"".to_string()));
    }

}
use regex::Regex;

// https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
pub fn is_register_name(input: &str) -> bool {
    let re = Regex::new("^(r|R)(\\d|[12]\\d|3[01])$").unwrap();
    re.is_match(input)
}

pub fn register_name_to_u16(input: &str) -> u16 {
    input[1..].parse::<u16>().unwrap()
}
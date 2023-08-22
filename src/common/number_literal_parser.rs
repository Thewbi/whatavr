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
    else if data.starts_with("0")
    {
        // parse octal
        let without_prefix = data.trim_start_matches("0");
        return u16::from_str_radix(without_prefix, 8).unwrap();
    }
    // else 
    // {
    //     // parse decimal
    //     let parse_result = data.parse::<u16>();
    //     if parse_result.is_ok() {
    //         self.record.data = parse_result.unwrap();
    //     } else  {
    //         self.target_label = data.clone();
    //     }
    // }

    return data.parse::<u16>().unwrap();

}
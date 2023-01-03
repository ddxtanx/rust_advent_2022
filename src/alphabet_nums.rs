pub fn alphabet_char_to_int(c: char) -> Option<u32> {
    let cb = c as u8;
    if c.is_ascii_uppercase() {
        let ab: u8 = 'A' as u8;
        let diff = (cb - ab) as u32;
        Some(diff + 26)
    } else if c.is_ascii_lowercase() {
        let ab: u8 = 'a' as u8;
        let diff = (cb - ab) as u32;
        Some(diff)
    } else {
        None
    }
}

pub fn alphabet_char_to_priority(c: char) -> Option<u32> {
    alphabet_char_to_int(c).map(|i| i + 1)
}


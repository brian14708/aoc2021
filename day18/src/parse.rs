pub fn take_char(s: &str) -> Option<(&str, char)> {
    let c = s.chars().next()?;
    Some((&s[c.len_utf8()..], c))
}

pub fn take_number(s: &str) -> Option<(&str, u32)> {
    let mut offset = 0;
    let mut result = 0;
    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            offset += c.len_utf8();
            result = result * 10 + d;
        } else {
            break;
        }
    }
    if offset == 0 {
        None
    } else {
        Some((&s[offset..], result))
    }
}

pub fn consume(s: &str, c: char) -> Option<&str> {
    let (s, cc) = take_char(s)?;
    if cc == c {
        Some(s)
    } else {
        None
    }
}

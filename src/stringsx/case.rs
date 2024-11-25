pub fn to_lower_initial(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let mut chars = s.chars();
    let first_char = chars.next().unwrap().to_lowercase().to_string();
    let rest = chars.collect::<String>();
    format!("{}{}", first_char, rest)
}

pub fn to_upper_inital(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let mut chars = s.chars();
    let first_char = chars.next().unwrap().to_uppercase().to_string();
    let rest = chars.collect::<String>();
    format!("{}{}", first_char, rest)
}

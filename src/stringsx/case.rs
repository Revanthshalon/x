/// Converts the first character of a string to lowercase
///
/// # Arguments
/// * `s` - Input string
///
/// # Returns
/// * String with first character converted to lowercase
pub fn to_lower_initial(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let mut chars = s.chars();
    let first_char = chars.next().unwrap().to_lowercase().to_string();
    let rest = chars.collect::<String>();
    format!("{}{}", first_char, rest)
}

/// Converts the first character of a string to uppercase
///
/// # Arguments
/// * `s` - Input string
///
/// # Returns
/// * String with first character converted to uppercase
pub fn to_upper_inital(s: String) -> String {
    if s.is_empty() {
        return s;
    }
    let mut chars = s.chars();
    let first_char = chars.next().unwrap().to_uppercase().to_string();
    let rest = chars.collect::<String>();
    format!("{}{}", first_char, rest)
}

//! String case conversion utilities
//!
//! This module provides functions for converting the first character of strings between
//! uppercase and lowercase.
//!
//! # Examples
//!
//! ```
//! use crate::string_utils::{to_lower_initial, to_upper_inital};
//!
//! let upper = to_upper_inital(String::from("hello"));
//! assert_eq!(upper, "Hello");
//!
//! let lower = to_lower_initial(String::from("World"));
//! assert_eq!(lower, "world");
//! ```

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

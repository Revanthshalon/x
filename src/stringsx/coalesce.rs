//! String Utility Functions
//!
//! This module provides utility functions for working with strings and string slices.
//! Functions include:
//! - `coalesce`: Find first non-empty string in a slice
//!
//! # Example
//! ```
//! use string_utils::coalesce;
//! let words = ["", "first", "second"];
//! assert_eq!(coalesce(&words), "first");
//! ```

/// Returns the first non-empty string from the slice, or an empty string if none found
///
/// # Arguments
/// * `words` - A slice of string references to search through
///
/// # Returns
/// * First non-empty string found, or empty string if all empty
pub fn coalesce<'r>(words: &[&'r str]) -> &'r str {
    for word in words {
        if !word.is_empty() {
            return word;
        }
    }
    ""
}

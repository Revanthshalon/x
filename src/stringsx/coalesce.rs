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

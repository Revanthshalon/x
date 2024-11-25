pub fn coalesce<'r>(words: &[&'r str]) -> &'r str {
    for word in words {
        if !word.is_empty() {
            return word;
        }
    }
    ""
}

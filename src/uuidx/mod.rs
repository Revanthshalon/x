use uuid::Uuid;

/// Generates a new random UUID v4
///
/// Returns a new UUID using the v4 format (random)
/// # Example
/// ```
/// let id = generate_new_v4();
/// ```
pub fn generate_new_v4() -> Uuid {
    Uuid::new_v4()
}

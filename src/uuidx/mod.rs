//! # UUID Generator Module
//!
//! This module provides functionality for generating UUID v4 (random) identifiers.
//! It wraps the uuid crate's functionality to provide a simple interface for
//! generating new UUIDs.
//!
//! # Example
//!
//! ```rust
//! use my_crate::uuid_generator;
//!
//! // Generate a new random UUID
//! let new_id = uuid_generator::generate_new_v4();
//! println!("Generated UUID: {}", new_id);
//! ```
//!
//! The generated UUIDs are compliant with RFC 4122 version 4 format
//! and are suitable for use as unique identifiers in databases,
//! distributed systems, or any other use case requiring unique IDs.

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

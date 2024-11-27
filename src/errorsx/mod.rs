//! # EnrichedErrors
//!
//! This module provides an enhanced error handling solution for Rust applications that need detailed
//! error context and debugging information. The `EnrichedErrors` type combines error messages, stack
//! traces, source locations, and additional contextual information into a comprehensive error type.
//!
//! ## Overview
//!
//! The main components are:
//! - `EnrichedErrors`: The core error type that holds all error details
//! - `EnrichedErrorsBuilder`: A builder pattern implementation for constructing errors
//!
//! ## Usage Scenarios
//!
//! Ideal for:
//! - Applications requiring detailed error tracking and debugging
//! - Error handling where context and error chains are important
//! - Situations where error source location and stack traces aid debugging
//!
//! ### Example
//! ```rust
//! let err = EnrichedErrors::builder("Failed to process file")
//!     .with_context("Processing user upload")
//!     .with_source(io_error)
//!     .build();
//! ```

use std::{backtrace::Backtrace, error::Error, fmt::Display, panic::Location};

/// An enriched error type with additional context and debug information
///
/// # Fields
/// * `message` - The main error message
/// * `backtrace` - Stack backtrace when error occurred
/// * `location` - Source code location where error was created
/// * `context` - Vector of context strings providing additional error details
/// * `source` - Optional source error that caused this error
#[derive(Debug)]
pub struct EnrichedErrors {
    message: String,
    backtrace: Backtrace,
    location: &'static Location<'static>,
    context: Vec<String>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
    status_code: u32,
    status: String,
}

/// Display implementation for EnrichedErrors
///
/// Formats the error information including context and backtrace for display
impl Display for EnrichedErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let context_info = self.context.join(",");
        let location_info = format!(
            "(at: {}, line_no: {})",
            self.location.file(),
            self.location.line()
        );
        write!(
            f,
            "Location: {},\nContext: {}\nSource:\n {:#?}",
            location_info, context_info, self.backtrace
        )
    }
}

/// Error implementation for EnrichedErrors
///
/// Provides access to the underlying source error if one exists
impl Error for EnrichedErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn Error + 'static))
    }
}

/// Builder for constructing EnrichedErrors with a fluent interface
///
/// # Fields
/// * `message` - The main error message
/// * `context` - Vector of context strings
/// * `location` - Source code location
/// * `source` - Optional source error
#[derive(Debug)]
pub struct EnrichedErrorsBuilder {
    message: String,
    context: Vec<String>,
    location: &'static Location<'static>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl EnrichedErrorsBuilder {
    /// Creates a new EnrichedErrorsBuilder with the given message
    ///
    /// # Parameters
    /// * `message` - The error message to use, anything that can be converted into a String
    ///
    /// # Returns
    /// A new EnrichedErrorsBuilder instance initialized with the message
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            context: Vec::new(),
            location: Location::caller(),
            source: None,
        }
    }

    /// Adds context information to the error
    ///
    /// # Parameters
    /// * `context` - Additional context string to add, anything that can be converted into a String
    ///
    /// # Returns
    /// Self with the new context added for chaining
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }

    /// Sets the source error that caused this error
    ///
    /// # Parameters
    /// * `source` - The source error that implements Error + Send + Sync
    ///
    /// # Returns
    /// Self with the source error set for chaining
    pub fn with_source(mut self, source: impl Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Builds and returns the final EnrichedErrors instance
    ///
    /// # Returns
    /// An EnrichedErrors instance with all the configured properties
    pub fn build(self) -> EnrichedErrors {
        EnrichedErrors {
            message: self.message,
            context: self.context,
            location: self.location,
            backtrace: Backtrace::force_capture(),
            source: self.source,
        }
    }
}

impl EnrichedErrors {
    /// Creates a new EnrichedErrors with just a message
    ///
    /// # Parameters
    /// * `message` - The error message, anything that can be converted into a String
    ///
    /// # Returns
    /// A new EnrichedErrors instance
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        EnrichedErrorsBuilder::new(message).build()
    }

    /// Creates a new EnrichedErrorsBuilder to construct an error with more options
    ///
    /// # Parameters
    /// * `message` - The error message, anything that can be converted into a String
    ///
    /// # Returns
    /// An EnrichedErrorsBuilder instance for fluent construction
    #[track_caller]
    pub fn builder(message: impl Into<String>) -> EnrichedErrorsBuilder {
        EnrichedErrorsBuilder::new(message)
    }

    /// Gets the error message
    ///
    /// # Returns
    /// A string slice containing the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Gets the context information
    ///
    /// # Returns
    /// A vector of context strings
    pub fn context(&self) -> &Vec<String> {
        &self.context
    }

    /// Gets the source code location where the error was created
    ///
    /// # Returns
    /// The source Location
    pub fn location(&self) -> &Location<'static> {
        self.location
    }

    /// Gets the stack backtrace from when the error occurred
    ///
    /// # Returns
    /// The Backtrace
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

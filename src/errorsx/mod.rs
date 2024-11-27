//! # Errorsx
//!
//! This module provides an enhanced error handling solution for Rust applications that need detailed
//! error context and debugging information. The `Errorsx` type combines error messages, stack
//! traces, source locations, and additional contextual information into a comprehensive error type.
//!
//! ## Overview
//!
//! The main components are:
//! - `Errorsx`: The core error type that holds all error details
//! - `ErrorsxBuilder`: A builder pattern implementation for constructing errors
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
//! let err = Errorsx::builder("Failed to process file")
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
/// * `status_code` - Optional HTTP status code associated with the error
/// * `status` - Optional status message associated with the error
#[derive(Debug)]
pub struct Errorsx {
    message: String,
    backtrace: Backtrace,
    location: &'static Location<'static>,
    context: Vec<String>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
    status_code: Option<u32>,
    status: Option<String>,
}

/// Display implementation for Errorsx
///
/// Formats the error information including context and backtrace for display
impl Display for Errorsx {
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

/// Error implementation for Errorsx
///
/// Provides access to the underlying source error if one exists
impl Error for Errorsx {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn Error + 'static))
    }
}

/// Builder for constructing Errorsx with a fluent interface
///
/// # Fields
/// * `message` - The main error message
/// * `context` - Vector of context strings
/// * `location` - Source code location
/// * `source` - Optional source error
/// * `status_code` - Optional HTTP status code
/// * `status` - Optional status message
#[derive(Debug)]
pub struct ErrorsxBuilder {
    message: String,
    context: Vec<String>,
    location: &'static Location<'static>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
    status_code: Option<u32>,
    status: Option<String>,
}

impl ErrorsxBuilder {
    /// Creates a new ErrorsxBuilder with the given message
    ///
    /// # Parameters
    /// * `message` - The error message to use, anything that can be converted into a String
    ///
    /// # Returns
    /// A new ErrorsxBuilder instance initialized with the message
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            context: Vec::new(),
            location: Location::caller(),
            source: None,
            status_code: None,
            status: None,
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

    /// Sets the HTTP status code for this error
    ///
    /// # Parameters
    /// * `status_code` - The HTTP status code to associate with this error
    ///
    /// # Returns
    /// Self with the status code set for chaining
    pub fn with_status_code(mut self, status_code: u32) -> Self {
        self.status_code = Some(status_code);
        self
    }

    /// Sets a status message for this error
    ///
    /// # Parameters
    /// * `status` - The status message to associate with this error
    ///
    /// # Returns
    /// Self with the status message set for chaining
    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Builds and returns the final Errorsx instance
    ///
    /// # Returns
    /// An Errorsx instance with all the configured properties
    pub fn build(self) -> Errorsx {
        Errorsx {
            message: self.message,
            context: self.context,
            location: self.location,
            backtrace: Backtrace::force_capture(),
            source: self.source,
            status_code: self.status_code,
            status: self.status,
        }
    }
}

impl Errorsx {
    /// Creates a new Errorsx with just a message
    ///
    /// # Parameters
    /// * `message` - The error message, anything that can be converted into a String
    ///
    /// # Returns
    /// A new Errorsx instance
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        ErrorsxBuilder::new(message).build()
    }

    /// Creates a new ErrorsxBuilder to construct an error with more options
    ///
    /// # Parameters
    /// * `message` - The error message, anything that can be converted into a String
    ///
    /// # Returns
    /// An ErrorsxBuilder instance for fluent construction
    #[track_caller]
    pub fn builder(message: impl Into<String>) -> ErrorsxBuilder {
        ErrorsxBuilder::new(message)
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

    /// Gets the HTTP status code if one was set
    ///
    /// # Returns
    /// Optional HTTP status code associated with the error
    pub fn status_code(&self) -> &Option<u32> {
        &self.status_code
    }

    /// Gets the status message if one was set
    ///
    /// # Returns
    /// Optional status message string associated with the error
    pub fn status(&self) -> &Option<String> {
        &self.status
    }
}

//! This module provides a robust error handling system for applications that need to attach
//! rich contextual information to errors. It defines an `ErrorContext` type that wraps standard
//! errors and adds metadata like HTTP status codes, request IDs, debug information, and arbitrary
//! key-value details.
//!
//! The module includes several carrier traits that define interfaces for accessing different types
//! of error metadata. These traits can be implemented by other error types to provide a consistent
//! way to access error context information across an application.
//!
//! # Key Features
//! - Wraps any error type that implements `std::error::Error`
//! - Attaches HTTP status codes, request IDs, and debug messages
//! - Supports arbitrary key-value metadata through a details HashMap
//! - Builder pattern for easy construction
//! - Implements standard Error and Display traits
//!
//! # Example
//! ```
//! use error_context::ErrorContext;
//!
//! let error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
//! let context = ErrorContext::new(error)
//!     .with_status_code(404)
//!     .with_reason("The requested resource could not be found".to_string());
//! ```

// Standard library imports for core functionality
use std::{any::Any, collections::HashMap, error::Error, fmt::Display};

/// Trait for types that carry an HTTP status code
pub trait StatusCodeCarrier {
    fn status_code(&self) -> i32;
}

/// Trait for types that carry a request ID
pub trait RequestIdCarrier {
    fn request_id(&self) -> Option<&str>;
}

/// Trait for types that carry a reason message
pub trait ReasonCarrier {
    fn reason(&self) -> Option<&str>;
}

/// Trait for types that carry debug information
pub trait DebugCarrier {
    fn debug(&self) -> Option<&str>;
}

/// Trait for types that carry a status message
pub trait StatusCarrier {
    fn status(&self) -> Option<&str>;
}

/// Trait for types that carry additional details in a HashMap
pub trait DetailsCarrier {
    fn details(&self) -> Option<&HashMap<String, Box<dyn Any + Send + Sync>>>;
}

/// Trait for types that carry an ID
pub trait IdCarrier {
    fn id(&self) -> Option<&str>;
}

/// A context wrapper for errors that provides additional metadata
///
/// # Fields
/// * `source` - The underlying error that is being wrapped
/// * `status_code` - HTTP status code associated with the error
/// * `request_id` - ID of the request where the error occurred
/// * `reason` - Human-readable reason for the error
/// * `debug` - Debug information about the error
/// * `status` - Status message associated with the error
/// * `details` - Additional contextual details stored as key-value pairs
/// * `id` - Unique identifier for the error
#[derive(Debug)]
pub struct ErrorContext {
    /// The underlying error that is being wrapped
    source: Box<dyn Error + Send + Sync>,
    /// HTTP status code associated with the error
    status_code: Option<i32>,
    /// ID of the request where the error occurred
    request_id: Option<String>,
    /// Human-readable reason for the error
    reason: Option<String>,
    /// Debug information about the error
    debug: Option<String>,
    /// Status message associated with the error
    status: Option<String>,
    /// Additional contextual details stored as key-value pairs
    details: Option<HashMap<String, Box<dyn Any + Send + Sync>>>,
    /// Unique identifier for the error
    id: Option<String>,
}

impl ErrorContext {
    /// Creates a new ErrorContext wrapping the given error
    pub fn new<E>(error: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            source: Box::new(error),
            status_code: None,
            request_id: None,
            reason: None,
            debug: None,
            status: None,
            details: None,
            id: None,
        }
    }

    /// Retrieves the root cause of an error by following the error chain
    pub fn cause<'a>(error: &'a (dyn Error + 'static)) -> &'a (dyn Error + 'static) {
        let mut current_error = error;
        while let Some(source) = current_error.source() {
            current_error = source;
        }
        current_error
    }

    /// Sets the status code for this error context
    pub fn with_status_code(mut self, code: i32) -> Self {
        self.status_code = Some(code);
        self
    }

    /// Sets the request ID for this error context
    pub fn with_request_id(mut self, id: String) -> Self {
        self.request_id = Some(id);
        self
    }

    /// Sets the reason message for this error context
    pub fn with_reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }

    /// Sets the debug information for this error context
    pub fn with_debug(mut self, debug: String) -> Self {
        self.debug = Some(debug);
        self
    }

    /// Sets the status message for this error context
    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets additional details for this error context
    pub fn with_details(mut self, details: HashMap<String, Box<dyn Any + Send + Sync>>) -> Self {
        self.details = Some(details);
        self
    }

    /// Sets the ID for this error context
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }
}

/// Implements Display for ErrorContext by delegating to the underlying error
impl Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}

/// Implements Error for ErrorContext, providing access to the underlying error source
impl Error for ErrorContext {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.source)
    }
}

/// Implementation of StatusCodeCarrier for ErrorContext
impl StatusCodeCarrier for ErrorContext {
    fn status_code(&self) -> i32 {
        self.status_code.unwrap_or(500)
    }
}

/// Implementation of RequestIdCarrier for ErrorContext
impl RequestIdCarrier for ErrorContext {
    fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}

/// Implementation of ReasonCarrier for ErrorContext
impl ReasonCarrier for ErrorContext {
    fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}

/// Implementation of DebugCarrier for ErrorContext
impl DebugCarrier for ErrorContext {
    fn debug(&self) -> Option<&str> {
        self.debug.as_deref()
    }
}

/// Implementation of StatusCarrier for ErrorContext
impl StatusCarrier for ErrorContext {
    fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }
}

/// Implementation of DetailsCarrier for ErrorContext
impl DetailsCarrier for ErrorContext {
    fn details(&self) -> Option<&HashMap<String, Box<dyn Any + Send + Sync>>> {
        self.details.as_ref()
    }
}

/// Implementation of IdCarrier for ErrorContext
impl IdCarrier for ErrorContext {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
}

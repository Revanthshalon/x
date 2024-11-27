# X Utils Library
[![Format](https://github.com/Revanthshalon/x/actions/workflows/format.yml/badge.svg)](https://github.com/Revanthshalon/x/actions/workflows/format.yml) [![Lint](https://github.com/Revanthshalon/x/actions/workflows/lint.yml/badge.svg)](https://github.com/Revanthshalon/x/actions/workflows/lint.yml) [![Security Audit](https://github.com/Revanthshalon/x/actions/workflows/audit.yml/badge.svg)](https://github.com/Revanthshalon/x/actions/workflows/audit.yml)

A comprehensive collection of utility modules providing robust error handling, string manipulation, tracing, and UUID generation functionality for Rust applications.

## Features

### Error Handling (`errorsx`)
- Enhanced error type with rich context and debugging information
- Stack trace capture and source location tracking
- HTTP status code and status message support
- Context chain building
- Source error linking
- Builder pattern for flexible error construction
- Implements standard Error and Display traits

### String Utilities (`stringsx`)
- Case manipulation functions
  - Convert first character to lowercase (`to_lower_initial`)
  - Convert first character to uppercase (`to_upper_initial`)
- String coalescing
  - Find first non-empty string from a list (`coalesce`)

### UUID Generation (`uuidx`)
- Simple UUID v4 generation utilities
- Wraps the `uuid` crate functionality

### Tracing (`tracex`)
- *Currently in development*

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
x = { git = "https://github.com/revanthshalon/x" }
```

## Usage Examples

### Enriched Errors

```rust
use x::errorsx::EnrichedErrors;

// Simple error creation
let error = EnrichedErrors::new("Operation failed");

// Using the builder pattern
let detailed_error = EnrichedErrors::builder("Failed to process file")
    .with_context("Processing user upload")
    .with_status_code(500)
    .with_status("Internal Server Error")
    .build();
```

### String Manipulation

```rust
use x::stringsx::case;
use x::stringsx::coalesce;

// Case manipulation
let lower = case::to_lower_initial("Hello".to_string()); // "hello"
let upper = case::to_upper_initial("hello".to_string()); // "Hello"

// String coalescing
let result = coalesce::coalesce(&["", "second", "third"]); // returns "second"
```

### UUID Generation

```rust
use x::uuidx;

let new_uuid = uuidx::generate_new_v4();
```

## Module Structure

```
x/
├── errorsx/     # Enhanced error handling with rich context
├── stringsx/    # String manipulation utilities
│   ├── case.rs    # Case conversion functions
│   └── coalesce.rs # String coalescing utilities
├── tracex/      # Tracing functionality (WIP)
└── uuidx/       # UUID generation utilities
```

## Features of EnrichedErrors

- Detailed error messages
- Stack trace capture
- Source code location tracking
- Context chain building
- Source error linking
- HTTP status codes
- Status messages
- Builder pattern for easy construction

## Requirements

- Rust 1.56.0 or higher
- `uuid` crate dependency for UUID functionality

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

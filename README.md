# X Utils Library
[![Format](https://github.com/Revanthshalon/x/actions/workflows/format.yml/badge.svg)](https://github.com/Revanthshalon/x/actions/workflows/format.yml) [![Lint](https://github.com/Revanthshalon/x/actions/workflows/lint.yml/badge.svg)](https://github.com/Revanthshalon/x/actions/workflows/lint.yml) [![Security Audit](https://github.com/Revanthshalon/x/actions/workflows/audit.yml/badge.svg)](https://github.com/Revanthshalon/x/actions/workflows/audit.yml)
A collection of utility modules providing robust error handling, tracing, and UUID generation functionality for Rust applications.

## Features

### Error Handling (`errorsx`)
- Rich error context system with metadata support
- HTTP status codes integration
- Request ID tracking
- Debug information attachment
- Flexible key-value details storage
- Builder pattern for easy error context construction
- Implements standard Error and Display traits

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

### Error Context

```rust
use x::errorsx::ErrorContext;

let error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
let context = ErrorContext::new(error)
    .with_status_code(404)
    .with_reason("The requested resource could not be found".to_string());
```

### UUID Generation

```rust
use x::uuidx;

let new_uuid = uuidx::generate_new_v4();
```

## Available Traits

The error handling system provides several carrier traits:

- `StatusCodeCarrier`
- `RequestIdCarrier`
- `ReasonCarrier`
- `DebugCarrier`
- `StatusCarrier`
- `DetailsCarrier`
- `IdCarrier`

## Requirements

- Rust 1.56.0 or higher
- `uuid` crate dependency for UUID functionality

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

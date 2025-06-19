//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error on crates.io](https://img.shields.io/crates/v/cdumay_error)](https://crates.io/crates/cdumay_error)
//! [![cdumay_error on docs.rs](https://docs.rs/cdumay_error/badge.svg)](https://docs.rs/cdumay_error)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error)
//!
//! A collection of standard error types and error kinds commonly used in Rust applications.
//! This crate provides predefined error types and kinds using the `cdumay_core` framework.
//!
//! ## Features
//!
//! - Common error types for IO operations and unexpected errors
//! - Easy integration with the `cdumay_core` framework
//!
//! ## Examples
//!
//! ```rust
//! use cdumay_error::{FileNotExists, Unexpected};
//! use std::path::Path;
//! use cdumay_core::Result;
//!
//! // Creating a FileNotExists error
//! fn check_file(path: &Path) -> Result<()> {
//!     if !path.exists() {
//!         return Err(FileNotExists::new().with_message(format!(
//!             "File {} does not exist",
//!             path.display()
//!         )).into());
//!     }
//!     Ok(())
//! }
//!
//! // Using Unexpected error for runtime errors
//! // Note: We use From<std::result::Result> to return cdumay_core::Result
//! fn divide(a: i32, b: i32) -> Result<i32> {
//!     if b == 0 {
//!         return Err(Unexpected::new().with_message("Division by zero".into()).into());
//!     }
//!     Ok(a / b)
//! }
//! ```
//!
//! ## Error Handling
//!
//! All errors implement the `Into<Error>`, providing consistent  error handling across your application:
//!
//! ```rust
//! use cdumay_error::FileRead;
//! use cdumay_core::Result;
//!
//! fn read_content() -> Result<String> {
//!     let err = FileRead::new().with_message("Failed to read config file".into());
//!     
//!     // Access error properties
//!     println!("Error code: {}", err.code());
//!     println!("Message: {}", err.message());
//!     
//!     Err(err.into())
//! }
//! ```

use cdumay_core::{define_errors, define_kinds};

define_kinds! {
    UnknownError = (500, "Unexpected error"),
    IoError = (500, "IO error"),
    ValidationError = (400, "Validation error"),
    InvalidConfiguration = (400, "Invalid Configuration"),
}

define_errors! {
    Unexpected = UnknownError,
    FileRead = IoError,
    FileNotExists = IoError,
    DeserializationError = ValidationError,
    SerializationError = ValidationError
}

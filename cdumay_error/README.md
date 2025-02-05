# cdumay_error

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_error on crates.io](https://img.shields.io/crates/v/cdumay_error)](https://crates.io/crates/cdumay_error)
[![cdumay_error on docs.rs](https://docs.rs/cdumay_error/badge.svg)](https://docs.rs/cdumay_error)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error)

`cdumay_error` is a Rust library designed for extended error management. It leverages the `cdumay_error_derive` crate, which provides procedural macros
to simplify the definition of structured error types. The primary goal of `cdumay_error` is to enhance error handling in Rust applications by
making error definition more declarative and reducing boilerplate code.

## Features

* Provides extended error management capabilities.
* Implements the `cdumay_error::AsError` trait for easy integration.
* Supports structured error kinds and categorized error handling.

## Usage

To utilize `cdumay_error` in your project, follow these steps:

1. **Add Dependencies**: Include `cdumay_error` with the feature `derive` in your `Cargo.toml`:

```toml
[dependencies]
cdumay_error = "1.0"
```

2. **Define Error**: Define `cdumay_error::ErrorKind` and struct which implement `cdumay_error::AsError` to handle an error:

```rust

use cdumay_error::AsError;

#[allow(non_upper_case_globals)]
pub const IoError: cdumay_error::ErrorKind = cdumay_error::ErrorKind(
    "IoError",
    "Input / output error",
    500,
    "The requested file raised error"
);
#[derive(Debug, Clone)]
pub struct NotFoundError {
    class: String,
    message: String,
    details: Option<std::collections::BTreeMap<String, serde_value::Value>>,
}

impl NotFoundError {
    pub const kind: cdumay_error::ErrorKind = IoError;

    pub fn new() -> Self {
        Self {
            class: format!("{}::{}::{}", Self::kind.side(), Self::kind.name(), "NotFoundError"),
            message: Self::kind.description().into(),
            details: None,
        }
    }

    pub fn set_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }

    pub fn set_details(mut self, details: std::collections::BTreeMap<String, serde_value::Value>) -> Self {
        self.details = Some(details);
        self
    }

    pub fn convert(error: cdumay_error::Error) -> Self {
        let mut err_clone = error.clone();
        let mut details = error.details.unwrap_or_default();
        err_clone.details = None;
        details.insert("origin".to_string(), serde_value::to_value(err_clone).unwrap());

        Self {
            class: format!("{}::{}::{}", Self::kind.side(), Self::kind.name(), "NotFoundError"),
            message: Self::kind.description().into(),
            details: Some(details),
        }
    }
}

impl AsError for NotFoundError {
    fn kind() -> cdumay_error::ErrorKind {
        Self::kind
    }
    fn message(&self) -> String {
        self.message.clone()
    }
    fn class(&self) -> String {
        self.class.clone()
    }
    fn details(&self) -> Option<std::collections::BTreeMap<String, serde_value::Value>> {
        self.details.clone()
    }
}

impl std::error::Error for NotFoundError {}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {} ({}): {}", Self::kind.message_id(), "NotFoundError", Self::kind.code(), self.message())
    }
}
```

In this example:

* we create the struct `IoError` as `cdumay_error::ErrorKind`
* we create the struct `NotFoundError` which implements `cdumay_error::AsError`

3. Implementing Error Handling: With the above definitions, you can now handle errors in your application as follows:

```rust
use std::fs::File;
use std::io::Read;

fn try_open_file(path: &str) -> cdumay_error::Result<File> {
    Ok(File::open(path).map_err(|err| {
        let mut err = cdumay_error::Error::default();
        err.message = err.to_string();
        err
    })?)
}

fn main() {
    let path = "example.txt";

    match try_open_file(path) {
        Ok(file) => println!("File: {:?}", file),
        Err(e) => eprintln!("{}", e),
    }
}
```
This will output:

```
[Err-00001] Client::IoError::NotFoundError (500) - No such file or directory (os error 2)
```

## Macros

To automatically generate implementations for custom error types, enable the feature `derive` in your Cargo.toml:

```toml
[dependencies]
cdumay_error = { version = "1.0", features = ["derive"] }
```

Then, use the provided derive macros to define your error and error kind structs:

```rust
use cdumay_error::{define_errors, define_kinds, AsError};

define_kinds! {
    UnknownError = ("Err-00001", 500, "Unexpected error"),
    IoError = ("Err-00001", 400, "IO error")
}
define_errors! {
    Unexpected = UnknownError,
    FileRead = IoError,
    FileNotExists = IoError
}
```

See [cdumay_error_derive](https://docs.rs/cdumay_error_derive) documentation for more information.


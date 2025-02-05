use crate::ErrorKind;
use serde::Serialize;

/// Trait representing a structured error with categorized information.
///
/// The `AsError` trait provides a standardized way to define and retrieve
/// detailed error information, including its kind, message, classification,
/// and additional details.
///
/// Implementing this trait allows for better error management and debugging
/// by ensuring consistency in error structures.
///
pub trait AsError {
    /// The kind of error.
    ///
    /// The error kind represents a categorized type of error, allowing
    /// consumers to handle different error types distinctly.
    fn kind() -> ErrorKind;

    /// The error message.
    ///
    /// This message provides a human-readable description of the error.
    fn message(&self) -> String;

    /// The class or category of the error.
    ///
    /// The class helps in further classifying errors, providing an additional
    /// layer of categorization beyond `ErrorKind`.
    fn class(&self) -> String;

    /// Additional details related to the error.
    ///
    /// This method provides an optional key-value mapping of extra data
    /// associated with the error, which can be useful for debugging
    /// or logging purposes.
    fn details(&self) -> Option<std::collections::BTreeMap<String, serde_value::Value>>;
}

/// A structured error type with categorized information.
///
/// The `Error` struct represents an error with a specific kind, classification,
/// message, and optional additional details.
///
/// This structure is designed to facilitate error handling by providing
/// detailed information that can be logged or displayed.
///
#[derive(Debug, Clone, Serialize)]
pub struct Error {
    /// The kind of error.
    ///
    /// This field categorizes the error, allowing distinct handling based on
    /// its type. It is skipped during serialization.
    #[serde(skip_serializing)]
    pub kind: ErrorKind,

    /// The class or category of the error.
    ///
    /// This helps further classify the error beyond its kind.
    pub class: String,

    /// A human-readable message describing the error.
    pub message: String,

    /// Additional details related to the error.
    ///
    /// This optional field contains extra context in a key-value format,
    /// which can be useful for debugging or logging purposes.
    pub details: Option<std::collections::BTreeMap<String, serde_value::Value>>,
}

/// Converts any type implementing `AsError` into an `Error` instance.
///
/// This implementation allows seamless conversion from custom error types
/// that implement `AsError` into the `Error` struct, preserving structured
/// error information.
///
/// # Type Parameters
/// - `E`: A type that implements the `AsError` trait.
///
/// # Example
/// ```rust
/// #[allow(non_upper_case_globals)]
/// pub const IoError: cdumay_error::ErrorKind = cdumay_error::ErrorKind(
///     "IoError",
///     "Input / output error",
///     500,
///     "The requested file raised error"
/// );
/// #[derive(Debug, Clone)]
/// pub struct NotFoundError {
///     class: String,
///     message: String,
///     details: Option<std::collections::BTreeMap<String, serde_value::Value>>,
/// }
/// impl NotFoundError {
///     pub const kind: cdumay_error::ErrorKind = IoError;
///
///     pub fn new() -> Self {
///         Self {
///             class: format!("{}::{}::{}", Self::kind.side(), Self::kind.name(), "NotFoundError"),
///             message: Self::kind.description().into(),
///             details: None,
///         }
///     }
/// }
/// impl cdumay_error::AsError for NotFoundError {
///     fn kind() -> cdumay_error::ErrorKind {
///         Self::kind
///     }
///     fn class(&self) -> String {
///         self.class.clone()
///     }
///     fn message(&self) -> String {
///         self.message.clone()
///     }
///     fn details(&self) -> Option<std::collections::BTreeMap<String, serde_value::Value>> {
///         self.details.clone()
///     }
/// }
/// let custom_error = NotFoundError::new();
/// let error: cdumay_error::Error = custom_error.into();
/// ```
impl<E: AsError> From<E> for Error {
    fn from(value: E) -> Self {
        Error {
            kind: E::kind(),
            class: value.class(),
            message: value.message(),
            details: value.details(),
        }
    }
}

/// Converts an `Error` into a `std::io::Error`.
///
/// This implementation maps an `Error` to an `std::io::Error` using the
/// `InvalidData` error kind and formats the error message accordingly.
/// This allows for seamless integration with Rust's standard I/O error handling.
///
/// # Example
/// ```rust
/// let custom_error = cdumay_error::Error {
///     kind: cdumay_error::ErrorKind("NotFound", "MSG001", 404, "Not Found"),
///     class: "Client::NotFound::MyError".to_string(),
///     message: "Not Found".to_string(),
///     details: None,
/// };
/// let io_error: std::io::Error = custom_error.into();
/// ```
impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{}", e))
    }
}

/// Implements the `Display` trait for `Error`.
///
/// This implementation formats the error as a human-readable string,
/// including its kind, class, code, and message. It provides a structured
/// error output that can be useful for logging or displaying errors in a UI.
///
/// # Format
/// ```text
/// [message_id] class (code) - message
/// ```
///
/// # Example
/// ```rust
/// let error = cdumay_error::Error {
///     kind: cdumay_error::ErrorKind("NotFound", "MSG001", 404, "Not Found"),
///     class: "Client::NotFound::MyError".to_string(),
///     message: "Not Found".to_string(),
///     details: None,
/// };
/// println!("{}", error);
/// ```
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "[{}] {} ({}) - {}",
                self.kind.message_id(),
                self.class,
                self.kind.code(),
                self.message
            )
        )
    }
}

impl Default for Error {
    /// Creates a default instance of `Error`.
    ///
    /// This implementation of the `Default` trait provides a default error
    /// of type `Error`, with predefined values:
    /// - **Kind**: Represents an internal server error (`InternalServerError`, `MSG000`, 500, "Internal Server Error").
    /// - **Class**: Describes the error as a server-side internal error (`Server::InternalServerError::Error`).
    /// - **Message**: The human-readable error message ("Internal Server Error").
    /// - **Details**: No additional error details are provided (`None`).
    ///
    /// This can be used when you need a generic error with standard values.
    ///
    /// # Example
    /// ```
    /// let error: cdumay_error::Error = Default::default();
    /// assert_eq!(error.kind.name(), "InternalServerError");
    /// assert_eq!(error.message, "Internal Server Error");
    /// assert_eq!(error.class, "Server::InternalServerError::Error");
    /// assert!(error.details.is_none());
    /// ```
    fn default() -> Self {
        Error {
            kind: ErrorKind("InternalServerError", "MSG000", 500, "Internal Server Error"),
            class: "Server::InternalServerError::Error".to_string(),
            message: "Internal Server Error".to_string(),
            details: None,
        }
    }
}

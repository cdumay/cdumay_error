pub use serde;

/// Represents a categorized error kind with associated metadata.
///
/// The `ErrorKind` struct defines a specific type of error, providing
/// a unique identifier, category, numeric code, and a message ID.
/// This allows for structured and meaningful error classification.
///
/// # Example
/// ```rust
/// let kind = cdumay_error::ErrorKind("NotFound", "MSG001", 404, "Not Found");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorKind(
    /// A unique error identifier (name).
    pub &'static str,
    /// A message ID associated with the error.
    pub &'static str,
    /// A numeric error code.
    pub u16,
    /// A human-readable description of the error.
    pub &'static str,
);

impl ErrorKind {
    /// Returns the name of the error.
    ///
    /// # Example
    /// ```
    /// let error = cdumay_error::ErrorKind("NotFound", "MSG001", 404, "Not Found");
    /// assert_eq!(error.name(), "NotFound");
    /// ```
    pub fn name(&self) -> &'static str {
        self.0
    }

    /// Returns the message ID of the error.
    ///
    /// # Example
    /// ```
    /// let error = cdumay_error::ErrorKind("NotFound", "MSG001", 404, "Not Found");
    /// assert_eq!(error.message_id(), "MSG001");
    /// ```
    pub fn message_id(&self) -> &'static str {
        self.1
    }

    /// Returns the numerical error code.
    ///
    /// # Example
    /// ```
    /// let error = cdumay_error::ErrorKind("NotFound", "MSG001", 404, "Not Found");
    /// assert_eq!(error.code(), 404);
    /// ```
    pub fn code(&self) -> u16 {
        self.2
    }

    /// Returns the description of the error.
    ///
    /// # Example
    /// ```
    /// let error = cdumay_error::ErrorKind("NotFound", "MSG001", 404, "Not Found");
    /// assert_eq!(error.description(), "Not Found");
    /// ```
    pub fn description(&self) -> &'static str {
        self.3
    }

    /// Determines whether the error originates from the client or the server.
    ///
    /// - Errors with codes in the range 0 to 499 are classified as **Client** errors.
    /// - Errors with codes 500 or higher are classified as **Server** errors.
    ///
    /// # Example
    /// ```
    /// let client_error = cdumay_error::ErrorKind("NotFound", "MSG001", 404, "Not Found");
    /// assert_eq!(client_error.side(), "Client");
    ///
    /// let server_error = cdumay_error::ErrorKind("InternalServerError", "MSG002", 500, "Internal Server Error");
    /// assert_eq!(server_error.side(), "Server");
    /// ```
    pub fn side(&self) -> &'static str {
        match self.code() {
            0..=499 => "Client",
            _ => "Server",
        }
    }
}

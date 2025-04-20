use crate::Error;
use std::collections::BTreeMap;

/// A trait for converting custom errors into a structured application-level `cdumay::Error`.
///
/// This trait provides a standard way to enrich errors with context and origin information,
/// and to convert them into a uniform format that supports metadata (e.g., HTTP status codes, error codes, etc.).
///
/// Types implementing this trait define how to transform their native error types into a `cdumay::Error`.
pub trait ErrorConverter {
    /// The associated error type being converted (e.g., a 3rd-party crate error).
    type Error: std::error::Error;

    /// Internal helper that extracts a message and attaches the original error to the context.
    ///
    /// # Arguments
    /// - `error`: The error instance being converted.
    /// - `text`: Optional custom message to override the default error string.
    /// - `context`: Key-value metadata that will be included with the error.
    ///
    /// # Returns
    /// A tuple of:
    /// - `String`: The error message to display.
    /// - `BTreeMap<String, serde_value::Value>`: The enriched context with the original error.
    fn store_origin(
        error: &Self::Error,
        text: Option<String>,
        context: BTreeMap<String, serde_value::Value>,
    ) -> (String, BTreeMap<String, serde_value::Value>) {
        match text {
            Some(text) => (text, {
                let mut ctx = context.clone();
                ctx.insert("origin".to_string(), serde_value::Value::String(error.to_string()));
                ctx
            }),
            None => (error.to_string(), context.clone()),
        }
    }

    /// Converts an error into a `cdumay::Error`, enriching it with context and an optional message.
    ///
    /// This is a convenience method that first stores the error origin using [`store_origin`] and then
    /// delegates to the implementor's [`convert`] method.
    ///
    /// # Arguments
    /// - `error`: The source error.
    /// - `text`: Optional message override.
    /// - `context`: Additional structured metadata to include.
    ///
    /// # Returns
    /// A `cdumay::Error` with standardized structure and context.
    fn convert_error(error: &Self::Error, text: Option<String>, context: BTreeMap<String, serde_value::Value>) -> Error {
        let (text, context) = Self::store_origin(error, text, context);
        Self::convert(error, text, context)
    }

    /// Implemented by concrete types to define how to transform the error into a `cdumay::Error`.
    ///
    /// This function should construct a fully-typed `cdumay::Error` using the provided message and context.
    ///
    /// # Arguments
    /// - `error`: The original error object.
    /// - `text`: The final message to associate with the error.
    /// - `context`: A map of structured metadata for debugging or HTTP responses.
    ///
    /// # Returns
    /// A fully constructed `cdumay::Error`.
    fn convert(error: &Self::Error, text: String, context: BTreeMap<String, serde_value::Value>) -> Error;
}

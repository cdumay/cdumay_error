#[cfg(test)]
#[cfg(feature = "derive")]
mod tests {
    use cdumay_error::{define_errors, define_kinds, AsError, Error, ErrorConverter};
    use serde_value::Value;
    use std::collections::BTreeMap;
    use std::fmt;

    // Define a mock error kind and wrapper for testing
    define_kinds! {
        MockKind = ("MOCK-00001", 500, "Mock error kind")
    }

    define_errors! {
        MockError = MockKind,
    }

    /// A simple error struct to test ErrorConverter
    #[derive(Debug)]
    struct MyError {
        message: String,
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl std::error::Error for MyError {}

    /// Implement the trait for MyError
    struct MyErrorConverter;

    impl ErrorConverter for MyErrorConverter {
        type Error = MyError;

        fn convert(error: &Self::Error, text: String, context: BTreeMap<String, Value>) -> Error {
            MockError::new().set_message(text).set_details(context).into()
        }
    }

    #[test]
    fn test_store_origin_with_text() {
        let error = MyError { message: "Oops".into() };
        let mut context = BTreeMap::new();
        context.insert("key".to_string(), Value::String("value".to_string()));

        let (message, updated_context) = MyErrorConverter::store_origin(&error, Some("Custom message".into()), context.clone());

        assert_eq!(message, "Custom message");
        assert!(updated_context.contains_key("origin"));
        assert_eq!(updated_context.get("origin").unwrap(), &Value::String("Oops".to_string()));
    }

    #[test]
    fn test_store_origin_without_text() {
        let error = MyError {
            message: "Default error".into(),
        };
        let context = BTreeMap::new();

        let (message, updated_context) = MyErrorConverter::store_origin(&error, None, context.clone());

        assert_eq!(message, "Default error");
        assert!(updated_context.is_empty());
    }

    #[test]
    fn test_convert_error_with_custom_text() {
        let error = MyError {
            message: "Conversion failed".into(),
        };
        let mut context = BTreeMap::new();
        context.insert("field".to_string(), Value::String("value".into()));

        let result = MyErrorConverter::convert_error(&error, Some("Something went wrong".into()), context.clone());

        assert_eq!(result.message, "Something went wrong");
        assert_eq!(result.kind.message_id(), "MOCK-00001");
        assert!(result.details.clone().unwrap().contains_key("origin"));
        assert_eq!(
            result.details.unwrap().get("origin").unwrap(),
            &Value::String("Conversion failed".to_string())
        );
    }

    #[test]
    fn test_convert_error_with_default_message() {
        let error = MyError {
            message: "Fallback error".into(),
        };
        let context = BTreeMap::new();

        let result = MyErrorConverter::convert_error(&error, None, context);

        assert_eq!(result.message, "Fallback error");
        assert_eq!(result.kind.message_id(), "MOCK-00001");
    }
}

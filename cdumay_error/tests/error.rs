#[cfg(test)]
mod test {
    use cdumay_error::{AsError, ErrorKind};

    const TEST_ERROR: ErrorKind = ErrorKind("TestError", "TEST-00001", 500, "Test error message");

    #[derive(Debug, Clone)]
    pub struct MyError {
        class: String,
        message: String,
        details: Option<std::collections::BTreeMap<String, serde_value::Value>>,
    }

    impl MyError {
        #[allow(non_upper_case_globals)]
        pub const kind: ErrorKind = TEST_ERROR;
        pub fn new() -> Self {
            Self {
                class: format!("{}::{}::MyError", Self::kind.side(), Self::kind.name(),),
                message: Self::kind.description().into(),
                details: None,
            }
        }
        pub fn set_message(mut self, message: String) -> Self {
            self.message = message;
            self
        }
        pub fn set_details(
            mut self,
            details: std::collections::BTreeMap<String, serde_value::Value>,
        ) -> Self {
            self.details = Some(details);
            self
        }
    }
    impl AsError for MyError {
        fn kind() -> ErrorKind {
            Self::kind
        }
        fn class(&self) -> String {
            self.class.clone()
        }
        fn message(&self) -> String {
            self.message.clone()
        }
        fn details(&self) -> Option<std::collections::BTreeMap<String, serde_value::Value>> {
            self.details.clone()
        }
    }

    impl std::error::Error for MyError {}

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "[{}] MyError ({}): {}",
                Self::kind.message_id(),
                Self::kind.code(),
                self.message()
            )
        }
    }

    #[test]
    fn test_kind() {
        assert_eq!(TEST_ERROR.name(), "TestError");
        assert_eq!(TEST_ERROR.message_id(), "TEST-00001");
        assert_eq!(TEST_ERROR.code(), 500);
        assert_eq!(TEST_ERROR.description(), "Test error message");
        assert_eq!(TEST_ERROR.side(), "Server");
    }
    #[test]
    fn test_error() {
        let mut details = std::collections::BTreeMap::new();
        details.insert("foo".to_string(), serde_value::to_value("foo").unwrap());

        let err = MyError::new()
            .set_message("Test error".to_string())
            .set_details(details.clone());
        assert_eq!(MyError::kind, TEST_ERROR);
        assert_eq!(err.message(), "Test error");
        assert_eq!(err.details(), Some(details));
        assert_eq!(err.class(), "Server::TestError::MyError");
        assert_eq!(format!("{}", err), "[TEST-00001] MyError (500): Test error");
    }
}
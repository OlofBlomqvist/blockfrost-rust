/// Useful for testing our types against JSON examples in the documentation.
#[cfg(test)]
macro_rules! test_schema {
    { $test_name:tt, $schema_name:ty, $text:expr } => {
        #[test]
        fn $test_name() {
            let parsed = serde_json::from_str::<$schema_name>($text);
            // Should succeed
            parsed.unwrap();
        }
    };
}

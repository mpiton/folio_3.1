// Test to validate that all test dependencies are properly accessible
//
// This test ensures that the testing infrastructure is correctly configured
// and all required crates are available for use in the test suite.

#[cfg(test)]
mod dependencies {
    // Verify wiremock is available
    #[test]
    fn wiremock_available() {
        use wiremock::matchers;
        // Just verifying the import works
        let _ = matchers::method("GET");
    }

    // Verify fake is available
    #[test]
    fn fake_available() {
        use fake::faker::internet::en::SafeEmail;
        use fake::Fake;
        let _email = SafeEmail().fake::<String>();
    }

    // Verify tokio is available for async tests
    #[tokio::test]
    async fn tokio_async_test_available() {
        // This demonstrates async test capability
        let _ = tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    }

    // Verify mongodb is available
    #[test]
    fn mongodb_available() {
        use mongodb::bson::doc;
        let _doc = doc! { "test": "value" };
    }

    // Verify tempfile is available
    #[test]
    fn tempfile_available() {
        let _temp_dir = tempfile::TempDir::new();
    }

    // Verify filetime is available
    #[test]
    fn filetime_available() {
        use filetime::FileTime;
        let _now = FileTime::now();
    }
}

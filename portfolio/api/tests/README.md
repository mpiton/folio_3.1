# Backend API Test Suite

This directory contains the test infrastructure for the portfolio API backend. The test suite is organized into unit tests (in `src/`), integration tests, and test utilities.

## Directory Structure

```
tests/
├── common/
│   ├── mod.rs          # Test utilities and helper functions
│   ├── fixtures.rs     # Test data factories and builders
│   └── lib.rs          # Module exports
├── integration/
│   └── mod.rs          # Integration test organization
├── integration_tests.rs # Integration test harness
├── dependencies_validation.rs  # Verify test dependencies
├── fixtures_usage.rs   # Examples of fixture usage
└── README.md           # This file
```

## Test Files Overview

### Core Test Utilities

- **`common/mod.rs`**: Core test infrastructure
  - `setup_mongodb()` - Sets up MongoDB testcontainer
  - `cleanup_db()` - Cleans up test databases
  - `mock_rss_feed_server()` - Creates WireMock server
  - `async_utils` module with `wait_for()` and `retry_async()`

- **`common/fixtures.rs`**: Test data generation
  - Contact request builders
  - RSS feed and item generators
  - Realistic fake data using the `fake` crate

### Test Harnesses

- **`integration_tests.rs`**: Main integration test harness
- **`dependencies_validation.rs`**: Validates all test dependencies are accessible
- **`fixtures_usage.rs`**: Demonstrates how to use fixtures in tests

## Running Tests

### All Tests
```bash
cargo test
```

### Unit Tests Only
```bash
cargo test --lib
```

### Integration Tests
```bash
cargo test --test integration_tests
```

### Specific Test File
```bash
cargo test --test fixtures_usage
```

### Specific Test
```bash
cargo test --test fixtures_usage -- contact_validation_works
```

### With Verbose Output
```bash
cargo test -- --nocapture
```

### Count Total Tests
```bash
cargo test 2>&1 | grep "test result:" | awk -F'[;]' '{gsub(/[^0-9]/,"",$1); sum+=$1} END {print "Total: " sum}'
```

## Test Dependencies

All test-specific dependencies are listed in `Cargo.toml` under `[dev-dependencies]`:

| Crate | Version | Purpose |
|-------|---------|---------|
| `fake` | 2.9 | Generate realistic test data |
| `wiremock` | 0.6.5 | Mock HTTP services |
| `testcontainers` | 0.23 | Docker container management for integration tests |
| `tempfile` | 3.23.0 | Create temporary files/directories |
| `filetime` | 0.2.26 | Manipulate file timestamps |
| `tokio-test` | 0.4 | Tokio async test utilities |
| `mockito` | 1.6.1 | HTTP mocking (alternative) |

## Using Fixtures in Tests

### Contact Request Fixture

```rust
#[tokio::test]
async fn test_contact_submission() {
    // Generate a random valid contact request
    let contact = fixtures::sample_contact_request();

    // Or use the builder for custom data
    let contact = fixtures::ContactRequestBuilder::new()
        .name("John Doe")
        .email("john@example.com")
        .subject("Test Subject")
        .message("This is a test message")
        .build();

    // Your test logic here
}
```

### RSS Feed Fixtures

```rust
#[test]
fn test_rss_parsing() {
    // Generate a single RSS item
    let item = fixtures::sample_rss_item();

    // Generate multiple RSS items
    let items = fixtures::sample_rss_items(5);

    // Your test logic here
}
```

### MongoDB Testing

```rust
#[tokio::test]
async fn test_database_operation() {
    let (client, db) = setup_mongodb().await.unwrap();

    // Your test logic here

    cleanup_db(&db, &["contacts"]).await.unwrap();
}
```

### HTTP Mocking with WireMock

```rust
#[tokio::test]
async fn test_external_api() {
    let server = mock_rss_feed_server().await.unwrap();

    // Your test logic here - use server.uri() to get the mock URL
}
```

## Phase 1 Infrastructure Status

Phase 1 focuses on setting up the test infrastructure and is now complete:

- [x] Test dependency configuration
- [x] MongoDB testcontainer setup
- [x] WireMock HTTP mocking
- [x] Async test utilities
- [x] Test data factories
- [x] Dependency validation

See `PHASE1_INFRASTRUCTURE.md` for detailed information.

## Phase 2 Planning (Coming Soon)

Phase 2 will implement actual integration tests for:

- FeedService RSS feed fetching and storage
- MessageService contact form submission
- Database operations with MongoDB
- Email service integration
- API endpoint testing

## Phase 3 Planning

Phase 3 will implement:

- Full API endpoint testing (GET /api/feeds, POST /api/contact, etc.)
- End-to-end user journey testing
- Performance and load testing
- Security testing (injection, XSS, CSRF, etc.)

## Best Practices

1. **Use Fixtures**: Always use the fixture factories for test data to keep tests clean and maintainable
2. **Cleanup**: Always call `cleanup_db()` after tests to avoid side effects
3. **Async**: Use `#[tokio::test]` for async tests
4. **Isolation**: Each test should be independent and not rely on other tests
5. **Naming**: Test names should clearly describe what they test

## Troubleshooting

### Docker Issues
If testcontainers fail, ensure Docker is running:
```bash
docker ps
```

### Slow Tests
- Use parallel test execution: `cargo test -- --test-threads=4`
- Reuse databases where possible using the same test container

### Import Errors
Make sure to import from the correct modules:
```rust
use portfolio_api::models::contact::Request as ContactRequest;
use portfolio_api::models::rss::RssItem;
```

## Contributing

When adding new tests:

1. Create test files in the appropriate directory
2. Use fixtures for test data
3. Ensure tests are isolated and can run in any order
4. Add documentation for non-obvious test scenarios
5. Run `cargo test` to verify everything passes

---

**Documentation**: See `PHASE1_INFRASTRUCTURE.md` for comprehensive infrastructure documentation.

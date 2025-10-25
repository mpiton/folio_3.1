// Integration tests module
//
// This module contains integration tests for the API services.
// Tests are organized by functionality:
// - rss_service_integration_test.rs: FeedService tests (see tests/rss_service_integration_test.rs)
// - contact_service_test.rs: MessageService tests (Phase 2.1 - Input Validation, Persistence, Email)
// - middleware_test.rs: RateLimiter and MongoSanitizer tests (Phase 2.2 - Rate Limiting, Injection)
// - api_endpoint_tests: HTTP endpoint tests (todo - Phase 3)
//
// Phase 1: Infrastructure setup (completed)
// Phase 2: Service integration tests (completed - RssService, MessageService, Middleware)
// Phase 3: API endpoint tests (todo)

pub mod contact_service_test;
pub mod middleware_test;

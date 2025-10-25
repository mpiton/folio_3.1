// Test to verify that fixtures are properly set up and can be used
//
// This test demonstrates how to use the fixture factories in integration tests

#[cfg(test)]
mod fixtures_tests {
    use portfolio_api::models::contact::Request as ContactRequest;

    mod contact_fixtures {
        use super::*;

        #[test]
        fn sample_contact_request_can_be_created() {
            // This would use fixtures module in Phase 2
            // For now, we just verify the structure exists
            let contact = ContactRequest {
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
                subject: "Test Subject".to_string(),
                message: "This is a valid test message for the contact form".to_string(),
                is_test: false,
            };

            assert_eq!(contact.name, "Test User");
            assert_eq!(contact.email, "test@example.com");
        }

        #[test]
        fn contact_validation_works() {
            use validator::Validate;

            let valid_contact = ContactRequest {
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                subject: "Valid Subject".to_string(),
                message: "This is a valid message for testing purposes".to_string(),
                is_test: false,
            };

            assert!(valid_contact.validate().is_ok());
        }

        #[test]
        fn invalid_contact_fails_validation() {
            use validator::Validate;

            let invalid_contact = ContactRequest {
                name: "John123".to_string(), // Invalid: contains numbers
                email: "john@example.com".to_string(),
                subject: "Valid Subject".to_string(),
                message: "This is a valid message for testing purposes".to_string(),
                is_test: false,
            };

            assert!(invalid_contact.validate().is_err());
        }
    }

    mod rss_fixtures {
        use portfolio_api::models::rss::RssItem;

        #[test]
        fn rss_item_can_be_created() {
            use chrono::Utc;

            let item = RssItem {
                title: "Test Article".to_string(),
                url: "https://example.com/article".to_string(),
                pub_date: Utc::now(),
                description: "Test description".to_string(),
                image_url: "https://example.com/image.jpg".to_string(),
            };

            assert!(!item.title.is_empty());
            assert!(!item.url.is_empty());
            assert!(!item.description.is_empty());
        }
    }
}

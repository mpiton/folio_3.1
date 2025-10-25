/// Test data fixtures for realistic integration testing
///
/// This module provides factory functions that generate realistic test data
/// for contacts, RSS feeds, and other models using the `fake` crate.
use chrono::{DateTime, Duration, Utc};
use fake::faker::internet::en::*;
use fake::faker::lorem::en::*;
use fake::faker::name::en::*;
use fake::Fake;
use mongodb::bson::oid::ObjectId;
use portfolio_api::models::contact::Request as ContactRequest;
use portfolio_api::models::rss::{Feed, FeedItem, ParsedFeed, ParsedItem, RssItem};

/// Generates a valid contact request with random but realistic data
///
/// # Returns
/// A `ContactRequest` with:
/// - Valid name (2-100 chars, only letters, spaces, hyphens, apostrophes)
/// - Valid email format
/// - Valid subject (2-100 chars)
/// - Valid message (10-1000 chars, no HTML or excessive links)
pub fn sample_contact_request() -> ContactRequest {
    ContactRequest {
        name: Name().fake::<String>(),
        email: SafeEmail().fake::<String>(),
        subject: Words(2..5).fake::<Vec<String>>().join(" "),
        message: Sentences(3..8).fake::<Vec<String>>().join(" "),
        is_test: false,
        test_name: None,
    }
}

/// Generates a valid contact request with test flag
#[allow(dead_code)]
pub fn sample_contact_request_test() -> ContactRequest {
    let mut request = sample_contact_request();
    request.is_test = true;
    request
}

/// Generates a contact request with specific field values
///
/// # Example
/// ```ignore
/// let contact = contact_request_builder()
///     .name("John Doe")
///     .email("john@example.com")
///     .build();
/// ```
pub struct ContactRequestBuilder {
    name: String,
    email: String,
    subject: String,
    message: String,
    is_test: bool,
    test_name: Option<String>,
}

impl ContactRequestBuilder {
    pub fn new() -> Self {
        Self {
            name: Name().fake::<String>(),
            email: SafeEmail().fake::<String>(),
            subject: Words(2..5).fake::<Vec<String>>().join(" "),
            message: Sentences(3..8).fake::<Vec<String>>().join(" "),
            is_test: false,
            test_name: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = email.to_string();
        self
    }

    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = subject.to_string();
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    #[allow(dead_code)]
    pub fn is_test(mut self, is_test: bool) -> Self {
        self.is_test = is_test;
        self
    }

    #[allow(dead_code)]
    pub fn test_name(mut self, test_name: &str) -> Self {
        self.test_name = Some(test_name.to_string());
        self
    }

    pub fn build(self) -> ContactRequest {
        ContactRequest {
            name: self.name,
            email: self.email,
            subject: self.subject,
            message: self.message,
            is_test: self.is_test,
            test_name: self.test_name,
        }
    }
}

impl Default for ContactRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Generates a sample RSS feed with realistic metadata
#[allow(dead_code)]
pub fn sample_rss_feed() -> Feed {
    Feed {
        id: Some(ObjectId::new()),
        link: format!("https://example.com/feed/{}", SafeEmail().fake::<String>()),
        created_at: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
        updated_at: Utc::now(),
    }
}

/// Generates multiple RSS feed items (articles)
pub fn sample_rss_items(count: usize) -> Vec<RssItem> {
    (0..count)
        .map(|i| RssItem {
            title: format!("{} {}", Word().fake::<String>(), i),
            url: format!(
                "https://example.com/article-{}-{}",
                i,
                SafeEmail().fake::<String>()
            ),
            pub_date: Utc::now() - Duration::days(i as i64),
            description: Sentences(2..4).fake::<Vec<String>>().join(" "),
            image_url: format!("https://example.com/images/{}.jpg", i),
        })
        .collect()
}

/// Generates a single RSS item
#[allow(dead_code)]
pub fn sample_rss_item() -> RssItem {
    RssItem {
        title: Words(3..6).fake::<Vec<String>>().join(" "),
        url: format!(
            "https://example.com/article/{}",
            Words(2..4).fake::<Vec<String>>().join("-")
        ),
        pub_date: Utc::now(),
        description: Sentences(2..4).fake::<Vec<String>>().join(" "),
        image_url: format!(
            "https://example.com/image/{}.jpg",
            SafeEmail().fake::<String>()
        ),
    }
}

/// Generates a parsed RSS feed (from parsing external source)
#[allow(dead_code)]
pub fn sample_parsed_feed() -> ParsedFeed {
    ParsedFeed {
        title: Words(2..4).fake::<Vec<String>>().join(" "),
        link: format!(
            "https://example.com/feed/{}",
            Words(2..3).fake::<Vec<String>>().join("/")
        ),
        description: Sentence(4..10).fake::<String>(),
        items: (0..3)
            .map(|i| ParsedItem {
                title: Words(3..6).fake::<Vec<String>>().join(" "),
                link: format!("https://example.com/post-{}", i),
                description: Sentence(4..10).fake::<String>(),
                pub_date: Some(Utc::now()),
            })
            .collect(),
    }
}

/// Generates a FeedItem (MongoDB document representation)
pub fn sample_feed_item() -> FeedItem {
    FeedItem {
        id: Some(ObjectId::new()),
        feed_id: ObjectId::new(),
        title: Words(3..6).fake::<Vec<String>>().join(" "),
        link: format!("https://example.com/feed-item/{}", ObjectId::new()),
        description: Sentences(2..4).fake::<Vec<String>>().join(" "),
        pub_date: Utc::now(),
        created_at: Utc::now(),
    }
}

/// Generates multiple FeedItems
pub fn sample_feed_items(count: usize) -> Vec<FeedItem> {
    (0..count).map(|_| sample_feed_item()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_contact_request_is_valid() {
        let contact = sample_contact_request();
        assert!(!contact.name.is_empty());
        assert!(!contact.email.is_empty());
        assert!(!contact.subject.is_empty());
        assert!(!contact.message.is_empty());
    }

    #[test]
    fn contact_request_builder_works() {
        let contact = ContactRequestBuilder::new()
            .name("Test User")
            .email("test@example.com")
            .subject("Test Subject")
            .message("This is a test message for the contact form")
            .build();

        assert_eq!(contact.name, "Test User");
        assert_eq!(contact.email, "test@example.com");
        assert_eq!(contact.subject, "Test Subject");
    }

    #[test]
    fn sample_rss_items_generates_correct_count() {
        let items = sample_rss_items(5);
        assert_eq!(items.len(), 5);
    }

    #[test]
    fn sample_feed_items_generates_correct_count() {
        let items = sample_feed_items(3);
        assert_eq!(items.len(), 3);
    }
}

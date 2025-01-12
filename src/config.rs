#[cfg(test)]
#[must_use]
pub fn test_config() -> Self {
    dotenv::dotenv().ok();
    Self {
        mongo_url: env::var("MONGO_URL").expect("MONGO_URL must be set"),
        host: "127.0.0.1".to_string(),
        port: 3001,
        rss_cache_duration: 60,
        brevo_api_key: "test_key".to_string(),
        recipient_email: "test@example.com".to_string(),
        sender_name: "Test Sender".to_string(),
        sender_email: "test@sender.com".to_string(),
    }
}

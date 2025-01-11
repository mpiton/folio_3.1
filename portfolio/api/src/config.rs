use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub mongo_url: String,
    pub host: String,
    pub port: u16,
    pub rss_cache_duration: u64,
    pub brevo_api_key: String,
    pub recipient_email: String,
    pub sender_name: String,
    pub sender_email: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let host = env::var("HOST").expect("HOST must be set");
        let port = env::var("PORT")
            .expect("PORT must be set")
            .parse()
            .expect("PORT must be a valid number");
        let rss_cache_duration = env::var("RSS_CACHE_DURATION")
            .unwrap_or_else(|_| "3600".to_string())
            .parse()
            .expect("RSS_CACHE_DURATION must be a valid number");

        // Configuration Brevo
        let brevo_api_key = env::var("BREVO_API_KEY").expect("BREVO_API_KEY must be set");
        let recipient_email = env::var("RECIPIENT_EMAIL").expect("RECIPIENT_EMAIL must be set");
        let sender_name = env::var("SENDER_NAME").expect("SENDER_NAME must be set");
        let sender_email = env::var("SENDER_EMAIL").expect("SENDER_EMAIL must be set");

        Config {
            mongo_url,
            host,
            port,
            rss_cache_duration,
            brevo_api_key,
            recipient_email,
            sender_name,
            sender_email,
        }
    }

    #[cfg(test)]
    pub fn test_config() -> Self {
        dotenv::dotenv().ok();
        Config {
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
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

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
    /// Crée une nouvelle instance de Config à partir des variables d'environnement.
    ///
    /// # Panics
    ///
    /// Cette fonction panique si l'une des variables d'environnement suivantes n'est pas définie
    /// ou n'a pas le format attendu :
    /// - `MONGO_URL`
    /// - `HOST`
    /// - `PORT` (doit être un nombre valide)
    /// - `BREVO_API_KEY`
    /// - `RECIPIENT_EMAIL`
    /// - `SENDER_NAME`
    /// - `SENDER_EMAIL`
    ///
    /// La variable `RSS_CACHE_DURATION` est optionnelle et vaut 3600 par défaut.
    #[must_use]
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

        Self {
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

    /// Crée une configuration de test avec des valeurs par défaut.
    ///
    /// # Panics
    ///
    /// Cette fonction panique si la variable d'environnement `MONGO_URL` n'est pas définie.
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
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

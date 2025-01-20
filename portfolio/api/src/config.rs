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
    pub frontend_url: String,
    pub rss_source_url: String,
    pub rss_source_db: String,
    pub rss_source_collection: String,
}

fn env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
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
    /// - `FRONTEND_URL`
    /// - `RSS_SOURCE_URL`
    /// - `RSS_SOURCE_DB`
    /// - `RSS_SOURCE_COLLECTION`
    ///
    /// La variable `RSS_CACHE_DURATION` est optionnelle et vaut 3600 par défaut.
    #[must_use]
    pub fn new() -> Self {
        // Charger les variables d'environnement depuis le fichier .env
        dotenvy::dotenv().ok();

        Self {
            mongo_url: env_or_default("MONGO_URL", "mongodb://localhost:27017"),
            frontend_url: env_or_default("FRONTEND_URL", "http://localhost:3000"),
            brevo_api_key: env_or_default("BREVO_API_KEY", ""),
            recipient_email: env_or_default("RECIPIENT_EMAIL", ""),
            sender_name: env_or_default("SENDER_NAME", ""),
            sender_email: env_or_default("SENDER_EMAIL", ""),
            host: env_or_default("HOST", "127.0.0.1"),
            port: env_or_default("PORT", "3000")
                .parse()
                .expect("PORT must be a number"),
            rss_cache_duration: env::var("RSS_CACHE_DURATION")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .expect("RSS_CACHE_DURATION must be a number"),
            rss_source_url: env::var("RSS_SOURCE_URL").expect("RSS_SOURCE_URL must be set"),
            rss_source_db: env::var("RSS_SOURCE_DB").expect("RSS_SOURCE_DB must be set"),
            rss_source_collection: env::var("RSS_SOURCE_COLLECTION")
                .expect("RSS_SOURCE_COLLECTION must be set"),
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
        // Charger les variables d'environnement depuis le fichier .env.test
        dotenvy::from_filename(".env.test").ok();
        let base_mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let mongo_db = env::var("MONGO_DB").expect("MONGO_DB must be set");
        let mongo_url = format!("{}?authSource={}", base_mongo_url, mongo_db);

        Self {
            mongo_url,
            host: String::from("127.0.0.1"),
            port: 3001,
            rss_cache_duration: 60,
            brevo_api_key: String::from("test_key"),
            recipient_email: String::from("test@example.com"),
            sender_name: String::from("Test Sender"),
            sender_email: String::from("test@sender.com"),
            frontend_url: String::from("http://localhost:3000"),
            rss_source_url: String::from("http://example.com/rss"),
            rss_source_db: String::from("rss_source"),
            rss_source_collection: String::from("rss_items"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

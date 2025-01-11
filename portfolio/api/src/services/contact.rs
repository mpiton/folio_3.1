use crate::config::Config;
use crate::models::contact::ContactForm;
use anyhow::Result;
use futures_util::TryStreamExt;
use mongodb::bson::{doc, Bson, Document};
use mongodb::Database;
use validator::Validate;

pub struct ContactService {
    db: Database,
    config: Config,
    #[cfg(test)]
    brevo_url: String,
}

impl ContactService {
    pub fn new(db: Database, config: Config) -> Self {
        ContactService {
            db,
            config,
            #[cfg(test)]
            brevo_url: "https://api.brevo.com/v3/smtp/email".to_string(),
        }
    }

    pub async fn submit_contact(&self, form: ContactForm) -> Result<()> {
        // Valider le formulaire
        form.validate()?;

        // Stocker dans la base de données
        let collection = self.db.collection::<Document>("contacts");

        collection
            .insert_one(doc! {
                "name": &form.name,
                "email": &form.email,
                "message": &form.message,
                "created_at": Bson::DateTime(mongodb::bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis())),
                "status": "pending"
            })
            .await?;

        // Envoyer l'email via Brevo
        self.send_email(&form).await?;

        Ok(())
    }

    pub async fn get_recent_contacts(&self, limit: i64) -> Result<Vec<ContactForm>> {
        let collection = self.db.collection::<Document>("contacts");

        let mut cursor = collection.find(doc! {}).await?;

        let mut contacts = Vec::new();
        let mut count = 0;

        while let Some(doc) = cursor.try_next().await? {
            if count >= limit {
                break;
            }
            contacts.push(ContactForm {
                name: doc.get_str("name")?.to_string(),
                email: doc.get_str("email")?.to_string(),
                message: doc.get_str("message")?.to_string(),
            });
            count += 1;
        }

        Ok(contacts)
    }

    pub async fn get_contact_stats(&self) -> Result<serde_json::Value> {
        let collection = self.db.collection::<Document>("contacts");

        let pipeline = vec![
            doc! {
                "$group": {
                    "_id": {
                        "$dateToString": {
                            "format": "%Y-%m-%d",
                            "date": "$created_at"
                        }
                    },
                    "count": { "$sum": 1 }
                }
            },
            doc! {
                "$sort": {
                    "_id": 1
                }
            },
        ];

        let mut cursor = collection.aggregate(pipeline).await?;
        let mut stats = Vec::new();

        while let Some(doc) = cursor.try_next().await? {
            stats.push(doc);
        }

        Ok(serde_json::json!({
            "daily_stats": stats
        }))
    }

    async fn send_email(&self, form: &ContactForm) -> Result<()> {
        let client = reqwest::Client::new();

        let email_data = serde_json::json!({
            "sender": {
                "name": self.config.sender_name,
                "email": self.config.sender_email
            },
            "to": [{
                "email": self.config.recipient_email,
                "name": self.config.recipient_email
            }],
            "subject": "Nouveau message de contact",
            "htmlContent": format!(
                "<p><strong>Nom:</strong> {}</p><p><strong>Email:</strong> {}</p><p><strong>Message:</strong></p><p>{}</p>",
                form.name, form.email, form.message
            )
        });

        #[cfg(test)]
        let brevo_url = format!("{}/v3/smtp/email", self.brevo_url);
        #[cfg(not(test))]
        let brevo_url = "https://api.brevo.com/v3/smtp/email";

        let response = client
            .post(brevo_url)
            .header("api-key", &self.config.brevo_api_key)
            .header("Content-Type", "application/json")
            .json(&email_data)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to send email: {}", response.status());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::db::test_utils::create_test_db;
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn test_submit_contact() {
        // Configuration de test
        let config = Config::test_config();

        // Créer une base de données de test
        let db = create_test_db()
            .await
            .expect("Failed to create test database");

        // Créer un mock server pour Brevo avec réponse immédiate
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/v3/smtp/email"))
            .and(header("api-key", "test_key"))
            .respond_with(
                ResponseTemplate::new(200).set_delay(std::time::Duration::from_millis(50)),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        // Créer le service avec l'URL du mock server
        let mut config = config;
        config.brevo_api_key = "test_key".to_string();
        let service = ContactService {
            db: db.clone(),
            config: config.clone(),
            #[cfg(test)]
            brevo_url: mock_server.uri(),
        };

        // Test avec un formulaire valide
        let form = ContactForm {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            message: "This is a test message".to_string(),
        };

        // Soumettre le formulaire
        let result = service.submit_contact(form).await;
        assert!(
            result.is_ok(),
            "Failed to submit contact form: {:?}",
            result
        );

        // Vérifier l'enregistrement en base
        let collection = db.collection::<mongodb::bson::Document>("contacts");
        let doc = collection
            .find_one(doc! { "email": "test@example.com" })
            .await
            .expect("Failed to query database")
            .expect("Document not found");

        assert_eq!(doc.get_str("name").unwrap(), "Test User");
        assert_eq!(doc.get_str("email").unwrap(), "test@example.com");
        assert_eq!(doc.get_str("message").unwrap(), "This is a test message");

        // Nettoyer
        db.drop().await.ok();
    }

    #[test]
    fn test_contact_form_validation() {
        // Test avec des données valides
        let valid_form = ContactForm {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            message: "This is a valid test message".to_string(),
        };
        assert!(valid_form.validate().is_ok());

        // Test avec un email invalide
        let invalid_email = ContactForm {
            name: "Test User".to_string(),
            email: "invalid-email".to_string(),
            message: "This is a test message".to_string(),
        };
        assert!(invalid_email.validate().is_err());

        // Test avec un message trop court
        let short_message = ContactForm {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            message: "Short".to_string(),
        };
        assert!(short_message.validate().is_err());
    }
}

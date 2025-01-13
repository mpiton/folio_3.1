use crate::config::Config;
use crate::models::contact::Request;
use anyhow::Result;
use futures_util::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::Database;
use tokio::sync::Mutex;
use validator::Validate;

lazy_static::lazy_static! {
    static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
}

pub struct MessageService {
    db: Database,
    config: Config,
    #[cfg(test)]
    brevo_url: String,
}

impl MessageService {
    #[must_use]
    pub const fn new(db: Database, config: Config) -> Self {
        Self {
            db,
            config,
            #[cfg(test)]
            brevo_url: String::new(),
        }
    }

    /// Soumet un nouveau formulaire de contact.
    ///
    /// # Errors
    ///
    /// Cette fonction retourne une erreur si :
    /// - La validation du formulaire échoue
    /// - L'insertion dans la base de données échoue
    /// - L'envoi de l'email échoue
    pub async fn submit_contact(&self, form: Request) -> Result<()> {
        // Valider le formulaire
        form.validate()?;

        // Stocker dans la base de données
        #[cfg(test)]
        let collection_name = "contacts_test_submit_contact";
        #[cfg(not(test))]
        let collection_name = "contacts";

        let collection = self.db.collection::<Document>(collection_name);
        println!(
            "Insertion du document dans la collection {}...",
            collection_name
        );

        let doc = doc! {
            "name": form.name.clone(),
            "email": form.email.clone(),
            "message": form.message.clone(),
            "created_at": mongodb::bson::DateTime::now()
        };
        println!("Document à insérer : {:?}", doc);

        let result = collection.insert_one(doc).await?;
        println!("Document inséré avec l'ID : {:?}", result.inserted_id);

        // Envoyer l'email
        self.send_email(&form).await?;

        Ok(())
    }

    #[cfg(test)]
    pub fn with_test_collections(mut self, _test_name: &str) -> Self {
        self.db = self.db.clone();
        self
    }

    /// Récupère les contacts récents, limités au nombre spécifié.
    ///
    /// # Errors
    ///
    /// Cette fonction retourne une erreur si :
    /// - La requête à la base de données échoue
    /// - La lecture des documents échoue
    pub async fn get_recent_contacts(&self, limit: i64) -> Result<Vec<Request>> {
        let collection = self.db.collection::<Document>("contacts");

        let cursor = collection
            .find(doc! {})
            .sort(doc! { "created_at": -1 })
            .limit(limit)
            .await?;

        let mut contacts = Vec::new();
        for doc in cursor.try_collect::<Vec<Document>>().await? {
            contacts.push(Request {
                name: doc
                    .get_str("name")
                    .map_err(|e| anyhow::anyhow!(e))?
                    .to_string(),
                email: doc
                    .get_str("email")
                    .map_err(|e| anyhow::anyhow!(e))?
                    .to_string(),
                message: doc
                    .get_str("message")
                    .map_err(|e| anyhow::anyhow!(e))?
                    .to_string(),
            });
        }
        Ok(contacts)
    }

    /// Récupère les statistiques des contacts.
    ///
    /// # Errors
    ///
    /// Cette fonction retourne une erreur si :
    /// - L'agrégation `MongoDB` échoue
    /// - La lecture des résultats échoue
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

        let cursor = collection.aggregate(pipeline).await?;
        let stats: Vec<Document> = cursor.try_collect().await?;

        Ok(serde_json::json!({
            "daily_stats": stats
        }))
    }

    /// Envoie un email via le service Brevo.
    ///
    /// # Errors
    ///
    /// Cette fonction retourne une erreur si :
    /// - La requête HTTP échoue
    /// - Le service Brevo retourne une erreur
    async fn send_email(&self, form: &Request) -> Result<()> {
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
    use crate::services::db::test_utils::{clean_collections, create_test_db};
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[tokio::test]
    async fn test_submit_contact() {
        // Acquérir le verrou pour éviter les interférences avec d'autres tests
        let _guard = TEST_MUTEX.lock().await;

        // Configuration de test
        let config = Config::test_config();

        // Créer une base de données de test
        let db = create_test_db("test_submit_contact")
            .await
            .expect("Failed to create test database");
        println!("Base de test créée");

        // Vider les collections avant le test
        clean_collections(&db, "test_submit_contact")
            .await
            .expect("Failed to clean test database");
        println!("Collections nettoyées");

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
        println!("Mock server configuré");

        // Créer le service avec l'URL du mock server
        let mut config = config;
        config.brevo_api_key = String::from("test_key");
        let mut service = MessageService::new(db.clone(), config.clone())
            .with_test_collections("test_submit_contact");
        #[cfg(test)]
        {
            service.brevo_url = mock_server.uri();
        }

        // Test avec un formulaire valide
        let form = Request {
            name: String::from("Test User"),
            email: String::from("test@example.com"),
            message: String::from("This is a test message"),
        };

        // Soumettre le formulaire
        println!("Soumission du formulaire...");
        let result = service.submit_contact(form).await;
        assert!(result.is_ok(), "Failed to submit contact form: {result:?}");
        println!("Formulaire soumis avec succès");

        // Attendre un peu pour s'assurer que l'insertion est terminée
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Vérifier l'enregistrement en base
        let collection = db.collection::<mongodb::bson::Document>("contacts_test_submit_contact");
        println!("Recherche du document dans la base...");
        let doc = collection
            .find_one(doc! { "email": "test@example.com" })
            .await
            .expect("Failed to query database")
            .expect("Document not found");
        println!("Document trouvé : {:?}", doc);

        assert_eq!(doc.get_str("name").unwrap(), "Test User");
        assert_eq!(doc.get_str("email").unwrap(), "test@example.com");
        assert_eq!(doc.get_str("message").unwrap(), "This is a test message");

        // Nettoyer la base de test
        clean_collections(&db, "test_submit_contact")
            .await
            .expect("Failed to cleanup test database");
        println!("Test terminé avec succès");
    }

    #[test]
    fn test_contact_form_validation() {
        // Test avec des données valides
        let valid_form = Request {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            message: "This is a valid test message".to_string(),
        };
        assert!(valid_form.validate().is_ok());

        // Test avec un email invalide
        let invalid_email = Request {
            name: "Test User".to_string(),
            email: "invalid-email".to_string(),
            message: "This is a test message".to_string(),
        };
        assert!(invalid_email.validate().is_err());

        // Test avec un message trop court
        let short_message = Request {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            message: "Short".to_string(),
        };
        assert!(short_message.validate().is_err());
    }
}

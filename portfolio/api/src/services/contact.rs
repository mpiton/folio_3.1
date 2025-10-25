use crate::config::Config;
use crate::models::contact::Request;
use anyhow::Result;
use futures_util::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::Database;
use validator::Validate;

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
        form.validate()
            .map_err(|e| anyhow::anyhow!("Validation error: {}", e))?;

        // Stocker dans la base de données
        // Use is_test field to determine collection name (more reliable than cfg(test))
        let collection_name = if form.is_test {
            "contacts_test_submit_contact"
        } else {
            "contacts"
        };

        let collection = self.db.collection::<Document>(collection_name);
        println!("Insertion du document dans la collection {collection_name}...");

        let doc = doc! {
            "name": form.name.clone(),
            "email": form.email.clone(),
            "subject": form.subject.clone(),
            "message": form.message.clone(),
            "created_at": mongodb::bson::DateTime::now()
        };
        println!("Document à insérer : {doc:?}");

        let result = collection.insert_one(doc).await?;
        println!("Document inséré avec l'ID : {:?}", result.inserted_id);

        // Ne pas envoyer d'email si c'est un test
        #[cfg(not(test))]
        if !form.is_test && self.config.brevo_api_key != "test_key" {
            // Envoyer l'email
            self.send_email(&form).await?;
        }

        #[cfg(test)]
        if !form.is_test {
            // Envoyer l'email
            self.send_email(&form).await?;
        }

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
                subject: doc
                    .get_str("subject")
                    .map_err(|e| anyhow::anyhow!(e))?
                    .to_string(),
                message: doc
                    .get_str("message")
                    .map_err(|e| anyhow::anyhow!(e))?
                    .to_string(),
                is_test: doc.get_bool("is_test").unwrap_or(false),
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

use anyhow::Result;
use async_channel::{bounded, Receiver, Sender};
use lazy_static::lazy_static;
use std::sync::Once;

static INIT: Once = Once::new();
const QUEUE_SIZE: usize = 100;

lazy_static! {
    static ref EMAIL_QUEUE: (Sender<EmailMessage>, Receiver<EmailMessage>) = bounded(QUEUE_SIZE);
}

#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub html_content: String,
}

pub async fn enqueue_email(message: EmailMessage) -> Result<()> {
    let sender = &EMAIL_QUEUE.0;
    sender.send(message).await?;
    Ok(())
}

pub async fn start_email_processor() {
    INIT.call_once(|| {
        tokio::spawn(async {
            let receiver = &EMAIL_QUEUE.1;

            while let Ok(message) = receiver.recv().await {
                if let Err(e) = process_email(message).await {
                    log::error!("Erreur lors du traitement de l'email : {}", e);
                }
            }
        });
    });
}

async fn process_email(message: EmailMessage) -> Result<()> {
    // TODO: Implémenter l'envoi d'email via Brevo
    // Pour l'instant, on log juste le message
    log::info!(
        "Email envoyé à {} avec le sujet : {}",
        message.to,
        message.subject
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_queue() {
        // Démarrer le processeur d'emails
        start_email_processor().await;

        // Créer un message de test
        let message = EmailMessage {
            to: "test@example.com".to_string(),
            subject: "Test Subject".to_string(),
            html_content: "<p>Test content</p>".to_string(),
        };

        // Envoyer le message
        let result = enqueue_email(message).await;
        assert!(result.is_ok());
    }
}

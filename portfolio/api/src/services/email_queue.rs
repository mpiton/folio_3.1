use anyhow::Result;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

static EMAIL_QUEUE: OnceCell<Arc<EmailQueue>> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

struct EmailQueue {
    sender: Sender<EmailMessage>,
    receiver: Mutex<Option<Receiver<EmailMessage>>>,
}

/// Initialise la file d'attente des emails.
///
/// Cette fonction est thread-safe et ne peut être appelée qu'une seule fois.
/// Les appels suivants n'auront aucun effet.
pub fn init_queue() {
    EMAIL_QUEUE.get_or_init(|| {
        let (sender, receiver) = mpsc::channel(100);
        Arc::new(EmailQueue {
            sender,
            receiver: Mutex::new(Some(receiver)),
        })
    });
}

/// Ajoute un email à la file d'attente pour envoi.
///
/// # Errors
///
/// Cette fonction retourne une erreur si :
/// - La file d'attente n'est pas initialisée
/// - La file d'attente est pleine
/// - Le canal de communication est fermé
pub async fn enqueue_email(message: EmailMessage) -> Result<()> {
    let queue = EMAIL_QUEUE
        .get()
        .ok_or_else(|| anyhow::anyhow!("La file d'attente n'est pas initialisée"))?;
    queue.sender.send(message).await?;
    Ok(())
}

/// Démarre le processeur d'emails en arrière-plan.
/// Cette fonction est thread-safe et peut être appelée plusieurs fois.
pub async fn start_email_processor() {
    let queue = match EMAIL_QUEUE.get() {
        Some(queue) => queue,
        None => return,
    };

    if let Some(mut receiver) = queue.receiver.lock().await.take() {
        while let Some(message) = receiver.recv().await {
            process_email(&message);
        }
    }
}

/// Traite un email de la file d'attente.
fn process_email(message: &EmailMessage) {
    println!(
        "Email envoyé à {} avec le sujet: {}",
        message.to, message.subject
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_queue() {
        // Initialiser la file d'attente
        init_queue();

        // Créer un message de test
        let message = EmailMessage {
            to: "test@example.com".to_string(),
            subject: "Test Subject".to_string(),
            body: "<p>Test content</p>".to_string(),
        };

        // Envoyer le message
        let result = enqueue_email(message).await;
        assert!(result.is_ok());

        // Démarrer le processeur d'emails dans un nouveau thread
        let handle = tokio::spawn(async {
            start_email_processor().await;
        });

        // Attendre que le processeur termine
        handle.await.unwrap();
    }
}

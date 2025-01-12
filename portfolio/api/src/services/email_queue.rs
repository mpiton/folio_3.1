use anyhow::Result;
use once_cell::sync::OnceCell;
use tokio::sync::mpsc;

static INIT: OnceCell<()> = OnceCell::new();
static mut EMAIL_SENDER: Option<mpsc::Sender<EmailMessage>> = None;
static mut EMAIL_RECEIVER: Option<mpsc::Receiver<EmailMessage>> = None;

#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub body: String,
}

/// Initialise la file d'attente des emails.
///
/// # Safety
///
/// Cette fonction doit être appelée une seule fois au démarrage de l'application.
pub fn init_queue() {
    let (sender, receiver) = mpsc::channel(100);
    unsafe {
        EMAIL_SENDER = Some(sender);
        EMAIL_RECEIVER = Some(receiver);
    }
}

/// Ajoute un email à la file d'attente pour envoi.
///
/// # Errors
///
/// Cette fonction retourne une erreur si :
/// - La file d'attente est pleine
/// - Le canal de communication est fermé
pub async fn enqueue_email(message: EmailMessage) -> Result<()> {
    let sender = unsafe { &EMAIL_SENDER };
    if let Some(sender) = sender {
        sender.send(message).await?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("La file d'attente n'est pas initialisée"))
    }
}

pub fn start_email_processor() {
    INIT.get_or_init(|| {
        tokio::spawn(async {
            if let Some(mut receiver) = unsafe { EMAIL_RECEIVER.take() } {
                while let Some(message) = receiver.recv().await {
                    process_email(&message);
                }
            }
        });
    });
}

/// Traite un email de la file d'attente.
fn process_email(message: &EmailMessage) {
    // TODO: Implémenter l'envoi réel des emails
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

        // Démarrer le processeur d'emails
        start_email_processor();

        // Créer un message de test
        let message = EmailMessage {
            to: "test@example.com".to_string(),
            subject: "Test Subject".to_string(),
            body: "<p>Test content</p>".to_string(),
        };

        // Envoyer le message
        let result = enqueue_email(message).await;
        assert!(result.is_ok());
    }
}

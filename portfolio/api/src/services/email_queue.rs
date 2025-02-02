use anyhow::Result;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

static EMAIL_QUEUE: OnceCell<Arc<EmailQueue>> = OnceCell::new();

/// Thread-safe email queue for asynchronous message processing
///
/// # Features
/// - Buffered channel with 100 message capacity
/// - Background processor thread
/// - Test utilities for isolated testing
///
///   Email message structure for queue items
#[derive(Debug, Clone)]
pub struct EmailMessage {
    /// Recipient email address
    pub to: String,
    /// Email subject line
    pub subject: String,
    /// HTML email body content
    pub body: String,
}

struct EmailQueue {
    sender: Sender<EmailMessage>,
    receiver: Mutex<Option<Receiver<EmailMessage>>>,
}

/// Initializes email queue
///
/// Thread-safe - can only be called once
/// Subsequent calls have no effect.
pub fn init_queue() {
    EMAIL_QUEUE.get_or_init(|| {
        let (sender, receiver) = mpsc::channel(100);
        Arc::new(EmailQueue {
            sender,
            receiver: Mutex::new(Some(receiver)),
        })
    });
}

/// Enqueues email for background processing
///
/// # Errors
/// Returns error if:
/// - Queue not initialized
/// - Channel is closed
pub async fn enqueue_email(message: EmailMessage) -> Result<()> {
    let queue = EMAIL_QUEUE
        .get()
        .ok_or_else(|| anyhow::anyhow!("Queue not initialized"))?;
    queue.sender.send(message).await?;
    Ok(())
}

/// Starts background email processing task
///
/// # Behavior
/// - Claims exclusive access to queue receiver
/// - Processes messages until channel closes
/// - Runs indefinitely in tokio task
///
/// # Safety
/// Should only be called once during application startup
pub async fn start_email_processor() {
    let Some(queue) = EMAIL_QUEUE.get() else {
        return;
    };

    if let Some(mut receiver) = queue.receiver.lock().await.take() {
        while let Some(message) = receiver.recv().await {
            process_email(&message);
        }
    }
}

/// Processes email from queue
fn process_email(message: &EmailMessage) {
    println!(
        "Email sent to {} with subject: {}",
        message.to, message.subject
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_email_queue() {
        // Initialize queue
        init_queue();

        // Create test message
        let message = EmailMessage {
            to: "test@example.com".to_string(),
            subject: "Test Subject".to_string(),
            body: "<p>Test content</p>".to_string(),
        };

        // Send message
        let result = enqueue_email(message).await;
        assert!(result.is_ok());

        // Retrieve message directly from queue
        if let Some(queue) = EMAIL_QUEUE.get() {
            if let Some(mut receiver) = queue.receiver.lock().await.take() {
                if let Some(received_message) = receiver.recv().await {
                    process_email(&received_message);
                    assert_eq!(received_message.to, "test@example.com");
                    assert_eq!(received_message.subject, "Test Subject");
                }
            }
        }
    }
}

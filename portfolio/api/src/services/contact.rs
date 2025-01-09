use crate::models::contact::ContactForm;
use anyhow::{Result, anyhow};
use serde_json::json;
use wiremock::matchers::{method, path};

// Constants for validation
const MAX_NAME_LENGTH: usize = 100;
const MAX_EMAIL_LENGTH: usize = 255;
const MAX_SUBJECT_LENGTH: usize = 200;
const MAX_MESSAGE_LENGTH: usize = 1000;

pub async fn send_contact_email(form: &ContactForm) -> Result<()> {
    // Validate input
    if form.name.len() > MAX_NAME_LENGTH {
        return Err(anyhow!("Name too long"));
    }
    if form.email.len() > MAX_EMAIL_LENGTH {
        return Err(anyhow!("Email too long"));
    }
    if !form.email.contains('@') {
        return Err(anyhow!("Invalid email format"));
    }
    if form.subject.len() > MAX_SUBJECT_LENGTH {
        return Err(anyhow!("Subject too long"));
    }
    if form.message.len() > MAX_MESSAGE_LENGTH {
        return Err(anyhow!("Message too long"));
    }

    // Get Brevo API configuration from environment variables
    let api_key = std::env::var("BREVO_API_KEY")
        .map_err(|_| anyhow!("BREVO_API_KEY not set"))?;
    let recipient_email = std::env::var("RECIPIENT_EMAIL")
        .map_err(|_| anyhow!("RECIPIENT_EMAIL not set"))?;
    let sender_name = std::env::var("SENDER_NAME")
        .map_err(|_| anyhow!("SENDER_NAME not set"))?;
    let sender_email = std::env::var("SENDER_EMAIL")
        .map_err(|_| anyhow!("SENDER_EMAIL not set"))?;
    let api_url = std::env::var("BREVO_API_URL")
        .unwrap_or_else(|_| String::from("https://api.brevo.com/v3/smtp/email"));

    // Create email payload
    let payload = json!({
        "sender": {
            "name": &sender_name,
            "email": &sender_email
        },
        "to": [{
            "email": &recipient_email,
            "name": "Mathieu Piton"
        }],
        "replyTo": {
            "email": &form.email,
            "name": &form.name
        },
        "subject": &form.subject,
        "htmlContent": format!(
            "<p><strong>New contact message from portfolio</strong></p>
            <p><strong>Name:</strong> {}</p>
            <p><strong>Email:</strong> {}</p>
            <p><strong>Message:</strong></p>
            <p>{}</p>",
            &form.name,
            &form.email,
            &form.message.replace("\n", "<br>")
        )
    });

    // Send request to Brevo API
    let client = reqwest::Client::new();
    let response = client
        .post(api_url)
        .header("api-key", api_key)
        .header("accept", "application/json")
        .json(&payload)
        .send()
        .await?;

    let status = response.status();

    let response_text = response.text().await?;

    if !status.is_success() {
        // Try to parse the error response as JSON
        if let Ok(error) = serde_json::from_str::<serde_json::Value>(&response_text) {
            if let Some(message) = error.get("message") {
                return Err(anyhow!("Failed to send email: {}", message));
            }
        }
        // Fallback to raw response text if not JSON
        return Err(anyhow!("Failed to send email: {}", response_text));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use wiremock::{MockServer, Mock, ResponseTemplate};

    async fn setup_test_env(mock_url: &str) {
        env::set_var("BREVO_API_KEY", "test-api-key");
        env::set_var("RECIPIENT_EMAIL", "recipient@example.com");
        env::set_var("SENDER_NAME", "Test Sender");
        env::set_var("SENDER_EMAIL", "sender@example.com");
        env::set_var("BREVO_API_URL", mock_url);
    }

    #[tokio::test]
    async fn test_successful_email_send() {
        let mock_server = MockServer::start().await;
        setup_test_env(&mock_server.uri()).await;

        let success_response = json!({
            "messageId": "test-message-id",
            "message": "Email sent successfully"
        });

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(&success_response))
            .expect(1)
            .named("successful_email_send")
            .mount(&mock_server)
            .await;

        let contact_form = ContactForm {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "Test Message".to_string(),
        };

        let result = send_contact_email(&contact_form).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_contact_email_validation() {
        let mock_server = MockServer::start().await;
        setup_test_env(&mock_server.uri()).await;

        // Test invalid email
        let contact_form = ContactForm {
            name: "Test User".to_string(),
            email: "invalid-email".to_string(),
            subject: "Test Subject".to_string(),
            message: "Test Message".to_string(),
        };

        let result = send_contact_email(&contact_form).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid email"));

        // Test too long name
        let contact_form = ContactForm {
            name: "a".repeat(MAX_NAME_LENGTH + 1),
            email: "test@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "Test Message".to_string(),
        };

        let result = send_contact_email(&contact_form).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Name too long"));
    }

    #[tokio::test]
    async fn test_api_error_handling() {
        let mock_server = MockServer::start().await;
        setup_test_env(&mock_server.uri()).await;

        let error_response = json!({
            "code": "invalid_api_key",
            "message": "Invalid API key"
        });

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(400)
                .set_body_json(&error_response))
            .expect(1)
            .named("error_handling")
            .mount(&mock_server)
            .await;

        let contact_form = ContactForm {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "Test Message".to_string(),
        };

        let result = send_contact_email(&contact_form).await;
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("Invalid API key") ||
            err.contains("invalid_api_key"),
            "Unexpected error message: {}",
            err
        );
    }
}

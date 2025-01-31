use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Request {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Le nom doit faire entre 2 et 100 caractères"
    ))]
    #[validate(custom(
        function = "validate_name",
        message = "Le nom contient des caractères invalides"
    ))]
    pub name: String,

    #[validate(email(message = "L'email n'est pas valide"))]
    #[validate(length(max = 100, message = "L'email est trop long"))]
    pub email: String,

    #[validate(length(
        min = 2,
        max = 100,
        message = "Le sujet doit faire entre 2 et 100 caractères"
    ))]
    #[validate(custom(
        function = "validate_text",
        message = "Le sujet contient des caractères non autorisés"
    ))]
    pub subject: String,

    #[validate(length(
        min = 10,
        max = 1000,
        message = "Le message doit faire entre 10 et 1000 caractères"
    ))]
    #[validate(custom(
        function = "validate_message_content",
        message = "Le message contient du contenu non autorisé"
    ))]
    pub message: String,

    #[serde(default)]
    pub is_test: bool,
}

lazy_static::lazy_static! {
    static ref NAME_RE: Regex = Regex::new(r"^[\p{L}\s\-']+$").unwrap();
    static ref SAFE_TEXT_REGEX: Regex = Regex::new(r"^[\p{L}\p{N}\s.,!?@()'\[\]\-_&+=%°:;]+$").unwrap();
}

fn validate_name(name: &str) -> Result<(), validator::ValidationError> {
    if NAME_RE.is_match(name) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("caractères_non_autorisés"))
    }
}

fn validate_text(text: &str) -> Result<(), validator::ValidationError> {
    if SAFE_TEXT_REGEX.is_match(text) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("caractères_non_autorisés"))
    }
}

fn validate_message_content(message: &str) -> Result<(), validator::ValidationError> {
    // Vérifier les caractères spéciaux dangereux
    if message.contains('<') || message.contains('>') {
        return Err(validator::ValidationError::new("caractères_html_interdits"));
    }

    // Vérifier les liens suspects
    if message.matches("http").count() > 3 {
        return Err(validator::ValidationError::new("trop_de_liens"));
    }

    // Vérifier la répétition de caractères
    let char_counts: std::collections::HashMap<char, usize> =
        message
            .chars()
            .fold(std::collections::HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            });

    if char_counts.values().any(|&count| count > 50) {
        return Err(validator::ValidationError::new("repetition_caracteres"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_valid_request() {
        let request = Request {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "This is a valid message with good content.".to_string(),
            is_test: false,
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_invalid_name() {
        let request = Request {
            name: "John123".to_string(),
            email: "john@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "Valid message".to_string(),
            is_test: false,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_invalid_email() {
        let request = Request {
            name: "John".to_string(),
            email: "not-an-email".to_string(),
            subject: "Test Subject".to_string(),
            message: "Valid message".to_string(),
            is_test: false,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_invalid_subject() {
        let request = Request {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            subject: "Test <script>alert('xss')</script>".to_string(),
            message: "Valid message".to_string(),
            is_test: false,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_message_with_html() {
        let request = Request {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "Message with <script>alert('xss')</script>".to_string(),
            is_test: false,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_message_with_too_many_links() {
        let request = Request {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "http://spam1.com http://spam2.com http://spam3.com http://spam4.com"
                .to_string(),
            is_test: false,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_message_with_char_repetition() {
        let request = Request {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "a".repeat(51),
            is_test: false,
        };
        assert!(request.validate().is_err());
    }
}

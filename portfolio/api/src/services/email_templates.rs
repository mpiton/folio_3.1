use anyhow::Result;
use handlebars::Handlebars;
use once_cell::sync::Lazy;
use serde_json::json;

static TEMPLATES: Lazy<Handlebars<'static>> = Lazy::new(|| {
    let mut reg = Handlebars::new();
    reg.register_template_string(
        "contact",
        r"
        <h2>New contact message</h2>
        <p><strong>From:</strong> {{name}} ({{email}})</p>
        <p><strong>Subject:</strong> {{subject}}</p>
        <hr>
        <p><strong>Message:</strong></p>
        <p>{{message}}</p>
        ",
    )
    .expect("Failed to register contact template");
    reg
});

/// Generates HTML email content from contact template.
///
/// # Errors
///
/// Returns error if:
/// - Template not found
/// - Template rendering fails
/// - Invalid template data
pub fn render_contact_template(
    name: &str,
    email: &str,
    subject: &str,
    message: &str,
) -> Result<String> {
    let data = json!({
        "name": name,
        "email": email,
        "subject": subject,
        "message": message,
    });

    Ok(TEMPLATES.render("contact", &data)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_contact_template() {
        let html = render_contact_template(
            "John Doe",
            "john@example.com",
            "Test Subject",
            "Test message",
        )
        .unwrap();

        assert!(html.contains("John Doe"));
        assert!(html.contains("john@example.com"));
        assert!(html.contains("Test Subject"));
        assert!(html.contains("Test message"));
    }
}

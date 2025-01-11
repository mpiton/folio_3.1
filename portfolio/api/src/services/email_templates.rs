use anyhow::Result;
use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde_json::json;

lazy_static! {
    static ref TEMPLATES: Handlebars<'static> = {
        let mut hb = Handlebars::new();

        // Template pour le formulaire de contact
        hb.register_template_string(
            "contact",
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>Nouveau message de contact</title>
            </head>
            <body>
                <h2>Nouveau message de contact</h2>
                <p><strong>De :</strong> {{name}} ({{email}})</p>
                <p><strong>Sujet :</strong> {{subject}}</p>
                <div><strong>Message :</strong></div>
                <div style="margin: 10px 0; padding: 10px; background: #f5f5f5; border-radius: 4px;">
                    {{message}}
                </div>
            </body>
            </html>
            "#,
        )
        .unwrap();

        hb
    };
}

pub fn render_contact_template(
    name: &str,
    email: &str,
    subject: &str,
    message: &str,
) -> Result<String> {
    TEMPLATES
        .render(
            "contact",
            &json!({
                "name": name,
                "email": email,
                "subject": subject,
                "message": message
            }),
        )
        .map_err(|e| anyhow::anyhow!("Failed to render template: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_contact_template() {
        let result = render_contact_template(
            "Test User",
            "test@example.com",
            "Test Subject",
            "This is a test message",
        );

        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("Test User"));
        assert!(html.contains("test@example.com"));
        assert!(html.contains("Test Subject"));
        assert!(html.contains("This is a test message"));
    }
}

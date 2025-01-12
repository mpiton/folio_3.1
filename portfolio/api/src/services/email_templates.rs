use anyhow::Result;
use handlebars::Handlebars;
use once_cell::sync::Lazy;
use serde_json::json;

static TEMPLATES: Lazy<Handlebars<'static>> = Lazy::new(|| {
    let mut reg = Handlebars::new();
    reg.register_template_string(
        "contact",
        r"
        <h2>Nouveau message de contact</h2>
        <p><strong>De:</strong> {{name}} ({{email}})</p>
        <p><strong>Sujet:</strong> {{subject}}</p>
        <hr>
        <p><strong>Message:</strong></p>
        <p>{{message}}</p>
        ",
    )
    .expect("Failed to register contact template");
    reg
});

/// Génère le contenu HTML d'un email à partir du template de contact.
///
/// # Errors
///
/// Cette fonction retourne une erreur si :
/// - Le template n'est pas trouvé
/// - Le rendu du template échoue
/// - Les données fournies sont invalides pour le template
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

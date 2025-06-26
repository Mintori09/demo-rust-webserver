use lettre::{
    Message, SmtpTransport, Transport,
    message::{SinglePart, header},
    transport::smtp::authentication::Credentials,
};
use tokio::fs;

use crate::config::email::MySMTP;

pub async fn send_email(
    to_email: &str,
    subject: &str,
    template_path: &str,
    placeholders: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_env = MySMTP::init();

    let mut html_template: String = fs::read_to_string(template_path).await?;

    for (key, value) in placeholders {
        html_template = html_template.replace(key, value);
    }

    let email = Message::builder()
        .from(smtp_env.username.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html_template),
        )?;
    let creds = Credentials::new(smtp_env.username, smtp_env.password);
    let mailer = SmtpTransport::starttls_relay(&smtp_env.server)?
        .credentials(creds)
        .port(smtp_env.port)
        .build();
    let result = mailer.send(&email);

    match result {
        Ok(_) => println!("Email sent succcessfully!"),
        Err(e) => println!("Failed to send email: {:?}", e),
    }

    Ok(())
}

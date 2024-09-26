use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::env;

// Email confirmation function
pub async fn send_confirmation_email(email: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;
    let smtp_host = env::var("SMTP_HOST")?;
    let smtp_port = env::var("SMTP_PORT")?.parse::<u16>()?;
    
    let creds = Credentials::new(smtp_username.clone(), smtp_password);
    let mailer = SmtpTransport::relay(&smtp_host)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    let email = Message::builder()
        .from(smtp_username.parse()?)
        .to(email.parse()?)
        .subject("Registration Confirmation")
        .body("Thank you for registering!".to_string())
        .unwrap();

    mailer.send(&email)?;
    Ok(())
}
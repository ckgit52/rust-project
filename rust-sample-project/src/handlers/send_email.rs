use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::env;

// Email confirmation function using Gmail SMTP with TLS settings
pub async fn send_confirmation_email(email: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;
    let smtp_host = "smtp.gmail.com"; // Gmail SMTP server
    let smtp_port = 587; // Port for TLS

    // Set up credentials using Gmail SMTP details
    let creds = Credentials::new(smtp_username.clone(), smtp_password);

    // Set up the mailer with TLS settings
    let mailer = SmtpTransport::relay(smtp_host)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    // Create the email message
    let email = Message::builder()
        .from(smtp_username.parse()?) // Your Gmail address as the sender
        .to(email.parse()?)
        .subject("Registration Confirmation")
        .body("Thank you for registering! Please confirm your email by clicking the link.".to_string())
        .unwrap();

    // Send the email
    mailer.send(&email)?;

    // Extract and print the recipient email
    if let Some(to_address) = email.envelope().to().first() {
        println!("Confirmation email sent to: {}", to_address);
    }

    Ok(())
}

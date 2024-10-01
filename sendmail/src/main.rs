use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the App Password here instead of your Gmail password
    let smtp_credentials =
        Credentials::new("chandan52chd@gmail.com".to_string(), "rustpassword".to_string());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(smtp_credentials)
        .port(587) // Specify the correct port for TLS
        .build();

    let from = "chandan52chd@gmail.com";
    let to = "chandan1012chd@gmail.com";
    let subject = "Hello World";
    let body = "<h1>Hello World</h1>".to_string();

    if let Err(e) = send_email_smtp(&mailer, from, to, subject, body).await {
        eprintln!("Error: {:?}", e); // Log the error for debugging
    }

    Ok(())
}

async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(body)?;

    mailer.send(email).await?;

    println!("Email sent successfully!");
    Ok(())
}

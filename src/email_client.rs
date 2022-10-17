use lettre::transport::smtp::authentication::Credentials;
use lettre::{message::header, Message, SmtpTransport, Transport};

pub fn send_mail(email: String, name: String) {
    let email = Message::builder()
        .from("Lemon <devlemon@mail.com>".parse().unwrap())
        .reply_to("Lemon <devlemon@mail.com>".parse().unwrap())
        .to(format!("{name} <{email}>").parse().unwrap())
        .subject("Account Verification")
        .header(header::ContentType::TEXT_HTML)
        .body("<h1>This is a html mail</h1><p>hey from rust<p>".to_string())
        .unwrap();

    let creds = Credentials::new("apikey".to_string(), "xxxxxxxxxxxxx".to_string());

    let mailer = SmtpTransport::relay("smtp.sendgrid.net")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email was send successfully!"),
        Err(e) => println!("Could not send the email: {:#?}", e),
    }
}

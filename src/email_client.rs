use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde_json::json;

use crate::domain::SubscriberEmail;

// #[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    sender: SubscriberEmail,
    base_url: String,
    authorization_token: Secret<String>,
}

struct SendEmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        authorization_token: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/v3/mail/send", self.base_url);
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().to_owned(),
            to: recipient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text_body: text_content.to_owned(),
        };

        let request_body_json = json!({
            "personalizations": [{
              "to": [{"email": format!("{}",request_body.to)}]
                             }],
                             "from": {
                                 "email": format!("{}", request_body.from)
                             },
                             "subject": format!("{}", request_body.subject),
                 "content": [{"type": "text/plain", "value": format!("{}", request_body.text_body)}]
        });

        self.http_client
            .post(&url)
            .header("Authorization", self.authorization_token.expose_secret())
            .json(&request_body_json)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

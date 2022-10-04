use reqwest::Client;
use secrecy::Secret;
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
    ) -> Self {
        Self {
            http_client: Client::new(),
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
    ) -> Result<(), String> {
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

        let builder = self.http_client.post(&url).json(&request_body_json);

        Ok(())
    }
}

/*
curl --request POST \
          --url https://api.sendgrid.com/v3/mail/send \
          --header "Authorization: Bearer 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" \
          --header 'Content-Type: application/json' \
          --data '{"personalizations": [{"to": [{"email": "snyxmk@gmail.com"}]}],"from": {"email": "devlemon@mail.com"},"subject": "Sending with SendGrid is Fun","content": [{"type": "text/plain", "value": "and easy to do anywhere, even with cURL"}]}'
*/

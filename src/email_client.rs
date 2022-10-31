use reqwest::Client;
use secrecy::{ExposeSecret, Secret};
use serde_json::json;

use crate::domain::SubscriberEmail;

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    auth_token: Secret<String>,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        auth_token: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();

        Self {
            http_client,
            base_url,
            sender,
            auth_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/v3/mail/send", self.base_url);

        let body = json!({
            "personalizations" : [
                {
                    "to" : [
                        {
                            "email": recipient.as_ref()
                        }
                    ]
                }
            ],
            "from": {
                "email": self.sender.as_ref()
            },
            "subject": subject,
            "content": [
                {
                    "type": "text/html",
                    "value": html_content
                }
            ]
        });

        self.http_client
            .post(&url)
            .bearer_auth(self.auth_token.expose_secret())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

// #[cfg(test)]
// mod test {
// TODO implement wiremock testing

//     use crate::domain::SubscriberEmail;
//     use crate::email_client::EmailClient;
//     use fake::faker::internet::en::SafeEmail;
//     use fake::faker::lorem::en::{Paragraph, Sentence};
//     use fake::{Fake, Faker};
//     use secrecy::Secret;
//     use wiremock::matchers::{header, header_exists, method, path};
//     use wiremock::Request;
//     use wiremock::{Mock, MockServer, ResponseTemplate};

//     struct SendEmailBodyMatcher;

//     impl wiremock::Match for SendEmailBodyMatcher {
//         fn matches(&self, request: &Request) -> bool {
//             unimplemented!()
//         }
//     }

//     #[tokio::test]
//     async fn send_email_fires_a_request_to_base_url() {
//         // Arrange
//         let mock_server = MockServer::start().await;
//         let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
//         let email_client = EmailClient::new(mock_server.uri(), sender, Secret::new(Faker.fake()));

//         Mock::given(header_exists("Authorization"))
//             .and(header("Content-Type", "application/json"))
//             .and(path("/v3/mail/send"))
//             .and(method("POST"))
//             .respond_with(ResponseTemplate::new(200))
//             .expect(1)
//             .mount(&mock_server)
//             .await;

//         let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
//         let subject: String = Sentence(1..2).fake();
//         let content: String = Paragraph(1..10).fake();
//         // Act

//         let _ = email_client
//             .send_email(subscriber_email, &subject, &content, &content)
//             .await;
//         // Assert
//     }
// }

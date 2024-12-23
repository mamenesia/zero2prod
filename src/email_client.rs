use crate::domain::SubscriberEmail;
use reqwest::Client;

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }

    pub fn send_email(
        &self,
        _recipient: &SubscriberEmail,
        _subject: &str,
        _html_body: &str,
        _text_body: &str,
    ) {
        // TODO: send an email to the recipient
    }
}

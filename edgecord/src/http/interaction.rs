use twilight_model::channel::{Message};
use worker::Method;
use crate::http::{HttpClient, Routes};
use super::super::model::rest::webhook::WebhookBody;


impl HttpClient {
    pub async fn create_followup_message(&self, interaction_token: String, body: WebhookBody) -> crate::Result<Message> {
        self.request::<WebhookBody, Message>(
            Method::Post,
            Routes::ApplicationInteraction(self.application_id, interaction_token),
            Some(body)
        ).await.map(|x| x.unwrap())
    }
}

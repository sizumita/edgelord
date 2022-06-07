use super::super::model::rest::webhook::WebhookBody;
use crate::http::{HttpClient, Routes};
use twilight_model::channel::Message;
use worker::Method;

impl HttpClient {
    pub async fn create_followup_message(
        &self,
        interaction_token: String,
        body: WebhookBody,
    ) -> crate::Result<Message> {
        self.request::<WebhookBody, Message>(
            Method::Post,
            Routes::ApplicationInteraction(self.application_id, interaction_token),
            Some(body),
        )
        .await
        .map(|x| x.unwrap())
    }
}

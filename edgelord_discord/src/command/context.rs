use twilight_model::channel::message::MessageFlags;
use crate::InteractionResponse;
use twilight_model::http::interaction::{InteractionResponseType, InteractionResponseData};

#[derive(Clone)]
pub struct Context {

}

impl Context {
    pub fn get_option<T>(&self, name: &str) -> T {
        todo!()
    }

    pub fn message(&self, message: &str) -> InteractionResponse {
        worker::Response::from_json(&twilight_model::http::interaction::InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: Some(InteractionResponseData {
                allowed_mentions: None,
                attachments: None,
                choices: None,
                components: None,
                content: Some(message.to_string()),
                custom_id: None,
                embeds: None,
                flags: Some(MessageFlags::EPHEMERAL),
                title: None,
                tts: None
            })
        })
    }
}

use crate::application_command::i18n::Locales;
use crate::application_command::FromCommandOptionValue;
use crate::http::HttpClient;
use crate::InteractionResponse;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_model::channel::message::MessageFlags;
use twilight_model::http::interaction::{InteractionResponseData, InteractionResponseType};
use worker::Env;

/**
Context for ChatInput Command.
**/
pub struct ChatInputCommandContext {
    pub interaction: Box<ApplicationCommand>,
    pub locale: Locales,
    pub env: Env,
    pub ctx: worker::Context,
    pub http: HttpClient,
}

impl ChatInputCommandContext {
    pub fn new(
        interaction: Box<ApplicationCommand>,
        env: Env,
        ctx: worker::Context,
        http: HttpClient,
    ) -> Self {
        Self {
            interaction: interaction.clone(),
            locale: serde_json::from_str::<Locales>(&*interaction.locale).unwrap_or(Locales::EnUS),
            env,
            ctx,
            http,
        }
    }

    pub fn get_option<T>(interaction: Box<ApplicationCommand>, name: &str) -> T
    where
        T: FromCommandOptionValue,
    {
        T::from_option(
            interaction
                .data
                .options
                .iter()
                .find(|x| x.name == *name)
                .unwrap()
                .clone()
                .value,
        )
        .unwrap()
    }

    pub fn message(&self, message: &str) -> InteractionResponse {
        worker::Response::from_json(&twilight_model::http::interaction::InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
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
                tts: None,
            }),
        })
    }
}
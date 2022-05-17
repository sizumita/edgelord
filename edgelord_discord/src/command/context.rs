use std::sync::Arc;
use twilight_model::application::interaction::{ApplicationCommand};
use twilight_model::application::interaction::application_command::CommandOptionValue;
use twilight_model::channel::message::MessageFlags;
use crate::{Command, InteractionResponse};
use twilight_model::http::interaction::{InteractionResponseType, InteractionResponseData};
use worker::{Context, Env};
use crate::i18n::Locales;


pub struct ChatInputCommandContext {
    pub interaction: Box<ApplicationCommand>,
    pub locale: Locales,
    pub env: Env,
    ctx: Arc<worker::Context>,
}

impl ChatInputCommandContext {
    pub fn new(interaction: Box<ApplicationCommand>, env: Env, ctx: worker::Context) -> Self {
        Self {
            interaction,
            locale: serde_json::from_str::<Locales>(&*interaction.locale).unwrap_or(Locales::EnUS),
            env,
            ctx: Arc::new(ctx),
        }
    }

    pub fn get_option<T: std::convert::From<String>>(interaction: Box<ApplicationCommand>, name: &str) -> T {
        match interaction.data.options.iter().find(|x| x.name == name.to_string()).unwrap().clone().value {
            CommandOptionValue::String(value) => value.into(),
            _ => unreachable!()
        }
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
                tts: None
            })
        })
    }
}

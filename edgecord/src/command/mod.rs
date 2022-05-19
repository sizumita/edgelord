mod choice;
mod context;
pub mod i18n;
pub mod option;
pub mod upload;

use futures::future::LocalBoxFuture;
use std::collections::HashMap;
use std::rc::Rc;
use twilight_model::application::interaction::ApplicationCommand;
use serde::{Serialize, Deserialize};
use twilight_model::application::command::{CommandOptionType, CommandType};

use crate::InteractionResponse;
pub use choice::*;
pub use context::*;

type I18nMap = Option<HashMap<i18n::Locales, String>>;
type AsyncCommandFn<'a> = Rc<
    dyn 'a
        + Fn(
            ChatInputCommandContext,
            Box<ApplicationCommand>,
        ) -> LocalBoxFuture<'a, worker::Result<worker::Response>>,
>;

fn serialize_permissions<S: serde::Serializer>(value: &Option<u64>, s: S) -> Result<S::Ok, S::Error> {
    if let Some(value) = value {
        s.serialize_u64(*value)
    } else {
        s.serialize_none()
    }
}

/**
Discord Chat Input Command Structure.
 **/
#[derive(Clone, Serialize)]
pub struct Command<'a> {
    #[serde(rename = "type")]
    pub command_type: CommandType,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "name_localizations")]
    pub i18n_names: I18nMap,
    #[serde(skip_serializing_if = "Option::is_none", rename = "description_localizations")]
    pub i18n_descriptions: I18nMap,
    #[serde(skip_serializing_if = "Option::is_none", serialize_with = "serialize_permissions")]
    pub default_permissions: Option<u64>,

    pub options: Vec<CommandOption>,

    #[serde(skip)]
    pub action: AsyncCommandFn<'a>,
}


#[derive(Clone, Serialize)]
pub struct CommandOption {
    #[serde(rename = "type")]
    pub option_type: CommandOptionType,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "name_localizations")]
    pub i18n_names: I18nMap,
    #[serde(skip_serializing_if = "Option::is_none", rename = "description_localizations")]
    pub i18n_descriptions: I18nMap,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<Choice>,
}

impl<'a> Command<'a> {
    pub async fn invoke(
        &self,
        ctx: ChatInputCommandContext,
        interaction: Box<ApplicationCommand>,
    ) -> InteractionResponse {
        (self.action)(ctx, interaction).await
    }
}

mod choice;
mod context;
pub mod i18n;
pub mod option;

use futures::future::LocalBoxFuture;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use twilight_model::application::command::{CommandOptionType, CommandType};
use twilight_model::application::interaction::ApplicationCommand;

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

fn serialize_permissions<S: serde::Serializer>(
    value: &Option<u64>,
    s: S,
) -> Result<S::Ok, S::Error> {
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
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "description_localizations"
    )]
    pub i18n_descriptions: I18nMap,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_permissions"
    )]
    pub default_permissions: Option<u64>,

    pub options: Vec<CommandOption>,

    #[serde(skip)]
    pub action: AsyncCommandFn<'a>,
}

impl Debug for Command<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{\n", self.name)?;
        write!(f, "    type: {:?},\n", self.command_type)?;
        write!(f, "    description: {:?},\n", self.description)?;
        write!(f, "    i18n_names: {:?},\n", self.i18n_names)?;
        write!(f, "    i18n_descriptions: {:?},\n", self.i18n_descriptions)?;
        write!(
            f,
            "    default_permissions: {:?},\n",
            self.default_permissions
        )?;
        write!(f, "    options: {:?},\n", self.options)?;
        write!(f, "}}")
    }
}

impl ToString for Command<'_> {
    /**
    Returns JSON serialized string.
    **/
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CommandOption {
    #[serde(rename = "type")]
    pub option_type: CommandOptionType,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "name_localizations")]
    pub i18n_names: I18nMap,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "description_localizations"
    )]
    pub i18n_descriptions: I18nMap,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<Choice>,
    pub required: bool,
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

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

/**
Discord Chat Input Command Structure.
 **/
#[derive(Clone)]
pub struct Command<'a> {
    pub name: String,
    pub description: String,
    pub i18n_names: I18nMap,
    pub i18n_descriptions: I18nMap,
    pub default_permissions: Option<u64>,

    pub options: Vec<CommandOption>,

    pub action: AsyncCommandFn<'a>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CommandSerializable {
    #[serde(rename = "type")]
    pub command_type: CommandType,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: I18nMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: I18nMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permissions: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandOptionSerializable>,
}

impl<'a> From<Command<'a>> for CommandSerializable {
    fn from(command: Command) -> Self {
        Self {
            command_type: CommandType::ChatInput,
            name: command.name.clone(),
            description: command.description.clone(),
            name_localizations: command.i18n_names.clone(),
            description_localizations: command.i18n_descriptions.clone(),
            default_permissions: {
                match command.default_permissions {
                    None => None,
                    Some(x) => Some(x.to_string())
                }
            },
            options: command.options.clone().into_iter().map(|x| CommandOptionSerializable::from(x)).collect::<Vec<_>>(),
        }
    }
}

#[derive(Clone)]
pub struct CommandOption {
    pub option_type: CommandOptionType,
    pub name: String,
    pub description: String,
    pub i18n_names: I18nMap,
    pub i18n_descriptions: I18nMap,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct CommandOptionSerializable {
    #[serde(rename = "type")]
    pub option_type: CommandOptionType,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: I18nMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: I18nMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<Choice>>,
}


impl From<CommandOption> for CommandOptionSerializable {
    fn from(option: CommandOption) -> Self {
        Self {
            option_type: option.option_type.clone(),
            name: option.name.clone(),
            description: option.description.clone(),
            name_localizations: option.i18n_names.clone(),
            description_localizations: option.i18n_descriptions.clone(),
            choices: {
                if option.choices.is_empty() {
                    None
                } else {
                    Some(option.choices)
                }
            }
        }
    }
}

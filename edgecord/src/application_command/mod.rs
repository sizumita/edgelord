pub mod choice;
pub mod context;
pub mod group;
pub mod i18n;
pub mod option;

use futures::future::LocalBoxFuture;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use twilight_model::application::command::CommandType;
use twilight_model::application::interaction::application_command::CommandDataOption;
use twilight_model::application::interaction::ApplicationCommand;

use crate::InteractionResponse;
pub use choice::*;
pub use context::*;
pub use group::*;
pub use option::*;

type I18nMap = Option<HashMap<i18n::Locales, String>>;
type AsyncCommandFn = Rc<
    dyn 'static
        + Fn(
            ChatInputCommandContext,
            Box<ApplicationCommand>,
            Vec<CommandDataOption>,
        ) -> LocalBoxFuture<'static, worker::Result<worker::Response>>,
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
pub struct Command {
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
    pub action: AsyncCommandFn,
}

impl Debug for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {{", self.name)?;
        writeln!(f, "    type: {:?},", self.command_type)?;
        writeln!(f, "    description: {:?},", self.description)?;
        writeln!(f, "    i18n_names: {:?},", self.i18n_names)?;
        writeln!(f, "    i18n_descriptions: {:?},", self.i18n_descriptions)?;
        writeln!(
            f,
            "    default_permissions: {:?},",
            self.default_permissions
        )?;
        writeln!(f, "    options: {:?},", self.options)?;
        write!(f, "}}")
    }
}

impl ToString for Command {
    /**
    Returns JSON serialized string.
    **/
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Command {
    pub async fn invoke(
        &self,
        ctx: ChatInputCommandContext,
        interaction: Box<ApplicationCommand>,
        options: Vec<CommandDataOption>,
    ) -> InteractionResponse {
        (self.action)(ctx, interaction, options).await
    }
}

#[derive(Clone)]
pub enum SubCommand {
    Command(Command),
    Group(CommandGroup),
}

impl Serialize for SubCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Command(command) => {
                CommandAsSubCommand::from(command.clone()).serialize(serializer)
            }
            Self::Group(group) => {
                CommandGroupAsSubCommandGroup::from(group.clone()).serialize(serializer)
            }
        }
    }
}

impl SubCommand {
    pub fn is_group(&self) -> bool {
        match self {
            Self::Command(_) => false,
            Self::Group(_) => true,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Self::Command(cmd) => cmd.name.clone(),
            Self::Group(group) => group.name.clone(),
        }
    }
}

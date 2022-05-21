pub mod choice;
pub mod context;
pub mod i18n;
pub mod option;

use futures::future::LocalBoxFuture;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use twilight_model::application::command::CommandType;
use twilight_model::application::interaction::ApplicationCommand;

use crate::InteractionResponse;
pub use choice::*;
pub use context::*;
pub use option::*;

type I18nMap = Option<HashMap<i18n::Locales, String>>;
type AsyncCommandFn = Rc<
    dyn 'static
        + Fn(
            ChatInputCommandContext,
            Box<ApplicationCommand>,
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
    ) -> InteractionResponse {
        (self.action)(ctx, interaction).await
    }
}

#[derive(Clone)]
pub struct CommandGroup {
    pub name: String,
    pub i18n_names: I18nMap,
    pub description: String,
    pub i18n_descriptions: I18nMap,
    pub default_permissions: Option<u64>,

    pub commands: Vec<Command>,
}

impl Serialize for CommandGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CommandGroup", 6)?;
        state.serialize_field("name", self.name.as_str())?;
        state.serialize_field("description", self.description.as_str())?;

        if let Some(map) = self.i18n_names.clone() {
            state.serialize_field("name_localizations", &map)?;
        } else {
            state.skip_field("name_localizations")?;
        }

        if let Some(map) = self.i18n_descriptions.clone() {
            state.serialize_field("description_localizations", &map)?;
        } else {
            state.skip_field("description_localizations")?;
        }

        if let Some(permissions) = self.default_permissions {
            state.serialize_field("default_member_permissions", &permissions)?;
        } else {
            state.skip_field("default_member_permissions")?;
        }

        let options = self
            .commands
            .iter()
            .map(|command| command.clone().into())
            .collect::<Vec<CommandAsSubCommand>>();
        state.serialize_field("options", &options)?;

        state.end()
    }
}

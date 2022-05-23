use crate::application_command::{Choice, Command, I18nMap};
use crate::Error;
use serde::{Serialize, Serializer};
use twilight_model::application::command::CommandOptionType;
use twilight_model::application::interaction::application_command::CommandOptionValue;
use twilight_model::id::marker::{
    AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker,
};
use twilight_model::id::Id;

#[derive(Debug, Clone)]
pub enum RangeValue {
    Integer(i64),
    Float(f64),
}

impl From<i64> for RangeValue {
    fn from(value: i64) -> Self {
        RangeValue::Integer(value)
    }
}

impl From<f64> for RangeValue {
    fn from(value: f64) -> Self {
        RangeValue::Float(value)
    }
}

impl Serialize for RangeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            RangeValue::Integer(i) => serializer.serialize_i64(*i),
            RangeValue::Float(f) => serializer.serialize_f64(*f),
        }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<RangeValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<RangeValue>,
}

#[derive(Clone, Serialize)]
pub struct CommandAsSubCommand {
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
    options: Vec<CommandOption>,
}

impl From<Command> for CommandAsSubCommand {
    fn from(command: Command) -> Self {
        Self {
            option_type: CommandOptionType::SubCommand,
            name: command.name,
            description: command.description,
            i18n_names: command.i18n_names,
            i18n_descriptions: command.i18n_descriptions,
            options: command.options,
        }
    }
}

/**
Trait for command option. If you implemented it, you can use it for command option.
**/
pub trait FromCommandOptionValue {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized;
    fn get_option_type() -> CommandOptionType;
    fn choices() -> Vec<Choice> {
        vec![]
    }
}

impl FromCommandOptionValue for Id<ChannelMarker> {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        match value {
            CommandOptionValue::Channel(value) => Ok(value),
            _ => Err(Error::WrongOptionType),
        }
    }

    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Channel
    }
}

impl FromCommandOptionValue for Id<RoleMarker> {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        match value {
            CommandOptionValue::Role(value) => Ok(value),
            _ => Err(Error::WrongOptionType),
        }
    }

    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Role
    }
}

impl FromCommandOptionValue for Id<UserMarker> {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        match value {
            CommandOptionValue::User(value) => Ok(value),
            _ => Err(Error::WrongOptionType),
        }
    }

    fn get_option_type() -> CommandOptionType {
        CommandOptionType::User
    }
}

impl FromCommandOptionValue for Id<GenericMarker> {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        match value {
            CommandOptionValue::Mentionable(value) => Ok(value),
            _ => Err(Error::WrongOptionType),
        }
    }

    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Mentionable
    }
}

impl FromCommandOptionValue for Id<AttachmentMarker> {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        match value {
            CommandOptionValue::Attachment(value) => Ok(value),
            _ => Err(Error::WrongOptionType),
        }
    }

    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Attachment
    }
}

impl FromCommandOptionValue for String {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        match value {
            CommandOptionValue::String(value) => Ok(value),
            _ => Err(Error::WrongOptionType),
        }
    }

    fn get_option_type() -> CommandOptionType {
        CommandOptionType::String
    }
}

impl FromCommandOptionValue for i64 {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        match value {
            CommandOptionValue::Integer(value) => Ok(value),
            _ => Err(Error::WrongOptionType),
        }
    }

    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Integer
    }
}

impl FromCommandOptionValue for bool {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        match value {
            CommandOptionValue::Boolean(value) => Ok(value),
            _ => Err(Error::WrongOptionType),
        }
    }

    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Boolean
    }
}

impl FromCommandOptionValue for f64 {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized,
    {
        match value {
            CommandOptionValue::Number(value) => Ok(value.0),
            _ => Err(Error::WrongOptionType),
        }
    }

    fn get_option_type() -> CommandOptionType {
        CommandOptionType::Number
    }
}

#[cfg(test)]
mod tests {
    use super::FromCommandOptionValue;
    use twilight_model::application::interaction::application_command::CommandOptionValue;

    #[test]
    fn test_string() {
        assert_eq!(
            &String::from_option(CommandOptionValue::String("avc".to_string())).unwrap(),
            "avc"
        )
    }
}

use crate::{Choice, Error};
use twilight_model::application::command::CommandOptionType;
use twilight_model::application::interaction::application_command::CommandOptionValue;
use twilight_model::id::marker::{
    AttachmentMarker, ChannelMarker, GenericMarker, RoleMarker, UserMarker,
};
use twilight_model::id::Id;
use serde::{Serialize};
use crate::command::I18nMap;

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

/**
Trait for command option. If you implemented it, you can use it for command option.
**/
pub trait FromCommandOptionValue {
    fn from_option(value: CommandOptionValue) -> Result<Self, crate::Error>
    where
        Self: Sized;
    fn get_option_type() -> CommandOptionType;
    fn has_choices() -> bool {
        false
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
    use crate::option::FromCommandOptionValue;
    use twilight_model::application::interaction::application_command::CommandOptionValue;

    #[test]
    fn test_string() {
        assert_eq!(
            &String::from_option(CommandOptionValue::String("avc".to_string())).unwrap(),
            "avc"
        )
    }
}

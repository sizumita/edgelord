use crate::application_command::{Command, I18nMap, SubCommand};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use twilight_model::application::command::CommandOptionType;
use twilight_model::application::interaction::application_command::{
    CommandDataOption, CommandOptionValue,
};

#[derive(Clone)]
pub struct CommandGroup {
    pub name: String,
    pub i18n_names: I18nMap,
    pub description: String,
    pub i18n_descriptions: I18nMap,
    pub default_permissions: Option<u64>,

    pub commands: Vec<SubCommand>,
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

        state.serialize_field("options", &self.commands)?;

        state.end()
    }
}

impl CommandGroup {
    pub fn get_command(
        &self,
        option: CommandDataOption,
    ) -> Option<(Command, Vec<CommandDataOption>)> {
        match option.value {
            CommandOptionValue::SubCommand(options) => {
                if let Some(SubCommand::Command(command)) = self
                    .commands
                    .iter()
                    .filter(|x| !x.is_group())
                    .find(|x| x.get_name() == option.name)
                    .cloned()
                {
                    Some((command, options))
                } else {
                    None
                }
            }
            CommandOptionValue::SubCommandGroup(subcommands) => {
                if let Some(SubCommand::Group(group)) = self
                    .commands
                    .iter()
                    .filter(|x| x.is_group())
                    .find(|x| x.get_name() == option.name)
                    .cloned()
                {
                    subcommands
                        .iter()
                        .filter_map(|x| group.get_command(x.clone()))
                        .collect::<Vec<_>>()
                        .first()
                        .cloned()
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct CommandGroupAsSubCommandGroup {
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
    options: Vec<SubCommand>,
}

impl From<CommandGroup> for CommandGroupAsSubCommandGroup {
    fn from(group: CommandGroup) -> Self {
        Self {
            option_type: CommandOptionType::SubCommandGroup,
            name: group.name,
            description: group.description,
            i18n_names: group.i18n_names,
            i18n_descriptions: group.i18n_descriptions,
            options: group.commands,
        }
    }
}

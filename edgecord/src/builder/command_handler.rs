use crate::application_command::{Command, CommandGroup};
use crate::handler::InteractionHandler;
use ed25519_dalek::PublicKey;
use std::str::FromStr;
use twilight_model::id::Id;

/**
A builder for [`InteractionHandler`].
 **/
#[derive(Default)]
pub struct CommandHandlerBuilder {
    commands: Vec<Command>,
    groups: Vec<CommandGroup>,
    public_key: Option<String>,
    token: Option<String>,
    application_id: Option<String>,
}

impl CommandHandlerBuilder {
    pub fn new() -> Self {
        CommandHandlerBuilder::default()
    }

    /**
    Register command for [`InteractionHandler`].
    You should create [`Command`] with `command` macro.
    **/
    pub fn command(&mut self, command: Command) -> &mut Self {
        self.commands.push(command);
        self
    }

    /**
    Register command group for [`InteractionHandler`].
    You can create [`CommandGroup`] with `group` macro.
    **/
    pub fn group(&mut self, group: CommandGroup) -> &mut Self {
        self.groups.push(group);
        self
    }

    /**
    Register application public key to handler.

    You have to call this function before build.
    **/
    pub fn public_key(&mut self, public_key: &str) -> &mut Self {
        self.public_key = Some(public_key.to_string());
        self
    }

    /**
    Register discord bot token to handler.
    **/
    pub fn token(&mut self, token: &str) -> &mut Self {
        self.token = Some(token.to_string());
        self
    }

    /**
    Register application id to handler.
    **/
    pub fn application_id(&mut self, application_id: &str) -> &mut Self {
        self.application_id = Some(application_id.to_string());
        self
    }

    /**
    Build and return [`InteractionHandler`].
    **/
    pub fn build(&mut self) -> Result<InteractionHandler, Box<dyn std::error::Error>> {
        Ok(InteractionHandler {
            commands: self.commands.clone(),
            groups: self.groups.clone(),
            public_key: PublicKey::from_bytes(&*hex::decode(
                self.public_key.clone().unwrap().as_bytes(),
            )?)?,
            token: self.token.clone().unwrap_or_default(),
            application_id: Id::from_str(&*self.application_id.clone().unwrap_or_default())
                .unwrap(),
        })
    }
}

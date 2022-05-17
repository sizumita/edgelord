use ed25519_dalek::PublicKey;
use crate::{Command, InteractionHandler, http};

/**
A builder for [`InteractionHandler`].
 **/
pub struct CommandHandlerBuilder<'a> {
    commands: Vec<Command<'a>>,
    public_key: Option<String>,
}


impl<'a> CommandHandlerBuilder<'a> {
    pub fn new() -> Self {
        CommandHandlerBuilder {
            commands: vec![],
            public_key: None,
        }
    }

    /**
    Register command for [`InteractionHandler`].
    You should create [`Command`] with `command` macro.
    **/
    pub fn command(&mut self, func: fn() -> Command<'a>) -> &mut Self {
        self.commands.push(func());
        self
    }

    /**
    Register application public key for handler.

    You have to call this function before build.
    **/
    pub fn public_key(&mut self, public_key: &str) -> &mut Self {
        self.public_key = Some(public_key.to_string());
        self
    }

    /**
    Build and return [`InteractionHandler`].
    **/
    pub fn build(&mut self, token: &str, application_id: &str) -> Result<InteractionHandler<'a>, Box<dyn std::error::Error>> {
        Ok(
            InteractionHandler {
                commands: self.commands.clone(),
                http: http::HttpClient::new(token, application_id),
                public_key: PublicKey::from_bytes(
                    &*hex::decode(
                        self.public_key.clone().unwrap().as_bytes()
                    )?
                )?,
            }
        )
    }
}

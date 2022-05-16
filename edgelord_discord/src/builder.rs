use crate::{Command, CommandHandler, http};

/**
A builder for [`CommandHandler`].
 **/
pub struct CommandHandlerBuilder {
    commands: Vec<Command>,
}


impl CommandHandlerBuilder {
    pub fn new() -> Self {
        CommandHandlerBuilder {
            commands: vec![],
        }
    }

    /**
    Register command for [`CommandHandler`].
    You should create [`Command`] with `command` macro.
    **/
    pub fn command(&mut self, func: fn() -> Command) -> &mut Self {
        self.commands.push(func());
        self
    }

    /**
    Build and return [`CommandHandler`].
    **/
    pub fn build(&mut self, token: &str, application_id: &str, public_key: &str) -> CommandHandler {
        CommandHandler {
            commands: self.commands.clone(),
            http: http::HttpClient::new(token, application_id, public_key),
        }
    }
}

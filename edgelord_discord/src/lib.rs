mod command;

use std::collections::HashMap;

pub use edgelord_discord_macros::*;
pub use command::{Command, Context};
pub use twilight_model as model;

pub type InteractionResponse = worker::Result<worker::Response>;

pub struct CommandHandler {

}

pub struct CommandHandlerBuilder {

}

impl CommandHandler {
    pub fn builder() -> CommandHandlerBuilder {
        CommandHandlerBuilder::new()
    }

    pub async fn process(&self, req: worker::Request, env: worker::Env, ctx: worker::Context) -> worker::Result<worker::Response> {
        worker::Response::ok("ok")
    }
}

impl CommandHandlerBuilder {
    pub fn new() -> Self {
        CommandHandlerBuilder {}
    }

    pub fn command(&mut self, command: &str) -> &mut Self {
        // TODO: Impl
        self
    }

    pub fn build(&mut self) -> CommandHandler {
        // TODO: Impl
        CommandHandler {}
    }
}

mod command;

use std::collections::HashMap;
use worker::{Request, Env, Context, Response};

pub use edgelord_discord_macros::*;
pub use command::*;

pub struct CommandHandler {

}

pub struct CommandHandlerBuilder {

}

impl CommandHandler {
    pub fn builder() -> CommandHandlerBuilder {
        CommandHandlerBuilder::new()
    }

    pub async fn process(&self, req: Request, env: Env, ctx: Context) -> worker::Result<Response> {
        Response::ok("ok")
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

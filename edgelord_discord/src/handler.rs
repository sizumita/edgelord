use crate::{Command, CommandHandlerBuilder, http};

/**
A Discord Interaction Handler.
Parse Interaction and dispatch commands.
**/
pub struct CommandHandler {
    pub commands: Vec<Command>,
    pub http: http::HttpClient,
}

impl CommandHandler {
    /**
    Returns [`CommandHandlerBuilder`].

    # Example

    ```rust
    let handler = CommandHandler::builder().build("token", "app id", "app public key");
    handler.process(req, env, ctx).await
    ```

    **/
    pub fn builder() -> CommandHandlerBuilder {
        CommandHandlerBuilder::new()
    }

    /**
    Handle Interaction and response.
    **/
    pub async fn process(&self, req: worker::Request, env: worker::Env, ctx: worker::Context) -> worker::Result<worker::Response> {
        worker::Response::ok("ok")
    }

    /**
    Verify interaction and return verify result.
    **/
    pub async fn verify(&self, req: worker::Request) -> bool {
        todo!()
    }
}

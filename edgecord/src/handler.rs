use crate::http::HttpClient;
use crate::{ChatInputCommandContext, Command, CommandHandlerBuilder};
use ed25519_dalek::{PublicKey, Signature, Verifier};
use twilight_model::application::interaction::{Interaction, InteractionType};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use worker::{console_error, Response};

/**
A Discord Interaction Handler.
Parse Interaction and dispatch commands.
**/
pub struct InteractionHandler {
    pub commands: Vec<Command>,
    pub public_key: PublicKey,
    pub token: String,
}

impl InteractionHandler {
    /**
    Returns [`CommandHandlerBuilder`].

    # Example

    ```ignore
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
    pub async fn process(
        &self,
        mut req: worker::Request,
        env: worker::Env,
        ctx: worker::Context,
    ) -> worker::Result<worker::Response> {
        if let Err(err) = self.verify(&req).await {
            console_error!("verify error: {}", err.to_string());
            return Response::error(err.to_string(), 401);
        }
        let interaction = req.json::<Interaction>().await?;

        match interaction {
            Interaction::Ping(_ping) => worker::Response::from_json(&InteractionResponse {
                kind: InteractionResponseType::Pong,
                data: None,
            }),
            Interaction::ApplicationCommand(command) => match command.kind {
                InteractionType::ApplicationCommand => {
                    match self.get_command(command.data.name.clone()) {
                        None => {
                            console_error!("command not found");
                            panic!("command not found");
                        }
                        Some(cmd) => {
                            let cmd_ctx = ChatInputCommandContext::new(
                                command.clone(),
                                env,
                                ctx,
                                HttpClient::new(&*self.token),
                            );
                            return cmd.invoke(cmd_ctx, command).await;
                        }
                    }
                }
                _ => {
                    unreachable!()
                }
            },
            _ => worker::Response::ok("ok"),
        }
    }

    /**
    Verify interaction and return verify result.
    **/
    pub async fn verify(&self, req: &worker::Request) -> Result<(), Box<dyn std::error::Error>> {
        let signature = Signature::from_bytes(&*hex::decode(
            req.headers().get("X-Signature-Ed25519")?.unwrap(),
        )?)?;
        let mut message = req
            .headers()
            .get("X-Signature-Timestamp")?
            .unwrap()
            .into_bytes();
        message.append(&mut req.clone()?.bytes().await?);

        self.public_key
            .verify(message.as_slice(), &signature)
            .map_err(|e| e.into())
    }

    pub fn get_command(&self, name: String) -> Option<Command> {
        self.commands.iter().find(|cmd| cmd.name == name).cloned()
    }
}

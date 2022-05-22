use crate::application_command::{ChatInputCommandContext, Command, CommandGroup};
use crate::builder::CommandHandlerBuilder;
use crate::http::HttpClient;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use twilight_model::application::interaction::application_command::CommandDataOption;
use twilight_model::application::interaction::{ApplicationCommand, Interaction};
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use worker::{console_error, Response};

/**
A Discord Interaction Handler.
Parse Interaction and dispatch commands.
**/
pub struct InteractionHandler {
    pub commands: Vec<Command>,
    pub groups: Vec<CommandGroup>,
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
            Interaction::ApplicationCommand(command) => {
                self.handle_application_command(command, req, env, ctx)
                    .await
            }
            _ => worker::Response::ok("ok"),
        }
    }

    async fn handle_application_command(
        &self,
        command: Box<ApplicationCommand>,
        _req: worker::Request,
        env: worker::Env,
        ctx: worker::Context,
    ) -> worker::Result<worker::Response> {
        match self.get_command(&command) {
            None => {
                console_error!("command not found");
                panic!("command not found");
            }
            Some((cmd, options)) => {
                let cmd_ctx = ChatInputCommandContext::new(
                    command.clone(),
                    env,
                    ctx,
                    HttpClient::new(&*self.token),
                );
                return cmd.invoke(cmd_ctx, command, options).await;
            }
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

    pub fn get_command(
        &self,
        command: &ApplicationCommand,
    ) -> Option<(Command, Vec<CommandDataOption>)> {
        for group in &self.groups {
            if group.name != command.data.name {
                continue;
            }
            let commands = command
                .data
                .options
                .iter()
                .filter_map(|x| group.get_command(x.clone()))
                .collect::<Vec<_>>();
            if let Some(cmd) = commands.first() {
                return Some(cmd.clone());
            }
        }

        self.commands
            .iter()
            .find(|cmd| cmd.name == command.data.name)
            .cloned()
            .map(|cmd| (cmd, command.data.options.clone()))
    }
}

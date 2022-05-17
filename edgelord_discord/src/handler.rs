use ed25519_dalek::{PublicKey, Signature, Verifier};
use twilight_model::application::interaction::Interaction;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use worker::{console_error, console_log, Response};
use crate::{Command, CommandHandlerBuilder, http};

/**
A Discord Interaction Handler.
Parse Interaction and dispatch commands.
**/
pub struct InteractionHandler {
    pub commands: Vec<Command>,
    pub http: http::HttpClient,
    pub public_key: PublicKey,
}

impl InteractionHandler {
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
    pub async fn process(&self, mut req: worker::Request, env: worker::Env) -> worker::Result<worker::Response> {
        if let Err(err) = self.verify(&req).await {
            console_error!("verify error: {}", err.to_string());
            return Response::error(err.to_string(), 401);
        }
        let interaction = req.json::<Interaction>().await?;

        match interaction {
            Interaction::Ping(ping) => {
                worker::Response::from_json(
                    &InteractionResponse {
                        kind: InteractionResponseType::Pong,
                        data: None
                    }
                )
            }
            _ => worker::Response::ok("ok")
        }
    }

    /**
    Verify interaction and return verify result.
    **/
    pub async fn verify(&self, mut req: &worker::Request) -> Result<(), Box<dyn std::error::Error>> {
        console_log!("{:?}", self.public_key);
        let signature = Signature::from_bytes(
            &*hex::decode(
                req.headers().get("X-Signature-Ed25519")?.unwrap()
            )?
        )?;
        let timestamp = req.headers().get("X-Signature-Timestamp")?.unwrap();
        let body = req.clone()?.text().await?;

        let message = format!("{}{}", timestamp, body);

        self.public_key.verify(message.as_bytes(), &signature).map_err(|e| e.into())
    }
}

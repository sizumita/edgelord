use std::collections::HashMap;
use edgelord_discord::{CommandHandler, command, ChatInputCommandContext, InteractionResponse};
use worker::*;


#[event(fetch)]
pub async fn fetch(req: Request, env: Env, ctx: worker::Context) -> Result<Response> {
    let handler = CommandHandler::builder()
        .command(help_command)
        .build(
            &*std::env::var("DISCORD_BOT_TOKEN").expect("discord bot token is not found"),
            &*std::env::var("APPLICATION_ID").expect("application id is not found"),
            &*std::env::var("APPLICATION_PUBLIC_KEY").expect("application public key is not found"),
        );
    handler.process(req, env, ctx).await
}

fn names() -> HashMap<&'static str, String> {
    HashMap::from([
        ("ja", "ヘルプだよ".to_string()),
    ])
}

#[command(
    name = "help",
    description = "help command for newer.",
    i18n_names = "names",
)]
pub async fn help_command(
    ctx: ChatInputCommandContext,
    #[description = "this is text what you want to show"]
    text: String) -> InteractionResponse {
    ctx.message(&*text)
}

mod utils;

use std::collections::HashMap;
use edgelord_discord::{InteractionHandler, command, ChatInputCommandContext, InteractionResponse};
use worker::*;
use edgelord_discord::i18n::Locales;


#[event(fetch)]
pub async fn fetch(req: Request, env: Env, ctx: worker::Context) -> Result<Response> {
    utils::set_panic_hook();

    let handler = InteractionHandler::builder()
        // .command(help_command)
        .public_key(&*env.secret("APPLICATION_PUBLIC_KEY")?.to_string())
        .build(
            &*env.secret("DISCORD_BOT_TOKEN")?.to_string(),
            &*env.secret("APPLICATION_ID")?.to_string(),
        ).unwrap();
    handler.process(req, env).await
}

fn names() -> HashMap<Locales, String> {
    HashMap::from([
        (Locales::Ja, "ヘルプだよ".to_string()),
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

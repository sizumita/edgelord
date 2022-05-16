use std::collections::HashMap;
use edgelord_discord::{CommandHandler, command};
use worker::*;


#[event(fetch)]
pub async fn fetch(req: Request, env: Env, ctx: worker::Context) -> Result<Response> {
    let handler = CommandHandler::builder()
        .command("help")
        .build();
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
pub async fn help_command() {

}

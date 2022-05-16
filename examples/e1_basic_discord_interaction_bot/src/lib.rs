use std::collections::HashMap;
use edgelord_discord::{CommandHandler, command, Context, model, InteractionResponse};
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
pub async fn help_command(
    ctx: Context,
    #[description = "this is abc"]
    abc: String) -> InteractionResponse {
    ctx.message("abc")
}

#[cfg(test)]
mod tests {
    use super::help_command;
    #[test]
    fn a() {
        println!("{:?}", help_command().i18n_names);
    }
}

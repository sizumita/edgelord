use edgelord_discord::CommandHandler;
use worker::*;


#[event(fetch)]
pub async fn fetch(req: Request, env: Env, ctx: worker::Context) -> Result<Response> {
    let handler = CommandHandler::builder()
        .command("help")
        .build();
    handler.process(req, env, ctx).await
}

use std::collections::HashMap;
use edgelord::discord::{InteractionHandler, command, ChatInputCommandContext, InteractionResponse};
use worker::*;
use edgelord::discord::i18n::Locales;


#[event(fetch)]
pub async fn fetch(req: Request, env: Env, worker_context: worker::Context) -> Result<Response> {
    edgelord::set_panic_hook();

    let router = Router::with_data(worker_context);

    router
        .post_async(
            "/", |req, ctx| async move {
                let RouteContext {env, data, ..} = ctx;
                let handler = InteractionHandler::builder()
                    .command(help_command)
                    .public_key(&*env.secret("APPLICATION_PUBLIC_KEY")?.to_string())
                    .build(
                        &*env.secret("DISCORD_BOT_TOKEN")?.to_string(),
                        &*env.secret("APPLICATION_ID")?.to_string(),
                    ).unwrap();
                handler.process(req, env, data).await
            }
        )
        .run(req, env).await
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
    ) -> InteractionResponse {
    ctx.message("this is what you want");
}

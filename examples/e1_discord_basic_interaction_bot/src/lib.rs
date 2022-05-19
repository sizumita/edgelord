use edgecord::i18n::Locales;
use edgecord::{
    command, ChatInputCommandContext, Choiceable, InteractionHandler, InteractionResponse,
};
use std::collections::HashMap;
use worker::*;

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, worker_context: worker::Context) -> Result<Response> {
    edgelord::set_panic_hook();

    let router = Router::with_data(worker_context);

    router
        .post_async("/", |req, ctx| async move {
            let RouteContext { env, data, .. } = ctx;
            let handler = InteractionHandler::builder()
                .command(help_command)
                .command(animal_image)
                .public_key(&*env.secret("APPLICATION_PUBLIC_KEY")?.to_string())
                .build(
                    &*env.secret("DISCORD_BOT_TOKEN")?.to_string(),
                    &*env.secret("APPLICATION_ID")?.to_string(),
                )
                .unwrap();
            handler.process(req, env, data).await
        })
        .run(req, env)
        .await
}

fn names() -> HashMap<Locales, String> {
    HashMap::from([(Locales::Ja, "ヘルプだよ".to_string())])
}

#[command(
    name = "help",
    description = "help command for newer.",
    i18n_names = "names",
    default_permissions(manage_roles, manage_webhooks)
)]
pub async fn help_command(ctx: ChatInputCommandContext) -> InteractionResponse {
    ctx.message("this is what you want")
}

#[command(name = "animal", description = "show animal image")]
pub async fn animal_image(
    ctx: ChatInputCommandContext,
    #[option(description = "the animal name you want to see", required)] name: Animals,
) -> InteractionResponse {
    match name {
        Animals::Cat => ctx.message("cat image"),
        Animals::Dog => ctx.message("dog image"),
    }
}

#[derive(Debug, Choiceable, PartialEq)]
#[choice(value_type = "integer")]
pub enum Animals {
    Dog = 12,
    Cat = 36,
}

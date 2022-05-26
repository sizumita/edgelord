use edgecord::application_command::{ChatInputCommandContext, SubCommand};
use edgecord::handler::InteractionHandler;
use edgecord::{command, group, InteractionResponse};
use worker::*;

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, worker_context: worker::Context) -> Result<Response> {
    edgelord::set_panic_hook();

    let router = Router::with_data(worker_context);

    router
        .post_async("/", |req, ctx| async move {
            let RouteContext { env, data, .. } = ctx;
            let handler = InteractionHandler::builder()
                .group(emojis())
                .public_key(&*env.secret("APPLICATION_PUBLIC_KEY")?.to_string())
                .application_id(&*env.secret("APPLICATION_ID")?.to_string())
                .token(&*env.secret("DISCORD_BOT_TOKEN")?.to_string())
                .build()
                .unwrap();
            handler.process(req, env, data).await
        })
        .run(req, env)
        .await
}

#[group(description = "show anything emoji")]
pub fn emojis() -> Vec<SubCommand> {
    vec![
        SubCommand::Group(animals()),
        SubCommand::Command(human_emoji()),
    ]
}

#[group(description = "show animal emoji")]
fn animals() -> Vec<SubCommand> {
    vec![
        SubCommand::Command(cat_emoji()),
        SubCommand::Command(dog_emoji()),
    ]
}

#[command(name = "cat", description = "show cat emoji")]
async fn cat_emoji(ctx: ChatInputCommandContext) -> InteractionResponse {
    ctx.message(|msg| msg.content("🐱"))
}

#[command(name = "dog", description = "show dog emoji")]
async fn dog_emoji(ctx: ChatInputCommandContext) -> InteractionResponse {
    ctx.message(|msg| msg.content("🐶"))
}

#[command(name = "human", description = "show human emoji")]
async fn human_emoji(ctx: ChatInputCommandContext) -> InteractionResponse {
    ctx.message(|msg| msg.content("👶"))
}

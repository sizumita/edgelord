use edgecord::application_command::ChatInputCommandContext;
use edgecord::handler::InteractionHandler;
use edgecord::{command, InteractionResponse};
use worker::*;
use edgecord::model::channel::message::MessageFlags;

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, worker_context: worker::Context) -> Result<Response> {
    edgelord::set_panic_hook();

    let router = Router::with_data(worker_context);

    router
        .post_async("/", |req, ctx| async move {
            let RouteContext { env, data, .. } = ctx;
            let handler = InteractionHandler::builder()
                .command(send_lazy_message())
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

#[command(name = "lazy", description = "send message after 3s")]
pub async fn send_lazy_message(ctx: ChatInputCommandContext) -> InteractionResponse {
    ctx.defer(|msg| msg.flag(MessageFlags::EPHEMERAL))
}

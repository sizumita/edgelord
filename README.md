Edge Computing + chūnibyō = Edgelord ✨👿

# Edgelord

<img src="https://img.shields.io/github/workflow/status/sizumita/edgelord/CI" />

**Edgelord is now working. You can contribute for it.**

Edgelord is a Rust library for cloudflare workers. 
You can use Edgelord to scaffold a basic bot for social networking service.

Edgelord supports previous bot:

- 🚧 discord -> edgecord
- 🚧 slack

Since Edgelord is a Light Wrapper for cloudflare workers, you can use as a foundation to build a bot on.

# Features

- 🚧 discord bot support
- 🚧 slack bot support
- 🚧 Documentation

## 🚧 Edgecord - Discord http interaction bot handler

`edgecord` is a fast, light weight, powerful discord http interaction bot framework.
It provides type safely commands, automatically publish system.

### Example

```rust
use edgecord::{
    command, ChatInputCommandContext, InteractionHandler, InteractionResponse,
};
use worker::*;

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, worker_context: worker::Context) -> Result<Response> {
    edgelord::set_panic_hook();

    let router = Router::with_data(worker_context);

    router
        .post_async("/", |req, ctx| async move {
            let RouteContext { env, data, .. } = ctx;
            let handler = InteractionHandler::builder()
                .command(help_command())
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

#[command(
    name = "help",
    description = "help command.",
)]
pub async fn help_command(ctx: ChatInputCommandContext) -> InteractionResponse {
    ctx.message("this is what you want")
}
```

# Contribution

You can create issue or PR to contribute.

I want to you to contribute 💪

# 🚧 Documentation

now working

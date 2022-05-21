#![allow(unused_imports)]
use e2_discord_subcommand_bot::animals;
use edgecord::application_command::CommandGroup;
use edgecord::http::{HttpClient, Routes};
use edgecord::model::application::command::Command;
use edgecord::model::id::Id;
use std::str::FromStr;
use worker::Method;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let client = HttpClient::new(std::env::var("DISCORD_BOT_TOKEN").unwrap().as_str());
    let result = client
        .request::<Vec<Command>, Vec<CommandGroup>>(
            Method::Put,
            Routes::ApplicationGuildCommands(
                Id::from_str(std::env::var("APPLICATION_ID").unwrap().as_str()).unwrap(),
                Id::new(731029130488971275),
            ),
            Some(vec![animals()]),
        )
        .await;
    println!("{:?}", result);
}

#[cfg(target_arch = "wasm32")]
fn main() {}

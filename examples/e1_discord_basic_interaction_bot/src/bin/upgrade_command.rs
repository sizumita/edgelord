use std::str::FromStr;
use worker::Method;
use e1_basic_discord_interaction_bot::{animal_image, help_command};
use edgecord::http::{HttpClient, Routes};
use edgecord::model::application::command::Command;
use edgecord::model::id::Id;


#[tokio::main]
async fn main() {
    let client = HttpClient::new(std::env::var("DISCORD_BOT_TOKEN").unwrap().as_str());
    let result = client.request::<Vec<Command>, Vec<edgecord::Command>>(
        Method::Put,
        Routes::ApplicationGuildCommands(
            Id::from_str(std::env::var("APPLICATION_ID").unwrap().as_str()).unwrap(),
            Id::new(731029130488971275),
        ),
        Some(
            vec![help_command(), animal_image()
        ]),
    ).await;
    println!("{:?}", result);
}

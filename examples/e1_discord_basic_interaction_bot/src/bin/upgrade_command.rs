use e1_basic_discord_interaction_bot::{animal_image, help_command};
use edgecord::upload::CommandUploader;

#[tokio::main]
async fn main() {
    let mut uploader = CommandUploader::new(
        std::env::var("DISCORD_BOT_TOKEN").unwrap().as_str(),
        std::env::var("APPLICATION_ID").unwrap().as_str(),
    );
    uploader.guild_id(731029130488971275);
    uploader.register_command(help_command());
    uploader.register_command(animal_image());
    uploader.upsert_commands().await;
}

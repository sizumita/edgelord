use crate::{Command, CommandSerializable};
use cfg_if::cfg_if;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use reqwest;


pub struct CommandUploader<'a> {
    commands: Vec<Command<'a>>,
    token: String,
    guild_id: Option<String>,
    application_id: String,
}

impl<'a> CommandUploader<'a> {
    pub fn new(token: &str, application_id: &str) -> Self {
        Self {
            commands: vec![],
            token: token.to_string(),
            guild_id: None,
            application_id: application_id.to_string(),
        }
    }

    pub fn register_command(&mut self, command: Command<'a>) -> &mut Self {
        self.commands.push(command);
        self
    }

    pub fn guild_id(&mut self, guild_id: u64) -> &mut Self {
        self.guild_id = Some(guild_id.to_string());
        self
    }

    pub fn make_body(&self) -> String {
        serde_json::to_string(
            &self.commands.clone().into_iter().map(
                |command| CommandSerializable::from(command.clone())
            ).collect::<Vec<_>>()
        ).unwrap()
    }

    cfg_if! {
        if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
            pub async fn upsert_commands(&self) {

            }
        } else {
            pub async fn upsert_commands(&self) {
                let client = reqwest::Client::new();
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Content-Type", reqwest::header::HeaderValue::from_str(&*"application/json").unwrap());
                headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&*format!("Bot {}", self.token)).unwrap());
                let response = client.put({
                    if self.guild_id.is_none() {
                        format!("https://discord.com/api/v10/applications/{}/commands", self.application_id)
                    } else {
                        format!("https://discord.com/api/v10/applications/{}/guilds/{}/commands", self.application_id, self.guild_id.as_ref().unwrap())
                    }
                }).headers(headers).body(self.make_body()).send().await.unwrap();
                // TODO improve visible
                if response.status().is_success().clone() {
                    println!("Successful updated {} commands.", self.commands.len());
                } else {
                    eprintln!("Error happend when updateing commands: \n{}", response.text().await.unwrap())
                }
            }
        }
    }
}

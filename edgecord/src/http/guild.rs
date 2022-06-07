use crate::http::{HttpClient, Routes};
use crate::Result;
use twilight_model::guild::Guild;
use twilight_model::id::marker::GuildMarker;
use twilight_model::id::Id;
use worker::Method;

impl HttpClient {
    pub async fn get_guild(&self, guild_id: Id<GuildMarker>) -> Result<Guild> {
        self.request::<(), Guild>(Method::Get, Routes::Guild(guild_id), None)
            .await
            .map(|g| g.unwrap())
    }
}

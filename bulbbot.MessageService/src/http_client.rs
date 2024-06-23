use actix_web::{web, App, HttpRequest, HttpResponse, Responder};
use reqwest::Client;
use serenity::all::GuildId;

use crate::dto::guild_dto::GuildDto;

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub client: Client,
}

impl HttpClient {
    pub fn init() -> HttpClient {
        let user_agent = format!(
            "bulbbot-{}({})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        HttpClient {
            client: Client::builder()
                .user_agent(user_agent)
                .build()
                .expect("Failed to create reqwest client"),
        }
    }

    pub async fn get_guild(&self, guild_id: GuildId) -> GuildDto {
        let url = format!("http://localhost:4614/api/guilds/{}", guild_id);
        let response = self.client.get(&url).send().await.unwrap();

        let guild = response.json::<GuildDto>().await.unwrap();

        guild
    }
}

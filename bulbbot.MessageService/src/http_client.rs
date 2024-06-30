use crate::{dto::guild_dto::GuildDto, injector::ReqwestInjector};
use opentelemetry::{global, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serenity::all::GuildId;

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

    pub async fn get_guild(&self, guild_id: GuildId, cx: &Context) -> GuildDto {
        let url = "http://localhost:4614/api/guilds";

        let mut request = self
            .client
            .get(format!("{}/{}", url, guild_id))
            .build()
            .unwrap();

        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(
                &cx,
                &mut ReqwestInjector {
                    headers: request.headers_mut(),
                },
            )
        });

        let response = self.client.execute(request).await.expect("Invalid reponse");

        let guild = match response.json::<GuildDto>().await {
            Ok(g) => g,
            Err(_) => {
                let mut request = self
                    .client
                    .post(url)
                    .json(&CreateGuildCommand {
                        id: guild_id.into(),
                    })
                    .build()
                    .unwrap();

                global::get_text_map_propagator(|propagator| {
                    propagator.inject_context(
                        &cx,
                        &mut ReqwestInjector {
                            headers: request.headers_mut(),
                        },
                    )
                });

                let response = self
                    .client
                    .execute(request)
                    .await
                    .expect("Invalid response");

                response.json::<GuildDto>().await.unwrap()
            }
        };

        guild
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildCommand {
    pub id: i64,
}

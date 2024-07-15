use crate::dto::guild_dto::GuildDto;
use actix_web::http::header::HeaderMap;
use common::telemetry::{
    extractor_actix_web::ActixWebExtractor, injector_reqwest::ReqwestInjector,
};
use opentelemetry::global;
use reqwest::{Client, Request};
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

    fn add_telelementry(&self, headers: &HeaderMap, request: &mut Request) {
        let cx = global::get_text_map_propagator(|propagator| {
            propagator.extract(&ActixWebExtractor { headers })
        });

        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(
                &cx,
                &mut ReqwestInjector {
                    headers: request.headers_mut(),
                },
            )
        });
    }

    pub async fn get_guild(&self, guild_id: GuildId, headers: &HeaderMap) -> GuildDto {
        let url = "http://localhost:4614/api/guilds";

        let mut request = self
            .client
            .get(format!("{}/{}", url, guild_id))
            .build()
            .unwrap();

        self.add_telelementry(headers, &mut request);

        let response = self.client.execute(request).await.expect("Invalid reponse");

        match response.json::<GuildDto>().await {
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

                self.add_telelementry(headers, &mut request);

                let response = self
                    .client
                    .execute(request)
                    .await
                    .expect("Invalid response");

                response.json::<GuildDto>().await.unwrap()
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGuildCommand {
    pub id: i64,
}

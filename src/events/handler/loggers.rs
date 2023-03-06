use serenity::prelude::Context;
use serenity::{http::Http, model::webhook::Webhook};

use crate::events::event_handler::Handler;

impl Handler {
    pub async fn send_log(&self, ctx: &Context, log_message: &str) {
        println!("Log message:\n{}", log_message);

        let http = Http::new("token");

        let url = "";
        let webhook = Webhook::from_url(&http, url).await.unwrap();

        webhook
            .execute(&http, true, |w| w.content(log_message))
            .await
            .unwrap();
    }
}

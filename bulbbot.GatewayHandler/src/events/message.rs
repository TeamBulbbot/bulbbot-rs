use crate::{handler::Handler, models::event_type::EventType};
use serde::{Deserialize, Serialize};
use serenity::model::channel::Message;
use tracing::{debug, error};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEvent {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: i64,
    pub request_id: Uuid,
    pub content: Message,
}

impl Handler {
    pub async fn handle_message_event(&self, event_data: &str) -> bool {
        let message: MessageEvent =
            serde_json::from_str(event_data).expect("Failed to parse data as message event");

        let url = format!("{}/messages", self.get_url(&message.event));
        let response = self.client.post(&url).json(&message).send().await;

        match response {
            Ok(_) => {
                debug!("Sucessful request to {}", url);
                true
            }
            Err(err) => {
                error!("Request to {} errored with {:#?}", url, err);
                false
            }
        }

        /*
           Message needs events
           - updates    [bulbbot.MessageService]     *
           - deletes    [bulbbot.MessageService]     *
           - automod    [bulbbot.AutomodService]     -
           - analytics  [bulbbot.ScienceService]     -

        */
    }
}

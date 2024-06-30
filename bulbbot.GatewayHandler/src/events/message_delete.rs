use crate::injector::ReqwestInjector;
use crate::{handler::Handler, models::event_type::EventType};
use opentelemetry::{global, Context};
use opentelemetry::{global::BoxedSpan, trace::Span};
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, GuildId, MessageId};
use tracing::{debug, error};

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteEventContent {
    pub channel_id: ChannelId,
    pub deleted_message_id: MessageId,
    pub guild_id: Option<GuildId>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageDeleteEvent {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: u64,
    pub content: MessageDeleteEventContent,
}

impl Handler {
    pub async fn handle_message_delete_event(
        &self,
        event_data: &str,
        span: &mut BoxedSpan,
        cx: &Context,
    ) -> bool {
        let message: MessageDeleteEvent =
            serde_json::from_str(event_data).expect("Failed to parse data as message event");

        let url = format!("{}/deletemessages", self.get_url(&message.event));
        span.add_event(format!("Sending post request to {}", url), Vec::new());

        let mut request = self.client.post(&url).json(&message).build().unwrap();
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(
                &cx,
                &mut ReqwestInjector {
                    headers: request.headers_mut(),
                },
            )
        });
        let response = self.client.execute(request).await;

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
    }
}

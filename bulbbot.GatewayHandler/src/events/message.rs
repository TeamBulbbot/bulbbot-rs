use crate::injector::ReqwestInjector;
use crate::{handler::Handler, models::event_type::EventType};
use opentelemetry::Context;
use opentelemetry::{
    global::{self, BoxedSpan},
    trace::Span,
};
use serde::{Deserialize, Serialize};
use serenity::model::channel::Message;
use tracing::{debug, error};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEvent {
    pub event: EventType,
    pub shard_id: u32,
    pub timestamp: i64,
    pub content: Message,
}

impl Handler {
    pub async fn handle_message_event(
        &self,
        event_data: &str,
        span: &mut BoxedSpan,
        cx: &Context,
    ) -> bool {
        let message: MessageEvent =
            serde_json::from_str(event_data).expect("Failed to parse data as message event");

        let url = format!("{}/messages", self.get_url(&message.event));
        span.add_event(format!("Sending post request to {}", url), Vec::new());

        let mut request = self.client.post(&url).json(&message).build().unwrap();
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(
                cx,
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
                let error_msg = format!("Request to {} errored with {:#?}", url, err);
                error!(error_msg);
                false
            }
        }
    }
}

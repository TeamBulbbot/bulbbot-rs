use crate::handler::Handler;
use common::telemetry::injector_reqwest::ReqwestInjector;
use models::message::message_update_event::MessageUpdateEvent;
use opentelemetry::{global, Context};
use opentelemetry::{global::BoxedSpan, trace::Span};
use tracing::{debug, error};

impl Handler {
    pub async fn handle_mesage_update_event(
        &self,
        event_data: &str,
        span: &mut BoxedSpan,
        cx: &Context,
    ) -> bool {
        let message: MessageUpdateEvent =
            serde_json::from_str(event_data).expect("Failed to parse data as message update event");

        let url = format!("{}/updatemessage", self.get_url(&message.event));
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
                error!("Request to {} errored with {:#?}", url, err);
                false
            }
        }
    }
}

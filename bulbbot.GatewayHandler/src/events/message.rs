use crate::handler::Handler;
use common::telemetry::injector_reqwest::ReqwestInjector;
use models::message::message_event::MessageEvent;
use opentelemetry::Context;
use opentelemetry::{
    global::{self, BoxedSpan},
    trace::Span,
};
use tracing::{debug, error};

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

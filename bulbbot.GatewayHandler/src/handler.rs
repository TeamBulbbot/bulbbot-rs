use crate::models::event::Event;
use crate::models::event_type::EventType;
use opentelemetry::global::BoxedSpan;
use opentelemetry::trace::{Span, Tracer, TracerProvider};
use opentelemetry::{global, Context};
use reqwest::Client;

#[derive(Debug)]
pub struct Handler {
    pub client: Client,
}

impl Handler {
    pub fn init() -> Handler {
        let user_agent = format!(
            "bulbbot-{}({})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        Handler {
            client: Client::builder()
                .user_agent(user_agent)
                .build()
                .expect("Failed to create reqwest client"),
        }
    }

    pub fn get_url(&self, event_type: &EventType) -> String {
        match event_type {
            EventType::Message => String::from("http://localhost:3521/api"),
            EventType::MessageUpdate => String::from("http://localhost:3521/api"),
            EventType::MessageDelete => String::from("http://localhost:3521/api"),
            _ => String::new(),
        }
    }

    pub async fn handle(&self, event_data: &str, cx: &Context) -> bool {
        let event: Event = serde_json::from_str(event_data).expect("Failed to parse data as event");

        let tracer_provider = global::tracer_provider();

        let tracer = tracer_provider
            .tracer_builder(format!("{:#?}", event.event))
            .with_version(env!("CARGO_PKG_VERSION"))
            .build();

        let mut span = tracer.start_with_context(format!("{:#?}", event.event), cx);
        span.add_event("Starting to handle the event", Vec::new());

        let event_response = match event.event {
            EventType::Message => self.handle_message_event(event_data, &mut span, cx).await,
            EventType::MessageDelete => self.handle_message_delete_event(event_data).await,
            _ => false, /*
                        EventType::MessageUpdate => todo!(),
                        EventType::MessageDelete => todo!(),
                        EventType::GuildMemberAddition => todo!(),
                        EventType::GuildMemberRemoval => todo!(),
                         */
        };

        span.add_event("Handled the event", Vec::new());

        event_response
    }
}

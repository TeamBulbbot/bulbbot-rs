use lapin::types::{AMQPValue, FieldTable};
use opentelemetry::propagation::Extractor;

pub struct RabbitMqExtractor<'a>(pub &'a FieldTable);

impl<'a> Extractor for RabbitMqExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.inner().get(key).and_then(|header_value| {
            if let AMQPValue::LongString(header_value) = header_value {
                std::str::from_utf8(header_value.as_bytes()).ok()
            } else {
                None
            }
        })
    }

    fn keys(&self) -> Vec<&str> {
        self.0.inner().keys().map(|k| k.as_str()).collect()
    }
}

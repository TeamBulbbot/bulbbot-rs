use lapin::types::{AMQPValue, FieldTable};
use opentelemetry::propagation::Injector;
use reqwest::header::HeaderMap;
pub struct ReqwestInjector<'a> {
    pub headers: &'a mut HeaderMap,
}

impl<'a> Injector for ReqwestInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        if let Ok(name) = reqwest::header::HeaderName::from_bytes(key.as_bytes()) {
            if let Ok(val) = reqwest::header::HeaderValue::from_str(&value) {
                self.headers.insert(name, val);
            }
        }
    }
}

pub struct RabbitMqInjector<'a>(pub &'a mut FieldTable);

impl<'a> Injector for RabbitMqInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        self.0
            .insert(key.into(), AMQPValue::LongString(value.into()));
    }
}

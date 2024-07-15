use lapin::types::{AMQPValue, FieldTable};
use opentelemetry::propagation::Injector;

pub struct RabbitMqInjector<'a>(pub &'a mut FieldTable);

impl<'a> Injector for RabbitMqInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        self.0
            .insert(key.into(), AMQPValue::LongString(value.into()));
    }
}

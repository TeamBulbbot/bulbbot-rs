use actix_web::http::header::HeaderMap;
use opentelemetry::propagation::Extractor;

pub struct ActixWebExtractor<'a> {
    pub headers: &'a HeaderMap,
}

impl<'a> Extractor for ActixWebExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).and_then(|value| value.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.headers.keys().map(|k| k.as_str()).collect()
    }
}

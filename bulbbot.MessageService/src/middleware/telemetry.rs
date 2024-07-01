use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use opentelemetry::{
    global::{self, ObjectSafeSpan},
    trace::{Tracer, TracerProvider},
    KeyValue,
};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use crate::extractor::ActixWebExtractor;

pub struct Telemetry;

impl<S, B> Transform<S, ServiceRequest> for Telemetry
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TelemetryMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TelemetryMiddleware { service }))
    }
}

pub struct TelemetryMiddleware<S> {
    service: S,
}

type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

impl<S, B> Service<ServiceRequest> for TelemetryMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let request = req.request();
        let cx = global::get_text_map_propagator(|propagator| {
            propagator.extract(&mut ActixWebExtractor {
                headers: &mut request.headers(),
            })
        });

        let tracer_provider = global::tracer_provider();
        let name = format!("{} {}", req.method().to_string().to_uppercase(), req.path());

        let tracer = tracer_provider
            .tracer_builder(name.clone())
            .with_version(env!("CARGO_PKG_VERSION"))
            .build();

        let mut span = tracer.start_with_context(name, &cx);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            span.set_attribute(KeyValue::new("http_status", res.status().to_string()));

            span.end();
            Ok(res)
        })
    }
}

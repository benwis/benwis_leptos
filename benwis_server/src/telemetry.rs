use anyhow::Result;
use opentelemetry::sdk::{
    trace::{self, RandomIdGenerator, Sampler},
    Resource,
};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use std::collections::HashMap;
use std::future::Future;
use tokio::task::{spawn, spawn_blocking, JoinHandle};
//use tonic::metadata::*;
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
//use std::{fs::File, io::Read};

pub struct TracingSettings {
    pub honeycomb_team: Option<String>,
    pub honeycomb_dataset: Option<String>,
    pub honeycomb_service_name: Option<String>,
}

/// Configure Honeycomb tracer
pub async fn otel_layer<S>(
    conf: &TracingSettings,
) -> Result<Option<OpenTelemetryLayer<S, opentelemetry::sdk::trace::Tracer>>>
where
    S: tracing::Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
{
    match (
        &conf.honeycomb_team,
        &conf.honeycomb_dataset,
        &conf.honeycomb_service_name,
    ) {
        (Some(honeycomb_team), Some(honeycomb_dataset), Some(honeycomb_service_name)) => {
            let client = reqwest::Client::builder()
                .use_rustls_tls()
                .build()
                .expect("Failed to build client!");
            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(
                    opentelemetry_otlp::new_exporter()
                        .http()
                        .with_endpoint("https://api.honeycomb.io/v1/traces")
                        .with_http_client(client)
                        .with_headers(HashMap::from([
                            ("x-honeycomb-dataset".into(), honeycomb_dataset.parse()?),
                            ("x-honeycomb-team".into(), honeycomb_team.parse()?),
                        ]))
                        .with_timeout(std::time::Duration::from_secs(2)),
                )
                .with_trace_config(
                    trace::config()
                        .with_sampler(Sampler::AlwaysOn)
                        .with_id_generator(RandomIdGenerator::default())
                        //.with_max_events_per_span(64)
                        //.with_max_attributes_per_span(16)
                        .with_resource(Resource::new(vec![KeyValue::new(
                            "service.name",
                            honeycomb_service_name.clone(),
                        )])),
                )
                .install_batch(opentelemetry::runtime::Tokio)?;
            Ok(Some(tracing_opentelemetry::layer().with_tracer(tracer)))
        }
        _ => Ok(None),
    }
}

// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
pub async fn get_subscriber_with_tracing<Sink>(
    name: String,
    conf: &TracingSettings,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let telemetry_layer = otel_layer(conf)
        .await
        .map_err(|e| {
            println!("Error: {}", e);
            e
        })
        .unwrap()
        .unwrap();
    print!(
        "Setting up Honeycomb logging for {:?} at {:?}",
        &conf.honeycomb_dataset, &conf.honeycomb_team
    );
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|e| {
        print!("RUST_LOG not set! Defaulting to {}: {:#?}", e, env_filter);
        EnvFilter::new(env_filter)
    });
    let _formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(telemetry_layer) // publish to honeycomb backend
        .with(JsonStorageLayer)
        .with(tracing_logfmt::layer())
    // .with(formatting_layer)
}
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    // Check out https://doc.rust-lang.org/nomicon/hrtb.html
    // for more details.
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    spawn_blocking(move || current_span.in_scope(f))
}

pub fn spawn_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static + Future<Output = R>,
{
    let current_span = tracing::Span::current();
    spawn(current_span.in_scope(f))
}

use tracing::{info, metadata::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn init() {
    // A tracing Layer that exports spans to Jaeger
    opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("simple-rust")
        .install_simple()
        .expect("Tracer creation failed");
    let otel_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(LevelFilter::INFO);

    // A tracing Layer that prints traces to stdout
    let stdout_layer = tracing_subscriber::fmt::Layer::default()
        .compact()
        .without_time()
        .with_ansi(true)
        .with_file(false)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(otel_layer)
        .try_init()
        .expect("Observability stack failed to initialize");
}

pub fn ship() {
    info!("Shipping traces");
    opentelemetry::global::shutdown_tracer_provider();
}

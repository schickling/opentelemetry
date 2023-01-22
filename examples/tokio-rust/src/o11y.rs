use anyhow::Result;
use tracing::{info, metadata::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

pub fn init() -> Result<()> {
    let subscriber = tracing_subscriber::registry();

    // A tracing Layer that prints traces to stdout
    let stdout_layer = tracing_subscriber::fmt::Layer::default()
        .compact()
        .without_time()
        .with_ansi(true)
        .with_file(false)
        .with_filter(LevelFilter::INFO);
    // Attach the stdout_layer to our subscriber. You may want to do this conditionally.
    let subscriber = subscriber.with(stdout_layer);

    // Attach an OTel layer during development
    #[cfg(feature = "otel")]
    let subscriber = {
        opentelemetry::global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name("tokio-rust")
            .install_simple()?;

        let otel_layer = tracing_opentelemetry::layer()
            .with_tracer(tracer)
            .with_filter(LevelFilter::INFO);

        subscriber.with(otel_layer)
    };

    subscriber.try_init()?;
    Ok(())
}

pub fn ship() {
    #[cfg(feature = "otel")]
    {
        info!("Shipping traces");
        opentelemetry::global::shutdown_tracer_provider();
    }
}

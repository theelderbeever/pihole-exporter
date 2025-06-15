use axum::{Router, routing::get};
use clap::Parser;
use pihole_exporter::{Args, PiholeCollector, health_handler, metrics_handler};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Args::parse();

    info!("Starting Pi-hole Prometheus exporter");
    info!("Pi-hole host: {}", args.pihole);

    // Create Pi-hole collector
    let collector = Arc::new(PiholeCollector::new(args.pihole, args.tls, args.password).await?);

    // Build the application router
    let app = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/healthz", get(health_handler))
        .with_state(collector)
        .layer(TraceLayer::new_for_http());

    // Start the server
    let listener = TcpListener::bind(format!("{}:{}", args.host, args.port)).await?;
    info!("Server listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}

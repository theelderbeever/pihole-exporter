use crate::collector::PiholeCollector;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tracing::warn;

/// Handler for the /metrics endpoint
///
/// Updates Pi-hole metrics and returns them in Prometheus format
pub async fn metrics_handler(State(collector): State<Arc<PiholeCollector>>) -> Response {
    match collector.update_metrics().await {
        Ok(()) => match collector.encode_metrics() {
            Ok(metrics) => (
                StatusCode::OK,
                [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
                metrics,
            )
                .into_response(),
            Err(e) => {
                warn!("Failed to encode metrics: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to encode metrics",
                )
                    .into_response()
            }
        },
        Err(e) => {
            warn!("Failed to collect metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to collect metrics",
            )
                .into_response()
        }
    }
}

/// Handler for the /health endpoint
///
/// Simple health check that returns OK
pub async fn health_handler() -> Response {
    (StatusCode::OK, "OK").into_response()
}

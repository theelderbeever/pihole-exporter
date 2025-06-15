//! Pi-hole Prometheus Exporter Library
//!
//! This library provides functionality to collect metrics from Pi-hole instances
//! and expose them in Prometheus format.

pub mod api;
pub mod args;
pub mod collector;
pub mod handlers;
pub mod metrics;

// Re-export commonly used types
pub use args::Args;
pub use collector::PiholeCollector;
pub use handlers::{health_handler, metrics_handler};
pub use metrics::PiholeMetrics;

use std::error::Error;

/// Result type used throughout the library
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

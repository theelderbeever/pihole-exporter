use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{family::Family, gauge::Gauge},
    registry::Registry,
};

/// Labels for query type metrics
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct QueryTypeLabels {
    pub query_type: String,
}

/// Labels for query status metrics
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct QueryStatusLabels {
    pub query_status: String,
}

/// Labels for reply type metrics
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ReplyTypeLabels {
    pub reply_type: String,
}

/// Labels for category-based metrics
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct CategoryLabels {
    pub category: String,
}

/// Labels for upstream server metrics
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct UpstreamLabels {
    pub ip: String,
    pub name: String,
    pub port: String,
}

/// Labels for client metrics
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ClientLabels {
    pub query_client: String,
}

/// Labels for upstream count metrics
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct UpstreamCountLabels {
    pub query_upstream: String,
}

/// Container for all Pi-hole Prometheus metrics
#[derive(Debug)]
pub struct PiholeMetrics {
    // 24h metrics
    pub query_by_type: Family<QueryTypeLabels, Gauge>,
    pub query_by_status: Family<QueryStatusLabels, Gauge>,
    pub query_replies: Family<ReplyTypeLabels, Gauge>,
    pub query_count: Family<CategoryLabels, Gauge>,
    pub client_count: Family<CategoryLabels, Gauge>,
    pub domains_being_blocked: Gauge,
    pub query_upstream_count: Family<UpstreamLabels, Gauge>,

    // 1m metrics
    pub query_type_1m: Family<QueryTypeLabels, Gauge>,
    pub query_status_1m: Family<QueryStatusLabels, Gauge>,
    pub query_reply_1m: Family<ReplyTypeLabels, Gauge>,
    pub query_client_1m: Family<ClientLabels, Gauge>,
    pub query_upstream_1m: Family<UpstreamCountLabels, Gauge>,
}

impl PiholeMetrics {
    /// Create a new instance of PiholeMetrics
    pub fn new() -> Self {
        Self {
            query_by_type: Family::default(),
            query_by_status: Family::default(),
            query_replies: Family::default(),
            query_count: Family::default(),
            client_count: Family::default(),
            domains_being_blocked: Gauge::default(),
            query_upstream_count: Family::default(),
            query_type_1m: Family::default(),
            query_status_1m: Family::default(),
            query_reply_1m: Family::default(),
            query_client_1m: Family::default(),
            query_upstream_1m: Family::default(),
        }
    }

    /// Register all metrics with the provided registry
    pub fn register(&self, registry: &mut Registry) {
        registry.register(
            "pihole_query_by_type",
            "Count of queries by type (24h)",
            self.query_by_type.clone(),
        );
        registry.register(
            "pihole_query_by_status",
            "Count of queries by status over 24h",
            self.query_by_status.clone(),
        );
        registry.register(
            "pihole_query_replies",
            "Count of replies by type over 24h",
            self.query_replies.clone(),
        );
        registry.register(
            "pihole_query_count",
            "Query counts by category, 24h",
            self.query_count.clone(),
        );
        registry.register(
            "pihole_client_count",
            "Total/active client counts",
            self.client_count.clone(),
        );
        registry.register(
            "pihole_domains_being_blocked",
            "Number of domains on current blocklist",
            self.domains_being_blocked.clone(),
        );
        registry.register(
            "pihole_query_upstream_count",
            "Total query upstream counts (24h)",
            self.query_upstream_count.clone(),
        );
        registry.register(
            "pihole_query_type_1m",
            "Count of query types (last whole 1m)",
            self.query_type_1m.clone(),
        );
        registry.register(
            "pihole_query_status_1m",
            "Count of query status (last whole 1m)",
            self.query_status_1m.clone(),
        );
        registry.register(
            "pihole_query_reply_1m",
            "Count of query reply types (last whole 1m)",
            self.query_reply_1m.clone(),
        );
        registry.register(
            "pihole_query_client_1m",
            "Count of query clients (last whole 1m)",
            self.query_client_1m.clone(),
        );
        registry.register(
            "pihole_query_upstream_1m",
            "Count of query upstream destinations (last whole 1m)",
            self.query_upstream_1m.clone(),
        );
    }
}

impl Default for PiholeMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = PiholeMetrics::new();
        let mut registry = Registry::default();
        metrics.register(&mut registry);

        // Test that metrics can be created and registered without panicking
        assert_eq!(metrics.domains_being_blocked.get(), 0);
    }
}

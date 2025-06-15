use crate::{
    Result,
    api::{AuthRequest, AuthResponse, QueriesResponse, StatsResponse, UpstreamsResponse},
    metrics::{
        CategoryLabels, ClientLabels, PiholeMetrics, QueryStatusLabels, QueryTypeLabels,
        ReplyTypeLabels, UpstreamCountLabels, UpstreamLabels,
    },
};
use prometheus_client::{encoding::text::encode, registry::Registry};
use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Pi-hole collector that fetches metrics from Pi-hole API and updates Prometheus metrics
#[derive(Debug)]
pub struct PiholeCollector {
    pub base: String,
    pub client: Client,
    pub sid: Option<String>,
    pub metrics: PiholeMetrics,
    pub registry: Arc<Mutex<Registry>>,
}

impl PiholeCollector {
    /// Create a new PiholeCollector instance
    pub async fn new(host: String, tls: bool, key: Option<SecretString>) -> Result<Self> {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(30))
            .build()?;

        let base = format!("{}://{host}", if tls { "https" } else { "http" });

        let sid = if let Some(key) = key {
            Some(Self::get_sid(&client, &base, key.expose_secret()).await?)
        } else {
            None
        };

        let metrics = PiholeMetrics::new();
        let mut registry = Registry::default();
        metrics.register(&mut registry);

        Ok(Self {
            base,
            client,
            sid,
            metrics,
            registry: Arc::new(Mutex::new(registry)),
        })
    }

    /// Authenticate with Pi-hole and get session ID
    async fn get_sid(client: &Client, base: &str, key: &str) -> Result<String> {
        let auth_url = format!("{base}/api/auth");
        let auth_request = AuthRequest {
            password: key.to_string(),
        };

        let response = client
            .post(&auth_url)
            .header("accept", "application/json")
            .header("content-type", "application/json")
            .json(&auth_request)
            .send()
            .await?;

        let auth_response: AuthResponse = response.json().await?;
        Ok(auth_response.session.sid)
    }

    /// Make an API call to Pi-hole
    async fn get_api_call(&self, api_path: &str) -> Result<Value> {
        let url = format!("{}/api/{}", self.base, api_path);
        let mut request = self.client.get(&url).header("accept", "application/json");

        if let Some(ref sid) = self.sid {
            request = request.header("sid", sid);
        }

        let response = request.send().await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Update all metrics by fetching data from Pi-hole API
    pub async fn update_metrics(&self) -> Result<()> {
        // Get summary stats
        let summary_json = self.get_api_call("stats/summary").await?;
        let summary: StatsResponse = serde_json::from_value(summary_json)?;

        // Update 24h query type metrics
        for (query_type, count) in &summary.queries.types {
            self.metrics
                .query_by_type
                .get_or_create(&QueryTypeLabels {
                    query_type: query_type.clone(),
                })
                .set(*count as i64);
        }

        // Update 24h query status metrics
        for (status, count) in &summary.queries.status {
            self.metrics
                .query_by_status
                .get_or_create(&QueryStatusLabels {
                    query_status: status.clone(),
                })
                .set(*count as i64);
        }

        // Update 24h reply type metrics
        for (reply_type, count) in &summary.queries.replies {
            self.metrics
                .query_replies
                .get_or_create(&ReplyTypeLabels {
                    reply_type: reply_type.clone(),
                })
                .set(*count as i64);
        }

        // Update total counts
        self.metrics
            .query_count
            .get_or_create(&CategoryLabels {
                category: "total".to_string(),
            })
            .set(summary.queries.total as i64);

        self.metrics
            .query_count
            .get_or_create(&CategoryLabels {
                category: "blocked".to_string(),
            })
            .set(summary.queries.blocked as i64);

        self.metrics
            .query_count
            .get_or_create(&CategoryLabels {
                category: "unique".to_string(),
            })
            .set(summary.queries.unique_domains as i64);

        self.metrics
            .query_count
            .get_or_create(&CategoryLabels {
                category: "forwarded".to_string(),
            })
            .set(summary.queries.forwarded as i64);

        self.metrics
            .query_count
            .get_or_create(&CategoryLabels {
                category: "cached".to_string(),
            })
            .set(summary.queries.cached as i64);

        // Update client counts
        self.metrics
            .client_count
            .get_or_create(&CategoryLabels {
                category: "active".to_string(),
            })
            .set(summary.clients.active as i64);

        self.metrics
            .client_count
            .get_or_create(&CategoryLabels {
                category: "total".to_string(),
            })
            .set(summary.clients.total as i64);

        // Update domains being blocked
        self.metrics
            .domains_being_blocked
            .set(summary.gravity.domains_being_blocked as i64);

        // Get upstream stats
        let upstreams_json = self.get_api_call("stats/upstreams").await?;
        let upstreams: UpstreamsResponse = serde_json::from_value(upstreams_json)?;

        for upstream in &upstreams.upstreams {
            self.metrics
                .query_upstream_count
                .get_or_create(&UpstreamLabels {
                    ip: upstream.ip.clone(),
                    name: upstream.name.clone(),
                    port: upstream.port.to_string(),
                })
                .set(upstream.count as i64);
        }

        // Get 1-minute stats
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let last_min = (now / 60) * 60;
        let min_before = last_min - 60;

        let queries_json = self
            .get_api_call(&format!(
                "queries?from={}&until={}&length=1000000",
                min_before, last_min
            ))
            .await?;

        let queries_response: QueriesResponse = serde_json::from_value(queries_json)?;

        let mut type_cnt: HashMap<String, u64> = HashMap::new();
        let mut status_cnt: HashMap<String, u64> = HashMap::new();
        let mut reply_cnt: HashMap<String, u64> = HashMap::new();
        let mut client_cnt: HashMap<String, u64> = HashMap::new();
        let mut upstream_cnt: HashMap<String, u64> = HashMap::new();

        // Process queries for 1-minute metrics
        for query in &queries_response.queries {
            *type_cnt.entry(query.query_type.clone()).or_insert(0) += 1;
            *status_cnt.entry(query.status.clone()).or_insert(0) += 1;
            *reply_cnt.entry(query.reply.reply_type.clone()).or_insert(0) += 1;
            *client_cnt.entry(query.client.ip.clone()).or_insert(0) += 1;

            let upstream = match &query.upstream {
                Some(upstream) => upstream.clone(),
                None => match query.status.as_str() {
                    "GRAVITY" | "CACHE" | "SPECIAL_DOMAIN" => format!("None-{}", query.status),
                    _ => "None-OTHER".to_string(),
                },
            };
            *upstream_cnt.entry(upstream).or_insert(0) += 1;
        }

        // Update 1-minute metrics
        for (query_type, count) in &type_cnt {
            self.metrics
                .query_type_1m
                .get_or_create(&QueryTypeLabels {
                    query_type: query_type.clone(),
                })
                .set(*count as i64);
        }

        for (status, count) in &status_cnt {
            self.metrics
                .query_status_1m
                .get_or_create(&QueryStatusLabels {
                    query_status: status.clone(),
                })
                .set(*count as i64);
        }

        for (reply_type, count) in &reply_cnt {
            self.metrics
                .query_reply_1m
                .get_or_create(&ReplyTypeLabels {
                    reply_type: reply_type.clone(),
                })
                .set(*count as i64);
        }

        for (client, count) in &client_cnt {
            self.metrics
                .query_client_1m
                .get_or_create(&ClientLabels {
                    query_client: client.clone(),
                })
                .set(*count as i64);
        }

        for (upstream, count) in &upstream_cnt {
            self.metrics
                .query_upstream_1m
                .get_or_create(&UpstreamCountLabels {
                    query_upstream: upstream.clone(),
                })
                .set(*count as i64);
        }

        Ok(())
    }

    /// Encode metrics to Prometheus format
    pub fn encode_metrics(&self) -> Result<String> {
        let mut buffer = String::new();
        let registry = self.registry.lock().unwrap();
        encode(&mut buffer, &registry)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pihole_collector_creation() {
        let collector = PiholeCollector::new("localhost".to_string(), false, None).await;
        assert!(collector.is_ok());
    }
}

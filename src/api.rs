use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication response from Pi-hole API
#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub session: SessionInfo,
}

/// Session information after authentication
#[derive(Debug, Deserialize)]
pub struct SessionInfo {
    pub sid: String,
}

/// Authentication request payload
#[derive(Debug, Serialize)]
pub struct AuthRequest {
    pub password: String,
}

/// Main statistics response from Pi-hole API
#[derive(Debug, Deserialize)]
pub struct StatsResponse {
    pub queries: QueryStats,
    pub clients: ClientStats,
    pub gravity: GravityStats,
}

/// Query statistics from Pi-hole
#[derive(Debug, Deserialize)]
pub struct QueryStats {
    pub types: HashMap<String, u64>,
    pub status: HashMap<String, u64>,
    pub replies: HashMap<String, u64>,
    pub total: u64,
    pub blocked: u64,
    pub unique_domains: u64,
    pub forwarded: u64,
    pub cached: u64,
}

/// Client statistics from Pi-hole
#[derive(Debug, Deserialize)]
pub struct ClientStats {
    pub active: u64,
    pub total: u64,
}

/// Gravity (blocklist) statistics from Pi-hole
#[derive(Debug, Deserialize)]
pub struct GravityStats {
    pub domains_being_blocked: u64,
}

/// Upstream servers response from Pi-hole API
#[derive(Debug, Deserialize)]
pub struct UpstreamsResponse {
    pub upstreams: Vec<UpstreamInfo>,
}

/// Information about an upstream DNS server
#[derive(Debug, Deserialize)]
pub struct UpstreamInfo {
    pub ip: String,
    pub name: String,
    pub port: i16,
    pub count: u64,
}

/// Queries response from Pi-hole API
#[derive(Debug, Deserialize)]
pub struct QueriesResponse {
    pub queries: Vec<QueryInfo>,
}

/// Information about a single DNS query
#[derive(Debug, Deserialize)]
pub struct QueryInfo {
    #[serde(rename = "type")]
    pub query_type: String,
    pub status: String,
    pub reply: ReplyInfo,
    pub client: ClientInfo,
    pub upstream: Option<String>,
}

/// Reply information for a DNS query
#[derive(Debug, Deserialize)]
pub struct ReplyInfo {
    #[serde(rename = "type")]
    pub reply_type: String,
}

/// Client information for a DNS query
#[derive(Debug, Deserialize)]
pub struct ClientInfo {
    pub ip: String,
}

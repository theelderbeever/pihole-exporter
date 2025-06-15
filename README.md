# Pi-hole Prometheus Exporter

Prometheus exporter for Pi-hole metrics written in Rust. Exposes Pi-hole statistics in Prometheus format for monitoring and alerting.

## Features

- Exposes Pi-hole query statistics, client data, and gravity metrics
- Supports Pi-hole authentication via password/token
- HTTP and HTTPS support for Pi-hole communication
- Health check endpoint
- Configurable via command line arguments or environment variables
- Systemd service integration

## Installation

### From GitHub Releases

Download the latest release for your platform:

```bash
# Linux aarch64
curl -L https://github.com/USERNAME/pihole-prometheus/releases/latest/download/pihole-exporter-linux-aarch64.tar.gz | tar xz
sudo mv pihole-exporter /usr/local/bin/
```

### From Source

```bash
git clone <repository-url>
cd pihole-prometheus
cargo build --release
sudo cp target/release/pihole-exporter /usr/local/bin/
```

## Usage

### Basic Usage

```bash
# Local Pi-hole on default port
pihole-exporter --pihole localhost

# Remote Pi-hole with authentication
pihole-exporter --pihole 192.168.1.100 --password your-pihole-password

# Custom exporter port and host
pihole-exporter --host 0.0.0.0 --port 9617 --pihole 192.168.1.100
```

### Command Line Options

| Option | Default | Description |
|--------|---------|-------------|
| `--host` | `127.0.0.1` | IP address to bind exporter |
| `--port` | `3141` | Port to expose metrics |
| `--pihole` | `localhost` | Pi-hole hostname/IP |
| `--tls` | `false` | Use HTTPS for Pi-hole communication |
| `--password` | None | Pi-hole authentication password |

### Environment Variables

```bash
export PIHOLE_EXPORTER__PIHOLE_PASSWORD="your-password"
```

## Systemd Service

Install as a system service:

```bash
sudo cp pihole-exporter.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable pihole-exporter
sudo systemctl start pihole-exporter
```

Edit the service file to configure your Pi-hole password:

```ini
[Environment]
PIHOLE_EXPORTER__PIHOLE_PASSWORD="your-pihole-password"
```

## Endpoints

- `/metrics` - Prometheus metrics
- `/healthz` - Health check

## Example Prometheus Configuration

```yaml
scrape_configs:
  - job_name: 'pihole'
    static_configs:
      - targets: ['localhost:3141']
    scrape_interval: 30s
```

## Metrics Exposed

- `pihole_queries_total` - Total DNS queries
- `pihole_queries_blocked_total` - Total blocked queries
- `pihole_queries_cached_total` - Total cached queries
- `pihole_queries_forwarded_total` - Total forwarded queries
- `pihole_unique_domains_total` - Total unique domains
- `pihole_clients_total` - Active clients
- `pihole_gravity_domains_total` - Domains in gravity database
- Query type, status, and reply type breakdowns with labels

## Requirements

- Rust 1.70+
- Pi-hole instance accessible via HTTP/HTTPS
- Network access between exporter and Pi-hole
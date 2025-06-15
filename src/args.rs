use clap::Parser;
use secrecy::SecretString;

/// Command line arguments for the Pi-hole Prometheus exporter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// IP for exporter instance. Usually 127.0.0.1 or 0.0.0.0
    #[arg(
        long,
        default_value = "127.0.0.1",
        env = "PIHOLE_EXPORTER__EXPORTER_HOST"
    )]
    pub host: String,

    /// Port to expose for scraping
    #[arg(
        short,
        long,
        default_value_t = 3141,
        env = "PIHOLE_EXPORTER__EXPORTER_PORT"
    )]
    pub port: u16,

    /// Base url/port of Pi-hole instance
    #[arg(
        long,
        default_value = "localhost",
        env = "PIHOLE_EXPORTER__PIHOLE_HOST"
    )]
    pub pihole: String,

    /// Use https for pihole communication
    #[arg(long, env = "PIHOLE_EXPORTER__PIHOLE_TLS")]
    pub tls: bool,

    /// Authentication token (if required)
    #[arg(short = 'P', long, env = "PIHOLE_EXPORTER__PIHOLE_PASSWORD")]
    pub password: Option<SecretString>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        let args = Args::parse_from(["pihole-exporter", "--host", "192.168.1.100", "-p", "80"]);
        assert_eq!(args.host, "192.168.1.1");
        assert_eq!(args.port, 8080);
        assert!(args.password.is_none());
    }
}

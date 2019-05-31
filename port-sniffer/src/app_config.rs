use config::{ConfigError, Config, File, Environment};
use std::env;
use std::net::{IpAddr, TcpStream};

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Sniffer {
    pub ipaddr: IpAddr,
    pub num_threads: u16,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    pub sniffer: Sniffer
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();

        // Start with merging the default configuration
        cfg.merge(File::with_name("config/default"))?;

        let env = env::var("RUN_MODE").unwrap_or("development".into());
        cfg.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Merge with the local file
        cfg.merge(File::with_name("config/local").required(false))?;

        cfg.try_into()
    }
}
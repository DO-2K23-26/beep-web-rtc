use std::{error::Error, net::IpAddr};

pub struct Config {
    pub app_secret: String,
    pub value_port_min: u16,
    pub value_port_max: u16,
    pub signal_port: u16,
    pub ip_endpoint: IpAddr,
}

impl Config {
    pub fn new(
        app_secret: String,
        value_port_min: u16,
        value_port_max: u16,
        signal_port: u16,
        ip_endpoint: IpAddr,
    ) -> Self {
        Config {
            app_secret,
            value_port_min,
            value_port_max,
            signal_port,
            ip_endpoint,
        }
    }
}

impl Config {
    pub fn from_env(
        app_secret_key: String,
        value_port_min_key: String,
        value_port_max_key: String,
        signal_port_key: String,
        ip_endpoint_key: String,
    ) -> Result<Self, Box<dyn Error>> {
        let app_secret = std::env::var(app_secret_key)?;
        let value_port_min: u16 = std::env::var(value_port_min_key)?.parse()?;
        let value_port_max: u16 = std::env::var(value_port_max_key)?.parse()?;
        let signal_port: u16 = std::env::var(signal_port_key)?.parse()?;
        let ip_endpoint: IpAddr = std::env::var(ip_endpoint_key)?.parse()?;
        let config = Config {
            app_secret,
            value_port_min,
            value_port_max,
            signal_port,
            ip_endpoint,
        };
        Ok(config)
    }
}

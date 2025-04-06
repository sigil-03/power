use crate::tasmota::{PowerStatusData, StatusResponse, TasmotaInterface, TasmotaInterfaceConfig};
use reqwest::Client;
use serde::Deserialize;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("toml parsing error")]
    ParseError(#[from] toml::de::Error),
    #[error("request error")]
    RequestError(#[from] reqwest::Error),
    #[error("JSON Parse error")]
    JsonParseError(#[from] serde_json::Error),
}

pub trait Monitoring {
    async fn get_power(&self) -> Result<isize, Error>;
}

#[derive(Deserialize)]
pub struct MonitorConfig {
    targets: Vec<TasmotaInterfaceConfig>,
}
impl MonitorConfig {
    fn print(&self) {
        for t in &self.targets {
            t.print();
        }
    }
}

pub struct Monitor {
    targets: Vec<TasmotaInterface>,
    client: Client,
}

impl Monitor {
    pub fn new_from_file(config_file: &str) -> Result<Self, Error> {
        let config_str = fs::read_to_string(config_file)?;
        let config: MonitorConfig = toml::from_str(&config_str)?;
        config.print();

        Ok(Self {
            targets: Monitor::load_targets(&config.targets),
            client: Client::new(),
        })
    }

    pub fn load_targets(targets: &Vec<TasmotaInterfaceConfig>) -> Vec<TasmotaInterface> {
        let mut v = Vec::new();
        for target in targets {
            v.push(TasmotaInterface::new(target.clone()));
        }
        v
    }

    pub async fn get_power(&self) -> Result<(), Error> {
        for target in &self.targets {
            if let Ok(res) = target.get_power().await {
                println!("POWER: {}W", res);
            }
        }
        Ok(())
    }
}

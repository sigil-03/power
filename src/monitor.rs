use crate::tasmota::{PowerStatusData, StatusResponse};
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

#[derive(Deserialize)]
pub struct MonitorConfig {
    target: Vec<String>,
}
impl MonitorConfig {
    fn print(&self) {
        for t in &self.target {
            println!("* {}", t);
        }
    }
}

pub struct Monitor {
    config: MonitorConfig,
    client: Client,
}

impl Monitor {
    pub fn new_from_file(config_file: &str) -> Result<Self, Error> {
        let config_str = fs::read_to_string(config_file)?;
        let config: MonitorConfig = toml::from_str(&config_str)?;
        // config.print();
        Ok(Self {
            config,
            client: Client::new(),
        })
    }

    pub async fn get_power(&self) -> Result<(), Error> {
        let ip = &self.config.target[0];
        let res = self
            .client
            .get(format!("http://{ip}/cm?cmnd=Status%208"))
            .send()
            .await?
            .text()
            .await?;

        // println!("body = {res:?}");
        let data: StatusResponse = serde_json::from_str(&res)?;
        println!("POWER: {}W", data.status.energy.power);
        Ok(())
    }
}

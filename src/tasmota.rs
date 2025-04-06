use reqwest::Client;
use serde::Deserialize;

use crate::monitor::Error;
use crate::monitor::Monitoring;

#[derive(Deserialize)]
pub struct EnergyData {
    #[serde(rename = "Power")]
    pub power: isize,
}

#[derive(Deserialize)]
pub struct PowerStatusData {
    // #[serde(rename = "Time")]
    // pub time: String,
    #[serde(rename = "ENERGY")]
    pub energy: EnergyData,
}

#[derive(Deserialize)]
pub struct StatusResponse {
    #[serde(rename = "StatusSNS")]
    pub status: PowerStatusData,
}

#[derive(Deserialize, Clone)]
pub struct TasmotaInterfaceConfig {
    name: String,
    target: String,
}

impl TasmotaInterfaceConfig {
    pub fn print(&self) {
        println!("{}", self.name);
        println!("* {}", self.target);
    }
}

pub struct TasmotaInterface {
    config: TasmotaInterfaceConfig,
    client: Client,
}

impl TasmotaInterface {
    pub fn new(config: TasmotaInterfaceConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
    pub fn print(&self) {
        println!("{}", self.config.name)
    }
}

// Monitoring
impl Monitoring for TasmotaInterface {
    async fn get_power(&self) -> Result<isize, Error> {
        let res = self
            .client
            .get(format!("http://{}/cm?cmnd=Status%208", &self.config.target))
            .send()
            .await?
            .text()
            .await?;

        let data: StatusResponse = serde_json::from_str(&res)?;
        Ok(data.status.energy.power)
    }
}

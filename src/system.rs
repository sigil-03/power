use crate::monitor::Monitoring;
use crate::tasmota::{TasmotaInterface, TasmotaInterfaceConfig};
use crate::types::Error;
use reqwest::Client;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct SystemConfig {
    components: Vec<TasmotaInterfaceConfig>,
}
impl SystemConfig {
    fn print(&self) {
        for t in &self.components {
            t.print();
        }
    }
}

pub struct System {
    components: Vec<TasmotaInterface>,
}

impl System {
    pub fn new_from_file(config_file: &str) -> Result<Self, Error> {
        let config_str = fs::read_to_string(config_file)?;
        let config: SystemConfig = toml::from_str(&config_str)?;
        Ok(Self {
            components: System::load_targets(&config.components),
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
        for component in &self.components {
            if let Ok(res) = component.get_power().await {
                component.print();
                println!("* POWER: {}W", res);
                println!("------------------")
            }
        }
        Ok(())
    }
}

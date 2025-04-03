use serde::Deserialize;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("toml parsing error")]
    ParseError(#[from] toml::de::Error),
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

pub struct Monitor {}

impl Monitor {
    pub fn new_from_file(config_file: &str) -> Result<Self, Error> {
        let config_str = fs::read_to_string(config_file)?;
        let config: MonitorConfig = toml::from_str(&config_str)?;
        config.print();
        Ok(Self {})
    }
}

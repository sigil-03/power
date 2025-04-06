use clap::Subcommand;
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

#[derive(Subcommand, Clone)]
pub enum PowerState {
    Off,
    On,
}

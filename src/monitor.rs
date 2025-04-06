use crate::tasmota::{PowerStatusData, StatusResponse, TasmotaInterface, TasmotaInterfaceConfig};
use crate::types::Error;
use reqwest::Client;
use serde::Deserialize;
use std::fs;

pub trait Monitoring {
    async fn get_power(&self) -> Result<isize, Error>;
}

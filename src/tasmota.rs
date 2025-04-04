use serde::Deserialize;

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

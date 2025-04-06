use crate::types::{Error, PowerState};

pub trait Control {
    async fn set_power(&self, state: PowerState) -> Result<(), Error>;
}
